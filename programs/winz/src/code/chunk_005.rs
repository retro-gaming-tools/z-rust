#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0043e137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e137); }
 let mut pc: u32 = 0x43e137;
 loop { match pc {
 0x43e137 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43e151); f_00406620(cpu); if cpu.returned_to != 0x43e151 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e156(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e156); }
 let mut pc: u32 = 0x43e156;
 loop { match pc {
 0x43e156 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e157(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e157); }
 let mut pc: u32 = 0x43e157;
 loop { match pc {
 0x43e157 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e158); }
 let mut pc: u32 = 0x43e158;
 loop { match pc {
 0x43e158 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e159(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e159); }
 let mut pc: u32 = 0x43e159;
 loop { match pc {
 0x43e159 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x43e1b4; continue; }
  pc = 0x43e15f; continue;
 }
 0x43e15f => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x43e166; continue; }
  pc = 0x43e164; continue;
 }
 0x43e164 => {
  pc = 0x43e1b4; continue;
 }
 0x43e166 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43e164; continue; }
  pc = 0x43e16f; continue;
 }
 0x43e16f => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x43e17e; continue; }
  pc = 0x43e174; continue;
 }
 0x43e174 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x43e164; continue; }
  pc = 0x43e179; continue;
 }
 0x43e179 => {
  if cpu.sf { cpu.poll(); pc = 0x43e164; continue; }
  pc = 0x43e17b; continue;
 }
 0x43e17b => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e17e; continue;
 }
 0x43e17e => {
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), cpu.r16(0x4d6ba4u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x43e194; continue; }
  pc = 0x43e18d; continue;
 }
 0x43e18d => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.ebp as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x43e164; continue; }
  pc = 0x43e192; continue;
 }
 0x43e192 => {
  if cpu.zf { cpu.poll(); pc = 0x43e164; continue; }
  pc = 0x43e194; continue;
 }
 0x43e194 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43e1b1); f_00406500(cpu); if cpu.returned_to != 0x43e1b1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x43e1b4; continue;
 }
 0x43e1b4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e1b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e1b6); }
 let mut pc: u32 = 0x43e1b6;
 loop { match pc {
 0x43e1b6 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x43e211; continue; }
  pc = 0x43e1bc; continue;
 }
 0x43e1bc => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x43e1c3; continue; }
  pc = 0x43e1c1; continue;
 }
 0x43e1c1 => {
  pc = 0x43e211; continue;
 }
 0x43e1c3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43e1c1; continue; }
  pc = 0x43e1cc; continue;
 }
 0x43e1cc => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x43e1db; continue; }
  pc = 0x43e1d1; continue;
 }
 0x43e1d1 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x43e1c1; continue; }
  pc = 0x43e1d6; continue;
 }
 0x43e1d6 => {
  if cpu.sf { cpu.poll(); pc = 0x43e1c1; continue; }
  pc = 0x43e1d8; continue;
 }
 0x43e1d8 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x43e1db; continue;
 }
 0x43e1db => {
  { let v: u16 = (cpu.ebx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), cpu.r16(0x4d6ba8u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x43e1f1; continue; }
  pc = 0x43e1ea; continue;
 }
 0x43e1ea => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.ebp as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x43e1c1; continue; }
  pc = 0x43e1ef; continue;
 }
 0x43e1ef => {
  if cpu.zf { cpu.poll(); pc = 0x43e1c1; continue; }
  pc = 0x43e1f1; continue;
 }
 0x43e1f1 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43e20e); f_00406590(cpu); if cpu.returned_to != 0x43e20e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x43e211; continue;
 }
 0x43e211 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e213); }
 let mut pc: u32 = 0x43e213;
 loop { match pc {
 0x43e213 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5a294eu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
  pc = 0x43e23c; continue;
 }
 0x43e23c => {
  { cpu.pushad(); }
  cpu.push(0x43e242); f_0043e278(cpu); if cpu.returned_to != 0x43e242 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x43e257; continue; }
  pc = 0x43e247; continue;
 }
 0x43e247 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e26a; continue;
 }
 0x43e257 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x43e26a; continue;
 }
 0x43e26a => {
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43e23c; continue; }
  pc = 0x43e271; continue;
 }
 0x43e271 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e272(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e272); }
 let mut pc: u32 = 0x43e272;
 loop { match pc {
 0x43e272 => {
  cpu.push(0x43e277); f_0043e213(cpu); if cpu.returned_to != 0x43e277 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e278(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e278); }
 let mut pc: u32 = 0x43e278;
 loop { match pc {
 0x43e278 => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e294); f_0043e303(cpu); if cpu.returned_to != 0x43e294 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2a3); f_0043e303(cpu); if cpu.returned_to != 0x43e2a3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2b2); f_0043e303(cpu); if cpu.returned_to != 0x43e2b2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2c1); f_0043e303(cpu); if cpu.returned_to != 0x43e2c1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = (cpu.edi as u16); let b = (cpu.esi as u16); { let v: u16 = b; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2d3); f_0043e303(cpu); if cpu.returned_to != 0x43e2d3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2e2); f_0043e303(cpu); if cpu.returned_to != 0x43e2e2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e2f1); f_0043e303(cpu); if cpu.returned_to != 0x43e2f1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e300); f_0043e303(cpu); if cpu.returned_to != 0x43e300 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e303(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e303); }
 let mut pc: u32 = 0x43e303;
 loop { match pc {
 0x43e303 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e336; continue; }
  pc = 0x43e30c; continue;
 }
 0x43e30c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if cpu.sf { pc = 0x43e336; continue; }
  pc = 0x43e315; continue;
 }
 0x43e315 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e336; continue; }
  pc = 0x43e31e; continue;
 }
 0x43e31e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e336; continue; }
  pc = 0x43e327; continue;
 }
 0x43e327 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x5a294eu32) as u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6fb8u32); cpu.push(0x43e335); cpu.call(t); if cpu.returned_to != 0x43e335 { return; }
  { cpu.popad(); }
  pc = 0x43e336; continue;
 }
 0x43e336 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e337(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e337); }
 let mut pc: u32 = 0x43e337;
 loop { match pc {
 0x43e337 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2954u32, v); }
  { let v: u8 = cpu.r8(0x5a2952u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2953u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x43e355; continue; }
  pc = 0x43e34f; continue;
 }
 0x43e34f => {
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x43e355); cpu.call(t); if cpu.returned_to != 0x43e355 { return; }
  pc = 0x43e355; continue;
 }
 0x43e355 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x43e37d; continue; }
  pc = 0x43e365; continue;
 }
 0x43e365 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x43e37d; continue; }
  pc = 0x43e36a; continue;
 }
 0x43e36a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5a2954u32) as u32; cpu.ebp = v; }
  cpu.push(0x43e377); f_0043e38b(cpu); if cpu.returned_to != 0x43e377 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  cpu.poll(); pc = 0x43e355; continue;
 }
 0x43e37d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2952u32)); cpu.w8(0x5a2952u32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0x5a2952u32), 0x3u8); cpu.w8(0x5a2952u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e38b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e38b); }
 let mut pc: u32 = 0x43e38b;
 loop { match pc {
 0x43e38b => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2958u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a295cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5a2960u32, v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43e406; continue; }
  pc = 0x43e3a0; continue;
 }
 0x43e3a0 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e3ad; continue; }
  pc = 0x43e3a9; continue;
 }
 0x43e3a9 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x43e3ad; continue;
 }
 0x43e3ad => {
  { let v: u32 = cpu.esi; cpu.w32(0x5a2964u32, v); }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x43e3e0; continue; }
  pc = 0x43e3b7; continue;
 }
 0x43e3b7 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5a2968u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a296cu32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a2974u32, v); }
  { let v: u32 = 0x43e4e6u32; cpu.w32(0x5a2978u32, v); }
  pc = 0x43e469; continue;
 }
 0x43e3e0 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5a2968u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a296cu32, v); }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a2970u32, v); }
  { let v: u32 = 0x43e531u32; cpu.w32(0x5a2978u32, v); }
  pc = 0x43e469; continue;
 }
 0x43e406 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e415; continue; }
  pc = 0x43e411; continue;
 }
 0x43e411 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x43e415; continue;
 }
 0x43e415 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5a2964u32, v); }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x43e445; continue; }
  pc = 0x43e41f; continue;
 }
 0x43e41f => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5a2968u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a296cu32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a2974u32, v); }
  { let v: u32 = 0x43e57cu32; cpu.w32(0x5a2978u32, v); }
  pc = 0x43e469; continue;
 }
 0x43e445 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5a2968u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a296cu32, v); }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5a2970u32, v); }
  { let v: u32 = 0x43e5c7u32; cpu.w32(0x5a2978u32, v); }
  pc = 0x43e469; continue;
 }
 0x43e469 => {
  { cpu.sub32(cpu.r32(0x5a2968u32), 0x0u32); }
  if cpu.zf { pc = 0x43e4e5; continue; }
  pc = 0x43e472; continue;
 }
 0x43e472 => {
  { let v: u32 = cpu.r8(0x5a2953u32) as u32; cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x43e49f; continue; }
  pc = 0x43e47d; continue;
 }
 0x43e47d => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x5a2968u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43e499; continue; }
  pc = 0x43e485; continue;
 }
 0x43e485 => {
  { let v: u32 = cpu.r32(0x5a2968u32); cpu.ecx = v; }
  { let v: u8 = cpu.sub8(cpu.r8(0x5a2953u32), (cpu.ecx as u8)); cpu.w8(0x5a2953u32, v); }
  let t = cpu.r32(0x5a2978u32); cpu.push(0x43e497); cpu.call(t); if cpu.returned_to != 0x43e497 { return; }
  pc = 0x43e4e5; continue;
 }
 0x43e499 => {
  let t = cpu.r32(0x5a2978u32); cpu.push(0x43e49f); cpu.call(t); if cpu.returned_to != 0x43e49f { return; }
  pc = 0x43e49f; continue;
 }
 0x43e49f => {
  { let v: u8 = 0x4u8; cpu.w8(0x5a2953u32, v); }
  { let v: u32 = cpu.r32(0x5a2958u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a295cu32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5a2960u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x43e469; continue; }
  pc = 0x43e4bf; continue;
 }
 0x43e4bf => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  if cpu.sf { cpu.poll(); pc = 0x43e469; continue; }
  pc = 0x43e4c7; continue;
 }
 0x43e4c7 => {
  { let v: u32 = cpu.r32(0x4d6ba4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43e469; continue; }
  pc = 0x43e4d2; continue;
 }
 0x43e4d2 => {
  { let v: u32 = cpu.r32(0x4d6ba8u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43e469; continue; }
  pc = 0x43e4dd; continue;
 }
 0x43e4dd => {
  let t = cpu.r32(0x4d6f64u32); cpu.push(0x43e4e3); cpu.call(t); if cpu.returned_to != 0x43e4e3 { return; }
  cpu.poll(); pc = 0x43e469; continue;
 }
 0x43e4e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e4e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e4e6); }
 let mut pc: u32 = 0x43e4e6;
 loop { match pc {
 0x43e4e6 => {
  { let v: u32 = cpu.r32(0x5a2958u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5a2964u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x43e4fd; continue; }
  pc = 0x43e4fb; continue;
 }
 0x43e4fb => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x43e4fd; continue;
 }
 0x43e4fd => {
  { let v: u32 = cpu.r32(0x5a297cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5a2974u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a296cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32).wrapping_sub(cpu.ecx); cpu.w32(0x5a2968u32, v); }
  pc = 0x43e514; continue;
 }
 0x43e514 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e520; continue; }
  pc = 0x43e518; continue;
 }
 0x43e518 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5a295cu32), cpu.esi); cpu.w32(0x5a295cu32, v); }
  pc = 0x43e520; continue;
 }
 0x43e520 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43e514; continue; }
  pc = 0x43e524; continue;
 }
 0x43e524 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2958u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e531(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e531); }
 let mut pc: u32 = 0x43e531;
 loop { match pc {
 0x43e531 => {
  { let v: u32 = cpu.r32(0x5a2958u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5a2964u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x43e548; continue; }
  pc = 0x43e546; continue;
 }
 0x43e546 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x43e548; continue;
 }
 0x43e548 => {
  { let v: u32 = cpu.r32(0x5a297cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5a2970u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a296cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32).wrapping_sub(cpu.ecx); cpu.w32(0x5a2968u32, v); }
  pc = 0x43e55f; continue;
 }
 0x43e55f => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e566; continue; }
  pc = 0x43e563; continue;
 }
 0x43e563 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x43e566; continue;
 }
 0x43e566 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5a295cu32), cpu.esi); cpu.w32(0x5a295cu32, v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43e55f; continue; }
  pc = 0x43e56f; continue;
 }
 0x43e56f => {
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2958u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e57c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e57c); }
 let mut pc: u32 = 0x43e57c;
 loop { match pc {
 0x43e57c => {
  { let v: u32 = cpu.r32(0x5a2958u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5a2964u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x43e593; continue; }
  pc = 0x43e591; continue;
 }
 0x43e591 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x43e593; continue;
 }
 0x43e593 => {
  { let v: u32 = cpu.r32(0x5a297cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5a2974u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a296cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32).wrapping_sub(cpu.ecx); cpu.w32(0x5a2968u32, v); }
  pc = 0x43e5aa; continue;
 }
 0x43e5aa => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e5b6; continue; }
  pc = 0x43e5ae; continue;
 }
 0x43e5ae => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5a295cu32), cpu.esi); cpu.w32(0x5a295cu32, v); }
  pc = 0x43e5b6; continue;
 }
 0x43e5b6 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43e5aa; continue; }
  pc = 0x43e5ba; continue;
 }
 0x43e5ba => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2958u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e5c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e5c7); }
 let mut pc: u32 = 0x43e5c7;
 loop { match pc {
 0x43e5c7 => {
  { let v: u32 = cpu.r32(0x5a2958u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5a2964u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x43e5de; continue; }
  pc = 0x43e5dc; continue;
 }
 0x43e5dc => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x43e5de; continue;
 }
 0x43e5de => {
  { let v: u32 = cpu.r32(0x5a297cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5a2970u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a296cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a2968u32).wrapping_sub(cpu.ecx); cpu.w32(0x5a2968u32, v); }
  pc = 0x43e5f5; continue;
 }
 0x43e5f5 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x43e600; continue; }
  pc = 0x43e5f9; continue;
 }
 0x43e5f9 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x5a296cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x43e600; continue;
 }
 0x43e600 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5a295cu32), cpu.esi); cpu.w32(0x5a295cu32, v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43e5f5; continue; }
  pc = 0x43e609; continue;
 }
 0x43e609 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5a297cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2958u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e618(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e618); }
 let mut pc: u32 = 0x43e618;
 loop { match pc {
 0x43e618 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e619(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e619); }
 let mut pc: u32 = 0x43e619;
 loop { match pc {
 0x43e619 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d67b8u32, v); }
  { cpu.sub16(cpu.r16(0x4d67b6u32), 0x0u16); }
  if cpu.zf { pc = 0x43e62a; continue; }
  pc = 0x43e62a; continue;
 }
 0x43e62a => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2980u32, v); }
  { let v: u16 = (cpu.eax as u16) & 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2984u32, v); }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43e64f); f_00420989(cpu); if cpu.returned_to != 0x43e64f { return; }
  { let v: u32 = cpu.r32(0x5a2980u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2980u32, v); }
  { let v: u32 = cpu.r32(0x5a2984u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2984u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1553u32, v); }
  { cpu.sub8(cpu.r8(0x4e15b7u32), 0x0u8); }
  if !cpu.zf { pc = 0x43e683; continue; }
  pc = 0x43e67d; continue;
 }
 0x43e67d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1553u32)); cpu.w8(0x4e1553u32, v); }
  pc = 0x43e683; continue;
 }
 0x43e683 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x15u8; cpu.w8(0x4e1554u32, v); }
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6ba4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1554u32).wrapping_sub((cpu.eax as u8)); cpu.w8(0x4e1554u32, v); }
  { let v = cpu.r32(0x5a2980u32); cpu.push(v); }
  pc = 0x43e6a6; continue;
 }
 0x43e6a6 => {
  cpu.push(0x43e6ab); f_0043e6d0(cpu); if cpu.returned_to != 0x43e6ab { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1554u32)); cpu.w8(0x4e1554u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x43e6a6; continue; }
  pc = 0x43e6b3; continue;
 }
 0x43e6b3 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a2980u32, v); }
  { let v: u32 = cpu.r32(0x5a2984u32).wrapping_add(0x10u32); cpu.w32(0x5a2984u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1553u32)); cpu.w8(0x4e1553u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x43e683; continue; }
  pc = 0x43e6cf; continue;
 }
 0x43e6cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e6d0); }
 let mut pc: u32 = 0x43e6d0;
 loop { match pc {
 0x43e6d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x43e6e5; continue; }
  pc = 0x43e6e1; continue;
 }
 0x43e6e1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e6e5; continue;
 }
 0x43e6e5 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ac40eu32)); }
  if cpu.zf { pc = 0x43e708; continue; }
  pc = 0x43e6ee; continue;
 }
 0x43e6ee => {
  { let v: u32 = cpu.shr32(cpu.eax, 6u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x5a2984u32); cpu.push(v); }
  { let v = cpu.r32(0x5a2980u32); cpu.push(v); }
  cpu.push(0x43e704); f_00406b60(cpu); if cpu.returned_to != 0x43e704 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x43e708; continue;
 }
 0x43e708 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5a2980u32), 0x10u32); cpu.w32(0x5a2980u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e710); }
 let mut pc: u32 = 0x43e710;
 loop { match pc {
 0x43e710 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2b3cu32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x43e72d; continue; }
  pc = 0x43e71f; continue;
 }
 0x43e71f => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x12u8); }
  if !cpu.zf { pc = 0x43e759; continue; }
  pc = 0x43e72d; continue;
 }
 0x43e72d => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x43e72d; continue; }
  pc = 0x43e73b; continue;
 }
 0x43e73b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43e743; continue;
 }
 0x43e743 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if !cpu.zf { pc = 0x43e759; continue; }
  pc = 0x43e747; continue;
 }
 0x43e747 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2b3cu32)); cpu.w8(0x5a2b3cu32, v); }
  cpu.poll(); pc = 0x43e743; continue;
 }
 0x43e759 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4000u32); cpu.eax = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.edi = v; }
  pc = 0x43e76d; continue;
 }
 0x43e76d => {
  { cpu.scas(4, 0); }
  if cpu.zf { pc = 0x43e776; continue; }
  pc = 0x43e770; continue;
 }
 0x43e770 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43e76d; continue; }
  pc = 0x43e775; continue;
 }
 0x43e775 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43e776 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x43e7f9; continue; }
  pc = 0x43e780; continue;
 }
 0x43e780 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d954cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x8au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x520e94u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x43e7c6); f_0043f196(cpu); if cpu.returned_to != 0x43e7c6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x43e7d6); f_0043eb85(cpu); if cpu.returned_to != 0x43e7d6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x43e7dd); f_0043e7fc(cpu); if cpu.returned_to != 0x43e7dd { return; }
  if cpu.cf { pc = 0x43e7e6; continue; }
  pc = 0x43e7df; continue;
 }
 0x43e7df => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x43e7e5); f_0043eb85(cpu); if cpu.returned_to != 0x43e7e5 { return; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x43e7e6; continue;
 }
 0x43e7e6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f96u32, v); }
  cpu.push(0x43e7f2); f_0043ec3e(cpu); if cpu.returned_to != 0x43e7f2 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x43e7f8); f_0043e990(cpu); if cpu.returned_to != 0x43e7f8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43e7f9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e7fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43e7fc); } r_0043e7fc(cpu, 0x43e7fc); }
pub fn f_0043e800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43e800); } r_0043e7fc(cpu, 0x43e800); }
fn r_0043e7fc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43e7fc => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e8464u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x43e8aa; continue; }
  pc = 0x43e813; continue;
 }
 0x43e800 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e8464u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x43e8aa; continue; }
  pc = 0x43e813; continue;
 }
 0x43e813 => {
  { let v: u32 = 0x4001u32.wrapping_add(cpu.edi); cpu.eax = v; }
  cpu.push(0x43e81e); f_0042c9e5(cpu); if cpu.returned_to != 0x43e81e { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a3d92u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a3d94u32, v); }
  { let v: u32 = 0x520e94u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x43e83d; continue;
 }
 0x43e83d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x5a3d92u32); cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x5a3d98u32)); }
  if cpu.zf { pc = 0x43e8ad; continue; }
  pc = 0x43e84d; continue;
 }
 0x43e84d => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x5a3d92u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x43e8ad; continue; }
  pc = 0x43e85d; continue;
 }
 0x43e85d => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x43e8ad; continue; }
  pc = 0x43e863; continue;
 }
 0x43e863 => {
  { let v: u16 = cpu.r16(0x5a3d94u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x43e8ad; continue; }
  pc = 0x43e870; continue;
 }
 0x43e870 => {
  { cpu.sub16((cpu.ebx as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x43e8ad; continue; }
  pc = 0x43e877; continue;
 }
 0x43e877 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  cpu.push(0x43e894); f_0043f154(cpu); if cpu.returned_to != 0x43e894 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x43e8a2; continue; }
  pc = 0x43e898; continue;
 }
 0x43e898 => {
  { let v: u16 = cpu.r16(0x5a3d94u32).wrapping_sub(0x10u16); cpu.w16(0x5a3d94u32, v); }
  pc = 0x43e8ad; continue;
 }
 0x43e8a2 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a3d92u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x43e8aa; continue;
 }
 0x43e8aa => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43e8ad => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a3d92u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43e83d; continue; }
  pc = 0x43e8c2; continue;
 }
 0x43e8c2 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e8c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e8c5); }
 let mut pc: u32 = 0x43e8c5;
 loop { match pc {
 0x43e8c5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi.wrapping_sub(0x4dd54cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2b28u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2b2cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x43e935; continue; }
  pc = 0x43e91e; continue;
 }
 0x43e91e => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x43e93a; continue; }
  pc = 0x43e927; continue;
 }
 0x43e927 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x43e93f; continue; }
  pc = 0x43e930; continue;
 }
 0x43e930 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x43e942; continue;
 }
 0x43e935 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x43e942; continue;
 }
 0x43e93a => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x43e942; continue;
 }
 0x43e93f => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x43e942; continue;
 }
 0x43e942 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x43e945; continue;
 }
 0x43e945 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43e94c); f_0041c743(cpu); if cpu.returned_to != 0x43e94c { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x43e95a; continue; }
  pc = 0x43e952; continue;
 }
 0x43e952 => {
  { let v: u16 = cpu.r16(0x5a2b02u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e960; continue;
 }
 0x43e95a => {
  { let v: u16 = cpu.r16(0x5a2b04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e960; continue;
 }
 0x43e960 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5a2b28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a2b2cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x43e98a); f_004829bb(cpu); if cpu.returned_to != 0x43e98a { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43e945; continue; }
  pc = 0x43e98e; continue;
 }
 0x43e98e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e990); }
 let mut pc: u32 = 0x43e990;
 loop { match pc {
 0x43e990 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edi.wrapping_sub(0x4dd54cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2b28u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2b2cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  cpu.push(0x43e9cc); f_0041c743(cpu); if cpu.returned_to != 0x43e9cc { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x43e9da; continue; }
  pc = 0x43e9d2; continue;
 }
 0x43e9d2 => {
  { let v: u16 = cpu.r16(0x5a2b0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e9e0; continue;
 }
 0x43e9da => {
  { let v: u16 = cpu.r16(0x5a2b0cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e9e0; continue;
 }
 0x43e9e0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = 0x482da9u32; cpu.w32(0x5a2b30u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x43ea1a; continue; }
  pc = 0x43ea03; continue;
 }
 0x43ea03 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x43ea1f; continue; }
  pc = 0x43ea0c; continue;
 }
 0x43ea0c => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x43ea24; continue; }
  pc = 0x43ea15; continue;
 }
 0x43ea15 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x43ea27; continue;
 }
 0x43ea1a => {
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  pc = 0x43ea27; continue;
 }
 0x43ea1f => {
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  pc = 0x43ea27; continue;
 }
 0x43ea24 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x43ea27; continue;
 }
 0x43ea27 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a2b28u32).wrapping_sub(0x8u32); cpu.w32(0x5a2b28u32, v); }
  { let v: u32 = cpu.r32(0x5a2b2cu32).wrapping_sub(0x8u32); cpu.w32(0x5a2b2cu32, v); }
  pc = 0x43ea38; continue;
 }
 0x43ea38 => {
  cpu.push(0x43ea3d); f_0043eb27(cpu); if cpu.returned_to != 0x43ea3d { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43ea38; continue; }
  pc = 0x43ea3f; continue;
 }
 0x43ea3f => {
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = 0x4829bbu32; cpu.w32(0x5a2b30u32, v); }
  { let v: u32 = cpu.r32(0x5a2b28u32).wrapping_add(0x4u32); cpu.w32(0x5a2b28u32, v); }
  { let v: u32 = cpu.r32(0x5a2b2cu32).wrapping_add(0x4u32); cpu.w32(0x5a2b2cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x43ea88; continue; }
  pc = 0x43ea71; continue;
 }
 0x43ea71 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x43ea8d; continue; }
  pc = 0x43ea7a; continue;
 }
 0x43ea7a => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x43ea92; continue; }
  pc = 0x43ea83; continue;
 }
 0x43ea83 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x43ea95; continue;
 }
 0x43ea88 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x43ea95; continue;
 }
 0x43ea8d => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x43ea95; continue;
 }
 0x43ea92 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x43ea95; continue;
 }
 0x43ea95 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x43ea98; continue;
 }
 0x43ea98 => {
  cpu.push(0x43ea9d); f_0041c743(cpu); if cpu.returned_to != 0x43ea9d { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x43eaab; continue; }
  pc = 0x43eaa3; continue;
 }
 0x43eaa3 => {
  { let v: u16 = cpu.r16(0x5a2b06u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43eab1; continue;
 }
 0x43eaab => {
  { let v: u16 = cpu.r16(0x5a2b08u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43eab1; continue;
 }
 0x43eab1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  cpu.push(0x43eabc); f_0043eb27(cpu); if cpu.returned_to != 0x43eabc { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43ea98; continue; }
  pc = 0x43eabe; continue;
 }
 0x43eabe => {
  { let v: u32 = cpu.r32(0x5a2b28u32).wrapping_add(0x2u32); cpu.w32(0x5a2b28u32, v); }
  { let v: u32 = cpu.r32(0x5a2b2cu32).wrapping_add(0x2u32); cpu.w32(0x5a2b2cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x43eaef; continue; }
  pc = 0x43ead8; continue;
 }
 0x43ead8 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x43eaf4; continue; }
  pc = 0x43eae1; continue;
 }
 0x43eae1 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x43eaf9; continue; }
  pc = 0x43eaea; continue;
 }
 0x43eaea => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x43eafc; continue;
 }
 0x43eaef => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x43eafc; continue;
 }
 0x43eaf4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x43eafc; continue;
 }
 0x43eaf9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x43eafc; continue;
 }
 0x43eafc => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x43eaff; continue;
 }
 0x43eaff => {
  cpu.push(0x43eb04); f_0041c743(cpu); if cpu.returned_to != 0x43eb04 { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x43eb12; continue; }
  pc = 0x43eb0a; continue;
 }
 0x43eb0a => {
  { let v: u16 = cpu.r16(0x5a2b02u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43eb18; continue;
 }
 0x43eb12 => {
  { let v: u16 = cpu.r16(0x5a2b04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43eb18; continue;
 }
 0x43eb18 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  cpu.push(0x43eb23); f_0043eb27(cpu); if cpu.returned_to != 0x43eb23 { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43eaff; continue; }
  pc = 0x43eb25; continue;
 }
 0x43eb25 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043eb27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43eb27); }
 let mut pc: u32 = 0x43eb27;
 loop { match pc {
 0x43eb27 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x5a2b3cu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x43eb43); f_0041c743(cpu); if cpu.returned_to != 0x43eb43 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x43eb54); f_0041c743(cpu); if cpu.returned_to != 0x43eb54 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2b28u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2b2cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  let t = cpu.r32(0x5a2b30u32); cpu.push(0x43eb83); cpu.call(t); if cpu.returned_to != 0x43eb83 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043eb85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43eb85); }
 let mut pc: u32 = 0x43eb85;
 loop { match pc {
 0x43eb85 => {
  cpu.push(0x43eb8a); f_0041c743(cpu); if cpu.returned_to != 0x43eb8a { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2e72u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x16u8; cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43eb85; continue; }
  pc = 0x43ebb0; continue;
 }
 0x43ebb0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ebdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ebdd); }
 let mut pc: u32 = 0x43ebdd;
 loop { match pc {
 0x43ebdd => {
  cpu.push(0x43ebe2); f_0043ec0c(cpu); if cpu.returned_to != 0x43ebe2 { return; }
  { let v: u32 = 0x520e94u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f96u32, v); }
  pc = 0x43ebf3; continue;
 }
 0x43ebf3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x43ebfa); f_0043ec3e(cpu); if cpu.returned_to != 0x43ebfa { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8au32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43ebf3; continue; }
  pc = 0x43ec04; continue;
 }
 0x43ec04 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f96u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ec0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ec0c); }
 let mut pc: u32 = 0x43ec0c;
 loop { match pc {
 0x43ec0c => {
  { let v: u32 = 0x520e94u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x43ec18; continue;
 }
 0x43ec18 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x43ec26; continue;
 }
 0x43ec26 => {
  cpu.push(0x43ec2b); f_0043f154(cpu); if cpu.returned_to != 0x43ec2b { return; }
  if cpu.zf { pc = 0x43ec30; continue; }
  pc = 0x43ec2d; continue;
 }
 0x43ec2d => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x43ec30; continue;
 }
 0x43ec30 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43ec26; continue; }
  pc = 0x43ec34; continue;
 }
 0x43ec34 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8au32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43ec18; continue; }
  pc = 0x43ec3d; continue;
 }
 0x43ec3d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ec3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ec3e); }
 let mut pc: u32 = 0x43ec3e;
 loop { match pc {
 0x43ec3e => {
  { let v: u8 = 0x5u8; cpu.w8(0x56c6e5u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5a3d98u32, v); }
  cpu.push(0x43ec50); f_0043ed4a(cpu); if cpu.returned_to != 0x43ec50 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4d954cu32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6934u32).wrapping_add(0x200u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32).wrapping_add(0x200u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x43eca2; continue;
 }
 0x43eca2 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  cpu.push(0x43ecab); f_0043f154(cpu); if cpu.returned_to != 0x43ecab { return; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.ecx; cpu.push(v); }
  if cpu.zf { pc = 0x43ece6; continue; }
  pc = 0x43ecb0; continue;
 }
 0x43ecb0 => {
  cpu.push(0x43ecb5); f_0043ed0c(cpu); if cpu.returned_to != 0x43ecb5 { return; }
  cpu.push(0x43ecba); f_0043efe7(cpu); if cpu.returned_to != 0x43ecba { return; }
  cpu.push(0x43ecbf); f_0043ed9a(cpu); if cpu.returned_to != 0x43ecbf { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x14u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(0x12u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x43ecc9; continue;
 }
 0x43ecc9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.sf != cpu.of) { pc = 0x43ece5; continue; }
  pc = 0x43ecd4; continue;
 }
 0x43ecd4 => {
  if !cpu.zf { pc = 0x43ecd9; continue; }
  pc = 0x43ecd6; continue;
 }
 0x43ecd6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x14u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x43ecd9; continue;
 }
 0x43ecd9 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x43ecc9; continue;
 }
 0x43ece5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x43ece6; continue;
 }
 0x43ece6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if cpu.cf { cpu.poll(); pc = 0x43eca2; continue; }
  pc = 0x43ecf7; continue;
 }
 0x43ecf7 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x200u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x60u32); cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if cpu.cf { cpu.poll(); pc = 0x43eca2; continue; }
  pc = 0x43ed0b; continue;
 }
 0x43ed0b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ed0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ed0c); }
 let mut pc: u32 = 0x43ed0c;
 loop { match pc {
 0x43ed0c => {
  { cpu.sub8(cpu.r8(0x4e1f96u32), 0x0u8); }
  if cpu.zf { pc = 0x43ed49; continue; }
  pc = 0x43ed15; continue;
 }
 0x43ed15 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6b44u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x4e1f97u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x43ed39); f_0041c743(cpu); if cpu.returned_to != 0x43ed39 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6b48u32), 0x8u32); cpu.w32(0x4d6b48u32, v); }
  pc = 0x43ed49; continue;
 }
 0x43ed49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ed4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ed4a); }
 let mut pc: u32 = 0x43ed4a;
 loop { match pc {
 0x43ed4a => {
  { cpu.sub8(cpu.r8(0x4e1f96u32), 0x0u8); }
  if cpu.zf { pc = 0x43ed90; continue; }
  pc = 0x43ed53; continue;
 }
 0x43ed53 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x520e94u32); cpu.eax = v; }
  { let v: u8 = 0x8au8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f97u32, v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, 0x5218d2u32); }
  if !cpu.zf { pc = 0x43ed7b; continue; }
  pc = 0x43ed76; continue;
 }
 0x43ed76 => {
  { let v: u32 = 0x9c38u32; cpu.ecx = v; }
  pc = 0x43ed7b; continue;
 }
 0x43ed7b => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x6u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  if cpu.zf { pc = 0x43ed90; continue; }
  pc = 0x43ed81; continue;
 }
 0x43ed81 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4d6b44u32)); cpu.edi = v; }
  { cpu.stos(2, true); }
  pc = 0x43ed90; continue;
 }
 0x43ed90 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b48u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ed9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ed9a); }
 let mut pc: u32 = 0x43ed9a;
 loop { match pc {
 0x43ed9a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xcu16); }
  if cpu.zf { pc = 0x43ede5; continue; }
  pc = 0x43edaf; continue;
 }
 0x43edaf => {
  { cpu.sub16((cpu.edx as u16), 0xeu16); }
  if cpu.zf { pc = 0x43ede5; continue; }
  pc = 0x43edb5; continue;
 }
 0x43edb5 => {
  { cpu.sub16((cpu.edx as u16), 0x10u16); }
  if cpu.zf { pc = 0x43ede5; continue; }
  pc = 0x43edbb; continue;
 }
 0x43edbb => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x12u8); }
  if !cpu.zf { pc = 0x43ede5; continue; }
  pc = 0x43edc9; continue;
 }
 0x43edc9 => {
  { let v: u16 = cpu.r16(0x4ba470u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x43edd5); f_0043eead(cpu); if cpu.returned_to != 0x43edd5 { return; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)) | 0x12u8; cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  pc = 0x43ede5; continue;
 }
 0x43ede5 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x43edf0); f_0043f6b3(cpu); if cpu.returned_to != 0x43edf0 { return; }
  if cpu.cf { pc = 0x43eea1; continue; }
  pc = 0x43edf6; continue;
 }
 0x43edf6 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.sf != cpu.of) { pc = 0x43eea8; continue; }
  pc = 0x43edff; continue;
 }
 0x43edff => {
  if !cpu.zf { pc = 0x43ee3c; continue; }
  pc = 0x43ee01; continue;
 }
 0x43ee01 => {
  { let v: u16 = cpu.r16(0x4ba474u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0xffffff81u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x43eea8; continue; }
  pc = 0x43ee1c; continue;
 }
 0x43ee1c => {
  { cpu.sub8(cpu.r8(0x3f81u32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x43eea8; continue; }
  pc = 0x43ee28; continue;
 }
 0x43ee28 => {
  { let v: u16 = cpu.r16(0x4ba472u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffff81u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3f81u32.wrapping_add(cpu.edi), v); }
  pc = 0x43eea8; continue;
 }
 0x43ee3c => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x12u8); }
  if !cpu.zf { pc = 0x43ee74; continue; }
  pc = 0x43ee4a; continue;
 }
 0x43ee4a => {
  { let v: u8 = cpu.r8(0xffffff81u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3f81u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4ba4a2u32)); }
  if cpu.zf { pc = 0x43ee68; continue; }
  pc = 0x43ee5f; continue;
 }
 0x43ee5f => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4ba4a4u32)); }
  if !cpu.zf { pc = 0x43ee74; continue; }
  pc = 0x43ee68; continue;
 }
 0x43ee68 => {
  { let v: u16 = cpu.r16(0x4ba4a4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x12u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x43ee98; continue;
 }
 0x43ee74 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.r16(0x4ba472u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x12u8); }
  if !cpu.zf { pc = 0x43ee98; continue; }
  pc = 0x43ee89; continue;
 }
 0x43ee89 => {
  { let v: u16 = cpu.r16(0x4ba470u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x43ee95); f_0043eead(cpu); if cpu.returned_to != 0x43ee95 { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x12u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x43ee98; continue;
 }
 0x43ee98 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  pc = 0x43eea1; continue;
 }
 0x43eea1 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x43ede5; continue;
 }
 0x43eea8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043eead(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43eead); }
 let mut pc: u32 = 0x43eead;
 loop { match pc {
 0x43eead => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba4a2u32)); }
  if cpu.zf { pc = 0x43efdb; continue; }
  pc = 0x43eec6; continue;
 }
 0x43eec6 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba4a4u32)); }
  if cpu.zf { pc = 0x43efe3; continue; }
  pc = 0x43eed3; continue;
 }
 0x43eed3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4ba456u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x14u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x43efd3; continue; }
  pc = 0x43eee9; continue;
 }
 0x43eee9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4ba456u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x28u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x43efdb; continue; }
  pc = 0x43eeff; continue;
 }
 0x43eeff => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba476u32)); }
  if cpu.zf { pc = 0x43efc3; continue; }
  pc = 0x43ef0c; continue;
 }
 0x43ef0c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba478u32)); }
  if cpu.zf { pc = 0x43efc3; continue; }
  pc = 0x43ef19; continue;
 }
 0x43ef19 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba47au32)); }
  if cpu.zf { pc = 0x43efc3; continue; }
  pc = 0x43ef26; continue;
 }
 0x43ef26 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba47cu32)); }
  if cpu.zf { pc = 0x43efc3; continue; }
  pc = 0x43ef33; continue;
 }
 0x43ef33 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba48eu32)); }
  if cpu.zf { pc = 0x43efcb; continue; }
  pc = 0x43ef40; continue;
 }
 0x43ef40 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba482u32)); }
  if cpu.zf { pc = 0x43efd3; continue; }
  pc = 0x43ef4d; continue;
 }
 0x43ef4d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba494u32)); }
  if cpu.zf { pc = 0x43efdb; continue; }
  pc = 0x43ef5a; continue;
 }
 0x43ef5a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba47eu32)); }
  if cpu.zf { pc = 0x43efa3; continue; }
  pc = 0x43ef63; continue;
 }
 0x43ef63 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba490u32)); }
  if cpu.zf { pc = 0x43efab; continue; }
  pc = 0x43ef6c; continue;
 }
 0x43ef6c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba484u32)); }
  if cpu.zf { pc = 0x43efd3; continue; }
  pc = 0x43ef75; continue;
 }
 0x43ef75 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba496u32)); }
  if cpu.zf { pc = 0x43efdb; continue; }
  pc = 0x43ef7e; continue;
 }
 0x43ef7e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba480u32)); }
  if cpu.zf { pc = 0x43efb3; continue; }
  pc = 0x43ef87; continue;
 }
 0x43ef87 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba492u32)); }
  if cpu.zf { pc = 0x43efbb; continue; }
  pc = 0x43ef90; continue;
 }
 0x43ef90 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba486u32)); }
  if cpu.zf { pc = 0x43efd3; continue; }
  pc = 0x43ef99; continue;
 }
 0x43ef99 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ba498u32)); }
  if cpu.zf { pc = 0x43efdb; continue; }
  pc = 0x43efa2; continue;
 }
 0x43efa2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efa3 => {
  { let v: u16 = cpu.r16(0x4ba49au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efab => {
  { let v: u16 = cpu.r16(0x4ba49eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efb3 => {
  { let v: u16 = cpu.r16(0x4ba49cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efbb => {
  { let v: u16 = cpu.r16(0x4ba4a0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efc3 => {
  { let v: u16 = cpu.r16(0x4ba46eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efcb => {
  { let v: u16 = cpu.r16(0x4ba470u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efd3 => {
  { let v: u16 = cpu.r16(0x4ba4a2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efdb => {
  { let v: u16 = cpu.r16(0x4ba4a4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43efe3 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043efe7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43efe7); } r_0043efe7(cpu, 0x43efe7); }
pub fn f_0043f000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43f000); } r_0043efe7(cpu, 0x43f000); }
fn r_0043efe7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43efe7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (cpu.sf != cpu.of) { pc = 0x43f001; continue; }
  pc = 0x43eff3; continue;
 }
 0x43eff3 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x43effc); f_0043f154(cpu); if cpu.returned_to != 0x43effc { return; }
  if cpu.zf { pc = 0x43f001; continue; }
  pc = 0x43effe; continue;
 }
 0x43effe => {
  { let v: u8 = (cpu.ebx as u8) & 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43f001; continue;
 }
 0x43f000 => {
  { cpu.push(0x2b); }
  pc = 0x43f001; continue;
 }
 0x43f001 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1fu16); }
  if cpu.zf { pc = 0x43f01d; continue; }
  pc = 0x43f00e; continue;
 }
 0x43f00e => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x43f018); f_0043f154(cpu); if cpu.returned_to != 0x43f018 { return; }
  if cpu.zf { pc = 0x43f01d; continue; }
  pc = 0x43f01a; continue;
 }
 0x43f01a => {
  { let v: u8 = (cpu.ebx as u8) & 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43f01d; continue;
 }
 0x43f01d => {
  { cpu.sub16((cpu.ecx as u16), 0x3e0u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x43f035; continue; }
  pc = 0x43f024; continue;
 }
 0x43f024 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x43f030); f_0043f154(cpu); if cpu.returned_to != 0x43f030 { return; }
  if cpu.zf { pc = 0x43f035; continue; }
  pc = 0x43f032; continue;
 }
 0x43f032 => {
  { let v: u8 = (cpu.ebx as u8) & 0xbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43f035; continue;
 }
 0x43f035 => {
  { cpu.and16((cpu.ecx as u16), 0x1fu16); }
  if cpu.zf { pc = 0x43f04b; continue; }
  pc = 0x43f03c; continue;
 }
 0x43f03c => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x43f046); f_0043f154(cpu); if cpu.returned_to != 0x43f046 { return; }
  if cpu.zf { pc = 0x43f04b; continue; }
  pc = 0x43f048; continue;
 }
 0x43f048 => {
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43f04b; continue;
 }
 0x43f04b => {
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u8 = 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x43f094; continue; }
  pc = 0x43f05c; continue;
 }
 0x43f05c => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x43f066; continue;
 }
 0x43f066 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if cpu.zf { pc = 0x43f070; continue; }
  pc = 0x43f06b; continue;
 }
 0x43f06b => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43f066; continue; }
  pc = 0x43f070; continue;
 }
 0x43f070 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4ba4b2u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(0x0u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f094 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x43f09f; continue;
 }
 0x43f09f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if cpu.zf { pc = 0x43f0a9; continue; }
  pc = 0x43f0a4; continue;
 }
 0x43f0a4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43f09f; continue; }
  pc = 0x43f0a9; continue;
 }
 0x43f0a9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x6u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4ba4d2u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
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
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba468u32)); }
  if !cpu.zf { pc = 0x43f0f5; continue; }
  pc = 0x43f0e6; continue;
 }
 0x43f0e6 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x4ba469u32)); }
  if !cpu.zf { pc = 0x43f0f5; continue; }
  pc = 0x43f0ee; continue;
 }
 0x43f0ee => {
  { let v: u16 = cpu.r16(0x4ba456u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x43f0f5; continue;
 }
 0x43f0f5 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba44cu32)); }
  if !cpu.zf { pc = 0x43f10c; continue; }
  pc = 0x43f0fd; continue;
 }
 0x43f0fd => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x4ba44du32)); }
  if !cpu.zf { pc = 0x43f10c; continue; }
  pc = 0x43f105; continue;
 }
 0x43f105 => {
  { let v: u16 = cpu.r16(0x4ba43au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x43f10c; continue;
 }
 0x43f10c => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba44eu32)); }
  if !cpu.zf { pc = 0x43f123; continue; }
  pc = 0x43f114; continue;
 }
 0x43f114 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x4ba44fu32)); }
  if !cpu.zf { pc = 0x43f123; continue; }
  pc = 0x43f11c; continue;
 }
 0x43f11c => {
  { let v: u16 = cpu.r16(0x4ba43cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x43f123; continue;
 }
 0x43f123 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba444u32)); }
  if cpu.zf { pc = 0x43f13c; continue; }
  pc = 0x43f12b; continue;
 }
 0x43f12b => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba44au32)); }
  if cpu.zf { pc = 0x43f144; continue; }
  pc = 0x43f133; continue;
 }
 0x43f133 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4ba450u32)); }
  if cpu.zf { pc = 0x43f14c; continue; }
  pc = 0x43f13b; continue;
 }
 0x43f13b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f13c => {
  { let v: u16 = cpu.r16(0x4ba432u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f144 => {
  { let v: u16 = cpu.r16(0x4ba438u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f14c => {
  { let v: u16 = cpu.r16(0x4ba43eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f154); }
 let mut pc: u32 = 0x43f154;
 loop { match pc {
 0x43f154 => {
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
pub fn f_0043f196(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f196); }
 let mut pc: u32 = 0x43f196;
 loop { match pc {
 0x43f196 => {
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
pub fn f_0043f1ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f1ba); }
 let mut pc: u32 = 0x43f1ba;
 loop { match pc {
 0x43f1ba => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a334cu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x4ba3f4u32; cpu.edi = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x5a29a8u32; cpu.esi = v; }
  { let v: u32 = 0x4ba4b2u32; cpu.edi = v; }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x43f1f0; continue;
 }
 0x43f1f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4ba3f4u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x43f1f0; continue; }
  pc = 0x43f206; continue;
 }
 0x43f206 => {
  { let v: u32 = 0x5a29e8u32; cpu.esi = v; }
  { let v: u32 = 0x4ba4d2u32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x43f212; continue;
 }
 0x43f212 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4ba3f4u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x43f212; continue; }
  pc = 0x43f228; continue;
 }
 0x43f228 => {
  { let v: u32 = 0x5a2988u32; cpu.esi = v; }
  { let v: u32 = 0x5a2e72u32; cpu.edi = v; }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x43f234; continue;
 }
 0x43f234 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4ba3f4u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x43f234; continue; }
  pc = 0x43f24a; continue;
 }
 0x43f24a => {
  cpu.push(0x43f24f); f_0043f250(cpu); if cpu.returned_to != 0x43f24f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f250); }
 let mut pc: u32 = 0x43f250;
 loop { match pc {
 0x43f250 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5a2b45u32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4ea5feu32, v); }
  { let v: u32 = cpu.r32(0x4ea5bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x4ea56cu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = 0x4ea59cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a3360u32); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4ea568u32, v); }
  { let v: u32 = 0x4ea58cu32; cpu.esi = v; }
  { let v: u32 = 0x4ea594u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2d32u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b0au32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b0cu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b06u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b08u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b02u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b04u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2d82u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2aecu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2af0u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2aeeu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2af2u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2ae8u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2af6u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2aeau32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2afau32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2af4u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2af8u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b00u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5d47e9u32); cpu.esi = v; }
  { let v: u32 = 0x5d47e1u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2e22u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d4ce0u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2e2cu32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d4ce2u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2cd3u32); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4eab29u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2e36u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2afeu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2afcu32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2cecu32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b0eu32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2cd8u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b10u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b12u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2e68u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b38u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2e4au32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b3au32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2cf6u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576505u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576507u32, v); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5d4875u32); cpu.esi = v; }
  { let v: u32 = 0x5d4855u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5a2d0au32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b3du32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b3fu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b41u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2b43u32, v); }
  { let v: u8 = 0xc5u8; cpu.w8(0x4ea225u32, v); }
  { let v: u8 = 0xc5u8; cpu.w8(0x4ea22bu32, v); }
  { let v: u8 = 0x5fu8; cpu.w8(0x4ea268u32, v); }
  { let v: u8 = 0x5fu8; cpu.w8(0x4ea26eu32, v); }
  { let v: u8 = 0xeau8; cpu.w8(0x4ea2abu32, v); }
  { let v: u8 = 0xeau8; cpu.w8(0x4ea2b1u32, v); }
  { let v: u8 = 0xf9u8; cpu.w8(0x4ea2eeu32, v); }
  { let v: u8 = 0xf9u8; cpu.w8(0x4ea2f4u32, v); }
  { let v: u8 = 0xd4u8; cpu.w8(0x4ea331u32, v); }
  { let v: u8 = 0xd4u8; cpu.w8(0x4ea337u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x43f537; continue; }
  pc = 0x43f4f1; continue;
 }
 0x43f4f1 => {
  { let v: u8 = 0x8cu8; cpu.w8(0x4ea225u32, v); }
  { let v: u8 = 0x8cu8; cpu.w8(0x4ea22bu32, v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x4ea268u32, v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x4ea26eu32, v); }
  { let v: u8 = 0xdfu8; cpu.w8(0x4ea2abu32, v); }
  { let v: u8 = 0xdfu8; cpu.w8(0x4ea2b1u32, v); }
  { let v: u8 = 0x54u8; cpu.w8(0x4ea2eeu32, v); }
  { let v: u8 = 0x54u8; cpu.w8(0x4ea2f4u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4ea331u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4ea337u32, v); }
  pc = 0x43f537; continue;
 }
 0x43f537 => {
  { let v: u32 = cpu.r32(0x5a2b45u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5a2e54u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x4ea0a6u32; cpu.edi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f6b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f6b3); }
 let mut pc: u32 = 0x43f6b3;
 loop { match pc {
 0x43f6b3 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e8464u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x43f765; continue; }
  pc = 0x43f6ca; continue;
 }
 0x43f6ca => {
  { let v: u32 = 0x4001u32.wrapping_add(cpu.edi); cpu.eax = v; }
  cpu.push(0x43f6d5); f_0042c9e5(cpu); if cpu.returned_to != 0x43f6d5 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a3d92u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a3d94u32, v); }
  { let v: u32 = 0x520e94u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x43f6f4; continue;
 }
 0x43f6f4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x5a3d92u32); cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x5a3d98u32)); }
  if cpu.zf { pc = 0x43f768; continue; }
  pc = 0x43f704; continue;
 }
 0x43f704 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x43f709; continue;
 }
 0x43f709 => {
  { let v: u16 = cpu.r16(0x5a3d92u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x43f768; continue; }
  pc = 0x43f714; continue;
 }
 0x43f714 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x43f768; continue; }
  pc = 0x43f71a; continue;
 }
 0x43f71a => {
  { let v: u16 = cpu.r16(0x5a3d94u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x43f768; continue; }
  pc = 0x43f727; continue;
 }
 0x43f727 => {
  { cpu.sub16((cpu.ebx as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x43f768; continue; }
  pc = 0x43f72e; continue;
 }
 0x43f72e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  cpu.push(0x43f74b); f_0043f154(cpu); if cpu.returned_to != 0x43f74b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x43f75d; continue; }
  pc = 0x43f74f; continue;
 }
 0x43f74f => {
  { let v: u16 = cpu.r16(0x5a3d94u32).wrapping_sub(0x10u16); cpu.w16(0x5a3d94u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x43f709; continue; }
  pc = 0x43f75b; continue;
 }
 0x43f75b => {
  pc = 0x43f768; continue;
 }
 0x43f75d => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a3d92u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x43f765; continue;
 }
 0x43f765 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f768 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a3d92u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43f6f4; continue; }
  pc = 0x43f77d; continue;
 }
 0x43f77d => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f780); }
 let mut pc: u32 = 0x43f780;
 loop { match pc {
 0x43f780 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x43f78d; continue; }
  pc = 0x43f787; continue;
 }
 0x43f787 => {
  cpu.push(0x43f78c); f_0043f7a4(cpu); if cpu.returned_to != 0x43f78c { return; }
  pc = 0x43f78c; continue;
 }
 0x43f78c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f78d => {
  { let v: u8 = 0xfu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43f78c; continue; }
  pc = 0x43f799; continue;
 }
 0x43f799 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f7a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f7a4); }
 let mut pc: u32 = 0x43f7a4;
 loop { match pc {
 0x43f7a4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x43f7ba); f_0041c743(cpu); if cpu.returned_to != 0x43f7ba { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x43f7da); f_0041c808(cpu); if cpu.returned_to != 0x43f7da { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f7df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f7df); }
 let mut pc: u32 = 0x43f7df;
 loop { match pc {
 0x43f7df => {
  { let v: u8 = cpu.r8(0x4e156au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x43f81d; continue; }
  pc = 0x43f7f2; continue;
 }
 0x43f7f2 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x43f81e; continue; }
  pc = 0x43f7f9; continue;
 }
 0x43f7f9 => {
  cpu.push(0x43f7fe); f_0048990c(cpu); if cpu.returned_to != 0x43f7fe { return; }
  if cpu.cf { pc = 0x43f811; continue; }
  pc = 0x43f800; continue;
 }
 0x43f800 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x43f81d; continue; }
  pc = 0x43f80a; continue;
 }
 0x43f80a => {
  cpu.push(0x43f80f); f_004895be(cpu); if cpu.returned_to != 0x43f80f { return; }
  pc = 0x43f81d; continue;
 }
 0x43f811 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x43f818); f_0045955b(cpu); if cpu.returned_to != 0x43f818 { return; }
  pc = 0x43f818; continue;
 }
 0x43f818 => {
  cpu.push(0x43f81d); f_0043f7a4(cpu); if cpu.returned_to != 0x43f81d { return; }
  pc = 0x43f81d; continue;
 }
 0x43f81d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43f81e => {
  cpu.push(0x43f823); f_0048990c(cpu); if cpu.returned_to != 0x43f823 { return; }
  if cpu.cf { cpu.poll(); pc = 0x43f818; continue; }
  pc = 0x43f825; continue;
 }
 0x43f825 => {
  { let v: u8 = 0x8u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043f830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43f830); }
 let mut pc: u32 = 0x43f830;
 loop { match pc {
 0x43f830 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43f886; continue; }
  pc = 0x43f837; continue;
 }
 0x43f837 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x43fb51; continue; }
  pc = 0x43f847; continue;
 }
 0x43f847 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x43f924; continue; }
  pc = 0x43f851; continue;
 }
 0x43f851 => {
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x43f863; continue; }
  pc = 0x43f858; continue;
 }
 0x43f858 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x43f924; continue; }
  pc = 0x43f863; continue;
 }
 0x43f863 => {
  cpu.push(0x43f868); f_0045bbbe(cpu); if cpu.returned_to != 0x43f868 { return; }
  if cpu.cf { pc = 0x43f924; continue; }
  pc = 0x43f86e; continue;
 }
 0x43f86e => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c72du32, v); }
  cpu.push(0x43f87a); f_0045302a(cpu); if cpu.returned_to != 0x43f87a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c72du32, v); }
  pc = 0x43fb56; continue;
 }
 0x43f886 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x43f8c7; continue; }
  pc = 0x43f88d; continue;
 }
 0x43f88d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x43f924; continue; }
  pc = 0x43f898; continue;
 }
 0x43f898 => {
  { let v: u32 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x5092bcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5092beu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43f8b8); f_00458a0c(cpu); if cpu.returned_to != 0x43f8b8 { return; }
  cpu.push(0x43f8bd); f_004895be(cpu); if cpu.returned_to != 0x43f8bd { return; }
  cpu.push(0x43f8c2); f_0048990c(cpu); if cpu.returned_to != 0x43f8c2 { return; }
  pc = 0x43fb56; continue;
 }
 0x43f8c7 => {
  cpu.push(0x43f8cc); f_0044bc7b(cpu); if cpu.returned_to != 0x43f8cc { return; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x43f904; continue; }
  pc = 0x43f8d8; continue;
 }
 0x43f8d8 => {
  { cpu.sub8(cpu.r8(0x3eu32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x43f904; continue; }
  pc = 0x43f8de; continue;
 }
 0x43f8de => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x43f911; continue; }
  pc = 0x43f8e5; continue;
 }
 0x43f8e5 => {
  cpu.push(0x43f8ea); f_0045bbbe(cpu); if cpu.returned_to != 0x43f8ea { return; }
  if cpu.cf { pc = 0x43f904; continue; }
  pc = 0x43f8ec; continue;
 }
 0x43f8ec => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c72du32, v); }
  cpu.push(0x43f8f8); f_0045302a(cpu); if cpu.returned_to != 0x43f8f8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c72du32, v); }
  pc = 0x43fb56; continue;
 }
 0x43f904 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x43f924; continue; }
  pc = 0x43f90b; continue;
 }
 0x43f90b => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  pc = 0x43f911; continue;
 }
 0x43f911 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x43f924; continue; }
  pc = 0x43f917; continue;
 }
 0x43f917 => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x43f91f); f_004895e3(cpu); if cpu.returned_to != 0x43f91f { return; }
  pc = 0x43fb56; continue;
 }
 0x43f924 => {
  { let v: u8 = 0x4u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x40u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x43fa1f; continue; }
  pc = 0x43f937; continue;
 }
 0x43f937 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x43fb3f; continue; }
  pc = 0x43f944; continue;
 }
 0x43f944 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x43f953); f_00451950(cpu); if cpu.returned_to != 0x43f953 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x40u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x43fa1b; continue; }
  pc = 0x43f962; continue;
 }
 0x43f962 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x43f99d; continue; }
  pc = 0x43f969; continue;
 }
 0x43f969 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x43f99d; continue; }
  pc = 0x43f970; continue;
 }
 0x43f970 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x43f978); f_0043ff77(cpu); if cpu.returned_to != 0x43f978 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x43f99d; continue; }
  pc = 0x43f97e; continue;
 }
 0x43f97e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x43f99d; continue; }
  pc = 0x43f986; continue;
 }
 0x43f986 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43f98c); f_0043fb57(cpu); if cpu.returned_to != 0x43f98c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x43f99d; continue; }
  pc = 0x43f98f; continue;
 }
 0x43f98f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43f995); f_00458731(cpu); if cpu.returned_to != 0x43f995 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x43f99d; continue; }
  pc = 0x43f998; continue;
 }
 0x43f998 => {
  pc = 0x43fb56; continue;
 }
 0x43f99d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x43f9a5); f_004404a3(cpu); if cpu.returned_to != 0x43f9a5 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x43f9ca; continue; }
  pc = 0x43f9aa; continue;
 }
 0x43f9aa => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x43f9ca; continue; }
  pc = 0x43f9b2; continue;
 }
 0x43f9b2 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5293a0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x43f9c6); f_0041a62f(cpu); if cpu.returned_to != 0x43f9c6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x43fa16; continue;
 }
 0x43f9ca => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x5a3d9eu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x43f9f0); f_0045955b(cpu); if cpu.returned_to != 0x43f9f0 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x43f9f6); f_004895c7(cpu); if cpu.returned_to != 0x43f9f6 { return; }
  cpu.push(0x43f9fb); f_00489637(cpu); if cpu.returned_to != 0x43f9fb { return; }
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x19u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43fa05; continue; }
  pc = 0x43fa04; continue;
 }
 0x43fa04 => {
  {  }
  pc = 0x43fa05; continue;
 }
 0x43fa05 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x43fa16; continue; }
  pc = 0x43fa0c; continue;
 }
 0x43fa0c => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x43fa16; continue;
 }
 0x43fa16 => {
  pc = 0x43fb56; continue;
 }
 0x43fa1b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43fa8d; continue;
 }
 0x43fa1f => {
  { let v: u8 = 0xffu8; cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x43fa30; continue; }
  pc = 0x43fa2a; continue;
 }
 0x43fa2a => {
  if !cpu.zf { pc = 0x43fb3f; continue; }
  pc = 0x43fa30; continue;
 }
 0x43fa30 => {
  { let v: u8 = 0x7u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.ebp))); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x43fa7e; continue; }
  pc = 0x43fa3b; continue;
 }
 0x43fa3b => {
  { let v: u8 = 0x7u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x43fa4a); f_0045bbfc(cpu); if cpu.returned_to != 0x43fa4a { return; }
  if cpu.cf { pc = 0x43fa7e; continue; }
  pc = 0x43fa4c; continue;
 }
 0x43fa4c => {
  cpu.push(0x43fa51); f_0045bbbe(cpu); if cpu.returned_to != 0x43fa51 { return; }
  if !cpu.cf { pc = 0x43fa65; continue; }
  pc = 0x43fa53; continue;
 }
 0x43fa53 => {
  cpu.push(0x43fa58); f_0041c743(cpu); if cpu.returned_to != 0x43fa58 { return; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x43fa65; continue; }
  pc = 0x43fa5c; continue;
 }
 0x43fa5c => {
  cpu.push(0x43fa61); f_0041c743(cpu); if cpu.returned_to != 0x43fa61 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43fa6d; continue;
 }
 0x43fa65 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43fa6d; continue;
 }
 0x43fa6d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x43fa76); f_0041c743(cpu); if cpu.returned_to != 0x43fa76 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  pc = 0x43fa7e; continue;
 }
 0x43fa7e => {
  cpu.push(0x43fa83); f_0041c743(cpu); if cpu.returned_to != 0x43fa83 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x43fa8d; continue;
 }
 0x43fa8d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x43facd; continue; }
  pc = 0x43fa94; continue;
 }
 0x43fa94 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x43faa2; continue; }
  pc = 0x43fa9b; continue;
 }
 0x43fa9b => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x43facd; continue;
 }
 0x43faa2 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43faa8); f_0043ff77(cpu); if cpu.returned_to != 0x43faa8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x43facd; continue; }
  pc = 0x43faae; continue;
 }
 0x43faae => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x43facd; continue; }
  pc = 0x43fab6; continue;
 }
 0x43fab6 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43fabc); f_0043fb57(cpu); if cpu.returned_to != 0x43fabc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x43facd; continue; }
  pc = 0x43fabf; continue;
 }
 0x43fabf => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43fac5); f_00458731(cpu); if cpu.returned_to != 0x43fac5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x43facd; continue; }
  pc = 0x43fac8; continue;
 }
 0x43fac8 => {
  pc = 0x43fb56; continue;
 }
 0x43facd => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x43fadc; continue; }
  pc = 0x43fad6; continue;
 }
 0x43fad6 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  pc = 0x43fadc; continue;
 }
 0x43fadc => {
  cpu.push(0x43fae1); f_0043fede(cpu); if cpu.returned_to != 0x43fae1 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x43fb41; continue; }
  pc = 0x43fae6; continue;
 }
 0x43fae6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x43fb41; continue; }
  pc = 0x43faee; continue;
 }
 0x43faee => {
  cpu.push(0x43faf3); f_004782f8(cpu); if cpu.returned_to != 0x43faf3 { return; }
  if cpu.cf { pc = 0x43fb41; continue; }
  pc = 0x43faf5; continue;
 }
 0x43faf5 => {
  cpu.push(0x43fafa); f_0047a472(cpu); if cpu.returned_to != 0x43fafa { return; }
  if cpu.cf { pc = 0x43fb41; continue; }
  pc = 0x43fafc; continue;
 }
 0x43fafc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x43fb0a; continue; }
  pc = 0x43fb03; continue;
 }
 0x43fb03 => {
  cpu.push(0x43fb08); f_004525bf(cpu); if cpu.returned_to != 0x43fb08 { return; }
  pc = 0x43fb56; continue;
 }
 0x43fb0a => {
  cpu.push(0x43fb0f); f_0047a294(cpu); if cpu.returned_to != 0x43fb0f { return; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5092beu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5092bcu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x43fb31); f_0045955b(cpu); if cpu.returned_to != 0x43fb31 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x43fb37); f_004895c7(cpu); if cpu.returned_to != 0x43fb37 { return; }
  cpu.push(0x43fb3c); f_00489637(cpu); if cpu.returned_to != 0x43fb3c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x43fb3f; continue;
 }
 0x43fb3f => {
  pc = 0x43fb56; continue;
 }
 0x43fb41 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x3u8); }
  if !cpu.zf { pc = 0x43fb51; continue; }
  pc = 0x43fb4a; continue;
 }
 0x43fb4a => {
  cpu.push(0x43fb4f); f_0043fb7a(cpu); if cpu.returned_to != 0x43fb4f { return; }
  if cpu.cf { pc = 0x43fb56; continue; }
  pc = 0x43fb51; continue;
 }
 0x43fb51 => {
  cpu.push(0x43fb56); f_0043fced(cpu); if cpu.returned_to != 0x43fb56 { return; }
  pc = 0x43fb56; continue;
 }
 0x43fb56 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fb57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fb57); }
 let mut pc: u32 = 0x43fb57;
 loop { match pc {
 0x43fb57 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x43fb76; continue; }
  pc = 0x43fb5e; continue;
 }
 0x43fb5e => {
  cpu.push(0x43fb63); f_0045bf77(cpu); if cpu.returned_to != 0x43fb63 { return; }
  if !cpu.cf { pc = 0x43fb6d; continue; }
  pc = 0x43fb65; continue;
 }
 0x43fb65 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x43fb78; continue; }
  pc = 0x43fb69; continue;
 }
 0x43fb69 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x43fb76; continue; }
  pc = 0x43fb6d; continue;
 }
 0x43fb6d => {
  { cpu.sub8(((cpu.edx >> 8) as u8), cpu.r8(0x99u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x43fb78; continue; }
  pc = 0x43fb76; continue;
 }
 0x43fb76 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fb78 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fb7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fb7a); }
 let mut pc: u32 = 0x43fb7a;
 loop { match pc {
 0x43fb7a => {
  { cpu.sub16(cpu.r16(0x4d69bau32), 0x0u16); }
  if cpu.zf { pc = 0x43fceb; continue; }
  pc = 0x43fb88; continue;
 }
 0x43fb88 => {
  { cpu.sub16(cpu.r16(0x5a2b38u32), 0xffffu16); }
  if cpu.zf { pc = 0x43fceb; continue; }
  pc = 0x43fb97; continue;
 }
 0x43fb97 => {
  { let v: u32 = 0x52ae94u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a3d9cu32, v); }
  pc = 0x43fba8; continue;
 }
 0x43fba8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x43fcd5; continue; }
  pc = 0x43fbb7; continue;
 }
 0x43fbb7 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0xb8u8); }
  if !cpu.zf { pc = 0x43fcd5; continue; }
  pc = 0x43fbc1; continue;
 }
 0x43fbc1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x18u32); cpu.edi = v; }
  cpu.push(0x43fbef); f_004714a5(cpu); if cpu.returned_to != 0x43fbef { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x43fcd5; continue; }
  pc = 0x43fbf7; continue;
 }
 0x43fbf7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x43fc1d; continue; }
  pc = 0x43fc1a; continue;
 }
 0x43fc1a => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x43fc1d; continue;
 }
 0x43fc1d => {
  { cpu.sub16((cpu.eax as u16), 0x6u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x43fc46; continue; }
  pc = 0x43fc23; continue;
 }
 0x43fc23 => {
  { cpu.sub16((cpu.ebx as u16), 0x6u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x43fc46; continue; }
  pc = 0x43fc29; continue;
 }
 0x43fc29 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x80u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x10u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x43fc45); f_00483023(cpu); if cpu.returned_to != 0x43fc45 { return; }
  { cpu.popad(); }
  pc = 0x43fc46; continue;
 }
 0x43fc46 => {
  cpu.push(0x43fc4b); f_0045ac41(cpu); if cpu.returned_to != 0x43fc4b { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43fc5a); f_004679cb(cpu); if cpu.returned_to != 0x43fc5a { return; }
  if cpu.cf { pc = 0x43fcce; continue; }
  pc = 0x43fc5c; continue;
 }
 0x43fc5c => {
  { cpu.sub8(cpu.r8(0x4eab2bu32), 0x0u8); }
  if !cpu.zf { pc = 0x43fcce; continue; }
  pc = 0x43fc65; continue;
 }
 0x43fc65 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x43fc8b; continue; }
  pc = 0x43fc7b; continue;
 }
 0x43fc7b => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43fc82); f_00401130(cpu); if cpu.returned_to != 0x43fc82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x43fc8b; continue;
 }
 0x43fc8b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2bu32, v); }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x43fc9d); f_00415d61(cpu); if cpu.returned_to != 0x43fc9d { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x79u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x43fcc3; continue; }
  pc = 0x43fcb3; continue;
 }
 0x43fcb3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43fcba); f_00401130(cpu); if cpu.returned_to != 0x43fcba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x43fcc3; continue;
 }
 0x43fcc3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x43fcce); f_00415d61(cpu); if cpu.returned_to != 0x43fcce { return; }
  pc = 0x43fcce; continue;
 }
 0x43fcce => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a3d9cu32, v); }
  pc = 0x43fcd5; continue;
 }
 0x43fcd5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43fba8; continue; }
  pc = 0x43fce3; continue;
 }
 0x43fce3 => {
  { cpu.sub8(cpu.r8(0x5a3d9cu32), 0x0u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fceb => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fced(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fced); }
 let mut pc: u32 = 0x43fced;
 loop { match pc {
 0x43fced => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x43fd4c; continue; }
  pc = 0x43fcf4; continue;
 }
 0x43fcf4 => {
  cpu.push(0x43fcf9); f_004899f2(cpu); if cpu.returned_to != 0x43fcf9 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x43fd03; continue; }
  pc = 0x43fcfd; continue;
 }
 0x43fcfd => {
  cpu.push(0x43fd02); f_0048990c(cpu); if cpu.returned_to != 0x43fd02 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fd03 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x43fd4c; continue; }
  pc = 0x43fd0f; continue;
 }
 0x43fd0f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x43fd15); f_004895be(cpu); if cpu.returned_to != 0x43fd15 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x43fd21; continue; }
  pc = 0x43fd1c; continue;
 }
 0x43fd1c => {
  cpu.push(0x43fd21); f_004895d0(cpu); if cpu.returned_to != 0x43fd21 { return; }
  pc = 0x43fd21; continue;
 }
 0x43fd21 => {
  cpu.push(0x43fd26); f_0043fd98(cpu); if cpu.returned_to != 0x43fd26 { return; }
  if cpu.cf { pc = 0x43fd4c; continue; }
  pc = 0x43fd28; continue;
 }
 0x43fd28 => {
  { cpu.sub8(cpu.r8(0x5a3d9du32), 0x0u8); }
  if !cpu.zf { pc = 0x43fd53; continue; }
  pc = 0x43fd31; continue;
 }
 0x43fd31 => {
  cpu.push(0x43fd36); f_0043fd68(cpu); if cpu.returned_to != 0x43fd36 { return; }
  if cpu.cf { pc = 0x43fd53; continue; }
  pc = 0x43fd38; continue;
 }
 0x43fd38 => {
  cpu.push(0x43fd3d); f_00489637(cpu); if cpu.returned_to != 0x43fd3d { return; }
  { let v: u8 = 0x7u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x43fd4d; continue; }
  pc = 0x43fd44; continue;
 }
 0x43fd44 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43fd4d; continue; }
  pc = 0x43fd4c; continue;
 }
 0x43fd4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fd4d => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fd53 => {
  cpu.push(0x43fd58); f_00458a0c(cpu); if cpu.returned_to != 0x43fd58 { return; }
  cpu.push(0x43fd5d); f_0048990c(cpu); if cpu.returned_to != 0x43fd5d { return; }
  cpu.push(0x43fd62); f_004895be(cpu); if cpu.returned_to != 0x43fd62 { return; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fd68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fd68); }
 let mut pc: u32 = 0x43fd68;
 loop { match pc {
 0x43fd68 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x400u32); }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fd98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fd98); }
 let mut pc: u32 = 0x43fd98;
 loop { match pc {
 0x43fd98 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x43fed6; continue; }
  pc = 0x43fda3; continue;
 }
 0x43fda3 => {
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x43fed6; continue; }
  pc = 0x43fdaf; continue;
 }
 0x43fdaf => {
  if cpu.sf { pc = 0x43fed6; continue; }
  pc = 0x43fdb5; continue;
 }
 0x43fdb5 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4b8de4u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x5a3d9du32), 0x0u8); }
  if !cpu.zf { pc = 0x43fe45; continue; }
  pc = 0x43fdd8; continue;
 }
 0x43fdd8 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x43fe3b; continue; }
  pc = 0x43fde2; continue;
 }
 0x43fde2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x38u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x43fe35; continue; }
  pc = 0x43fdfc; continue;
 }
 0x43fdfc => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x43fe35; continue; }
  pc = 0x43fe03; continue;
 }
 0x43fe03 => {
  { cpu.pushad(); }
  cpu.push(0x43fe09); f_00488be6(cpu); if cpu.returned_to != 0x43fe09 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x43fe18; continue; }
  pc = 0x43fe0c; continue;
 }
 0x43fe0c => {
  { cpu.pushad(); }
  cpu.push(0x43fe12); f_0045c2d0(cpu); if cpu.returned_to != 0x43fe12 { return; }
  cpu.push(0x43fe17); f_0045c294(cpu); if cpu.returned_to != 0x43fe17 { return; }
  { cpu.popad(); }
  pc = 0x43fe18; continue;
 }
 0x43fe18 => {
  pc = 0x43fed3; continue;
 }
 0x43fe35 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x43fed1; continue;
 }
 0x43fe3b => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x43fed6; continue; }
  pc = 0x43fe45; continue;
 }
 0x43fe45 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6adcu32)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x43fe7a); f_0045bde8(cpu); if cpu.returned_to != 0x43fe7a { return; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x43fe84); f_0045bde8(cpu); if cpu.returned_to != 0x43fe84 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x43fe94); f_0045bde8(cpu); if cpu.returned_to != 0x43fe94 { return; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x43fea2); f_0045bde8(cpu); if cpu.returned_to != 0x43fea2 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5092beu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5092bcu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x43fed1; continue;
 }
 0x43fed1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fed3 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fed6 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043fede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43fede); }
 let mut pc: u32 = 0x43fede;
 loop { match pc {
 0x43fede => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x43fee8; continue; }
  pc = 0x43fee5; continue;
 }
 0x43fee5 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43fee8 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x43ff03; continue; }
  pc = 0x43fef2; continue;
 }
 0x43fef2 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x43ff03; continue; }
  pc = 0x43fefc; continue;
 }
 0x43fefc => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x43ff06; continue; }
  pc = 0x43ff03; continue;
 }
 0x43ff03 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43ff06 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x20u8); }
  if !cpu.zf { cpu.poll(); pc = 0x43ff03; continue; }
  pc = 0x43ff12; continue;
 }
 0x43ff12 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x43ff3e; continue; }
  pc = 0x43ff1a; continue;
 }
 0x43ff1a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x43ff3e; continue; }
  pc = 0x43ff21; continue;
 }
 0x43ff21 => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) ^ 0x8u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x43ff3e; continue; }
  pc = 0x43ff2d; continue;
 }
 0x43ff2d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x74u32.wrapping_add(cpu.ebp))); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43ff45; continue;
 }
 0x43ff3e => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  pc = 0x43ff45; continue;
 }
 0x43ff45 => {
  { let v: u32 = cpu.r32(0x4d7194u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x43ff59); f_004400c4(cpu); if cpu.returned_to != 0x43ff59 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d71b4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b50u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x43ff6d); f_00440304(cpu); if cpu.returned_to != 0x43ff6d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.and8((cpu.edx as u8), 0x80u8); }
  if cpu.zf { pc = 0x43ff75; continue; }
  pc = 0x43ff73; continue;
 }
 0x43ff73 => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x43ff75; continue;
 }
 0x43ff75 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ff77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ff77); }
 let mut pc: u32 = 0x43ff77;
 loop { match pc {
 0x43ff77 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x43ff88; continue; }
  pc = 0x43ff81; continue;
 }
 0x43ff81 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x43ff8b; continue; }
  pc = 0x43ff88; continue;
 }
 0x43ff88 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43ff8b => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) ^ 0x40u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x43ffb4; continue; }
  pc = 0x43ff9a; continue;
 }
 0x43ff9a => {
  { let v: u8 = cpu.inc8(cpu.r8(0xa0u32.wrapping_add(cpu.ebp))); cpu.w8(0xa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0xa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43ffbb; continue;
 }
 0x43ffb4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  pc = 0x43ffbb; continue;
 }
 0x43ffbb => {
  { let v: u32 = cpu.r32(0x4d7194u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43ffc8); f_00440119(cpu); if cpu.returned_to != 0x43ffc8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d71d4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b50u32, v); }
  cpu.push(0x43ffda); f_00440173(cpu); if cpu.returned_to != 0x43ffda { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ffdb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43ffdb); } r_0043ffdb(cpu, 0x43ffdb); }
pub fn f_00440000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440000); } r_0043ffdb(cpu, 0x440000); }
pub fn f_00440010(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440010); } r_0043ffdb(cpu, 0x440010); }
pub fn f_00440017(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440017); } r_0043ffdb(cpu, 0x440017); }
pub fn f_0044001d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44001d); } r_0043ffdb(cpu, 0x44001d); }
pub fn f_0044001e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44001e); } r_0043ffdb(cpu, 0x44001e); }
pub fn f_0044001f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44001f); } r_0043ffdb(cpu, 0x44001f); }
pub fn f_00440020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440020); } r_0043ffdb(cpu, 0x440020); }
pub fn f_0044002c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44002c); } r_0043ffdb(cpu, 0x44002c); }
pub fn f_00440041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440041); } r_0043ffdb(cpu, 0x440041); }
pub fn f_00440043(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440043); } r_0043ffdb(cpu, 0x440043); }
pub fn f_00440044(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440044); } r_0043ffdb(cpu, 0x440044); }
pub fn f_00440049(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440049); } r_0043ffdb(cpu, 0x440049); }
pub fn f_0044005b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44005b); } r_0043ffdb(cpu, 0x44005b); }
pub fn f_0044005d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44005d); } r_0043ffdb(cpu, 0x44005d); }
pub fn f_00440065(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440065); } r_0043ffdb(cpu, 0x440065); }
pub fn f_00440080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440080); } r_0043ffdb(cpu, 0x440080); }
pub fn f_00440082(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440082); } r_0043ffdb(cpu, 0x440082); }
pub fn f_004400a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4400a3); } r_0043ffdb(cpu, 0x4400a3); }
pub fn f_004400a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4400a8); } r_0043ffdb(cpu, 0x4400a8); }
pub fn f_004400b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4400b0); } r_0043ffdb(cpu, 0x4400b0); }
fn r_0043ffdb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43ffdb => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x43fff4; continue; }
  pc = 0x43fff0; continue;
 }
 0x43fff0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  pc = 0x43fff4; continue;
 }
 0x43fff4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x440040; continue; }
  pc = 0x43fffb; continue;
 }
 0x43fffb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  pc = 0x440000; continue;
 }
 0x440000 => {
  if cpu.zf { pc = 0x4400b1; continue; }
  pc = 0x440006; continue;
 }
 0x440006 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x440017; continue;
 }
 0x440010 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x440017; continue;
 }
 0x440017 => {
  cpu.push(0x44001c); f_0041c743(cpu); if cpu.returned_to != 0x44001c { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44001e; continue;
 }
 0x44001d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.edi = v; }
  pc = 0x44001f; continue;
 }
 0x44001e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x44003c); f_0041c808(cpu); if cpu.returned_to != 0x44003c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x440040; continue;
 }
 0x44001f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x44003c); f_0041c808(cpu); if cpu.returned_to != 0x44003c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x440040; continue;
 }
 0x440020 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x44003c); f_0041c808(cpu); if cpu.returned_to != 0x44003c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x440040; continue;
 }
 0x44002c => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xffffffbbu32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffbbu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x44003c); f_0041c808(cpu); if cpu.returned_to != 0x44003c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x440040; continue;
 }
 0x440040 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4400b1; continue; }
  pc = 0x440047; continue;
 }
 0x440041 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4400b1; continue; }
  pc = 0x440047; continue;
 }
 0x440043 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x440044; continue;
 }
 0x440044 => {
  { let v: u8 = cpu.r8(0x6au32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x6au32.wrapping_add(cpu.ecx), v); }
  pc = 0x440047; continue;
 }
 0x440047 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if cpu.zf { pc = 0x44006a; continue; }
  pc = 0x44004e; continue;
 }
 0x440049 => {
  if (cpu.sf == cpu.of) { pc = 0x4400a5; continue; }
  pc = 0x44004b; continue;
 }
 0x44004b => {
  { cpu.interrupts_off = false; }
  if cpu.zf { pc = 0x44006a; continue; }
  pc = 0x44004e; continue;
 }
 0x44004e => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4400b1; continue; }
  pc = 0x440055; continue;
 }
 0x440055 => {
  cpu.push(0x44005a); f_0048990c(cpu); if cpu.returned_to != 0x44005a { return; }
  if !cpu.cf { pc = 0x4400b1; continue; }
  pc = 0x44005c; continue;
 }
 0x44005b => {
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x44005c; continue;
 }
 0x44005c => {
  cpu.push(0x440061); f_004899f2(cpu); if cpu.returned_to != 0x440061 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4400b1; continue; }
  pc = 0x440065; continue;
 }
 0x44005d => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4400b1; continue; }
  pc = 0x440065; continue;
 }
 0x440065 => {
  cpu.push(0x44006a); f_0048990c(cpu); if cpu.returned_to != 0x44006a { return; }
  pc = 0x44006a; continue;
 }
 0x44006a => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44007d; continue; }
  pc = 0x440077; continue;
 }
 0x440077 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44007d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44008c; continue; }
  pc = 0x440086; continue;
 }
 0x440080 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x7500004eu32); cpu.eax = v; }
  { cpu.push(0x2b); }
  pc = 0x440086; continue;
 }
 0x440082 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x44008c; continue; }
  pc = 0x440086; continue;
 }
 0x440086 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  pc = 0x44008c; continue;
 }
 0x44008c => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x440093); f_0043fede(cpu); if cpu.returned_to != 0x440093 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x4400ad; continue; }
  pc = 0x440098; continue;
 }
 0x440098 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4400ad; continue; }
  pc = 0x4400a0; continue;
 }
 0x4400a0 => {
  cpu.push(0x4400a5); f_004782f8(cpu); if cpu.returned_to != 0x4400a5 { return; }
  pc = 0x4400a5; continue;
 }
 0x4400a3 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  pc = 0x4400a5; continue;
 }
 0x4400a5 => {
  if cpu.cf { pc = 0x4400ad; continue; }
  pc = 0x4400a7; continue;
 }
 0x4400a7 => {
  cpu.push(0x4400ac); f_004525bf(cpu); if cpu.returned_to != 0x4400ac { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4400a8 => {
  { let v: u32 = cpu.adc32(cpu.esp, cpu.r32(0x3ec30001u32)); cpu.esp = v; }
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x4400b1; continue;
 }
 0x4400ad => {
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x4400b1; continue;
 }
 0x4400b0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x4400b1; continue;
 }
 0x4400b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440015(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440015); }
 let mut pc: u32 = 0x440015;
 loop { match pc {
 0x440015 => {
  { cpu.sub32(cpu.eax, 0xc727e808u32); }
  { cpu.df = true; }
  cpu.poll(); pc = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004400bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4400bf); }
 let mut pc: u32 = 0x4400bf;
 loop { match pc {
 0x4400bf => {
  { let v: u32 = 0x4d73f4u32; cpu.esi = v; }
  f_004400c4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004400c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4400c4); }
 let mut pc: u32 = 0x4400c4;
 loop { match pc {
 0x4400c4 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x55755du32; cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4400d9; continue;
 }
 0x4400d9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x44010d; continue; }
  pc = 0x4400e6; continue;
 }
 0x4400e6 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x44010d; continue; }
  pc = 0x4400ee; continue;
 }
 0x4400ee => {
  { let v: u8 = cpu.r8(0x55dcb8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44010d; continue; }
  pc = 0x4400f8; continue;
 }
 0x4400f8 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x55bcb8u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44010d; continue; }
  pc = 0x44010b; continue;
 }
 0x44010b => {
  { cpu.movs(1, true); }
  pc = 0x44010d; continue;
 }
 0x44010d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4400d9; continue; }
  pc = 0x440114; continue;
 }
 0x440114 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440119(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440119); }
 let mut pc: u32 = 0x440119;
 loop { match pc {
 0x440119 => {
  { let v: u32 = 0x55755du32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x44012e; continue;
 }
 0x44012e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x440167; continue; }
  pc = 0x44013b; continue;
 }
 0x44013b => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x440167; continue; }
  pc = 0x440143; continue;
 }
 0x440143 => {
  { let v: u32 = 0x55ddb8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x55fdb8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x440167; continue; }
  pc = 0x440152; continue;
 }
 0x440152 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x55ddb8u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x440167; continue; }
  pc = 0x440165; continue;
 }
 0x440165 => {
  { cpu.movs(1, true); }
  pc = 0x440167; continue;
 }
 0x440167 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44012e; continue; }
  pc = 0x44016e; continue;
 }
 0x44016e => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440173(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440173); } r_00440173(cpu, 0x440173); }
pub fn f_00440259(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440259); } r_00440173(cpu, 0x440259); }
pub fn f_0044026e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44026e); } r_00440173(cpu, 0x44026e); }
pub fn f_00440283(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440283); } r_00440173(cpu, 0x440283); }
pub fn f_00440298(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440298); } r_00440173(cpu, 0x440298); }
pub fn f_004402b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4402b0); } r_00440173(cpu, 0x4402b0); }
pub fn f_004402c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4402c5); } r_00440173(cpu, 0x4402c5); }
pub fn f_004402da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4402da); } r_00440173(cpu, 0x4402da); }
pub fn f_004402ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4402ef); } r_00440173(cpu, 0x4402ef); }
fn r_00440173(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x440173 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e156bu32, v); }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44018c; continue;
 }
 0x44018c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x440256; continue; }
  pc = 0x440195; continue;
 }
 0x440195 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4401d8; continue; }
  pc = 0x4401ad; continue;
 }
 0x4401ad => {
  { let v: u32 = 0x4b8658u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  if !cpu.zf { pc = 0x4401d8; continue; }
  pc = 0x4401b7; continue;
 }
 0x4401b7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4401d8; continue; }
  pc = 0x4401c2; continue;
 }
 0x4401c2 => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x4401d8; continue; }
  pc = 0x4401d5; continue;
 }
 0x4401d5 => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x44018c; continue;
 }
 0x4401d8 => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x4e156bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4401e3; continue;
 }
 0x4401e3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x4d6b50u32); continue;
 }
 0x4401fa => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if (cpu.sf == cpu.of) { pc = 0x440252; continue; }
  pc = 0x440223; continue;
 }
 0x440223 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44022b; continue;
 }
 0x44022b => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x440233; continue;
 }
 0x440233 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x440241); f_00489d13(cpu); if cpu.returned_to != 0x440241 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x440248; continue;
 }
 0x440248 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x44018c; continue;
 }
 0x440252 => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x44022b; continue;
 }
 0x440256 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x440259 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440263; continue; }
  pc = 0x440260; continue;
 }
 0x440260 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x440263; continue;
 }
 0x440263 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x44026c; continue;
 }
 0x44026c => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x44026e => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440275; continue; }
  pc = 0x440272; continue;
 }
 0x440272 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x440275; continue;
 }
 0x440275 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x44027e; continue;
 }
 0x44027e => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x440283 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x44028a; continue; }
  pc = 0x440287; continue;
 }
 0x440287 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44028a; continue;
 }
 0x44028a => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x440293; continue;
 }
 0x440293 => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x440298 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x4402a2; continue; }
  pc = 0x44029f; continue;
 }
 0x44029f => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4402a2; continue;
 }
 0x4402a2 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402ab; continue;
 }
 0x4402ab => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x4402b0 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402b8; continue;
 }
 0x4402b8 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402c0; continue;
 }
 0x4402c0 => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x4402c5 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402cd; continue;
 }
 0x4402cd => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402d5; continue;
 }
 0x4402d5 => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x4402da => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402e2; continue;
 }
 0x4402e2 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402ea; continue;
 }
 0x4402ea => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 0x4402ef => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402f7; continue;
 }
 0x4402f7 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44018c; continue; }
  pc = 0x4402ff; continue;
 }
 0x4402ff => {
  cpu.poll(); pc = 0x4401fa; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440304(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440304); } r_00440304(cpu, 0x440304); }
pub fn f_004403f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4403f8); } r_00440304(cpu, 0x4403f8); }
pub fn f_0044040d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44040d); } r_00440304(cpu, 0x44040d); }
pub fn f_00440422(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440422); } r_00440304(cpu, 0x440422); }
pub fn f_00440437(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440437); } r_00440304(cpu, 0x440437); }
pub fn f_0044044f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44044f); } r_00440304(cpu, 0x44044f); }
pub fn f_00440464(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440464); } r_00440304(cpu, 0x440464); }
pub fn f_00440479(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440479); } r_00440304(cpu, 0x440479); }
pub fn f_0044048e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44048e); } r_00440304(cpu, 0x44048e); }
fn r_00440304(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x440304 => {
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44030d; continue;
 }
 0x44030d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4403f5; continue; }
  pc = 0x440316; continue;
 }
 0x440316 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440334; continue;
 }
 0x440334 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44035c; continue; }
  pc = 0x44033d; continue;
 }
 0x44033d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44035c; continue;
 }
 0x44035c => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440365; continue;
 }
 0x440365 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44036b; continue;
 }
 0x44036b => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440375; continue;
 }
 0x440375 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44037c; continue;
 }
 0x44037c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440382; continue;
 }
 0x440382 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x4d6b50u32); continue;
 }
 0x44039a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if (cpu.sf == cpu.of) { pc = 0x4403f1; continue; }
  pc = 0x4403c2; continue;
 }
 0x4403c2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4403ca; continue;
 }
 0x4403ca => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x4403d2; continue;
 }
 0x4403d2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x4403e0); f_00489e05(cpu); if cpu.returned_to != 0x4403e0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x4403e7; continue;
 }
 0x4403e7 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x44030d; continue;
 }
 0x4403f1 => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x4403ca; continue;
 }
 0x4403f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4403f8 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440402; continue; }
  pc = 0x4403ff; continue;
 }
 0x4403ff => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x440402; continue;
 }
 0x440402 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44040b; continue;
 }
 0x44040b => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x44040d => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440414; continue; }
  pc = 0x440411; continue;
 }
 0x440411 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x440414; continue;
 }
 0x440414 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44041d; continue;
 }
 0x44041d => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x440422 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440429; continue; }
  pc = 0x440426; continue;
 }
 0x440426 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x440429; continue;
 }
 0x440429 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440432; continue;
 }
 0x440432 => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x440437 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440441; continue; }
  pc = 0x44043e; continue;
 }
 0x44043e => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x440441; continue;
 }
 0x440441 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44044a; continue;
 }
 0x44044a => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x44044f => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440457; continue;
 }
 0x440457 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44045f; continue;
 }
 0x44045f => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x440464 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44046c; continue;
 }
 0x44046c => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440474; continue;
 }
 0x440474 => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x440479 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440481; continue;
 }
 0x440481 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440489; continue;
 }
 0x440489 => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 0x44048e => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x440496; continue;
 }
 0x440496 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x44030d; continue; }
  pc = 0x44049e; continue;
 }
 0x44049e => {
  cpu.poll(); pc = 0x44039a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004404a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4404a3); }
 let mut pc: u32 = 0x4404a3;
 loop { match pc {
 0x4404a3 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4404b4; continue; }
  pc = 0x4404ad; continue;
 }
 0x4404ad => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x4404b7; continue; }
  pc = 0x4404b4; continue;
 }
 0x4404b4 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4404b7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d7194u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4404cb); f_004404de(cpu); if cpu.returned_to != 0x4404cb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d71f4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b50u32, v); }
  cpu.push(0x4404dd); f_00440538(cpu); if cpu.returned_to != 0x4404dd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004404de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4404de); }
 let mut pc: u32 = 0x4404de;
 loop { match pc {
 0x4404de => {
  { let v: u32 = 0x55755du32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4404f3; continue;
 }
 0x4404f3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x44052c; continue; }
  pc = 0x440500; continue;
 }
 0x440500 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x44052c; continue; }
  pc = 0x440508; continue;
 }
 0x440508 => {
  { let v: u32 = 0x55feb8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x561eb8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44052c; continue; }
  pc = 0x440517; continue;
 }
 0x440517 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x55feb8u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44052c; continue; }
  pc = 0x44052a; continue;
 }
 0x44052a => {
  { cpu.movs(1, true); }
  pc = 0x44052c; continue;
 }
 0x44052c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4404f3; continue; }
  pc = 0x440533; continue;
 }
 0x440533 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440538(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440538); } r_00440538(cpu, 0x440538); }
pub fn f_00440601(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440601); } r_00440538(cpu, 0x440601); }
pub fn f_00440616(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440616); } r_00440538(cpu, 0x440616); }
pub fn f_0044062b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44062b); } r_00440538(cpu, 0x44062b); }
pub fn f_00440640(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440640); } r_00440538(cpu, 0x440640); }
pub fn f_00440658(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440658); } r_00440538(cpu, 0x440658); }
pub fn f_0044066d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44066d); } r_00440538(cpu, 0x44066d); }
pub fn f_00440682(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440682); } r_00440538(cpu, 0x440682); }
pub fn f_00440697(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440697); } r_00440538(cpu, 0x440697); }
fn r_00440538(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x440538 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e156bu32, v); }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x440551; continue;
 }
 0x440551 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4405fe; continue; }
  pc = 0x44055a; continue;
 }
 0x44055a => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x5293a0u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e156bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0xau8); }
  if !cpu.zf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440573; continue;
 }
 0x440573 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.edi)), 0x34u8); }
  if !cpu.zf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440579; continue;
 }
 0x440579 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44057f; continue;
 }
 0x44057f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x6u8); }
  if cpu.zf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440585; continue;
 }
 0x440585 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44058b; continue;
 }
 0x44058b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x4d6b50u32); continue;
 }
 0x4405a2 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if (cpu.sf == cpu.of) { pc = 0x4405fa; continue; }
  pc = 0x4405cb; continue;
 }
 0x4405cb => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4405d3; continue;
 }
 0x4405d3 => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x4405db; continue;
 }
 0x4405db => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x4405e9); f_00489d8b(cpu); if cpu.returned_to != 0x4405e9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x4405f0; continue;
 }
 0x4405f0 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x440551; continue;
 }
 0x4405fa => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x4405d3; continue;
 }
 0x4405fe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x440601 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x44060b; continue; }
  pc = 0x440608; continue;
 }
 0x440608 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44060b; continue;
 }
 0x44060b => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440614; continue;
 }
 0x440614 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x440616 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x44061d; continue; }
  pc = 0x44061a; continue;
 }
 0x44061a => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44061d; continue;
 }
 0x44061d => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440626; continue;
 }
 0x440626 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x44062b => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x440632; continue; }
  pc = 0x44062f; continue;
 }
 0x44062f => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x440632; continue;
 }
 0x440632 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44063b; continue;
 }
 0x44063b => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x440640 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x44064a; continue; }
  pc = 0x440647; continue;
 }
 0x440647 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44064a; continue;
 }
 0x44064a => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440653; continue;
 }
 0x440653 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x440658 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440660; continue;
 }
 0x440660 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440668; continue;
 }
 0x440668 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x44066d => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440675; continue;
 }
 0x440675 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44067d; continue;
 }
 0x44067d => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x440682 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44068a; continue;
 }
 0x44068a => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x440692; continue;
 }
 0x440692 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 0x440697 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x44069f; continue;
 }
 0x44069f => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x440551; continue; }
  pc = 0x4406a7; continue;
 }
 0x4406a7 => {
  cpu.poll(); pc = 0x4405a2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004406ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4406ac); }
 let mut pc: u32 = 0x4406ac;
 loop { match pc {
 0x4406ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004407fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4407fd); }
 let mut pc: u32 = 0x4407fd;
 loop { match pc {
 0x4407fd => {
  { let v: u32 = cpu.r32(0x4d6c10u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if !cpu.zf { pc = 0x440828; continue; }
  pc = 0x440808; continue;
 }
 0x440808 => {
  { cpu.sub32(cpu.r32(0x4d6c44u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x4408f0; continue; }
  pc = 0x440815; continue;
 }
 0x440815 => {
  { let v: u32 = cpu.r32(0x5c4e3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x440828; continue; }
  pc = 0x44081f; continue;
 }
 0x44081f => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x4408f0; continue; }
  pc = 0x440828; continue;
 }
 0x440828 => {
  { let v: u32 = cpu.r32(0x4d6c10u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x4408f0; continue; }
  pc = 0x440837; continue;
 }
 0x440837 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c54u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c50u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c4cu32, v); }
  cpu.push(0x44085d); f_00440914(cpu); if cpu.returned_to != 0x44085d { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xa1u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c28u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x5092eeu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c2cu32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67eau32, v); }
  { let v: u32 = cpu.r32(0x4d6a14u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d69f0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x4408a9; continue; }
  pc = 0x4408a4; continue;
 }
 0x4408a4 => {
  { let v: u32 = cpu.r32(0x4d69f0u32); cpu.eax = v; }
  pc = 0x4408a9; continue;
 }
 0x4408a9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d69f4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x4408b6; continue; }
  pc = 0x4408b1; continue;
 }
 0x4408b1 => {
  { let v: u32 = cpu.r32(0x4d69f4u32); cpu.eax = v; }
  pc = 0x4408b6; continue;
 }
 0x4408b6 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d69f8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x4408c3; continue; }
  pc = 0x4408be; continue;
 }
 0x4408be => {
  { let v: u32 = cpu.r32(0x4d69f8u32); cpu.eax = v; }
  pc = 0x4408c3; continue;
 }
 0x4408c3 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d69fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x4408d0; continue; }
  pc = 0x4408cb; continue;
 }
 0x4408cb => {
  { let v: u32 = cpu.r32(0x4d69fcu32); cpu.eax = v; }
  pc = 0x4408d0; continue;
 }
 0x4408d0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a14u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c40u32, v); }
  { let v: u32 = cpu.r32(0x4d69f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c38u32, v); }
  { let v: u32 = cpu.r32(0x4d69f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c3cu32, v); }
  pc = 0x4408f7; continue;
 }
 0x4408f0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d67e2u32, v); }
  pc = 0x4408f7; continue;
 }
 0x4408f7 => {
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x440902); f_00474aaf(cpu); if cpu.returned_to != 0x440902 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67e0u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4d67e1u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4d67dfu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440914(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440914); }
 let mut pc: u32 = 0x440914;
 loop { match pc {
 0x440914 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67e2u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x440970; continue; }
  pc = 0x440921; continue;
 }
 0x440921 => {
  { let v: u16 = cpu.r16(0x5c9afeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x12cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e8u32, v); }
  cpu.push(0x440936); f_0046d525(cpu); if cpu.returned_to != 0x440936 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67deu32, v); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x44094c; continue; }
  pc = 0x440943; continue;
 }
 0x440943 => {
  { cpu.sub8(cpu.r8(0x75u32.wrapping_add(cpu.ebp)), 0x5cu8); }
  if (cpu.sf != cpu.of) { pc = 0x44094c; continue; }
  pc = 0x44094a; continue;
 }
 0x44094a => {
  { let v: u8 = 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44094c; continue;
 }
 0x44094c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67e3u32, v); }
  { let v: u32 = cpu.r8(0x3bu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x44095d; continue; }
  pc = 0x44095b; continue;
 }
 0x44095b => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44095d; continue;
 }
 0x44095d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e6u32, v); }
  { let v: u32 = cpu.r8(0x3au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e4u32, v); }
  pc = 0x4409b9; continue;
 }
 0x440970 => {
  { let v: u16 = cpu.r16(0x5c9b00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e8u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44098a; continue; }
  pc = 0x440985; continue;
 }
 0x440985 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44098f; continue; }
  pc = 0x44098a; continue;
 }
 0x44098a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44098f; continue;
 }
 0x44098f => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e4u32, v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d67e4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x4409ac; continue; }
  pc = 0x4409a6; continue;
 }
 0x4409a6 => {
  { let v: u16 = cpu.r16(0x4d67e4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4409ac; continue;
 }
 0x4409ac => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x4409b3; continue; }
  pc = 0x4409b1; continue;
 }
 0x4409b1 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4409b3; continue;
 }
 0x4409b3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67e6u32, v); }
  pc = 0x4409b9; continue;
 }
 0x4409b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004409ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4409ba); }
 let mut pc: u32 = 0x4409ba;
 loop { match pc {
 0x4409ba => {
  { let v: u32 = 0x5a3de2u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fbau32, v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x52b614u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a3ed2u32, v); }
  pc = 0x4409e2; continue;
 }
 0x4409e2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x440a1a; continue; }
  pc = 0x4409e9; continue;
 }
 0x4409e9 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x440a1a; continue; }
  pc = 0x4409f8; continue;
 }
 0x4409f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x440a01); f_00445c78(cpu); if cpu.returned_to != 0x440a01 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.cf { pc = 0x440a07; continue; }
  pc = 0x440a05; continue;
 }
 0x440a05 => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x440a07; continue;
 }
 0x440a07 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x440a12); f_00440a30(cpu); if cpu.returned_to != 0x440a12 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5a3ed2u32)); cpu.w8(0x5a3ed2u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x440a1a; continue;
 }
 0x440a1a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4409e2; continue; }
  pc = 0x440a21; continue;
 }
 0x440a21 => {
  { cpu.sub8(cpu.r8(0x5a3ed2u32), 0x0u8); }
  if !cpu.zf { pc = 0x440a2f; continue; }
  pc = 0x440a2a; continue;
 }
 0x440a2a => {
  cpu.push(0x440a2f); f_00440b80(cpu); if cpu.returned_to != 0x440a2f { return; }
  pc = 0x440a2f; continue;
 }
 0x440a2f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440a30); }
 let mut pc: u32 = 0x440a30;
 loop { match pc {
 0x440a30 => {
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x50u32); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x440a52; continue;
 }
 0x440a52 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x440b69; continue; }
  pc = 0x440a60; continue;
 }
 0x440a60 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x440b69; continue; }
  pc = 0x440a72; continue;
 }
 0x440a72 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x440b69; continue; }
  pc = 0x440a7b; continue;
 }
 0x440a7b => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6790u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x440b69; continue; }
  pc = 0x440a8d; continue;
 }
 0x440a8d => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x440b69; continue; }
  pc = 0x440a96; continue;
 }
 0x440a96 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x440ad6; continue; }
  pc = 0x440aa0; continue;
 }
 0x440aa0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bdcu32)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x440b69; continue; }
  pc = 0x440ad5; continue;
 }
 0x440ad5 => {
  {  }
  pc = 0x440ad6; continue;
 }
 0x440ad6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5a3de2u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x440b69; continue; }
  pc = 0x440ae7; continue;
 }
 0x440ae7 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x440b2d; continue; }
  pc = 0x440b16; continue;
 }
 0x440b16 => {
  { cpu.and32(cpu.r32(0x4d6a18u32), 0x1u32); }
  if !cpu.zf { pc = 0x440b2d; continue; }
  pc = 0x440b22; continue;
 }
 0x440b22 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x440b2d; continue; }
  pc = 0x440b29; continue;
 }
 0x440b29 => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x440b39; continue;
 }
 0x440b2d => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x440b39; continue;
 }
 0x440b39 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x440b4d; continue; }
  pc = 0x440b44; continue;
 }
 0x440b44 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x440b4d; continue; }
  pc = 0x440b4b; continue;
 }
 0x440b4b => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x440b4d; continue;
 }
 0x440b4d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6b64u32).wrapping_add(0x6u32); cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1fbau32)); cpu.w8(0x4e1fbau32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5a3de2u32.wrapping_add(cpu.eax), v); }
  pc = 0x440b69; continue;
 }
 0x440b69 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x440a52; continue; }
  pc = 0x440b79; continue;
 }
 0x440b79 => {
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440b80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440b80); }
 let mut pc: u32 = 0x440b80;
 loop { match pc {
 0x440b80 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x440b8f; continue;
 }
 0x440b8f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x440c3f; continue; }
  pc = 0x440b9d; continue;
 }
 0x440b9d => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x440c3f; continue; }
  pc = 0x440bab; continue;
 }
 0x440bab => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x440c3f; continue; }
  pc = 0x440bbd; continue;
 }
 0x440bbd => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x440c3f; continue; }
  pc = 0x440bc2; continue;
 }
 0x440bc2 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6790u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x440c3f; continue; }
  pc = 0x440bd0; continue;
 }
 0x440bd0 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x440c3f; continue; }
  pc = 0x440bd5; continue;
 }
 0x440bd5 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5a3de2u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x440c3f; continue; }
  pc = 0x440bea; continue;
 }
 0x440bea => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4ea20au32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x440c23; continue; }
  pc = 0x440c1a; continue;
 }
 0x440c1a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x440c23; continue; }
  pc = 0x440c21; continue;
 }
 0x440c21 => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x440c23; continue;
 }
 0x440c23 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6b64u32).wrapping_add(0x6u32); cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1fbau32)); cpu.w8(0x4e1fbau32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5a3de2u32.wrapping_add(cpu.eax), v); }
  pc = 0x440c3f; continue;
 }
 0x440c3f => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x440b8f; continue; }
  pc = 0x440c4f; continue;
 }
 0x440c4f => {
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440c56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440c56); }
 let mut pc: u32 = 0x440c56;
 loop { match pc {
 0x440c56 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x440c5d; continue;
 }
 0x440c5d => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x440d36; continue; }
  pc = 0x440c68; continue;
 }
 0x440c68 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x440d36; continue; }
  pc = 0x440c7b; continue;
 }
 0x440c7b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x440c82); f_00445c78(cpu); if cpu.returned_to != 0x440c82 { return; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x50u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.cf { pc = 0x440c91; continue; }
  pc = 0x440c8c; continue;
 }
 0x440c8c => {
  { let v: u32 = 0x7ffu32; cpu.eax = v; }
  pc = 0x440c91; continue;
 }
 0x440c91 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5c5a82u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x5c7a86u32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 1u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x5c5a82u32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  cpu.push(0x440cb6); f_00440d42(cpu); if cpu.returned_to != 0x440cb6 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x576bfcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x440cee; continue; }
  pc = 0x440cdc; continue;
 }
 0x440cdc => {
  { let v: u8 = cpu.inc8(cpu.r8(0x33u32.wrapping_add(cpu.ebp))); cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x33u32.wrapping_add(cpu.ebp)), (cpu.ecx as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x440cee; continue; }
  pc = 0x440ce9; continue;
 }
 0x440ce9 => {
  { let v: u8 = 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  pc = 0x440cee; continue;
 }
 0x440cee => {
  { let v: u32 = cpu.r32(0x5c5a68u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x120u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x440d09; continue; }
  pc = 0x440d03; continue;
 }
 0x440d03 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x140u32); cpu.ecx = v; }
  pc = 0x440d09; continue;
 }
 0x440d09 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5c5a6cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x92u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x440d26; continue; }
  pc = 0x440d20; continue;
 }
 0x440d20 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x92u32); cpu.ebx = v; }
  pc = 0x440d26; continue;
 }
 0x440d26 => {
  { let v: u16 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x440d34); f_004417d7(cpu); if cpu.returned_to != 0x440d34 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x440d36; continue;
 }
 0x440d36 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x440c5d; continue; }
  pc = 0x440d41; continue;
 }
 0x440d41 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440d42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440d42); }
 let mut pc: u32 = 0x440d42;
 loop { match pc {
 0x440d42 => {
  { let v: u32 = cpu.r32(0x5c5a82u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5c7a86u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440d51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440d51); }
 let mut pc: u32 = 0x440d51;
 loop { match pc {
 0x440d51 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x440d84; continue; }
  pc = 0x440d5a; continue;
 }
 0x440d5a => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x440d83; continue; }
  pc = 0x440d63; continue;
 }
 0x440d63 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x440d83; continue; }
  pc = 0x440d6c; continue;
 }
 0x440d6c => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x440d83; continue; }
  pc = 0x440d75; continue;
 }
 0x440d75 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af7u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x59f43au32, v); }
  pc = 0x440d83; continue;
 }
 0x440d83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x440d84 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x440da6; continue; }
  pc = 0x440d8d; continue;
 }
 0x440d8d => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x440da6; continue; }
  pc = 0x440d96; continue;
 }
 0x440d96 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x440da6; continue; }
  pc = 0x440d9f; continue;
 }
 0x440d9f => {
  { let v: u8 = 0x1u8; cpu.w8(0x57595au32, v); }
  pc = 0x440da6; continue;
 }
 0x440da6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440da7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440da7); }
 let mut pc: u32 = 0x440da7;
 loop { match pc {
 0x440da7 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x440df1; continue; }
  pc = 0x440db0; continue;
 }
 0x440db0 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x440df1; continue; }
  pc = 0x440dba; continue;
 }
 0x440dba => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x440df1; continue; }
  pc = 0x440dc3; continue;
 }
 0x440dc3 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x440df1; continue; }
  pc = 0x440dcc; continue;
 }
 0x440dcc => {
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af8u32, v); }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.sf { pc = 0x440de5; continue; }
  pc = 0x440ddc; continue;
 }
 0x440ddc => {
  { let v: u8 = 0x0u8; cpu.w8(0x575959u32, v); }
  pc = 0x440df1; continue;
 }
 0x440de5 => {
  { let v: u8 = 0xffu8; cpu.w8(0x575959u32, v); }
  cpu.push(0x440df1); f_00440e68(cpu); if cpu.returned_to != 0x440df1 { return; }
  pc = 0x440df1; continue;
 }
 0x440df1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440df2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440df2); }
 let mut pc: u32 = 0x440df2;
 loop { match pc {
 0x440df2 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x581b71u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x440e66; continue; }
  pc = 0x440dfc; continue;
 }
 0x440dfc => {
  cpu.push(0x440e01); f_00440e68(cpu); if cpu.returned_to != 0x440e01 { return; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x440e29; continue; }
  pc = 0x440e13; continue;
 }
 0x440e13 => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x440e29; continue; }
  pc = 0x440e22; continue;
 }
 0x440e22 => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x440e66; continue;
 }
 0x440e29 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x440e56; continue; }
  pc = 0x440e46; continue;
 }
 0x440e46 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x440e56; continue; }
  pc = 0x440e4e; continue;
 }
 0x440e4e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x440e5f; continue; }
  pc = 0x440e56; continue;
 }
 0x440e56 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x440e5f); f_00441589(cpu); if cpu.returned_to != 0x440e5f { return; }
  pc = 0x440e5f; continue;
 }
 0x440e5f => {
  { let v: u8 = 0xffu8; cpu.w8(0x575959u32, v); }
  pc = 0x440e66; continue;
 }
 0x440e66 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440e68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440e68); }
 let mut pc: u32 = 0x440e68;
 loop { match pc {
 0x440e68 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5cc416u32; cpu.edi = v; }
  { let v: u32 = 0x5cc467u32; cpu.esi = v; }
  { let v: u32 = 0x5cc482u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x5cc467u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u8 = 0xffu8; cpu.w8(0x58c553u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x440e9d); f_00440ece(cpu); if cpu.returned_to != 0x440e9d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x58c44bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x440ead); f_00440ece(cpu); if cpu.returned_to != 0x440ead { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x58c44cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x440ebd); f_00440ece(cpu); if cpu.returned_to != 0x440ebd { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x58c44du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x440ecc); f_00440ece(cpu); if cpu.returned_to != 0x440ecc { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440ece(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440ece); }
 let mut pc: u32 = 0x440ece;
 loop { match pc {
 0x440ece => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x1u16); }
  if cpu.zf { pc = 0x440f14; continue; }
  pc = 0x440ed8; continue;
 }
 0x440ed8 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if cpu.zf { pc = 0x440f14; continue; }
  pc = 0x440ee2; continue;
 }
 0x440ee2 => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x440eec; continue; }
  pc = 0x440ee7; continue;
 }
 0x440ee7 => {
  pc = 0x466c9b; continue;
 }
 0x440eec => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x440ef4); f_0042ebd2(cpu); if cpu.returned_to != 0x440ef4 { return; }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x3u16); }
  if cpu.zf { pc = 0x440f09; continue; }
  pc = 0x440efe; continue;
 }
 0x440efe => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x440f1b; continue; }
  pc = 0x440f03; continue;
 }
 0x440f03 => {
  { cpu.popad(); }
  pc = 0x466d5a; continue;
 }
 0x440f09 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x440f1b; continue; }
  pc = 0x440f0e; continue;
 }
 0x440f0e => {
  { cpu.popad(); }
  pc = 0x466d5a; continue;
 }
 0x440f14 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x466d5a; continue;
 }
 0x440f1b => {
  { cpu.popad(); }
  pc = 0x466cfc; continue;
 }
 0x466c9b => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.sf { pc = 0x466cac; continue; }
  pc = 0x466ca7; continue;
 }
 0x466ca7 => {
  pc = 0x466d5a; continue;
 }
 0x466cac => {
  { cpu.pushad(); }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x466cd7; continue; }
  pc = 0x466cc3; continue;
 }
 0x466cc3 => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x466cd7; continue; }
  pc = 0x466cd2; continue;
 }
 0x466cd2 => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x466cd7; continue;
 }
 0x466cd7 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x466cf7; continue; }
  pc = 0x466cf3; continue;
 }
 0x466cf3 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x80u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x466cf7; continue;
 }
 0x466cf7 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x466cd7; continue; }
  pc = 0x466cfa; continue;
 }
 0x466cfa => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x466cfc => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.sf { pc = 0x466d0a; continue; }
  pc = 0x466d08; continue;
 }
 0x466d08 => {
  pc = 0x466d5a; continue;
 }
 0x466d0a => {
  { cpu.pushad(); }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x466d35; continue; }
  pc = 0x466d21; continue;
 }
 0x466d21 => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x466d35; continue; }
  pc = 0x466d30; continue;
 }
 0x466d30 => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x466d35; continue;
 }
 0x466d35 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x466d55; continue; }
  pc = 0x466d51; continue;
 }
 0x466d51 => {
  { let v: u8 = cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x7fu8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x466d55; continue;
 }
 0x466d55 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x466d35; continue; }
  pc = 0x466d58; continue;
 }
 0x466d58 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x466d5a => {
  { cpu.pushad(); }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x466d85; continue; }
  pc = 0x466d71; continue;
 }
 0x466d71 => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x466d85; continue; }
  pc = 0x466d80; continue;
 }
 0x466d80 => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x466d85; continue;
 }
 0x466d85 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x466da5; continue; }
  pc = 0x466da1; continue;
 }
 0x466da1 => {
  { let v: u8 = 0x8u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x466da5; continue;
 }
 0x466da5 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x466d85; continue; }
  pc = 0x466da8; continue;
 }
 0x466da8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440f21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x440f21); }
 let mut pc: u32 = 0x440f21;
 loop { match pc {
 0x440f21 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x440f5e; continue; }
  pc = 0x440f2a; continue;
 }
 0x440f2a => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x440f5e; continue; }
  pc = 0x440f33; continue;
 }
 0x440f33 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x440f5e; continue; }
  pc = 0x440f3c; continue;
 }
 0x440f3c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af8u32, v); }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if cpu.sf { pc = 0x440f57; continue; }
  pc = 0x440f4c; continue;
 }
 0x440f4c => {
  if cpu.zf { pc = 0x440f57; continue; }
  pc = 0x440f4e; continue;
 }
 0x440f4e => {
  { let v: u8 = 0x0u8; cpu.w8(0x575959u32, v); }
  pc = 0x440f5e; continue;
 }
 0x440f57 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575959u32, v); }
  pc = 0x440f5e; continue;
 }
 0x440f5e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00440f5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440f5f); } r_00440f5f(cpu, 0x440f5f); }
pub fn f_00440f76(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x440f76); } r_00440f5f(cpu, 0x440f76); }
pub fn f_00441093(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x441093); } r_00440f5f(cpu, 0x441093); }
pub fn f_004411ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4411ed); } r_00440f5f(cpu, 0x4411ed); }
fn r_00440f5f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x440f5f => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x440f8d; continue; }
  pc = 0x440f6b; continue;
 }
 0x440f6b => {
  { cpu.sub32(cpu.r32(0x58c85eu32), 0x0u32); }
  if !cpu.zf { pc = 0x440f8d; continue; }
  pc = 0x440f74; continue;
 }
 0x440f74 => {
  pc = 0x440fd1; continue;
 }
 0x440f76 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x440f8d; continue; }
  pc = 0x440f82; continue;
 }
 0x440f82 => {
  { cpu.sub32(cpu.r32(0x58c862u32), 0x0u32); }
  if !cpu.zf { pc = 0x440f8d; continue; }
  pc = 0x440f8b; continue;
 }
 0x440f8b => {
  pc = 0x440fd1; continue;
 }
 0x440f8d => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x44122b; continue; }
  pc = 0x440f9f; continue;
 }
 0x440f9f => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1580u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x440fd0; continue; }
  pc = 0x440fb0; continue;
 }
 0x440fb0 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x440fd0; continue; }
  pc = 0x440fb9; continue;
 }
 0x440fb9 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x440fd0; continue; }
  pc = 0x440fc2; continue;
 }
 0x440fc2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56c6e4u32, v); }
  pc = 0x440fd0; continue;
 }
 0x440fd0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x440fd1 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x44122b; continue; }
  pc = 0x440fe3; continue;
 }
 0x440fe3 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1580u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x441014; continue; }
  pc = 0x440ff4; continue;
 }
 0x440ff4 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x441014; continue; }
  pc = 0x440ffd; continue;
 }
 0x440ffd => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x441014; continue; }
  pc = 0x441006; continue;
 }
 0x441006 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af9u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56c6e4u32, v); }
  pc = 0x441014; continue;
 }
 0x441014 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441093 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x44122b; continue; }
  pc = 0x4410a5; continue;
 }
 0x4410a5 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e157cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x4410d4; continue; }
  pc = 0x4410b6; continue;
 }
 0x4410b6 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x4410d4; continue; }
  pc = 0x4410bf; continue;
 }
 0x4410bf => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x4410d4; continue; }
  pc = 0x4410c8; continue;
 }
 0x4410c8 => {
  cpu.push(0x4410cd); f_0042c4dc(cpu); if cpu.returned_to != 0x4410cd { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9afau32, v); }
  pc = 0x4410d4; continue;
 }
 0x4410d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4411ed => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x44122b; continue; }
  pc = 0x4411fb; continue;
 }
 0x4411fb => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1574u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x44122a; continue; }
  pc = 0x44120c; continue;
 }
 0x44120c => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x44122a; continue; }
  pc = 0x441215; continue;
 }
 0x441215 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x44122a; continue; }
  pc = 0x44121e; continue;
 }
 0x44121e => {
  cpu.push(0x441223); f_0042c4f8(cpu); if cpu.returned_to != 0x441223 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9afcu32, v); }
  pc = 0x44122a; continue;
 }
 0x44122a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44122b => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x4412ac; continue; }
  pc = 0x441234; continue;
 }
 0x441234 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x4412ac; continue; }
  pc = 0x44123d; continue;
 }
 0x44123d => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.sf { pc = 0x44124d; continue; }
  pc = 0x441246; continue;
 }
 0x441246 => {
  cpu.push(0x44124b); f_00441589(cpu); if cpu.returned_to != 0x44124b { return; }
  pc = 0x4412ac; continue;
 }
 0x44124d => {
  { let v: u32 = 0x5cc431u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x441272; continue; }
  pc = 0x44125c; continue;
 }
 0x44125c => {
  { let v: u32 = 0x5cc43du32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x441272; continue; }
  pc = 0x44126b; continue;
 }
 0x44126b => {
  { let v: u32 = 0x5cc446u32; cpu.ebp = v; }
  pc = 0x4412ac; continue;
 }
 0x441272 => {
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x4412ac; continue; }
  pc = 0x441292; continue;
 }
 0x441292 => {
  { let v: u8 = (cpu.eax as u8) ^ 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x4412ac; continue; }
  pc = 0x4412a7; continue;
 }
 0x4412a7 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x4412ac; continue;
 }
 0x4412ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441015(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441015); }
 let mut pc: u32 = 0x441015;
 loop { match pc {
 0x441015 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x441041; continue; }
  pc = 0x441035; continue;
 }
 0x441035 => {
  { let v: u32 = cpu.r32(0x58c852u32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x441041; continue;
 }
 0x441041 => {
  cpu.push(0x441046); f_00441093(cpu); if cpu.returned_to != 0x441046 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441054(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x441054); } r_00441054(cpu, 0x441054); }
pub fn f_00441080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x441080); } r_00441054(cpu, 0x441080); }
fn r_00441054(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x441054 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c838u32, v); }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x441080; continue; }
  pc = 0x441074; continue;
 }
 0x441074 => {
  { let v: u32 = cpu.r32(0x58c84eu32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x441080; continue;
 }
 0x441080 => {
  cpu.push(0x441085); f_00441093(cpu); if cpu.returned_to != 0x441085 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004410d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4410d5); }
 let mut pc: u32 = 0x4410d5;
 loop { match pc {
 0x4410d5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  cpu.push(0x4410ee); f_00441136(cpu); if cpu.returned_to != 0x4410ee { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004410fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4410fc); }
 let mut pc: u32 = 0x4410fc;
 loop { match pc {
 0x4410fc => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x441130; continue; }
  pc = 0x441108; continue;
 }
 0x441108 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  cpu.push(0x441121); f_00441136(cpu); if cpu.returned_to != 0x441121 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  pc = 0x441135; continue;
 }
 0x441130 => {
  cpu.push(0x441135); f_00441136(cpu); if cpu.returned_to != 0x441135 { return; }
  pc = 0x441135; continue;
 }
 0x441135 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441136(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441136); }
 let mut pc: u32 = 0x441136;
 loop { match pc {
 0x441136 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x44116e; continue; }
  pc = 0x44113f; continue;
 }
 0x44113f => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1578u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x44116e; continue; }
  pc = 0x441150; continue;
 }
 0x441150 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x44116e; continue; }
  pc = 0x441159; continue;
 }
 0x441159 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x44116e; continue; }
  pc = 0x441162; continue;
 }
 0x441162 => {
  cpu.push(0x441167); f_0042c514(cpu); if cpu.returned_to != 0x441167 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9afbu32, v); }
  pc = 0x44116e; continue;
 }
 0x44116e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044116f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44116f); }
 let mut pc: u32 = 0x44116f;
 loop { match pc {
 0x44116f => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x44119b; continue; }
  pc = 0x44118f; continue;
 }
 0x44118f => {
  { let v: u32 = cpu.r32(0x58c85au32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x44119b; continue;
 }
 0x44119b => {
  cpu.push(0x4411a0); f_004411ed(cpu); if cpu.returned_to != 0x4411a0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004411ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4411ae); }
 let mut pc: u32 = 0x4411ae;
 loop { match pc {
 0x4411ae => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c838u32, v); }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x4411da; continue; }
  pc = 0x4411ce; continue;
 }
 0x4411ce => {
  { let v: u32 = cpu.r32(0x58c856u32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x4411da; continue;
 }
 0x4411da => {
  cpu.push(0x4411df); f_004411ed(cpu); if cpu.returned_to != 0x4411df { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004412ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4412ad); } r_004412ad(cpu, 0x4412ad); }
pub fn f_004412eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4412eb); } r_004412ad(cpu, 0x4412eb); }
fn r_004412ad(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4412ad => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x58c553u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x4412ee; continue; }
  pc = 0x4412bb; continue;
 }
 0x4412bb => {
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x441361; continue; }
  pc = 0x4412c4; continue;
 }
 0x4412c4 => {
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x4412cc); f_004412eb(cpu); if cpu.returned_to != 0x4412cc { return; }
  { let v: u32 = cpu.r32(0x5ae134u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5a3ed3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4412eb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x4412ee; continue;
 }
 0x4412ee => {
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x441313; continue; }
  pc = 0x4412fd; continue;
 }
 0x4412fd => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x441313; continue; }
  pc = 0x44130c; continue;
 }
 0x44130c => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x441361; continue;
 }
 0x441313 => {
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x441361; continue; }
  pc = 0x441334; continue;
 }
 0x441334 => {
  { let v: u32 = cpu.r32(0x5ae138u32); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5a3ed3u32, v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.sf { pc = 0x44134e; continue; }
  pc = 0x441344; continue;
 }
 0x441344 => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5ae140u32); cpu.esi = v; }
  pc = 0x44135c; continue;
 }
 0x44134e => {
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441359; continue;
 }
 0x441359 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44135c; continue;
 }
 0x44135c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441361 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44136f; continue;
 }
 0x44136f => {
  { let v: u8 = cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x441381; continue; }
  pc = 0x44137b; continue;
 }
 0x44137b => {
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x59c5f2u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441381; continue;
 }
 0x441381 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x44136f; continue; }
  pc = 0x441387; continue;
 }
 0x441387 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  { let v: u32 = cpu.r32(0x5ae13cu32); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x441359; continue; }
  pc = 0x441392; continue;
 }
 0x441392 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5ae130u32); cpu.esi = v; }
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044139f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44139f); }
 let mut pc: u32 = 0x44139f;
 loop { match pc {
 0x44139f => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x58c553u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if cpu.sf { pc = 0x4414a8; continue; }
  pc = 0x4413af; continue;
 }
 0x4413af => {
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x4413d7; continue; }
  pc = 0x4413be; continue;
 }
 0x4413be => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x4413d7; continue; }
  pc = 0x4413cd; continue;
 }
 0x4413cd => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x4414a8; continue;
 }
 0x4413d7 => {
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x4414a8; continue; }
  pc = 0x4413fc; continue;
 }
 0x4413fc => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.sf { pc = 0x441411; continue; }
  pc = 0x441400; continue;
 }
 0x441400 => {
  { let v: u8 = 0xefu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x441448; continue;
 }
 0x441411 => {
  { let v: u8 = 0xeeu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x58c553u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x441431); f_00440e68(cpu); if cpu.returned_to != 0x441431 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x58c553u32, v); }
  { let v: u32 = 0x154u32; cpu.w32(0x58c554u32, v); }
  { cpu.popad(); }
  pc = 0x441487; continue;
 }
 0x441448 => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441460; continue; }
  pc = 0x44145d; continue;
 }
 0x44145d => {
  { let v: u8 = (cpu.ecx as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441460; continue;
 }
 0x441460 => {
  { cpu.sub8(cpu.r8(0x58c44bu32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x44146d; continue; }
  pc = 0x44146a; continue;
 }
 0x44146a => {
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44146d; continue;
 }
 0x44146d => {
  { cpu.sub8(cpu.r8(0x58c44cu32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x44147a; continue; }
  pc = 0x441477; continue;
 }
 0x441477 => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44147a; continue;
 }
 0x44147a => {
  { cpu.sub8(cpu.r8(0x58c44du32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441487; continue; }
  pc = 0x441484; continue;
 }
 0x441484 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441487; continue;
 }
 0x441487 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4d68b0u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x44149e); f_00413337(cpu); if cpu.returned_to != 0x44149e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4414a8 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004414ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4414ab); }
 let mut pc: u32 = 0x4414ab;
 loop { match pc {
 0x4414ab => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x58c549u32), 0x1u32); }
  if !cpu.zf { pc = 0x44155d; continue; }
  pc = 0x4414b9; continue;
 }
 0x4414b9 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x4414d5; continue; }
  pc = 0x4414d2; continue;
 }
 0x4414d2 => {
  { let v: u8 = (cpu.ecx as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4414d5; continue;
 }
 0x4414d5 => {
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.edx.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x4414e2; continue; }
  pc = 0x4414df; continue;
 }
 0x4414df => {
  { let v: u8 = (cpu.ecx as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4414e2; continue;
 }
 0x4414e2 => {
  { cpu.sub8(cpu.r8(0x58c44bu32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x4414ef; continue; }
  pc = 0x4414ec; continue;
 }
 0x4414ec => {
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4414ef; continue;
 }
 0x4414ef => {
  { cpu.sub8(cpu.r8(0x58c44bu32.wrapping_add(cpu.edx.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x4414fc; continue; }
  pc = 0x4414f9; continue;
 }
 0x4414f9 => {
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4414fc; continue;
 }
 0x4414fc => {
  { cpu.sub8(cpu.r8(0x58c44cu32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441509; continue; }
  pc = 0x441506; continue;
 }
 0x441506 => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441509; continue;
 }
 0x441509 => {
  { cpu.sub8(cpu.r8(0x58c44cu32.wrapping_add(cpu.edx.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441516; continue; }
  pc = 0x441513; continue;
 }
 0x441513 => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441516; continue;
 }
 0x441516 => {
  { cpu.sub8(cpu.r8(0x58c44du32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441523; continue; }
  pc = 0x441520; continue;
 }
 0x441520 => {
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441523; continue;
 }
 0x441523 => {
  { cpu.sub8(cpu.r8(0x58c44du32.wrapping_add(cpu.edx.wrapping_mul(4))), 0xffu8); }
  if cpu.zf { pc = 0x441530; continue; }
  pc = 0x44152d; continue;
 }
 0x44152d => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x441530; continue;
 }
 0x441530 => {
  { let v: u8 = 0xf0u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4d68b0u32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x441553); f_00413337(cpu); if cpu.returned_to != 0x441553 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44155d => {
  { let v: u8 = 0xebu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68b0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x44157f); f_00413337(cpu); if cpu.returned_to != 0x44157f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441589(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441589); }
 let mut pc: u32 = 0x441589;
 loop { match pc {
 0x441589 => {
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x58c553u32), 0x0u8); }
  if cpu.sf { pc = 0x4415a7; continue; }
  pc = 0x441596; continue;
 }
 0x441596 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x58c553u32)); }
  if !cpu.zf { pc = 0x4415b0; continue; }
  pc = 0x44159e; continue;
 }
 0x44159e => {
  { let v: u8 = 0xffu8; cpu.w8(0x58c553u32, v); }
  pc = 0x4415c3; continue;
 }
 0x4415a7 => {
  { let v: u8 = 0xffu8; cpu.w8(0x58c553u32, v); }
  cpu.poll(); pc = 0x441596; continue;
 }
 0x4415b0 => {
  { let v: u8 = cpu.r8(0x58c553u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x58c553u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x4415c3); f_004415c4(cpu); if cpu.returned_to != 0x4415c3 { return; }
  pc = 0x4415c3; continue;
 }
 0x4415c3 => {
  { cpu.popad(); }
  f_004415c4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004415c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4415c4); }
 let mut pc: u32 = 0x4415c4;
 loop { match pc {
 0x4415c4 => {
  { let v: u32 = 0x5cc416u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x4415e9; continue; }
  pc = 0x4415d3; continue;
 }
 0x4415d3 => {
  { let v: u32 = 0x5cc422u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x4415e9; continue; }
  pc = 0x4415e2; continue;
 }
 0x4415e2 => {
  { let v: u32 = 0x5cc42bu32; cpu.ebp = v; }
  pc = 0x44160e; continue;
 }
 0x4415e9 => {
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x44160e; continue; }
  pc = 0x441609; continue;
 }
 0x441609 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x44160e; continue;
 }
 0x44160e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044160f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44160f); }
 let mut pc: u32 = 0x44160f;
 loop { match pc {
 0x44160f => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x44168b; continue; }
  pc = 0x441618; continue;
 }
 0x441618 => {
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x581b71u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x44168b; continue; }
  pc = 0x441624; continue;
 }
 0x441624 => {
  { let v: u32 = 0x5cc431u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x441649; continue; }
  pc = 0x441633; continue;
 }
 0x441633 => {
  { let v: u32 = 0x5cc43du32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x441649; continue; }
  pc = 0x441642; continue;
 }
 0x441642 => {
  { let v: u32 = 0x5cc446u32; cpu.ebp = v; }
  pc = 0x441684; continue;
 }
 0x441649 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x44167e; continue; }
  pc = 0x441672; continue;
 }
 0x441672 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x44167e; continue; }
  pc = 0x441678; continue;
 }
 0x441678 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x441684; continue; }
  pc = 0x44167e; continue;
 }
 0x44167e => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x441684; continue;
 }
 0x441684 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575959u32, v); }
  pc = 0x44168b; continue;
 }
 0x44168b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044168d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44168d); }
 let mut pc: u32 = 0x44168d;
 loop { match pc {
 0x44168d => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  cpu.push(0x4416a6); f_004416e2(cpu); if cpu.returned_to != 0x4416a6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004416e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4416e2); }
 let mut pc: u32 = 0x4416e2;
 loop { match pc {
 0x4416e2 => {
  { cpu.sub8(cpu.r8(0x56c6e1u32), 0x0u8); }
  if cpu.zf { pc = 0x441709; continue; }
  pc = 0x4416eb; continue;
 }
 0x4416eb => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x441709; continue; }
  pc = 0x4416f4; continue;
 }
 0x4416f4 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x441709; continue; }
  pc = 0x4416fd; continue;
 }
 0x4416fd => {
  cpu.push(0x441702); f_0042c4c0(cpu); if cpu.returned_to != 0x441702 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af3u32, v); }
  pc = 0x441709; continue;
 }
 0x441709 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044170a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44170a); }
 let mut pc: u32 = 0x44170a;
 loop { match pc {
 0x44170a => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x44172a; continue; }
  pc = 0x441713; continue;
 }
 0x441713 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x44172a; continue; }
  pc = 0x44171c; continue;
 }
 0x44171c => {
  { let v: u8 = cpu.xor8(cpu.r8(0x5c9af4u32), 0x1u8); cpu.w8(0x5c9af4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af0u32, v); }
  pc = 0x44172a; continue;
 }
 0x44172a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044172b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44172b); }
 let mut pc: u32 = 0x44172b;
 loop { match pc {
 0x44172b => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x44174b; continue; }
  pc = 0x441734; continue;
 }
 0x441734 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x44174b; continue; }
  pc = 0x44173d; continue;
 }
 0x44173d => {
  { let v: u8 = cpu.xor8(cpu.r8(0x5c9af5u32), 0x1u8); cpu.w8(0x5c9af5u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af1u32, v); }
  pc = 0x44174b; continue;
 }
 0x44174b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044174c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44174c); }
 let mut pc: u32 = 0x44174c;
 loop { match pc {
 0x44174c => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x44176c; continue; }
  pc = 0x441755; continue;
 }
 0x441755 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x44176c; continue; }
  pc = 0x44175e; continue;
 }
 0x44175e => {
  { let v: u8 = cpu.xor8(cpu.r8(0x5c9af6u32), 0x1u8); cpu.w8(0x5c9af6u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af2u32, v); }
  pc = 0x44176c; continue;
 }
 0x44176c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044176d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44176d); }
 let mut pc: u32 = 0x44176d;
 loop { match pc {
 0x44176d => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x441777; continue; }
  pc = 0x441776; continue;
 }
 0x441776 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441777 => {
  cpu.push(0x44177c); f_00440c56(cpu); if cpu.returned_to != 0x44177c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044179f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44179f); }
 let mut pc: u32 = 0x44179f;
 loop { match pc {
 0x44179f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6b9cu32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba0u32)); cpu.ebx = v; }
  cpu.push(0x4417b7); f_00464cd3(cpu); if cpu.returned_to != 0x4417b7 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004417b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4417b9); }
 let mut pc: u32 = 0x4417b9;
 loop { match pc {
 0x4417b9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6b9cu32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba0u32)); cpu.ebx = v; }
  cpu.push(0x4417d3); f_00464d09(cpu); if cpu.returned_to != 0x4417d3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004417d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4417d7); }
 let mut pc: u32 = 0x4417d7;
 loop { match pc {
 0x4417d7 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6ba4u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6b9cu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6ba0u32); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5c5a68u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x120u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x441809; continue; }
  pc = 0x441804; continue;
 }
 0x441804 => {
  { let v: u32 = cpu.eax.wrapping_add(0x140u32); cpu.eax = v; }
  pc = 0x441809; continue;
 }
 0x441809 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b9cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5c5a70u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6ba4u32, v); }
  { let v: u32 = cpu.r32(0x5c5a6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6ba0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5c5a74u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x7u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x441838; continue;
 }
 0x441838 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x2u16); }
  if cpu.cf { pc = 0x441853; continue; }
  pc = 0x441842; continue;
 }
 0x441842 => {
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x441853; continue; }
  pc = 0x44184b; continue;
 }
 0x44184b => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6fbcu32); cpu.push(0x441852); cpu.call(t); if cpu.returned_to != 0x441852 { return; }
  { cpu.popad(); }
  pc = 0x441853; continue;
 }
 0x441853 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x8u32); }
  if !cpu.zf { cpu.poll(); pc = 0x441838; continue; }
  pc = 0x44185a; continue;
 }
 0x44185a => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b9cu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba4u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441874); }
 let mut pc: u32 = 0x441874;
 loop { match pc {
 0x441874 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x44187b; continue;
 }
 0x44187b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x44189b; continue; }
  pc = 0x441895; continue;
 }
 0x441895 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x19u32); cpu.eax = v; }
  pc = 0x44189b; continue;
 }
 0x44189b => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if cpu.cf { pc = 0x4418a3; continue; }
  pc = 0x4418a0; continue;
 }
 0x4418a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x4418a3; continue;
 }
 0x4418a3 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44187b; continue; }
  pc = 0x4418a5; continue;
 }
 0x4418a5 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5c9b00u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004418ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4418ad); }
 let mut pc: u32 = 0x4418ad;
 loop { match pc {
 0x4418ad => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x4418b4; continue;
 }
 0x4418b4 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4418cd; continue; }
  pc = 0x4418ca; continue;
 }
 0x4418ca => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4418cd; continue;
 }
 0x4418cd => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if cpu.cf { pc = 0x4418d5; continue; }
  pc = 0x4418d2; continue;
 }
 0x4418d2 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x4418d5; continue;
 }
 0x4418d5 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4418b4; continue; }
  pc = 0x4418d7; continue;
 }
 0x4418d7 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5c9afeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004418e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4418e0); }
 let mut pc: u32 = 0x4418e0;
 loop { match pc {
 0x4418e0 => {
  { cpu.sub16((cpu.ecx as u16), 0x1bb3u16); }
  if cpu.cf { pc = 0x4418e8; continue; }
  pc = 0x4418e7; continue;
 }
 0x4418e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4418e8 => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4418fa); f_00447634(cpu); if cpu.returned_to != 0x4418fa { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bd0u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ac410u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  cpu.push(0x441912); f_0044193d(cpu); if cpu.returned_to != 0x441912 { return; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44192b; continue; }
  pc = 0x44191b; continue;
 }
 0x44191b => {
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x441926); f_0040b7e0(cpu); if cpu.returned_to != 0x441926 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44192b => {
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x441936); f_0040bc80(cpu); if cpu.returned_to != 0x441936 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044193d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44193d); }
 let mut pc: u32 = 0x44193d;
 loop { match pc {
 0x44193d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441954); }
 let mut pc: u32 = 0x441954;
 loop { match pc {
 0x441954 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441955(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441955); }
 let mut pc: u32 = 0x441955;
 loop { match pc {
 0x441955 => {
  cpu.push(0x44195a); f_00418bd9(cpu); if cpu.returned_to != 0x44195a { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x441966); f_0046d5b9(cpu); if cpu.returned_to != 0x441966 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  if cpu.cf { pc = 0x441a6c; continue; }
  pc = 0x441973; continue;
 }
 0x441973 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44198a); f_004184ca(cpu); if cpu.returned_to != 0x44198a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x441a6a; continue; }
  pc = 0x441993; continue;
 }
 0x441993 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x441a68; continue; }
  pc = 0x4419a7; continue;
 }
 0x4419a7 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4419b9; continue; }
  pc = 0x4419ad; continue;
 }
 0x4419ad => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4419c6; continue;
 }
 0x4419b9 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4419c6; continue;
 }
 0x4419c6 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x4419d7); f_0041b7b0(cpu); if cpu.returned_to != 0x4419d7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x4419fc); f_00442681(cpu); if cpu.returned_to != 0x4419fc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x441a11); f_00442636(cpu); if cpu.returned_to != 0x441a11 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4e159fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f50u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6b04u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x4d67c4u32, v); }
  { cpu.pushad(); }
  cpu.push(0x441a46); f_00441bc3(cpu); if cpu.returned_to != 0x441a46 { return; }
  if cpu.cf { pc = 0x441a4d; continue; }
  pc = 0x441a48; continue;
 }
 0x441a48 => {
  cpu.push(0x441a4d); f_00416654(cpu); if cpu.returned_to != 0x441a4d { return; }
  pc = 0x441a4d; continue;
 }
 0x441a4d => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x441a63; continue; }
  pc = 0x441a57; continue;
 }
 0x441a57 => {
  cpu.push(0x441a5c); f_0041879e(cpu); if cpu.returned_to != 0x441a5c { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x441a63; continue;
 }
 0x441a63 => {
  cpu.push(0x441a68); f_0044236a(cpu); if cpu.returned_to != 0x441a68 { return; }
  pc = 0x441a68; continue;
 }
 0x441a68 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441a6a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441a6c => {
  cpu.push(0x441a71); f_004184ca(cpu); if cpu.returned_to != 0x441a71 { return; }
  cpu.push(0x441a76); f_0046d525(cpu); if cpu.returned_to != 0x441a76 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x441a90; continue; }
  pc = 0x441a7a; continue;
 }
 0x441a7a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x441b0c; continue; }
  pc = 0x441a85; continue;
 }
 0x441a85 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x441b0c; continue; }
  pc = 0x441a90; continue;
 }
 0x441a90 => {
  cpu.push(0x441a95); f_00418bd9(cpu); if cpu.returned_to != 0x441a95 { return; }
  cpu.push(0x441a9a); f_00470fc5(cpu); if cpu.returned_to != 0x441a9a { return; }
  if !cpu.cf { pc = 0x441aa8; continue; }
  pc = 0x441a9c; continue;
 }
 0x441a9c => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x441b0e; continue; }
  pc = 0x441aa8; continue;
 }
 0x441aa8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x441aaf); f_0042e0fa(cpu); if cpu.returned_to != 0x441aaf { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59d9f4u32); cpu.esi = v; }
  if !cpu.zf { pc = 0x441b0e; continue; }
  pc = 0x441abb; continue;
 }
 0x441abb => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x441af6; continue; }
  pc = 0x441add; continue;
 }
 0x441add => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if !cpu.zf { pc = 0x441af6; continue; }
  pc = 0x441ae7; continue;
 }
 0x441ae7 => {
  { let v: u16 = cpu.r16(0x4e159au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4e159cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x441b0e; continue;
 }
 0x441af6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x441aff); f_004712e3(cpu); if cpu.returned_to != 0x441aff { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x441b0e; continue; }
  pc = 0x441b05; continue;
 }
 0x441b05 => {
  cpu.push(0x441b0a); f_004481e0(cpu); if cpu.returned_to != 0x441b0a { return; }
  if cpu.cf { pc = 0x441b0e; continue; }
  pc = 0x441b0c; continue;
 }
 0x441b0c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441b0e => {
  cpu.push(0x441b13); f_0046d86b(cpu); if cpu.returned_to != 0x441b13 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0xd80u16; cpu.w16(0x4d68aeu32, v); }
  { cpu.sub8(cpu.r8(0x4e15a3u32), 0x0u8); }
  if cpu.zf { pc = 0x441b3f; continue; }
  pc = 0x441b32; continue;
 }
 0x441b32 => {
  { let v: u16 = cpu.r16(0x4e159au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4e159cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x441b3f; continue;
 }
 0x441b3f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b2u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x441b59); f_0041b7b0(cpu); if cpu.returned_to != 0x441b59 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x441b66); f_00442681(cpu); if cpu.returned_to != 0x441b66 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x441b7b); f_00442636(cpu); if cpu.returned_to != 0x441b7b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f50u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c6u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  cpu.push(0x441ba7); f_004165fb(cpu); if cpu.returned_to != 0x441ba7 { return; }
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x441bbc; continue; }
  pc = 0x441bb0; continue;
 }
 0x441bb0 => {
  cpu.push(0x441bb5); f_0041879e(cpu); if cpu.returned_to != 0x441bb5 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x441bbc; continue;
 }
 0x441bbc => {
  cpu.push(0x441bc1); f_0044236a(cpu); if cpu.returned_to != 0x441bc1 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441bc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441bc3); }
 let mut pc: u32 = 0x441bc3;
 loop { match pc {
 0x441bc3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x441bd4); f_0044c7f9(cpu); if cpu.returned_to != 0x441bd4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x441bf0; continue; }
  pc = 0x441bd7; continue;
 }
 0x441bd7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x441c33; continue; }
  pc = 0x441bdb; continue;
 }
 0x441bdb => {
  { let v: u32 = cpu.esi.wrapping_add(0x1u32); cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x441be3; continue;
 }
 0x441be3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x441bec; continue; }
  pc = 0x441be8; continue;
 }
 0x441be8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x441c33; continue; }
  pc = 0x441bec; continue;
 }
 0x441bec => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x441be3; continue; }
  pc = 0x441bf0; continue;
 }
 0x441bf0 => {
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x441c09); f_004853e8(cpu); if cpu.returned_to != 0x441c09 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b04u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x441c2a); f_004853e8(cpu); if cpu.returned_to != 0x441c2a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6bd8u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x441c36; continue; }
  pc = 0x441c33; continue;
 }
 0x441c33 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441c36 => {
  cpu.push(0x441c3b); f_00416ba7(cpu); if cpu.returned_to != 0x441c3b { return; }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441c3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441c3e); }
 let mut pc: u32 = 0x441c3e;
 loop { match pc {
 0x441c3e => {
  cpu.push(0x441c43); f_004184ca(cpu); if cpu.returned_to != 0x441c43 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x441c49); f_00418bd9(cpu); if cpu.returned_to != 0x441c49 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x441c55); f_0046d5b9(cpu); if cpu.returned_to != 0x441c55 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x441c86; continue; }
  pc = 0x441c5f; continue;
 }
 0x441c5f => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x441c86; continue; }
  pc = 0x441c82; continue;
 }
 0x441c82 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x441c88; continue;
 }
 0x441c86 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x441c88; continue;
 }
 0x441c88 => {
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x441c8f); f_004184ca(cpu); if cpu.returned_to != 0x441c8f { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x441cb3; continue; }
  pc = 0x441c9a; continue;
 }
 0x441c9a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x441cb3; continue; }
  pc = 0x441ca1; continue;
 }
 0x441ca1 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x441cb3); f_00441f16(cpu); if cpu.returned_to != 0x441cb3 { return; }
  pc = 0x441cb3; continue;
 }
 0x441cb3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441cb4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x441cb4); } r_00441cb4(cpu, 0x441cb4); }
pub fn f_00441e1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x441e1b); } r_00441cb4(cpu, 0x441e1b); }
fn r_00441cb4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x441cb4 => {
  { cpu.sub8(cpu.r8(0x4e1564u32), 0x0u8); }
  if cpu.zf { pc = 0x441e3f; continue; }
  pc = 0x441cc1; continue;
 }
 0x441cc1 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x441cd6; continue; }
  pc = 0x441ccd; continue;
 }
 0x441ccd => {
  { cpu.sub8(cpu.r8(0x58c86eu32), 0x0u8); }
  if cpu.zf { pc = 0x441d17; continue; }
  pc = 0x441cd6; continue;
 }
 0x441cd6 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x441d17; continue; }
  pc = 0x441ce0; continue;
 }
 0x441ce0 => {
  { cpu.pushad(); }
  cpu.push(0x441ce6); f_004184ca(cpu); if cpu.returned_to != 0x441ce6 { return; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x57556au32)); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x441d17; continue; }
  pc = 0x441cef; continue;
 }
 0x441cef => {
  { let v: u16 = cpu.r16(0x4e1598u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x441d0b; continue; }
  pc = 0x441cfc; continue;
 }
 0x441cfc => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x57556au32); cpu.ebp = v; }
  cpu.push(0x441d08); f_004523ec(cpu); if cpu.returned_to != 0x441d08 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x441d17; continue; }
  pc = 0x441d0b; continue;
 }
 0x441d0b => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1564u32, v); }
  pc = 0x441e3f; continue;
 }
 0x441d17 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1564u32)); cpu.w8(0x4e1564u32, v); }
  if !cpu.zf { pc = 0x441d1f; continue; }
  pc = 0x441d1f; continue;
 }
 0x441d1f => {
  { let v: u16 = cpu.r16(0x4e1598u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x441d35; continue; }
  pc = 0x441d2c; continue;
 }
 0x441d2c => {
  { let v: u16 = 0xae2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x441e03; continue;
 }
 0x441d35 => {
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x441d50; continue; }
  pc = 0x441d3b; continue;
 }
 0x441d3b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xaeau32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441d50 => {
  { cpu.sub16((cpu.ecx as u16), 0x2du16); }
  if !cpu.zf { pc = 0x441d6b; continue; }
  pc = 0x441d56; continue;
 }
 0x441d56 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xaceu32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441d6b => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x441d86; continue; }
  pc = 0x441d71; continue;
 }
 0x441d71 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xac6u32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441d86 => {
  { cpu.sub16((cpu.ecx as u16), 0x2bu16); }
  if !cpu.zf { pc = 0x441da1; continue; }
  pc = 0x441d8c; continue;
 }
 0x441d8c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xad6u32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441da1 => {
  { cpu.sub16((cpu.ecx as u16), 0x2cu16); }
  if !cpu.zf { pc = 0x441dbc; continue; }
  pc = 0x441da7; continue;
 }
 0x441da7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xadeu32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441dbc => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x441dd4; continue; }
  pc = 0x441dc2; continue;
 }
 0x441dc2 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb24u32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441dd4 => {
  { cpu.sub16((cpu.ecx as u16), 0xaefu16); }
  if !cpu.zf { pc = 0x441ded; continue; }
  pc = 0x441ddb; continue;
 }
 0x441ddb => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xaefu32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441ded => {
  { cpu.sub16((cpu.ecx as u16), 0x6u16); }
  if !cpu.zf { pc = 0x441e03; continue; }
  pc = 0x441df3; continue;
 }
 0x441df3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb1cu32); cpu.ecx = v; }
  pc = 0x441e03; continue;
 }
 0x441e03 => {
  { cpu.sub32(cpu.r32(0x4e159fu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x441e1e; continue; }
  pc = 0x441e0c; continue;
 }
 0x441e0c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4e159fu32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x441e1b; continue;
 }
 0x441e1b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x441e2b; continue;
 }
 0x441e1e => {
  { let v: u16 = cpu.r16(0x4e1594u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4e1596u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x441e2b; continue;
 }
 0x441e2b => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x441e3f); cpu.call(t); if cpu.returned_to != 0x441e3f { return; }
  pc = 0x441e3f; continue;
 }
 0x441e3f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441e40); }
 let mut pc: u32 = 0x441e40;
 loop { match pc {
 0x441e40 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x441e4e; continue; }
  pc = 0x441e49; continue;
 }
 0x441e49 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x441e53; continue; }
  pc = 0x441e4e; continue;
 }
 0x441e4e => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x441e53; continue;
 }
 0x441e53 => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x441e6b; continue; }
  pc = 0x441e62; continue;
 }
 0x441e62 => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x441e92; continue; }
  pc = 0x441e69; continue;
 }
 0x441e69 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441e6b => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x52bbf0u32); cpu.esi = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x441e9d; continue; }
  pc = 0x441e7c; continue;
 }
 0x441e7c => {
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x509290u32); cpu.esi = v; }
  pc = 0x441e88; continue;
 }
 0x441e88 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441e92 => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x509290u32); cpu.esi = v; }
  cpu.poll(); pc = 0x441e88; continue;
 }
 0x441e9d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x15u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441f16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441f16); }
 let mut pc: u32 = 0x441f16;
 loop { match pc {
 0x441f16 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x441f46; continue; }
  pc = 0x441f1d; continue;
 }
 0x441f1d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x441f24); f_00447617(cpu); if cpu.returned_to != 0x441f24 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xaeau32); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x441f45); cpu.call(t); if cpu.returned_to != 0x441f45 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x441f46 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x441f60; continue; }
  pc = 0x441f4d; continue;
 }
 0x441f4d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xadeu32); cpu.ecx = v; }
  pc = 0x441f71; continue;
 }
 0x441f60 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xac6u32); cpu.ecx = v; }
  pc = 0x441f71; continue;
 }
 0x441f71 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x441f77); cpu.call(t); if cpu.returned_to != 0x441f77 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441f78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441f78); }
 let mut pc: u32 = 0x441f78;
 loop { match pc {
 0x441f78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441fe0); }
 let mut pc: u32 = 0x441fe0;
 loop { match pc {
 0x441fe0 => {
  cpu.push(0x441fe5); f_00441fe6(cpu); if cpu.returned_to != 0x441fe5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441fe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441fe6); }
 let mut pc: u32 = 0x441fe6;
 loop { match pc {
 0x441fe6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00441ff7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x441ff7); }
 let mut pc: u32 = 0x441ff7;
 loop { match pc {
 0x441ff7 => {
  cpu.push(0x441ffc); f_0041b1ef(cpu); if cpu.returned_to != 0x441ffc { return; }
  cpu.push(0x442001); f_004184ca(cpu); if cpu.returned_to != 0x442001 { return; }
  cpu.push(0x442006); f_00418bd9(cpu); if cpu.returned_to != 0x442006 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6774u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6776u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44201b); f_00418bd9(cpu); if cpu.returned_to != 0x44201b { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x442027); f_0046d5b9(cpu); if cpu.returned_to != 0x442027 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x442289; continue; }
  pc = 0x442036; continue;
 }
 0x442036 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x44203c); f_004184ca(cpu); if cpu.returned_to != 0x44203c { return; }
  cpu.push(0x442041); f_00418bd9(cpu); if cpu.returned_to != 0x442041 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x4420d2; continue; }
  pc = 0x44204c; continue;
 }
 0x44204c => {
  { cpu.sub16(cpu.r16(0x4d6856u32), 0xaf7u16); }
  if cpu.zf { pc = 0x4420a2; continue; }
  pc = 0x442057; continue;
 }
 0x442057 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x4420a2; continue; }
  pc = 0x44205e; continue;
 }
 0x44205e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x442070); f_0046c3d4(cpu); if cpu.returned_to != 0x442070 { return; }
  if cpu.cf { pc = 0x442156; continue; }
  pc = 0x442076; continue;
 }
 0x442076 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x442088); f_0042e0fa(cpu); if cpu.returned_to != 0x442088 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x4420d2; continue; }
  pc = 0x442099; continue;
 }
 0x442099 => {
  { cpu.pushad(); }
  cpu.push(0x44209f); f_004481e0(cpu); if cpu.returned_to != 0x44209f { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x4420d2; continue; }
  pc = 0x4420a2; continue;
 }
 0x4420a2 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x4420bb); f_00442681(cpu); if cpu.returned_to != 0x4420bb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1564u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4420d2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4420d9); f_00418bd9(cpu); if cpu.returned_to != 0x4420d9 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4420df); f_004712e3(cpu); if cpu.returned_to != 0x4420df { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x4420e4; continue; }
  pc = 0x4420e4; continue;
 }
 0x4420e4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4420eb); f_0042e0fa(cpu); if cpu.returned_to != 0x4420eb { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59d9f4u32); cpu.esi = v; }
  if !cpu.zf { pc = 0x442158; continue; }
  pc = 0x4420fb; continue;
 }
 0x4420fb => {
  { cpu.pushad(); }
  cpu.push(0x442101); f_004481e0(cpu); if cpu.returned_to != 0x442101 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x442158; continue; }
  pc = 0x442104; continue;
 }
 0x442104 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x442156; continue; }
  pc = 0x44210b; continue;
 }
 0x44210b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x442116); f_0041b7b0(cpu); if cpu.returned_to != 0x442116 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x44213f); f_00442681(cpu); if cpu.returned_to != 0x44213f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1564u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  pc = 0x442156; continue;
 }
 0x442156 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x442158 => {
  cpu.push(0x44215d); f_0046d86b(cpu); if cpu.returned_to != 0x44215d { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x442175); f_0041b7b0(cpu); if cpu.returned_to != 0x442175 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x442191); f_00442681(cpu); if cpu.returned_to != 0x442191 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x4421a6); f_00442636(cpu); if cpu.returned_to != 0x4421a6 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f50u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c6u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x4421db; continue; }
  pc = 0x4421d4; continue;
 }
 0x4421d4 => {
  cpu.push(0x4421d9); f_004165a2(cpu); if cpu.returned_to != 0x4421d9 { return; }
  pc = 0x4421e0; continue;
 }
 0x4421db => {
  cpu.push(0x4421e0); f_00416654(cpu); if cpu.returned_to != 0x4421e0 { return; }
  pc = 0x4421e0; continue;
 }
 0x4421e0 => {
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x4421f5; continue; }
  pc = 0x4421e9; continue;
 }
 0x4421e9 => {
  cpu.push(0x4421ee); f_0041879e(cpu); if cpu.returned_to != 0x4421ee { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x4421f5; continue;
 }
 0x4421f5 => {
  cpu.push(0x4421fa); f_0044236a(cpu); if cpu.returned_to != 0x4421fa { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x442289 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x4d68aeu32, v); }
  cpu.push(0x44229a); f_0044234d(cpu); if cpu.returned_to != 0x44229a { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6b04u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x4422d4); f_00442636(cpu); if cpu.returned_to != 0x4422d4 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4e159fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f50u32, v); }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6786u32, v); }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6788u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x442312); f_0041b7b0(cpu); if cpu.returned_to != 0x442312 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x442318); f_00442681(cpu); if cpu.returned_to != 0x442318 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x44232d; continue; }
  pc = 0x442326; continue;
 }
 0x442326 => {
  cpu.push(0x44232b); f_004165a2(cpu); if cpu.returned_to != 0x44232b { return; }
  pc = 0x442332; continue;
 }
 0x44232d => {
  cpu.push(0x442332); f_00416654(cpu); if cpu.returned_to != 0x442332 { return; }
  pc = 0x442332; continue;
 }
 0x442332 => {
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x442347; continue; }
  pc = 0x44233b; continue;
 }
 0x44233b => {
  cpu.push(0x442340); f_0041879e(cpu); if cpu.returned_to != 0x442340 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x442347; continue;
 }
 0x442347 => {
  cpu.push(0x44234c); f_0044236a(cpu); if cpu.returned_to != 0x44234c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044234d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44234d); }
 let mut pc: u32 = 0x44234d;
 loop { match pc {
 0x44234d => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x442359; continue; }
  pc = 0x442353; continue;
 }
 0x442353 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x442359 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { let v: u16 = 0xef1fu16; cpu.w16(0x4d68aeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044236a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44236a); }
 let mut pc: u32 = 0x44236a;
 loop { match pc {
 0x44236a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044237b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44237b); }
 let mut pc: u32 = 0x44237b;
 loop { match pc {
 0x44237b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x442398); f_00418bd9(cpu); if cpu.returned_to != 0x442398 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.zf { pc = 0x442447; continue; }
  pc = 0x4423ab; continue;
 }
 0x4423ab => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x575545u32), 0x0u8); }
  if cpu.zf { pc = 0x4423d6; continue; }
  pc = 0x4423b5; continue;
 }
 0x4423b5 => {
  { cpu.pushad(); }
  cpu.push(0x4423bb); f_00472452(cpu); if cpu.returned_to != 0x4423bb { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x4423d6; continue; }
  pc = 0x4423be; continue;
 }
 0x4423be => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4b94bcu32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44245d; continue;
 }
 0x4423d6 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x59dafdu32, v); }
  cpu.push(0x4423e3); f_004716d0(cpu); if cpu.returned_to != 0x4423e3 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6b64u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x442447; continue; }
  pc = 0x4423f2; continue;
 }
 0x4423f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.ecx as u16), 0x1fu16); }
  if !cpu.zf { pc = 0x442447; continue; }
  pc = 0x442411; continue;
 }
 0x442411 => {
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x44243b; continue; }
  pc = 0x44242c; continue;
 }
 0x44242c => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x44243b; continue; }
  pc = 0x442433; continue;
 }
 0x442433 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44243b; continue;
 }
 0x44243b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44245d; continue;
 }
 0x442447 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x44244d); f_004712e3(cpu); if cpu.returned_to != 0x44244d { return; }
  if cpu.cf { pc = 0x44245d; continue; }
  pc = 0x44244f; continue;
 }
 0x44244f => {
  cpu.push(0x442454); f_0041a7e3(cpu); if cpu.returned_to != 0x442454 { return; }
  if cpu.cf { pc = 0x44245d; continue; }
  pc = 0x442456; continue;
 }
 0x442456 => {
  cpu.push(0x44245b); f_0041a69f(cpu); if cpu.returned_to != 0x44245b { return; }
  if cpu.cf { pc = 0x44248b; continue; }
  pc = 0x44245d; continue;
 }
 0x44245d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x543110u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x543112u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x543114u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x55b6b4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x55b6b6u32, v); }
  pc = 0x442493; continue;
 }
 0x44248b => {
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x442493 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x4424e9; continue; }
  pc = 0x4424af; continue;
 }
 0x4424af => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x4424e9; continue; }
  pc = 0x4424b9; continue;
 }
 0x4424b9 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x4424d7; continue; }
  pc = 0x4424c3; continue;
 }
 0x4424c3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4424e9; continue;
 }
 0x4424d7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4424e9; continue;
 }
 0x4424e9 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaceu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x575545u32), 0x0u8); }
  if !cpu.zf { pc = 0x442503; continue; }
  pc = 0x4424fe; continue;
 }
 0x4424fe => {
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  pc = 0x442503; continue;
 }
 0x442503 => {
  cpu.push(0x442508); f_0041b7b0(cpu); if cpu.returned_to != 0x442508 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.r32(0x543110u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68aeu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x442526); f_00442681(cpu); if cpu.returned_to != 0x442526 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x442545); f_00442636(cpu); if cpu.returned_to != 0x442545 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x4e1f50u32, v); }
  cpu.push(0x442558); f_004165fb(cpu); if cpu.returned_to != 0x442558 { return; }
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x44256d; continue; }
  pc = 0x442561; continue;
 }
 0x442561 => {
  cpu.push(0x442566); f_0041879e(cpu); if cpu.returned_to != 0x442566 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x44256d; continue;
 }
 0x44256d => {
  cpu.push(0x442572); f_0044236a(cpu); if cpu.returned_to != 0x442572 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044257a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44257a); }
 let mut pc: u32 = 0x44257a;
 loop { match pc {
 0x44257a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4e1594u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4e1596u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e159eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xad6u32; cpu.eax = v; }
  cpu.push(0x4425dc); f_0041b7b0(cpu); if cpu.returned_to != 0x4425dc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x4425e9); f_00442681(cpu); if cpu.returned_to != 0x4425e9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6780u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x4e1564u32, v); }
  cpu.push(0x442608); f_00442636(cpu); if cpu.returned_to != 0x442608 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x4e1f50u32, v); }
  cpu.push(0x44261b); f_004165fb(cpu); if cpu.returned_to != 0x44261b { return; }
  { cpu.sub32(cpu.r32(0x58c866u32), 0x0u32); }
  if !cpu.zf { pc = 0x442630; continue; }
  pc = 0x442624; continue;
 }
 0x442624 => {
  cpu.push(0x442629); f_0041879e(cpu); if cpu.returned_to != 0x442629 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  pc = 0x442630; continue;
 }
 0x442630 => {
  cpu.push(0x442635); f_0044236a(cpu); if cpu.returned_to != 0x442635 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442636(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442636); }
 let mut pc: u32 = 0x442636;
 loop { match pc {
 0x442636 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4e159fu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x442647); f_00418bd9(cpu); if cpu.returned_to != 0x442647 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67d6u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d67d8u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e159eu32), 0x0u8); }
  if !cpu.zf { pc = 0x44266b; continue; }
  pc = 0x44265e; continue;
 }
 0x44265e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4e1594u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4e1596u32, v); }
  pc = 0x44266b; continue;
 }
 0x44266b => {
  { let v: u16 = cpu.r16(0x4d6856u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4e1598u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e159eu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442681(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442681); }
 let mut pc: u32 = 0x442681;
 loop { match pc {
 0x442681 => {
  { cpu.sub8(cpu.r8(0x5928aeu32), 0x0u8); }
  if cpu.zf { pc = 0x442691; continue; }
  pc = 0x44268a; continue;
 }
 0x44268a => {
  { let v: u8 = cpu.add8(cpu.r8(0x4e1f4cu32), 0x20u8); cpu.w8(0x4e1f4cu32, v); }
  pc = 0x442691; continue;
 }
 0x442691 => {
  cpu.push(0x442696); f_00413337(cpu); if cpu.returned_to != 0x442696 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442698); }
 let mut pc: u32 = 0x442698;
 loop { match pc {
 0x442698 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x88u32); }
  if (cpu.sf == cpu.of) { pc = 0x44270a; continue; }
  pc = 0x4426a5; continue;
 }
 0x4426a5 => {
  { let v: u32 = cpu.r32(0x4d6a1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x56c6f6u32, v); }
  { let v: u32 = 0xddu32; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5b3594u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4426e8; continue; }
  pc = 0x4426c7; continue;
 }
 0x4426c7 => {
  { let v: u32 = 0x5b387eu32; cpu.w32(0x5b3590u32, v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x442707; continue; }
  pc = 0x4426da; continue;
 }
 0x4426da => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x4426e5); f_00416a62(cpu); if cpu.returned_to != 0x4426e5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x442707; continue;
 }
 0x4426e8 => {
  { let v: u32 = 0x5b3882u32; cpu.w32(0x5b3590u32, v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x442707; continue; }
  pc = 0x4426fb; continue;
 }
 0x4426fb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x442706); f_00416a62(cpu); if cpu.returned_to != 0x442706 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x442707; continue;
 }
 0x442707 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44270a => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004427d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4427d7); }
 let mut pc: u32 = 0x4427d7;
 loop { match pc {
 0x4427d7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  { cpu.sub32(cpu.r32(0x5b3588u32), cpu.ebp); }
  if !cpu.zf { pc = 0x4427f0; continue; }
  pc = 0x4427e6; continue;
 }
 0x4427e6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  pc = 0x4427f0; continue;
 }
 0x4427f0 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x442805; continue; }
  pc = 0x4427f7; continue;
 }
 0x4427f7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x442805 => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.r32(0x5a4054u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x442845; continue; }
  pc = 0x442841; continue;
 }
 0x442841 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x442845; continue;
 }
 0x442845 => {
  cpu.push(0x44284a); f_004435ee(cpu); if cpu.returned_to != 0x44284a { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r16(0x4d6938u32) as u32; cpu.ecx = v; }
  let t = cpu.r32(0x444153u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x442859); cpu.call(t); if cpu.returned_to != 0x442859 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x442862); f_00444177(cpu); if cpu.returned_to != 0x442862 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x442871); f_0044372e(cpu); if cpu.returned_to != 0x442871 { return; }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x442889; continue; }
  pc = 0x44287c; continue;
 }
 0x44287c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x442888); f_004586f4(cpu); if cpu.returned_to != 0x442888 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x442889; continue;
 }
 0x442889 => {
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
  { let v: u8 = 0x0u8; cpu.w8(0x57925du32, v); }
  cpu.push(0x4428c3); f_0045bec8(cpu); if cpu.returned_to != 0x4428c3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004428c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4428c4); }
 let mut pc: u32 = 0x4428c4;
 loop { match pc {
 0x4428c4 => {
  { let v: u32 = cpu.r32(0x4d6a20u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x384u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6a20u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442d56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442d56); }
 let mut pc: u32 = 0x442d56;
 loop { match pc {
 0x442d56 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x442d60; continue;
 }
 0x442d60 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x442dc1; continue; }
  pc = 0x442d68; continue;
 }
 0x442d68 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x442d8b; continue; }
  pc = 0x442d80; continue;
 }
 0x442d80 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x442da7; continue; }
  pc = 0x442d87; continue;
 }
 0x442d87 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x442dc1; continue;
 }
 0x442d8b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x614u32; cpu.ecx = v; }
  cpu.push(0x442da3); f_00442dc8(cpu); if cpu.returned_to != 0x442da3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x442dc1; continue;
 }
 0x442da7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x613u32; cpu.ecx = v; }
  cpu.push(0x442dbf); f_00442dc8(cpu); if cpu.returned_to != 0x442dbf { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x442dc1; continue;
 }
 0x442dc1 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x442d60; continue; }
  pc = 0x442dc7; continue;
 }
 0x442dc7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442dc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442dc8); }
 let mut pc: u32 = 0x442dc8;
 loop { match pc {
 0x442dc8 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x442e00; continue; }
  pc = 0x442dd5; continue;
 }
 0x442dd5 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x442e00; continue; }
  pc = 0x442de2; continue;
 }
 0x442de2 => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x442e00; continue; }
  pc = 0x442dee; continue;
 }
 0x442dee => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x442e00; continue; }
  pc = 0x442dfa; continue;
 }
 0x442dfa => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x442e00); cpu.call(t); if cpu.returned_to != 0x442e00 { return; }
  pc = 0x442e00; continue;
 }
 0x442e00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442e01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x442e01); }
 let mut pc: u32 = 0x442e01;
 loop { match pc {
 0x442e01 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x4u8); }
  if cpu.zf { pc = 0x442e35; continue; }
  pc = 0x442e0f; continue;
 }
 0x442e0f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575f19u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x442e2b; continue; }
  pc = 0x442e24; continue;
 }
 0x442e24 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x442e35; continue; }
  pc = 0x442e2b; continue;
 }
 0x442e2b => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x442e35; continue; }
  pc = 0x442e30; continue;
 }
 0x442e30 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x442e35; continue;
 }
 0x442e35 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00442e36(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x442e36); } r_00442e36(cpu, 0x442e36); }
pub fn f_00442e39(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x442e39); } r_00442e36(cpu, 0x442e39); }
fn r_00442e36(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x442e36 => {
  cpu.push(0x442e3b); f_0046a31c(cpu); if cpu.returned_to != 0x442e3b { return; }
  cpu.push(0x442e40); f_00445d40(cpu); if cpu.returned_to != 0x442e40 { return; }
  cpu.push(0x442e45); f_0044665e(cpu); if cpu.returned_to != 0x442e45 { return; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x442e4f; continue;
 }
 0x442e39 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x442e40); f_00445d40(cpu); if cpu.returned_to != 0x442e40 { return; }
  cpu.push(0x442e45); f_0044665e(cpu); if cpu.returned_to != 0x442e45 { return; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x442e4f; continue;
 }
 0x442e4f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4433e8; continue; }
  pc = 0x442e5b; continue;
 }
 0x442e5b => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x442ff2; continue; }
  pc = 0x442e68; continue;
 }
 0x442e68 => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x442ff2; continue; }
  pc = 0x442e76; continue;
 }
 0x442e76 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x4u8); }
  if cpu.zf { pc = 0x442ecf; continue; }
  pc = 0x442e8a; continue;
 }
 0x442e8a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x442eaa; continue; }
  pc = 0x442e9a; continue;
 }
 0x442e9a => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x442ecf; continue; }
  pc = 0x442ea3; continue;
 }
 0x442ea3 => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x442eb8; continue;
 }
 0x442eaa => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x442ecf; continue; }
  pc = 0x442eb3; continue;
 }
 0x442eb3 => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x442eb8; continue;
 }
 0x442eb8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x575f19u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x442ecc; continue; }
  pc = 0x442ec7; continue;
 }
 0x442ec7 => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x442ecc; continue;
 }
 0x442ecc => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x442ecf; continue;
 }
 0x442ecf => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x442ee4; continue; }
  pc = 0x442ede; continue;
 }
 0x442ede => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x442ee4; continue;
 }
 0x442ee4 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x442efc; continue; }
  pc = 0x442ee9; continue;
 }
 0x442ee9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x442efa; continue; }
  pc = 0x442ef2; continue;
 }
 0x442ef2 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.sf { pc = 0x442efc; continue; }
  pc = 0x442ef6; continue;
 }
 0x442ef6 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x442efc; continue;
 }
 0x442efa => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x442efc; continue;
 }
 0x442efc => {
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
  if !cpu.zf { pc = 0x442f22; continue; }
  pc = 0x442f17; continue;
 }
 0x442f17 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x442f21); f_0048a7d1(cpu); if cpu.returned_to != 0x442f21 { return; }
  { cpu.popad(); }
  pc = 0x442f22; continue;
 }
 0x442f22 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x4u8); }
  if cpu.zf { pc = 0x442f77; continue; }
  pc = 0x442f32; continue;
 }
 0x442f32 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x442f52; continue; }
  pc = 0x442f42; continue;
 }
 0x442f42 => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x442f77; continue; }
  pc = 0x442f4b; continue;
 }
 0x442f4b => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x442f60; continue;
 }
 0x442f52 => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x442f77; continue; }
  pc = 0x442f5b; continue;
 }
 0x442f5b => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x442f60; continue;
 }
 0x442f60 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x575f19u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x442f74; continue; }
  pc = 0x442f6f; continue;
 }
 0x442f6f => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x442f74; continue;
 }
 0x442f74 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x442f77; continue;
 }
 0x442f77 => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x442f8c; continue; }
  pc = 0x442f86; continue;
 }
 0x442f86 => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x442f8c; continue;
 }
 0x442f8c => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x442fa4; continue; }
  pc = 0x442f91; continue;
 }
 0x442f91 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x442fa2; continue; }
  pc = 0x442f9a; continue;
 }
 0x442f9a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.sf { pc = 0x442fa4; continue; }
  pc = 0x442f9e; continue;
 }
 0x442f9e => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x442fa4; continue;
 }
 0x442fa2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x442fa4; continue;
 }
 0x442fa4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x45u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.ebx); cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.sf { pc = 0x442ff2; continue; }
  pc = 0x442fb1; continue;
 }
 0x442fb1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x442fba); f_0044357a(cpu); if cpu.returned_to != 0x442fba { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x442fce; continue; }
  pc = 0x442fc7; continue;
 }
 0x442fc7 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x442fec; continue;
 }
 0x442fce => {
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
  pc = 0x442fec; continue;
 }
 0x442fec => {
  { let v: u32 = cpu.edx; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x442ff2; continue;
 }
 0x442ff2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f1eu32, v); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x443113; continue; }
  pc = 0x44300c; continue;
 }
 0x44300c => {
  { cpu.sub32(cpu.r32(0x576e72u32), 0x0u32); }
  if !cpu.zf { pc = 0x443113; continue; }
  pc = 0x443019; continue;
 }
 0x443019 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x443113; continue; }
  pc = 0x443029; continue;
 }
 0x443029 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x4430cf; continue; }
  pc = 0x443034; continue;
 }
 0x443034 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x443113; continue; }
  pc = 0x443041; continue;
 }
 0x443041 => {
  { let v: u32 = 0x4d67a2u32; cpu.esi = v; }
  { let v: u32 = 0x4d67a4u32; cpu.edi = v; }
  { let v: u32 = 0x5159f4u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x1u8); }
  if cpu.zf { pc = 0x4430a3; continue; }
  pc = 0x443059; continue;
 }
 0x443059 => {
  { let v: u32 = 0x4d67a6u32; cpu.esi = v; }
  { let v: u32 = 0x4d67a8u32; cpu.edi = v; }
  { let v: u32 = 0x515d78u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x0u8); }
  if cpu.zf { pc = 0x4430a3; continue; }
  pc = 0x443071; continue;
 }
 0x443071 => {
  { let v: u32 = 0x4d67aau32; cpu.esi = v; }
  { let v: u32 = 0x4d67acu32; cpu.edi = v; }
  { let v: u32 = 0x5160fcu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x2u8); }
  if cpu.zf { pc = 0x4430a3; continue; }
  pc = 0x443089; continue;
 }
 0x443089 => {
  { let v: u32 = 0x4d67aeu32; cpu.esi = v; }
  { let v: u32 = 0x4d67b0u32; cpu.edi = v; }
  { let v: u32 = 0x516480u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x3u8); }
  if cpu.zf { pc = 0x4430a3; continue; }
  pc = 0x4430a1; continue;
 }
 0x4430a1 => {
  pc = 0x443113; continue;
 }
 0x4430a3 => {
  cpu.push(0x4430a8); f_00442e01(cpu); if cpu.returned_to != 0x4430a8 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.ecx as u16)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x4430c7; continue; }
  pc = 0x4430b5; continue;
 }
 0x4430b5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x4430c5); f_0046797f(cpu); if cpu.returned_to != 0x4430c5 { return; }
  pc = 0x443113; continue;
 }
 0x4430c7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x443113; continue;
 }
 0x4430cf => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x443113; continue; }
  pc = 0x4430d9; continue;
 }
 0x4430d9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x443113; continue; }
  pc = 0x4430e2; continue;
 }
 0x4430e2 => {
  cpu.push(0x4430e7); f_00442e01(cpu); if cpu.returned_to != 0x4430e7 { return; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4430fd; continue; }
  pc = 0x4430f8; continue;
 }
 0x4430f8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4430fd; continue;
 }
 0x4430fd => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443113); f_0046797f(cpu); if cpu.returned_to != 0x443113 { return; }
  pc = 0x443113; continue;
 }
 0x443113 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44315f; continue; }
  pc = 0x44311c; continue;
 }
 0x44311c => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x44315f; continue; }
  pc = 0x443126; continue;
 }
 0x443126 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x443142; continue; }
  pc = 0x44312f; continue;
 }
 0x44312f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5a3edcu32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a3edcu32).wrapping_add(0x4u32); cpu.w32(0x5a3edcu32, v); }
  pc = 0x44315f; continue;
 }
 0x443142 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44315f; continue; }
  pc = 0x44314e; continue;
 }
 0x44314e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5a3ed8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5a3ed8u32).wrapping_add(0x4u32); cpu.w32(0x5a3ed8u32, v); }
  pc = 0x44315f; continue;
 }
 0x44315f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x34u32.wrapping_add(cpu.ebp))); cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x443170; continue; }
  pc = 0x443169; continue;
 }
 0x443169 => {
  { let v: u16 = 0x0u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  pc = 0x443170; continue;
 }
 0x443170 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x3au32.wrapping_add(cpu.esi)); cpu.push(0x443182); cpu.call(t); if cpu.returned_to != 0x443182 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5884d2u32), 0x0u8); }
  if !cpu.zf { pc = 0x4431ab; continue; }
  pc = 0x44318d; continue;
 }
 0x44318d => {
  { cpu.sub8(cpu.r8(0x575f18u32), 0x0u8); }
  if cpu.zf { pc = 0x44331e; continue; }
  pc = 0x44319a; continue;
 }
 0x44319a => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x44331e; continue; }
  pc = 0x4431a8; continue;
 }
 0x4431a8 => {
  { cpu.pushad(); }
  pc = 0x4431e7; continue;
 }
 0x4431ab => {
  { cpu.sub16(cpu.r16(0x49u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x44331e; continue; }
  pc = 0x4431b8; continue;
 }
 0x4431b8 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x575f1eu32, v); }
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
  pc = 0x4431fd; continue;
 }
 0x4431e7 => {
  { let v: u32 = cpu.r32(0x45u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x44331d; continue; }
  pc = 0x4431f4; continue;
 }
 0x4431f4 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x44331d; continue; }
  pc = 0x4431fd; continue;
 }
 0x4431fd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x575f1eu32), 0x4u8); }
  if cpu.zf { pc = 0x443252; continue; }
  pc = 0x44320d; continue;
 }
 0x44320d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44322d; continue; }
  pc = 0x44321d; continue;
 }
 0x44321d => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x443252; continue; }
  pc = 0x443226; continue;
 }
 0x443226 => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x44323b; continue;
 }
 0x44322d => {
  { cpu.sub8(cpu.r8(0x575f19u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x443252; continue; }
  pc = 0x443236; continue;
 }
 0x443236 => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x44323b; continue;
 }
 0x44323b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x575f19u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x44324f; continue; }
  pc = 0x44324a; continue;
 }
 0x44324a => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x44324f; continue;
 }
 0x44324f => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x443252; continue;
 }
 0x443252 => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x443267; continue; }
  pc = 0x443261; continue;
 }
 0x443261 => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x443267; continue;
 }
 0x443267 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x44327f; continue; }
  pc = 0x44326c; continue;
 }
 0x44326c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x44327d; continue; }
  pc = 0x443275; continue;
 }
 0x443275 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.sf { pc = 0x44327f; continue; }
  pc = 0x443279; continue;
 }
 0x443279 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x44327f; continue;
 }
 0x44327d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x44327f; continue;
 }
 0x44327f => {
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
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x4432ed; continue; }
  pc = 0x4432c5; continue;
 }
 0x4432c5 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x4432ed; continue; }
  pc = 0x4432cb; continue;
 }
 0x4432cb => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x4432ed; continue; }
  pc = 0x4432d1; continue;
 }
 0x4432d1 => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x4432ed; continue; }
  pc = 0x4432d8; continue;
 }
 0x4432d8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x4432ea); f_0041b671(cpu); if cpu.returned_to != 0x4432ea { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4432ed; continue;
 }
 0x4432ed => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x44331d; continue; }
  pc = 0x443305; continue;
 }
 0x443305 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x44331d; continue; }
  pc = 0x44330b; continue;
 }
 0x44330b => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x44331d; continue; }
  pc = 0x443311; continue;
 }
 0x443311 => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x44331d; continue; }
  pc = 0x443318; continue;
 }
 0x443318 => {
  cpu.push(0x44331d); f_0041b671(cpu); if cpu.returned_to != 0x44331d { return; }
  pc = 0x44331d; continue;
 }
 0x44331d => {
  { cpu.popad(); }
  pc = 0x44331e; continue;
 }
 0x44331e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1fc5u32), 0x0u8); }
  if cpu.zf { pc = 0x4433e6; continue; }
  pc = 0x44332d; continue;
 }
 0x44332d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44333c); f_0041b74b(cpu); if cpu.returned_to != 0x44333c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x4433e6; continue; }
  pc = 0x44334f; continue;
 }
 0x44334f => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x4433e6; continue; }
  pc = 0x443361; continue;
 }
 0x443361 => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x30u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x4433e6; continue; }
  pc = 0x443371; continue;
 }
 0x443371 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x4433e6; continue; }
  pc = 0x443381; continue;
 }
 0x443381 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2fu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  let t = cpu.r32(0x4d6f54u32); cpu.push(0x443399); cpu.call(t); if cpu.returned_to != 0x443399 { return; }
  { cpu.popad(); }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  cpu.push(0x4433ab); f_00447fe3(cpu); if cpu.returned_to != 0x4433ab { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4433b5); f_00445c78(cpu); if cpu.returned_to != 0x4433b5 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4433ce; continue; }
  pc = 0x4433bb; continue;
 }
 0x4433bb => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x4433ce; continue;
 }
 0x4433ce => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4433d9); f_0041b74b(cpu); if cpu.returned_to != 0x4433d9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  cpu.push(0x4433e6); f_00447fe3(cpu); if cpu.returned_to != 0x4433e6 { return; }
  pc = 0x4433e6; continue;
 }
 0x4433e6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4433e8; continue;
 }
 0x4433e8 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x442e4f; continue; }
  pc = 0x4433f2; continue;
 }
 0x4433f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004433f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4433f3); }
 let mut pc: u32 = 0x4433f3;
 loop { match pc {
 0x4433f3 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x443455; continue; }
  pc = 0x4433fd; continue;
 }
 0x4433fd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5a2b3au32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a4613u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x443455; continue; }
  pc = 0x44342f; continue;
 }
 0x44342f => {
  { let v: u16 = cpu.r16(0x5a4611u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x443455); cpu.call(t); if cpu.returned_to != 0x443455 { return; }
  pc = 0x443455; continue;
 }
 0x443455 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443457(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443457); }
 let mut pc: u32 = 0x443457;
 loop { match pc {
 0x443457 => {
  { let v: u32 = 0x516804u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6a20u32, v); }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x443467; continue;
 }
 0x443467 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44346d); f_004674e8(cpu); if cpu.returned_to != 0x44346d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x443467; continue; }
  pc = 0x443470; continue;
 }
 0x443470 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44347a; continue;
 }
 0x44347a => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44356e; continue; }
  pc = 0x443486; continue;
 }
 0x443486 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
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
  if !cpu.zf { pc = 0x443533; continue; }
  pc = 0x44352c; continue;
 }
 0x44352c => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x443551; continue;
 }
 0x443533 => {
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
  pc = 0x443551; continue;
 }
 0x443551 => {
  { let v: u32 = cpu.edx; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x44355b); f_004428c4(cpu); if cpu.returned_to != 0x44355b { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  let t = cpu.r32(0x36u32.wrapping_add(cpu.esi)); cpu.push(0x443567); cpu.call(t); if cpu.returned_to != 0x443567 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x44356e); f_004435aa(cpu); if cpu.returned_to != 0x44356e { return; }
  pc = 0x44356e; continue;
 }
 0x44356e => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44347a; continue; }
  pc = 0x443579; continue;
 }
 0x443579 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044357a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44357a); }
 let mut pc: u32 = 0x44357a;
 loop { match pc {
 0x44357a => {
  { cpu.pushad(); }
  cpu.push(0x443580); f_0048aadc(cpu); if cpu.returned_to != 0x443580 { return; }
  if cpu.cf { pc = 0x4435a8; continue; }
  pc = 0x443582; continue;
 }
 0x443582 => {
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
  pc = 0x4435a8; continue;
 }
 0x4435a8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004435aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4435aa); }
 let mut pc: u32 = 0x4435aa;
 loop { match pc {
 0x4435aa => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4435ba); f_0047a2d9(cpu); if cpu.returned_to != 0x4435ba { return; }
  if cpu.cf { pc = 0x4435c0; continue; }
  pc = 0x4435bc; continue;
 }
 0x4435bc => {
  { let v: u32 = cpu.esi; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4435c0; continue;
 }
 0x4435c0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004435c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4435c3); }
 let mut pc: u32 = 0x4435c3;
 loop { match pc {
 0x4435c3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a3f44u32; cpu.esi = v; }
  cpu.push(0x4435ed); f_004435ee(cpu); if cpu.returned_to != 0x4435ed { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004435ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4435ee); } r_004435ee(cpu, 0x4435ee); }
pub fn f_0044361e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44361e); } r_004435ee(cpu, 0x44361e); }
fn r_004435ee(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4435ee => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0x5a3f44u32); }
  if cpu.zf { pc = 0x44360b; continue; }
  pc = 0x4435f7; continue;
 }
 0x4435f7 => {
  { cpu.sub32(cpu.esi, 0x5a4078u32); }
  if cpu.zf { pc = 0x44360b; continue; }
  pc = 0x4435ff; continue;
 }
 0x4435ff => {
  { cpu.sub32(cpu.esi, 0x5a3f30u32); }
  if cpu.zf { pc = 0x44360b; continue; }
  pc = 0x443607; continue;
 }
 0x443607 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44360b; continue;
 }
 0x44360b => {
  cpu.push(0x443610); f_00420989(cpu); if cpu.returned_to != 0x443610 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  pc = 0x443619; continue;
 }
 0x443619 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x44361c; continue;
 }
 0x44361c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x44361e; continue;
 }
 0x44361e => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44361c; continue; }
  pc = 0x443627; continue;
 }
 0x443627 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x443619; continue; }
  pc = 0x443632; continue;
 }
 0x443632 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443633(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443633); }
 let mut pc: u32 = 0x443633;
 loop { match pc {
 0x443633 => {
  { let v = cpu.r32(0x5b3d24u32); cpu.push(v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x5cc64bu32), 0x0u8); }
  if !cpu.zf { pc = 0x44364b; continue; }
  pc = 0x443643; continue;
 }
 0x443643 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44364b; continue;
 }
 0x44364b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x443658); f_004712e3(cpu); if cpu.returned_to != 0x443658 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x443663; continue; }
  pc = 0x44365b; continue;
 }
 0x44365b => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.w32(0x5b3d24u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x443663 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x44365b; continue; }
  pc = 0x443678; continue;
 }
 0x443678 => {
  { cpu.sub8(cpu.r8(0x4fu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x4436b5; continue; }
  pc = 0x44367f; continue;
 }
 0x44367f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x56c9ebu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4436b5; continue;
 }
 0x4436b5 => {
  { let v: u32 = (cpu.edx as u8) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4436c3; continue; }
  pc = 0x4436c1; continue;
 }
 0x4436c1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x4436c3; continue;
 }
 0x4436c3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44365b; continue; }
  pc = 0x4436cc; continue;
 }
 0x4436cc => {
  { cpu.sub8(cpu.r8(0x5cc647u32), 0x0u8); }
  if !cpu.zf { pc = 0x4436f4; continue; }
  pc = 0x4436d5; continue;
 }
 0x4436d5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x4436f1); f_00416949(cpu); if cpu.returned_to != 0x4436f1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4436f4; continue;
 }
 0x4436f4 => {
  { cpu.pushad(); }
  cpu.push(0x4436fa); f_004427d7(cpu); if cpu.returned_to != 0x4436fa { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x44365b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443700); }
 let mut pc: u32 = 0x443700;
 loop { match pc {
 0x443700 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  cpu.push(0x443721); f_00482f3b(cpu); if cpu.returned_to != 0x443721 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5a463du32), 0xau32); cpu.w32(0x5a463du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044372e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44372e); }
 let mut pc: u32 = 0x44372e;
 loop { match pc {
 0x44372e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x443738; continue;
 }
 0x443738 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x44374e; continue; }
  pc = 0x44373f; continue;
 }
 0x44373f => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), 0xfffeu16); }
  if !cpu.zf { pc = 0x443749; continue; }
  pc = 0x443747; continue;
 }
 0x443747 => {
  pc = 0x443751; continue;
 }
 0x443749 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x443738; continue; }
  pc = 0x44374e; continue;
 }
 0x44374e => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x443751 => {
  { let v: u32 = cpu.edi; cpu.w32(0x16u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443758(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443758); }
 let mut pc: u32 = 0x443758;
 loop { match pc {
 0x443758 => {
  { let v: u32 = cpu.r8(0x4e1f86u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u32 = 0x566fccu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xau16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a4641u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0xau8; cpu.w8(0x5a4645u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a463du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3du16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb4fu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4437e4); f_00443700(cpu); if cpu.returned_to != 0x4437e4 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3du16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb50u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44380e); f_00443700(cpu); if cpu.returned_to != 0x44380e { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb64u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443838); f_00443700(cpu); if cpu.returned_to != 0x443838 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb66u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443862); f_00443700(cpu); if cpu.returned_to != 0x443862 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb69u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44388c); f_00443700(cpu); if cpu.returned_to != 0x44388c { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4438b6); f_00443700(cpu); if cpu.returned_to != 0x4438b6 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4438e0); f_00443700(cpu); if cpu.returned_to != 0x4438e0 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44390a); f_00443700(cpu); if cpu.returned_to != 0x44390a { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443934); f_00443700(cpu); if cpu.returned_to != 0x443934 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44395e); f_00443700(cpu); if cpu.returned_to != 0x44395e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443965(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443965); }
 let mut pc: u32 = 0x443965;
 loop { match pc {
 0x443965 => {
  { let v: u32 = cpu.r8(0x4e1f86u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u32 = 0x5647ccu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xcu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a4641u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0xcu8; cpu.w8(0x5a4645u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a463du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x852u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4439f1); f_00443700(cpu); if cpu.returned_to != 0x4439f1 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x849u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443a1b); f_00443700(cpu); if cpu.returned_to != 0x443a1b { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x84eu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443a45); f_00443700(cpu); if cpu.returned_to != 0x443a45 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x46u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x850u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443a6f); f_00443700(cpu); if cpu.returned_to != 0x443a6f { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x37u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x84cu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443a99); f_00443700(cpu); if cpu.returned_to != 0x443a99 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xfu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x83eu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443ac3); f_00443700(cpu); if cpu.returned_to != 0x443ac3 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443aed); f_00443700(cpu); if cpu.returned_to != 0x443aed { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443b17); f_00443700(cpu); if cpu.returned_to != 0x443b17 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443b41); f_00443700(cpu); if cpu.returned_to != 0x443b41 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443b6b); f_00443700(cpu); if cpu.returned_to != 0x443b6b { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443b95); f_00443700(cpu); if cpu.returned_to != 0x443b95 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443bbf); f_00443700(cpu); if cpu.returned_to != 0x443bbf { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443bc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443bc6); }
 let mut pc: u32 = 0x443bc6;
 loop { match pc {
 0x443bc6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443c3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443c3e); }
 let mut pc: u32 = 0x443c3e;
 loop { match pc {
 0x443c3e => {
  { let v: u32 = cpu.r8(0x4e1f86u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u32 = 0x561fccu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x17u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a4641u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x5a4645u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a463du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1du16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb46u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443cca); f_00443700(cpu); if cpu.returned_to != 0x443cca { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x26u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x37u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb5cu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443cf4); f_00443700(cpu); if cpu.returned_to != 0x443cf4 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x854u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443d1e); f_00443700(cpu); if cpu.returned_to != 0x443d1e { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x836u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443d48); f_00443700(cpu); if cpu.returned_to != 0x443d48 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x846u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443d72); f_00443700(cpu); if cpu.returned_to != 0x443d72 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x856u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443d9c); f_00443700(cpu); if cpu.returned_to != 0x443d9c { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443da5); f_00443700(cpu); if cpu.returned_to != 0x443da5 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443dae); f_00443700(cpu); if cpu.returned_to != 0x443dae { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443db7); f_00443700(cpu); if cpu.returned_to != 0x443db7 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443dc0); f_00443700(cpu); if cpu.returned_to != 0x443dc0 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443dc9); f_00443700(cpu); if cpu.returned_to != 0x443dc9 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443df3); f_00443700(cpu); if cpu.returned_to != 0x443df3 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443dfc); f_00443700(cpu); if cpu.returned_to != 0x443dfc { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443e05); f_00443700(cpu); if cpu.returned_to != 0x443e05 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443e0e); f_00443700(cpu); if cpu.returned_to != 0x443e0e { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443e17); f_00443700(cpu); if cpu.returned_to != 0x443e17 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x443e20); f_00443700(cpu); if cpu.returned_to != 0x443e20 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443e4a); f_00443700(cpu); if cpu.returned_to != 0x443e4a { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443e74); f_00443700(cpu); if cpu.returned_to != 0x443e74 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443e9e); f_00443700(cpu); if cpu.returned_to != 0x443e9e { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443ec8); f_00443700(cpu); if cpu.returned_to != 0x443ec8 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443ef2); f_00443700(cpu); if cpu.returned_to != 0x443ef2 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443f1c); f_00443700(cpu); if cpu.returned_to != 0x443f1c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443f23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x443f23); }
 let mut pc: u32 = 0x443f23;
 loop { match pc {
 0x443f23 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x6a7u16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x14u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x443f5f); f_00443700(cpu); if cpu.returned_to != 0x443f5f { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6b3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443f71); f_00443700(cpu); if cpu.returned_to != 0x443f71 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6bfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443f83); f_00443700(cpu); if cpu.returned_to != 0x443f83 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x19u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6cbu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x443f99); f_00443700(cpu); if cpu.returned_to != 0x443f99 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00443f9a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x443f9a); } r_00443f9a(cpu, 0x443f9a); }
pub fn f_00444000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444000); } r_00443f9a(cpu, 0x444000); }
pub fn f_00444152(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444152); } r_00443f9a(cpu, 0x444152); }
fn r_00443f9a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x443f9a => {
  { let v: u32 = cpu.r8(0x4e1f86u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u32 = 0x5697ccu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a4641u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0x8u8; cpu.w8(0x5a4645u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a463du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb46u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444026); f_00443700(cpu); if cpu.returned_to != 0x444026 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x12u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x5dbu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444050); f_00443700(cpu); if cpu.returned_to != 0x444050 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb56u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44407a); f_00443700(cpu); if cpu.returned_to != 0x44407a { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb6du16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440a4); f_00443700(cpu); if cpu.returned_to != 0x4440a4 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440ce); f_00443700(cpu); if cpu.returned_to != 0x4440ce { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440f8); f_00443700(cpu); if cpu.returned_to != 0x4440f8 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444122); f_00443700(cpu); if cpu.returned_to != 0x444122 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44414c); f_00443700(cpu); if cpu.returned_to != 0x44414c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  pc = 0x444152; continue;
 }
 0x444000 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a463du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb46u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444026); f_00443700(cpu); if cpu.returned_to != 0x444026 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x12u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x5dbu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444050); f_00443700(cpu); if cpu.returned_to != 0x444050 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb56u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44407a); f_00443700(cpu); if cpu.returned_to != 0x44407a { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0xb6du16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440a4); f_00443700(cpu); if cpu.returned_to != 0x4440a4 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440ce); f_00443700(cpu); if cpu.returned_to != 0x4440ce { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4440f8); f_00443700(cpu); if cpu.returned_to != 0x4440f8 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4cfu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x444122); f_00443700(cpu); if cpu.returned_to != 0x444122 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4c3u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x44414c); f_00443700(cpu); if cpu.returned_to != 0x44414c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  pc = 0x444152; continue;
 }
 0x444152 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444145(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444145); }
 let mut pc: u32 = 0x444145;
 loop { match pc {
 0x444145 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444177); }
 let mut pc: u32 = 0x444177;
 loop { match pc {
 0x444177 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x44417f); f_00485e62(cpu); if cpu.returned_to != 0x44417f { return; }
  if cpu.cf { pc = 0x444223; continue; }
  pc = 0x444185; continue;
 }
 0x444185 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe2u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1dau16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1e1u16; cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x4441cc; continue;
 }
 0x4441cc => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4441d2); f_00485e62(cpu); if cpu.returned_to != 0x4441d2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x444223; continue; }
  pc = 0x4441d5; continue;
 }
 0x4441d5 => {
  cpu.push(0x4441da); f_0041c6a6(cpu); if cpu.returned_to != 0x4441da { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = 0x1fu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4441ea); f_0041c6a6(cpu); if cpu.returned_to != 0x4441ea { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe1u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x18cu16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x198u16; cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4441cc; continue; }
  pc = 0x444223; continue;
 }
 0x444223 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444225(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444225); }
 let mut pc: u32 = 0x444225;
 loop { match pc {
 0x444225 => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a3f30u32; cpu.esi = v; }
  cpu.push(0x444253); f_004435ee(cpu); if cpu.returned_to != 0x444253 { return; }
  cpu.push(0x444258); f_0041c743(cpu); if cpu.returned_to != 0x444258 { return; }
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
  { let v: u32 = 0x5a434cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x44428d); f_0046771f(cpu); if cpu.returned_to != 0x44428d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044428e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44428e); } r_0044428e(cpu, 0x44428e); }
pub fn f_00444320(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444320); } r_0044428e(cpu, 0x444320); }
pub fn f_00444449(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444449); } r_0044428e(cpu, 0x444449); }
pub fn f_00444547(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444547); } r_0044428e(cpu, 0x444547); }
pub fn f_0044454c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44454c); } r_0044428e(cpu, 0x44454c); }
pub fn f_00444552(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444552); } r_0044428e(cpu, 0x444552); }
pub fn f_00444554(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444554); } r_0044428e(cpu, 0x444554); }
fn r_0044428e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44428e => {
  { let v: u8 = 0x0u8; cpu.w8(0x5764eau32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a45d8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a45dau32, v); }
  cpu.push(0x4442bf); f_00444762(cpu); if cpu.returned_to != 0x4442bf { return; }
  { let v: u8 = 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4442d6; continue; }
  pc = 0x4442cb; continue;
 }
 0x4442cb => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x4442d6; continue; }
  pc = 0x4442d4; continue;
 }
 0x4442d4 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4442d6; continue;
 }
 0x4442d6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x3bu32.wrapping_add(cpu.ebp))); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4443a2; continue; }
  pc = 0x4442ff; continue;
 }
 0x4442ff => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4443a2; continue; }
  pc = 0x44430c; continue;
 }
 0x44430c => {
  { cpu.sub16((cpu.eax as u16), 0xffb0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4443a2; continue; }
  pc = 0x444316; continue;
 }
 0x444316 => {
  { cpu.sub16((cpu.ebx as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4443a2; continue; }
  pc = 0x444320; continue;
 }
 0x444320 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4445e4; continue; }
  pc = 0x44432d; continue;
 }
 0x44432d => {
  { let v: u32 = 0x5a3ee0u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x444349; continue; }
  pc = 0x444344; continue;
 }
 0x444344 => {
  cpu.push(0x444349); f_004449f5(cpu); if cpu.returned_to != 0x444349 { return; }
  pc = 0x444349; continue;
 }
 0x444349 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444355); cpu.call(t); if cpu.returned_to != 0x444355 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x44435e); f_00444791(cpu); if cpu.returned_to != 0x44435e { return; }
  cpu.push(0x444363); f_00444813(cpu); if cpu.returned_to != 0x444363 { return; }
  cpu.push(0x444368); f_0044489d(cpu); if cpu.returned_to != 0x444368 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x44437b; continue; }
  pc = 0x444377; continue;
 }
 0x444377 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44439d; continue;
 }
 0x44437b => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x444388); cpu.call(t); if cpu.returned_to != 0x444388 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b43u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44439c); cpu.call(t); if cpu.returned_to != 0x44439c { return; }
  { cpu.popad(); }
  pc = 0x44439d; continue;
 }
 0x44439d => {
  cpu.push(0x4443a2); f_00444652(cpu); if cpu.returned_to != 0x4443a2 { return; }
  pc = 0x4443a2; continue;
 }
 0x4443a2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x444638; continue; }
  pc = 0x4443af; continue;
 }
 0x4443af => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4443c5); f_004677e0(cpu); if cpu.returned_to != 0x4443c5 { return; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.cf { pc = 0x4445e3; continue; }
  pc = 0x4443d3; continue;
 }
 0x4443d3 => {
  { cpu.sub8(cpu.r8(0x5884d2u32), 0x0u8); }
  if !cpu.zf { pc = 0x4445e3; continue; }
  pc = 0x4443e0; continue;
 }
 0x4443e0 => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4445e3; continue; }
  pc = 0x4443ee; continue;
 }
 0x4443ee => {
  { let v: u16 = 0x1au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x29u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4443fb); f_00444aaf(cpu); if cpu.returned_to != 0x4443fb { return; }
  cpu.push(0x444400); f_004446a4(cpu); if cpu.returned_to != 0x444400 { return; }
  cpu.push(0x444405); f_00444940(cpu); if cpu.returned_to != 0x444405 { return; }
  cpu.push(0x44440a); f_00445c78(cpu); if cpu.returned_to != 0x44440a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4445e3; continue; }
  pc = 0x444417; continue;
 }
 0x444417 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebp)), 0x11u8); }
  if (cpu.sf != cpu.of) { pc = 0x4444d1; continue; }
  pc = 0x444422; continue;
 }
 0x444422 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if !cpu.zf { pc = 0x444435; continue; }
  pc = 0x444433; continue;
 }
 0x444433 => {
  pc = 0x444464; continue;
 }
 0x444435 => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x44445e; continue; }
  pc = 0x444452; continue;
 }
 0x444449 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.r32(0xc33b03e0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0xc33b03e0u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x44445e; continue; }
  pc = 0x444452; continue;
 }
 0x444452 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5764eau32, v); }
  pc = 0x4444db; continue;
 }
 0x44445e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if !cpu.zf { pc = 0x44447b; continue; }
  pc = 0x444464; continue;
 }
 0x444464 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4445e3; continue;
 }
 0x44447b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x444488; continue; }
  pc = 0x444483; continue;
 }
 0x444483 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44448d; continue; }
  pc = 0x444488; continue;
 }
 0x444488 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44448d; continue;
 }
 0x44448d => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4444ce; continue; }
  pc = 0x444497; continue;
 }
 0x444497 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4444a8); f_00446220(cpu); if cpu.returned_to != 0x4444a8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if cpu.zf { pc = 0x4444ce; continue; }
  pc = 0x4444af; continue;
 }
 0x4444af => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x4444ca; continue; }
  pc = 0x4444c6; continue;
 }
 0x4444c6 => {
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4444ca; continue;
 }
 0x4444ca => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x4444ce; continue;
 }
 0x4444ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4444d1; continue;
 }
 0x4444d1 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4445e3; continue; }
  pc = 0x4444db; continue;
 }
 0x4444db => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44450a; continue; }
  pc = 0x444505; continue;
 }
 0x444505 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44450f; continue; }
  pc = 0x44450a; continue;
 }
 0x44450a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44450f; continue;
 }
 0x44450f => {
  { cpu.sub8(cpu.r8(0x5764eau32), 0x0u8); }
  if !cpu.zf { pc = 0x444520; continue; }
  pc = 0x444518; continue;
 }
 0x444518 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x444520; continue;
 }
 0x444520 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x444538); f_0046ab56(cpu); if cpu.returned_to != 0x444538 { return; }
  cpu.push(0x44453d); f_00489968(cpu); if cpu.returned_to != 0x44453d { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x444547; continue;
 }
 0x444547 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x42u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x444583); f_0041b7b0(cpu); if cpu.returned_to != 0x444583 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444589); f_00413337(cpu); if cpu.returned_to != 0x444589 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x66u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4445c4; continue; }
  pc = 0x4445b0; continue;
 }
 0x44454c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x42u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x444583); f_0041b7b0(cpu); if cpu.returned_to != 0x444583 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444589); f_00413337(cpu); if cpu.returned_to != 0x444589 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x66u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4445c4; continue; }
  pc = 0x4445b0; continue;
 }
 0x444552 => {
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4e1f4cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6aec2du32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x4d6aec2du32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  { cpu.scas(1, 0); }
  { let v = 0x8966004du32; cpu.push(v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x4d68b0u32); cpu.eax = v; }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x444583); f_0041b7b0(cpu); if cpu.returned_to != 0x444583 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444589); f_00413337(cpu); if cpu.returned_to != 0x444589 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x66u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4445c4; continue; }
  pc = 0x4445b0; continue;
 }
 0x444554 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x444583); f_0041b7b0(cpu); if cpu.returned_to != 0x444583 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444589); f_00413337(cpu); if cpu.returned_to != 0x444589 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x66u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4445c4; continue; }
  pc = 0x4445b0; continue;
 }
 0x4445b0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) | 0x20u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4445c4; continue;
 }
 0x4445c4 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4445dc; continue; }
  pc = 0x4445cd; continue;
 }
 0x4445cd => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4445e1; continue; }
  pc = 0x4445dc; continue;
 }
 0x4445dc => {
  cpu.push(0x4445e1); f_00416a55(cpu); if cpu.returned_to != 0x4445e1 { return; }
  pc = 0x4445e1; continue;
 }
 0x4445e1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4445e3; continue;
 }
 0x4445e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4445e4 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a3ee0u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4445fc); cpu.call(t); if cpu.returned_to != 0x4445fc { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x444610; continue; }
  pc = 0x44460c; continue;
 }
 0x44460c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x444638; continue;
 }
 0x444610 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x44461d); cpu.call(t); if cpu.returned_to != 0x44461d { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x444638; continue; }
  pc = 0x444624; continue;
 }
 0x444624 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b43u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444637); cpu.call(t); if cpu.returned_to != 0x444637 { return; }
  { cpu.popad(); }
  pc = 0x444638; continue;
 }
 0x444638 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0x96u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x444651); f_004677e0(cpu); if cpu.returned_to != 0x444651 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044454d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44454d); }
 let mut pc: u32 = 0x44454d;
 loop { match pc {
 0x44454d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444652(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444652); }
 let mut pc: u32 = 0x444652;
 loop { match pc {
 0x444652 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4446a2; continue; }
  pc = 0x44465a; continue;
 }
 0x44465a => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44468e; continue; }
  pc = 0x444661; continue;
 }
 0x444661 => {
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x31u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x76du32); cpu.ecx = v; }
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44468d); cpu.call(t); if cpu.returned_to != 0x44468d { return; }
  { cpu.popad(); }
  pc = 0x44468e; continue;
 }
 0x44468e => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x4446a2; continue; }
  pc = 0x444695; continue;
 }
 0x444695 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x4446a2; continue; }
  pc = 0x44469e; continue;
 }
 0x44469e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4446a2; continue;
 }
 0x4446a2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004446a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4446a4); }
 let mut pc: u32 = 0x4446a4;
 loop { match pc {
 0x4446a4 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x4446fe; continue; }
  pc = 0x4446ae; continue;
 }
 0x4446ae => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4446c3; continue; }
  pc = 0x4446b6; continue;
 }
 0x4446b6 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) | (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4446fe; continue;
 }
 0x4446c3 => {
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4446ec); f_004862bf(cpu); if cpu.returned_to != 0x4446ec { return; }
  cpu.push(0x4446f1); f_0041c743(cpu); if cpu.returned_to != 0x4446f1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) | (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4446fe; continue;
 }
 0x4446fe => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x444760; continue; }
  pc = 0x444707; continue;
 }
 0x444707 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x444722; continue; }
  pc = 0x44470f; continue;
 }
 0x44470f => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444760; continue;
 }
 0x444722 => {
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x38u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44474b); f_004862bf(cpu); if cpu.returned_to != 0x44474b { return; }
  cpu.push(0x444750); f_0041c743(cpu); if cpu.returned_to != 0x444750 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444760; continue;
 }
 0x444760 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444762(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444762); } r_00444762(cpu, 0x444762); }
pub fn f_0044489d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44489d); } r_00444762(cpu, 0x44489d); }
pub fn f_00444d58(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444d58); } r_00444762(cpu, 0x444d58); }
fn r_00444762(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x444762 => {
  { cpu.pushad(); }
  cpu.push(0x444768); f_0044664d(cpu); if cpu.returned_to != 0x444768 { return; }
  if !cpu.cf { pc = 0x44493e; continue; }
  pc = 0x44476e; continue;
 }
 0x44476e => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x44493e; continue; }
  pc = 0x444779; continue;
 }
 0x444779 => {
  { let v: u8 = 0x4u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x44478e); f_004435c3(cpu); if cpu.returned_to != 0x44478e { return; }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44489d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb69u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44493e; continue; }
  pc = 0x4448c7; continue;
 }
 0x4448c7 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44493e; continue; }
  pc = 0x4448d0; continue;
 }
 0x4448d0 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44493e; continue; }
  pc = 0x4448d6; continue;
 }
 0x4448d6 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44493e; continue; }
  pc = 0x4448dc; continue;
 }
 0x4448dc => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4448e3); cpu.call(t); if cpu.returned_to != 0x4448e3 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4448ee); f_0044664d(cpu); if cpu.returned_to != 0x4448ee { return; }
  if !cpu.cf { pc = 0x44493e; continue; }
  pc = 0x4448f0; continue;
 }
 0x4448f0 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x444deb; continue; }
  pc = 0x44491c; continue;
 }
 0x44491c => {
  { let v: u8 = 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x44493e; continue; }
  pc = 0x444928; continue;
 }
 0x444928 => {
  { let v: u8 = 0x4u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x44493d); f_004435c3(cpu); if cpu.returned_to != 0x44493d { return; }
  { cpu.popad(); }
  pc = 0x44493e; continue;
 }
 0x44493e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x444d58 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb43u32; cpu.ecx = v; }
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444deb; continue; }
  pc = 0x444d86; continue;
 }
 0x444d86 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444deb; continue; }
  pc = 0x444d8f; continue;
 }
 0x444d8f => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444deb; continue; }
  pc = 0x444d95; continue;
 }
 0x444d95 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444deb; continue; }
  pc = 0x444d9b; continue;
 }
 0x444d9b => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444da2); cpu.call(t); if cpu.returned_to != 0x444da2 { return; }
  { cpu.popad(); }
  cpu.push(0x444da8); f_0044664d(cpu); if cpu.returned_to != 0x444da8 { return; }
  if !cpu.cf { pc = 0x444deb; continue; }
  pc = 0x444daa; continue;
 }
 0x444daa => {
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
  if (cpu.cf || cpu.zf) { pc = 0x444deb; continue; }
  pc = 0x444dd2; continue;
 }
 0x444dd2 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x444deb; continue; }
  pc = 0x444ddb; continue;
 }
 0x444ddb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if !cpu.zf { pc = 0x444deb; continue; }
  pc = 0x444de6; continue;
 }
 0x444de6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444deb; continue;
 }
 0x444deb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444791); }
 let mut pc: u32 = 0x444791;
 loop { match pc {
 0x444791 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb64u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444811; continue; }
  pc = 0x4447bb; continue;
 }
 0x4447bb => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444811; continue; }
  pc = 0x4447c4; continue;
 }
 0x4447c4 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444811; continue; }
  pc = 0x4447ca; continue;
 }
 0x4447ca => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444811; continue; }
  pc = 0x4447d0; continue;
 }
 0x4447d0 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4447d7); cpu.call(t); if cpu.returned_to != 0x4447d7 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4447e2); f_0044664d(cpu); if cpu.returned_to != 0x4447e2 { return; }
  if !cpu.cf { pc = 0x444811; continue; }
  pc = 0x4447e4; continue;
 }
 0x4447e4 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x444811; continue; }
  pc = 0x44480c; continue;
 }
 0x44480c => {
  { let v: u8 = 0x1u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444811; continue;
 }
 0x444811 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444813(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444813); } r_00444813(cpu, 0x444813); }
pub fn f_00444845(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444845); } r_00444813(cpu, 0x444845); }
fn r_00444813(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x444813 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb66u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44489b; continue; }
  pc = 0x44483d; continue;
 }
 0x44483d => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44489b; continue; }
  pc = 0x444846; continue;
 }
 0x444845 => {
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x444846; continue;
 }
 0x444846 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44489b; continue; }
  pc = 0x44484c; continue;
 }
 0x44484c => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44489b; continue; }
  pc = 0x444852; continue;
 }
 0x444852 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444859); cpu.call(t); if cpu.returned_to != 0x444859 { return; }
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
  if (cpu.cf || cpu.zf) { pc = 0x444896; continue; }
  pc = 0x444882; continue;
 }
 0x444882 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x44489b; continue; }
  pc = 0x44488b; continue;
 }
 0x44488b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x44489b; continue; }
  pc = 0x444896; continue;
 }
 0x444896 => {
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44489b; continue;
 }
 0x44489b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444940(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444940); } r_00444940(cpu, 0x444940); }
pub fn f_00444967(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444967); } r_00444940(cpu, 0x444967); }
fn r_00444940(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x444940 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb6bu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444999; continue; }
  pc = 0x44496a; continue;
 }
 0x444967 => {
  { let v: u8 = cpu.r8(0x2fu32.wrapping_add(cpu.edi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  pc = 0x44496a; continue;
 }
 0x44496a => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444999; continue; }
  pc = 0x444973; continue;
 }
 0x444973 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444999; continue; }
  pc = 0x444979; continue;
 }
 0x444979 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444999; continue; }
  pc = 0x44497f; continue;
 }
 0x44497f => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444986); cpu.call(t); if cpu.returned_to != 0x444986 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x444999; continue; }
  pc = 0x444990; continue;
 }
 0x444990 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444999; continue;
 }
 0x444999 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004449f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4449f5); }
 let mut pc: u32 = 0x4449f5;
 loop { match pc {
 0x4449f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444a4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444a4e); }
 let mut pc: u32 = 0x444a4e;
 loop { match pc {
 0x444a4e => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a3f58u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x444a85); f_004435ee(cpu); if cpu.returned_to != 0x444a85 { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5a425au32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x444aae); f_0046771f(cpu); if cpu.returned_to != 0x444aae { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444aaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444aaf); }
 let mut pc: u32 = 0x444aaf;
 loop { match pc {
 0x444aaf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x444ab6); f_00445c78(cpu); if cpu.returned_to != 0x444ab6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x444ac9; continue; }
  pc = 0x444aba; continue;
 }
 0x444aba => {
  cpu.push(0x444abf); f_00446220(cpu); if cpu.returned_to != 0x444abf { return; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x444ac8); f_00446247(cpu); if cpu.returned_to != 0x444ac8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x444ac9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { let v: u8 = 0x2au8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x3u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444b0c); f_0041cbda(cpu); if cpu.returned_to != 0x444b0c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444b0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444b0e); }
 let mut pc: u32 = 0x444b0e;
 loop { match pc {
 0x444b0e => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x444b39; continue; }
  pc = 0x444b1d; continue;
 }
 0x444b1d => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e840cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e840du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e840eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e840fu32, v); }
  pc = 0x444b39; continue;
 }
 0x444b39 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x444b7f; continue; }
  pc = 0x444b44; continue;
 }
 0x444b44 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x444b7a; continue; }
  pc = 0x444b4e; continue;
 }
 0x444b4e => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x444b61; continue; }
  pc = 0x444b55; continue;
 }
 0x444b55 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444b7f; continue;
 }
 0x444b61 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x444b7f; continue; }
  pc = 0x444b68; continue;
 }
 0x444b68 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x444b7f; continue; }
  pc = 0x444b73; continue;
 }
 0x444b73 => {
  cpu.push(0x444b78); f_00416a21(cpu); if cpu.returned_to != 0x444b78 { return; }
  pc = 0x444b7f; continue;
 }
 0x444b7a => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444b7f; continue;
 }
 0x444b7f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444c28; continue; }
  pc = 0x444ba4; continue;
 }
 0x444ba4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444c28; continue; }
  pc = 0x444bad; continue;
 }
 0x444bad => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444c28; continue; }
  pc = 0x444bb3; continue;
 }
 0x444bb3 => {
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444c28; continue; }
  pc = 0x444bb9; continue;
 }
 0x444bb9 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x444c48; continue; }
  pc = 0x444bc6; continue;
 }
 0x444bc6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a45d8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a45dau32, v); }
  cpu.push(0x444bd8); f_00444f19(cpu); if cpu.returned_to != 0x444bd8 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x444c11; continue; }
  pc = 0x444be6; continue;
 }
 0x444be6 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x444bf3); cpu.call(t); if cpu.returned_to != 0x444bf3 { return; }
  { let v: u16 = cpu.r16(0x5a45d8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a45dau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b41u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444c11); cpu.call(t); if cpu.returned_to != 0x444c11 { return; }
  pc = 0x444c11; continue;
 }
 0x444c11 => {
  { cpu.popad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x444c19); f_00444ded(cpu); if cpu.returned_to != 0x444c19 { return; }
  cpu.push(0x444c1e); f_00444c7b(cpu); if cpu.returned_to != 0x444c1e { return; }
  cpu.push(0x444c23); f_00444cfd(cpu); if cpu.returned_to != 0x444c23 { return; }
  cpu.push(0x444c28); f_00444d58(cpu); if cpu.returned_to != 0x444c28 { return; }
  pc = 0x444c28; continue;
 }
 0x444c28 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x444c61; continue; }
  pc = 0x444c31; continue;
 }
 0x444c31 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x444c47); f_004677e0(cpu); if cpu.returned_to != 0x444c47 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x444c48 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a3f1cu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444c60); cpu.call(t); if cpu.returned_to != 0x444c60 { return; }
  { cpu.popad(); }
  pc = 0x444c61; continue;
 }
 0x444c61 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0x96u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x444c7a); f_004677e0(cpu); if cpu.returned_to != 0x444c7a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444c7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444c7b); }
 let mut pc: u32 = 0x444c7b;
 loop { match pc {
 0x444c7b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb56u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444cfb; continue; }
  pc = 0x444ca5; continue;
 }
 0x444ca5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444cfb; continue; }
  pc = 0x444cae; continue;
 }
 0x444cae => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444cfb; continue; }
  pc = 0x444cb4; continue;
 }
 0x444cb4 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444cfb; continue; }
  pc = 0x444cba; continue;
 }
 0x444cba => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444cc1); cpu.call(t); if cpu.returned_to != 0x444cc1 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x444ccc); f_0044664d(cpu); if cpu.returned_to != 0x444ccc { return; }
  if !cpu.cf { pc = 0x444cfb; continue; }
  pc = 0x444cce; continue;
 }
 0x444cce => {
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
  if (cpu.cf || cpu.zf) { pc = 0x444cfb; continue; }
  pc = 0x444cf6; continue;
 }
 0x444cf6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444cfb; continue;
 }
 0x444cfb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444cfd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444cfd); }
 let mut pc: u32 = 0x444cfd;
 loop { match pc {
 0x444cfd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb6du16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444d56; continue; }
  pc = 0x444d27; continue;
 }
 0x444d27 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x444d56; continue; }
  pc = 0x444d30; continue;
 }
 0x444d30 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444d56; continue; }
  pc = 0x444d36; continue;
 }
 0x444d36 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x444d56; continue; }
  pc = 0x444d3c; continue;
 }
 0x444d3c => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444d43); cpu.call(t); if cpu.returned_to != 0x444d43 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x444d56; continue; }
  pc = 0x444d4d; continue;
 }
 0x444d4d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x444d56; continue;
 }
 0x444d56 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444ded(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444ded); } r_00444ded(cpu, 0x444ded); }
pub fn f_00444e49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444e49); } r_00444ded(cpu, 0x444e49); }
fn r_00444ded(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x444ded => {
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
  { let v: u8 = cpu.r8(0x5a4609u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x444e3f; continue; }
  pc = 0x444e1f; continue;
 }
 0x444e1f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x444e3f; continue; }
  pc = 0x444e2f; continue;
 }
 0x444e2f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x444e48; continue; }
  pc = 0x444e3f; continue;
 }
 0x444e3f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x444e6d; continue;
 }
 0x444e48 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x32u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5dbu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xceu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  pc = 0x444e6d; continue;
 }
 0x444e49 => {
  { cpu.sub32(cpu.eax, 0x4d6ab4u32); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x32u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5dbu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xceu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  pc = 0x444e6d; continue;
 }
 0x444e6d => {
  cpu.push(0x444e72); f_0044664d(cpu); if cpu.returned_to != 0x444e72 { return; }
  if !cpu.cf { pc = 0x444f15; continue; }
  pc = 0x444e78; continue;
 }
 0x444e78 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x444f15; continue; }
  pc = 0x444ea0; continue;
 }
 0x444ea0 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x444f15; continue; }
  pc = 0x444ea9; continue;
 }
 0x444ea9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xau32.wrapping_add(cpu.ebp))); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x444f15; continue; }
  pc = 0x444eb9; continue;
 }
 0x444eb9 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x444f15; continue; }
  pc = 0x444ec8; continue;
 }
 0x444ec8 => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x444f15; continue; }
  pc = 0x444ed1; continue;
 }
 0x444ed1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x444ee2); f_004679cb(cpu); if cpu.returned_to != 0x444ee2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x444f0a; continue; }
  pc = 0x444efa; continue;
 }
 0x444efa => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x444f01); f_00401130(cpu); if cpu.returned_to != 0x444f01 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x444f0a; continue;
 }
 0x444f0a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x444f15); f_00415d61(cpu); if cpu.returned_to != 0x444f15 { return; }
  pc = 0x444f15; continue;
 }
 0x444f15 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444f19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444f19); }
 let mut pc: u32 = 0x444f19;
 loop { match pc {
 0x444f19 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5a3f1cu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x444f34); cpu.call(t); if cpu.returned_to != 0x444f34 { return; }
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
pub fn f_00444f3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444f3a); }
 let mut pc: u32 = 0x444f3a;
 loop { match pc {
 0x444f3a => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x444f51); f_0041c6a6(cpu); if cpu.returned_to != 0x444f51 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a3f64u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x444f75); f_004435ee(cpu); if cpu.returned_to != 0x444f75 { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5a40c0u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x444f9e); f_0046771f(cpu); if cpu.returned_to != 0x444f9e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444f9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x444f9f); }
 let mut pc: u32 = 0x444f9f;
 loop { match pc {
 0x444f9f => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x444fb6); f_0041c6a6(cpu); if cpu.returned_to != 0x444fb6 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444fc1); f_004435aa(cpu); if cpu.returned_to != 0x444fc1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x444fca); f_00445cce(cpu); if cpu.returned_to != 0x444fca { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00444fcd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x444fcd); } r_00444fcd(cpu, 0x444fcd); }
pub fn f_00445301(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x445301); } r_00444fcd(cpu, 0x445301); }
fn r_00444fcd(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x444fcd => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d6934u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a464cu32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4450d8; continue; }
  pc = 0x445000; continue;
 }
 0x445000 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4450d8; continue; }
  pc = 0x44500d; continue;
 }
 0x44500d => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4450d8; continue; }
  pc = 0x445017; continue;
 }
 0x445017 => {
  { cpu.sub16((cpu.ebx as u16), 0xff90u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4450d8; continue; }
  pc = 0x445021; continue;
 }
 0x445021 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4452ca; continue; }
  pc = 0x44502e; continue;
 }
 0x44502e => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a464cu32, v); }
  { let v: u32 = 0x5a3ef4u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a45d8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a45dau32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44505a); cpu.call(t); if cpu.returned_to != 0x44505a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x44509c; continue; }
  pc = 0x445069; continue;
 }
 0x445069 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x445076); cpu.call(t); if cpu.returned_to != 0x445076 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5a45d8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a45dau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b3du32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44509a); cpu.call(t); if cpu.returned_to != 0x44509a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44509c; continue;
 }
 0x44509c => {
  cpu.push(0x4450a1); f_00445989(cpu); if cpu.returned_to != 0x4450a1 { return; }
  cpu.push(0x4450a6); f_004433f3(cpu); if cpu.returned_to != 0x4450a6 { return; }
  cpu.push(0x4450ab); f_0044592f(cpu); if cpu.returned_to != 0x4450ab { return; }
  cpu.push(0x4450b0); f_004457d5(cpu); if cpu.returned_to != 0x4450b0 { return; }
  cpu.push(0x4450b5); f_00445722(cpu); if cpu.returned_to != 0x4450b5 { return; }
  cpu.push(0x4450ba); f_004458d0(cpu); if cpu.returned_to != 0x4450ba { return; }
  cpu.push(0x4450bf); f_00445866(cpu); if cpu.returned_to != 0x4450bf { return; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4450cf; continue; }
  pc = 0x4450c6; continue;
 }
 0x4450c6 => {
  cpu.push(0x4450cb); f_0041c6a6(cpu); if cpu.returned_to != 0x4450cb { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4450cf; continue;
 }
 0x4450cf => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4450d8); f_00445a09(cpu); if cpu.returned_to != 0x4450d8 { return; }
  pc = 0x4450d8; continue;
 }
 0x4450d8 => {
  { cpu.sub8(cpu.r8(0x5884d2u32), 0x0u8); }
  if !cpu.zf { pc = 0x4452b3; continue; }
  pc = 0x4450e5; continue;
 }
 0x4450e5 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x445323; continue; }
  pc = 0x4450f2; continue;
 }
 0x4450f2 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.cf { pc = 0x4452b3; continue; }
  pc = 0x445100; continue;
 }
 0x445100 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x445112; continue; }
  pc = 0x445109; continue;
 }
 0x445109 => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x44517b; continue;
 }
 0x445112 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44517b; continue; }
  pc = 0x445119; continue;
 }
 0x445119 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x445128; continue; }
  pc = 0x445122; continue;
 }
 0x445122 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x445131; continue; }
  pc = 0x445128; continue;
 }
 0x445128 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44514a; continue; }
  pc = 0x44512f; continue;
 }
 0x44512f => {
  pc = 0x445166; continue;
 }
 0x445131 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x445170; continue; }
  pc = 0x445138; continue;
 }
 0x445138 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  cpu.push(0x445144); f_0047a522(cpu); if cpu.returned_to != 0x445144 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445166; continue;
 }
 0x44514a => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x445170; continue; }
  pc = 0x445151; continue;
 }
 0x445151 => {
  { let v: u16 = cpu.r16(0x4d67f0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44515c); f_0044db1f(cpu); if cpu.returned_to != 0x44515c { return; }
  if cpu.cf { pc = 0x4452b3; continue; }
  pc = 0x445162; continue;
 }
 0x445162 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445166; continue;
 }
 0x445166 => {
  cpu.push(0x44516b); f_004456c8(cpu); if cpu.returned_to != 0x44516b { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445170; continue;
 }
 0x445170 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4452b3; continue; }
  pc = 0x44517b; continue;
 }
 0x44517b => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4451a7; continue; }
  pc = 0x445184; continue;
 }
 0x445184 => {
  { cpu.and16(cpu.r16(0x4d677eu32), 0x2u16); }
  if cpu.zf { pc = 0x4452b3; continue; }
  pc = 0x445193; continue;
 }
 0x445193 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x23u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4451a2); f_00446247(cpu); if cpu.returned_to != 0x4451a2 { return; }
  pc = 0x4452b3; continue;
 }
 0x4451a7 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5a464cu32), 0x0u8); }
  if cpu.zf { pc = 0x4451bc; continue; }
  pc = 0x4451b5; continue;
 }
 0x4451b5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59daddu32, v); }
  pc = 0x4451bc; continue;
 }
 0x4451bc => {
  { let v: u16 = 0x23u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4451c9); f_00444aaf(cpu); if cpu.returned_to != 0x4451c9 { return; }
  cpu.push(0x4451ce); f_00445ca3(cpu); if cpu.returned_to != 0x4451ce { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4452b3; continue; }
  pc = 0x4451db; continue;
 }
 0x4451db => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x4451f5; continue; }
  pc = 0x4451f0; continue;
 }
 0x4451f0 => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x4451f5; continue;
 }
 0x4451f5 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x445204); f_0044e7cd(cpu); if cpu.returned_to != 0x445204 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  if cpu.cf { pc = 0x4452b3; continue; }
  pc = 0x445210; continue;
 }
 0x445210 => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (cpu.sf == cpu.of) { pc = 0x4452b3; continue; }
  pc = 0x44521f; continue;
 }
 0x44521f => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44522c); f_004456c8(cpu); if cpu.returned_to != 0x44522c { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  cpu.push(0x445245); f_00445a95(cpu); if cpu.returned_to != 0x445245 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5d3860u32), 0x0u8); }
  if !cpu.zf { pc = 0x445337; continue; }
  pc = 0x445253; continue;
 }
 0x445253 => {
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x445279; continue; }
  pc = 0x445264; continue;
 }
 0x445264 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x445279; continue; }
  pc = 0x44526e; continue;
 }
 0x44526e => {
  cpu.push(0x445273); f_0047a522(cpu); if cpu.returned_to != 0x445273 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4452b3; continue;
 }
 0x445279 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x445282); f_0044db1f(cpu); if cpu.returned_to != 0x445282 { return; }
  if cpu.cf { pc = 0x44529f; continue; }
  pc = 0x445284; continue;
 }
 0x445284 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44529f; continue; }
  pc = 0x44528c; continue;
 }
 0x44528c => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x445297); f_0044e79b(cpu); if cpu.returned_to != 0x445297 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  if !cpu.cf { pc = 0x4452b3; continue; }
  pc = 0x44529f; continue;
 }
 0x44529f => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x4452b3; continue;
 }
 0x4452b3 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4452c9); f_004677e0(cpu); if cpu.returned_to != 0x4452c9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4452ca => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a3ef4u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4452e2); cpu.call(t); if cpu.returned_to != 0x4452e2 { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x4452f6; continue; }
  pc = 0x4452f2; continue;
 }
 0x4452f2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x445323; continue;
 }
 0x4452f6 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x445303); cpu.call(t); if cpu.returned_to != 0x445303 { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x445323; continue; }
  pc = 0x44530a; continue;
 }
 0x445301 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x585b04fbu32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x585b04fbu32.wrapping_add(cpu.ebx), v); }
  if (cpu.sf == cpu.of) { pc = 0x445323; continue; }
  pc = 0x44530a; continue;
 }
 0x44530a => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b3du32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445322); cpu.call(t); if cpu.returned_to != 0x445322 { return; }
  { cpu.popad(); }
  pc = 0x445323; continue;
 }
 0x445323 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x445330); f_004677e0(cpu); if cpu.returned_to != 0x445330 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445337 => {
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x524b70u32).wrapping_add(cpu.eax); cpu.w32(0x524b70u32, v); }
  cpu.push(0x445346); f_0047a522(cpu); if cpu.returned_to != 0x445346 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x4452b3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445356(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445356); }
 let mut pc: u32 = 0x445356;
 loop { match pc {
 0x445356 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d6934u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5a48acu32, v); }
  { cpu.sub8(cpu.r8(0x5884d2u32), 0x0u8); }
  if !cpu.zf { pc = 0x445638; continue; }
  pc = 0x44537e; continue;
 }
 0x44537e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44564a; continue; }
  pc = 0x44538b; continue;
 }
 0x44538b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  { let v: u32 = 0x2u32; cpu.w32(0x5a48acu32, v); }
  if !cpu.cf { pc = 0x445638; continue; }
  pc = 0x4453a3; continue;
 }
 0x4453a3 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4453b3); f_00456b3a(cpu); if cpu.returned_to != 0x4453b3 { return; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.ecx)), 0x0u8); }
  { cpu.popad(); }
  { let v: u32 = 0x3u32; cpu.w32(0x5a48acu32, v); }
  if cpu.cf { pc = 0x445638; continue; }
  pc = 0x4453cb; continue;
 }
 0x4453cb => {
  if cpu.zf { pc = 0x445638; continue; }
  pc = 0x4453d1; continue;
 }
 0x4453d1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4453d8); f_00445cce(cpu); if cpu.returned_to != 0x4453d8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4453ec; continue; }
  pc = 0x4453e3; continue;
 }
 0x4453e3 => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x445462; continue;
 }
 0x4453ec => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x445462; continue; }
  pc = 0x4453f3; continue;
 }
 0x4453f3 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x445402; continue; }
  pc = 0x4453fc; continue;
 }
 0x4453fc => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x44540b; continue; }
  pc = 0x445402; continue;
 }
 0x445402 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44541d; continue; }
  pc = 0x445409; continue;
 }
 0x445409 => {
  pc = 0x445443; continue;
 }
 0x44540b => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44544d; continue; }
  pc = 0x445412; continue;
 }
 0x445412 => {
  cpu.push(0x445417); f_0047a58e(cpu); if cpu.returned_to != 0x445417 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445443; continue;
 }
 0x44541d => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44544d; continue; }
  pc = 0x445424; continue;
 }
 0x445424 => {
  { let v: u16 = cpu.r16(0x4d67f0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44542f); f_0044db1f(cpu); if cpu.returned_to != 0x44542f { return; }
  { let v: u32 = 0x4u32; cpu.w32(0x5a48acu32, v); }
  if cpu.cf { pc = 0x445638; continue; }
  pc = 0x44543f; continue;
 }
 0x44543f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445443; continue;
 }
 0x445443 => {
  cpu.push(0x445448); f_004456c8(cpu); if cpu.returned_to != 0x445448 { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44544d; continue;
 }
 0x44544d => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  { let v: u32 = 0x5u32; cpu.w32(0x5a48acu32, v); }
  if cpu.zf { pc = 0x445638; continue; }
  pc = 0x445462; continue;
 }
 0x445462 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4454b9; continue; }
  pc = 0x44546b; continue;
 }
 0x44546b => {
  { cpu.and16(cpu.r16(0x4d677eu32), 0x2u16); }
  { let v: u32 = 0x6u32; cpu.w32(0x5a48acu32, v); }
  if cpu.zf { pc = 0x445638; continue; }
  pc = 0x445484; continue;
 }
 0x445484 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x4au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  { let v: u32 = 0x7u32; cpu.w32(0x5a48acu32, v); }
  if !cpu.zf { pc = 0x445638; continue; }
  pc = 0x4454a5; continue;
 }
 0x4454a5 => {
  cpu.push(0x4454aa); f_00446247(cpu); if cpu.returned_to != 0x4454aa { return; }
  { let v: u32 = 0x8u32; cpu.w32(0x5a48acu32, v); }
  pc = 0x445638; continue;
 }
 0x4454b9 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x4au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4454d4; continue; }
  pc = 0x4454cf; continue;
 }
 0x4454cf => {
  cpu.push(0x4454d4); f_00444aaf(cpu); if cpu.returned_to != 0x4454d4 { return; }
  pc = 0x4454d4; continue;
 }
 0x4454d4 => {
  cpu.push(0x4454d9); f_00445ca3(cpu); if cpu.returned_to != 0x4454d9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x9u32; cpu.w32(0x5a48acu32, v); }
  if cpu.zf { pc = 0x445638; continue; }
  pc = 0x4454f0; continue;
 }
 0x4454f0 => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x44550a; continue; }
  pc = 0x445505; continue;
 }
 0x445505 => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x44550a; continue;
 }
 0x44550a => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x445519); f_0044e7cd(cpu); if cpu.returned_to != 0x445519 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0xau32; cpu.w32(0x5a48acu32, v); }
  if cpu.cf { pc = 0x445638; continue; }
  pc = 0x44552f; continue;
 }
 0x44552f => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0x5a48acu32, v); }
  if (cpu.sf == cpu.of) { pc = 0x445638; continue; }
  pc = 0x445548; continue;
 }
 0x445548 => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x445555); f_004456c8(cpu); if cpu.returned_to != 0x445555 { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x445590; continue; }
  pc = 0x44556e; continue;
 }
 0x44556e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if cpu.zf { pc = 0x445590; continue; }
  pc = 0x445573; continue;
 }
 0x445573 => {
  { let v: u16 = 0x48u16; cpu.w16(0x5a45e0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x5a45e2u32, v); }
  { let v: u16 = 0xffd0u16; cpu.w16(0x5a45e4u32, v); }
  pc = 0x4455ab; continue;
 }
 0x445590 => {
  { let v: u16 = 0x47u16; cpu.w16(0x5a45e0u32, v); }
  { let v: u16 = 0x68u16; cpu.w16(0x5a45e2u32, v); }
  { let v: u16 = 0x70u16; cpu.w16(0x5a45e4u32, v); }
  pc = 0x4455ab; continue;
 }
 0x4455ab => {
  cpu.push(0x4455b0); f_00445a95(cpu); if cpu.returned_to != 0x4455b0 { return; }
  { let v: u16 = 0x20u16; cpu.w16(0x5a45e0u32, v); }
  { let v: u16 = 0x32u16; cpu.w16(0x5a45e2u32, v); }
  { let v: u16 = 0x30u16; cpu.w16(0x5a45e4u32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4455f4; continue; }
  pc = 0x4455d5; continue;
 }
 0x4455d5 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x4455f4; continue; }
  pc = 0x4455df; continue;
 }
 0x4455df => {
  cpu.push(0x4455e4); f_0047a58e(cpu); if cpu.returned_to != 0x4455e4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0x5a48acu32, v); }
  pc = 0x445638; continue;
 }
 0x4455f4 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4455fd); f_0044db1f(cpu); if cpu.returned_to != 0x4455fd { return; }
  if cpu.cf { pc = 0x445624; continue; }
  pc = 0x4455ff; continue;
 }
 0x4455ff => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x445624; continue; }
  pc = 0x445607; continue;
 }
 0x445607 => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x445612); f_0044e79b(cpu); if cpu.returned_to != 0x445612 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0xdu32; cpu.w32(0x5a48acu32, v); }
  if !cpu.cf { pc = 0x445638; continue; }
  pc = 0x445624; continue;
 }
 0x445624 => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445638; continue;
 }
 0x445638 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44564a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445651(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445651); }
 let mut pc: u32 = 0x445651;
 loop { match pc {
 0x445651 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x445661; continue; }
  pc = 0x44565a; continue;
 }
 0x44565a => {
  { let v: u32 = 0x11111111u32; cpu.eax = v; }
  pc = 0x44567c; continue;
 }
 0x445661 => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44567b); f_00485b77(cpu); if cpu.returned_to != 0x44567b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x44567c; continue;
 }
 0x44567c => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if !cpu.zf { pc = 0x44569e; continue; }
  pc = 0x445699; continue;
 }
 0x445699 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x44569e; continue;
 }
 0x44569e => {
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004456ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4456ab); }
 let mut pc: u32 = 0x4456ab;
 loop { match pc {
 0x4456ab => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x370u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004456c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4456c8); }
 let mut pc: u32 = 0x4456c8;
 loop { match pc {
 0x4456c8 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x4456d8; continue; }
  pc = 0x4456d1; continue;
 }
 0x4456d1 => {
  { let v: u32 = 0x11111111u32; cpu.eax = v; }
  pc = 0x4456f3; continue;
 }
 0x4456d8 => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4456f2); f_00485b77(cpu); if cpu.returned_to != 0x4456f2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4456f3; continue;
 }
 0x4456f3 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if !cpu.zf { pc = 0x445715; continue; }
  pc = 0x445710; continue;
 }
 0x445710 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x445715; continue;
 }
 0x445715 => {
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445722); }
 let mut pc: u32 = 0x445722;
 loop { match pc {
 0x445722 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x44572f; continue; }
  pc = 0x44572a; continue;
 }
 0x44572a => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44572f; continue;
 }
 0x44572f => {
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x44574f; continue;
 }
 0x44574f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb62u32); cpu.ecx = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445781; continue; }
  pc = 0x445764; continue;
 }
 0x445764 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445781; continue; }
  pc = 0x44576d; continue;
 }
 0x44576d => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445781; continue; }
  pc = 0x445773; continue;
 }
 0x445773 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445781; continue; }
  pc = 0x445779; continue;
 }
 0x445779 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445780); cpu.call(t); if cpu.returned_to != 0x445780 { return; }
  { cpu.popad(); }
  pc = 0x445781; continue;
 }
 0x445781 => {
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44574f; continue; }
  pc = 0x44578b; continue;
 }
 0x44578b => {
  cpu.push(0x445790); f_0044664d(cpu); if cpu.returned_to != 0x445790 { return; }
  if !cpu.cf { pc = 0x4457d3; continue; }
  pc = 0x445792; continue;
 }
 0x445792 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x4457d3; continue; }
  pc = 0x4457ba; continue;
 }
 0x4457ba => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x4457d3; continue; }
  pc = 0x4457c3; continue;
 }
 0x4457c3 => {
  { let v: u8 = cpu.shl8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4457d3; continue; }
  pc = 0x4457ce; continue;
 }
 0x4457ce => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4457d3; continue;
 }
 0x4457d3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004457d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4457d5); }
 let mut pc: u32 = 0x4457d5;
 loop { match pc {
 0x4457d5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xdu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb43u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445864; continue; }
  pc = 0x4457ff; continue;
 }
 0x4457ff => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445864; continue; }
  pc = 0x445808; continue;
 }
 0x445808 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445864; continue; }
  pc = 0x44580e; continue;
 }
 0x44580e => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445864; continue; }
  pc = 0x445814; continue;
 }
 0x445814 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44581b); cpu.call(t); if cpu.returned_to != 0x44581b { return; }
  { cpu.popad(); }
  cpu.push(0x445821); f_0044664d(cpu); if cpu.returned_to != 0x445821 { return; }
  if !cpu.cf { pc = 0x445864; continue; }
  pc = 0x445823; continue;
 }
 0x445823 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x445864; continue; }
  pc = 0x44584b; continue;
 }
 0x44584b => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x445864; continue; }
  pc = 0x445854; continue;
 }
 0x445854 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11u32.wrapping_add(cpu.ebp))); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if !cpu.zf { pc = 0x445864; continue; }
  pc = 0x44585f; continue;
 }
 0x44585f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445864; continue;
 }
 0x445864 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445866(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445866); }
 let mut pc: u32 = 0x445866;
 loop { match pc {
 0x445866 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x5a45d8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5a45dau32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x26u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x27u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb5cu32); cpu.ecx = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4458ce; continue; }
  pc = 0x445891; continue;
 }
 0x445891 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4458ce; continue; }
  pc = 0x44589a; continue;
 }
 0x44589a => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4458ce; continue; }
  pc = 0x4458a0; continue;
 }
 0x4458a0 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4458ce; continue; }
  pc = 0x4458a6; continue;
 }
 0x4458a6 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4458ad); cpu.call(t); if cpu.returned_to != 0x4458ad { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4458ce; continue; }
  pc = 0x4458b5; continue;
 }
 0x4458b5 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x4458ce; continue; }
  pc = 0x4458be; continue;
 }
 0x4458be => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x4458ce; continue; }
  pc = 0x4458c9; continue;
 }
 0x4458c9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4458ce; continue;
 }
 0x4458ce => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004458d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4458d0); }
 let mut pc: u32 = 0x4458d0;
 loop { match pc {
 0x4458d0 => {
  cpu.push(0x4458d5); f_0044664d(cpu); if cpu.returned_to != 0x4458d5 { return; }
  if !cpu.cf { pc = 0x44592e; continue; }
  pc = 0x4458d7; continue;
 }
 0x4458d7 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x44592e; continue; }
  pc = 0x4458ff; continue;
 }
 0x4458ff => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x855u32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44592c); cpu.call(t); if cpu.returned_to != 0x44592c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44592e; continue;
 }
 0x44592e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044592f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44592f); }
 let mut pc: u32 = 0x44592f;
 loop { match pc {
 0x44592f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xfffeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x836u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445961); cpu.call(t); if cpu.returned_to != 0x445961 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x445988; continue; }
  pc = 0x44596a; continue;
 }
 0x44596a => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x445988; continue; }
  pc = 0x445971; continue;
 }
 0x445971 => {
  cpu.push(0x445976); f_0044e720(cpu); if cpu.returned_to != 0x445976 { return; }
  if cpu.cf { pc = 0x445988; continue; }
  pc = 0x445978; continue;
 }
 0x445978 => {
  { cpu.pushad(); }
  cpu.push(0x44597e); f_004459cd(cpu); if cpu.returned_to != 0x44597e { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x14u32.wrapping_add(cpu.ebp))); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x14u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445988; continue;
 }
 0x445988 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445989); }
 let mut pc: u32 = 0x445989;
 loop { match pc {
 0x445989 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x846u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4459ca); cpu.call(t); if cpu.returned_to != 0x4459ca { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004459cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4459cd); }
 let mut pc: u32 = 0x4459cd;
 loop { match pc {
 0x4459cd => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x445a08; continue; }
  pc = 0x4459d6; continue;
 }
 0x4459d6 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x445a04; continue; }
  pc = 0x4459dd; continue;
 }
 0x4459dd => {
  cpu.push(0x4459e2); f_0041c6a6(cpu); if cpu.returned_to != 0x4459e2 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x23u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xfff6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x445a02); f_004862bf(cpu); if cpu.returned_to != 0x445a02 { return; }
  pc = 0x445a08; continue;
 }
 0x445a04 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x445a08; continue;
 }
 0x445a08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445a09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445a09); }
 let mut pc: u32 = 0x445a09;
 loop { match pc {
 0x445a09 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  cpu.push(0x445a14); f_0044664d(cpu); if cpu.returned_to != 0x445a14 { return; }
  if !cpu.cf { pc = 0x445a94; continue; }
  pc = 0x445a16; continue;
 }
 0x445a16 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x445a94; continue; }
  pc = 0x445a3e; continue;
 }
 0x445a3e => {
  { cpu.pushad(); }
  { let v: u16 = 0x1au16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x44u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x445a62; continue;
 }
 0x445a62 => {
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x856u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.adc16((cpu.ecx as u16), 0x0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.ebx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.edx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445a85); cpu.call(t); if cpu.returned_to != 0x445a85 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x3u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x445a62; continue; }
  pc = 0x445a93; continue;
 }
 0x445a93 => {
  { cpu.popad(); }
  pc = 0x445a94; continue;
 }
 0x445a94 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445a95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445a95); }
 let mut pc: u32 = 0x445a95;
 loop { match pc {
 0x445a95 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x445c72; continue; }
  pc = 0x445aa2; continue;
 }
 0x445aa2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x52u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4650u32); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5a45f6u32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x445b04; continue; }
  pc = 0x445af0; continue;
 }
 0x445af0 => {
  cpu.push(0x445af5); f_004465a4(cpu); if cpu.returned_to != 0x445af5 { return; }
  if !cpu.cf { pc = 0x445c72; continue; }
  pc = 0x445afb; continue;
 }
 0x445afb => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d3860u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445b04 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x445c72; continue; }
  pc = 0x445b1f; continue;
 }
 0x445b1f => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5a45e0u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d69e2u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5a45e2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d69e4u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x78u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e1e46u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fcau32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x445b6a); f_00449b96(cpu); if cpu.returned_to != 0x445b6a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcau32, v); }
  if cpu.cf { pc = 0x445c72; continue; }
  pc = 0x445b78; continue;
 }
 0x445b78 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x56c735u32, v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x445b99; continue; }
  pc = 0x445b92; continue;
 }
 0x445b92 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c735u32, v); }
  pc = 0x445b99; continue;
 }
 0x445b99 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x445baa; continue;
 }
 0x445baa => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x445c66; continue; }
  pc = 0x445bb8; continue;
 }
 0x445bb8 => {
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x56c735u32), 0x0u8); }
  if !cpu.zf { pc = 0x445bda; continue; }
  pc = 0x445bc9; continue;
 }
 0x445bc9 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.eax = v; }
  cpu.push(0x445bd3); f_00442698(cpu); if cpu.returned_to != 0x445bd3 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x56c735u32, v); }
  pc = 0x445bda; continue;
 }
 0x445bda => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_sub(0x54u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d69e2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d69e4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5a45e4u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x54u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x445c4e); f_0041b7b0(cpu); if cpu.returned_to != 0x445c4e { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x445c54); f_00413337(cpu); if cpu.returned_to != 0x445c54 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x445c66; continue;
 }
 0x445c66 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x445baa; continue; }
  pc = 0x445c6f; continue;
 }
 0x445c6f => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445c72 => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445c78); }
 let mut pc: u32 = 0x445c78;
 loop { match pc {
 0x445c78 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.mul16(v); }
  { let v = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.div16(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a464du32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u8 = 0x64u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445ca3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445ca3); }
 let mut pc: u32 = 0x445ca3;
 loop { match pc {
 0x445ca3 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.mul16(v); }
  { let v = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.div16(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a464du32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u8 = 0x64u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445cce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445cce); }
 let mut pc: u32 = 0x445cce;
 loop { match pc {
 0x445cce => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x445cfb; continue; }
  pc = 0x445cda; continue;
 }
 0x445cda => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x445d12; continue; }
  pc = 0x445cdf; continue;
 }
 0x445cdf => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x445d29; continue; }
  pc = 0x445ce4; continue;
 }
 0x445ce4 => {
  { let v: u16 = cpu.r16(0x4d67a2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67a4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445cfb => {
  { let v: u16 = cpu.r16(0x4d67a6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67a8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445d12 => {
  { let v: u16 = cpu.r16(0x4d67aau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67acu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x445d29 => {
  { let v: u16 = cpu.r16(0x4d67aeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445d40); }
 let mut pc: u32 = 0x445d40;
 loop { match pc {
 0x445d40 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x445d47); f_00445d71(cpu); if cpu.returned_to != 0x445d47 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a464du32, v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x445d53); f_00445d71(cpu); if cpu.returned_to != 0x445d53 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a464eu32, v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x445d5f); f_00445d71(cpu); if cpu.returned_to != 0x445d5f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a464fu32, v); }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x445d6b); f_00445d71(cpu); if cpu.returned_to != 0x445d6b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a4650u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445d71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445d71); }
 let mut pc: u32 = 0x445d71;
 loop { match pc {
 0x445d71 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x445d7c; continue;
 }
 0x445d7c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x445d8c; continue; }
  pc = 0x445d83; continue;
 }
 0x445d83 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x445d8c; continue; }
  pc = 0x445d8a; continue;
 }
 0x445d8a => {
  { let v: u8 = cpu.inc8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x445d8c; continue;
 }
 0x445d8c => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x445d7c; continue; }
  pc = 0x445d93; continue;
 }
 0x445d93 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = ((cpu.ebx >> 8) as u8); cpu.mul8(v); }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c87fu32), 0x0u8); }
  if cpu.zf { pc = 0x445de7; continue; }
  pc = 0x445dab; continue;
 }
 0x445dab => {
  { cpu.sub16(cpu.r16(0x4d680cu32), 0x0u16); }
  if cpu.zf { pc = 0x445de7; continue; }
  pc = 0x445db5; continue;
 }
 0x445db5 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = cpu.r16(0x4d680cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = 0x2710u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d680cu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x64u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445de7; continue; }
  pc = 0x445de2; continue;
 }
 0x445de2 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  pc = 0x445de7; continue;
 }
 0x445de7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c880u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445df9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445df9); }
 let mut pc: u32 = 0x445df9;
 loop { match pc {
 0x445df9 => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a3f78u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x445e26); f_004435ee(cpu); if cpu.returned_to != 0x445e26 { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5a447au32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x445e4f); f_0046771f(cpu); if cpu.returned_to != 0x445e4f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00445e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x445e50); }
 let mut pc: u32 = 0x445e50;
 loop { match pc {
 0x445e50 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a464cu32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445f99; continue; }
  pc = 0x445e7c; continue;
 }
 0x445e7c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x445f99; continue; }
  pc = 0x445e89; continue;
 }
 0x445e89 => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445f99; continue; }
  pc = 0x445e93; continue;
 }
 0x445e93 => {
  { cpu.sub16((cpu.ebx as u16), 0xff90u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x445f99; continue; }
  pc = 0x445e9d; continue;
 }
 0x445e9d => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44619a; continue; }
  pc = 0x445eaa; continue;
 }
 0x445eaa => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a464cu32, v); }
  { let v: u32 = 0x5a3f08u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a45d8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a45dau32, v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445ed5); cpu.call(t); if cpu.returned_to != 0x445ed5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x445f13; continue; }
  pc = 0x445ee4; continue;
 }
 0x445ee4 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x445ef1); cpu.call(t); if cpu.returned_to != 0x445ef1 { return; }
  { let v: u16 = cpu.r16(0x5a45d8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a45dau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b3fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x445f13); cpu.call(t); if cpu.returned_to != 0x445f13 { return; }
  pc = 0x445f13; continue;
 }
 0x445f13 => {
  { cpu.popad(); }
  cpu.push(0x445f19); f_004433f3(cpu); if cpu.returned_to != 0x445f19 { return; }
  cpu.push(0x445f1e); f_004462c2(cpu); if cpu.returned_to != 0x445f1e { return; }
  cpu.push(0x445f23); f_00446306(cpu); if cpu.returned_to != 0x445f23 { return; }
  cpu.push(0x445f28); f_004463b3(cpu); if cpu.returned_to != 0x445f28 { return; }
  cpu.push(0x445f2d); f_0044664d(cpu); if cpu.returned_to != 0x445f2d { return; }
  if !cpu.cf { pc = 0x445f99; continue; }
  pc = 0x445f2f; continue;
 }
 0x445f2f => {
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
  if (cpu.cf || cpu.zf) { pc = 0x445f5f; continue; }
  pc = 0x445f57; continue;
 }
 0x445f57 => {
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x445f5f; continue;
 }
 0x445f5f => {
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
  if (cpu.cf || cpu.zf) { pc = 0x445f99; continue; }
  pc = 0x445f87; continue;
 }
 0x445f87 => {
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x445f90); f_00446380(cpu); if cpu.returned_to != 0x445f90 { return; }
  { let v: u16 = 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x445f99); f_00446380(cpu); if cpu.returned_to != 0x445f99 { return; }
  pc = 0x445f99; continue;
 }
 0x445f99 => {
  { cpu.sub8(cpu.r8(0x5884d2u32), 0x0u8); }
  if !cpu.zf { pc = 0x446183; continue; }
  pc = 0x445fa6; continue;
 }
 0x445fa6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4461f3; continue; }
  pc = 0x445fb3; continue;
 }
 0x445fb3 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.cf { pc = 0x446183; continue; }
  pc = 0x445fc1; continue;
 }
 0x445fc1 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x445fd3; continue; }
  pc = 0x445fca; continue;
 }
 0x445fca => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x44603c; continue;
 }
 0x445fd3 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44603c; continue; }
  pc = 0x445fda; continue;
 }
 0x445fda => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x445fe9; continue; }
  pc = 0x445fe3; continue;
 }
 0x445fe3 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x445ff2; continue; }
  pc = 0x445fe9; continue;
 }
 0x445fe9 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44600b; continue; }
  pc = 0x445ff0; continue;
 }
 0x445ff0 => {
  pc = 0x446027; continue;
 }
 0x445ff2 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x446031; continue; }
  pc = 0x445ff9; continue;
 }
 0x445ff9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  cpu.push(0x446005); f_0047a4c0(cpu); if cpu.returned_to != 0x446005 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x446027; continue;
 }
 0x44600b => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x446031; continue; }
  pc = 0x446012; continue;
 }
 0x446012 => {
  { let v: u16 = cpu.r16(0x4d67f2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44601d); f_0044db1f(cpu); if cpu.returned_to != 0x44601d { return; }
  if cpu.cf { pc = 0x446183; continue; }
  pc = 0x446023; continue;
 }
 0x446023 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x446027; continue;
 }
 0x446027 => {
  cpu.push(0x44602c); f_00445651(cpu); if cpu.returned_to != 0x44602c { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x446031; continue;
 }
 0x446031 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x446183; continue; }
  pc = 0x44603c; continue;
 }
 0x44603c => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x446068; continue; }
  pc = 0x446045; continue;
 }
 0x446045 => {
  { cpu.and16(cpu.r16(0x4d677eu32), 0x2u16); }
  if cpu.zf { pc = 0x446183; continue; }
  pc = 0x446054; continue;
 }
 0x446054 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x20u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x39u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x446063); f_00446247(cpu); if cpu.returned_to != 0x446063 { return; }
  pc = 0x446183; continue;
 }
 0x446068 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5a464cu32), 0x0u8); }
  if cpu.zf { pc = 0x44607d; continue; }
  pc = 0x446076; continue;
 }
 0x446076 => {
  { let v: u8 = 0x2u8; cpu.w8(0x59daddu32, v); }
  pc = 0x44607d; continue;
 }
 0x44607d => {
  { let v: u16 = 0x20u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x39u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44608a); f_00444aaf(cpu); if cpu.returned_to != 0x44608a { return; }
  cpu.push(0x44608f); f_0044e720(cpu); if cpu.returned_to != 0x44608f { return; }
  if cpu.cf { pc = 0x446096; continue; }
  pc = 0x446091; continue;
 }
 0x446091 => {
  cpu.push(0x446096); f_0044628f(cpu); if cpu.returned_to != 0x446096 { return; }
  pc = 0x446096; continue;
 }
 0x446096 => {
  cpu.push(0x44609b); f_00445ca3(cpu); if cpu.returned_to != 0x44609b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x446183; continue; }
  pc = 0x4460a8; continue;
 }
 0x4460a8 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x4460ba; continue; }
  pc = 0x4460b1; continue;
 }
 0x4460b1 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4460ba; continue;
 }
 0x4460ba => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x4460d4; continue; }
  pc = 0x4460cf; continue;
 }
 0x4460cf => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x4460d4; continue;
 }
 0x4460d4 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x4460e3); f_0044e7cd(cpu); if cpu.returned_to != 0x4460e3 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  if cpu.cf { pc = 0x446183; continue; }
  pc = 0x4460ef; continue;
 }
 0x4460ef => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (cpu.sf == cpu.of) { pc = 0x446183; continue; }
  pc = 0x4460fe; continue;
 }
 0x4460fe => {
  cpu.push(0x446103); f_00445651(cpu); if cpu.returned_to != 0x446103 { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  cpu.push(0x44611c); f_00446423(cpu); if cpu.returned_to != 0x44611c { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5d3860u32), 0x0u8); }
  if !cpu.zf { pc = 0x446201; continue; }
  pc = 0x44612a; continue;
 }
 0x44612a => {
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x446150; continue; }
  pc = 0x44613b; continue;
 }
 0x44613b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x446150; continue; }
  pc = 0x446145; continue;
 }
 0x446145 => {
  cpu.push(0x44614a); f_0047a4c0(cpu); if cpu.returned_to != 0x44614a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x446183; continue;
 }
 0x446150 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44616f; continue; }
  pc = 0x44615c; continue;
 }
 0x44615c => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  cpu.push(0x446167); f_0044e79b(cpu); if cpu.returned_to != 0x446167 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  if !cpu.cf { pc = 0x446183; continue; }
  pc = 0x44616f; continue;
 }
 0x44616f => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x446183; continue;
 }
 0x446183 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x446199); f_004677e0(cpu); if cpu.returned_to != 0x446199 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44619a => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a3f08u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4461b2); cpu.call(t); if cpu.returned_to != 0x4461b2 { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x4461c6; continue; }
  pc = 0x4461c2; continue;
 }
 0x4461c2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4461f3; continue;
 }
 0x4461c6 => {
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x4461d3); cpu.call(t); if cpu.returned_to != 0x4461d3 { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x4461f3; continue; }
  pc = 0x4461da; continue;
 }
 0x4461da => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a2b3fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4461f2); cpu.call(t); if cpu.returned_to != 0x4461f2 { return; }
  { cpu.popad(); }
  pc = 0x4461f3; continue;
 }
 0x4461f3 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x446200); f_004677e0(cpu); if cpu.returned_to != 0x446200 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446201 => {
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x524b70u32).wrapping_add(cpu.eax); cpu.w32(0x524b70u32, v); }
  cpu.push(0x446210); f_0047a4c0(cpu); if cpu.returned_to != 0x446210 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x446183; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446220); }
 let mut pc: u32 = 0x446220;
 loop { match pc {
 0x446220 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x446229); f_00445c78(cpu); if cpu.returned_to != 0x446229 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x446240; continue; }
  pc = 0x44622f; continue;
 }
 0x44622f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x446240; continue;
 }
 0x446240 => {
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
pub fn f_00446247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446247); }
 let mut pc: u32 = 0x446247;
 loop { match pc {
 0x446247 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x3cu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  cpu.push(0x44628d); f_0044e0dc(cpu); if cpu.returned_to != 0x44628d { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044628f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44628f); }
 let mut pc: u32 = 0x44628f;
 loop { match pc {
 0x44628f => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4462bd; continue; }
  pc = 0x446296; continue;
 }
 0x446296 => {
  cpu.push(0x44629b); f_0041c6a6(cpu); if cpu.returned_to != 0x44629b { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xfff3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4462bb); f_004862bf(cpu); if cpu.returned_to != 0x4462bb { return; }
  pc = 0x4462c1; continue;
 }
 0x4462bd => {
  { let v: u8 = cpu.dec8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4462c1; continue;
 }
 0x4462c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004462c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4462c2); }
 let mut pc: u32 = 0x4462c2;
 loop { match pc {
 0x4462c2 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x852u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x446303); cpu.call(t); if cpu.returned_to != 0x446303 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446306(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446306); }
 let mut pc: u32 = 0x446306;
 loop { match pc {
 0x446306 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x848u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x446339); cpu.call(t); if cpu.returned_to != 0x446339 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x446340); f_0044664d(cpu); if cpu.returned_to != 0x446340 { return; }
  if !cpu.cf { pc = 0x44634b; continue; }
  pc = 0x446342; continue;
 }
 0x446342 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11u32.wrapping_add(cpu.ebp))); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44634b; continue;
 }
 0x44634b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446380); }
 let mut pc: u32 = 0x446380;
 loop { match pc {
 0x446380 => {
  { cpu.pushad(); }
  { let v: u16 = 0x84eu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4463b1); cpu.call(t); if cpu.returned_to != 0x4463b1 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004463b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4463b3); }
 let mut pc: u32 = 0x4463b3;
 loop { match pc {
 0x4463b3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x17u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x27u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.sf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x84cu32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4463ec); cpu.call(t); if cpu.returned_to != 0x4463ec { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xffffu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x83eu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x446420); cpu.call(t); if cpu.returned_to != 0x446420 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446423(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446423); }
 let mut pc: u32 = 0x446423;
 loop { match pc {
 0x446423 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4465a3; continue; }
  pc = 0x446430; continue;
 }
 0x446430 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d3860u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x52u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4650u32); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5a45f6u32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x446492; continue; }
  pc = 0x44647e; continue;
 }
 0x44647e => {
  cpu.push(0x446483); f_004465a4(cpu); if cpu.returned_to != 0x446483 { return; }
  if !cpu.cf { pc = 0x4465a3; continue; }
  pc = 0x446489; continue;
 }
 0x446489 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d3860u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446492 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x4465a3; continue; }
  pc = 0x4464a7; continue;
 }
 0x4464a7 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x17u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d69e2u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d69e4u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x78u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e1e46u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fcau32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4464ec); f_00449b96(cpu); if cpu.returned_to != 0x4464ec { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcau32, v); }
  if cpu.cf { pc = 0x4465a3; continue; }
  pc = 0x4464fa; continue;
 }
 0x4464fa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x44652b; continue; }
  pc = 0x44650f; continue;
 }
 0x44650f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a1cu32); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x44652a); f_00442698(cpu); if cpu.returned_to != 0x44652a { return; }
  { cpu.popad(); }
  pc = 0x44652b; continue;
 }
 0x44652b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a1cu32); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d69e2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d69e4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x44658b); f_0041b7b0(cpu); if cpu.returned_to != 0x44658b { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x446591); f_00413337(cpu); if cpu.returned_to != 0x446591 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  pc = 0x4465a3; continue;
 }
 0x4465a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004465a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4465a4); }
 let mut pc: u32 = 0x4465a4;
 loop { match pc {
 0x4465a4 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4465d6; continue; }
  pc = 0x4465ad; continue;
 }
 0x4465ad => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4465d6; continue; }
  pc = 0x4465b7; continue;
 }
 0x4465b7 => {
  { let a = cpu.r8(0x4e1fccu32); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x4e1fccu32, v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x5b3598u32); cpu.push(v); }
  cpu.push(0x4465c9); f_0047a5fa(cpu); if cpu.returned_to != 0x4465c9 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x5b3598u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4465d6 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x44662d; continue; }
  pc = 0x4465e8; continue;
 }
 0x4465e8 => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x88u32); }
  if (cpu.sf == cpu.of) { pc = 0x44662d; continue; }
  pc = 0x4465f4; continue;
 }
 0x4465f4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x56c6f6u32, v); }
  { let v: u32 = 0xddu32; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = 0x5b3887u32; cpu.w32(0x5b3590u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3594u32, v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x44662d; continue; }
  pc = 0x446621; continue;
 }
 0x446621 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x44662c); f_00416a62(cpu); if cpu.returned_to != 0x44662c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44662d; continue;
 }
 0x44662d => {
  cpu.push(0x446632); f_00446634(cpu); if cpu.returned_to != 0x446632 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446634); }
 let mut pc: u32 = 0x446634;
 loop { match pc {
 0x446634 => {
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
pub fn f_0044664d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44664d); }
 let mut pc: u32 = 0x44664d;
 loop { match pc {
 0x44664d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44665b; continue; }
  pc = 0x446658; continue;
 }
 0x446658 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44665b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044665e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44665e); }
 let mut pc: u32 = 0x44665e;
 loop { match pc {
 0x44665e => {
  { let v: u32 = 0x5a4794u32; cpu.esi = v; }
  { let v: u32 = 0x5a4820u32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5a3edcu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a3ed8u32, v); }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x446679; continue;
 }
 0x446679 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x446679; continue; }
  pc = 0x44668d; continue;
 }
 0x44668d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044668e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44668e); }
 let mut pc: u32 = 0x44668e;
 loop { match pc {
 0x44668e => {
  { let v: u32 = 0x0u32; cpu.w32(0x4e1580u32, v); }
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4466a2; continue;
 }
 0x4466a2 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4466cc; continue; }
  pc = 0x4466a9; continue;
 }
 0x4466a9 => {
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x4466cc; continue; }
  pc = 0x4466b6; continue;
 }
 0x4466b6 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x4466cc; continue; }
  pc = 0x4466c0; continue;
 }
 0x4466c0 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4466cc; continue; }
  pc = 0x4466c6; continue;
 }
 0x4466c6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1580u32.wrapping_add(cpu.eax))); cpu.w8(0x4e1580u32.wrapping_add(cpu.eax), v); }
  pc = 0x4466cc; continue;
 }
 0x4466cc => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4466a2; continue; }
  pc = 0x4466d1; continue;
 }
 0x4466d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004466d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4466d2); }
 let mut pc: u32 = 0x4466d2;
 loop { match pc {
 0x4466d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004466d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4466d3); }
 let mut pc: u32 = 0x4466d3;
 loop { match pc {
 0x4466d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004466d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4466d4); }
 let mut pc: u32 = 0x4466d4;
 loop { match pc {
 0x4466d4 => {
  { cpu.pushad(); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4466df; continue;
 }
 0x4466df => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x446733; continue; }
  pc = 0x4466e7; continue;
 }
 0x4466e7 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x446733; continue; }
  pc = 0x4466ee; continue;
 }
 0x4466ee => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x446733; continue; }
  pc = 0x4466f5; continue;
 }
 0x4466f5 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x446733; continue; }
  pc = 0x4466fc; continue;
 }
 0x4466fc => {
  { let v: u32 = cpu.r32(0x26u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x22u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x446726; continue; }
  pc = 0x446717; continue;
 }
 0x446717 => {
  cpu.push(0x44671c); f_004456c8(cpu); if cpu.returned_to != 0x44671c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x446733; continue;
 }
 0x446726 => {
  cpu.push(0x44672b); f_00445651(cpu); if cpu.returned_to != 0x44672b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x446733; continue;
 }
 0x446733 => {
  { cpu.popad(); }
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4466df; continue; }
  pc = 0x44673a; continue;
 }
 0x44673a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044673c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44673c); }
 let mut pc: u32 = 0x44673c;
 loop { match pc {
 0x44673c => {
  { cpu.pushad(); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x446747; continue;
 }
 0x446747 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x446766; continue; }
  pc = 0x44674e; continue;
 }
 0x44674e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x446766; continue; }
  pc = 0x446757; continue;
 }
 0x446757 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x44676f; continue; }
  pc = 0x446766; continue;
 }
 0x446766 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x446747; continue; }
  pc = 0x44676c; continue;
 }
 0x44676c => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44676f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044679d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44679d); }
 let mut pc: u32 = 0x44679d;
 loop { match pc {
 0x44679d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4467a7; continue; }
  pc = 0x4467a6; continue;
 }
 0x4467a6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4467a7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x586697u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x586698u32, v); }
  cpu.push(0x4467ba); f_00446895(cpu); if cpu.returned_to != 0x4467ba { return; }
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x582424u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x582428u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58242cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x582430u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x582434u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x586697u32, v); }
  { let v: u32 = cpu.r32(0x582438u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582420u32, v); }
  { let v: u32 = cpu.r32(0x58243cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582424u32, v); }
  { let v: u32 = cpu.r32(0x582440u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582428u32, v); }
  { let v: u32 = cpu.r32(0x582444u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58242cu32, v); }
  { let v: u32 = cpu.r32(0x582448u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582430u32, v); }
  { let v: u32 = cpu.r32(0x58244cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582434u32, v); }
  cpu.push(0x446826); f_00446895(cpu); if cpu.returned_to != 0x446826 { return; }
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582438u32, v); }
  { let v: u32 = cpu.r32(0x582424u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58243cu32, v); }
  { let v: u32 = cpu.r32(0x582428u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582440u32, v); }
  { let v: u32 = cpu.r32(0x58242cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582444u32, v); }
  { let v: u32 = cpu.r32(0x582430u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582448u32, v); }
  { let v: u32 = cpu.r32(0x582434u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58244cu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582434u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582430u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58242cu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582428u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582424u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x582420u32, v); }
  { cpu.sub8(cpu.r8(0x586698u32), 0x0u8); }
  if cpu.zf { pc = 0x446894; continue; }
  pc = 0x44688f; continue;
 }
 0x44688f => {
  cpu.push(0x446894); f_00446d1d(cpu); if cpu.returned_to != 0x446894 { return; }
  pc = 0x446894; continue;
 }
 0x446894 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446895(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446895); }
 let mut pc: u32 = 0x446895;
 loop { match pc {
 0x446895 => {
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x582424u32)); cpu.eax = v; }
  if !cpu.zf { pc = 0x4468a3; continue; }
  pc = 0x4468a2; continue;
 }
 0x4468a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4468a3 => {
  { let v: u32 = cpu.r32(0x58242cu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x582434u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x582428u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4469d3; continue; }
  pc = 0x4468d4; continue;
 }
 0x4468d4 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x582424u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x4468f8); f_00486542(cpu); if cpu.returned_to != 0x4468f8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x4468ff); f_00418beb(cpu); if cpu.returned_to != 0x4468ff { return; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x446972; continue; }
  pc = 0x44690b; continue;
 }
 0x44690b => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x446972; continue; }
  pc = 0x446911; continue;
 }
 0x446911 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xcu16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x446972; continue; }
  pc = 0x44691d; continue;
 }
 0x44691d => {
  { cpu.sub16((cpu.edx as u16), 0xcu16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x446972; continue; }
  pc = 0x446923; continue;
 }
 0x446923 => {
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x582424u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x586697u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x18au32); cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x446956); f_00486542(cpu); if cpu.returned_to != 0x446956 { return; }
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x58242cu32), 0x3u32); }
  if cpu.zf { pc = 0x446972; continue; }
  pc = 0x446960; continue;
 }
 0x446960 => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x3u8); }
  if cpu.zf { pc = 0x446972; continue; }
  pc = 0x446969; continue;
 }
 0x446969 => {
  { let v: u8 = 0x1u8; cpu.w8(0x586698u32, v); }
  pc = 0x4469d2; continue;
 }
 0x446972 => {
  { cpu.sub32(cpu.r32(0x58242cu32), 0x3u32); }
  if cpu.zf { pc = 0x4469a0; continue; }
  pc = 0x44697b; continue;
 }
 0x44697b => {
  { cpu.sub32(cpu.r32(0x58242cu32), 0x0u32); }
  if cpu.zf { pc = 0x4469a0; continue; }
  pc = 0x446984; continue;
 }
 0x446984 => {
  { let v: u32 = cpu.r32(0x582420u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x582424u32); cpu.ebx = v; }
  cpu.push(0x446994); f_00446a55(cpu); if cpu.returned_to != 0x446994 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4469a0; continue; }
  pc = 0x446999; continue;
 }
 0x446999 => {
  { cpu.sub32(cpu.eax, 0x8d00u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x446969; continue; }
  pc = 0x4469a0; continue;
 }
 0x4469a0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x582430u32)); cpu.w32(0x582430u32, v); }
  if !cpu.zf { pc = 0x4469d2; continue; }
  pc = 0x4469aa; continue;
 }
 0x4469aa => {
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x58242cu32), 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x4469c6; continue; }
  pc = 0x4469b9; continue;
 }
 0x4469b9 => {
  cpu.push(0x4469be); f_0041c743(cpu); if cpu.returned_to != 0x4469be { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x4469c6; continue;
 }
 0x4469c6 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x582430u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x582428u32)); cpu.w32(0x582428u32, v); }
  pc = 0x4469d2; continue;
 }
 0x4469d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4469d3 => {
  { cpu.sub32(cpu.r32(0x58242cu32), 0x3u32); }
  if cpu.zf { pc = 0x446a3c; continue; }
  pc = 0x4469dc; continue;
 }
 0x4469dc => {
  { cpu.sub32(cpu.r32(0x58242cu32), 0x0u32); }
  if cpu.zf { pc = 0x4469f5; continue; }
  pc = 0x4469e5; continue;
 }
 0x4469e5 => {
  { cpu.sub32(cpu.r32(0x58242cu32), 0x1u32); }
  if !cpu.zf { pc = 0x4469fc; continue; }
  pc = 0x4469ee; continue;
 }
 0x4469ee => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x446a0b; continue;
 }
 0x4469f5 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x446a0b; continue;
 }
 0x4469fc => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x446a0b); f_0041c808(cpu); if cpu.returned_to != 0x446a0b { return; }
  pc = 0x446a0b; continue;
 }
 0x446a0b => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58242cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582428u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x582434u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x582430u32, v); }
  cpu.poll(); pc = 0x4468a3; continue;
 }
 0x446a3c => {
  { let v: u32 = 0x0u32; cpu.w32(0x582420u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582424u32, v); }
  cpu.poll(); pc = 0x4469d2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446a55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446a55); }
 let mut pc: u32 = 0x446a55;
 loop { match pc {
 0x446a55 => {
  { let v: u32 = cpu.eax; cpu.w32(0x59da9bu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x59da9fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x588499u32, v); }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x586699u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x446a76; continue;
 }
 0x446a76 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x446adf; continue; }
  pc = 0x446a7d; continue;
 }
 0x446a7d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x446adf; continue; }
  pc = 0x446a83; continue;
 }
 0x446a83 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x446aa3; continue; }
  pc = 0x446a89; continue;
 }
 0x446a89 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x446aa3; continue; }
  pc = 0x446a8f; continue;
 }
 0x446a8f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x446aa3; continue; }
  pc = 0x446a95; continue;
 }
 0x446a95 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x446aa3; continue; }
  pc = 0x446a9b; continue;
 }
 0x446a9b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x446aa3; continue; }
  pc = 0x446aa1; continue;
 }
 0x446aa1 => {
  pc = 0x446adf; continue;
 }
 0x446aa3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59da9bu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59da9fu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x588499u32)); cpu.w8(0x588499u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x446adf; continue;
 }
 0x446adf => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x446a76; continue; }
  pc = 0x446ae8; continue;
 }
 0x446ae8 => {
  { cpu.sub8(cpu.r8(0x588499u32), 0x1u8); }
  if cpu.zf { pc = 0x446b34; continue; }
  pc = 0x446af1; continue;
 }
 0x446af1 => {
  { cpu.sub8(cpu.r8(0x588499u32), 0x0u8); }
  if !cpu.zf { pc = 0x446b01; continue; }
  pc = 0x446afa; continue;
 }
 0x446afa => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x446b3c; continue;
 }
 0x446b01 => {
  { let v: u8 = cpu.r8(0x588499u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x446b07; continue;
 }
 0x446b07 => {
  { let v: u32 = 0x586699u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x588499u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x446b14; continue;
 }
 0x446b14 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x446b29; continue; }
  pc = 0x446b1c; continue;
 }
 0x446b1c => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x446b29; continue;
 }
 0x446b29 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x446b14; continue; }
  pc = 0x446b30; continue;
 }
 0x446b30 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x446b07; continue; }
  pc = 0x446b34; continue;
 }
 0x446b34 => {
  { let v: u32 = 0x586699u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x446b3c; continue;
 }
 0x446b3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446b3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446b3d); }
 let mut pc: u32 = 0x446b3d;
 loop { match pc {
 0x446b3d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x446b47; continue; }
  pc = 0x446b46; continue;
 }
 0x446b46 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446b47 => {
  { cpu.sub32(cpu.r32(0x58849eu32), 0x0u32); }
  if cpu.zf { pc = 0x446b57; continue; }
  pc = 0x446b50; continue;
 }
 0x446b50 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x58849eu32)); cpu.w32(0x58849eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x446b99; continue; }
  pc = 0x446b62; continue;
 }
 0x446b62 => {
  { let v: u8 = cpu.r8(0x575f19u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x50u8); }
  if (cpu.sf != cpu.of) { pc = 0x446b99; continue; }
  pc = 0x446b6b; continue;
 }
 0x446b6b => {
  { cpu.sub32(cpu.r32(0x576e76u32), 0x0u32); }
  if !cpu.zf { pc = 0x446b99; continue; }
  pc = 0x446b74; continue;
 }
 0x446b74 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x9c4u32; cpu.ecx = v; }
  cpu.push(0x446b83); f_0041c808(cpu); if cpu.returned_to != 0x446b83 { return; }
  { cpu.sub16((cpu.eax as u16), 0x2au16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x446b99; continue; }
  pc = 0x446b89; continue;
 }
 0x446b89 => {
  cpu.push(0x446b8e); f_00446b9a(cpu); if cpu.returned_to != 0x446b8e { return; }
  { let v: u32 = 0x2710u32; cpu.w32(0x58849eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446b99 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446b9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446b9a); }
 let mut pc: u32 = 0x446b9a;
 loop { match pc {
 0x446b9a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x446ba4; continue; }
  pc = 0x446ba3; continue;
 }
 0x446ba3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446ba4 => {
  { cpu.sub32(cpu.r32(0x582420u32), 0x0u32); }
  if !cpu.zf { pc = 0x446cd8; continue; }
  pc = 0x446bb1; continue;
 }
 0x446bb1 => {
  { cpu.sub32(cpu.r32(0x582424u32), 0x0u32); }
  if !cpu.zf { pc = 0x446cd8; continue; }
  pc = 0x446bbe; continue;
 }
 0x446bbe => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x446bd2); f_0047a2d9(cpu); if cpu.returned_to != 0x446bd2 { return; }
  if cpu.cf { pc = 0x446cd8; continue; }
  pc = 0x446bd8; continue;
 }
 0x446bd8 => {
  cpu.push(0x446bdd); f_0041c743(cpu); if cpu.returned_to != 0x446bdd { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x446c00; continue; }
  pc = 0x446be5; continue;
 }
 0x446be5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x446c42; continue;
 }
 0x446c00 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x446c0c); f_0041c808(cpu); if cpu.returned_to != 0x446c0c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x446c1a); f_0041c808(cpu); if cpu.returned_to != 0x446c1a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x446c35); f_00420989(cpu); if cpu.returned_to != 0x446c35 { return; }
  cpu.push(0x446c3a); f_00446cd9(cpu); if cpu.returned_to != 0x446c3a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x446cd8; continue; }
  pc = 0x446c42; continue;
 }
 0x446c42 => {
  { let v: u32 = cpu.eax; cpu.w32(0x582420u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x582424u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582428u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58242cu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x582430u32, v); }
  { let v: u32 = 0x58247cu32; cpu.w32(0x582434u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x582438u32, v); }
  { let v: u32 = cpu.r32(0x582438u32).wrapping_add(0x10u32); cpu.w32(0x582438u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58243cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582440u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x582444u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x582448u32, v); }
  { let v: u32 = 0x582450u32; cpu.w32(0x58244cu32, v); }
  { cpu.sub8(cpu.r8(0x5adbecu32), 0x2u8); }
  if !cpu.zf { pc = 0x446cd8; continue; }
  pc = 0x446cb8; continue;
 }
 0x446cb8 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x446cbe); f_0041c743(cpu); if cpu.returned_to != 0x446cbe { return; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x446cd7; continue; }
  pc = 0x446cc4; continue;
 }
 0x446cc4 => {
  cpu.push(0x446cc9); f_0041c743(cpu); if cpu.returned_to != 0x446cc9 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x446cd7; continue; }
  pc = 0x446ccf; continue;
 }
 0x446ccf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x446cd5); f_0047b41e(cpu); if cpu.returned_to != 0x446cd5 { return; }
  pc = 0x446cd8; continue;
 }
 0x446cd7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x446cd8; continue;
 }
 0x446cd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446cd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446cd9); }
 let mut pc: u32 = 0x446cd9;
 loop { match pc {
 0x446cd9 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x446ce9; continue; }
  pc = 0x446ce5; continue;
 }
 0x446ce5 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x446d1b; continue; }
  pc = 0x446ce9; continue;
 }
 0x446ce9 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x446cf9; continue; }
  pc = 0x446cf5; continue;
 }
 0x446cf5 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x446d1b; continue; }
  pc = 0x446cf9; continue;
 }
 0x446cf9 => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x446d09; continue; }
  pc = 0x446d05; continue;
 }
 0x446d05 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x446d1b; continue; }
  pc = 0x446d09; continue;
 }
 0x446d09 => {
  { let v: u8 = cpu.r8(0x4081u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x446d19; continue; }
  pc = 0x446d15; continue;
 }
 0x446d15 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x446d1b; continue; }
  pc = 0x446d19; continue;
 }
 0x446d19 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446d1b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446d1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446d1d); }
 let mut pc: u32 = 0x446d1d;
 loop { match pc {
 0x446d1d => {
  { let v: u32 = 0x0u32; cpu.w32(0x582428u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58242cu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x582430u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582440u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582444u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x582448u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446d5a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x446d5a); } r_00446d5a(cpu, 0x446d5a); }
pub fn f_00446d63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x446d63); } r_00446d5a(cpu, 0x446d63); }
fn r_00446d5a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x446d5a => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a48f0u32, v); }
  pc = 0x446d6c; continue;
 }
 0x446d63 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a48f0u32, v); }
  pc = 0x446d6c; continue;
 }
 0x446d6c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x446d76; continue; }
  pc = 0x446d75; continue;
 }
 0x446d75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446d76 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if cpu.zf { pc = 0x446d80; continue; }
  pc = 0x446d7f; continue;
 }
 0x446d7f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446d80 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x576be4u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446da3; continue; }
  pc = 0x446d94; continue;
 }
 0x446d94 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x576be4u32, v); }
  pc = 0x446da3; continue;
 }
 0x446da3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x576486u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446dc3; continue; }
  pc = 0x446dbc; continue;
 }
 0x446dbc => {
  { let v: u32 = cpu.r32(0x5764b2u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  pc = 0x446dc3; continue;
 }
 0x446dc3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5764f5u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446de3; continue; }
  pc = 0x446ddd; continue;
 }
 0x446ddd => {
  { let v: u8 = cpu.r8(0x5764fau32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x446de3; continue;
 }
 0x446de3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x576bd0u32, v); }
  pc = 0x446def; continue;
 }
 0x446def => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbdu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xc7u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446e20; continue; }
  pc = 0x446e08; continue;
 }
 0x446e08 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x17au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x18eu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x576be4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x576be4u32)); cpu.edx = v; }
  pc = 0x446e20; continue;
 }
 0x446e20 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6be4u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.push(0x446e43); f_004714a5(cpu); if cpu.returned_to != 0x446e43 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x446e55; continue; }
  pc = 0x446e48; continue;
 }
 0x446e48 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x576bd0u32)); cpu.w8(0x576bd0u32, v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x446def; continue; }
  pc = 0x446e54; continue;
 }
 0x446e54 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446e55 => {
  { let v: u8 = cpu.r8(0x576bd0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x446e73; continue; }
  pc = 0x446e63; continue;
 }
 0x446e63 => {
  { cpu.sub8(cpu.r8(0x5a48f0u32), 0x0u8); }
  if cpu.zf { pc = 0x446e73; continue; }
  pc = 0x446e6c; continue;
 }
 0x446e6c => {
  cpu.push(0x446e71); f_00447023(cpu); if cpu.returned_to != 0x446e71 { return; }
  cpu.poll(); pc = 0x446e54; continue;
 }
 0x446e73 => {
  { cpu.sub8(cpu.r8(0x5a48f0u32), 0x0u8); }
  if cpu.zf { pc = 0x446e82; continue; }
  pc = 0x446e7c; continue;
 }
 0x446e7c => {
  cpu.push(0x446e81); f_0044160f(cpu); if cpu.returned_to != 0x446e81 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446e82 => {
  cpu.push(0x446e87); f_00440df2(cpu); if cpu.returned_to != 0x446e87 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446e88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446e88); }
 let mut pc: u32 = 0x446e88;
 loop { match pc {
 0x446e88 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x446e92; continue; }
  pc = 0x446e91; continue;
 }
 0x446e91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446e92 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if cpu.zf { pc = 0x446e9c; continue; }
  pc = 0x446e9b; continue;
 }
 0x446e9b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x446e9c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x576be4u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446ebf; continue; }
  pc = 0x446eb0; continue;
 }
 0x446eb0 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x576be4u32, v); }
  pc = 0x446ebf; continue;
 }
 0x446ebf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { let v: u16 = 0xc6u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x576486u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446efe; continue; }
  pc = 0x446ee7; continue;
 }
 0x446ee7 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.r32(0x5764b2u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  pc = 0x446efe; continue;
 }
 0x446efe => {
  { let v: u32 = 0x575f47u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5764ebu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446f23; continue; }
  pc = 0x446f1d; continue;
 }
 0x446f1d => {
  { let v: u8 = cpu.r8(0x5764f0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x446f23; continue;
 }
 0x446f23 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be4u32, v); }
  pc = 0x446f28; continue;
 }
 0x446f28 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xbu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xbdu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x446f52; continue; }
  pc = 0x446f3d; continue;
 }
 0x446f3d => {
  { let v: u8 = 0x1u8; cpu.w8(0x581b67u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0xbu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x576be4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1u32); cpu.ebx = v; }
  pc = 0x446f52; continue;
 }
 0x446f52 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x13c3u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  cpu.push(0x446f65); f_004200a5(cpu); if cpu.returned_to != 0x446f65 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x581b67u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x446f28; continue; }
  pc = 0x446f76; continue;
 }
 0x446f76 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446f7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446f7d); }
 let mut pc: u32 = 0x446f7d;
 loop { match pc {
 0x446f7d => {
  { cpu.pushad(); }
  cpu.push(0x446f83); f_004470cc(cpu); if cpu.returned_to != 0x446f83 { return; }
  if !cpu.cf { pc = 0x446f92; continue; }
  pc = 0x446f85; continue;
 }
 0x446f85 => {
  { cpu.pushad(); }
  cpu.push(0x446f8b); f_00446fee(cpu); if cpu.returned_to != 0x446f8b { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  pc = 0x446f92; continue;
 }
 0x446f92 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(0xcu32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xaau32; cpu.w32(0x576bc7u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x575fa6u32); cpu.eax = v; }
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x576bcbu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x446fd4; continue;
 }
 0x446fd4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x446fec; continue; }
  pc = 0x446fdf; continue;
 }
 0x446fdf => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3bu32); }
  if !cpu.zf { cpu.poll(); pc = 0x446fd4; continue; }
  pc = 0x446fe5; continue;
 }
 0x446fe5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x446fec; continue;
 }
 0x446fec => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00446fee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x446fee); }
 let mut pc: u32 = 0x446fee;
 loop { match pc {
 0x446fee => {
  { cpu.pushad(); }
  { let v: u32 = 0x575fa6u32; cpu.edi = v; }
  { let v: u32 = 0x575fefu32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x446ffe; continue;
 }
 0x446ffe => {
  { let v: u32 = 0x49u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_sub(0x10u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x49u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.movs(1, true); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x446ffe; continue; }
  pc = 0x447016; continue;
 }
 0x447016 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447023); }
 let mut pc: u32 = 0x447023;
 loop { match pc {
 0x447023 => {
  { cpu.sub32(cpu.r32(0x576bc7u32), 0x0u32); }
  if cpu.zf { pc = 0x44706e; continue; }
  pc = 0x44702c; continue;
 }
 0x44702c => {
  cpu.push(0x447031); f_004470ef(cpu); if cpu.returned_to != 0x447031 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x447078; continue; }
  pc = 0x447036; continue;
 }
 0x447036 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x44706e; continue; }
  pc = 0x44703b; continue;
 }
 0x44703b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x576bcbu32)); cpu.w32(0x576bcbu32, v); }
  { let v: u32 = cpu.r32(0x576bcbu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.sf { pc = 0x447059; continue; }
  pc = 0x44704e; continue;
 }
 0x44704e => {
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x576bcbu32, v); }
  pc = 0x447059; continue;
 }
 0x447059 => {
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x575fa6u32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x44703b; continue; }
  pc = 0x44706e; continue;
 }
 0x44706e => {
  { let v: u32 = 0xaau32; cpu.w32(0x576bc7u32, v); }
  pc = 0x447078; continue;
 }
 0x447078 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447079(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447079); }
 let mut pc: u32 = 0x447079;
 loop { match pc {
 0x447079 => {
  { cpu.sub32(cpu.r32(0x576bc7u32), 0x0u32); }
  if cpu.zf { pc = 0x4470c1; continue; }
  pc = 0x447082; continue;
 }
 0x447082 => {
  cpu.push(0x447087); f_004470ef(cpu); if cpu.returned_to != 0x447087 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x4470cb; continue; }
  pc = 0x44708c; continue;
 }
 0x44708c => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x4470c1; continue; }
  pc = 0x447091; continue;
 }
 0x447091 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576bcbu32)); cpu.w32(0x576bcbu32, v); }
  { let v: u32 = cpu.r32(0x576bcbu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x6u32); }
  if !cpu.zf { pc = 0x4470ac; continue; }
  pc = 0x4470a4; continue;
 }
 0x4470a4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x576bcbu32, v); }
  pc = 0x4470ac; continue;
 }
 0x4470ac => {
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x575fa6u32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x447091; continue; }
  pc = 0x4470c1; continue;
 }
 0x4470c1 => {
  { let v: u32 = 0xaau32; cpu.w32(0x576bc7u32, v); }
  pc = 0x4470cb; continue;
 }
 0x4470cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004470cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4470cc); }
 let mut pc: u32 = 0x4470cc;
 loop { match pc {
 0x4470cc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x575fa6u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x4470d9; continue;
 }
 0x4470d9 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x4470ec; continue; }
  pc = 0x4470e0; continue;
 }
 0x4470e0 => {
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4470d9; continue; }
  pc = 0x4470e9; continue;
 }
 0x4470e9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4470ec => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004470ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4470ef); }
 let mut pc: u32 = 0x4470ef;
 loop { match pc {
 0x4470ef => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x575fa6u32; cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x4470fb; continue;
 }
 0x4470fb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x447103; continue; }
  pc = 0x447102; continue;
 }
 0x447102 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x447103; continue;
 }
 0x447103 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4470fb; continue; }
  pc = 0x447109; continue;
 }
 0x447109 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044710a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44710a); }
 let mut pc: u32 = 0x44710a;
 loop { match pc {
 0x44710a => {
  { let v: u32 = 0x575fa6u32; cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x447114; continue;
 }
 0x447114 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x447114; continue; }
  pc = 0x44711f; continue;
 }
 0x44711f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447120); }
 let mut pc: u32 = 0x447120;
 loop { match pc {
 0x447120 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x447139; continue; }
  pc = 0x447129; continue;
 }
 0x447129 => {
  { cpu.sub8(cpu.r8(0x581c01u32), 0xffu8); }
  if cpu.zf { pc = 0x447139; continue; }
  pc = 0x447132; continue;
 }
 0x447132 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1313u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44713f; continue;
 }
 0x447139 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x12e3u32); cpu.ecx = v; }
  pc = 0x44713f; continue;
 }
 0x44713f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x447147); cpu.call(t); if cpu.returned_to != 0x447147 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x447185; continue; }
  pc = 0x447153; continue;
 }
 0x447153 => {
  { let v: u32 = cpu.eax.wrapping_add(0x16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581c01u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x447185; continue; }
  pc = 0x447166; continue;
 }
 0x447166 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x44716e); cpu.call(t); if cpu.returned_to != 0x44716e { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x575f47u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x136eu32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x447185); f_0041cf66(cpu); if cpu.returned_to != 0x447185 { return; }
  pc = 0x447185; continue;
 }
 0x447185 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447187(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447187); }
 let mut pc: u32 = 0x447187;
 loop { match pc {
 0x447187 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x447191; continue; }
  pc = 0x447190; continue;
 }
 0x447190 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447191 => {
  { cpu.sub32(cpu.r32(0x576bc7u32), 0x0u32); }
  if !cpu.zf { pc = 0x44719b; continue; }
  pc = 0x44719a; continue;
 }
 0x44719a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44719b => {
  cpu.push(0x4471a0); f_004470ef(cpu); if cpu.returned_to != 0x4471a0 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x4471a6; continue; }
  pc = 0x4471a5; continue;
 }
 0x4471a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4471a6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x57615cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x576160u32, v); }
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4471e0; continue; }
  pc = 0x4471d6; continue;
 }
 0x4471d6 => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x4471e0; continue;
 }
 0x4471e0 => {
  { let v: u32 = 0xa4u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x57615cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x576160u32); cpu.edi = v; }
  cpu.push(0x4471fa); f_004714a5(cpu); if cpu.returned_to != 0x4471fa { return; }
  if !cpu.cf { pc = 0x447225; continue; }
  pc = 0x4471fc; continue;
 }
 0x4471fc => {
  { let v: u32 = 0x0u32; cpu.w32(0x576bc7u32, v); }
  { let v: u32 = cpu.r32(0x576bcbu32); cpu.ebx = v; }
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x575fa6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x447222); f_0044160f(cpu); if cpu.returned_to != 0x447222 { return; }
  { cpu.cf = true; }
  pc = 0x447274; continue;
 }
 0x447225 => {
  { let v: u32 = 0xe4u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xf4u32; cpu.ecx = v; }
  { let v: u32 = 0x4eu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x447256; continue; }
  pc = 0x447242; continue;
 }
 0x447242 => {
  { let v: u32 = 0x164u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x174u32; cpu.ecx = v; }
  { let v: u32 = 0x8eu32; cpu.edx = v; }
  pc = 0x447256; continue;
 }
 0x447256 => {
  { let v: u32 = cpu.r32(0x57615cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x576160u32); cpu.edi = v; }
  cpu.push(0x447267); f_004714a5(cpu); if cpu.returned_to != 0x447267 { return; }
  if !cpu.cf { pc = 0x447274; continue; }
  pc = 0x447269; continue;
 }
 0x447269 => {
  { let v: u32 = 0x0u32; cpu.w32(0x576bc7u32, v); }
  { cpu.cf = true; }
  pc = 0x447274; continue;
 }
 0x447274 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447276(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447276); }
 let mut pc: u32 = 0x447276;
 loop { match pc {
 0x447276 => {
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x447299; continue; }
  pc = 0x44728e; continue;
 }
 0x44728e => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba8u32, v); }
  pc = 0x447299; continue;
 }
 0x447299 => {
  { let v: u32 = 0x575fa6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x16du32); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x38u32; cpu.eax = v; }
  { let v: u32 = 0xbau32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4472c6; continue; }
  pc = 0x4472b9; continue;
 }
 0x4472b9 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x576be4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  pc = 0x4472c6; continue;
 }
 0x4472c6 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x447306; continue; }
  pc = 0x4472d8; continue;
 }
 0x4472d8 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4472e8; continue; }
  pc = 0x4472e1; continue;
 }
 0x4472e1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581b67u32, v); }
  pc = 0x4472e8; continue;
 }
 0x4472e8 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if cpu.zf { pc = 0x4472ff; continue; }
  pc = 0x4472f1; continue;
 }
 0x4472f1 => {
  if !cpu.sf { pc = 0x4472fa; continue; }
  pc = 0x4472f3; continue;
 }
 0x4472f3 => {
  cpu.push(0x4472f8); f_004412ad(cpu); if cpu.returned_to != 0x4472f8 { return; }
  if cpu.cf { pc = 0x4472ff; continue; }
  pc = 0x4472fa; continue;
 }
 0x4472fa => {
  cpu.push(0x4472ff); f_00447366(cpu); if cpu.returned_to != 0x4472ff { return; }
  pc = 0x4472ff; continue;
 }
 0x4472ff => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b67u32, v); }
  pc = 0x447306; continue;
 }
 0x447306 => {
  { cpu.sub32(cpu.r32(0x576bc7u32), 0x0u32); }
  if cpu.zf { pc = 0x44735f; continue; }
  pc = 0x44730f; continue;
 }
 0x44730f => {
  { let v: u32 = cpu.dec32(cpu.r32(0x576bc7u32)); cpu.w32(0x576bc7u32, v); }
  { let v: u32 = cpu.r32(0x576bcbu32); cpu.ebx = v; }
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x575fa6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x447346; continue; }
  pc = 0x44733c; continue;
 }
 0x44733c => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x447346; continue;
 }
 0x447346 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x44735f; continue; }
  pc = 0x44735a; continue;
 }
 0x44735a => {
  cpu.push(0x44735f); f_00447366(cpu); if cpu.returned_to != 0x44735f { return; }
  pc = 0x44735f; continue;
 }
 0x44735f => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447366(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x447366); } r_00447366(cpu, 0x447366); }
pub fn f_004474c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4474c7); } r_00447366(cpu, 0x4474c7); }
fn r_00447366(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x447366 => {
  { let v: u16 = 0x0u16; cpu.w16(0x593005u32, v); }
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x4473a9; continue; }
  pc = 0x44737a; continue;
 }
 0x44737a => {
  { cpu.pushad(); }
  { let v: u32 = 0x17f9u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x447386); cpu.call(t); if cpu.returned_to != 0x447386 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x68u32); cpu.eax = v; }
  { let v: u32 = 0x17fau32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x447396); cpu.call(t); if cpu.returned_to != 0x447396 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0xa4u32); cpu.eax = v; }
  { let v: u32 = 0x17fbu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4473a8); cpu.call(t); if cpu.returned_to != 0x4473a8 { return; }
  { cpu.popad(); }
  pc = 0x4473a9; continue;
 }
 0x4473a9 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.sf { pc = 0x4473bc; continue; }
  pc = 0x4473ae; continue;
 }
 0x4473ae => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  cpu.push(0x4473bc); f_00447559(cpu); if cpu.returned_to != 0x4473bc { return; }
  pc = 0x4473bc; continue;
 }
 0x4473bc => {
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x447402; continue; }
  pc = 0x4473c5; continue;
 }
 0x4473c5 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x4473cf; continue; }
  pc = 0x4473ca; continue;
 }
 0x4473ca => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x4473e2; continue; }
  pc = 0x4473cf; continue;
 }
 0x4473cf => {
  { let v: u32 = cpu.eax.wrapping_add(0x23u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x1297u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4473e1); cpu.call(t); if cpu.returned_to != 0x4473e1 { return; }
  { cpu.popad(); }
  pc = 0x4473e2; continue;
 }
 0x4473e2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x581b67u32, v); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x4473ff); f_0041cf66(cpu); if cpu.returned_to != 0x4473ff { return; }
  { cpu.popad(); }
  pc = 0x447431; continue;
 }
 0x447402 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x44740d; continue; }
  pc = 0x447408; continue;
 }
 0x447408 => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44741d; continue; }
  pc = 0x44740d; continue;
 }
 0x44740d => {
  { let v: u32 = cpu.eax.wrapping_add(0x46u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1b80u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44741c); cpu.call(t); if cpu.returned_to != 0x44741c { return; }
  { cpu.popad(); }
  pc = 0x44741d; continue;
 }
 0x44741d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x100u32; cpu.edx = v; }
  cpu.push(0x447430); f_0041cf66(cpu); if cpu.returned_to != 0x447430 { return; }
  { cpu.popad(); }
  pc = 0x447431; continue;
 }
 0x447431 => {
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x44748e; continue; }
  pc = 0x447436; continue;
 }
 0x447436 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x447440; continue; }
  pc = 0x44743b; continue;
 }
 0x44743b => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44748e; continue; }
  pc = 0x447440; continue;
 }
 0x447440 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  if !cpu.zf { pc = 0x44748d; continue; }
  pc = 0x44744c; continue;
 }
 0x44744c => {
  { let v: u32 = cpu.eax.wrapping_add(0x27u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x44745a; continue; }
  pc = 0x447454; continue;
 }
 0x447454 => {
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44745a; continue;
 }
 0x44745a => {
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x191eu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if cpu.zf { pc = 0x447487; continue; }
  pc = 0x44746b; continue;
 }
 0x44746b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x447473; continue; }
  pc = 0x447470; continue;
 }
 0x447470 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  pc = 0x447473; continue;
 }
 0x447473 => {
  { let v: u32 = 0x191eu32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x447481); cpu.call(t); if cpu.returned_to != 0x447481 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x447487; continue;
 }
 0x447487 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44748d); cpu.call(t); if cpu.returned_to != 0x44748d { return; }
  pc = 0x44748d; continue;
 }
 0x44748d => {
  { cpu.popad(); }
  pc = 0x44748e; continue;
 }
 0x44748e => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x447552; continue; }
  pc = 0x447497; continue;
 }
 0x447497 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x447552; continue; }
  pc = 0x4474a0; continue;
 }
 0x4474a0 => {
  { cpu.sub8((cpu.ecx as u8), 0x8fu8); }
  if cpu.zf { pc = 0x447552; continue; }
  pc = 0x4474a9; continue;
 }
 0x4474a9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  if cpu.zf { pc = 0x4474d9; continue; }
  pc = 0x4474b8; continue;
 }
 0x4474b8 => {
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4474c7; continue;
 }
 0x4474c7 => {
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x4474d4; continue; }
  pc = 0x4474d0; continue;
 }
 0x4474d0 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4474d4; continue;
 }
 0x4474d4 => {
  cpu.push(0x4474d9); f_00447559(cpu); if cpu.returned_to != 0x4474d9 { return; }
  pc = 0x4474d9; continue;
 }
 0x4474d9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x447501; continue; }
  pc = 0x4474e0; continue;
 }
 0x4474e0 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x4474fc; continue; }
  pc = 0x4474f8; continue;
 }
 0x4474f8 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4474fc; continue;
 }
 0x4474fc => {
  cpu.push(0x447501); f_00447559(cpu); if cpu.returned_to != 0x447501 { return; }
  pc = 0x447501; continue;
 }
 0x447501 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x447529; continue; }
  pc = 0x447508; continue;
 }
 0x447508 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x447524; continue; }
  pc = 0x447520; continue;
 }
 0x447520 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x447524; continue;
 }
 0x447524 => {
  cpu.push(0x447529); f_00447559(cpu); if cpu.returned_to != 0x447529 { return; }
  pc = 0x447529; continue;
 }
 0x447529 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x447551; continue; }
  pc = 0x447530; continue;
 }
 0x447530 => {
  { let v: u8 = 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x44754c; continue; }
  pc = 0x447548; continue;
 }
 0x447548 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44754c; continue;
 }
 0x44754c => {
  cpu.push(0x447551); f_00447559(cpu); if cpu.returned_to != 0x447551 { return; }
  pc = 0x447551; continue;
 }
 0x447551 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x447552; continue;
 }
 0x447552 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447559(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447559); }
 let mut pc: u32 = 0x447559;
 loop { match pc {
 0x447559 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x1u16); }
  if !cpu.zf { pc = 0x44758c; continue; }
  pc = 0x447563; continue;
 }
 0x447563 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a4918u32; cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if !cpu.zf { pc = 0x447580; continue; }
  pc = 0x44757c; continue;
 }
 0x44757c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x447580; continue;
 }
 0x447580 => {
  cpu.push(0x447585); f_0041cf66(cpu); if cpu.returned_to != 0x447585 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44758c; continue;
 }
 0x44758c => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x447594); cpu.call(t); if cpu.returned_to != 0x447594 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x575f47u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u16 = cpu.r16(0x4d6ba8u32).wrapping_add(0xcu16); cpu.w16(0x4d6ba8u32, v); }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if cpu.zf { pc = 0x4475c6; continue; }
  pc = 0x4475bb; continue;
 }
 0x4475bb => {
  { let v: u16 = cpu.r16(0x4d6ba8u32).wrapping_add(0x4u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x3u32); cpu.ebx = v; }
  pc = 0x4475c6; continue;
 }
 0x4475c6 => {
  { let v: u32 = 0x136eu32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x4475d5); f_0041cf66(cpu); if cpu.returned_to != 0x4475d5 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6934u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004475e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4475e6); }
 let mut pc: u32 = 0x4475e6;
 loop { match pc {
 0x4475e6 => {
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f61u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x4e1f61u32); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x5092eeu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447617(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447617); }
 let mut pc: u32 = 0x447617;
 loop { match pc {
 0x447617 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1584u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1584u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x44762f); cpu.call(t); if cpu.returned_to != 0x44762f { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447760); }
 let mut pc: u32 = 0x447760;
 loop { match pc {
 0x447760 => {
  cpu.push(0x447765); f_00417c69(cpu); if cpu.returned_to != 0x447765 { return; }
  { let v: u32 = cpu.r32(0x5addf0u32); cpu.edx = v; }
  cpu.push(0x447770); f_004211b6(cpu); if cpu.returned_to != 0x447770 { return; }
  cpu.push(0x447775); f_0041c11a(cpu); if cpu.returned_to != 0x447775 { return; }
  { let v: u32 = cpu.r32(0x4d6ad8u32); cpu.esp = v; }
  cpu.push(0x447780); f_0046c9d8(cpu); if cpu.returned_to != 0x447780 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447781(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447781); }
 let mut pc: u32 = 0x447781;
 loop { match pc {
 0x447781 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x44778f); f_00421241(cpu); if cpu.returned_to != 0x44778f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447791); }
 let mut pc: u32 = 0x447791;
 loop { match pc {
 0x447791 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d682cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004477ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4477ab); }
 let mut pc: u32 = 0x4477ab;
 loop { match pc {
 0x4477ab => {
  { let v: u32 = 0x4ea8cau32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477b9; continue;
 }
 0x4477b9 => {
  { let v: u32 = 0x4ea8d6u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477c7; continue;
 }
 0x4477c7 => {
  { let v: u32 = 0x4ea8e3u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477d5; continue;
 }
 0x4477d5 => {
  { let v: u32 = 0x4ea8f0u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477e3; continue;
 }
 0x4477e3 => {
  { let v: u32 = 0x4ea8fdu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477f1; continue;
 }
 0x4477f1 => {
  { let v: u32 = 0x4ea90au32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x4477ff; continue;
 }
 0x4477ff => {
  { let v: u32 = 0x4ea917u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x44780d; continue; }
  pc = 0x44780d; continue;
 }
 0x44780d => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44781c); f_0040d370(cpu); if cpu.returned_to != 0x44781c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x576e7fu32, v); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x447839; continue;
 }
 0x447839 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576e7fu32, v); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d682cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d684eu32, v); }
  cpu.push(0x447859); f_00447791(cpu); if cpu.returned_to != 0x447859 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x447862); f_00447781(cpu); if cpu.returned_to != 0x447862 { return; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x4d6858u32; cpu.edx = v; }
  cpu.push(0x447871); f_0041c901(cpu); if cpu.returned_to != 0x447871 { return; }
  { let v: u16 = cpu.r16(0x4d6858u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5431b2u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447874); }
 let mut pc: u32 = 0x447874;
 loop { match pc {
 0x447874 => {
  { let v = 0xa366004du32; cpu.push(v); }
  { let v: u8 = 0x31u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  f_0044787e(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044787e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44787e); }
 let mut pc: u32 = 0x44787e;
 loop { match pc {
 0x44787e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d682cu32); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x44788f; continue; }
  pc = 0x44788a; continue;
 }
 0x44788a => {
  cpu.push(0x44788f); f_004211ec(cpu); if cpu.returned_to != 0x44788f { return; }
  pc = 0x44788f; continue;
 }
 0x44788f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d684eu32); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x4478a0; continue; }
  pc = 0x44789b; continue;
 }
 0x44789b => {
  cpu.push(0x4478a0); f_004211ec(cpu); if cpu.returned_to != 0x4478a0 { return; }
  pc = 0x4478a0; continue;
 }
 0x4478a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004478a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4478a1); }
 let mut pc: u32 = 0x4478a1;
 loop { match pc {
 0x4478a1 => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x1u8); }
  if cpu.zf { pc = 0x4478ca; continue; }
  pc = 0x4478aa; continue;
 }
 0x4478aa => {
  cpu.push(0x4478af); f_004175a9(cpu); if cpu.returned_to != 0x4478af { return; }
  { let v: u32 = 0x5431b4u32; cpu.edi = v; }
  { let v: u32 = 0x1d84u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x4d6ab8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6ab0u32, v); }
  pc = 0x4478ca; continue;
 }
 0x4478ca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004478cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4478cb); }
 let mut pc: u32 = 0x4478cb;
 loop { match pc {
 0x4478cb => {
  { cpu.pushad(); }
  { let v: u32 = 0x588856u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x4478df); f_0048ab3e(cpu); if cpu.returned_to != 0x4478df { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x44795e; continue; }
  pc = 0x4478e4; continue;
 }
 0x4478e4 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x4ea924u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x4478f4; continue;
 }
 0x4478f4 => {
  { let v: u32 = 0x4ea930u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x447902; continue;
 }
 0x447902 => {
  { let v: u32 = 0x4ea93du32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x447910; continue;
 }
 0x447910 => {
  { let v: u32 = 0x4ea94au32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x44791e; continue;
 }
 0x44791e => {
  { let v: u32 = 0x4ea957u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x44792c; continue;
 }
 0x44792c => {
  { let v: u32 = 0x4ea964u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x44793a; continue;
 }
 0x44793a => {
  { let v: u32 = 0x4ea971u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x447948; continue; }
  pc = 0x447948; continue;
 }
 0x447948 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x447956); f_0048ab3e(cpu); if cpu.returned_to != 0x447956 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x44795e; continue; }
  pc = 0x44795b; continue;
 }
 0x44795b => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44795e => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447961(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447961); }
 let mut pc: u32 = 0x447961;
 loop { match pc {
 0x447961 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6af8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5a48f4u32, v); }
  { cpu.sub8(cpu.r8(0x4e1f6au32), 0x0u8); }
  if cpu.zf { pc = 0x447999; continue; }
  pc = 0x447978; continue;
 }
 0x447978 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6af8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a48f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44798b); f_0040d400(cpu); if cpu.returned_to != 0x44798b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x447999; continue; }
  pc = 0x447993; continue;
 }
 0x447993 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x447a79; continue;
 }
 0x447999 => {
  { cpu.pushad(); }
  { let v: u32 = 0x588856u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x4479ad); f_0048ab3e(cpu); if cpu.returned_to != 0x4479ad { return; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  { let v: u8 = 0x3u8; cpu.w8(0x576e7fu32, v); }
  if cpu.zf { pc = 0x4479d5; continue; }
  pc = 0x4479b9; continue;
 }
 0x4479b9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576e7fu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.sf { pc = 0x4479dd; continue; }
  pc = 0x4479c4; continue;
 }
 0x4479c4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a94u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f3u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4479d5 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_0041c653(cpu); return;
 }
 0x4479dd => {
  { cpu.popad(); }
  { let v: u32 = 0x4ea924u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x4479ec; continue;
 }
 0x4479ec => {
  { let v: u32 = 0x4ea930u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x4479fa; continue;
 }
 0x4479fa => {
  { let v: u32 = 0x4ea93du32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x447a08; continue;
 }
 0x447a08 => {
  { let v: u32 = 0x4ea94au32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x447a16; continue;
 }
 0x447a16 => {
  { let v: u32 = 0x4ea957u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x447a24; continue;
 }
 0x447a24 => {
  { let v: u32 = 0x4ea964u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x447a32; continue;
 }
 0x447a32 => {
  { let v: u32 = 0x4ea971u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x447a40; continue; }
  pc = 0x447a40; continue;
 }
 0x447a40 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x447a4e); f_0048ab3e(cpu); if cpu.returned_to != 0x447a4e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  { let v: u8 = 0x3u8; cpu.w8(0x576e7fu32, v); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x447a5f; continue;
 }
 0x447a5f => {
  { let v: u8 = 0x0u8; cpu.w8(0x576e7fu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.sf { pc = 0x447a79; continue; }
  pc = 0x447a6a; continue;
 }
 0x447a6a => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f3u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a94u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447a79 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6af8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5a48f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x447a8d); f_0040d400(cpu); if cpu.returned_to != 0x447a8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x447ad0; continue; }
  pc = 0x447a95; continue;
 }
 0x447a95 => {
  { let v: u32 = cpu.r32(0x4d6af8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5a48f4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  cpu.push(0x447aaa); f_0041c919(cpu); if cpu.returned_to != 0x447aaa { return; }
  if !cpu.cf { pc = 0x447ac8; continue; }
  pc = 0x447aac; continue;
 }
 0x447aac => {
  { cpu.sub8(cpu.r8(0x4e1fb5u32), 0x0u8); }
  { let v: u8 = 0x5u8; cpu.w8(0x576e7fu32, v); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x447ac0; continue;
 }
 0x447ac0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576e7fu32, v); }
  { cpu.cf = true; }
  pc = 0x447ac8; continue;
 }
 0x447ac8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f3u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447ad0 => {
  { cpu.sub8(cpu.r8(0x4e1fb5u32), 0x0u8); }
  { let v: u8 = 0x6u8; cpu.w8(0x576e7fu32, v); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x447ae4; continue;
 }
 0x447ae4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576e7fu32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
