#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0103589b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103589b); }
 let mut pc: u32 = 0x103589b;
 loop { match pc {
 0x103589b => {
  { let v: u8 = cpu.r8(0x112446fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  cpu.push(0x10358a8); f_01012c1c(cpu); if cpu.returned_to != 0x10358a8 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112446du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  cpu.push(0x10358b7); f_010133d9(cpu); if cpu.returned_to != 0x10358b7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010358b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10358b8); }
 let mut pc: u32 = 0x10358b8;
 loop { match pc {
 0x10358b8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10358bf); f_0101329b(cpu); if cpu.returned_to != 0x10358bf { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010358c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10358c0); }
 let mut pc: u32 = 0x10358c0;
 loop { match pc {
 0x10358c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x11b4c6du32; cpu.edx = v; }
  cpu.push(0x10358cc); f_01013067(cpu); if cpu.returned_to != 0x10358cc { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10358d3); f_01013224(cpu); if cpu.returned_to != 0x10358d3 { return; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x10358d8; continue;
 }
 0x10358d8 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e4350u32); cpu.push(0x10358df); cpu.call(t); if cpu.returned_to != 0x10358df { return; }
  { cpu.popad(); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10358d8; continue; }
  pc = 0x10358e2; continue;
 }
 0x10358e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010358e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10358e3); }
 let mut pc: u32 = 0x10358e3;
 loop { match pc {
 0x10358e3 => {
  { let v: u32 = 0x1124630u32; cpu.edx = v; }
  { let v: u32 = 0x111138cu32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x10358f8); f_01033bea(cpu); if cpu.returned_to != 0x10358f8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1119dc9u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1119dc5u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1119d90u32, v); }
  cpu.push(0x103591c); f_01034269(cpu); if cpu.returned_to != 0x103591c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103591d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103591d); }
 let mut pc: u32 = 0x103591d;
 loop { match pc {
 0x103591d => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035940; continue; }
  pc = 0x1035926; continue;
 }
 0x1035926 => {
  { cpu.sub8(cpu.r8(0x1124469u32), 0x0u8); }
  if cpu.zf { pc = 0x1035941; continue; }
  pc = 0x103592f; continue;
 }
 0x103592f => {
  { cpu.sub16(cpu.r16(0x110d767u32), 0x0u16); }
  if cpu.zf { pc = 0x1035941; continue; }
  pc = 0x1035939; continue;
 }
 0x1035939 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x110d767u32)); cpu.w16(0x110d767u32, v); }
  pc = 0x1035940; continue;
 }
 0x1035940 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035941 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1035a87; continue; }
  pc = 0x103594e; continue;
 }
 0x103594e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1035b88; continue; }
  pc = 0x1035962; continue;
 }
 0x1035962 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112447bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110c198u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110c1c8u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x110c1cau32, v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x10359b0); f_01035d35(cpu); if cpu.returned_to != 0x10359b0 { return; }
  if cpu.cf { pc = 0x1035a30; continue; }
  pc = 0x10359b6; continue;
 }
 0x10359b6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc4cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112447bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x110c1c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110c1cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x10359e7); f_01035da4(cpu); if cpu.returned_to != 0x10359e7 { return; }
  if cpu.cf { pc = 0x10359ee; continue; }
  pc = 0x10359e9; continue;
 }
 0x10359e9 => {
  pc = 0x1035b88; continue;
 }
 0x10359ee => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if cpu.zf { pc = 0x1035c8f; continue; }
  pc = 0x10359fb; continue;
 }
 0x10359fb => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035a09; continue; }
  pc = 0x1035a04; continue;
 }
 0x1035a04 => {
  pc = 0x1035c2f; continue;
 }
 0x1035a09 => {
  { let v: u8 = 0x7u8; cpu.w8(0x10c7968u32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x1105ea6u32, v); }
  cpu.push(0x1035a1f); f_0105f178(cpu); if cpu.returned_to != 0x1035a1f { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x112445cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1124458u32)); cpu.eax = v; }
  pc = 0x1035c62; continue;
 }
 0x1035a30 => {
  { cpu.sub8(cpu.r8(0x10fcc4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1035a45; continue; }
  pc = 0x1035a39; continue;
 }
 0x1035a39 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc4cu32, v); }
  cpu.push(0x1035a45); f_01035469(cpu); if cpu.returned_to != 0x1035a45 { return; }
  pc = 0x1035a45; continue;
 }
 0x1035a45 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x8u8); }
  if cpu.zf { pc = 0x1035c8f; continue; }
  pc = 0x1035a52; continue;
 }
 0x1035a52 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035a60; continue; }
  pc = 0x1035a5b; continue;
 }
 0x1035a5b => {
  pc = 0x1035c2f; continue;
 }
 0x1035a60 => {
  { let v: u8 = 0x8u8; cpu.w8(0x10c7968u32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x1105ea6u32, v); }
  cpu.push(0x1035a76); f_0105f178(cpu); if cpu.returned_to != 0x1035a76 { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x1124464u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1124460u32)); cpu.eax = v; }
  pc = 0x1035c62; continue;
 }
 0x1035a87 => {
  { let v: u32 = cpu.r8(0x10e3d63u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112447bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1035ab7); f_01035cc6(cpu); if cpu.returned_to != 0x1035ab7 { return; }
  if !cpu.cf { pc = 0x1035afb; continue; }
  pc = 0x1035ab9; continue;
 }
 0x1035ab9 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if cpu.zf { pc = 0x1035c8f; continue; }
  pc = 0x1035ac6; continue;
 }
 0x1035ac6 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035ad4; continue; }
  pc = 0x1035acf; continue;
 }
 0x1035acf => {
  pc = 0x1035c2f; continue;
 }
 0x1035ad4 => {
  { let v: u8 = 0x7u8; cpu.w8(0x10c7968u32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x1105ea6u32, v); }
  cpu.push(0x1035aea); f_0105f178(cpu); if cpu.returned_to != 0x1035aea { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x112445cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1124458u32)); cpu.eax = v; }
  pc = 0x1035c62; continue;
 }
 0x1035afb => {
  { let v: u32 = cpu.r8(0x10e3d63u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112447bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1035b2b); f_01035cc6(cpu); if cpu.returned_to != 0x1035b2b { return; }
  if !cpu.cf { pc = 0x1035b88; continue; }
  pc = 0x1035b31; continue;
 }
 0x1035b31 => {
  { cpu.sub8(cpu.r8(0x10fcc4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1035b46; continue; }
  pc = 0x1035b3a; continue;
 }
 0x1035b3a => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc4cu32, v); }
  cpu.push(0x1035b46); f_01035469(cpu); if cpu.returned_to != 0x1035b46 { return; }
  pc = 0x1035b46; continue;
 }
 0x1035b46 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x8u8); }
  if cpu.zf { pc = 0x1035c8f; continue; }
  pc = 0x1035b53; continue;
 }
 0x1035b53 => {
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035b61; continue; }
  pc = 0x1035b5c; continue;
 }
 0x1035b5c => {
  pc = 0x1035c2f; continue;
 }
 0x1035b61 => {
  { let v: u8 = 0x8u8; cpu.w8(0x10c7968u32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x1105ea6u32, v); }
  cpu.push(0x1035b77); f_0105f178(cpu); if cpu.returned_to != 0x1035b77 { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x1124464u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1124460u32)); cpu.eax = v; }
  pc = 0x1035c62; continue;
 }
 0x1035b88 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc4cu32, v); }
  { let v: u16 = cpu.r16(0x10c6a58u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x1035c2f; continue; }
  pc = 0x1035b9f; continue;
 }
 0x1035b9f => {
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (cpu.sf == cpu.of) { pc = 0x1035c18; continue; }
  pc = 0x1035ba9; continue;
 }
 0x1035ba9 => {
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x1035c0a; continue; }
  pc = 0x1035bb3; continue;
 }
 0x1035bb3 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x1035bf3; continue; }
  pc = 0x1035bb9; continue;
 }
 0x1035bb9 => {
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x1035bd9; continue; }
  pc = 0x1035bbf; continue;
 }
 0x1035bbf => {
  { let v: u32 = 0x1124448u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035c0a; continue; }
  pc = 0x1035bcd; continue;
 }
 0x1035bcd => {
  { let v: u8 = 0x1u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035bd9 => {
  { let v: u32 = 0x1124438u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035c0a; continue; }
  pc = 0x1035be7; continue;
 }
 0x1035be7 => {
  { let v: u8 = 0x2u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035bf3 => {
  { let v: u32 = 0x1124428u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x1035c0a; continue; }
  pc = 0x1035c01; continue;
 }
 0x1035c01 => {
  { let v: u8 = 0x3u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035c0a => {
  { let v: u32 = 0x1124418u32; cpu.esi = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035c18 => {
  { let v: u32 = 0x1124408u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10c7968u32), 0x7u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1035c0a; continue; }
  pc = 0x1035c26; continue;
 }
 0x1035c26 => {
  { let v: u8 = 0x5u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035c2f => {
  { let v: u32 = 0x11243f8u32; cpu.esi = v; }
  { let v: u8 = 0x7u8; cpu.w8(0x112454bu32, v); }
  pc = 0x1035c3b; continue;
 }
 0x1035c3b => {
  { let v: u8 = cpu.r8(0x112454bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10c7968u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1035c8f; continue; }
  pc = 0x1035c48; continue;
 }
 0x1035c48 => {
  { let v: u32 = 0xffu32; cpu.w32(0x1105ea6u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10c7968u32, v); }
  cpu.push(0x1035c5c); f_0105f178(cpu); if cpu.returned_to != 0x1035c5c { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1035c62; continue;
 }
 0x1035c62 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x112454cu32), 0x0u8); }
  if cpu.zf { pc = 0x1035c7c; continue; }
  pc = 0x1035c77; continue;
 }
 0x1035c77 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x1035c7c; continue;
 }
 0x1035c7c => {
  { let v: u32 = cpu.edx; cpu.w32(0x10c7969u32, v); }
  cpu.push(0x1035c87); f_010131a8(cpu); if cpu.returned_to != 0x1035c87 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x110d767u32, v); }
  pc = 0x1035c8e; continue;
 }
 0x1035c8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035c8f => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1105ea6u32)); cpu.w32(0x1105ea6u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x1035c8e; continue; }
  pc = 0x1035c97; continue;
 }
 0x1035c97 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc4cu32, v); }
  cpu.poll(); pc = 0x1035c48; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035ca1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035ca1); }
 let mut pc: u32 = 0x1035ca1;
 loop { match pc {
 0x1035ca1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e40c0u32), cpu.eax); cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035cc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035cc6); }
 let mut pc: u32 = 0x1035cc6;
 loop { match pc {
 0x1035cc6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124472u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1124474u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1124476u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1124477u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1035ce9; continue;
 }
 0x1035ce9 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x1035d29; continue; }
  pc = 0x1035cef; continue;
 }
 0x1035cef => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1124476u32)); }
  if !cpu.zf { pc = 0x1035d29; continue; }
  pc = 0x1035cfc; continue;
 }
 0x1035cfc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1124472u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1124474u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1035d20); f_01035ca1(cpu); if cpu.returned_to != 0x1035d20 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x1124477u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1035d33; continue; }
  pc = 0x1035d29; continue;
 }
 0x1035d29 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1035ce9; continue; }
  pc = 0x1035d31; continue;
 }
 0x1035d31 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035d33 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035d35); }
 let mut pc: u32 = 0x1035d35;
 loop { match pc {
 0x1035d35 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124472u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1124474u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1124476u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1124477u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1035d58; continue;
 }
 0x1035d58 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x1035d98; continue; }
  pc = 0x1035d5e; continue;
 }
 0x1035d5e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1124476u32)); }
  if cpu.zf { pc = 0x1035d98; continue; }
  pc = 0x1035d6b; continue;
 }
 0x1035d6b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1124472u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1124474u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1035d8f); f_01035ca1(cpu); if cpu.returned_to != 0x1035d8f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x1124477u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1035da2; continue; }
  pc = 0x1035d98; continue;
 }
 0x1035d98 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1035d58; continue; }
  pc = 0x1035da0; continue;
 }
 0x1035da0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035da2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035da4); }
 let mut pc: u32 = 0x1035da4;
 loop { match pc {
 0x1035da4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124472u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1124474u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1124476u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1124477u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1035dc7; continue;
 }
 0x1035dc7 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x1035e60; continue; }
  pc = 0x1035dd1; continue;
 }
 0x1035dd1 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1124476u32)); }
  if !cpu.zf { pc = 0x1035e60; continue; }
  pc = 0x1035de2; continue;
 }
 0x1035de2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = 0x110c198u32; cpu.esi = v; }
  pc = 0x1035ded; continue;
 }
 0x1035ded => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10e3d7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1035e55; continue; }
  pc = 0x1035e03; continue;
 }
 0x1035e03 => {
  { cpu.sub8(cpu.r8(0x10fd438u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1035e55; continue; }
  pc = 0x1035e0c; continue;
 }
 0x1035e0c => {
  { cpu.sub8(cpu.r8(0x10fd43cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1035e55; continue; }
  pc = 0x1035e15; continue;
 }
 0x1035e15 => {
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
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x1124472u32)); }
  if !cpu.zf { pc = 0x1035e48; continue; }
  pc = 0x1035e3f; continue;
 }
 0x1035e3f => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1124474u32)); }
  if cpu.zf { pc = 0x1035e55; continue; }
  pc = 0x1035e48; continue;
 }
 0x1035e48 => {
  cpu.push(0x1035e4d); f_01035ca1(cpu); if cpu.returned_to != 0x1035e4d { return; }
  { cpu.sub32(cpu.r32(0x1124477u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1035e6f; continue; }
  pc = 0x1035e55; continue;
 }
 0x1035e55 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x5u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1035ded; continue; }
  pc = 0x1035e5f; continue;
 }
 0x1035e5f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1035e60; continue;
 }
 0x1035e60 => {
  { let v: u32 = cpu.edi.wrapping_add(0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1035dc7; continue; }
  pc = 0x1035e6d; continue;
 }
 0x1035e6d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035e6f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035e73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035e73); }
 let mut pc: u32 = 0x1035e73;
 loop { match pc {
 0x1035e73 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035f23; continue; }
  pc = 0x1035e80; continue;
 }
 0x1035e80 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x1035ee7; continue; }
  pc = 0x1035e8c; continue;
 }
 0x1035e8c => {
  { let v: u16 = cpu.r16(0xa1u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10cu32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7950u16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035e9f; continue;
 }
 0x1035e9f => {
  { let v: u32 = 0x10bu32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x614cu16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035eab; continue;
 }
 0x1035eab => {
  { let v: u32 = 0x10au32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x6f54u16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035eb7; continue;
 }
 0x1035eb7 => {
  { let v: u32 = 0x108u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7350u16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035ec3; continue;
 }
 0x1035ec3 => {
  { let v: u32 = 0x109u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x6e53u16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035ecf; continue;
 }
 0x1035ecf => {
  { let v: u32 = 0x107u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7247u16); }
  if cpu.zf { pc = 0x1035edd; continue; }
  pc = 0x1035edb; continue;
 }
 0x1035edb => {
  pc = 0x1035ee7; continue;
 }
 0x1035edd => {
  cpu.push(0x1035ee2); f_0105f178(cpu); if cpu.returned_to != 0x1035ee2 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  if cpu.zf { pc = 0x1035f0e; continue; }
  pc = 0x1035ee7; continue;
 }
 0x1035ee7 => {
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2a7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1035ef8); f_01036086(cpu); if cpu.returned_to != 0x1035ef8 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2a7u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057bbu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  pc = 0x1035f0e; continue;
 }
 0x1035f0e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1035f22); f_01033e70(cpu); if cpu.returned_to != 0x1035f22 { return; }
  { cpu.popad(); }
  pc = 0x1035f23; continue;
 }
 0x1035f23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035f24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035f24); }
 let mut pc: u32 = 0x1035f24;
 loop { match pc {
 0x1035f24 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035f7b; continue; }
  pc = 0x1035f2e; continue;
 }
 0x1035f2e => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1035f40; continue; }
  pc = 0x1035f37; continue;
 }
 0x1035f37 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035f7b; continue; }
  pc = 0x1035f40; continue;
 }
 0x1035f40 => {
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2bdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1035f51); f_01036086(cpu); if cpu.returned_to != 0x1035f51 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2bdu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057d9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1035f7b); f_01033e70(cpu); if cpu.returned_to != 0x1035f7b { return; }
  pc = 0x1035f7b; continue;
 }
 0x1035f7b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035f7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035f7d); }
 let mut pc: u32 = 0x1035f7d;
 loop { match pc {
 0x1035f7d => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035fd4; continue; }
  pc = 0x1035f87; continue;
 }
 0x1035f87 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1035f99; continue; }
  pc = 0x1035f90; continue;
 }
 0x1035f90 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035fd4; continue; }
  pc = 0x1035f99; continue;
 }
 0x1035f99 => {
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2a9u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1035faa); f_01036086(cpu); if cpu.returned_to != 0x1035faa { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2a9u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057c7u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1035fd4); f_01033e70(cpu); if cpu.returned_to != 0x1035fd4 { return; }
  pc = 0x1035fd4; continue;
 }
 0x1035fd4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035fd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035fd6); }
 let mut pc: u32 = 0x1035fd6;
 loop { match pc {
 0x1035fd6 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103602d; continue; }
  pc = 0x1035fe0; continue;
 }
 0x1035fe0 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1035ff2; continue; }
  pc = 0x1035fe9; continue;
 }
 0x1035fe9 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x103602d; continue; }
  pc = 0x1035ff2; continue;
 }
 0x1035ff2 => {
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2abu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1036003); f_01036086(cpu); if cpu.returned_to != 0x1036003 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2abu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057e5u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x103602d); f_01033e70(cpu); if cpu.returned_to != 0x103602d { return; }
  pc = 0x103602d; continue;
 }
 0x103602d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103602f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103602f); }
 let mut pc: u32 = 0x103602f;
 loop { match pc {
 0x103602f => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036084; continue; }
  pc = 0x1036039; continue;
 }
 0x1036039 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x103604b; continue; }
  pc = 0x1036042; continue;
 }
 0x1036042 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036084; continue; }
  pc = 0x103604b; continue;
 }
 0x103604b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2b5u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103605d); f_01036086(cpu); if cpu.returned_to != 0x103605d { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2b5u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x110582bu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1036084); f_01033e70(cpu); if cpu.returned_to != 0x1036084 { return; }
  pc = 0x1036084; continue;
 }
 0x1036084 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036086(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036086); }
 let mut pc: u32 = 0x1036086;
 loop { match pc {
 0x1036086 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  pc = 0x103608e; continue;
 }
 0x103608e => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103608e; continue; }
  pc = 0x1036093; continue;
 }
 0x1036093 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x103609a); f_0105f329(cpu); if cpu.returned_to != 0x103609a { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & cpu.ebp; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { pc = 0x10360a8; continue; }
  pc = 0x10360a3; continue;
 }
 0x10360a3 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  pc = 0x10360a8; continue;
 }
 0x10360a8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x10360b4; continue; }
  pc = 0x10360b2; continue;
 }
 0x10360b2 => {
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x10360b4; continue;
 }
 0x10360b4 => {
  { cpu.and16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x10360ca; continue; }
  pc = 0x10360b9; continue;
 }
 0x10360b9 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10360b4; continue; }
  pc = 0x10360c1; continue;
 }
 0x10360c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x10360b4; continue;
 }
 0x10360ca => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010360cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10360cf); }
 let mut pc: u32 = 0x10360cf;
 loop { match pc {
 0x10360cf => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036163; continue; }
  pc = 0x10360dd; continue;
 }
 0x10360dd => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x10360f3; continue; }
  pc = 0x10360e6; continue;
 }
 0x10360e6 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036163; continue; }
  pc = 0x10360f3; continue;
 }
 0x10360f3 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x40u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1036163; continue; }
  pc = 0x1036106; continue;
 }
 0x1036106 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1036163; continue; }
  pc = 0x103610c; continue;
 }
 0x103610c => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036163; continue; }
  pc = 0x1036119; continue;
 }
 0x1036119 => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2b7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103612a); f_01036086(cpu); if cpu.returned_to != 0x103612a { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2b7u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x110581bu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1036151); f_01033e70(cpu); if cpu.returned_to != 0x1036151 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x1036163; continue;
 }
 0x1036163 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036165(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036165); }
 let mut pc: u32 = 0x1036165;
 loop { match pc {
 0x1036165 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10361d5; continue; }
  pc = 0x1036174; continue;
 }
 0x1036174 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036186; continue; }
  pc = 0x103617d; continue;
 }
 0x103617d => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10361d5; continue; }
  pc = 0x1036186; continue;
 }
 0x1036186 => {
  { let v: u8 = 0x11u8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x10361ce; continue; }
  pc = 0x1036190; continue;
 }
 0x1036190 => {
  cpu.push(0x1036195); f_01047a18(cpu); if cpu.returned_to != 0x1036195 { return; }
  if cpu.zf { pc = 0x10361ce; continue; }
  pc = 0x1036197; continue;
 }
 0x1036197 => {
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2b9u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10361a8); f_01036086(cpu); if cpu.returned_to != 0x10361a8 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2b9u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1105825u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x10361ce); f_01033e70(cpu); if cpu.returned_to != 0x10361ce { return; }
  pc = 0x10361ce; continue;
 }
 0x10361ce => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10361d5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010361d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10361d8); }
 let mut pc: u32 = 0x10361d8;
 loop { match pc {
 0x10361d8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036222; continue; }
  pc = 0x10361e2; continue;
 }
 0x10361e2 => {
  { cpu.sub32(cpu.r32(0x10e41bcu32), 0x725u32); }
  if cpu.zf { pc = 0x1036222; continue; }
  pc = 0x10361ee; continue;
 }
 0x10361ee => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036200; continue; }
  pc = 0x10361f7; continue;
 }
 0x10361f7 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036222; continue; }
  pc = 0x1036200; continue;
 }
 0x1036200 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036222; continue; }
  pc = 0x103620d; continue;
 }
 0x103620d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x725u32; cpu.ebx = v; }
  cpu.push(0x1036222); f_01033e70(cpu); if cpu.returned_to != 0x1036222 { return; }
  pc = 0x1036222; continue;
 }
 0x1036222 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036224); }
 let mut pc: u32 = 0x1036224;
 loop { match pc {
 0x1036224 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036262; continue; }
  pc = 0x103622e; continue;
 }
 0x103622e => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036240; continue; }
  pc = 0x1036237; continue;
 }
 0x1036237 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036262; continue; }
  pc = 0x1036240; continue;
 }
 0x1036240 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036262; continue; }
  pc = 0x103624d; continue;
 }
 0x103624d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x726u32; cpu.ebx = v; }
  cpu.push(0x1036262); f_01033e70(cpu); if cpu.returned_to != 0x1036262 { return; }
  pc = 0x1036262; continue;
 }
 0x1036262 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036264(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036264); }
 let mut pc: u32 = 0x1036264;
 loop { match pc {
 0x1036264 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10362a2; continue; }
  pc = 0x103626e; continue;
 }
 0x103626e => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036280; continue; }
  pc = 0x1036277; continue;
 }
 0x1036277 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10362a2; continue; }
  pc = 0x1036280; continue;
 }
 0x1036280 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10362a2; continue; }
  pc = 0x103628d; continue;
 }
 0x103628d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x82fu32; cpu.ebx = v; }
  cpu.push(0x10362a2); f_01033e70(cpu); if cpu.returned_to != 0x10362a2 { return; }
  pc = 0x10362a2; continue;
 }
 0x10362a2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010362a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10362a4); }
 let mut pc: u32 = 0x10362a4;
 loop { match pc {
 0x10362a4 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10362e2; continue; }
  pc = 0x10362ae; continue;
 }
 0x10362ae => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x10362c0; continue; }
  pc = 0x10362b7; continue;
 }
 0x10362b7 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10362e2; continue; }
  pc = 0x10362c0; continue;
 }
 0x10362c0 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10362e2; continue; }
  pc = 0x10362cd; continue;
 }
 0x10362cd => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x727u32; cpu.ebx = v; }
  cpu.push(0x10362e2); f_01033e70(cpu); if cpu.returned_to != 0x10362e2 { return; }
  pc = 0x10362e2; continue;
 }
 0x10362e2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010362e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10362e4); }
 let mut pc: u32 = 0x10362e4;
 loop { match pc {
 0x10362e4 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036322; continue; }
  pc = 0x10362ee; continue;
 }
 0x10362ee => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036300; continue; }
  pc = 0x10362f7; continue;
 }
 0x10362f7 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036322; continue; }
  pc = 0x1036300; continue;
 }
 0x1036300 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036322; continue; }
  pc = 0x103630d; continue;
 }
 0x103630d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x830u32; cpu.ebx = v; }
  cpu.push(0x1036322); f_01033e70(cpu); if cpu.returned_to != 0x1036322 { return; }
  pc = 0x1036322; continue;
 }
 0x1036322 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036324); }
 let mut pc: u32 = 0x1036324;
 loop { match pc {
 0x1036324 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036362; continue; }
  pc = 0x103632e; continue;
 }
 0x103632e => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036340; continue; }
  pc = 0x1036337; continue;
 }
 0x1036337 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036362; continue; }
  pc = 0x1036340; continue;
 }
 0x1036340 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036362; continue; }
  pc = 0x103634d; continue;
 }
 0x103634d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x831u32; cpu.ebx = v; }
  cpu.push(0x1036362); f_01033e70(cpu); if cpu.returned_to != 0x1036362 { return; }
  pc = 0x1036362; continue;
 }
 0x1036362 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036364(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036364); }
 let mut pc: u32 = 0x1036364;
 loop { match pc {
 0x1036364 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x103636f); f_010363b2(cpu); if cpu.returned_to != 0x103636f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036371); }
 let mut pc: u32 = 0x1036371;
 loop { match pc {
 0x1036371 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x103637c); f_010363b2(cpu); if cpu.returned_to != 0x103637c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103637e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103637e); }
 let mut pc: u32 = 0x103637e;
 loop { match pc {
 0x103637e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1036389); f_010363b2(cpu); if cpu.returned_to != 0x1036389 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036398); }
 let mut pc: u32 = 0x1036398;
 loop { match pc {
 0x1036398 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.push(0x10363a3); f_010363b2(cpu); if cpu.returned_to != 0x10363a3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010363a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10363a5); }
 let mut pc: u32 = 0x10363a5;
 loop { match pc {
 0x10363a5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x10363b0); f_010363b2(cpu); if cpu.returned_to != 0x10363b0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010363b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10363b2); }
 let mut pc: u32 = 0x10363b2;
 loop { match pc {
 0x10363b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.eax)), 0xffu8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x10363e7; continue; }
  pc = 0x10363c4; continue;
 }
 0x10363c4 => {
  { cpu.sub32(cpu.r32(0x10fcc4fu32), 0x0u32); }
  if !cpu.zf { pc = 0x10363e7; continue; }
  pc = 0x10363cd; continue;
 }
 0x10363cd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10363d4); f_010138b3(cpu); if cpu.returned_to != 0x10363d4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10fd019u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc4fu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10363e7; continue;
 }
 0x10363e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010363e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10363e8); }
 let mut pc: u32 = 0x10363e8;
 loop { match pc {
 0x10363e8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036426; continue; }
  pc = 0x10363f2; continue;
 }
 0x10363f2 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036404; continue; }
  pc = 0x10363fb; continue;
 }
 0x10363fb => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036426; continue; }
  pc = 0x1036404; continue;
 }
 0x1036404 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036426; continue; }
  pc = 0x1036411; continue;
 }
 0x1036411 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x832u32; cpu.ebx = v; }
  cpu.push(0x1036426); f_01033e70(cpu); if cpu.returned_to != 0x1036426 { return; }
  pc = 0x1036426; continue;
 }
 0x1036426 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036428(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036428); }
 let mut pc: u32 = 0x1036428;
 loop { match pc {
 0x1036428 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036466; continue; }
  pc = 0x1036432; continue;
 }
 0x1036432 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036444; continue; }
  pc = 0x103643b; continue;
 }
 0x103643b => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036466; continue; }
  pc = 0x1036444; continue;
 }
 0x1036444 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1036466; continue; }
  pc = 0x1036451; continue;
 }
 0x1036451 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x833u32; cpu.ebx = v; }
  cpu.push(0x1036466); f_01033e70(cpu); if cpu.returned_to != 0x1036466 { return; }
  pc = 0x1036466; continue;
 }
 0x1036466 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036468(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036468); }
 let mut pc: u32 = 0x1036468;
 loop { match pc {
 0x1036468 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10364a6; continue; }
  pc = 0x1036472; continue;
 }
 0x1036472 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036484; continue; }
  pc = 0x103647b; continue;
 }
 0x103647b => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10364a6; continue; }
  pc = 0x1036484; continue;
 }
 0x1036484 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10364a6; continue; }
  pc = 0x1036491; continue;
 }
 0x1036491 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x834u32; cpu.ebx = v; }
  cpu.push(0x10364a6); f_01033e70(cpu); if cpu.returned_to != 0x10364a6 { return; }
  pc = 0x10364a6; continue;
 }
 0x10364a6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010364a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10364a8); }
 let mut pc: u32 = 0x10364a8;
 loop { match pc {
 0x10364a8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10364e6; continue; }
  pc = 0x10364b2; continue;
 }
 0x10364b2 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x10364c4; continue; }
  pc = 0x10364bb; continue;
 }
 0x10364bb => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10364e6; continue; }
  pc = 0x10364c4; continue;
 }
 0x10364c4 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10364e6; continue; }
  pc = 0x10364d1; continue;
 }
 0x10364d1 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x835u32; cpu.ebx = v; }
  cpu.push(0x10364e6); f_01033e70(cpu); if cpu.returned_to != 0x10364e6 { return; }
  pc = 0x10364e6; continue;
 }
 0x10364e6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010364e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10364e8); }
 let mut pc: u32 = 0x10364e8;
 loop { match pc {
 0x10364e8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103653d; continue; }
  pc = 0x10364f2; continue;
 }
 0x10364f2 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036504; continue; }
  pc = 0x10364fb; continue;
 }
 0x10364fb => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x103653d; continue; }
  pc = 0x1036504; continue;
 }
 0x1036504 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2adu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1036516); f_01036086(cpu); if cpu.returned_to != 0x1036516 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2adu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057ebu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x103653d); f_01033e70(cpu); if cpu.returned_to != 0x103653d { return; }
  pc = 0x103653d; continue;
 }
 0x103653d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103653f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103653f); }
 let mut pc: u32 = 0x103653f;
 loop { match pc {
 0x103653f => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036594; continue; }
  pc = 0x1036549; continue;
 }
 0x1036549 => {
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x103655b; continue; }
  pc = 0x1036552; continue;
 }
 0x1036552 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036594; continue; }
  pc = 0x103655b; continue;
 }
 0x103655b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2afu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103656d); f_01036086(cpu); if cpu.returned_to != 0x103656d { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2afu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057f3u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1036594); f_01033e70(cpu); if cpu.returned_to != 0x1036594 { return; }
  pc = 0x1036594; continue;
 }
 0x1036594 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036596(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036596); }
 let mut pc: u32 = 0x1036596;
 loop { match pc {
 0x1036596 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x10365a9; continue; }
  pc = 0x10365a0; continue;
 }
 0x10365a0 => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036602; continue; }
  pc = 0x10365a9; continue;
 }
 0x10365a9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2b1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10365bb); f_01036086(cpu); if cpu.returned_to != 0x10365bb { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2b1u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1105801u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v = cpu.r32(0x10e3f68u32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x1119d94u32, v); }
  cpu.push(0x10365fc); f_01033e70(cpu); if cpu.returned_to != 0x10365fc { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f68u32, v); }
  pc = 0x1036602; continue;
 }
 0x1036602 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036604(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036604); }
 let mut pc: u32 = 0x1036604;
 loop { match pc {
 0x1036604 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10fcc12u32), 0x0u32); }
  if cpu.zf { pc = 0x1036617; continue; }
  pc = 0x103660e; continue;
 }
 0x103660e => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1036670; continue; }
  pc = 0x1036617; continue;
 }
 0x1036617 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2b3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1036629); f_01036086(cpu); if cpu.returned_to != 0x1036629 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2b3u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x110580du32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v = cpu.r32(0x10e3f68u32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x1119d94u32, v); }
  cpu.push(0x103666a); f_01033e70(cpu); if cpu.returned_to != 0x103666a { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f68u32, v); }
  pc = 0x1036670; continue;
 }
 0x1036670 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036672(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036672); }
 let mut pc: u32 = 0x1036672;
 loop { match pc {
 0x1036672 => {
  { cpu.sub8(cpu.r8(0x1124469u32), 0x0u8); }
  if cpu.zf { pc = 0x10366a5; continue; }
  pc = 0x103667b; continue;
 }
 0x103667b => {
  { cpu.sub8(cpu.r8(0x10366a7u32), 0x0u8); }
  if cpu.zf { pc = 0x10366a5; continue; }
  pc = 0x1036684; continue;
 }
 0x1036684 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  cpu.push(0x1036693); f_0105f329(cpu); if cpu.returned_to != 0x1036693 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x5au32); cpu.eax = v; }
  cpu.push(0x103669e); f_01012780(cpu); if cpu.returned_to != 0x103669e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10366a7u32, v); }
  pc = 0x10366a5; continue;
 }
 0x10366a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010366a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10366a6); }
 let mut pc: u32 = 0x10366a6;
 loop { match pc {
 0x10366a6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010366a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10366a7); } r_010366a7(cpu, 0x10366a7); }
pub fn f_010366a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10366a8); } r_010366a7(cpu, 0x10366a8); }
fn r_010366a7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10366a7 => {
  { let v: u8 = cpu.r8(0xffffff81u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff81u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10366b0; continue;
 }
 0x10366a8 => {
  { cpu.sub16((cpu.ecx as u16), 0x1c2au16); }
  if cpu.cf { pc = 0x10366b0; continue; }
  pc = 0x10366af; continue;
 }
 0x10366af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10366b0 => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10366c2); f_010338ed(cpu); if cpu.returned_to != 0x10366c2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4130u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10c052cu32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  cpu.push(0x10366da); f_010366f1(cpu); if cpu.returned_to != 0x10366da { return; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10366e9; continue; }
  pc = 0x10366e3; continue;
 }
 0x10366e3 => {
  cpu.push(0x10366e8); f_0102dd7c(cpu); if cpu.returned_to != 0x10366e8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10366e9 => {
  cpu.push(0x10366ee); f_0102e3dc(cpu); if cpu.returned_to != 0x10366ee { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010366f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10366f1); }
 let mut pc: u32 = 0x10366f1;
 loop { match pc {
 0x10366f1 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118cd68u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edi = v; }
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
pub fn f_01036708(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036708); }
 let mut pc: u32 = 0x1036708;
 loop { match pc {
 0x1036708 => {
  { let v: u16 = 0x1u16; cpu.w16(0x10e3d75u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x1125282u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef381u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124469u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112446bu32, v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x112446du32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x112446fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc12u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3f03u32, v); }
  { let v: u8 = 0x14u8; cpu.w8(0x1124f57u32, v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x10e3f1du32, v); }
  { let v: u8 = 0x8u8; cpu.w8(0x10ef382u32, v); }
  { let v: u32 = 0x30303030u32; cpu.w32(0x10ef383u32, v); }
  { let v: u32 = 0x30303030u32; cpu.w32(0x10ef387u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef38bu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x11b51b2u32, v); }
  { let v: u32 = 0x1bu32; cpu.w32(0x10e4128u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10e3d63u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f17u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f37u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f39u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f41u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10ef374u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f1bu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3f19u32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x10e412cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d954u32, v); }
  { let v: u8 = 0x18u8; cpu.w8(0x10fd4cdu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11302acu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x110d897u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x110d899u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x110d89bu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x110d89du32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d89fu32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8a1u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8a3u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8a5u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8a7u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8a9u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8abu32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8adu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8afu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8b1u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8b3u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8b5u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8b7u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8b9u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8bbu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8bdu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8bfu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8c1u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8c3u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8c5u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8c7u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8c9u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8cbu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8cdu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8cfu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8d1u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8d3u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8d5u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8d7u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8d9u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8dbu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8ddu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8dfu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8e1u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8e3u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8e5u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8e7u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8e9u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x110d8ebu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8edu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8efu32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8f1u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x110d8f3u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8f5u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x110d8f7u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x110d8f9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010369bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10369bd); }
 let mut pc: u32 = 0x10369bd;
 loop { match pc {
 0x10369bd => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4bfeu32, v); }
  { let v: u32 = 0x10c2394u32; cpu.esi = v; }
  { let v: u32 = 0x2fd0u32; cpu.ecx = v; }
  cpu.push(0x10369d6); f_0103903f(cpu); if cpu.returned_to != 0x10369d6 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4bfeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010369e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10369e3); }
 let mut pc: u32 = 0x10369e3;
 loop { match pc {
 0x10369e3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4c06u32, v); }
  { let v: u32 = 0x1194208u32; cpu.ebp = v; }
  { let v: u32 = 0x1124868u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x10369fc; continue;
 }
 0x10369fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1036a21; continue; }
  pc = 0x1036a03; continue;
 }
 0x1036a03 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x2u32); }
  if cpu.zf { pc = 0x1036a12; continue; }
  pc = 0x1036a0e; continue;
 }
 0x1036a0e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1036a15; continue;
 }
 0x1036a12 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1036a15; continue;
 }
 0x1036a15 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10369fc; continue;
 }
 0x1036a21 => {
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x1036a3c; continue; }
  pc = 0x1036a26; continue;
 }
 0x1036a26 => {
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  cpu.push(0x1036a30); f_0103903f(cpu); if cpu.returned_to != 0x1036a30 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4c06u32, v); }
  pc = 0x1036a3c; continue;
 }
 0x1036a3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036a3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036a3d); }
 let mut pc: u32 = 0x1036a3d;
 loop { match pc {
 0x1036a3d => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.esi = v; }
  { let v: u32 = 0x61a8u32; cpu.ecx = v; }
  cpu.push(0x1036a4d); f_0103903f(cpu); if cpu.returned_to != 0x1036a4d { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4c02u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036a5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036a5c); }
 let mut pc: u32 = 0x1036a5c;
 loop { match pc {
 0x1036a5c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1036a74; continue; }
  pc = 0x1036a65; continue;
 }
 0x1036a65 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1124ec0u32, v); }
  cpu.push(0x1036a74); f_010387ec(cpu); if cpu.returned_to != 0x1036a74 { return; }
  pc = 0x1036a74; continue;
 }
 0x1036a74 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036a75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036a75); }
 let mut pc: u32 = 0x1036a75;
 loop { match pc {
 0x1036a75 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5abbu32, v); }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0x5u8); }
  if !cpu.zf { pc = 0x1036a97; continue; }
  pc = 0x1036a85; continue;
 }
 0x1036a85 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.zf { pc = 0x1036b88; continue; }
  pc = 0x1036a97; continue;
 }
 0x1036a97 => {
  { cpu.sub8(cpu.r8(0x1124e88u32), 0x0u8); }
  if !cpu.zf { pc = 0x1036bd3; continue; }
  pc = 0x1036aa4; continue;
 }
 0x1036aa4 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1036b8d; continue; }
  pc = 0x1036ab1; continue;
 }
 0x1036ab1 => {
  { cpu.sub8(cpu.r8(0x10ef20du32), 0x2u8); }
  if !cpu.zf { pc = 0x1036b8d; continue; }
  pc = 0x1036abe; continue;
 }
 0x1036abe => {
  { cpu.sub8(cpu.r8(0x1124e87u32), 0x0u8); }
  if !cpu.zf { pc = 0x1036bc5; continue; }
  pc = 0x1036acb; continue;
 }
 0x1036acb => {
  { let v: u32 = 0xecu32; cpu.eax = v; }
  { let v = cpu.r8(0x10ee91du32); cpu.mul8(v); }
  { let v: u32 = 0x10fa7d8u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1036aeb); f_0103799c(cpu); if cpu.returned_to != 0x1036aeb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x1036af3; continue; }
  pc = 0x1036aee; continue;
 }
 0x1036aee => {
  pc = 0x1036bc5; continue;
 }
 0x1036af3 => {
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x1124e2eu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbbu8); }
  if (cpu.cf || cpu.zf) { pc = 0x1036b08; continue; }
  pc = 0x1036b06; continue;
 }
 0x1036b06 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1036b08; continue;
 }
 0x1036b08 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x81u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ef210u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0xffu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1036b32; continue; }
  pc = 0x1036b2d; continue;
 }
 0x1036b2d => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1036b32; continue;
 }
 0x1036b32 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0xfeu8); }
  if !cpu.zf { pc = 0x1036b6f; continue; }
  pc = 0x1036b3f; continue;
 }
 0x1036b3f => {
  { let v: u32 = cpu.r32(0x10e3e15u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e3e19u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e3e1du32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e3e21u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e3e25u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e3e29u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1036b6f; continue;
 }
 0x1036b6f => {
  { let v: u32 = cpu.r32(0x10e3e0fu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1036b87); f_01036bf2(cpu); if cpu.returned_to != 0x1036b87 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1036b88 => {
  cpu.push(0x1036b8d); f_01065882(cpu); if cpu.returned_to != 0x1036b8d { return; }
  pc = 0x1036b8d; continue;
 }
 0x1036b8d => {
  { cpu.sub8(cpu.r8(0x10e3f39u32), 0x0u8); }
  if cpu.zf { pc = 0x1036bc0; continue; }
  pc = 0x1036b96; continue;
 }
 0x1036b96 => {
  { cpu.sub8(cpu.r8(0x11336c4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1036bd3; continue; }
  pc = 0x1036b9f; continue;
 }
 0x1036b9f => {
  { cpu.sub8(cpu.r8(0x10ef210u32), 0xffu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1036bc0; continue; }
  pc = 0x1036ba8; continue;
 }
 0x1036ba8 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1036bc0; continue; }
  pc = 0x1036bb1; continue;
 }
 0x1036bb1 => {
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1036bcd; continue; }
  pc = 0x1036bc0; continue;
 }
 0x1036bc0 => {
  cpu.push(0x1036bc5); f_01036e55(cpu); if cpu.returned_to != 0x1036bc5 { return; }
  pc = 0x1036bc5; continue;
 }
 0x1036bc5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5abbu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1036bcd => {
  cpu.push(0x1036bd2); f_010865f4(cpu); if cpu.returned_to != 0x1036bd2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1036bd3 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x1036bec); f_01086779(cpu); if cpu.returned_to != 0x1036bec { return; }
  cpu.push(0x1036bf1); f_01036e55(cpu); if cpu.returned_to != 0x1036bf1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036bf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036bf2); }
 let mut pc: u32 = 0x1036bf2;
 loop { match pc {
 0x1036bf2 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x1124cb1u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xbu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1du32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0xfeu8); }
  if !cpu.zf { pc = 0x1036c4a; continue; }
  pc = 0x1036c47; continue;
 }
 0x1036c47 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x18u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1036c4a; continue;
 }
 0x1036c4a => {
  { let v: u32 = 0x1124cb4u32; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1036c58); f_0103903f(cpu); if cpu.returned_to != 0x1036c58 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1036c62); f_01036c63(cpu); if cpu.returned_to != 0x1036c62 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036c63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036c63); }
 let mut pc: u32 = 0x1036c63;
 loop { match pc {
 0x1036c63 => {
  { let v: u32 = 0x1124cb1u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = 0x7fu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1036c75; continue;
 }
 0x1036c75 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1036c79; continue;
 }
 0x1036c79 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1036c96; continue; }
  pc = 0x1036c8c; continue;
 }
 0x1036c8c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1036c75; continue; }
  pc = 0x1036c94; continue;
 }
 0x1036c94 => {
  pc = 0x1036c99; continue;
 }
 0x1036c96 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1036c79; continue; }
  pc = 0x1036c99; continue;
 }
 0x1036c99 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x1124cb1u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3eb7u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036cb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036cb2); }
 let mut pc: u32 = 0x1036cb2;
 loop { match pc {
 0x1036cb2 => {
  { let v: u32 = cpu.r32(0x10e3ff8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u8 = 0x80u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1036cc6; continue;
 }
 0x1036cc6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1036ccd; continue;
 }
 0x1036ccd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), (cpu.ecx & 0xff)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1036ce3; continue; }
  pc = 0x1036cde; continue;
 }
 0x1036cde => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1036cc6; continue; }
  pc = 0x1036ce1; continue;
 }
 0x1036ce1 => {
  pc = 0x1036ce6; continue;
 }
 0x1036ce3 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1036ccd; continue; }
  pc = 0x1036ce6; continue;
 }
 0x1036ce6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036ce7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036ce7); }
 let mut pc: u32 = 0x1036ce7;
 loop { match pc {
 0x1036ce7 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1036cf1; continue; }
  pc = 0x1036cf0; continue;
 }
 0x1036cf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1036cf1 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u32 = 0x10fa7d8u32; cpu.esi = v; }
  cpu.push(0x1036d00); f_01036d2e(cpu); if cpu.returned_to != 0x1036d00 { return; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u32 = 0x10fa8c4u32; cpu.esi = v; }
  cpu.push(0x1036d0f); f_01036d2e(cpu); if cpu.returned_to != 0x1036d0f { return; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = 0x10fa9b0u32; cpu.esi = v; }
  cpu.push(0x1036d1e); f_01036d2e(cpu); if cpu.returned_to != 0x1036d1e { return; }
  { let v: u32 = 0x3u32; cpu.edi = v; }
  { let v: u32 = 0x10faa9cu32; cpu.esi = v; }
  cpu.push(0x1036d2d); f_01036d2e(cpu); if cpu.returned_to != 0x1036d2d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01036d2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1036d2e); }
 let mut pc: u32 = 0x1036d2e;
 loop { match pc {
 0x1036d2e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x81u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1036e30; continue; }
  pc = 0x1036d44; continue;
 }
 0x1036d44 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x10ee929u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1036d65; continue; }
  pc = 0x1036d4d; continue;
 }
 0x1036d4d => {
  if (cpu.sf != cpu.of) { pc = 0x1036d51; continue; }
  pc = 0x1036d4f; continue;
 }
 0x1036d4f => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1036d51; continue;
 }
 0x1036d51 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x10ee929u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10fd446u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if (cpu.sf != cpu.of) { pc = 0x1036d65; continue; }
  pc = 0x1036d65; continue;
 }
 0x1036d65 => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee929u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1036e30; continue; }
  pc = 0x1036d74; continue;
 }
 0x1036d74 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef33cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33bu32, v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1036d93; continue; }
  pc = 0x1036d8c; continue;
 }
 0x1036d8c => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef20fu32, v); }
  pc = 0x1036d93; continue;
 }
 0x1036d93 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0xffu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1036dd7; continue; }
  pc = 0x1036dad; continue;
 }
 0x1036dad => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1036e30; continue; }
  pc = 0x1036dc1; continue;
 }
 0x1036dc1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1036e30; continue; }
  pc = 0x1036dcb; continue;
 }
 0x1036dcb => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  pc = 0x1036dd7; continue;
 }
 0x1036dd7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4048u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0xfeu8); }
  if !cpu.zf { pc = 0x1036e27; continue; }
  pc = 0x1036df7; continue;
 }
 0x1036df7 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e15u32, v); }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e19u32, v); }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e1du32, v); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e21u32, v); }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e25u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e29u32, v); }
  pc = 0x1036e27; continue;
 }
 0x1036e27 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1036e2d); f_01036e55(cpu); if cpu.returned_to != 0x1036e2d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1036e30; continue;
 }
 0x1036e30 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x81u8); }
  if !cpu.zf { pc = 0x1036e42; continue; }
  pc = 0x1036e3c; continue;
 }
 0x1036e3c => {
  { let v: u32 = cpu.esi.wrapping_add(0x7d6u32); cpu.esi = v; }
  pc = 0x1036e42; continue;
 }
 0x1036e42 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffdeu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.eax as u8), 0x82u8); }
  if !cpu.zf { pc = 0x1036e50; continue; }
  pc = 0x1036e4a; continue;
 }
 0x1036e4a => {
  { let v: u32 = cpu.add32(cpu.esi, 0x7d6u32); cpu.esi = v; }
  pc = 0x1036e50; continue;
 }
 0x1036e50 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffbcu32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010372f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10372f8); }
 let mut pc: u32 = 0x10372f8;
 loop { match pc {
 0x10372f8 => {
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f43u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f45u32, v); }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef20au32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e4048u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10eea7bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef37bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef209u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x1037348); f_0103fb02(cpu); if cpu.returned_to != 0x1037348 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112635cu32); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11296e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37bu32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef37du32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1037382; continue; }
  pc = 0x103737b; continue;
 }
 0x103737b => {
  { let v: u8 = 0x0u8; cpu.w8(0x112529fu32, v); }
  pc = 0x1037382; continue;
 }
 0x1037382 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037383(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037383); }
 let mut pc: u32 = 0x1037383;
 loop { match pc {
 0x1037383 => {
  { let v: u32 = cpu.r16(0x10e3e0fu32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x116c0d8u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x8u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10373cf; continue; }
  pc = 0x10373a3; continue;
 }
 0x10373a3 => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef37du32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11de584u32.wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110eac4u32.wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  pc = 0x10373cf; continue;
 }
 0x10373cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010373d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10373d0); }
 let mut pc: u32 = 0x10373d0;
 loop { match pc {
 0x10373d0 => {
  cpu.push(0x10373d5); f_01037546(cpu); if cpu.returned_to != 0x10373d5 { return; }
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x103749a; continue; }
  pc = 0x10373e2; continue;
 }
 0x10373e2 => {
  { let v: u32 = cpu.r32(0x1124ec4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4078u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4088u32); cpu.ebx = v; }
  { cpu.interrupts_off = true; }
  { let v: u32 = cpu.r32(0x1124eacu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x11dc39cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1124ec4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x11dc39cu32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1037498; continue; }
  pc = 0x1037416; continue;
 }
 0x1037416 => {
  if !cpu.sf { pc = 0x103741e; continue; }
  pc = 0x1037418; continue;
 }
 0x1037418 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x400u32); cpu.ecx = v; }
  pc = 0x103741e; continue;
 }
 0x103741e => {
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124ec4u32, v); }
  { cpu.interrupts_off = false; }
  pc = 0x103742a; continue;
 }
 0x103742a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x103743a; continue; }
  pc = 0x1037431; continue;
 }
 0x1037431 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef327u32, v); }
  pc = 0x1037476; continue;
 }
 0x103743a => {
  { cpu.sub8((cpu.eax as u8), 0xfeu8); }
  if !cpu.zf { pc = 0x1037440; continue; }
  pc = 0x103743e; continue;
 }
 0x103743e => {
  pc = 0x1037476; continue;
 }
 0x1037440 => {
  { cpu.sub8((cpu.eax as u8), 0xfdu8); }
  if !cpu.zf { pc = 0x1037446; continue; }
  pc = 0x1037444; continue;
 }
 0x1037444 => {
  pc = 0x1037476; continue;
 }
 0x1037446 => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x1037461; continue; }
  pc = 0x103744a; continue;
 }
 0x103744a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11dc81cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x100u32); }
  if cpu.cf { pc = 0x1037476; continue; }
  pc = 0x1037459; continue;
 }
 0x1037459 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100u32); cpu.edi = v; }
  pc = 0x1037476; continue;
 }
 0x1037461 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11dc79cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x1037476; continue; }
  pc = 0x1037470; continue;
 }
 0x1037470 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x80u32); cpu.ebx = v; }
  pc = 0x1037476; continue;
 }
 0x1037476 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x11dc39cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x3ffu32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x11dc39cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103742a; continue; }
  pc = 0x103748b; continue;
 }
 0x103748b => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4078u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4088u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037498 => {
  { cpu.interrupts_off = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103749a => {
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  cpu.push(0x10374a4); f_010375da(cpu); if cpu.returned_to != 0x10374a4 { return; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  cpu.push(0x10374ae); f_010375da(cpu); if cpu.returned_to != 0x10374ae { return; }
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  cpu.push(0x10374b8); f_010375da(cpu); if cpu.returned_to != 0x10374b8 { return; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x10374c2); f_010375da(cpu); if cpu.returned_to != 0x10374c2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010374c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10374c3); }
 let mut pc: u32 = 0x10374c3;
 loop { match pc {
 0x10374c3 => {
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x10374f9; continue; }
  pc = 0x10374c8; continue;
 }
 0x10374c8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1124eacu32); cpu.edi = v; }
  { let v: u32 = 0x11dc39cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  pc = 0x10374de; continue;
 }
 0x10374de => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3ffu32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10374de; continue; }
  pc = 0x10374f2; continue;
 }
 0x10374f2 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1124eacu32, v); }
  { cpu.popad(); }
  pc = 0x10374f9; continue;
 }
 0x10374f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010374fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10374fa); }
 let mut pc: u32 = 0x10374fa;
 loop { match pc {
 0x10374fa => {
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x1037545; continue; }
  pc = 0x10374ff; continue;
 }
 0x10374ff => {
  { cpu.sub32(cpu.edx, 0x3cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1037505; continue; }
  pc = 0x1037504; continue;
 }
 0x1037504 => {
  {  }
  pc = 0x1037505; continue;
 }
 0x1037505 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u32 = 0x10b0332u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10b07b2u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  pc = 0x103751e; continue;
 }
 0x103751e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3ffu32; cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10b07b4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1037544; continue; }
  pc = 0x1037534; continue;
 }
 0x1037534 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103751e; continue; }
  pc = 0x1037538; continue;
 }
 0x1037538 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10b07b2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1037544); f_01037dd8(cpu); if cpu.returned_to != 0x1037544 { return; }
  pc = 0x1037544; continue;
 }
 0x1037544 => {
  { cpu.popad(); }
  pc = 0x1037545; continue;
 }
 0x1037545 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037546(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037546); }
 let mut pc: u32 = 0x1037546;
 loop { match pc {
 0x1037546 => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if cpu.cf { pc = 0x1037560; continue; }
  pc = 0x103754f; continue;
 }
 0x103754f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1037559; continue; }
  pc = 0x1037558; continue;
 }
 0x1037558 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037559 => {
  cpu.push(0x103755e); f_01061f55(cpu); if cpu.returned_to != 0x103755e { return; }
  pc = 0x1037575; continue;
 }
 0x1037560 => {
  { cpu.pushad(); }
  cpu.push(0x1037566); f_01037576(cpu); if cpu.returned_to != 0x1037566 { return; }
  { cpu.sub8(cpu.r8(0x11dde24u32), 0x0u8); }
  if cpu.zf { pc = 0x1037574; continue; }
  pc = 0x103756f; continue;
 }
 0x103756f => {
  cpu.push(0x1037574); f_01085436(cpu); if cpu.returned_to != 0x1037574 { return; }
  pc = 0x1037574; continue;
 }
 0x1037574 => {
  { cpu.popad(); }
  pc = 0x1037575; continue;
 }
 0x1037575 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037576(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037576); }
 let mut pc: u32 = 0x1037576;
 loop { match pc {
 0x1037576 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x10b0c3cu32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  pc = 0x1037584; continue;
 }
 0x1037584 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10b0c3au32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10375d1; continue; }
  pc = 0x103758d; continue;
 }
 0x103758d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10b07bau32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11dde24u32), 0x0u8); }
  if cpu.zf { pc = 0x10375aa; continue; }
  pc = 0x103759f; continue;
 }
 0x103759f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10375a6); f_01085a69(cpu); if cpu.returned_to != 0x10375a6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x10375c8; continue;
 }
 0x10375aa => {
  { let v: u32 = cpu.r32(0x1124eacu32); cpu.edi = v; }
  { let v: u32 = 0x11dc39cu32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3ffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1124eacu32, v); }
  pc = 0x10375c8; continue;
 }
 0x10375c8 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  cpu.poll(); pc = 0x1037584; continue;
 }
 0x10375d1 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10b0c3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010375da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10375da); }
 let mut pc: u32 = 0x10375da;
 loop { match pc {
 0x10375da => {
  { let v: u32 = cpu.r32(0x1124ec4u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4078u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4088u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { cpu.interrupts_off = true; }
  { let v: u32 = cpu.r32(0x1124eacu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x11dc36cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1124ec4u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x11dc36cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10376a6; continue; }
  pc = 0x1037616; continue;
 }
 0x1037616 => {
  if !cpu.sf { pc = 0x103761e; continue; }
  pc = 0x1037618; continue;
 }
 0x1037618 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x400u32); cpu.ecx = v; }
  pc = 0x103761e; continue;
 }
 0x103761e => {
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124ec4u32.wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { cpu.interrupts_off = false; }
  pc = 0x103762c; continue;
 }
 0x103762c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x103763c; continue; }
  pc = 0x1037633; continue;
 }
 0x1037633 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef327u32, v); }
  pc = 0x1037680; continue;
 }
 0x103763c => {
  { cpu.sub8((cpu.eax as u8), 0xfeu8); }
  if !cpu.zf { pc = 0x1037642; continue; }
  pc = 0x1037640; continue;
 }
 0x1037640 => {
  pc = 0x1037680; continue;
 }
 0x1037642 => {
  { cpu.sub8((cpu.eax as u8), 0xfdu8); }
  if !cpu.zf { pc = 0x1037648; continue; }
  pc = 0x1037646; continue;
 }
 0x1037646 => {
  pc = 0x1037680; continue;
 }
 0x1037648 => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x1037667; continue; }
  pc = 0x103764c; continue;
 }
 0x103764c => {
  { let v: u32 = cpu.r32(0x11dc38cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x100u32); }
  if cpu.cf { pc = 0x1037680; continue; }
  pc = 0x103765f; continue;
 }
 0x103765f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100u32); cpu.edi = v; }
  pc = 0x1037680; continue;
 }
 0x1037667 => {
  { let v: u32 = cpu.r32(0x11dc37cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x1037680; continue; }
  pc = 0x103767a; continue;
 }
 0x103767a => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x80u32); cpu.ebx = v; }
  pc = 0x1037680; continue;
 }
 0x1037680 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x11dc36cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x3ffu32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x11dc36cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103762c; continue; }
  pc = 0x1037697; continue;
 }
 0x1037697 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4078u32.wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4088u32.wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10376a6 => {
  { cpu.interrupts_off = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010376a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10376a8); }
 let mut pc: u32 = 0x10376a8;
 loop { match pc {
 0x10376a8 => {
  cpu.push(0x10376ad); f_0103775b(cpu); if cpu.returned_to != 0x10376ad { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124d1fu32, v); }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x10376c0; continue; }
  pc = 0x10376be; continue;
 }
 0x10376be => {
  { let v: u8 = 0xbbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10376c0; continue;
 }
 0x10376c0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee929u32, v); }
  cpu.push(0x10376ca); f_0103775b(cpu); if cpu.returned_to != 0x10376ca { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124d20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee929u32, v); }
  { let v: u32 = 0x1124d1fu32; cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x10376e4); f_01037727(cpu); if cpu.returned_to != 0x10376e4 { return; }
  cpu.push(0x10376e9); f_01037d64(cpu); if cpu.returned_to != 0x10376e9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010376ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10376ea); }
 let mut pc: u32 = 0x10376ea;
 loop { match pc {
 0x10376ea => {
  cpu.push(0x10376ef); f_0103775b(cpu); if cpu.returned_to != 0x10376ef { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124d1fu32, v); }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x1037702; continue; }
  pc = 0x1037700; continue;
 }
 0x1037700 => {
  { let v: u8 = 0xbbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1037702; continue;
 }
 0x1037702 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee929u32, v); }
  cpu.push(0x103770c); f_0103775b(cpu); if cpu.returned_to != 0x103770c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124d20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee929u32, v); }
  { let v: u32 = 0x1124d1fu32; cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x1037726); f_01037d64(cpu); if cpu.returned_to != 0x1037726 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037727(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037727); }
 let mut pc: u32 = 0x1037727;
 loop { match pc {
 0x1037727 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1124e87u32), 0x0u8); }
  if cpu.zf { pc = 0x103775a; continue; }
  pc = 0x1037732; continue;
 }
 0x1037732 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x1124ca3u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3eb7u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x7u32); cpu.ecx = v; }
  { let v: u32 = 0x1124d21u32; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.interrupts_off = true; }
  { cpu.movs(1, true); }
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x1124d1fu32; cpu.esi = v; }
  pc = 0x103775a; continue;
 }
 0x103775a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103775b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103775b); }
 let mut pc: u32 = 0x103775b;
 loop { match pc {
 0x103775b => {
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1124dc6u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037770); }
 let mut pc: u32 = 0x1037770;
 loop { match pc {
 0x1037770 => {
  { let v: u32 = 0x1124e46u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103777f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103777f); }
 let mut pc: u32 = 0x103777f;
 loop { match pc {
 0x103777f => {
  { let v: u8 = cpu.r8(0x10e3cdfu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124dc6u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037793(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037793); }
 let mut pc: u32 = 0x1037793;
 loop { match pc {
 0x1037793 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103779d; continue; }
  pc = 0x103779c; continue;
 }
 0x103779c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103779d => {
  pc = 0x103779f; continue;
 }
 0x103779f => {
  { cpu.sub8(cpu.r8(0x10ef33du32), 0x0u8); }
  if !cpu.zf { pc = 0x10377ad; continue; }
  pc = 0x10377a8; continue;
 }
 0x10377a8 => {
  cpu.push(0x10377ad); f_010373d0(cpu); if cpu.returned_to != 0x10377ad { return; }
  pc = 0x10377ad; continue;
 }
 0x10377ad => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10377db; continue; }
  pc = 0x10377b6; continue;
 }
 0x10377b6 => {
  { let v: u32 = 0x11dc81cu32; cpu.esi = v; }
  { let v: u32 = 0x10fa8c4u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x10377d3; continue; }
  pc = 0x10377c9; continue;
 }
 0x10377c9 => {
  { let v: u32 = 0x10fa7d8u32; cpu.edi = v; }
  { let v: u32 = 0x11dc81cu32; cpu.esi = v; }
  pc = 0x10377d3; continue;
 }
 0x10377d3 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10377da); f_01037820(cpu); if cpu.returned_to != 0x10377da { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10377db => {
  { let v: u32 = 0x11dc81cu32; cpu.esi = v; }
  { let v: u32 = 0x10fa7d8u32; cpu.edi = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10377ec); f_01037820(cpu); if cpu.returned_to != 0x10377ec { return; }
  { let v: u32 = 0x11dcd9cu32; cpu.esi = v; }
  { let v: u32 = 0x10fa8c4u32; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10377fd); f_01037820(cpu); if cpu.returned_to != 0x10377fd { return; }
  { let v: u32 = 0x11dd31cu32; cpu.esi = v; }
  { let v: u32 = 0x10fa9b0u32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x103780e); f_01037820(cpu); if cpu.returned_to != 0x103780e { return; }
  { let v: u32 = 0x11dd89cu32; cpu.esi = v; }
  { let v: u32 = 0x10faa9cu32; cpu.edi = v; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x103781f); f_01037820(cpu); if cpu.returned_to != 0x103781f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037820(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1037820); } r_01037820(cpu, 0x1037820); }
pub fn f_010385d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10385d7); } r_01037820(cpu, 0x10385d7); }
fn r_01037820(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1037820 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33bu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1124ed4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1124ed8u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124ee5u32, v); }
  pc = 0x1037838; continue;
 }
 0x1037838 => {
  { let v: u32 = 0x1124ca3u32; cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  pc = 0x1037842; continue;
 }
 0x1037842 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1124ed4u32); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1037856); f_01037d10(cpu); if cpu.returned_to != 0x1037856 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x1037864; continue; }
  pc = 0x103785a; continue;
 }
 0x103785a => {
  { let v: u32 = cpu.esi.wrapping_add(0x5u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1037842; continue; }
  pc = 0x103785f; continue;
 }
 0x103785f => {
  pc = 0x10378ee; continue;
 }
 0x1037864 => {
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x10378a1; continue; }
  pc = 0x1037870; continue;
 }
 0x1037870 => {
  { let v: u8 = 0x3u8; cpu.w8(0x10ef33bu32, v); }
  { let v = cpu.r32(0x10e3ff8u32); cpu.push(v); }
  let t = cpu.r32(0x1124da0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1037884); cpu.call(t); if cpu.returned_to != 0x1037884 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3ff8u32, v); }
  { let v: u32 = cpu.r32(0x10e3ff8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x3u32); cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x5u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.poll(); pc = 0x1037838; continue;
 }
 0x10378a1 => {
  cpu.push(0x10378a6); f_01036cb2(cpu); if cpu.returned_to != 0x10378a6 { return; }
  { let v: u32 = cpu.r32(0x10e3ff8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.lods(2, false); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11de2b1u32; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x10378bd; continue;
 }
 0x10378bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10378bd; continue; }
  pc = 0x10378c6; continue;
 }
 0x10378c6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x11de2b1u32; cpu.esi = v; }
  cpu.push(0x10378d1); f_0103903f(cpu); if cpu.returned_to != 0x10378d1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x103860f; continue; }
  pc = 0x10378dd; continue;
 }
 0x10378dd => {
  { let v: u32 = cpu.r32(0x1124ed8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x81u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.movs(1, true); }
  pc = 0x10378ee; continue;
 }
 0x10378ee => {
  { let v: u32 = cpu.r32(0x1124ed4u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r8(0x1124ee5u32) as u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4078u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = cpu.r8(0x11de2b1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x11de2b1u32, v); }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x103795f; continue; }
  pc = 0x1037925; continue;
 }
 0x1037925 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xbcu8); }
  if !cpu.zf { pc = 0x103792e; continue; }
  pc = 0x103792c; continue;
 }
 0x103792c => {
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103792e; continue;
 }
 0x103792e => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x103795f; continue; }
  pc = 0x1037932; continue;
 }
 0x1037932 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xbcu8); }
  if !cpu.zf { pc = 0x103793b; continue; }
  pc = 0x1037939; continue;
 }
 0x1037939 => {
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103793b; continue;
 }
 0x103793b => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x103795f; continue; }
  pc = 0x103793f; continue;
 }
 0x103793f => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x1037948; continue; }
  pc = 0x1037946; continue;
 }
 0x1037946 => {
  { let v: u8 = 0xbbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1037948; continue;
 }
 0x1037948 => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x1037951; continue; }
  pc = 0x103794f; continue;
 }
 0x103794f => {
  { let v: u8 = 0xbbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1037951; continue;
 }
 0x1037951 => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x103795a; continue; }
  pc = 0x1037958; continue;
 }
 0x1037958 => {
  { let v: u8 = 0xbbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103795a; continue;
 }
 0x103795a => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x103795f; continue; }
  pc = 0x103795e; continue;
 }
 0x103795e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103795f => {
  { let v: u8 = 0x4u8; cpu.w8(0x10ef33bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10385d7 => {
  { let v: u32 = 0x8cu32; cpu.ecx = v; }
  pc = 0x10385dc; continue;
 }
 0x10385dc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x103860f; continue; }
  pc = 0x10385df; continue;
 }
 0x10385df => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10385e6); f_01045e5f(cpu); if cpu.returned_to != 0x10385e6 { return; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.pushad(); }
  cpu.push(0x10385ee); f_01042066(cpu); if cpu.returned_to != 0x10385ee { return; }
  cpu.push(0x10385f3); f_01059ae3(cpu); if cpu.returned_to != 0x10385f3 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x10fd3e8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1038611; continue; }
  pc = 0x10385fd; continue;
 }
 0x10385fd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1038608); cpu.call(t); if cpu.returned_to != 0x1038608 { return; }
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x10385dc; continue; }
  pc = 0x103860e; continue;
 }
 0x103860e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103860f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038611 => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd3eau32, v); }
  { cpu.pushad(); }
  cpu.push(0x1038622); f_01042066(cpu); if cpu.returned_to != 0x1038622 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x103862e); cpu.call(t); if cpu.returned_to != 0x103862e { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd3eau32, v); }
  { cpu.pushad(); }
  cpu.push(0x103863b); f_01042066(cpu); if cpu.returned_to != 0x103863b { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x103860f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037967); }
 let mut pc: u32 = 0x1037967;
 loop { match pc {
 0x1037967 => {
  { let v: u32 = cpu.r32(0x1124ed4u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r8(0x1124ee5u32) as u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4078u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = cpu.r8(0x11de2b1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x11de2b1u32, v); }
  { let v: u8 = 0x2au8; cpu.w8(0x10ef33bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103799c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103799c); }
 let mut pc: u32 = 0x103799c;
 loop { match pc {
 0x103799c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10379b9; continue; }
  pc = 0x10379a5; continue;
 }
 0x10379a5 => {
  { cpu.sub8(cpu.r8(0x10ef20fu32), 0xffu8); }
  if !cpu.zf { pc = 0x10379bb; continue; }
  pc = 0x10379ae; continue;
 }
 0x10379ae => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef20fu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e3e09u32, v); }
  pc = 0x10379b9; continue;
 }
 0x10379b9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10379bb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010379bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10379bd); }
 let mut pc: u32 = 0x10379bd;
 loop { match pc {
 0x10379bd => {
  { cpu.sub8(cpu.r8(0x10ef20fu32), 0xffu8); }
  if cpu.zf { pc = 0x10379f7; continue; }
  pc = 0x10379c6; continue;
 }
 0x10379c6 => {
  { cpu.sub8(cpu.r8(0x10ef20fu32), 0xfeu8); }
  if cpu.zf { pc = 0x10379f7; continue; }
  pc = 0x10379cf; continue;
 }
 0x10379cf => {
  { let v: u8 = cpu.r8(0x10ef20fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10379d9); f_010379f8(cpu); if cpu.returned_to != 0x10379d9 { return; }
  { let v: u8 = 0xfeu8; cpu.w8(0x10ef20fu32, v); }
  { cpu.sub16(cpu.r16(0x10e3e0bu32), 0xffffu16); }
  if cpu.zf { pc = 0x10379f7; continue; }
  pc = 0x10379ea; continue;
 }
 0x10379ea => {
  let t = cpu.r32(0x10e3e09u32); cpu.push(0x10379f0); cpu.call(t); if cpu.returned_to != 0x10379f0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef327u32, v); }
  pc = 0x10379f7; continue;
 }
 0x10379f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010379f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10379f8); }
 let mut pc: u32 = 0x10379f8;
 loop { match pc {
 0x10379f8 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1037a63; continue; }
  pc = 0x1037a01; continue;
 }
 0x1037a01 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x1124ca3u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1037a2a; continue; }
  pc = 0x1037a11; continue;
 }
 0x1037a11 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1037a1c; continue; }
  pc = 0x1037a15; continue;
 }
 0x1037a15 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x1037a34; continue;
 }
 0x1037a1c => {
  { let v: u32 = 0x1105933u32; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = 0x1037a34; continue;
 }
 0x1037a2a => {
  { let v: u32 = cpu.r16(0x10e3eb7u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  pc = 0x1037a34; continue;
 }
 0x1037a34 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x1037a38; continue;
 }
 0x1037a38 => {
  cpu.push(0x1037a3d); f_01037d64(cpu); if cpu.returned_to != 0x1037a3d { return; }
  if cpu.cf { cpu.poll(); pc = 0x1037a38; continue; }
  pc = 0x1037a3f; continue;
 }
 0x1037a3f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e87u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1037a63; continue; }
  pc = 0x1037a5c; continue;
 }
 0x1037a5c => {
  { let v: u8 = 0x0u8; cpu.w8(0x1124e87u32, v); }
  pc = 0x1037a63; continue;
 }
 0x1037a63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037a64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037a64); }
 let mut pc: u32 = 0x1037a64;
 loop { match pc {
 0x1037a64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037a70); }
 let mut pc: u32 = 0x1037a70;
 loop { match pc {
 0x1037a70 => {
  cpu.push(0x1037a75); f_01038abf(cpu); if cpu.returned_to != 0x1037a75 { return; }
  cpu.push(0x1037a7a); f_01038f34(cpu); if cpu.returned_to != 0x1037a7a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037a7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037a7b); }
 let mut pc: u32 = 0x1037a7b;
 loop { match pc {
 0x1037a7b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1037a8d); f_010385ad(cpu); if cpu.returned_to != 0x1037a8d { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1037a95); cpu.call(t); if cpu.returned_to != 0x1037a95 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u16 = 0xf0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4318u32); cpu.push(0x1037ab7); cpu.call(t); if cpu.returned_to != 0x1037ab7 { return; }
  { let v: u32 = 0x110b77cu32; cpu.esi = v; }
  { let v: u32 = 0x5au32; cpu.ebx = v; }
  cpu.push(0x1037ac6); f_0104bb49(cpu); if cpu.returned_to != 0x1037ac6 { return; }
  cpu.push(0x1037acb); f_0104b988(cpu); if cpu.returned_to != 0x1037acb { return; }
  { let v: u32 = 0x110b7a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  cpu.push(0x1037ada); f_0104bb49(cpu); if cpu.returned_to != 0x1037ada { return; }
  cpu.push(0x1037adf); f_0104b988(cpu); if cpu.returned_to != 0x1037adf { return; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1037ae9); cpu.call(t); if cpu.returned_to != 0x1037ae9 { return; }
  cpu.push(0x1037aee); f_0107162e(cpu); if cpu.returned_to != 0x1037aee { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037ba3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037ba3); }
 let mut pc: u32 = 0x1037ba3;
 loop { match pc {
 0x1037ba3 => {
  cpu.push(0x1037ba8); f_01037546(cpu); if cpu.returned_to != 0x1037ba8 { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  cpu.push(0x1037bb9); f_01037bbd(cpu); if cpu.returned_to != 0x1037bb9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037bbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037bbd); }
 let mut pc: u32 = 0x1037bbd;
 loop { match pc {
 0x1037bbd => {
  { let v: u32 = cpu.ecx.wrapping_add(0xau32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124ee4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd546u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd54au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd54eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd552u32, v); }
  cpu.push(0x1037bf4); f_01037546(cpu); if cpu.returned_to != 0x1037bf4 { return; }
  pc = 0x1037c42; continue;
 }
 0x1037c42 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x11dc39cu32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124ee4u32, v); }
  { cpu.sub8(cpu.r8(0x11c5b26u32), 0x0u8); }
  if cpu.zf { pc = 0x1037c79; continue; }
  pc = 0x1037c58; continue;
 }
 0x1037c58 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.edi, 0x11dc39cu32); cpu.edi = v; }
  cpu.push(0x1037c64); f_01037bf6(cpu); if cpu.returned_to != 0x1037c64 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1037c79; continue; }
  pc = 0x1037c67; continue;
 }
 0x1037c67 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1124ee4u32)); cpu.w8(0x1124ee4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10fd556u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd546u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1037c79; continue;
 }
 0x1037c79 => {
  { cpu.sub8(cpu.r8(0x11c5b27u32), 0x0u8); }
  if cpu.zf { pc = 0x1037ca3; continue; }
  pc = 0x1037c82; continue;
 }
 0x1037c82 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.edi, 0x11dc91cu32); cpu.edi = v; }
  cpu.push(0x1037c8e); f_01037bf6(cpu); if cpu.returned_to != 0x1037c8e { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1037ca3; continue; }
  pc = 0x1037c91; continue;
 }
 0x1037c91 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1124ee4u32)); cpu.w8(0x1124ee4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10fd556u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd54au32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1037ca3; continue;
 }
 0x1037ca3 => {
  { cpu.sub8(cpu.r8(0x11c5b28u32), 0x0u8); }
  if cpu.zf { pc = 0x1037ccd; continue; }
  pc = 0x1037cac; continue;
 }
 0x1037cac => {
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.edi, 0x11dce9cu32); cpu.edi = v; }
  cpu.push(0x1037cb8); f_01037bf6(cpu); if cpu.returned_to != 0x1037cb8 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1037ccd; continue; }
  pc = 0x1037cbb; continue;
 }
 0x1037cbb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1124ee4u32)); cpu.w8(0x1124ee4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10fd556u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd54eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1037ccd; continue;
 }
 0x1037ccd => {
  { cpu.sub8(cpu.r8(0x11c5b29u32), 0x0u8); }
  if cpu.zf { pc = 0x1037cf7; continue; }
  pc = 0x1037cd6; continue;
 }
 0x1037cd6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.edi, 0x11dd41cu32); cpu.edi = v; }
  cpu.push(0x1037ce2); f_01037bf6(cpu); if cpu.returned_to != 0x1037ce2 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1037cf7; continue; }
  pc = 0x1037ce5; continue;
 }
 0x1037ce5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1124ee4u32)); cpu.w8(0x1124ee4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10fd556u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd552u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1037cf7; continue;
 }
 0x1037cf7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x11dc39cu32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1124ee4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x11c5b24u32)); }
  if !cpu.zf { pc = 0x1037d0e; continue; }
  pc = 0x1037d0c; continue;
 }
 0x1037d0c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037d0e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037bf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037bf6); }
 let mut pc: u32 = 0x1037bf6;
 loop { match pc {
 0x1037bf6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40f8u32, v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x1037c06; continue;
 }
 0x1037c06 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x1037c0a; continue;
 }
 0x1037c0a => {
  { cpu.cmps(1, 0); }
  if !cpu.zf { pc = 0x1037c2f; continue; }
  pc = 0x1037c0d; continue;
 }
 0x1037c0d => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10e40f8u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi & cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e40f8u32)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x1037c0a; continue; }
  pc = 0x1037c1f; continue;
 }
 0x1037c1f => {
  if !cpu.zf { pc = 0x1037c3f; continue; }
  pc = 0x1037c21; continue;
 }
 0x1037c21 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e3ff8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd556u32, v); }
  { cpu.cf = true; }
  pc = 0x1037c40; continue;
 }
 0x1037c2f => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10e40f8u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi & cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x10e40f8u32)); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1037c06; continue; }
  pc = 0x1037c3f; continue;
 }
 0x1037c3f => {
  { cpu.cf = false; }
  pc = 0x1037c40; continue;
 }
 0x1037c40 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037d10); }
 let mut pc: u32 = 0x1037d10;
 loop { match pc {
 0x1037d10 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x1037d15; continue;
 }
 0x1037d15 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x1037d17; continue;
 }
 0x1037d17 => {
  { cpu.cmps(1, 0); }
  if !cpu.zf { pc = 0x1037d29; continue; }
  pc = 0x1037d1a; continue;
 }
 0x1037d1a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x1037d17; continue; }
  pc = 0x1037d1e; continue;
 }
 0x1037d1e => {
  if !cpu.zf { pc = 0x1037d2b; continue; }
  pc = 0x1037d20; continue;
 }
 0x1037d20 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e3ff8u32, v); }
  { cpu.cf = true; }
  pc = 0x1037d2c; continue;
 }
 0x1037d29 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1037d15; continue; }
  pc = 0x1037d2b; continue;
 }
 0x1037d2b => {
  { cpu.cf = false; }
  pc = 0x1037d2c; continue;
 }
 0x1037d2c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037d2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037d2e); }
 let mut pc: u32 = 0x1037d2e;
 loop { match pc {
 0x1037d2e => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1037d59; continue; }
  pc = 0x1037d37; continue;
 }
 0x1037d37 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  pc = 0x1037d3c; continue;
 }
 0x1037d3c => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1037d58; continue; }
  pc = 0x1037d40; continue;
 }
 0x1037d40 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  pc = 0x1037d45; continue;
 }
 0x1037d45 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1037d45; continue; }
  pc = 0x1037d47; continue;
 }
 0x1037d47 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10b0330u32.wrapping_add(cpu.ebp)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1037d3c; continue; }
  pc = 0x1037d58; continue;
 }
 0x1037d58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037d59 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  cpu.push(0x1037d63); f_010385ad(cpu); if cpu.returned_to != 0x1037d63 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037d64); }
 let mut pc: u32 = 0x1037d64;
 loop { match pc {
 0x1037d64 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1037daa; continue; }
  pc = 0x1037d6d; continue;
 }
 0x1037d6d => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1037dac; continue; }
  pc = 0x1037d76; continue;
 }
 0x1037d76 => {
  { let v: u16 = cpu.r16(0x1124e96u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1037daa; continue; }
  pc = 0x1037d82; continue;
 }
 0x1037d82 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11dde24u32), 0x0u8); }
  if !cpu.zf { pc = 0x1037d99; continue; }
  pc = 0x1037d92; continue;
 }
 0x1037d92 => {
  cpu.push(0x1037d97); f_010374fa(cpu); if cpu.returned_to != 0x1037d97 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037d99 => {
  cpu.push(0x1037d9e); f_010851bd(cpu); if cpu.returned_to != 0x1037d9e { return; }
  { cpu.pushad(); }
  cpu.push(0x1037da4); f_01037576(cpu); if cpu.returned_to != 0x1037da4 { return; }
  cpu.push(0x1037da9); f_01085436(cpu); if cpu.returned_to != 0x1037da9 { return; }
  { cpu.popad(); }
  pc = 0x1037daa; continue;
 }
 0x1037daa => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1037dac => {
  cpu.push(0x1037db1); f_010620c7(cpu); if cpu.returned_to != 0x1037db1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037db4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037db4); }
 let mut pc: u32 = 0x1037db4;
 loop { match pc {
 0x1037db4 => {
  { cpu.interrupts_off = true; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b0330u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb7u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1124e8cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfdu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.interrupts_off = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037dd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037dd8); }
 let mut pc: u32 = 0x1037dd8;
 loop { match pc {
 0x1037dd8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.interrupts_off = true; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10b0330u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5bb7u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1124e8cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.interrupts_off = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037e00); }
 let mut pc: u32 = 0x1037e00;
 loop { match pc {
 0x1037e00 => {
  { cpu.sub8(cpu.r8(0x112c97cu32.wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x1037eb9; continue; }
  pc = 0x1037e0d; continue;
 }
 0x1037e0d => {
  { cpu.sub8(cpu.r8(0x112c97cu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if cpu.zf { pc = 0x1037eb9; continue; }
  pc = 0x1037e1a; continue;
 }
 0x1037e1a => {
  { cpu.sub8(cpu.r8(0x10ee91eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1037e2d; continue; }
  pc = 0x1037e23; continue;
 }
 0x1037e23 => {
  cpu.flat_ds(0x1037e23); cpu.push(0x1037e28); f_01038580(cpu); if cpu.returned_to != 0x1037e28 { return; }
  cpu.flat_ds(0x1037e28); cpu.push(0x1037e2d); f_010385c4(cpu); if cpu.returned_to != 0x1037e2d { return; }
  pc = 0x1037e2d; continue;
 }
 0x1037e2d => {
  cpu.flat_ds(0x1037e2d); cpu.push(0x1037e32); f_01037db4(cpu); if cpu.returned_to != 0x1037e32 { return; }
  { cpu.interrupts_off = true; }
  { let v: u16 = cpu.r16(0x10e3dc7u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1037e56; continue; }
  pc = 0x1037e3e; continue;
 }
 0x1037e3e => {
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x10e3dc7u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e3fecu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x1124ea5u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x1037e56; continue;
 }
 0x1037e56 => {
  { cpu.sub16(cpu.r16(0x11b4dccu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x1037eb6; continue; }
  pc = 0x1037e60; continue;
 }
 0x1037e60 => {
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3dc7u32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0x11b4dccu32.wrapping_add(cpu.bases[3])) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x11b4dcau32.wrapping_add(cpu.bases[3])) as u32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u8 = cpu.port_in(0x21u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x1124eeau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x21u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e92u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = 0x0u16; cpu.w16(0x11b4dccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11b4dcau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3dc7u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1037eb6; continue;
 }
 0x1037eb6 => {
  { cpu.interrupts_off = false; }
  pc = 0x1037ebe; continue;
 }
 0x1037eb9 => {
  cpu.flat_ds(0x1037eb9); cpu.push(0x1037ebe); f_01061f35(cpu); if cpu.returned_to != 0x1037ebe { return; }
  pc = 0x1037ebe; continue;
 }
 0x1037ebe => {
  cpu.flat_ds(0x1037ebe); cpu.push(0x1037ec3); f_01037edc(cpu); if cpu.returned_to != 0x1037ec3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcbe2u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1037ed1); cpu.push(0x1037ed6); f_010754f0(cpu); if cpu.returned_to != 0x1037ed6 { return; }
  cpu.flat_ds(0x1037ed6); cpu.push(0x1037edb); f_01038abe(cpu); if cpu.returned_to != 0x1037edb { return; }
  cpu.flat_ds(0x1037edb); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037edc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1037edc); }
 let mut pc: u32 = 0x1037edc;
 loop { match pc {
 0x1037edc => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x10ee950u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91du32, v); }
  { let v: u32 = 0x3020100u32; cpu.w32(0x10ee948u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01037ef8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1037ef8); } r_01037ef8(cpu, 0x1037ef8); }
pub fn f_01038000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1038000); } r_01037ef8(cpu, 0x1038000); }
fn r_01037ef8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1037ef8 => {
  { cpu.sub8(cpu.r8(0x112c97cu32.wrapping_add(cpu.bases[3])), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1038146; continue; }
  pc = 0x1037f05; continue;
 }
 0x1037f05 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.flat_ds(0x1037f0a); cpu.push(0x1037f0f); f_01039358(cpu); if cpu.returned_to != 0x1037f0f { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b032fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11dde24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1131390u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  cpu.flat_ds(0x1037f2b); cpu.push(0x1037f30); f_01040c19(cpu); if cpu.returned_to != 0x1037f30 { return; }
  { cpu.interrupts_off = true; }
  cpu.flat_ds(0x1037f31); cpu.push(0x1037f36); f_01084fb7(cpu); if cpu.returned_to != 0x1037f36 { return; }
  cpu.flat_ds(0x1037f36); cpu.push(0x1037f3b); f_0103938c(cpu); if cpu.returned_to != 0x1037f3b { return; }
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b0300u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b0302u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e8cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b0304u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e8eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b0306u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e90u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b0308u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e92u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b030au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e94u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b030cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e96u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b030eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1124e98u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1037fb8); cpu.push(0x1037fbd); f_010385c4(cpu); if cpu.returned_to != 0x1037fbd { return; }
  { let v: u8 = cpu.r8(0x11263c7u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1124eecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])).wrapping_add(0x8u8); cpu.w8(0x1124eecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124eeau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124eebu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.interrupts_off = true; }
  { let v: u16 = cpu.r16(0x1124e90u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124bfdu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x112ca44u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x112ca45u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e90u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11b4dccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11b4dcau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dc7u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10e40f4u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x10b01a4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10b0000u32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dc7u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e3fecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1124ea5u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x103863eu32; cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x1124e90u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e92u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in(0x21u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x1124eebu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x21u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10b032cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x1124b9eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x103811e; continue; }
  pc = 0x10380fd; continue;
 }
 0x1038000 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x112ca45u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e90u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11b4dccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11b4dcau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dc7u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10e40f4u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x10b01a4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10b0000u32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dc7u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e3fecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1124ea5u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1124eecu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x103863eu32; cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x1124e90u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e92u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in(0x21u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x1124eebu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x21u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10b032cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x1124b9eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x103811e; continue; }
  pc = 0x10380fd; continue;
 }
 0x10380fd => {
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x87u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10b032cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x1124b9eu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x103811e; continue;
 }
 0x103811e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10b0310u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1124b9cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.interrupts_off = false; }
  { cpu.sub8(cpu.r8(0x112c97cu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x1038144; continue; }
  pc = 0x103813e; continue;
 }
 0x103813e => {
  cpu.flat_ds(0x103813e); cpu.push(0x1038143); f_010381dc(cpu); if cpu.returned_to != 0x1038143 { return; }
  cpu.flat_ds(0x1038143); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038144 => {
  { cpu.cf = false; }
  cpu.flat_ds(0x1038145); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038146 => {
  cpu.flat_ds(0x1038146); cpu.push(0x103814b); f_0103814c(cpu); if cpu.returned_to != 0x103814b { return; }
  cpu.flat_ds(0x103814b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103814c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103814c); }
 let mut pc: u32 = 0x103814c;
 loop { match pc {
 0x103814c => {
  { cpu.sub8(cpu.r8(0x1105c5du32), 0x0u8); }
  if !cpu.zf { pc = 0x10381a8; continue; }
  pc = 0x1038155; continue;
 }
 0x1038155 => {
  { let v: u32 = cpu.r32(0x1131068u32); cpu.esi = v; }
  cpu.push(0x1038160); f_01040c19(cpu); if cpu.returned_to != 0x1038160 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1038168); cpu.call(t); if cpu.returned_to != 0x1038168 { return; }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x1038172); f_010951f4(cpu); if cpu.returned_to != 0x1038172 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x109589cu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11c6340u32, v); }
  { let v: u32 = 0x10958cbu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11c6344u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6348u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10381d3; continue; }
  pc = 0x10381a1; continue;
 }
 0x10381a1 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1105c5du32, v); }
  pc = 0x10381a8; continue;
 }
 0x10381a8 => {
  cpu.push(0x10381ad); f_010957c1(cpu); if cpu.returned_to != 0x10381ad { return; }
  { let v: u32 = cpu.r32(0x113106cu32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x10381be; continue; }
  pc = 0x10381b8; continue;
 }
 0x10381b8 => {
  { let v: u32 = cpu.r32(0x1131070u32); cpu.esi = v; }
  pc = 0x10381be; continue;
 }
 0x10381be => {
  cpu.push(0x10381c3); f_01040c19(cpu); if cpu.returned_to != 0x10381c3 { return; }
  cpu.push(0x10381c8); f_010620dd(cpu); if cpu.returned_to != 0x10381c8 { return; }
  if cpu.cf { pc = 0x10381d3; continue; }
  pc = 0x10381ca; continue;
 }
 0x10381ca => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91eu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10381d3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010381dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10381dc); }
 let mut pc: u32 = 0x10381dc;
 loop { match pc {
 0x10381dc => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd3eau32, v); }
  cpu.push(0x10381e8); f_010754f0(cpu); if cpu.returned_to != 0x10381e8 { return; }
  { let v: u16 = cpu.r16(0x1124e92u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = 0xau32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1038201); cpu.call(t); if cpu.returned_to != 0x1038201 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  cpu.push(0x1038208); f_0103853d(cpu); if cpu.returned_to != 0x1038208 { return; }
  { let v: u32 = 0x1124c12u32; cpu.edi = v; }
  cpu.push(0x1038212); f_010384e5(cpu); if cpu.returned_to != 0x1038212 { return; }
  { let v: u32 = 0x1124c7du32; cpu.esi = v; }
  cpu.push(0x103821c); f_010385d7(cpu); if cpu.returned_to != 0x103821c { return; }
  if !cpu.cf { pc = 0x1038475; continue; }
  pc = 0x1038222; continue;
 }
 0x1038222 => {
  { let v: u32 = 0x10f7acdu32; cpu.edi = v; }
  cpu.push(0x103822c); f_010384e5(cpu); if cpu.returned_to != 0x103822c { return; }
  { let v: u32 = 0x1124c7du32; cpu.esi = v; }
  cpu.push(0x1038236); f_010385d7(cpu); if cpu.returned_to != 0x1038236 { return; }
  if !cpu.cf { pc = 0x1038475; continue; }
  pc = 0x103823c; continue;
 }
 0x103823c => {
  { cpu.sub8(cpu.r8(0x112c97du32), 0x0u8); }
  if cpu.zf { pc = 0x103829a; continue; }
  pc = 0x1038245; continue;
 }
 0x1038245 => {
  { let v: u32 = cpu.r32(0x1131110u32); cpu.esi = v; }
  cpu.push(0x1038250); f_01040c19(cpu); if cpu.returned_to != 0x1038250 { return; }
  { let v: u32 = 0x20d0u32; cpu.ecx = v; }
  pc = 0x1038255; continue;
 }
 0x1038255 => {
  cpu.push(0x103825a); f_010384bb(cpu); if cpu.returned_to != 0x103825a { return; }
  if cpu.cf { pc = 0x1038465; continue; }
  pc = 0x1038260; continue;
 }
 0x1038260 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1038467; continue; }
  pc = 0x1038267; continue;
 }
 0x1038267 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1124c71u32; cpu.esi = v; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  cpu.push(0x103827e); f_01037bbd(cpu); if cpu.returned_to != 0x103827e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x1038255; continue; }
  pc = 0x1038281; continue;
 }
 0x1038281 => {
  { let v: u32 = 0x1124c78u32; cpu.edi = v; }
  cpu.push(0x103828b); f_010384e5(cpu); if cpu.returned_to != 0x103828b { return; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  cpu.push(0x1038295); f_010385ad(cpu); if cpu.returned_to != 0x1038295 { return; }
  pc = 0x1038379; continue;
 }
 0x103829a => {
  { cpu.sub8(cpu.r8(0x10ef383u32), 0x0u8); }
  if cpu.zf { pc = 0x103849f; continue; }
  pc = 0x10382a7; continue;
 }
 0x10382a7 => {
  { cpu.sub8(cpu.r8(0x10ef383u32), 0xdu8); }
  if cpu.zf { pc = 0x103849f; continue; }
  pc = 0x10382b4; continue;
 }
 0x10382b4 => {
  { let v: u32 = cpu.r32(0x113110cu32); cpu.esi = v; }
  { let v: u32 = 0x11c5f54u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x10382c4; continue;
 }
 0x10382c4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10382d1; continue; }
  pc = 0x10382ca; continue;
 }
 0x10382ca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10382c4; continue; }
  pc = 0x10382d1; continue;
 }
 0x10382d1 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x10382e6; continue;
 }
 0x10382e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10382f3; continue; }
  pc = 0x10382ec; continue;
 }
 0x10382ec => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10382e6; continue; }
  pc = 0x10382f3; continue;
 }
 0x10382f3 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x11c5f54u32; cpu.esi = v; }
  cpu.push(0x103830e); f_01040c19(cpu); if cpu.returned_to != 0x103830e { return; }
  { cpu.sub8(cpu.r8(0x10ef383u32), 0x0u8); }
  if cpu.zf { pc = 0x103849f; continue; }
  pc = 0x103831b; continue;
 }
 0x103831b => {
  { cpu.sub8(cpu.r8(0x10ef383u32), 0xdu8); }
  if cpu.zf { pc = 0x103849f; continue; }
  pc = 0x1038328; continue;
 }
 0x1038328 => {
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u32 = 0x10f7fc8u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x1038334; continue;
 }
 0x1038334 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xdu8); }
  if cpu.zf { pc = 0x1038343; continue; }
  pc = 0x1038339; continue;
 }
 0x1038339 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1038343; continue; }
  pc = 0x103833e; continue;
 }
 0x103833e => {
  { cpu.movs(1, false); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1038334; continue;
 }
 0x1038343 => {
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  pc = 0x1038348; continue;
 }
 0x1038348 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xdu8); }
  if cpu.zf { pc = 0x1038357; continue; }
  pc = 0x103834d; continue;
 }
 0x103834d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1038357; continue; }
  pc = 0x1038352; continue;
 }
 0x1038352 => {
  { cpu.movs(1, false); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1038348; continue;
 }
 0x1038357 => {
  { let v: u8 = 0xdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91eu32, v); }
  cpu.push(0x103836d); f_01037d64(cpu); if cpu.returned_to != 0x103836d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32, v); }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  pc = 0x1038379; continue;
 }
 0x1038379 => {
  cpu.push(0x103837e); f_010384bb(cpu); if cpu.returned_to != 0x103837e { return; }
  if cpu.cf { pc = 0x1038465; continue; }
  pc = 0x1038384; continue;
 }
 0x1038384 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1038467; continue; }
  pc = 0x103838b; continue;
 }
 0x103838b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1124bfeu32; cpu.esi = v; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  cpu.push(0x10383a2); f_01037bbd(cpu); if cpu.returned_to != 0x10383a2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x1038459; continue; }
  pc = 0x10383a9; continue;
 }
 0x10383a9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1124c0au32; cpu.esi = v; }
  { let v: u8 = 0x4cu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  cpu.push(0x10383c0); f_01037bbd(cpu); if cpu.returned_to != 0x10383c0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x103849f; continue; }
  pc = 0x10383c7; continue;
 }
 0x10383c7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10f7ac4u32; cpu.esi = v; }
  { let v: u8 = 0x54u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  cpu.push(0x10383de); f_01037bbd(cpu); if cpu.returned_to != 0x10383de { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x10383e3; continue; }
  pc = 0x10383e1; continue;
 }
 0x10383e1 => {
  cpu.poll(); pc = 0x1038379; continue;
 }
 0x10383e3 => {
  { let v: u32 = cpu.r32(0x10fd546u32); cpu.esi = v; }
  { let v: u32 = 0x1124c18u32; cpu.edi = v; }
  { let v: u32 = 0x37u32; cpu.ecx = v; }
  pc = 0x10383f3; continue;
 }
 0x10383f3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x103841c; continue; }
  pc = 0x10383f9; continue;
 }
 0x10383f9 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x103841c; continue; }
  pc = 0x10383fd; continue;
 }
 0x10383fd => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103841c; continue; }
  pc = 0x1038401; continue;
 }
 0x1038401 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x103841c; continue; }
  pc = 0x1038406; continue;
 }
 0x1038406 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x11dc39cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x3ffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11dc39cu32); cpu.esi = v; }
  cpu.poll(); pc = 0x10383f3; continue;
 }
 0x103841c => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x1038421; continue;
 }
 0x1038421 => {
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1038421; continue; }
  pc = 0x1038428; continue;
 }
 0x1038428 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1124c18u32; cpu.esi = v; }
  cpu.push(0x1038435); f_01040c19(cpu); if cpu.returned_to != 0x1038435 { return; }
  { let v: u16 = 0x8cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x103843f); cpu.call(t); if cpu.returned_to != 0x103843f { return; }
  { let v: u32 = 0x1124bfeu32; cpu.esi = v; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  cpu.push(0x1038455); f_01037bbd(cpu); if cpu.returned_to != 0x1038455 { return; }
  if cpu.cf { pc = 0x1038459; continue; }
  pc = 0x1038457; continue;
 }
 0x1038457 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038459 => {
  { let v: u32 = cpu.r32(0x1131114u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  pc = 0x1038465; continue;
 }
 0x1038465 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038467 => {
  { let v: u32 = cpu.r32(0x11310f8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038475 => {
  { let v: u32 = cpu.r32(0x1131100u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103849f => {
  { let v: u32 = cpu.r32(0x1131108u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038483); }
 let mut pc: u32 = 0x1038483;
 loop { match pc {
 0x1038483 => {
  { let v: u32 = cpu.r32(0x1131104u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010384ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10384ad); }
 let mut pc: u32 = 0x10384ad;
 loop { match pc {
 0x10384ad => {
  { let v: u32 = cpu.r32(0x1131108u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010384bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10384bb); }
 let mut pc: u32 = 0x10384bb;
 loop { match pc {
 0x10384bb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10384c7); cpu.call(t); if cpu.returned_to != 0x10384c7 { return; }
  cpu.push(0x10384cc); f_0105eb1d(cpu); if cpu.returned_to != 0x10384cc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x10384d5; continue; }
  pc = 0x10384cf; continue;
 }
 0x10384cf => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x10384d5; continue; }
  pc = 0x10384d3; continue;
 }
 0x10384d3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10384d5 => {
  cpu.push(0x10384da); f_01059ae3(cpu); if cpu.returned_to != 0x10384da { return; }
  { cpu.sub8(cpu.r8(0x10fd3e8u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10384d3; continue; }
  pc = 0x10384e3; continue;
 }
 0x10384e3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010384e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10384e5); }
 let mut pc: u32 = 0x10384e5;
 loop { match pc {
 0x10384e5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10384e8; continue;
 }
 0x10384e8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))), 0xdu8); }
  if cpu.zf { pc = 0x10384f7; continue; }
  pc = 0x10384ee; continue;
 }
 0x10384ee => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x10384f7; continue; }
  pc = 0x10384f4; continue;
 }
 0x10384f4 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x10384e8; continue;
 }
 0x10384f7 => {
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = 0x5eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103850d; continue;
 }
 0x103850d => {
  { cpu.scas(1, 2); }
  if cpu.ecx & 0xffff == 0 { pc = 0x1038518; continue; }
  pc = 0x1038512; continue;
 }
 0x1038512 => {
  { let v: u8 = 0xdu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x103850d; continue;
 }
 0x1038518 => {
  { let v: u8 = 0xdu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x10ee91eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91eu32, v); }
  cpu.push(0x1038535); f_01037d64(cpu); if cpu.returned_to != 0x1038535 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee91eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103853d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103853d); }
 let mut pc: u32 = 0x103853d;
 loop { match pc {
 0x103853d => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x1u8); }
  if !cpu.zf { pc = 0x103857f; continue; }
  pc = 0x1038546; continue;
 }
 0x1038546 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b032fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11dde24u32, v); }
  cpu.push(0x1038561); f_01037d2e(cpu); if cpu.returned_to != 0x1038561 { return; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  cpu.push(0x103856b); f_010385ad(cpu); if cpu.returned_to != 0x103856b { return; }
  { let v: u32 = 0x1124c59u32; cpu.edi = v; }
  cpu.push(0x1038575); f_010384e5(cpu); if cpu.returned_to != 0x1038575 { return; }
  { let v: u32 = 0x1124c7du32; cpu.esi = v; }
  cpu.push(0x103857f); f_010385d7(cpu); if cpu.returned_to != 0x103857f { return; }
  pc = 0x103857f; continue;
 }
 0x103857f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038580); }
 let mut pc: u32 = 0x1038580;
 loop { match pc {
 0x1038580 => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x1u8); }
  if !cpu.zf { pc = 0x10385ac; continue; }
  pc = 0x1038589; continue;
 }
 0x1038589 => {
  cpu.push(0x103858e); f_010754f0(cpu); if cpu.returned_to != 0x103858e { return; }
  cpu.push(0x1038593); f_0103853d(cpu); if cpu.returned_to != 0x1038593 { return; }
  { let v: u32 = 0x1124c5fu32; cpu.edi = v; }
  cpu.push(0x103859d); f_010384e5(cpu); if cpu.returned_to != 0x103859d { return; }
  cpu.push(0x10385a2); f_01037d2e(cpu); if cpu.returned_to != 0x10385a2 { return; }
  { let v: u32 = 0x1124c7du32; cpu.esi = v; }
  cpu.push(0x10385ac); f_010385d7(cpu); if cpu.returned_to != 0x10385ac { return; }
  pc = 0x10385ac; continue;
 }
 0x10385ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010385ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10385ad); }
 let mut pc: u32 = 0x10385ad;
 loop { match pc {
 0x10385ad => {
  { let v: u32 = cpu.and32(cpu.ecx, 0x3ffu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4350u32); cpu.push(0x10385ba); cpu.call(t); if cpu.returned_to != 0x10385ba { return; }
  let t = cpu.r32(0x10e4340u32); cpu.push(0x10385c0); cpu.call(t); if cpu.returned_to != 0x10385c0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10385ad; continue; }
  pc = 0x10385c3; continue;
 }
 0x10385c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010385c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10385c4); }
 let mut pc: u32 = 0x10385c4;
 loop { match pc {
 0x10385c4 => {
  { let v: u16 = cpu.r16(0x1124e92u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103863e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103863e); }
 let mut pc: u32 = 0x103863e;
 loop { match pc {
 0x103863e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1038658; continue;
 }
 0x1038658 => {
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1038682; continue; }
  pc = 0x1038666; continue;
 }
 0x1038666 => {
  { let v: u16 = cpu.r16(0x1124e94u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10387c6; continue; }
  pc = 0x103867c; continue;
 }
 0x103867c => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10386e1; continue;
 }
 0x1038682 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1038758; continue; }
  pc = 0x103868a; continue;
 }
 0x103868a => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x1038758; continue; }
  pc = 0x1038692; continue;
 }
 0x1038692 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10386da; continue; }
  pc = 0x1038696; continue;
 }
 0x1038696 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10386bf; continue; }
  pc = 0x103869a; continue;
 }
 0x103869a => {
  { let v: u16 = cpu.r16(0x1124e96u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1124e94u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) & 0x60u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x60u8); }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10386e1; continue; }
  pc = 0x10386ba; continue;
 }
 0x10386ba => {
  pc = 0x10387c1; continue;
 }
 0x10386bf => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if !cpu.zf { pc = 0x10387c1; continue; }
  pc = 0x10386c7; continue;
 }
 0x10386c7 => {
  { let v: u16 = cpu.r16(0x1124e94u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10387c1; continue;
 }
 0x10386da => {
  { let v: u16 = cpu.r16(0x1124b9eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10386e1; continue;
 }
 0x10386e1 => {
  { let v: u16 = cpu.r16(0x1124e96u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10386f6; continue; }
  pc = 0x10386ed; continue;
 }
 0x10386ed => {
  { cpu.sub8(cpu.r8(0x10b0330u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x1038703; continue; }
  pc = 0x10386f6; continue;
 }
 0x10386f6 => {
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10387c6; continue;
 }
 0x1038703 => {
  { let v: u32 = cpu.r16(0x10b07b4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10b0332u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10b0312u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.w16(0x10b0312u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3ffu32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10b07b4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10b07b2u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x1038752; continue; }
  pc = 0x1038737; continue;
 }
 0x1038737 => {
  { let v: u16 = cpu.r16(0x1124e8cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b0330u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb7u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1038756; continue;
 }
 0x1038752 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1038703; continue; }
  pc = 0x1038756; continue;
 }
 0x1038756 => {
  pc = 0x10387c1; continue;
 }
 0x1038758 => {
  { let v: u16 = cpu.r16(0x1124e8au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10b0314u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.w16(0x10b0314u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10b0c3au32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10b0c3cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x1038790; continue; }
  pc = 0x1038780; continue;
 }
 0x1038780 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10b07bau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10b0c3au32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1038797; continue;
 }
 0x1038790 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10b031eu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.w16(0x10b031eu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1038797; continue;
 }
 0x1038797 => {
  { cpu.sub16(cpu.r16(0x1124b9cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x10387c1; continue; }
  pc = 0x10387a1; continue;
 }
 0x10387a1 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10b0326u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.w16(0x10b0326u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x1124e94u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1038758; continue; }
  pc = 0x10387ba; continue;
 }
 0x10387ba => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10b0326u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.w16(0x10b0326u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10387c1; continue;
 }
 0x10387c1 => {
  cpu.poll(); pc = 0x1038658; continue;
 }
 0x10387c6 => {
  { let v: u8 = cpu.r8(0x10b0331u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10387e4; continue; }
  pc = 0x10387d0; continue;
 }
 0x10387d0 => {
  { cpu.sub16(cpu.r16(0x1124b9cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x10387e4; continue; }
  pc = 0x10387da; continue;
 }
 0x10387da => {
  { let v: u16 = cpu.r16(0x1124e8eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x10387e4; continue;
 }
 0x10387e4 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x10387eb); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010387ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10387ec); }
 let mut pc: u32 = 0x10387ec;
 loop { match pc {
 0x10387ec => {
  { let v: u32 = 0x0u32; cpu.w32(0x1124ec0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb3u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x1105e2du32, v); }
  cpu.push(0x1038809); f_010376ea(cpu); if cpu.returned_to != 0x1038809 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32bu32, v); }
  { let v: u8 = cpu.r8(0x10ee96au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124e86u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee92au32, v); }
  pc = 0x1038826; continue;
 }
 0x1038826 => {
  { cpu.sub8(cpu.r8(0x11c5bb3u32), 0x0u8); }
  if cpu.zf { pc = 0x103883a; continue; }
  pc = 0x103882f; continue;
 }
 0x103882f => {
  cpu.push(0x1038834); f_0107b521(cpu); if cpu.returned_to != 0x1038834 { return; }
  if cpu.cf { pc = 0x1038918; continue; }
  pc = 0x103883a; continue;
 }
 0x103883a => {
  cpu.push(0x103883f); f_010373d0(cpu); if cpu.returned_to != 0x103883f { return; }
  { let v: u8 = cpu.r8(0x10ee96au32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef32bu32), 0x0u8); }
  if cpu.zf { pc = 0x1038886; continue; }
  pc = 0x103884e; continue;
 }
 0x103884e => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10ef32au32)); }
  if !cpu.zf { pc = 0x1038895; continue; }
  pc = 0x1038856; continue;
 }
 0x1038856 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x1105e2du32)); cpu.w8(0x1105e2du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32bu32, v); }
  if !cpu.zf { pc = 0x1038895; continue; }
  pc = 0x1038865; continue;
 }
 0x1038865 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5bb3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  cpu.push(0x1038878); f_01007ee0(cpu); if cpu.returned_to != 0x1038878 { return; }
  cpu.push(0x103887d); f_010376ea(cpu); if cpu.returned_to != 0x103887d { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1105e2du32, v); }
  pc = 0x1038895; continue;
 }
 0x1038886 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10ef32au32)); }
  if cpu.zf { pc = 0x1038895; continue; }
  pc = 0x103888e; continue;
 }
 0x103888e => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32bu32, v); }
  pc = 0x1038895; continue;
 }
 0x1038895 => {
  { let v: u8 = !cpu.r8(0x10ee92au32); cpu.w8(0x10ee92au32, v); }
  cpu.push(0x10388a0); f_01038a00(cpu); if cpu.returned_to != 0x10388a0 { return; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x11c5b24u32)); }
  if cpu.zf { pc = 0x1038922; continue; }
  pc = 0x10388a8; continue;
 }
 0x10388a8 => {
  { let v: u8 = cpu.r8(0x1124e86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if cpu.zf { cpu.poll(); pc = 0x1038826; continue; }
  pc = 0x10388b9; continue;
 }
 0x10388b9 => {
  { cpu.sub8(cpu.r8(0x11b4c13u32), 0x0u8); }
  if cpu.zf { pc = 0x10388d4; continue; }
  pc = 0x10388c2; continue;
 }
 0x10388c2 => {
  cpu.push(0x10388c7); f_010376ea(cpu); if cpu.returned_to != 0x10388c7 { return; }
  { cpu.sub8(cpu.r8(0x11c5f46u32), 0x0u8); }
  if cpu.zf { pc = 0x10389a1; continue; }
  pc = 0x10388d4; continue;
 }
 0x10388d4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1124ec0u32)); cpu.w32(0x1124ec0u32, v); }
  { cpu.sub32(cpu.r32(0x1124ec0u32), 0x14u32); }
  if cpu.cf { pc = 0x10388fb; continue; }
  pc = 0x10388ea; continue;
 }
 0x10388ea => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = 0x13u32; cpu.w32(0x1124ec0u32, v); }
  pc = 0x10388fb; continue;
 }
 0x10388fb => {
  { cpu.pushad(); }
  cpu.push(0x1038901); f_01038d64(cpu); if cpu.returned_to != 0x1038901 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = cpu.r8(0x10ee96au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124e86u32, v); }
  cpu.poll(); pc = 0x1038826; continue;
 }
 0x1038918 => {
  cpu.push(0x103891d); f_01038abf(cpu); if cpu.returned_to != 0x103891d { return; }
  cpu.push(0x1038922); f_01038f34(cpu); if cpu.returned_to != 0x1038922 { return; }
  pc = 0x1038922; continue;
 }
 0x1038922 => {
  { let v: u8 = cpu.r8(0x1124edcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x103893c; continue; }
  pc = 0x103892b; continue;
 }
 0x103892b => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124e46u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1038986; continue; }
  pc = 0x103893c; continue;
 }
 0x103893c => {
  { let v: u8 = cpu.r8(0x1124eddu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1038956; continue; }
  pc = 0x1038945; continue;
 }
 0x1038945 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124e46u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1038986; continue; }
  pc = 0x1038956; continue;
 }
 0x1038956 => {
  { let v: u8 = cpu.r8(0x1124edeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124e46u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1038986; continue; }
  pc = 0x103896c; continue;
 }
 0x103896c => {
  { let v: u8 = cpu.r8(0x1124edfu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1038984; continue; }
  pc = 0x1038975; continue;
 }
 0x1038975 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124e46u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  pc = 0x1038984; continue;
 }
 0x1038984 => {
  if cpu.zf { pc = 0x10389a1; continue; }
  pc = 0x1038986; continue;
 }
 0x1038986 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef325u32)); cpu.w8(0x10ef325u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef326u32, v); }
  { cpu.sub8(cpu.r8(0x10ef325u32), 0x78u8); }
  if cpu.zf { pc = 0x10389c4; continue; }
  pc = 0x103899c; continue;
 }
 0x103899c => {
  cpu.poll(); pc = 0x10388b9; continue;
 }
 0x10389a1 => {
  { cpu.sub8(cpu.r8(0x11c5bb3u32), 0x0u8); }
  if cpu.zf { pc = 0x10389b6; continue; }
  pc = 0x10389aa; continue;
 }
 0x10389aa => {
  cpu.push(0x10389af); f_01059ab7(cpu); if cpu.returned_to != 0x10389af { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  pc = 0x10389b6; continue;
 }
 0x10389b6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef325u32, v); }
  pc = 0x10389c4; continue;
 }
 0x10389c4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee929u32)); cpu.w8(0x10ee929u32, v); }
  { cpu.sub8(cpu.r8(0x10ee929u32), 0xbcu8); }
  if !cpu.zf { pc = 0x10389da; continue; }
  pc = 0x10389d3; continue;
 }
 0x10389d3 => {
  { let v: u8 = 0x81u8; cpu.w8(0x10ee929u32, v); }
  pc = 0x10389da; continue;
 }
 0x10389da => {
  cpu.push(0x10389df); f_0103777f(cpu); if cpu.returned_to != 0x10389df { return; }
  cpu.push(0x10389e4); f_01038a54(cpu); if cpu.returned_to != 0x10389e4 { return; }
  cpu.push(0x10389e9); f_010376a8(cpu); if cpu.returned_to != 0x10389e9 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x1124e87u32)); cpu.w8(0x1124e87u32, v); }
  if !cpu.sf { pc = 0x10389f8; continue; }
  pc = 0x10389f1; continue;
 }
 0x10389f1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1124e87u32, v); }
  pc = 0x10389f8; continue;
 }
 0x10389f8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b4c13u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038a00); }
 let mut pc: u32 = 0x1038a00;
 loop { match pc {
 0x1038a00 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1124edcu32, v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11c5b26u32), 0x0u8); }
  if cpu.zf { pc = 0x1038a1e; continue; }
  pc = 0x1038a17; continue;
 }
 0x1038a17 => {
  cpu.push(0x1038a1c); f_01038a6e(cpu); if cpu.returned_to != 0x1038a1c { return; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1038a1e; continue;
 }
 0x1038a1e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11c5b27u32), 0x0u8); }
  if cpu.zf { pc = 0x1038a2f; continue; }
  pc = 0x1038a28; continue;
 }
 0x1038a28 => {
  cpu.push(0x1038a2d); f_01038a6e(cpu); if cpu.returned_to != 0x1038a2d { return; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1038a2f; continue;
 }
 0x1038a2f => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11c5b28u32), 0x0u8); }
  if cpu.zf { pc = 0x1038a40; continue; }
  pc = 0x1038a39; continue;
 }
 0x1038a39 => {
  cpu.push(0x1038a3e); f_01038a6e(cpu); if cpu.returned_to != 0x1038a3e { return; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1038a40; continue;
 }
 0x1038a40 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11c5b29u32), 0x0u8); }
  if cpu.zf { pc = 0x1038a51; continue; }
  pc = 0x1038a4a; continue;
 }
 0x1038a4a => {
  cpu.push(0x1038a4f); f_01038a6e(cpu); if cpu.returned_to != 0x1038a4f { return; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1038a51; continue;
 }
 0x1038a51 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038a54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038a54); }
 let mut pc: u32 = 0x1038a54;
 loop { match pc {
 0x1038a54 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.push(0x1038a5b); f_01038a9b(cpu); if cpu.returned_to != 0x1038a5b { return; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.push(0x1038a61); f_01038a9b(cpu); if cpu.returned_to != 0x1038a61 { return; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.push(0x1038a67); f_01038a9b(cpu); if cpu.returned_to != 0x1038a67 { return; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.push(0x1038a6d); f_01038a9b(cpu); if cpu.returned_to != 0x1038a6d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038a6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038a6e); }
 let mut pc: u32 = 0x1038a6e;
 loop { match pc {
 0x1038a6e => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11dc37cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1038a80; continue;
 }
 0x1038a80 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1038a8f; continue; }
  pc = 0x1038a89; continue;
 }
 0x1038a89 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1038a80; continue; }
  pc = 0x1038a8c; continue;
 }
 0x1038a8c => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1038a8f => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124edcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038a9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038a9b); }
 let mut pc: u32 = 0x1038a9b;
 loop { match pc {
 0x1038a9b => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11dc37cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ee929u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1038aad; continue;
 }
 0x1038aad => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1038aba; continue; }
  pc = 0x1038ab6; continue;
 }
 0x1038ab6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  pc = 0x1038aba; continue;
 }
 0x1038aba => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1038aad; continue; }
  pc = 0x1038abd; continue;
 }
 0x1038abd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038abe); }
 let mut pc: u32 = 0x1038abe;
 loop { match pc {
 0x1038abe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038abf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038abf); }
 let mut pc: u32 = 0x1038abf;
 loop { match pc {
 0x1038abf => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1038add; continue; }
  pc = 0x1038ac8; continue;
 }
 0x1038ac8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc4eu32, v); }
  pc = 0x1038acf; continue;
 }
 0x1038acf => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  cpu.push(0x1038adb); f_010379f8(cpu); if cpu.returned_to != 0x1038adb { return; }
  if cpu.cf { cpu.poll(); pc = 0x1038acf; continue; }
  pc = 0x1038add; continue;
 }
 0x1038add => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038ade); }
 let mut pc: u32 = 0x1038ade;
 loop { match pc {
 0x1038ade => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c12u32, v); }
  cpu.push(0x1038aea); f_01038b0c(cpu); if cpu.returned_to != 0x1038aea { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038aeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038aeb); }
 let mut pc: u32 = 0x1038aeb;
 loop { match pc {
 0x1038aeb => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b4c12u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11c5bb6u32, v); }
  cpu.push(0x1038afe); f_01038b0c(cpu); if cpu.returned_to != 0x1038afe { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038aff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038aff); }
 let mut pc: u32 = 0x1038aff;
 loop { match pc {
 0x1038aff => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b4c12u32, v); }
  cpu.push(0x1038b0b); f_01038b0c(cpu); if cpu.returned_to != 0x1038b0b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01038ea5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1038ea5); }
 let mut pc: u32 = 0x1038ea5;
 loop { match pc {
 0x1038ea5 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1038f33; continue; }
  pc = 0x1038eb2; continue;
 }
 0x1038eb2 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x10c6880u32; cpu.esi = v; }
  { let v: u16 = 0x3cu16; cpu.w16(0x10e3dfdu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef20cu32, v); }
  cpu.push(0x1038ed6); f_01037d64(cpu); if cpu.returned_to != 0x1038ed6 { return; }
  { let v: u8 = 0xf6u8; cpu.w8(0x10ef325u32, v); }
  { let v: u32 = 0xau32; cpu.ebp = v; }
  pc = 0x1038ee2; continue;
 }
 0x1038ee2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0xf6u8; cpu.w8(0x10ef325u32, v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x10c68bfu32; cpu.esi = v; }
  { let v: u16 = 0xau16; cpu.w16(0x10e3dfdu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef20cu32, v); }
  cpu.push(0x1038f0e); f_01037aef(cpu); if cpu.returned_to != 0x1038f0e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1038f1f; continue; }
  pc = 0x1038f11; continue;
 }
 0x1038f11 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1038ee2; continue; }
  pc = 0x1038f14; continue;
 }
 0x1038f14 => {
  { cpu.sub8(cpu.r8(0x10c6877u32), 0x1u8); }
  if cpu.zf { pc = 0x1038f1f; continue; }
  pc = 0x1038f1d; continue;
 }
 0x1038f1d => {
  cpu.poll(); pc = 0x1038eb2; continue;
 }
 0x1038f1f => {
  { let v: u32 = 0x10c68bfu32; cpu.esi = v; }
  cpu.push(0x1038f29); f_01039178(cpu); if cpu.returned_to != 0x1038f29 { return; }
  { let v: u32 = 0x10c6880u32; cpu.esi = v; }
  cpu.push(0x1038f33); f_01039178(cpu); if cpu.returned_to != 0x1038f33 { return; }
  pc = 0x1038f33; continue;
 }
 0x1038f33 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103903f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103903f); }
 let mut pc: u32 = 0x103903f;
 loop { match pc {
 0x103903f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x103904a; continue;
 }
 0x103904a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x112499cu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.xor16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103904a; continue; }
  pc = 0x103906c; continue;
 }
 0x103906c => {
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
pub fn f_01039072(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039072); }
 let mut pc: u32 = 0x1039072;
 loop { match pc {
 0x1039072 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1039161; continue; }
  pc = 0x103907f; continue;
 }
 0x103907f => {
  { cpu.sub8(cpu.r8(0x11c5f46u32), 0x0u8); }
  if cpu.zf { pc = 0x1039089; continue; }
  pc = 0x1039088; continue;
 }
 0x1039088 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039089 => {
  { cpu.sub8(cpu.r8(0x10fd44au32), 0x0u8); }
  if cpu.zf { pc = 0x10390a3; continue; }
  pc = 0x1039092; continue;
 }
 0x1039092 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10fd44au32)); cpu.w8(0x10fd44au32, v); }
  if !cpu.zf { pc = 0x1039161; continue; }
  pc = 0x103909e; continue;
 }
 0x103909e => {
  pc = 0x103915c; continue;
 }
 0x10390a3 => {
  { cpu.sub8(cpu.r8(0x10ef20du32), 0x2u8); }
  if !cpu.zf { pc = 0x1039161; continue; }
  pc = 0x10390b0; continue;
 }
 0x10390b0 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11c5b2bu32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1039161; continue; }
  pc = 0x10390c1; continue;
 }
 0x10390c1 => {
  { cpu.sub8(cpu.r8(0x10ef325u32), 0x0u8); }
  if cpu.zf { pc = 0x1039161; continue; }
  pc = 0x10390ce; continue;
 }
 0x10390ce => {
  if cpu.sf { pc = 0x1039161; continue; }
  pc = 0x10390d4; continue;
 }
 0x10390d4 => {
  cpu.push(0x10390d9); f_0105f178(cpu); if cpu.returned_to != 0x10390d9 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124ee6u32, v); }
  cpu.push(0x10390e6); f_0105f178(cpu); if cpu.returned_to != 0x10390e6 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124ee7u32, v); }
  cpu.push(0x10390f3); f_0105f178(cpu); if cpu.returned_to != 0x10390f3 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3fu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124ee8u32, v); }
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1124ee6u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1124ee7u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1124ee8u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1039115); f_010726f9(cpu); if cpu.returned_to != 0x1039115 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef325u32, v); }
  { let v: u8 = 0x6u8; cpu.w8(0x10fd44au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11c5f46u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11c5f47u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1039144); f_010385ad(cpu); if cpu.returned_to != 0x1039144 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x103914c); cpu.call(t); if cpu.returned_to != 0x103914c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd361u32, v); }
  pc = 0x1039161; continue;
 }
 0x103915c => {
  cpu.push(0x1039161); f_01037a70(cpu); if cpu.returned_to != 0x1039161 { return; }
  pc = 0x1039161; continue;
 }
 0x1039161 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039163(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039163); }
 let mut pc: u32 = 0x1039163;
 loop { match pc {
 0x1039163 => {
  { let v: u32 = 0x10fab88u32; cpu.edi = v; }
  { let v: u32 = 0x10fcae0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10fab88u32); cpu.ecx = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039178); }
 let mut pc: u32 = 0x1039178;
 loop { match pc {
 0x1039178 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1039182; continue; }
  pc = 0x1039181; continue;
 }
 0x1039181 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039182 => {
  cpu.push(0x1039187); f_01045e5f(cpu); if cpu.returned_to != 0x1039187 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010391e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10391e8); }
 let mut pc: u32 = 0x10391e8;
 loop { match pc {
 0x10391e8 => {
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1105995u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1105c95u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f7f7f7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124bd2u32, v); }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1105995u32)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x7f7f7f7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124bd6u32, v); }
  { let v: u32 = 0x1124bdau32; cpu.edi = v; }
  { let v: u32 = 0x112636fu32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x1039242; continue;
 }
 0x1039242 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x103924f; continue; }
  pc = 0x1039247; continue;
 }
 0x1039247 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039242; continue; }
  pc = 0x103924f; continue;
 }
 0x103924f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x112636fu32; cpu.esi = v; }
  { let v: u32 = 0x1105e15u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x1124bceu32; cpu.esi = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  cpu.push(0x1039272); f_01037d64(cpu); if cpu.returned_to != 0x1039272 { return; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x1039277; continue;
 }
 0x1039277 => {
  { let v: u32 = 0x1124be1u32; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1039282); f_01045e4d(cpu); if cpu.returned_to != 0x1039282 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10392a6; continue; }
  pc = 0x103928c; continue;
 }
 0x103928c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1039293); f_01037546(cpu); if cpu.returned_to != 0x1039293 { return; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x103929d); cpu.call(t); if cpu.returned_to != 0x103929d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039277; continue; }
  pc = 0x10392a1; continue;
 }
 0x10392a1 => {
  pc = 0x1039356; continue;
 }
 0x10392a6 => {
  { let v: u32 = 0x1124bceu32; cpu.esi = v; }
  cpu.push(0x10392b0); f_01045e4d(cpu); if cpu.returned_to != 0x10392b0 { return; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if cpu.zf { pc = 0x1039356; continue; }
  pc = 0x10392bd; continue;
 }
 0x10392bd => {
  { let v: u32 = 0x1124be6u32; cpu.edi = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10fd546u32); cpu.esi = v; }
  { let v: u32 = 0x11dc39cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  pc = 0x10392d8; continue;
 }
 0x10392d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x3ffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10392d8; continue; }
  pc = 0x10392ec; continue;
 }
 0x10392ec => {
  { let v: u32 = 0x1124bf2u32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x1105e16u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1124beau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1124beeu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1124bd6u32), cpu.ebx); }
  if cpu.zf { pc = 0x1039356; continue; }
  pc = 0x1039313; continue;
 }
 0x1039313 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x103931b; continue; }
  pc = 0x1039315; continue;
 }
 0x1039315 => {
  { let v: u32 = cpu.r32(0x1124bd2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x103931b; continue;
 }
 0x103931b => {
  { let v: u32 = cpu.eax | 0xffff0000u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1105c62u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1105c91u32, v); }
  { let v: u8 = cpu.r8(0x1105c91u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1105e15u32; cpu.ebx = v; }
  { let v: u32 = 0x1105e1bu32; cpu.edx = v; }
  { let v: u32 = 0x1105e83u32; cpu.esi = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x1039347; continue; }
  pc = 0x1039345; continue;
 }
 0x1039345 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x1039347; continue;
 }
 0x1039347 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10fcb43u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039356 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039358); }
 let mut pc: u32 = 0x1039358;
 loop { match pc {
 0x1039358 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d7fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c5b24u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b26u32, v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103938b; continue; }
  pc = 0x103936e; continue;
 }
 0x103936e => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b27u32, v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103938b; continue; }
  pc = 0x1039379; continue;
 }
 0x1039379 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b28u32, v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103938b; continue; }
  pc = 0x1039384; continue;
 }
 0x1039384 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b29u32, v); }
  pc = 0x103938b; continue;
 }
 0x103938b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103938c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103938c); }
 let mut pc: u32 = 0x103938c;
 loop { match pc {
 0x103938c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10b0c3au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10b0c3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10b07bau32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u16 = 0x0u16; cpu.w16(0x10b07b2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10b07b4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10b0332u32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010393d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10393d8); }
 let mut pc: u32 = 0x10393d8;
 loop { match pc {
 0x10393d8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10fcd62u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10fcd5eu32, v); }
  { let v: u32 = 0x11a188cu32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x10393f2; continue;
 }
 0x10393f2 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x103942f; continue; }
  pc = 0x10393f9; continue;
 }
 0x10393f9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x1039415; continue; }
  pc = 0x1039409; continue;
 }
 0x1039409 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x1039415; continue; }
  pc = 0x103940d; continue;
 }
 0x103940d => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x1039415; continue; }
  pc = 0x1039411; continue;
 }
 0x1039411 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103942f; continue; }
  pc = 0x1039415; continue;
 }
 0x1039415 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1039421); f_0108c1c5(cpu); if cpu.returned_to != 0x1039421 { return; }
  { cpu.sub32(cpu.r32(0x10fcd5eu32), cpu.esi); }
  if !cpu.zf { pc = 0x103942f; continue; }
  pc = 0x1039429; continue;
 }
 0x1039429 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10fcd62u32)); cpu.w32(0x10fcd62u32, v); }
  pc = 0x103942f; continue;
 }
 0x103942f => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10393f2; continue; }
  pc = 0x1039435; continue;
 }
 0x1039435 => {
  { let v: u32 = cpu.r32(0x10fcd62u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103943b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103943b); }
 let mut pc: u32 = 0x103943b;
 loop { match pc {
 0x103943b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4144u32, v); }
  { let v: u32 = 0x11a188cu32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x103945d; continue;
 }
 0x103945d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1039501; continue; }
  pc = 0x1039468; continue;
 }
 0x1039468 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039478; continue;
 }
 0x1039478 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x103947c; continue;
 }
 0x103947c => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039480; continue;
 }
 0x1039480 => {
  { cpu.sub8((cpu.eax as u8), 0x1du8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039484; continue;
 }
 0x1039484 => {
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039488; continue;
 }
 0x1039488 => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x103948c; continue;
 }
 0x103948c => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039490; continue;
 }
 0x1039490 => {
  { cpu.sub8((cpu.eax as u8), 0x21u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039494; continue;
 }
 0x1039494 => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x1039498; continue;
 }
 0x1039498 => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x10394a0; continue; }
  pc = 0x103949c; continue;
 }
 0x103949c => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if !cpu.zf { pc = 0x1039501; continue; }
  pc = 0x10394a0; continue;
 }
 0x10394a0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4138u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1039501; continue; }
  pc = 0x10394ab; continue;
 }
 0x10394ab => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4140u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039501; continue; }
  pc = 0x10394b3; continue;
 }
 0x10394b3 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e413cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1039501; continue; }
  pc = 0x10394bf; continue;
 }
 0x10394bf => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4144u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039501; continue; }
  pc = 0x10394c7; continue;
 }
 0x10394c7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10394d7); f_0104f36d(cpu); if cpu.returned_to != 0x10394d7 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x14u32); cpu.edx = v; }
  cpu.push(0x10394e6); f_01073cc7(cpu); if cpu.returned_to != 0x10394e6 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x10394ec); f_01073cc7(cpu); if cpu.returned_to != 0x10394ec { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7eu32); cpu.esi = v; }
  cpu.push(0x10394f5); f_01073cc7(cpu); if cpu.returned_to != 0x10394f5 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x10394fb); f_01073cc7(cpu); if cpu.returned_to != 0x10394fb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1039501; continue;
 }
 0x1039501 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103945d; continue; }
  pc = 0x103950b; continue;
 }
 0x103950b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103950d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103950d); }
 let mut pc: u32 = 0x103950d;
 loop { match pc {
 0x103950d => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  cpu.push(0x1039555); f_0105aa11(cpu); if cpu.returned_to != 0x1039555 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039557(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039557); }
 let mut pc: u32 = 0x1039557;
 loop { match pc {
 0x1039557 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1039564; continue;
 }
 0x1039564 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x10395a6; continue; }
  pc = 0x103956b; continue;
 }
 0x103956b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x10u8); }
  if cpu.zf { pc = 0x10395a6; continue; }
  pc = 0x1039574; continue;
 }
 0x1039574 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x10395a6; continue; }
  pc = 0x1039579; continue;
 }
 0x1039579 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x10395a6; continue; }
  pc = 0x103957f; continue;
 }
 0x103957f => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10395a6; continue; }
  pc = 0x1039594; continue;
 }
 0x1039594 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10395a6; continue; }
  pc = 0x103959d; continue;
 }
 0x103959d => {
  { let v: u32 = cpu.esi; cpu.w32(0x110d6e4u32, v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10395a6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1039564; continue; }
  pc = 0x10395ac; continue;
 }
 0x10395ac => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010395af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10395af); }
 let mut pc: u32 = 0x10395af;
 loop { match pc {
 0x10395af => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11a188eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11a188cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x10395cf; continue; }
  pc = 0x10395cd; continue;
 }
 0x10395cd => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10395cf => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010395d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10395d1); }
 let mut pc: u32 = 0x10395d1;
 loop { match pc {
 0x10395d1 => {
  { let v: u32 = 0x116a8b4u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x10395db; continue;
 }
 0x10395db => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x10395e3; continue; }
  pc = 0x10395e1; continue;
 }
 0x10395e1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10395e3 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10395db; continue; }
  pc = 0x10395e8; continue;
 }
 0x10395e8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010395ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10395ea); }
 let mut pc: u32 = 0x10395ea;
 loop { match pc {
 0x10395ea => {
  { let v: u32 = 0x116a8b4u32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x10395f4; continue;
 }
 0x10395f4 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1039695; continue; }
  pc = 0x10395ff; continue;
 }
 0x10395ff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x103960d); cpu.call(t); if cpu.returned_to != 0x103960d { return; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x1123084u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x103962d; continue; }
  pc = 0x1039628; continue;
 }
 0x1039628 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103962d; continue;
 }
 0x103962d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1039693; continue; }
  pc = 0x103963a; continue;
 }
 0x103963a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039693; continue; }
  pc = 0x1039643; continue;
 }
 0x1039643 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1039693; continue; }
  pc = 0x1039650; continue;
 }
 0x1039650 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039693; continue; }
  pc = 0x1039659; continue;
 }
 0x1039659 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x103966c); cpu.call(t); if cpu.returned_to != 0x103966c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x13du16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1039680; continue; }
  pc = 0x103967b; continue;
 }
 0x103967b => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1039680; continue;
 }
 0x1039680 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x1039693); cpu.call(t); if cpu.returned_to != 0x1039693 { return; }
  pc = 0x1039693; continue;
 }
 0x1039693 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1039695; continue;
 }
 0x1039695 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10395f4; continue; }
  pc = 0x103969f; continue;
 }
 0x103969f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010396a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10396a0); }
 let mut pc: u32 = 0x10396a0;
 loop { match pc {
 0x10396a0 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11de3adu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11de3acu32, v); }
  { cpu.sub16((cpu.ecx as u16), 0x1fu16); }
  if cpu.zf { pc = 0x10396c3; continue; }
  pc = 0x10396b4; continue;
 }
 0x10396b4 => {
  { cpu.sub16((cpu.ecx as u16), 0x3au16); }
  if !cpu.zf { pc = 0x10396ca; continue; }
  pc = 0x10396ba; continue;
 }
 0x10396ba => {
  { let v: u8 = 0x2u8; cpu.w8(0x11de3acu32, v); }
  pc = 0x10396ca; continue;
 }
 0x10396c3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11de3acu32, v); }
  pc = 0x10396ca; continue;
 }
 0x10396ca => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10396d3); f_01039af3(cpu); if cpu.returned_to != 0x10396d3 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10397a5; continue; }
  pc = 0x10396dd; continue;
 }
 0x10396dd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10396fa); f_01039870(cpu); if cpu.returned_to != 0x10396fa { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x1039700); f_010397a6(cpu); if cpu.returned_to != 0x1039700 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x103970d; continue; }
  pc = 0x1039709; continue;
 }
 0x1039709 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x103970d; continue;
 }
 0x103970d => {
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1039756; continue; }
  pc = 0x1039713; continue;
 }
 0x1039713 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1039724; continue; }
  pc = 0x1039720; continue;
 }
 0x1039720 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1039724; continue;
 }
 0x1039724 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1039735); f_0104f36d(cpu); if cpu.returned_to != 0x1039735 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x103977a; continue;
 }
 0x1039756 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10397a5; continue; }
  pc = 0x103975f; continue;
 }
 0x103975f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103976b); f_0104f36d(cpu); if cpu.returned_to != 0x103976b { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103977a; continue;
 }
 0x103977a => {
  { cpu.sub16(cpu.r16(0x11de3adu32), 0x47u16); }
  if (cpu.sf != cpu.of) { pc = 0x1039792; continue; }
  pc = 0x1039785; continue;
 }
 0x1039785 => {
  { cpu.sub16(cpu.r16(0x11de3adu32), 0x4du16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039792; continue; }
  pc = 0x1039790; continue;
 }
 0x1039790 => {
  pc = 0x1039794; continue;
 }
 0x1039792 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1039794; continue;
 }
 0x1039794 => {
  { cpu.sub8(cpu.r8(0x1124ef0u32), 0x1u8); }
  if !cpu.zf { pc = 0x10397a5; continue; }
  pc = 0x103979d; continue;
 }
 0x103979d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x1748u16); }
  if !cpu.zf { pc = 0x10397a5; continue; }
  pc = 0x10397a5; continue;
 }
 0x10397a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010397a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10397a6); }
 let mut pc: u32 = 0x10397a6;
 loop { match pc {
 0x10397a6 => {
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1039806; continue; }
  pc = 0x10397ab; continue;
 }
 0x10397ab => {
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039806 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039870(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039870); }
 let mut pc: u32 = 0x1039870;
 loop { match pc {
 0x1039870 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x1039882; continue; }
  pc = 0x1039874; continue;
 }
 0x1039874 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) & 0xfff0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) & 0xfff0u16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub(0xeu16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1039882; continue;
 }
 0x1039882 => {
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
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 7u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10398c4); f_01039a58(cpu); if cpu.returned_to != 0x10398c4 { return; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5u32); }
  if cpu.zf { pc = 0x103998e; continue; }
  pc = 0x10398d3; continue;
 }
 0x10398d3 => {
  { cpu.sub32(cpu.esi, 0x15u32); }
  if cpu.zf { pc = 0x103998e; continue; }
  pc = 0x10398dc; continue;
 }
 0x10398dc => {
  { cpu.sub32(cpu.esi, 0x16u32); }
  if cpu.zf { pc = 0x103998e; continue; }
  pc = 0x10398e5; continue;
 }
 0x10398e5 => {
  { cpu.sub32(cpu.esi, 0x17u32); }
  if cpu.zf { pc = 0x103998e; continue; }
  pc = 0x10398ee; continue;
 }
 0x10398ee => {
  { cpu.sub32(cpu.esi, 0x8u32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x10398f7; continue;
 }
 0x10398f7 => {
  { cpu.sub32(cpu.esi, 0x1fu32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x1039900; continue;
 }
 0x1039900 => {
  { cpu.sub32(cpu.esi, 0x18u32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x1039909; continue;
 }
 0x1039909 => {
  { cpu.sub32(cpu.esi, 0x21u32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x1039912; continue;
 }
 0x1039912 => {
  { cpu.sub32(cpu.esi, 0x25u32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x103991b; continue;
 }
 0x103991b => {
  { cpu.sub32(cpu.esi, 0x23u32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x1039924; continue;
 }
 0x1039924 => {
  { cpu.sub32(cpu.esi, 0x1au32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x103992d; continue;
 }
 0x103992d => {
  { cpu.sub32(cpu.esi, 0x1du32); }
  if cpu.zf { pc = 0x10399e7; continue; }
  pc = 0x1039936; continue;
 }
 0x1039936 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x80u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x32u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.esi as u16), 0xbu16); }
  if cpu.zf { pc = 0x1039980; continue; }
  pc = 0x1039969; continue;
 }
 0x1039969 => {
  { cpu.sub16((cpu.esi as u16), 0xcu16); }
  if cpu.zf { pc = 0x1039980; continue; }
  pc = 0x103996f; continue;
 }
 0x103996f => {
  { cpu.sub16((cpu.esi as u16), 0x11u16); }
  if cpu.zf { pc = 0x1039980; continue; }
  pc = 0x1039975; continue;
 }
 0x1039975 => {
  { cpu.sub32(cpu.esi, 0x9u32); }
  if cpu.zf { pc = 0x1039989; continue; }
  pc = 0x103997a; continue;
 }
 0x103997a => {
  { cpu.sub32(cpu.esi, 0xau32); }
  if cpu.zf { pc = 0x1039989; continue; }
  pc = 0x103997f; continue;
 }
 0x103997f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039980 => {
  { let v: u8 = cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edi)), 0x7fu8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039989 => {
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103998e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3f1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f25u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f29u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) & 0xc1u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x60u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0xdu32.wrapping_add(cpu.edi)), 0x10u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10399e7 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x30u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3f2bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f2du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f2fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f31u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f33u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3f35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x11de3acu32), 0x1u8); }
  if !cpu.zf { pc = 0x1039a42; continue; }
  pc = 0x1039a3c; continue;
 }
 0x1039a3c => {
  { let v: u8 = 0x1eu8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x1039a4f; continue;
 }
 0x1039a42 => {
  { cpu.sub8(cpu.r8(0x11de3acu32), 0x2u8); }
  if !cpu.zf { pc = 0x1039a4f; continue; }
  pc = 0x1039a4b; continue;
 }
 0x1039a4b => {
  { let v: u8 = 0x6u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x1039a4f; continue;
 }
 0x1039a4f => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039a58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039a58); }
 let mut pc: u32 = 0x1039a58;
 loop { match pc {
 0x1039a58 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  cpu.push(0x1039a65); f_0108c206(cpu); if cpu.returned_to != 0x1039a65 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1039a69; continue; }
  pc = 0x1039a68; continue;
 }
 0x1039a68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039a69 => {
  { let v: u8 = cpu.or8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039a6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039a6e); }
 let mut pc: u32 = 0x1039a6e;
 loop { match pc {
 0x1039a6e => {
  cpu.push(0x1039a73); f_01039a7b(cpu); if cpu.returned_to != 0x1039a73 { return; }
  if cpu.cf { pc = 0x1039a7a; continue; }
  pc = 0x1039a75; continue;
 }
 0x1039a75 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1039a7a; continue;
 }
 0x1039a7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039a7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039a7b); }
 let mut pc: u32 = 0x1039a7b;
 loop { match pc {
 0x1039a7b => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1039af1; continue; }
  pc = 0x1039a94; continue;
 }
 0x1039a94 => {
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1039af1; continue; }
  pc = 0x1039aa4; continue;
 }
 0x1039aa4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x4u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1039acd; continue;
 }
 0x1039acd => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1039aeb; continue; }
  pc = 0x1039ad3; continue;
 }
 0x1039ad3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039aeb; continue; }
  pc = 0x1039ad8; continue;
 }
 0x1039ad8 => {
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x1039aeb; continue; }
  pc = 0x1039add; continue;
 }
 0x1039add => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1039aeb; continue; }
  pc = 0x1039ae3; continue;
 }
 0x1039ae3 => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x1039aeb; continue; }
  pc = 0x1039ae9; continue;
 }
 0x1039ae9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039aeb => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1039acd; continue; }
  pc = 0x1039af1; continue;
 }
 0x1039af1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039af3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039af3); }
 let mut pc: u32 = 0x1039af3;
 loop { match pc {
 0x1039af3 => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1039afd; continue;
 }
 0x1039afd => {
  { cpu.lods(2, false); }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x1039b0b; continue; }
  pc = 0x1039b04; continue;
 }
 0x1039b04 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x26u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039afd; continue; }
  pc = 0x1039b09; continue;
 }
 0x1039b09 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039b0b => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039b16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039b16); }
 let mut pc: u32 = 0x1039b16;
 loop { match pc {
 0x1039b16 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1039b25; continue;
 }
 0x1039b25 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1039b2d; continue; }
  pc = 0x1039b2c; continue;
 }
 0x1039b2c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1039b2d; continue;
 }
 0x1039b2d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039b25; continue; }
  pc = 0x1039b32; continue;
 }
 0x1039b32 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039b33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039b33); }
 let mut pc: u32 = 0x1039b33;
 loop { match pc {
 0x1039b33 => {
  { let v: u32 = 0x11a188cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1039b42; continue;
 }
 0x1039b42 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039b42; continue; }
  pc = 0x1039b49; continue;
 }
 0x1039b49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039b4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039b4a); }
 let mut pc: u32 = 0x1039b4a;
 loop { match pc {
 0x1039b4a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c5c7bu32); }
  if !cpu.zf { pc = 0x1039b5d; continue; }
  pc = 0x1039b53; continue;
 }
 0x1039b53 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1039b5d; continue; }
  pc = 0x1039b59; continue;
 }
 0x1039b59 => {
  { let v: u8 = 0xffu8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x1039b5d; continue;
 }
 0x1039b5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039b5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039b5e); }
 let mut pc: u32 = 0x1039b5e;
 loop { match pc {
 0x1039b5e => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1039b67; continue; }
  pc = 0x1039b67; continue;
 }
 0x1039b67 => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x1039b77; continue;
 }
 0x1039b77 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x1039b87; continue; }
  pc = 0x1039b7e; continue;
 }
 0x1039b7e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1039b85); f_01039b93(cpu); if cpu.returned_to != 0x1039b85 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1039b87; continue;
 }
 0x1039b87 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1039b77; continue; }
  pc = 0x1039b8c; continue;
 }
 0x1039b8c => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039b93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039b93); }
 let mut pc: u32 = 0x1039b93;
 loop { match pc {
 0x1039b93 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x1fu32.wrapping_add(cpu.esi))); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x1039b9d; continue; }
  pc = 0x1039b9a; continue;
 }
 0x1039b9a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  pc = 0x1039b9d; continue;
 }
 0x1039b9d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x145fu32); }
  if cpu.zf { pc = 0x1039d08; continue; }
  pc = 0x1039bb1; continue;
 }
 0x1039bb1 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1039bd2; continue; }
  pc = 0x1039bba; continue;
 }
 0x1039bba => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1039bd2; continue; }
  pc = 0x1039bc2; continue;
 }
 0x1039bc2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1039bde; continue; }
  pc = 0x1039bd2; continue;
 }
 0x1039bd2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1039d08; continue;
 }
 0x1039bde => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1039c31; continue; }
  pc = 0x1039be7; continue;
 }
 0x1039be7 => {
  { cpu.sub32(cpu.eax, 0x22du32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039bee; continue;
 }
 0x1039bee => {
  { cpu.sub32(cpu.eax, 0x1d5u32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039bf5; continue;
 }
 0x1039bf5 => {
  { cpu.sub32(cpu.eax, 0x16ccu32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039bfc; continue;
 }
 0x1039bfc => {
  { cpu.sub32(cpu.eax, 0x16d0u32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039c03; continue;
 }
 0x1039c03 => {
  { cpu.sub32(cpu.eax, 0x16d4u32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039c0a; continue;
 }
 0x1039c0a => {
  { cpu.sub32(cpu.eax, 0x16d8u32); }
  if cpu.zf { pc = 0x1039c18; continue; }
  pc = 0x1039c11; continue;
 }
 0x1039c11 => {
  { cpu.sub32(cpu.eax, 0x16deu32); }
  if !cpu.zf { pc = 0x1039c1f; continue; }
  pc = 0x1039c18; continue;
 }
 0x1039c18 => {
  cpu.push(0x1039c1d); f_01039f6f(cpu); if cpu.returned_to != 0x1039c1d { return; }
  pc = 0x1039c31; continue;
 }
 0x1039c1f => {
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x1039c2d; continue; }
  pc = 0x1039c27; continue;
 }
 0x1039c27 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x1039c31; continue; }
  pc = 0x1039c2d; continue;
 }
 0x1039c2d => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x1039c31; continue;
 }
 0x1039c31 => {
  cpu.push(0x1039c36); f_0103a097(cpu); if cpu.returned_to != 0x1039c36 { return; }
  if cpu.cf { pc = 0x1039d08; continue; }
  pc = 0x1039c3c; continue;
 }
 0x1039c3c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffe0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1039d08; continue; }
  pc = 0x1039c4e; continue;
 }
 0x1039c4e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1039d08; continue; }
  pc = 0x1039c5a; continue;
 }
 0x1039c5a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffe0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1039d08; continue; }
  pc = 0x1039c6f; continue;
 }
 0x1039c6f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1039d08; continue; }
  pc = 0x1039c7b; continue;
 }
 0x1039c7b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1039cc0; continue; }
  pc = 0x1039c8d; continue;
 }
 0x1039c8d => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xb7eu16); }
  if cpu.zf { pc = 0x1039cb5; continue; }
  pc = 0x1039c99; continue;
 }
 0x1039c99 => {
  { cpu.sub16((cpu.ecx as u16), 0xb7cu16); }
  if cpu.zf { pc = 0x1039cb5; continue; }
  pc = 0x1039ca0; continue;
 }
 0x1039ca0 => {
  { cpu.sub16((cpu.ecx as u16), 0xb7du16); }
  if cpu.zf { pc = 0x1039cb5; continue; }
  pc = 0x1039ca7; continue;
 }
 0x1039ca7 => {
  { cpu.sub16((cpu.ecx as u16), 0xb7fu16); }
  if cpu.zf { pc = 0x1039cb5; continue; }
  pc = 0x1039cae; continue;
 }
 0x1039cae => {
  { cpu.sub16((cpu.ecx as u16), 0xb80u16); }
  if !cpu.zf { pc = 0x1039cb9; continue; }
  pc = 0x1039cb5; continue;
 }
 0x1039cb5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1039cb9; continue;
 }
 0x1039cb9 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1039cbf); cpu.call(t); if cpu.returned_to != 0x1039cbf { return; }
  { cpu.popad(); }
  pc = 0x1039cc0; continue;
 }
 0x1039cc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x21u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1e0u16); }
  if cpu.zf { pc = 0x1039d08; continue; }
  pc = 0x1039cd5; continue;
 }
 0x1039cd5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x1124ef0u32), 0x1u8); }
  if !cpu.zf { pc = 0x1039cee; continue; }
  pc = 0x1039ce2; continue;
 }
 0x1039ce2 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x1748u16); }
  if cpu.zf { pc = 0x1039d73; continue; }
  pc = 0x1039cee; continue;
 }
 0x1039cee => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x1039d09; continue; }
  pc = 0x1039cf4; continue;
 }
 0x1039cf4 => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x1039d08; continue;
 }
 0x1039d08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039d09 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ebx as u16), 0x1673u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d22; continue;
 }
 0x1039d22 => {
  { cpu.sub16((cpu.ebx as u16), 0x1637u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d2d; continue;
 }
 0x1039d2d => {
  { cpu.sub16((cpu.ebx as u16), 0x200u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d38; continue;
 }
 0x1039d38 => {
  { cpu.sub16((cpu.ebx as u16), 0x14c5u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d43; continue;
 }
 0x1039d43 => {
  { cpu.sub16((cpu.ebx as u16), 0x163fu16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d4e; continue;
 }
 0x1039d4e => {
  { cpu.sub16((cpu.ebx as u16), 0x1638u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d59; continue;
 }
 0x1039d59 => {
  { cpu.sub16((cpu.ebx as u16), 0x1640u16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d64; continue;
 }
 0x1039d64 => {
  { cpu.sub16((cpu.ebx as u16), 0x163cu16); }
  if cpu.zf { pc = 0x1039e25; continue; }
  pc = 0x1039d6f; continue;
 }
 0x1039d6f => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039d73 => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x124u16; cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1039dc2); f_0108c1c5(cpu); if cpu.returned_to != 0x1039dc2 { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e4138u32, v); }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x1039ddb; continue; }
  pc = 0x1039dd9; continue;
 }
 0x1039dd9 => {
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1039ddb; continue;
 }
 0x1039ddb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11b5420u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11cc287u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x125u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x18u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1039e25 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x1637u16); }
  if !cpu.zf { pc = 0x1039e44; continue; }
  pc = 0x1039e41; continue;
 }
 0x1039e41 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x1039e44; continue;
 }
 0x1039e44 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3d07u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x6f6u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xe900u16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee92du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x708u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xff00u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1124f13u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x10e3d07u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6fau32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x10e3d07u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6fau32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xfe00u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x10e3d07u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6fau32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.pushad(); }
  cpu.push(0x1039f40); f_0105f178(cpu); if cpu.returned_to != 0x1039f40 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1039f67; continue; }
  pc = 0x1039f45; continue;
 }
 0x1039f45 => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  cpu.push(0x1039f67); f_0101c251(cpu); if cpu.returned_to != 0x1039f67 { return; }
  pc = 0x1039f67; continue;
 }
 0x1039f67 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039f69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039f69); }
 let mut pc: u32 = 0x1039f69;
 loop { match pc {
 0x1039f69 => {
  cpu.push(0x1039f6e); f_0105b1d3(cpu); if cpu.returned_to != 0x1039f6e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01039f6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1039f6f); }
 let mut pc: u32 = 0x1039f6f;
 loop { match pc {
 0x1039f6f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1039f80); f_0108c1c5(cpu); if cpu.returned_to != 0x1039f80 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1039f8c; continue; }
  pc = 0x1039f8c; continue;
 }
 0x1039f8c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x22du32); }
  if cpu.zf { pc = 0x1039fb4; continue; }
  pc = 0x1039f97; continue;
 }
 0x1039f97 => {
  { cpu.sub32(cpu.eax, 0x16d8u32); }
  if cpu.zf { pc = 0x1039fb4; continue; }
  pc = 0x1039f9e; continue;
 }
 0x1039f9e => {
  { cpu.sub32(cpu.eax, 0x16deu32); }
  if cpu.zf { pc = 0x1039fb4; continue; }
  pc = 0x1039fa5; continue;
 }
 0x1039fa5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x3u8); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x103a036; continue; }
  pc = 0x1039fb2; continue;
 }
 0x1039fb2 => {
  pc = 0x1039fc3; continue;
 }
 0x1039fb4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.cf { pc = 0x103a036; continue; }
  pc = 0x1039fbd; continue;
 }
 0x1039fbd => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  pc = 0x1039fc3; continue;
 }
 0x1039fc3 => {
  cpu.push(0x1039fc8); f_0105f215(cpu); if cpu.returned_to != 0x1039fc8 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x1039fdb; continue; }
  pc = 0x1039fcf; continue;
 }
 0x1039fcf => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1039fdb); f_0106e1e0(cpu); if cpu.returned_to != 0x1039fdb { return; }
  pc = 0x1039fdb; continue;
 }
 0x1039fdb => {
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124f58u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x1124f57u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x112c964u32, v); }
  cpu.push(0x1039ffc); f_0103a03f(cpu); if cpu.returned_to != 0x1039ffc { return; }
  cpu.push(0x103a001); f_01074f8f(cpu); if cpu.returned_to != 0x103a001 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c964u32, v); }
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if cpu.zf { pc = 0x103a036; continue; }
  pc = 0x103a015; continue;
 }
 0x103a015 => {
  pc = 0x103a037; continue;
 }
 0x103a025 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x103a036); f_0106e1e0(cpu); if cpu.returned_to != 0x103a036 { return; }
  pc = 0x103a036; continue;
 }
 0x103a036 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a037 => {
  { let v: u8 = cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0x8u8); cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { cpu.poll(); pc = 0x103a036; continue; }
  pc = 0x103a03d; continue;
 }
 0x103a03d => {
  cpu.poll(); pc = 0x103a025; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a03f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a03f); }
 let mut pc: u32 = 0x103a03f;
 loop { match pc {
 0x103a03f => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b5432u32, v); }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x103a056); f_01018e6e(cpu); if cpu.returned_to != 0x103a056 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  if cpu.cf { pc = 0x103a068; continue; }
  pc = 0x103a05f; continue;
 }
 0x103a05f => {
  cpu.push(0x103a064); f_0101ac6c(cpu); if cpu.returned_to != 0x103a064 { return; }
  if cpu.cf { pc = 0x103a068; continue; }
  pc = 0x103a066; continue;
 }
 0x103a066 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a068 => {
  { cpu.popad(); }
  { let v: u32 = 0x1u32; cpu.w32(0x11b5432u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x10fd4cdu32); cpu.mul8(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x103a091; continue; }
  pc = 0x103a08d; continue;
 }
 0x103a08d => {
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103a091; continue;
 }
 0x103a091 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a097); }
 let mut pc: u32 = 0x103a097;
 loop { match pc {
 0x103a097 => {
  { cpu.sub16(cpu.r16(0x23u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x103a0de; continue; }
  pc = 0x103a09e; continue;
 }
 0x103a09e => {
  { let v: u16 = cpu.dec16(cpu.r16(0x23u32.wrapping_add(cpu.esi))); cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x103a0de; continue; }
  pc = 0x103a0a4; continue;
 }
 0x103a0a4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124f12u32, v); }
  { let v: u32 = 0x50u32; cpu.edx = v; }
  cpu.push(0x103a0c8); f_010738df(cpu); if cpu.returned_to != 0x103a0c8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1124f12u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a0de => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a0e0); }
 let mut pc: u32 = 0x103a0e0;
 loop { match pc {
 0x103a0e0 => {
  { let v: u32 = 0x11a7ef0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x11a5ef0u32; cpu.edi = v; }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103a105; continue;
 }
 0x103a105 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x103a124; continue; }
  pc = 0x103a10a; continue;
 }
 0x103a10a => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x103a124; continue; }
  pc = 0x103a117; continue;
 }
 0x103a117 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x103a124; continue;
 }
 0x103a124 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103a105; continue; }
  pc = 0x103a12c; continue;
 }
 0x103a12c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a173(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a173); }
 let mut pc: u32 = 0x103a173;
 loop { match pc {
 0x103a173 => {
  cpu.push(0x103a178); f_0103b555(cpu); if cpu.returned_to != 0x103a178 { return; }
  cpu.push(0x103a17d); f_0103b555(cpu); if cpu.returned_to != 0x103a17d { return; }
  cpu.push(0x103a182); f_0103b555(cpu); if cpu.returned_to != 0x103a182 { return; }
  { let v: u32 = 0x103a18du32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a18d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a18d); }
 let mut pc: u32 = 0x103a18d;
 loop { match pc {
 0x103a18d => {
  cpu.push(0x103a192); f_0103b555(cpu); if cpu.returned_to != 0x103a192 { return; }
  cpu.push(0x103a197); f_0103b555(cpu); if cpu.returned_to != 0x103a197 { return; }
  cpu.push(0x103a19c); f_0103b569(cpu); if cpu.returned_to != 0x103a19c { return; }
  { let v: u32 = 0x103a1a7u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a150(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a150); } r_0103a150(cpu, 0x103a150); }
pub fn f_0103a1a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a1a7); } r_0103a150(cpu, 0x103a1a7); }
pub fn f_0103a7f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a7f1); } r_0103a150(cpu, 0x103a7f1); }
fn r_0103a150(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103a130 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef338u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103a388; continue; }
  pc = 0x103a14a; continue;
 }
 0x103a14a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef338u32)); cpu.w8(0x10ef338u32, v); }
  pc = 0x103a150; continue;
 }
 0x103a150 => {
  cpu.push(0x103a155); f_0103a310(cpu); if cpu.returned_to != 0x103a155 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.push(0x103a15e); f_0103b555(cpu); if cpu.returned_to != 0x103a15e { return; }
  cpu.push(0x103a163); f_0103b555(cpu); if cpu.returned_to != 0x103a163 { return; }
  cpu.push(0x103a168); f_0103b555(cpu); if cpu.returned_to != 0x103a168 { return; }
  { let v: u32 = 0x103a173u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a1a7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103a388; continue; }
  pc = 0x103a1b1; continue;
 }
 0x103a1b1 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x103a1c3); f_0103b699(cpu); if cpu.returned_to != 0x103a1c3 { return; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x103a1dbu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a2a0 => {
  cpu.push(0x103a2a5); f_0103a2ba(cpu); if cpu.returned_to != 0x103a2a5 { return; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a388 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x103a39a); f_0103b699(cpu); if cpu.returned_to != 0x103a39a { return; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  if cpu.cf { cpu.poll(); pc = 0x103a2a0; continue; }
  pc = 0x103a3ad; continue;
 }
 0x103a3ad => {
  { let v: u32 = 0x103a3b8u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a7f1 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x103a92b; continue; }
  pc = 0x103a801; continue;
 }
 0x103a801 => {
  { let v: u32 = 0x1125060u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103a80d; continue;
 }
 0x103a80d => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x103a81d; continue; }
  pc = 0x103a813; continue;
 }
 0x103a813 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.cf { pc = 0x103a81d; continue; }
  pc = 0x103a819; continue;
 }
 0x103a819 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x103a81d; continue;
 }
 0x103a81d => {
  { let v: u32 = cpu.esi.wrapping_add(0x64u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103a80d; continue; }
  pc = 0x103a822; continue;
 }
 0x103a822 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x103a865; continue; }
  pc = 0x103a826; continue;
 }
 0x103a826 => {
  { cpu.sub8(cpu.r8(0x10ef338u32), 0x7fu8); }
  if cpu.zf { cpu.poll(); pc = 0x103a130; continue; }
  pc = 0x103a833; continue;
 }
 0x103a833 => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103a83c); f_0103c55d(cpu); if cpu.returned_to != 0x103a83c { return; }
  if cpu.cf { pc = 0x103a864; continue; }
  pc = 0x103a83e; continue;
 }
 0x103a83e => {
  cpu.push(0x103a843); f_0103aa31(cpu); if cpu.returned_to != 0x103a843 { return; }
  if cpu.cf { pc = 0x103a85f; continue; }
  pc = 0x103a845; continue;
 }
 0x103a845 => {
  { cpu.sub32(cpu.r32(0x10e40e4u32), 0x0u32); }
  if cpu.zf { pc = 0x103a9c5; continue; }
  pc = 0x103a852; continue;
 }
 0x103a852 => {
  { cpu.sub32(cpu.r32(0x10e40e0u32), 0x0u32); }
  if cpu.zf { pc = 0x103a97c; continue; }
  pc = 0x103a85f; continue;
 }
 0x103a85f => {
  cpu.push(0x103a864); f_0103a2ba(cpu); if cpu.returned_to != 0x103a864 { return; }
  pc = 0x103a864; continue;
 }
 0x103a864 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a865 => {
  cpu.push(0x103a86a); f_0103aaf5(cpu); if cpu.returned_to != 0x103a86a { return; }
  cpu.push(0x103a86f); f_0103b256(cpu); if cpu.returned_to != 0x103a86f { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x10f5870u32; cpu.w32(0x10e41e8u32, v); }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103a890; continue; }
  pc = 0x103a886; continue;
 }
 0x103a886 => {
  { let v: u32 = 0x10f58b3u32; cpu.w32(0x10e41e8u32, v); }
  pc = 0x103a890; continue;
 }
 0x103a890 => {
  cpu.push(0x103a895); f_0103bb2b(cpu); if cpu.returned_to != 0x103a895 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x103a932; continue; }
  pc = 0x103a89c; continue;
 }
 0x103a89c => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x10d10dbu32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x103a8b5; continue;
 }
 0x103a8b5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x103a926; continue; }
  pc = 0x103a8bf; continue;
 }
 0x103a8bf => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x103a8c8; continue; }
  pc = 0x103a8c4; continue;
 }
 0x103a8c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103a8ca; continue;
 }
 0x103a8c8 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x103a8ca; continue;
 }
 0x103a8ca => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(2, false); }
  { let v: u32 = cpu.eax.wrapping_add(0x10ea910u32); cpu.eax = v; }
  cpu.push(0x103a8da); f_0103b3e9(cpu); if cpu.returned_to != 0x103a8da { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x103a926; continue; }
  pc = 0x103a8ea; continue;
 }
 0x103a8ea => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0x7du8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x64u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103a90e; continue;
 }
 0x103a90e => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x103a925; continue; }
  pc = 0x103a914; continue;
 }
 0x103a914 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x103a925; continue;
 }
 0x103a925 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a926 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x103a930; continue; }
  pc = 0x103a92b; continue;
 }
 0x103a92b => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a930 => {
  cpu.poll(); pc = 0x103a8b5; continue;
 }
 0x103a932 => {
  cpu.push(0x103a937); f_0103a9e1(cpu); if cpu.returned_to != 0x103a937 { return; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x2u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x103a95d); f_0100b130(cpu); if cpu.returned_to != 0x103a95d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x103a97b; continue; }
  pc = 0x103a968; continue;
 }
 0x103a968 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x103a97b; continue;
 }
 0x103a97b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a97c => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x103a9dc; continue; }
  pc = 0x103a984; continue;
 }
 0x103a984 => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103a98d; continue;
 }
 0x103a98d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10d10dbu32; cpu.edi = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x103a99f; continue;
 }
 0x103a99f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x103a9ab; continue; }
  pc = 0x103a9a5; continue;
 }
 0x103a9a5 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103a99f; continue; }
  pc = 0x103a9aa; continue;
 }
 0x103a9aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a9ab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x103a90e; continue;
 }
 0x103a9c5 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0xfu8); }
  if cpu.zf { pc = 0x103a9dc; continue; }
  pc = 0x103a9d2; continue;
 }
 0x103a9d2 => {
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x103a98d; continue;
 }
 0x103a9dc => {
  cpu.poll(); pc = 0x103a85f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a1db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a1db); } r_0103a1db(cpu, 0x103a1db); }
pub fn f_0103a1f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a1f4); } r_0103a1db(cpu, 0x103a1f4); }
pub fn f_0103a21c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a21c); } r_0103a1db(cpu, 0x103a21c); }
pub fn f_0103a244(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a244); } r_0103a1db(cpu, 0x103a244); }
pub fn f_0103a26c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a26c); } r_0103a1db(cpu, 0x103a26c); }
fn r_0103a1db(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103a1db => {
  cpu.push(0x103a1e0); f_0103a4bd(cpu); if cpu.returned_to != 0x103a1e0 { return; }
  if cpu.cf { pc = 0x103a203; continue; }
  pc = 0x103a1e2; continue;
 }
 0x103a1e2 => {
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a1e9; continue;
 }
 0x103a1e9 => {
  { let v: u32 = 0x103a1f4u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a1f4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a203; continue; }
  pc = 0x103a1fc; continue;
 }
 0x103a1fc => {
  cpu.push(0x103a201); f_0103ac8e(cpu); if cpu.returned_to != 0x103a201 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a1e9; continue; }
  pc = 0x103a203; continue;
 }
 0x103a203 => {
  cpu.push(0x103a208); f_0103a5fd(cpu); if cpu.returned_to != 0x103a208 { return; }
  if cpu.cf { pc = 0x103a22b; continue; }
  pc = 0x103a20a; continue;
 }
 0x103a20a => {
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a211; continue;
 }
 0x103a211 => {
  { let v: u32 = 0x103a21cu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a21c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a22b; continue; }
  pc = 0x103a224; continue;
 }
 0x103a224 => {
  cpu.push(0x103a229); f_0103ac8e(cpu); if cpu.returned_to != 0x103a229 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a211; continue; }
  pc = 0x103a22b; continue;
 }
 0x103a22b => {
  cpu.push(0x103a230); f_0103abde(cpu); if cpu.returned_to != 0x103a230 { return; }
  if cpu.cf { pc = 0x103a253; continue; }
  pc = 0x103a232; continue;
 }
 0x103a232 => {
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a239; continue;
 }
 0x103a239 => {
  { let v: u32 = 0x103a244u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a244 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a253; continue; }
  pc = 0x103a24c; continue;
 }
 0x103a24c => {
  cpu.push(0x103a251); f_0103ac8e(cpu); if cpu.returned_to != 0x103a251 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a239; continue; }
  pc = 0x103a253; continue;
 }
 0x103a253 => {
  cpu.push(0x103a258); f_0103ac36(cpu); if cpu.returned_to != 0x103a258 { return; }
  if cpu.cf { pc = 0x103a27b; continue; }
  pc = 0x103a25a; continue;
 }
 0x103a25a => {
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a261; continue;
 }
 0x103a261 => {
  { let v: u32 = 0x103a26cu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a26c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a27b; continue; }
  pc = 0x103a274; continue;
 }
 0x103a274 => {
  cpu.push(0x103a279); f_0103ac8e(cpu); if cpu.returned_to != 0x103a279 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a261; continue; }
  pc = 0x103a27b; continue;
 }
 0x103a27b => {
  { let v: u32 = 0x103a286u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a286); }
 let mut pc: u32 = 0x103a286;
 loop { match pc {
 0x103a286 => {
  cpu.push(0x103a28b); f_0103a7f1(cpu); if cpu.returned_to != 0x103a28b { return; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a2ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a2ba); }
 let mut pc: u32 = 0x103a2ba;
 loop { match pc {
 0x103a2ba => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103a2de; continue; }
  pc = 0x103a2ca; continue;
 }
 0x103a2ca => {
  if !cpu.sf { pc = 0x103a2fe; continue; }
  pc = 0x103a2cc; continue;
 }
 0x103a2cc => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x103a2de); f_01047379(cpu); if cpu.returned_to != 0x103a2de { return; }
  pc = 0x103a2de; continue;
 }
 0x103a2de => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103a2e7); f_0106a326(cpu); if cpu.returned_to != 0x103a2e7 { return; }
  cpu.push(0x103a2ec); f_010675fd(cpu); if cpu.returned_to != 0x103a2ec { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103a2fd; continue; }
  pc = 0x103a2f2; continue;
 }
 0x103a2f2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.eax)) & 0xfcu8; cpu.w8(0x17u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103a2fd; continue;
 }
 0x103a2fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a2fe => {
  { let v: u8 = cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103a30a); f_0106a326(cpu); if cpu.returned_to != 0x103a30a { return; }
  cpu.push(0x103a30f); f_010675fd(cpu); if cpu.returned_to != 0x103a30f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a310); }
 let mut pc: u32 = 0x103a310;
 loop { match pc {
 0x103a310 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103a325; continue; }
  pc = 0x103a31c; continue;
 }
 0x103a31c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124f5cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a325 => {
  { let v: u8 = cpu.r8(0xaeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xaeu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xaeu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103a363; continue; }
  pc = 0x103a33e; continue;
 }
 0x103a33e => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1124f5cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a363 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1124f5cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a3b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a3b8); } r_0103a3b8(cpu, 0x103a3b8); }
pub fn f_0103a3e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a3e0); } r_0103a3b8(cpu, 0x103a3e0); }
pub fn f_0103a418(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a418); } r_0103a3b8(cpu, 0x103a418); }
pub fn f_0103a450(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a450); } r_0103a3b8(cpu, 0x103a450); }
pub fn f_0103a488(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103a488); } r_0103a3b8(cpu, 0x103a488); }
fn r_0103a3b8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103a3b8 => {
  cpu.push(0x103a3bd); f_0103a51d(cpu); if cpu.returned_to != 0x103a3bd { return; }
  if cpu.cf { pc = 0x103a3f0; continue; }
  pc = 0x103a3bf; continue;
 }
 0x103a3bf => {
  { let v: u32 = 0x103ad4eu32; cpu.w32(0x10e40ccu32, v); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a3d5; continue;
 }
 0x103a3d5 => {
  { let v: u32 = 0x103a3e0u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a3e0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a3f0; continue; }
  pc = 0x103a3e8; continue;
 }
 0x103a3e8 => {
  let t = cpu.r32(0x10e40ccu32); cpu.push(0x103a3ee); cpu.call(t); if cpu.returned_to != 0x103a3ee { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a3d5; continue; }
  pc = 0x103a3f0; continue;
 }
 0x103a3f0 => {
  cpu.push(0x103a3f5); f_0103a555(cpu); if cpu.returned_to != 0x103a3f5 { return; }
  if cpu.cf { pc = 0x103a428; continue; }
  pc = 0x103a3f7; continue;
 }
 0x103a3f7 => {
  { let v: u32 = 0x103ae72u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a40d; continue;
 }
 0x103a40d => {
  { let v: u32 = 0x103a418u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a418 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a428; continue; }
  pc = 0x103a420; continue;
 }
 0x103a420 => {
  let t = cpu.r32(0x10e40ccu32); cpu.push(0x103a426); cpu.call(t); if cpu.returned_to != 0x103a426 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a40d; continue; }
  pc = 0x103a428; continue;
 }
 0x103a428 => {
  cpu.push(0x103a42d); f_0103a58d(cpu); if cpu.returned_to != 0x103a42d { return; }
  if cpu.cf { pc = 0x103a460; continue; }
  pc = 0x103a42f; continue;
 }
 0x103a42f => {
  { let v: u32 = 0x103af97u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a445; continue;
 }
 0x103a445 => {
  { let v: u32 = 0x103a450u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a450 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a460; continue; }
  pc = 0x103a458; continue;
 }
 0x103a458 => {
  let t = cpu.r32(0x10e40ccu32); cpu.push(0x103a45e); cpu.call(t); if cpu.returned_to != 0x103a45e { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a445; continue; }
  pc = 0x103a460; continue;
 }
 0x103a460 => {
  cpu.push(0x103a465); f_0103a5c5(cpu); if cpu.returned_to != 0x103a465 { return; }
  if cpu.cf { pc = 0x103a498; continue; }
  pc = 0x103a467; continue;
 }
 0x103a467 => {
  { let v: u32 = 0x103b0b6u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x10ef329u32, v); }
  pc = 0x103a47d; continue;
 }
 0x103a47d => {
  { let v: u32 = 0x103a488u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a488 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef329u32)); cpu.w8(0x10ef329u32, v); }
  if cpu.zf { pc = 0x103a498; continue; }
  pc = 0x103a490; continue;
 }
 0x103a490 => {
  let t = cpu.r32(0x10e40ccu32); cpu.push(0x103a496); cpu.call(t); if cpu.returned_to != 0x103a496 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103a47d; continue; }
  pc = 0x103a498; continue;
 }
 0x103a498 => {
  { let v: u32 = 0x103a4a3u32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a4a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a4a3); }
 let mut pc: u32 = 0x103a4a3;
 loop { match pc {
 0x103a4a3 => {
  cpu.push(0x103a4a8); f_0103a7f1(cpu); if cpu.returned_to != 0x103a4a8 { return; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e3fbcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a4bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a4bd); }
 let mut pc: u32 = 0x103a4bd;
 loop { match pc {
 0x103a4bd => {
  cpu.push(0x103a4c2); f_0103ab70(cpu); if cpu.returned_to != 0x103a4c2 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x1125060u32, v); }
  { let v: u32 = 0x1125060u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x1125062u32; cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef323u32, v); }
  cpu.push(0x103a4eb); f_0103a65d(cpu); if cpu.returned_to != 0x103a4eb { return; }
  if !cpu.cf { pc = 0x103a50f; continue; }
  pc = 0x103a4ed; continue;
 }
 0x103a4ed => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef323u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  cpu.push(0x103a4ff); f_0103a65d(cpu); if cpu.returned_to != 0x103a4ff { return; }
  if !cpu.cf { pc = 0x103a50f; continue; }
  pc = 0x103a501; continue;
 }
 0x103a501 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x100u32); cpu.esi = v; }
  cpu.push(0x103a50c); f_0103a65d(cpu); if cpu.returned_to != 0x103a50c { return; }
  if !cpu.cf { pc = 0x103a50f; continue; }
  pc = 0x103a50e; continue;
 }
 0x103a50e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a50f => {
  cpu.push(0x103a514); f_0103aba3(cpu); if cpu.returned_to != 0x103a514 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a51d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a51d); }
 let mut pc: u32 = 0x103a51d;
 loop { match pc {
 0x103a51d => {
  cpu.push(0x103a522); f_0103ab70(cpu); if cpu.returned_to != 0x103a522 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x1125060u32, v); }
  { let v: u32 = 0x1125060u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x1125062u32; cpu.w32(0x10e40d4u32, v); }
  cpu.push(0x103a544); f_0103a789(cpu); if cpu.returned_to != 0x103a544 { return; }
  if !cpu.cf { pc = 0x103a547; continue; }
  pc = 0x103a546; continue;
 }
 0x103a546 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a547 => {
  cpu.push(0x103a54c); f_0103aba3(cpu); if cpu.returned_to != 0x103a54c { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a555(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a555); }
 let mut pc: u32 = 0x103a555;
 loop { match pc {
 0x103a555 => {
  cpu.push(0x103a55a); f_0103ab70(cpu); if cpu.returned_to != 0x103a55a { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x11250c4u32, v); }
  { let v: u32 = 0x11250c4u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x11250c6u32; cpu.w32(0x10e40d4u32, v); }
  cpu.push(0x103a57c); f_0103a789(cpu); if cpu.returned_to != 0x103a57c { return; }
  if !cpu.cf { pc = 0x103a57f; continue; }
  pc = 0x103a57e; continue;
 }
 0x103a57e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a57f => {
  cpu.push(0x103a584); f_0103aba3(cpu); if cpu.returned_to != 0x103a584 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a58d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a58d); }
 let mut pc: u32 = 0x103a58d;
 loop { match pc {
 0x103a58d => {
  cpu.push(0x103a592); f_0103ab70(cpu); if cpu.returned_to != 0x103a592 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x1125128u32, v); }
  { let v: u32 = 0x1125128u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x112512au32; cpu.w32(0x10e40d4u32, v); }
  cpu.push(0x103a5b4); f_0103a7b4(cpu); if cpu.returned_to != 0x103a5b4 { return; }
  if !cpu.cf { pc = 0x103a5b7; continue; }
  pc = 0x103a5b6; continue;
 }
 0x103a5b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a5b7 => {
  cpu.push(0x103a5bc); f_0103aba3(cpu); if cpu.returned_to != 0x103a5bc { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a5c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a5c5); }
 let mut pc: u32 = 0x103a5c5;
 loop { match pc {
 0x103a5c5 => {
  cpu.push(0x103a5ca); f_0103ab70(cpu); if cpu.returned_to != 0x103a5ca { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x112518cu32, v); }
  { let v: u32 = 0x112518cu32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x112518eu32; cpu.w32(0x10e40d4u32, v); }
  cpu.push(0x103a5ec); f_0103a7b4(cpu); if cpu.returned_to != 0x103a5ec { return; }
  if !cpu.cf { pc = 0x103a5ef; continue; }
  pc = 0x103a5ee; continue;
 }
 0x103a5ee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a5ef => {
  cpu.push(0x103a5f4); f_0103aba3(cpu); if cpu.returned_to != 0x103a5f4 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a5fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a5fd); }
 let mut pc: u32 = 0x103a5fd;
 loop { match pc {
 0x103a5fd => {
  cpu.push(0x103a602); f_0103ab70(cpu); if cpu.returned_to != 0x103a602 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x11250c4u32, v); }
  { let v: u32 = 0x11250c4u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x11250c6u32; cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef323u32, v); }
  cpu.push(0x103a62b); f_0103a65d(cpu); if cpu.returned_to != 0x103a62b { return; }
  if !cpu.cf { pc = 0x103a64f; continue; }
  pc = 0x103a62d; continue;
 }
 0x103a62d => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef323u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  cpu.push(0x103a63f); f_0103a65d(cpu); if cpu.returned_to != 0x103a63f { return; }
  if !cpu.cf { pc = 0x103a64f; continue; }
  pc = 0x103a641; continue;
 }
 0x103a641 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x100u32); cpu.esi = v; }
  cpu.push(0x103a64c); f_0103a65d(cpu); if cpu.returned_to != 0x103a64c { return; }
  if !cpu.cf { pc = 0x103a64f; continue; }
  pc = 0x103a64e; continue;
 }
 0x103a64e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a64f => {
  cpu.push(0x103a654); f_0103aba3(cpu); if cpu.returned_to != 0x103a654 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a65d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a65d); }
 let mut pc: u32 = 0x103a65d;
 loop { match pc {
 0x103a65d => {
  { let v: u32 = cpu.r32(0x10e40e0u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x103a6c4; continue; }
  pc = 0x103a66b; continue;
 }
 0x103a66b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x103a6d9; continue; }
  pc = 0x103a676; continue;
 }
 0x103a676 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x103a6e4; continue; }
  pc = 0x103a681; continue;
 }
 0x103a681 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103a6a8; continue; }
  pc = 0x103a694; continue;
 }
 0x103a694 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x81u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a6c6; continue; }
  pc = 0x103a69f; continue;
 }
 0x103a69f => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a6c6; continue; }
  pc = 0x103a6a8; continue;
 }
 0x103a6a8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103a6c4; continue; }
  pc = 0x103a6b2; continue;
 }
 0x103a6b2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7fu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a6d2; continue; }
  pc = 0x103a6bb; continue;
 }
 0x103a6bb => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a6d2; continue; }
  pc = 0x103a6c4; continue;
 }
 0x103a6c4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a6c6 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  pc = 0x103a6c9; continue;
 }
 0x103a6c9 => {
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x103a6d0); f_0103b1e1(cpu); if cpu.returned_to != 0x103a6d0 { return; }
  pc = 0x103a6d7; continue;
 }
 0x103a6d2 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.poll(); pc = 0x103a6c9; continue;
 }
 0x103a6d7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a6d9 => {
  { cpu.sub8(cpu.r8(0x10ef323u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x103a6d7; continue; }
  pc = 0x103a6e2; continue;
 }
 0x103a6e2 => {
  cpu.poll(); pc = 0x103a6c6; continue;
 }
 0x103a6e4 => {
  { cpu.sub8(cpu.r8(0x10ef323u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x103a6d7; continue; }
  pc = 0x103a6ed; continue;
 }
 0x103a6ed => {
  cpu.poll(); pc = 0x103a6d2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a6ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a6ef); }
 let mut pc: u32 = 0x103a6ef;
 loop { match pc {
 0x103a6ef => {
  { let v: u32 = cpu.r32(0x10e40e4u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x103a75b; continue; }
  pc = 0x103a6fd; continue;
 }
 0x103a6fd => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x103a773; continue; }
  pc = 0x103a70c; continue;
 }
 0x103a70c => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x103a77e; continue; }
  pc = 0x103a719; continue;
 }
 0x103a719 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103a73e; continue; }
  pc = 0x103a72c; continue;
 }
 0x103a72c => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x81u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a75d; continue; }
  pc = 0x103a737; continue;
 }
 0x103a737 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a75d; continue; }
  pc = 0x103a73e; continue;
 }
 0x103a73e => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103a75b; continue; }
  pc = 0x103a74b; continue;
 }
 0x103a74b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7fu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a769; continue; }
  pc = 0x103a754; continue;
 }
 0x103a754 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x103a769; continue; }
  pc = 0x103a75b; continue;
 }
 0x103a75b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a75d => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.edi); cpu.esi = v; }
  pc = 0x103a760; continue;
 }
 0x103a760 => {
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x103a767); f_0103b1e1(cpu); if cpu.returned_to != 0x103a767 { return; }
  pc = 0x103a771; continue;
 }
 0x103a769 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.poll(); pc = 0x103a760; continue;
 }
 0x103a771 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a773 => {
  { cpu.sub8(cpu.r8(0x10ef323u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x103a771; continue; }
  pc = 0x103a77c; continue;
 }
 0x103a77c => {
  cpu.poll(); pc = 0x103a75d; continue;
 }
 0x103a77e => {
  { cpu.sub8(cpu.r8(0x10ef323u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x103a771; continue; }
  pc = 0x103a787; continue;
 }
 0x103a787 => {
  cpu.poll(); pc = 0x103a769; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a789(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a789); }
 let mut pc: u32 = 0x103a789;
 loop { match pc {
 0x103a789 => {
  { let v: u32 = cpu.r32(0x10e40e0u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x103a7b2; continue; }
  pc = 0x103a793; continue;
 }
 0x103a793 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x10ea910u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xff80u32; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10ea910u32); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a7b2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a7b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a7b4); }
 let mut pc: u32 = 0x103a7b4;
 loop { match pc {
 0x103a7b4 => {
  { let v: u32 = cpu.r32(0x10e40e4u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x103a7ef; continue; }
  pc = 0x103a7be; continue;
 }
 0x103a7be => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x10ea910u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xff80u32; cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10ea910u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x103a7ed; continue; }
  pc = 0x103a7e0; continue;
 }
 0x103a7e0 => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x103a7ed; continue; }
  pc = 0x103a7e7; continue;
 }
 0x103a7e7 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x80u32); cpu.esi = v; }
  pc = 0x103a7ed; continue;
 }
 0x103a7ed => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103a7ef => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103a9e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103a9e1); }
 let mut pc: u32 = 0x103a9e1;
 loop { match pc {
 0x103a9e1 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  cpu.push(0x103a9ec); f_0106a326(cpu); if cpu.returned_to != 0x103a9ec { return; }
  { let v: u32 = 0x11de3c0u32; cpu.esi = v; }
  { let v: u32 = 0x10c7adbu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x103aa00; continue;
 }
 0x103aa00 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x103aa2a; continue; }
  pc = 0x103aa06; continue;
 }
 0x103aa06 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x103aa0f; continue; }
  pc = 0x103aa0b; continue;
 }
 0x103aa0b => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103aa11; continue;
 }
 0x103aa0f => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x103aa11; continue;
 }
 0x103aa11 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x103aa2a; continue; }
  pc = 0x103aa23; continue;
 }
 0x103aa23 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103aa2a => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103aa00; continue; }
  pc = 0x103aa30; continue;
 }
 0x103aa30 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103aa31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103aa31); }
 let mut pc: u32 = 0x103aa31;
 loop { match pc {
 0x103aa31 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e415cu32, v); }
  cpu.push(0x103aa44); f_0103b57b(cpu); if cpu.returned_to != 0x103aa44 { return; }
  { cpu.sub32(cpu.edi, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x103aaf1; continue; }
  pc = 0x103aa4d; continue;
 }
 0x103aa4d => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103aa5a); f_0104f36d(cpu); if cpu.returned_to != 0x103aa5a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x103aaf3; continue; }
  pc = 0x103aa74; continue;
 }
 0x103aa74 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103aa82; continue; }
  pc = 0x103aa7f; continue;
 }
 0x103aa7f => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x103aa82; continue;
 }
 0x103aa82 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  cpu.push(0x103aa8c); f_0104f36d(cpu); if cpu.returned_to != 0x103aa8c { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x103aaf3; continue; }
  pc = 0x103aaa2; continue;
 }
 0x103aaa2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103aaae; continue; }
  pc = 0x103aaab; continue;
 }
 0x103aaab => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x103aaae; continue;
 }
 0x103aaae => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  cpu.push(0x103aab9); f_0104f36d(cpu); if cpu.returned_to != 0x103aab9 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x103aaf3; continue; }
  pc = 0x103aacd; continue;
 }
 0x103aacd => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  cpu.push(0x103aadd); f_0104f36d(cpu); if cpu.returned_to != 0x103aadd { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x103aaf3; continue; }
  pc = 0x103aaf1; continue;
 }
 0x103aaf1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103aaf3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103aaf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103aaf5); }
 let mut pc: u32 = 0x103aaf5;
 loop { match pc {
 0x103aaf5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e40d0u32, v); }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40d4u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ab13); f_0103b57b(cpu); if cpu.returned_to != 0x103ab13 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103ab66; continue; }
  pc = 0x103ab16; continue;
 }
 0x103ab16 => {
  { let v: u16 = (cpu.ecx as u16) & 0xfff0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xfff0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10ea910u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1124f60u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x103ab48; continue;
 }
 0x103ab48 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.sf { pc = 0x103ab6b; continue; }
  pc = 0x103ab55; continue;
 }
 0x103ab55 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103ab48; continue; }
  pc = 0x103ab5a; continue;
 }
 0x103ab5a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ab65); f_0103b1e1(cpu); if cpu.returned_to != 0x103ab65 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x103ab66; continue;
 }
 0x103ab66 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ab6b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = 0x103ab66; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ab70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ab70); }
 let mut pc: u32 = 0x103ab70;
 loop { match pc {
 0x103ab70 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x56u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x58u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x103ab87; continue; }
  pc = 0x103ab82; continue;
 }
 0x103ab82 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10ea910u32); cpu.eax = v; }
  pc = 0x103ab87; continue;
 }
 0x103ab87 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x103ab91; continue; }
  pc = 0x103ab8b; continue;
 }
 0x103ab8b => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x10ea910u32); cpu.ebx = v; }
  pc = 0x103ab91; continue;
 }
 0x103ab91 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e40e0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e40e4u32, v); }
  { let v: u32 = cpu.r32(0x1124f5cu32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103aba3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103aba3); }
 let mut pc: u32 = 0x103aba3;
 loop { match pc {
 0x103aba3 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x10ea910u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103abb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103abb6); }
 let mut pc: u32 = 0x103abb6;
 loop { match pc {
 0x103abb6 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x103abc5; continue; }
  pc = 0x103abbd; continue;
 }
 0x103abbd => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x103abc5; continue; }
  pc = 0x103abc1; continue;
 }
 0x103abc1 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x103abcb; continue; }
  pc = 0x103abc5; continue;
 }
 0x103abc5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  pc = 0x103abcb; continue;
 }
 0x103abcb => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x10ea910u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103abde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103abde); }
 let mut pc: u32 = 0x103abde;
 loop { match pc {
 0x103abde => {
  cpu.push(0x103abe3); f_0103ab70(cpu); if cpu.returned_to != 0x103abe3 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x1125128u32, v); }
  { let v: u32 = 0x1125128u32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x112512au32; cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef323u32, v); }
  cpu.push(0x103ac0c); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac0c { return; }
  if !cpu.cf { pc = 0x103ac28; continue; }
  pc = 0x103ac0e; continue;
 }
 0x103ac0e => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef323u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x103ac1b); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac1b { return; }
  if !cpu.cf { pc = 0x103ac28; continue; }
  pc = 0x103ac1d; continue;
 }
 0x103ac1d => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  cpu.push(0x103ac25); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac25 { return; }
  if !cpu.cf { pc = 0x103ac28; continue; }
  pc = 0x103ac27; continue;
 }
 0x103ac27 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ac28 => {
  cpu.push(0x103ac2d); f_0103aba3(cpu); if cpu.returned_to != 0x103ac2d { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ac36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ac36); }
 let mut pc: u32 = 0x103ac36;
 loop { match pc {
 0x103ac36 => {
  cpu.push(0x103ac3b); f_0103ab70(cpu); if cpu.returned_to != 0x103ac3b { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x112518cu32, v); }
  { let v: u32 = 0x112518cu32; cpu.w32(0x10e40d0u32, v); }
  { let v: u32 = 0x112518eu32; cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef323u32, v); }
  cpu.push(0x103ac64); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac64 { return; }
  if !cpu.cf { pc = 0x103ac80; continue; }
  pc = 0x103ac66; continue;
 }
 0x103ac66 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef323u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x103ac73); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac73 { return; }
  if !cpu.cf { pc = 0x103ac80; continue; }
  pc = 0x103ac75; continue;
 }
 0x103ac75 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  cpu.push(0x103ac7d); f_0103a6ef(cpu); if cpu.returned_to != 0x103ac7d { return; }
  if !cpu.cf { pc = 0x103ac80; continue; }
  pc = 0x103ac7f; continue;
 }
 0x103ac7f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ac80 => {
  cpu.push(0x103ac85); f_0103aba3(cpu); if cpu.returned_to != 0x103ac85 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ac8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ac8e); }
 let mut pc: u32 = 0x103ac8e;
 loop { match pc {
 0x103ac8e => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x10e3e79u32, v); }
  pc = 0x103aca5; continue;
 }
 0x103aca5 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e79u32)); cpu.w16(0x10e3e79u32, v); }
  if !cpu.sf { pc = 0x103acb0; continue; }
  pc = 0x103acae; continue;
 }
 0x103acae => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103acb0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x103acfc; continue; }
  pc = 0x103acc6; continue;
 }
 0x103acc6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x103acfc; continue; }
  pc = 0x103accb; continue;
 }
 0x103accb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103acfe; continue; }
  pc = 0x103acd4; continue;
 }
 0x103acd4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103ad1d; continue; }
  pc = 0x103acdd; continue;
 }
 0x103acdd => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103acec; continue; }
  pc = 0x103ace8; continue;
 }
 0x103ace8 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103acf0; continue; }
  pc = 0x103acec; continue;
 }
 0x103acec => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103aca5; continue; }
  pc = 0x103acee; continue;
 }
 0x103acee => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103acf0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103acf9); f_0103b1e1(cpu); if cpu.returned_to != 0x103acf9 { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103acfc; continue;
 }
 0x103acfc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103acfe => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ad0a); f_0103b1e1(cpu); if cpu.returned_to != 0x103ad0a { return; }
  if cpu.cf { pc = 0x103ad4b; continue; }
  pc = 0x103ad0c; continue;
 }
 0x103ad0c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103aca5; continue; }
  pc = 0x103ad1b; continue;
 }
 0x103ad1b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ad1d => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ad25); f_0103b1e1(cpu); if cpu.returned_to != 0x103ad25 { return; }
  if cpu.cf { pc = 0x103ad4b; continue; }
  pc = 0x103ad27; continue;
 }
 0x103ad27 => {
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
  if cpu.zf { pc = 0x103ad41; continue; }
  pc = 0x103ad3d; continue;
 }
 0x103ad3d => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x103acf0; continue; }
  pc = 0x103ad41; continue;
 }
 0x103ad41 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103aca5; continue; }
  pc = 0x103ad49; continue;
 }
 0x103ad49 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ad4b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ad4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103ad4e); } r_0103ad4e(cpu, 0x103ad4e); }
pub fn f_0103ae72(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103ae72); } r_0103ad4e(cpu, 0x103ae72); }
pub fn f_0103af97(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103af97); } r_0103ad4e(cpu, 0x103af97); }
pub fn f_0103b0b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103b0b6); } r_0103ad4e(cpu, 0x103b0b6); }
fn r_0103ad4e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103ad4e => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x10e3e79u32, v); }
  pc = 0x103ad65; continue;
 }
 0x103ad65 => {
  { let v: u32 = 0x103ad4eu32; cpu.w32(0x10e40ccu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e79u32)); cpu.w16(0x10e3e79u32, v); }
  if cpu.sf { pc = 0x103ae14; continue; }
  pc = 0x103ad7c; continue;
 }
 0x103ad7c => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103ad96; continue;
 }
 0x103ad96 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103ad9f; continue;
 }
 0x103ad9f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103ae16; continue; }
  pc = 0x103adac; continue;
 }
 0x103adac => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103ae16; continue; }
  pc = 0x103adb6; continue;
 }
 0x103adb6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103add0; continue; }
  pc = 0x103adbf; continue;
 }
 0x103adbf => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103ae47; continue; }
  pc = 0x103add0; continue;
 }
 0x103add0 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103addd; continue;
 }
 0x103addd => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103adea; continue;
 }
 0x103adea => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103adfa; continue;
 }
 0x103adfa => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103ae0a; continue;
 }
 0x103ae0a => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103ad65; continue; }
  pc = 0x103ae14; continue;
 }
 0x103ae14 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ae16 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ae22); f_0103b1e1(cpu); if cpu.returned_to != 0x103ae22 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103ae29; continue;
 }
 0x103ae29 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103ae3a; continue; }
  pc = 0x103ae2d; continue;
 }
 0x103ae2d => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x103afae; continue;
 }
 0x103ae3a => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x103b0cd; continue;
 }
 0x103ae47 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ae4d); f_0103b1e1(cpu); if cpu.returned_to != 0x103ae4d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103ae62; continue; }
  pc = 0x103ae52; continue;
 }
 0x103ae52 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x103b0cd; continue;
 }
 0x103ae62 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x103afae; continue;
 }
 0x103ae72 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x10e3e79u32, v); }
  pc = 0x103ae89; continue;
 }
 0x103ae89 => {
  { let v: u32 = 0x103ae72u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e79u32)); cpu.w16(0x10e3e79u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x103ae14; continue; }
  pc = 0x103aea0; continue;
 }
 0x103aea0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103aeba; continue;
 }
 0x103aeba => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103aec3; continue;
 }
 0x103aec3 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103af41; continue; }
  pc = 0x103aed4; continue;
 }
 0x103aed4 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103af41; continue; }
  pc = 0x103aee1; continue;
 }
 0x103aee1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103aefb; continue; }
  pc = 0x103aeea; continue;
 }
 0x103aeea => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103af75; continue; }
  pc = 0x103aefb; continue;
 }
 0x103aefb => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103af08; continue;
 }
 0x103af08 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103af15; continue;
 }
 0x103af15 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103af25; continue;
 }
 0x103af25 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103af35; continue;
 }
 0x103af35 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103ae89; continue; }
  pc = 0x103af3f; continue;
 }
 0x103af3f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103af41 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103af4d); f_0103b1e1(cpu); if cpu.returned_to != 0x103af4d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103af54; continue;
 }
 0x103af54 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103af65; continue; }
  pc = 0x103af58; continue;
 }
 0x103af58 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x103afae; continue;
 }
 0x103af65 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x103b0cd; continue;
 }
 0x103af75 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103af7b); f_0103b1e1(cpu); if cpu.returned_to != 0x103af7b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103af8d; continue; }
  pc = 0x103af80; continue;
 }
 0x103af80 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x103b0cd; continue;
 }
 0x103af8d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x103afae; continue;
 }
 0x103af97 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x10e3e79u32, v); }
  pc = 0x103afae; continue;
 }
 0x103afae => {
  { let v: u32 = 0x103af97u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e79u32)); cpu.w16(0x10e3e79u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x103ae14; continue; }
  pc = 0x103afc5; continue;
 }
 0x103afc5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103afdf; continue;
 }
 0x103afdf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103afe8; continue;
 }
 0x103afe8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b060; continue; }
  pc = 0x103aff6; continue;
 }
 0x103aff6 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b060; continue; }
  pc = 0x103b003; continue;
 }
 0x103b003 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b01a; continue; }
  pc = 0x103b00c; continue;
 }
 0x103b00c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103b091; continue; }
  pc = 0x103b01a; continue;
 }
 0x103b01a => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b027; continue;
 }
 0x103b027 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b034; continue;
 }
 0x103b034 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b044; continue;
 }
 0x103b044 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b054; continue;
 }
 0x103b054 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103afae; continue; }
  pc = 0x103b05e; continue;
 }
 0x103b05e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b060 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b06c); f_0103b1e1(cpu); if cpu.returned_to != 0x103b06c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103b073; continue;
 }
 0x103b073 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103b084; continue; }
  pc = 0x103b077; continue;
 }
 0x103b077 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x103ae89; continue;
 }
 0x103b084 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x103ad65; continue;
 }
 0x103b091 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b097); f_0103b1e1(cpu); if cpu.returned_to != 0x103b097 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103b0a9; continue; }
  pc = 0x103b09c; continue;
 }
 0x103b09c => {
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x103ad65; continue;
 }
 0x103b0a9 => {
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x103ae89; continue;
 }
 0x103b0b6 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x10e3e79u32, v); }
  pc = 0x103b0cd; continue;
 }
 0x103b0cd => {
  { let v: u32 = 0x103b0b6u32; cpu.w32(0x10e40ccu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e79u32)); cpu.w16(0x10e3e79u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x103ae14; continue; }
  pc = 0x103b0e4; continue;
 }
 0x103b0e4 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x10e3f8cu32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103b0fe; continue;
 }
 0x103b0fe => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103b107; continue;
 }
 0x103b107 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b17a; continue; }
  pc = 0x103b114; continue;
 }
 0x103b114 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b17a; continue; }
  pc = 0x103b121; continue;
 }
 0x103b121 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103b138; continue; }
  pc = 0x103b12a; continue;
 }
 0x103b12a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103b1a7; continue; }
  pc = 0x103b138; continue;
 }
 0x103b138 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b145; continue;
 }
 0x103b145 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b152; continue;
 }
 0x103b152 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b162; continue;
 }
 0x103b162 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103b1cc; continue; }
  pc = 0x103b16e; continue;
 }
 0x103b16e => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103b0cd; continue; }
  pc = 0x103b178; continue;
 }
 0x103b178 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b17a => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b186); f_0103b1e1(cpu); if cpu.returned_to != 0x103b186 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103b1df; continue; }
  pc = 0x103b189; continue;
 }
 0x103b189 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103b19a; continue; }
  pc = 0x103b18d; continue;
 }
 0x103b18d => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x103ae89; continue;
 }
 0x103b19a => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x103ad65; continue;
 }
 0x103b1a7 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b1ad); f_0103b1e1(cpu); if cpu.returned_to != 0x103b1ad { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x103b1bf; continue; }
  pc = 0x103b1b2; continue;
 }
 0x103b1b2 => {
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x103ad65; continue;
 }
 0x103b1bf => {
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x103ae89; continue;
 }
 0x103b1cc => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103b1dc); f_0103b1e1(cpu); if cpu.returned_to != 0x103b1dc { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103b1df; continue;
 }
 0x103b1df => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b1e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b1e1); }
 let mut pc: u32 = 0x103b1e1;
 loop { match pc {
 0x103b1e1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x10ea910u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e40d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e40d4u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x103b222; continue; }
  pc = 0x103b1fb; continue;
 }
 0x103b1fb => {
  { cpu.sub16((cpu.esi as u16), cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x103b205; continue; }
  pc = 0x103b201; continue;
 }
 0x103b201 => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103b252; continue;
 }
 0x103b205 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x103b222; continue; }
  pc = 0x103b20b; continue;
 }
 0x103b20b => {
  { cpu.sub16((cpu.esi as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x103b222; continue; }
  pc = 0x103b211; continue;
 }
 0x103b211 => {
  { let v: u32 = cpu.r32(0x10e40d4u32).wrapping_sub(0x2u32); cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103b252; continue;
 }
 0x103b222 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if cpu.cf { pc = 0x103b239; continue; }
  pc = 0x103b228; continue;
 }
 0x103b228 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x103b230); f_0103b256(cpu); if cpu.returned_to != 0x103b230 { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if !cpu.cf { pc = 0x103b24e; continue; }
  pc = 0x103b239; continue;
 }
 0x103b239 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e40d4u32).wrapping_add(0x2u32); cpu.w32(0x10e40d4u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if cpu.cf { pc = 0x103b252; continue; }
  pc = 0x103b24e; continue;
 }
 0x103b24e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b252 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b256); }
 let mut pc: u32 = 0x103b256;
 loop { match pc {
 0x103b256 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if (cpu.cf || cpu.zf) { pc = 0x103b2d5; continue; }
  pc = 0x103b269; continue;
 }
 0x103b269 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x103b276; continue;
 }
 0x103b276 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x103b280); f_0103b2eb(cpu); if cpu.returned_to != 0x103b280 { return; }
  if cpu.cf { pc = 0x103b2b9; continue; }
  pc = 0x103b282; continue;
 }
 0x103b282 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x103b2b9; continue; }
  pc = 0x103b288; continue;
 }
 0x103b288 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x103b2b9; continue; }
  pc = 0x103b28e; continue;
 }
 0x103b28e => {
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
  pc = 0x103b2c4; continue;
 }
 0x103b2b9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x103b2c4; continue;
 }
 0x103b2c4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.cf { pc = 0x103b2d4; continue; }
  pc = 0x103b2d2; continue;
 }
 0x103b2d2 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103b276; continue; }
  pc = 0x103b2d4; continue;
 }
 0x103b2d4 => {
  { cpu.popad(); }
  pc = 0x103b2d5; continue;
 }
 0x103b2d5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40d4u32, v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b2eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b2eb); }
 let mut pc: u32 = 0x103b2eb;
 loop { match pc {
 0x103b2eb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103b320; continue; }
  pc = 0x103b30b; continue;
 }
 0x103b30b => {
  { let v: u32 = 0x115dad4u32; cpu.ebp = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103b32f; continue;
 }
 0x103b320 => {
  { let v: u32 = 0x115dad4u32; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103b32f; continue;
 }
 0x103b32f => {
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
  { let v: u32 = cpu.eax.wrapping_add(0x10ea910u32); cpu.eax = v; }
  cpu.push(0x103b35e); f_0103b3e9(cpu); if cpu.returned_to != 0x103b35e { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103b374); f_0106868f(cpu); if cpu.returned_to != 0x103b374 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10e413cu32)); }
  if !cpu.zf { pc = 0x103b38d; continue; }
  pc = 0x103b37d; continue;
 }
 0x103b37d => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103b3a2; continue;
 }
 0x103b38d => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10ea910u32); cpu.eax = v; }
  cpu.push(0x103b39a); f_0103b3e9(cpu); if cpu.returned_to != 0x103b39a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103b3a2; continue;
 }
 0x103b3a2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b3a9); f_0106a04e(cpu); if cpu.returned_to != 0x103b3a9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x103b3ae; continue; }
  pc = 0x103b3ad; continue;
 }
 0x103b3ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b3ae => {
  { let v: u8 = 0x1u8; cpu.w8(0x60u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3e79u32).wrapping_sub(0x5u16); cpu.w16(0x10e3e79u32, v); }
  pc = 0x103b3ba; continue;
 }
 0x103b3ba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103b3c1); f_0106a269(cpu); if cpu.returned_to != 0x103b3c1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x103b3d5; continue; }
  pc = 0x103b3c9; continue;
 }
 0x103b3c9 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { cpu.poll(); pc = 0x103b3ba; continue; }
  pc = 0x103b3cf; continue;
 }
 0x103b3cf => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103b3e7; continue;
 }
 0x103b3d5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103b3e7; continue; }
  pc = 0x103b3db; continue;
 }
 0x103b3db => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x103b3e7; continue; }
  pc = 0x103b3e1; continue;
 }
 0x103b3e1 => {
  { let v: u8 = 0x2u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x103b3ba; continue;
 }
 0x103b3e7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b3e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b3e9); }
 let mut pc: u32 = 0x103b3e9;
 loop { match pc {
 0x103b3e9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103b424; continue; }
  pc = 0x103b41c; continue;
 }
 0x103b41c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103b424; continue;
 }
 0x103b424 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b426(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b426); }
 let mut pc: u32 = 0x103b426;
 loop { match pc {
 0x103b426 => {
  { cpu.sub32(cpu.r32(0x10e3fbcu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x103b516; continue; }
  pc = 0x103b433; continue;
 }
 0x103b433 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
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
  if cpu.zf { pc = 0x103b494; continue; }
  pc = 0x103b472; continue;
 }
 0x103b472 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x103b494; continue; }
  pc = 0x103b478; continue;
 }
 0x103b478 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x103b494; continue; }
  pc = 0x103b47e; continue;
 }
 0x103b47e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x103b494; continue; }
  pc = 0x103b484; continue;
 }
 0x103b484 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103b490; continue; }
  pc = 0x103b48a; continue;
 }
 0x103b48a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x103b494; continue; }
  pc = 0x103b490; continue;
 }
 0x103b490 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x103b494; continue;
 }
 0x103b494 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.eax)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.eax)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11251f4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11251f8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11251fcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1125200u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1125204u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1125208u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112520cu32); cpu.ebp = v; }
  let t = cpu.r32(0x10e3fbcu32); cpu.push(0x103b4db); cpu.call(t); if cpu.returned_to != 0x103b4db { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11251f4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11251f8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x11251fcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1125200u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1125204u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1125208u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x112520cu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b516 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x103b521; continue;
 }
 0x103b521 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.cf { pc = 0x103b534; continue; }
  pc = 0x103b52f; continue;
 }
 0x103b52f => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  pc = 0x103b534; continue;
 }
 0x103b534 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x103b552; continue; }
  pc = 0x103b53a; continue;
 }
 0x103b53a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e3f90u32, v); }
  { let v: u32 = 0x103a150u32; cpu.w32(0x10e3fbcu32, v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x10ef338u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b552 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103b521; continue; }
  pc = 0x103b554; continue;
 }
 0x103b554 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b555(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b555); }
 let mut pc: u32 = 0x103b555;
 loop { match pc {
 0x103b555 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3f8cu32)); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0x10e3f8cu32)); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b569(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b569); }
 let mut pc: u32 = 0x103b569;
 loop { match pc {
 0x103b569 => {
  { let v: u32 = 0x1125060u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b57b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b57b); }
 let mut pc: u32 = 0x103b57b;
 loop { match pc {
 0x103b57b => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x103b5f0; continue; }
  pc = 0x103b58e; continue;
 }
 0x103b58e => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x103b5d8; continue; }
  pc = 0x103b597; continue;
 }
 0x103b597 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x103b5d8; continue; }
  pc = 0x103b5a0; continue;
 }
 0x103b5a0 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103b5c8; continue; }
  pc = 0x103b5a6; continue;
 }
 0x103b5a6 => {
  if cpu.sf { pc = 0x103b5c8; continue; }
  pc = 0x103b5a8; continue;
 }
 0x103b5a8 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x11757b7u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1152e54u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b5c8 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x103b64b; continue; }
  pc = 0x103b5d2; continue;
 }
 0x103b5d2 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x103b60a; continue; }
  pc = 0x103b5d8; continue;
 }
 0x103b5d8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x103b649; continue; }
  pc = 0x103b5e3; continue;
 }
 0x103b5e3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b5f0 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x103b5f8); f_010395af(cpu); if cpu.returned_to != 0x103b5f8 { return; }
  if cpu.cf { pc = 0x103b600; continue; }
  pc = 0x103b5fa; continue;
 }
 0x103b5fa => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b600 => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x103b649; continue;
 }
 0x103b60a => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x103b61a; continue; }
  pc = 0x103b613; continue;
 }
 0x103b613 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x103b5d8; continue; }
  pc = 0x103b61a; continue;
 }
 0x103b61a => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x103b62c; continue; }
  pc = 0x103b622; continue;
 }
 0x103b622 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b62c => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x103b5d8; continue; }
  pc = 0x103b635; continue;
 }
 0x103b635 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103b63b); f_01063324(cpu); if cpu.returned_to != 0x103b63b { return; }
  if cpu.cf { pc = 0x103b648; continue; }
  pc = 0x103b63d; continue;
 }
 0x103b63d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b648 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103b649; continue;
 }
 0x103b649 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b64b => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xfb2eu16); }
  if cpu.zf { pc = 0x103b66e; continue; }
  pc = 0x103b654; continue;
 }
 0x103b654 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xef1fu16); }
  if cpu.zf { pc = 0x103b662; continue; }
  pc = 0x103b65c; continue;
 }
 0x103b65c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x103b5d8; continue;
 }
 0x103b662 => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  pc = 0x103b68e; continue;
 }
 0x103b66e => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x103b67d; continue; }
  pc = 0x103b677; continue;
 }
 0x103b677 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x103b5d8; continue;
 }
 0x103b67d => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x11757a4u32); cpu.esi = v; }
  cpu.push(0x103b68c); f_010536af(cpu); if cpu.returned_to != 0x103b68c { return; }
  if cpu.cf { cpu.poll(); pc = 0x103b648; continue; }
  pc = 0x103b68e; continue;
 }
 0x103b68e => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b699(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b699); }
 let mut pc: u32 = 0x103b699;
 loop { match pc {
 0x103b699 => {
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e415cu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  { cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103b6b8; continue; }
  pc = 0x103b6b4; continue;
 }
 0x103b6b4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  pc = 0x103b6b8; continue;
 }
 0x103b6b8 => {
  { let v: u8 = cpu.r8(0x10ef338u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef30bu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef30cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef30du32, v); }
  cpu.push(0x103b6d3); f_0103b57b(cpu); if cpu.returned_to != 0x103b6d3 { return; }
  if cpu.cf { pc = 0x103b866; continue; }
  pc = 0x103b6d9; continue;
 }
 0x103b6d9 => {
  cpu.push(0x103b6de); f_0103b870(cpu); if cpu.returned_to != 0x103b6de { return; }
  { let v: u16 = cpu.r16(0x10e3e4du32).wrapping_add(0x10u16); cpu.w16(0x10e3e4du32, v); }
  pc = 0x103b6e6; continue;
 }
 0x103b6e6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3e49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3e4bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3f8cu32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10ea910u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x103b728; continue; }
  pc = 0x103b71d; continue;
 }
 0x103b71d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x103b734; continue; }
  pc = 0x103b728; continue;
 }
 0x103b728 => {
  { cpu.sub8(cpu.r8(0x10ef30bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103b73b; continue; }
  pc = 0x103b731; continue;
 }
 0x103b731 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x103b734; continue;
 }
 0x103b734 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30bu32, v); }
  pc = 0x103b73b; continue;
 }
 0x103b73b => {
  { cpu.sub32(cpu.r32(0x10e3fccu32), 0x103b92bu32); }
  if !cpu.zf { pc = 0x103b7a5; continue; }
  pc = 0x103b74b; continue;
 }
 0x103b74b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x103b761; continue; }
  pc = 0x103b756; continue;
 }
 0x103b756 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x103b76e; continue; }
  pc = 0x103b761; continue;
 }
 0x103b761 => {
  { cpu.sub8(cpu.r8(0x10ef30cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103b775; continue; }
  pc = 0x103b76a; continue;
 }
 0x103b76a => {
  { let v: u8 = 0xffu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x103b76e; continue;
 }
 0x103b76e => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30cu32, v); }
  pc = 0x103b775; continue;
 }
 0x103b775 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x103b78b; continue; }
  pc = 0x103b780; continue;
 }
 0x103b780 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x103b79c; continue; }
  pc = 0x103b78b; continue;
 }
 0x103b78b => {
  { cpu.sub8(cpu.r8(0x10ef30du32), 0x0u8); }
  if !cpu.zf { pc = 0x103b7ff; continue; }
  pc = 0x103b798; continue;
 }
 0x103b798 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x103b79c; continue;
 }
 0x103b79c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30du32, v); }
  pc = 0x103b7ff; continue;
 }
 0x103b7a5 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x103b7be; continue; }
  pc = 0x103b7b3; continue;
 }
 0x103b7b3 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x103b7ce; continue; }
  pc = 0x103b7be; continue;
 }
 0x103b7be => {
  { cpu.sub8(cpu.r8(0x10ef30cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103b7d5; continue; }
  pc = 0x103b7c7; continue;
 }
 0x103b7c7 => {
  { let v: u8 = 0xffu8; cpu.w8(0x80u32.wrapping_add(cpu.edi), v); }
  pc = 0x103b7ce; continue;
 }
 0x103b7ce => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30cu32, v); }
  pc = 0x103b7d5; continue;
 }
 0x103b7d5 => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x103b7eb; continue; }
  pc = 0x103b7e0; continue;
 }
 0x103b7e0 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x103b7f8; continue; }
  pc = 0x103b7eb; continue;
 }
 0x103b7eb => {
  { cpu.sub8(cpu.r8(0x10ef30du32), 0x0u8); }
  if !cpu.zf { pc = 0x103b7ff; continue; }
  pc = 0x103b7f4; continue;
 }
 0x103b7f4 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffff80u32.wrapping_add(cpu.edi), v); }
  pc = 0x103b7f8; continue;
 }
 0x103b7f8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30du32, v); }
  pc = 0x103b7ff; continue;
 }
 0x103b7ff => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x10e3e4du32), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103b85b; continue; }
  pc = 0x103b80a; continue;
 }
 0x103b80a => {
  { let v: u8 = 0x10u8; cpu.w8(0x10e3dc1u32, v); }
  { let v: u16 = cpu.r16(0x10e3e4fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e3fccu32); cpu.push(0x103b81d); cpu.call(t); if cpu.returned_to != 0x103b81d { return; }
  { let v: u16 = cpu.r16(0x10e3e4du32).wrapping_sub(0x10u16); cpu.w16(0x10e3e4du32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef30cu32)); cpu.w8(0x10ef30cu32, v); }
  if !cpu.sf { pc = 0x103b834; continue; }
  pc = 0x103b82d; continue;
 }
 0x103b82d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30cu32, v); }
  pc = 0x103b834; continue;
 }
 0x103b834 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef30du32)); cpu.w8(0x10ef30du32, v); }
  if !cpu.sf { pc = 0x103b843; continue; }
  pc = 0x103b83c; continue;
 }
 0x103b83c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30du32, v); }
  pc = 0x103b843; continue;
 }
 0x103b843 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef30bu32)); cpu.w8(0x10ef30bu32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x103b6e6; continue; }
  pc = 0x103b84f; continue;
 }
 0x103b84f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef30bu32, v); }
  cpu.poll(); pc = 0x103b6e6; continue;
 }
 0x103b85b => {
  { let v: u32 = cpu.r32(0x10e415cu32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b866 => {
  { let v: u32 = cpu.r32(0x10e415cu32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b870(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b870); }
 let mut pc: u32 = 0x103b870;
 loop { match pc {
 0x103b870 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.esi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.esi as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103b89f; continue; }
  pc = 0x103b88e; continue;
 }
 0x103b88e => {
  { cpu.sub8(cpu.r8(0xaeu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103b89f; continue; }
  pc = 0x103b897; continue;
 }
 0x103b897 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103b89f; continue;
 }
 0x103b89f => {
  { let v: u32 = 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.esi; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e49u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e4bu32, v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103b8ce; continue; }
  pc = 0x103b8be; continue;
 }
 0x103b8be => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3e4fu32, v); }
  pc = 0x103b8d7; continue;
 }
 0x103b8ce => {
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e4fu32, v); }
  pc = 0x103b8d7; continue;
 }
 0x103b8d7 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x103b8e9; continue; }
  pc = 0x103b8e3; continue;
 }
 0x103b8e3 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x103b8e9; continue;
 }
 0x103b8e9 => {
  { cpu.sub16(cpu.r16(0x10e3e4fu32), 0x0u16); }
  if !cpu.sf { pc = 0x103b8f6; continue; }
  pc = 0x103b8f3; continue;
 }
 0x103b8f3 => {
  { let v: u16 = cpu.neg16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x103b8f6; continue;
 }
 0x103b8f6 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if cpu.cf { pc = 0x103b913; continue; }
  pc = 0x103b8fb; continue;
 }
 0x103b8fb => {
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e4du32, v); }
  { let v: u32 = 0x103b94au32; cpu.w32(0x10e3fccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b913 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e4du32, v); }
  { let v: u32 = 0x103b92bu32; cpu.w32(0x10e3fccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b969(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b969); }
 let mut pc: u32 = 0x103b969;
 loop { match pc {
 0x103b969 => {
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x103b976; continue;
 }
 0x103b976 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x103b996; continue; }
  pc = 0x103b97d; continue;
 }
 0x103b97d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x103b996; continue; }
  pc = 0x103b982; continue;
 }
 0x103b982 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103b996; continue; }
  pc = 0x103b988; continue;
 }
 0x103b988 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x103b996; continue; }
  pc = 0x103b98e; continue;
 }
 0x103b98e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103b996; continue; }
  pc = 0x103b994; continue;
 }
 0x103b994 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103b996 => {
  { let v: u32 = cpu.esi.wrapping_add(0x88u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103b976; continue; }
  pc = 0x103b9a0; continue;
 }
 0x103b9a0 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103b9a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103b9a2); }
 let mut pc: u32 = 0x103b9a2;
 loop { match pc {
 0x103b9a2 => {
  { let v: u32 = 0x10f5870u32; cpu.w32(0x10e41e8u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103b9bc; continue; }
  pc = 0x103b9b2; continue;
 }
 0x103b9b2 => {
  { let v: u32 = 0x10f58b3u32; cpu.w32(0x10e41e8u32, v); }
  pc = 0x103b9bc; continue;
 }
 0x103b9bc => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef34fu32, v); }
  { let v: u32 = 0x1124f90u32; cpu.w32(0x10e40e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11de3b4u32, v); }
  { let v: u32 = cpu.r32(0x11de3b4u32).wrapping_sub(0x4u32); cpu.w32(0x11de3b4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11de3b8u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11cc281u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x3840u32); }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103ba1b; continue;
 }
 0x103ba1b => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103ba2a); f_0103bec2(cpu); if cpu.returned_to != 0x103ba2a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103ba32; continue;
 }
 0x103ba32 => {
  { let v = cpu.r32(0x1124f88u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1124f8cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103ba4f); f_0103bec2(cpu); if cpu.returned_to != 0x103ba4f { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103ba5f; continue;
 }
 0x103ba5f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3eddu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3edfu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103ba74); f_0103b969(cpu); if cpu.returned_to != 0x103ba74 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103ba7d; continue;
 }
 0x103ba7d => {
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103ba87); f_0103b969(cpu); if cpu.returned_to != 0x103ba87 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103ba90; continue;
 }
 0x103ba90 => {
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x103bb03; continue; }
  pc = 0x103ba98; continue;
 }
 0x103ba98 => {
  cpu.push(0x103ba9d); f_0103bb07(cpu); if cpu.returned_to != 0x103ba9d { return; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103baa3; continue;
 }
 0x103baa3 => {
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x1124f88u32), 0x0u32); }
  if !cpu.zf { pc = 0x103baba; continue; }
  pc = 0x103bab5; continue;
 }
 0x103bab5 => {
  cpu.push(0x103baba); f_0103bd84(cpu); if cpu.returned_to != 0x103baba { return; }
  pc = 0x103baba; continue;
 }
 0x103baba => {
  cpu.push(0x103babf); f_0103bd1d(cpu); if cpu.returned_to != 0x103babf { return; }
  if !cpu.cf { pc = 0x103bac7; continue; }
  pc = 0x103bac1; continue;
 }
 0x103bac1 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103bb03; continue;
 }
 0x103bac7 => {
  { let v: u16 = cpu.r16(0x10e3eddu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3edfu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x1124f8cu32), 0x0u32); }
  if !cpu.zf { pc = 0x103bae2; continue; }
  pc = 0x103badd; continue;
 }
 0x103badd => {
  cpu.push(0x103bae2); f_0103bd84(cpu); if cpu.returned_to != 0x103bae2 { return; }
  pc = 0x103bae2; continue;
 }
 0x103bae2 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x103bb03; continue; }
  pc = 0x103bae8; continue;
 }
 0x103bae8 => {
  cpu.push(0x103baed); f_0103bd84(cpu); if cpu.returned_to != 0x103baed { return; }
  cpu.push(0x103baf2); f_0103bde7(cpu); if cpu.returned_to != 0x103baf2 { return; }
  { cpu.sub8(cpu.r8(0x1125211u32), 0x0u8); }
  if !cpu.zf { pc = 0x103bb00; continue; }
  pc = 0x103bafb; continue;
 }
 0x103bafb => {
  cpu.push(0x103bb00); f_0103bc51(cpu); if cpu.returned_to != 0x103bb00 { return; }
  pc = 0x103bb00; continue;
 }
 0x103bb00 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bb03 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bb07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bb07); }
 let mut pc: u32 = 0x103bb07;
 loop { match pc {
 0x103bb07 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x88u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e40a8u32)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x103bb27; continue; }
  pc = 0x103bb23; continue;
 }
 0x103bb23 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bb27 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bb2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bb2b); }
 let mut pc: u32 = 0x103bb2b;
 loop { match pc {
 0x103bb2b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = 0x11de3bcu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e3f90u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x103bb9b; continue; }
  pc = 0x103bb47; continue;
 }
 0x103bb47 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x103bb59; continue;
 }
 0x103bb59 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10ea910u32); cpu.eax = v; }
  cpu.push(0x103bb68); f_0103b3e9(cpu); if cpu.returned_to != 0x103bb68 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11de3b0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11de3b2u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103bb59; continue; }
  pc = 0x103bb86; continue;
 }
 0x103bb86 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x11de3bcu32; cpu.esi = v; }
  cpu.push(0x103bb96); f_01017b7c(cpu); if cpu.returned_to != 0x103bb96 { return; }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103bb9d; continue; }
  pc = 0x103bb9b; continue;
 }
 0x103bb9b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bb9d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fcbe3u32, v); }
  { let v: u32 = 0x1129394u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x11de3b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11de3b2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1125211u32, v); }
  cpu.push(0x103bbe7); f_0103b9a2(cpu); if cpu.returned_to != 0x103bbe7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1125211u32, v); }
  if cpu.cf { pc = 0x103bc47; continue; }
  pc = 0x103bbf0; continue;
 }
 0x103bbf0 => {
  { let v: u32 = 0x11de3bcu32; cpu.edi = v; }
  { let v: u32 = 0x1eu32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x1129394u32; cpu.esi = v; }
  pc = 0x103bc11; continue;
 }
 0x103bc11 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x103bc2c; continue; }
  pc = 0x103bc15; continue;
 }
 0x103bc15 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103bc11; continue; }
  pc = 0x103bc2c; continue;
 }
 0x103bc2c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x11de3bcu32; cpu.esi = v; }
  cpu.push(0x103bc3c); f_01017b7c(cpu); if cpu.returned_to != 0x103bc3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcbe3u32, v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bc47 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcbe3u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bc51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bc51); }
 let mut pc: u32 = 0x103bc51;
 loop { match pc {
 0x103bc51 => {
  { let v: u32 = cpu.r32(0x11de3b4u32); cpu.ebx = v; }
  { let v: u32 = 0x1124f90u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11de3b8u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x103bc74; continue;
 }
 0x103bc74 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x103bc74; continue; }
  pc = 0x103bc82; continue;
 }
 0x103bc82 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1124f90u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x103bc93); f_01017b7c(cpu); if cpu.returned_to != 0x103bc93 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11d5885u32, v); }
  cpu.push(0x103bcb1); f_01017b7c(cpu); if cpu.returned_to != 0x103bcb1 { return; }
  { cpu.sub8(cpu.r8(0x11c5bb5u32), 0x0u8); }
  if !cpu.zf { pc = 0x103bcba; continue; }
  pc = 0x103bcba; continue;
 }
 0x103bcba => {
  { cpu.sub8(cpu.r8(0x10e3d63u32), 0x5u8); }
  if !cpu.zf { pc = 0x103bcd1; continue; }
  pc = 0x103bcc3; continue;
 }
 0x103bcc3 => {
  { cpu.sub8(cpu.r8(0x11d5885u32), 0x2u8); }
  if cpu.cf { pc = 0x103bcd1; continue; }
  pc = 0x103bccc; continue;
 }
 0x103bccc => {
  { let v: u32 = 0xfffffffu32; cpu.eax = v; }
  pc = 0x103bcd1; continue;
 }
 0x103bcd1 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x103bcec; continue; }
  pc = 0x103bcd5; continue;
 }
 0x103bcd5 => {
  { let v: u32 = cpu.r32(0x11de3b8u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11c5bb5u32), 0x0u8); }
  if !cpu.zf { pc = 0x103bceb; continue; }
  pc = 0x103bce4; continue;
 }
 0x103bce4 => {
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103bceb; continue;
 }
 0x103bceb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bcec => {
  { let v: u32 = cpu.r32(0x11de3b4u32); cpu.ebx = v; }
  pc = 0x103bcf2; continue;
 }
 0x103bcf2 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x103bd04; continue; }
  pc = 0x103bcf6; continue;
 }
 0x103bcf6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.poll(); pc = 0x103bcf2; continue;
 }
 0x103bd04 => {
  { let v: u32 = cpu.r32(0x11de3b4u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1124f94u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1124f96u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103bd1c); f_0103bd84(cpu); if cpu.returned_to != 0x103bd1c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bd1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bd1d); }
 let mut pc: u32 = 0x103bd1d;
 loop { match pc {
 0x103bd1d => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.edi = v; }
  { let v: u32 = 0x88u32; cpu.eax = v; }
  pc = 0x103bd2c; continue;
 }
 0x103bd2c => {
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x103bd80; continue; }
  pc = 0x103bd30; continue;
 }
 0x103bd30 => {
  { let v: u8 = 0x88u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e40a8u32)); cpu.edi = v; }
  { let v: u32 = 0x88u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x103bd82; continue; }
  pc = 0x103bd4b; continue;
 }
 0x103bd4b => {
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e40a8u32)); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x103bd80; continue; }
  pc = 0x103bd59; continue;
 }
 0x103bd59 => {
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
  cpu.push(0x103bd7c); f_0103bd84(cpu); if cpu.returned_to != 0x103bd7c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x103bd2c; continue; }
  pc = 0x103bd7f; continue;
 }
 0x103bd7f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bd80 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bd82 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bd84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bd84); }
 let mut pc: u32 = 0x103bd84;
 loop { match pc {
 0x103bd84 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x103bd86; continue;
 }
 0x103bd86 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x103bdaa; continue; }
  pc = 0x103bd8c; continue;
 }
 0x103bd8c => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10d10dbu32); }
  if cpu.cf { cpu.poll(); pc = 0x103bd86; continue; }
  pc = 0x103bd97; continue;
 }
 0x103bd97 => {
  { cpu.sub8(cpu.r8(0x10fcbe3u32), 0x1u8); }
  if !cpu.zf { pc = 0x103bda8; continue; }
  pc = 0x103bda0; continue;
 }
 0x103bda0 => {
  { cpu.sub32(cpu.edi, 0x1129484u32); }
  if cpu.cf { cpu.poll(); pc = 0x103bd86; continue; }
  pc = 0x103bda8; continue;
 }
 0x103bda8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bdaa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x103bde5; continue; }
  pc = 0x103bdbc; continue;
 }
 0x103bdbc => {
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef34fu32)); cpu.w8(0x10ef34fu32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e40e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e40e8u32, v); }
  { cpu.sub32(cpu.ebx, 0x1125030u32); }
  if cpu.cf { pc = 0x103bde2; continue; }
  pc = 0x103bddf; continue;
 }
 0x103bddf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bde2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x103bde5; continue;
 }
 0x103bde5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bde7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bde7); }
 let mut pc: u32 = 0x103bde7;
 loop { match pc {
 0x103bde7 => {
  { cpu.sub8(cpu.r8(0x10ef34fu32), 0x5u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103bec1; continue; }
  pc = 0x103bdf4; continue;
 }
 0x103bdf4 => {
  { let v: u32 = 0x1124f90u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e8bu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e91u32, v); }
  cpu.push(0x103be3b); f_0105c89c(cpu); if cpu.returned_to != 0x103be3b { return; }
  if cpu.cf { pc = 0x103be5a; continue; }
  pc = 0x103be3d; continue;
 }
 0x103be3d => {
  { let v: u32 = 0x1124f90u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef34fu32)); cpu.w8(0x10ef34fu32, v); }
  pc = 0x103be5a; continue;
 }
 0x103be5a => {
  { let v: u32 = cpu.r32(0x10e40e8u32); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e8bu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e91u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103bea6); f_0105c89c(cpu); if cpu.returned_to != 0x103bea6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x103bec1; continue; }
  pc = 0x103bea9; continue;
 }
 0x103bea9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef34fu32)); cpu.w8(0x10ef34fu32, v); }
  pc = 0x103bec1; continue;
 }
 0x103bec1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103bec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103bec2); }
 let mut pc: u32 = 0x103bec2;
 loop { match pc {
 0x103bec2 => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125030u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125032u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125034u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125036u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125038u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112503au32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112503cu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112503eu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1124f88u32, v); }
  cpu.push(0x103bf1c); f_0104f36d(cpu); if cpu.returned_to != 0x103bf1c { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x7u8); }
  if cpu.zf { pc = 0x103bf36; continue; }
  pc = 0x103bf2c; continue;
 }
 0x103bf2c => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x33u8); }
  if cpu.zf { pc = 0x103bf36; continue; }
  pc = 0x103bf31; continue;
 }
 0x103bf31 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103bf55; continue; }
  pc = 0x103bf36; continue;
 }
 0x103bf36 => {
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x1124f88u32, v); }
  cpu.push(0x103bf53); f_0103c3ab(cpu); if cpu.returned_to != 0x103bf53 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103bf55 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff80u32; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103bf91); f_0103c4e1(cpu); if cpu.returned_to != 0x103bf91 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103bfc7; continue; }
  pc = 0x103bf95; continue;
 }
 0x103bf95 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125030u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125040u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125042u32, v); }
  pc = 0x103bfc7; continue;
 }
 0x103bfc7 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c00c); f_0103c4e1(cpu); if cpu.returned_to != 0x103c00c { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c042; continue; }
  pc = 0x103c010; continue;
 }
 0x103c010 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125032u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125044u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125046u32, v); }
  pc = 0x103c042; continue;
 }
 0x103c042 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c087); f_0103c4e1(cpu); if cpu.returned_to != 0x103c087 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c0bd; continue; }
  pc = 0x103c08b; continue;
 }
 0x103c08b => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125034u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125048u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112504au32, v); }
  pc = 0x103c0bd; continue;
 }
 0x103c0bd => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c102); f_0103c4e1(cpu); if cpu.returned_to != 0x103c102 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c138; continue; }
  pc = 0x103c106; continue;
 }
 0x103c106 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125036u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112504cu32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112504eu32, v); }
  pc = 0x103c138; continue;
 }
 0x103c138 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff81u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c17d); f_0103c4e1(cpu); if cpu.returned_to != 0x103c17d { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c1bc; continue; }
  pc = 0x103c181; continue;
 }
 0x103c181 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125038u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125050u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125052u32, v); }
  pc = 0x103c1bc; continue;
 }
 0x103c1bc => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x81u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c201); f_0103c4e1(cpu); if cpu.returned_to != 0x103c201 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c240; continue; }
  pc = 0x103c205; continue;
 }
 0x103c205 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112503au32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125054u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125056u32, v); }
  pc = 0x103c240; continue;
 }
 0x103c240 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c285); f_0103c4e1(cpu); if cpu.returned_to != 0x103c285 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c2c4; continue; }
  pc = 0x103c289; continue;
 }
 0x103c289 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112503cu32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1125058u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112505au32, v); }
  pc = 0x103c2c4; continue;
 }
 0x103c2c4 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff7fu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.r16(0x10e3e93u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e95u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103c309); f_0103c4e1(cpu); if cpu.returned_to != 0x103c309 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c348; continue; }
  pc = 0x103c30d; continue;
 }
 0x103c30d => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112503eu32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112505cu32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x112505eu32, v); }
  pc = 0x103c348; continue;
 }
 0x103c348 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e95u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1125030u32; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x103c367; continue;
 }
 0x103c367 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if !cpu.cf { pc = 0x103c373; continue; }
  pc = 0x103c36e; continue;
 }
 0x103c36e => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103c373; continue;
 }
 0x103c373 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103c367; continue; }
  pc = 0x103c377; continue;
 }
 0x103c377 => {
  { cpu.sub16((cpu.ebx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x103c3a8; continue; }
  pc = 0x103c37d; continue;
 }
 0x103c37d => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r16(0x10e3e95u32) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1125040u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125042u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103c3a6); f_0103c3ab(cpu); if cpu.returned_to != 0x103c3a6 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103c3a8 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c3ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c3ab); }
 let mut pc: u32 = 0x103c3ab;
 loop { match pc {
 0x103c3ab => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124f80u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1124f84u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103c3c9); f_0104f36d(cpu); if cpu.returned_to != 0x103c3c9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c3e9; continue; }
  pc = 0x103c3db; continue;
 }
 0x103c3db => {
  { cpu.sub8((cpu.ecx as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c3e9; continue; }
  pc = 0x103c3e0; continue;
 }
 0x103c3e0 => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103c4df; continue; }
  pc = 0x103c3e9; continue;
 }
 0x103c3e9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x103c3ec; continue;
 }
 0x103c3ec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c400; continue; }
  pc = 0x103c3f6; continue;
 }
 0x103c3f6 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c400; continue; }
  pc = 0x103c3fb; continue;
 }
 0x103c3fb => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103c407; continue; }
  pc = 0x103c400; continue;
 }
 0x103c400 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x10u32); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x103c3ec; continue; }
  pc = 0x103c407; continue;
 }
 0x103c407 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103c450; continue; }
  pc = 0x103c40c; continue;
 }
 0x103c40c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x103c412; continue;
 }
 0x103c412 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c426; continue; }
  pc = 0x103c41c; continue;
 }
 0x103c41c => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c426; continue; }
  pc = 0x103c421; continue;
 }
 0x103c421 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103c434; continue; }
  pc = 0x103c426; continue;
 }
 0x103c426 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x103c412; continue; }
  pc = 0x103c434; continue;
 }
 0x103c434 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if cpu.zf { pc = 0x103c448; continue; }
  pc = 0x103c439; continue;
 }
 0x103c439 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x103c450; continue; }
  pc = 0x103c43e; continue;
 }
 0x103c43e => {
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124f80u32, v); }
  pc = 0x103c450; continue;
 }
 0x103c448 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1124f80u32, v); }
  pc = 0x103c450; continue;
 }
 0x103c450 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103c453; continue;
 }
 0x103c453 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c467; continue; }
  pc = 0x103c45d; continue;
 }
 0x103c45d => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c467; continue; }
  pc = 0x103c462; continue;
 }
 0x103c462 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103c473; continue; }
  pc = 0x103c467; continue;
 }
 0x103c467 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x103c453; continue; }
  pc = 0x103c473; continue;
 }
 0x103c473 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103c4cc; continue; }
  pc = 0x103c47c; continue;
 }
 0x103c47c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x103c487; continue;
 }
 0x103c487 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c49b; continue; }
  pc = 0x103c491; continue;
 }
 0x103c491 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c49b; continue; }
  pc = 0x103c496; continue;
 }
 0x103c496 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x103c4ae; continue; }
  pc = 0x103c49b; continue;
 }
 0x103c49b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3d19u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x103c487; continue; }
  pc = 0x103c4ae; continue;
 }
 0x103c4ae => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if cpu.zf { pc = 0x103c4c3; continue; }
  pc = 0x103c4b3; continue;
 }
 0x103c4b3 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x103c4cc; continue; }
  pc = 0x103c4b8; continue;
 }
 0x103c4b8 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x20u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1124f84u32, v); }
  pc = 0x103c4cc; continue;
 }
 0x103c4c3 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1124f84u32, v); }
  pc = 0x103c4cc; continue;
 }
 0x103c4cc => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x1124f80u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1124f84u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xfu32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103c4df => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c4e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c4e1); }
 let mut pc: u32 = 0x103c4e1;
 loop { match pc {
 0x103c4e1 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x103c4e2; continue;
 }
 0x103c4e2 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x103c55b; continue; }
  pc = 0x103c4f0; continue;
 }
 0x103c4f0 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x800u16); }
  if !cpu.cf { pc = 0x103c55b; continue; }
  pc = 0x103c4ff; continue;
 }
 0x103c4ff => {
  { cpu.sub16(cpu.r16(0x10e3e95u32), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x103c55b; continue; }
  pc = 0x103c509; continue;
 }
 0x103c509 => {
  { cpu.sub16(cpu.r16(0x10e3e95u32), 0x800u16); }
  if !cpu.cf { pc = 0x103c55b; continue; }
  pc = 0x103c514; continue;
 }
 0x103c514 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x103c552; continue; }
  pc = 0x103c51c; continue;
 }
 0x103c51c => {
  { cpu.sub8((cpu.eax as u8), 0x33u8); }
  if cpu.zf { pc = 0x103c552; continue; }
  pc = 0x103c520; continue;
 }
 0x103c520 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x103c552; continue; }
  pc = 0x103c524; continue;
 }
 0x103c524 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x103c52b; continue; }
  pc = 0x103c529; continue;
 }
 0x103c529 => {
  { let v: u8 = (cpu.edx as u8) | ((cpu.ecx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x103c52b; continue;
 }
 0x103c52b => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e9du32)); cpu.w16(0x10e3e9du32, v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e93u32).wrapping_add((cpu.edi as u16)); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32).wrapping_add((cpu.edi as u16)); cpu.w16(0x10e3e95u32, v); }
  cpu.poll(); pc = 0x103c4e2; continue;
 }
 0x103c552 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e9du32)); cpu.w16(0x10e3e9du32, v); }
  pc = 0x103c55b; continue;
 }
 0x103c55b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c55d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c55d); }
 let mut pc: u32 = 0x103c55d;
 loop { match pc {
 0x103c55d => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x103c573; continue; }
  pc = 0x103c564; continue;
 }
 0x103c564 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103c59c; continue; }
  pc = 0x103c56a; continue;
 }
 0x103c56a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x103c59c; continue; }
  pc = 0x103c573; continue;
 }
 0x103c573 => {
  { let v: u32 = cpu.r32(0x10e40e0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40d8u32, v); }
  { let v: u32 = cpu.r32(0x10e40e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40dcu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1125210u32, v); }
  cpu.push(0x103c593); f_01053190(cpu); if cpu.returned_to != 0x103c593 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1125210u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103c59c => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c5a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c5a4); }
 let mut pc: u32 = 0x103c5a4;
 loop { match pc {
 0x103c5a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c5a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c5a5); }
 let mut pc: u32 = 0x103c5a5;
 loop { match pc {
 0x103c5a5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103c5ac); f_010599d1(cpu); if cpu.returned_to != 0x103c5ac { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cfbu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40f0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef305u32, v); }
  cpu.push(0x103c5d2); f_0104bb49(cpu); if cpu.returned_to != 0x103c5d2 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e37u32, v); }
  { let v: u32 = 0x10c692bu32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103c5e8; continue;
 }
 0x103c5e8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x103c5ef; continue; }
  pc = 0x103c5ec; continue;
 }
 0x103c5ec => {
  { cpu.movs(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103c5e8; continue; }
  pc = 0x103c5ef; continue;
 }
 0x103c5ef => {
  { cpu.stos(1, false); }
  { let v: u16 = 0x50u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e39u32, v); }
  { let v: u32 = 0x10e3e3bu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c6f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c6f4); }
 let mut pc: u32 = 0x103c6f4;
 loop { match pc {
 0x103c6f4 => {
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if !cpu.zf { pc = 0x103c71f; continue; }
  pc = 0x103c6ff; continue;
 }
 0x103c6ff => {
  { cpu.sub32(cpu.r32(0x10e41a8u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x103c86d; continue; }
  pc = 0x103c70c; continue;
 }
 0x103c70c => {
  { let v: u32 = cpu.r32(0x1119d94u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x103c71f; continue; }
  pc = 0x103c716; continue;
 }
 0x103c716 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x103c86d; continue; }
  pc = 0x103c71f; continue;
 }
 0x103c71f => {
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x103c86d; continue; }
  pc = 0x103c72e; continue;
 }
 0x103c72e => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41b4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41b0u32, v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3d6bu32, v); }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x103c7a1; continue; }
  pc = 0x103c75a; continue;
 }
 0x103c75a => {
  { let v: u16 = cpu.r16(0x111e9feu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d71u32, v); }
  cpu.push(0x103c76b); f_01068d2a(cpu); if cpu.returned_to != 0x103c76b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3d67u32, v); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x103c77f; continue; }
  pc = 0x103c777; continue;
 }
 0x103c777 => {
  { cpu.sub8(cpu.r8(0x75u32.wrapping_add(cpu.ebp)), 0x5cu8); }
  if (cpu.sf != cpu.of) { pc = 0x103c77f; continue; }
  pc = 0x103c77d; continue;
 }
 0x103c77d => {
  { let v: u8 = 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103c77f; continue;
 }
 0x103c77f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3d6cu32, v); }
  { let v: u32 = cpu.r8(0x3bu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x103c78f; continue; }
  pc = 0x103c78d; continue;
 }
 0x103c78d => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103c78f; continue;
 }
 0x103c78f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d6fu32, v); }
  { let v: u32 = cpu.r8(0x3au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d6du32, v); }
  pc = 0x103c7da; continue;
 }
 0x103c7a1 => {
  { let v: u16 = cpu.r16(0x111ea00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d71u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d6du32, v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d6du32)); }
  if (cpu.sf == cpu.of) { pc = 0x103c7cd; continue; }
  pc = 0x103c7c7; continue;
 }
 0x103c7c7 => {
  { let v: u16 = cpu.r16(0x10e3d6du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103c7cd; continue;
 }
 0x103c7cd => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x103c7d4; continue; }
  pc = 0x103c7d2; continue;
 }
 0x103c7d2 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103c7d4; continue;
 }
 0x103c7d4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d6fu32, v); }
  pc = 0x103c7da; continue;
 }
 0x103c7da => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xa1u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e418cu32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1152eb2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4190u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d73u32, v); }
  { let v: u32 = cpu.r32(0x10e3f74u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e3f50u32)); }
  if (cpu.sf == cpu.of) { pc = 0x103c826; continue; }
  pc = 0x103c821; continue;
 }
 0x103c821 => {
  { let v: u32 = cpu.r32(0x10e3f50u32); cpu.eax = v; }
  pc = 0x103c826; continue;
 }
 0x103c826 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e3f54u32)); }
  if (cpu.sf == cpu.of) { pc = 0x103c833; continue; }
  pc = 0x103c82e; continue;
 }
 0x103c82e => {
  { let v: u32 = cpu.r32(0x10e3f54u32); cpu.eax = v; }
  pc = 0x103c833; continue;
 }
 0x103c833 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e3f58u32)); }
  if (cpu.sf == cpu.of) { pc = 0x103c840; continue; }
  pc = 0x103c83b; continue;
 }
 0x103c83b => {
  { let v: u32 = cpu.r32(0x10e3f58u32); cpu.eax = v; }
  pc = 0x103c840; continue;
 }
 0x103c840 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e3f5cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x103c84d; continue; }
  pc = 0x103c848; continue;
 }
 0x103c848 => {
  { let v: u32 = cpu.r32(0x10e3f5cu32); cpu.eax = v; }
  pc = 0x103c84d; continue;
 }
 0x103c84d => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f74u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41a4u32, v); }
  { let v: u32 = cpu.r32(0x10e3f50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e419cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41a0u32, v); }
  pc = 0x103c874; continue;
 }
 0x103c86d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3d6bu32, v); }
  pc = 0x103c874; continue;
 }
 0x103c874 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  cpu.push(0x103c87f); f_01079abe(cpu); if cpu.returned_to != 0x103c87f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3d69u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10e3d6au32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10e3d68u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c891(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c891); }
 let mut pc: u32 = 0x103c891;
 loop { match pc {
 0x103c891 => {
  { let v: u32 = 0x11de486u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef36bu32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11751c8u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11de576u32, v); }
  pc = 0x103c8b9; continue;
 }
 0x103c8b9 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x103c8ef; continue; }
  pc = 0x103c8bf; continue;
 }
 0x103c8bf => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x103c8ef; continue; }
  pc = 0x103c8cd; continue;
 }
 0x103c8cd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x103c8d6); f_01031c81(cpu); if cpu.returned_to != 0x103c8d6 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.cf { pc = 0x103c8dc; continue; }
  pc = 0x103c8da; continue;
 }
 0x103c8da => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103c8dc; continue;
 }
 0x103c8dc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x103c8e7); f_0103c905(cpu); if cpu.returned_to != 0x103c8e7 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11de576u32)); cpu.w8(0x11de576u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x103c8ef; continue;
 }
 0x103c8ef => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103c8b9; continue; }
  pc = 0x103c8f6; continue;
 }
 0x103c8f6 => {
  { cpu.sub8(cpu.r8(0x11de576u32), 0x0u8); }
  if !cpu.zf { pc = 0x103c904; continue; }
  pc = 0x103c8ff; continue;
 }
 0x103c8ff => {
  cpu.push(0x103c904); f_0103ca43(cpu); if cpu.returned_to != 0x103c904 { return; }
  pc = 0x103c904; continue;
 }
 0x103c904 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103c905(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103c905); }
 let mut pc: u32 = 0x103c905;
 loop { match pc {
 0x103c905 => {
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x50u32); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103c927; continue;
 }
 0x103c927 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103ca2c; continue; }
  pc = 0x103c934; continue;
 }
 0x103c934 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103ca2c; continue; }
  pc = 0x103c945; continue;
 }
 0x103c945 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x103ca2c; continue; }
  pc = 0x103c94e; continue;
 }
 0x103c94e => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d19u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103ca2c; continue; }
  pc = 0x103c95f; continue;
 }
 0x103c95f => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x103ca2c; continue; }
  pc = 0x103c968; continue;
 }
 0x103c968 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x103c9a5; continue; }
  pc = 0x103c971; continue;
 }
 0x103c971 => {
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
  { cpu.sub32(cpu.esi, cpu.r32(0x10e413cu32)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x103ca2c; continue; }
  pc = 0x103c9a4; continue;
 }
 0x103c9a4 => {
  {  }
  pc = 0x103c9a5; continue;
 }
 0x103c9a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11de486u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x103ca2c; continue; }
  pc = 0x103c9b2; continue;
 }
 0x103c9b2 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x103c9f3; continue; }
  pc = 0x103c9dd; continue;
 }
 0x103c9dd => {
  { cpu.and32(cpu.r32(0x10e3f78u32), 0x1u32); }
  if !cpu.zf { pc = 0x103c9f3; continue; }
  pc = 0x103c9e9; continue;
 }
 0x103c9e9 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x103c9f3; continue; }
  pc = 0x103c9ef; continue;
 }
 0x103c9ef => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103c9fe; continue;
 }
 0x103c9f3 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103c9fe; continue;
 }
 0x103c9fe => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103ca10; continue; }
  pc = 0x103ca08; continue;
 }
 0x103ca08 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x103ca10; continue; }
  pc = 0x103ca0e; continue;
 }
 0x103ca0e => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103ca10; continue;
 }
 0x103ca10 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e40c0u32).wrapping_add(0x6u32); cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef36bu32)); cpu.w8(0x10ef36bu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x11de486u32.wrapping_add(cpu.eax), v); }
  pc = 0x103ca2c; continue;
 }
 0x103ca2c => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103c927; continue; }
  pc = 0x103ca3c; continue;
 }
 0x103ca3c => {
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ca43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ca43); }
 let mut pc: u32 = 0x103ca43;
 loop { match pc {
 0x103ca43 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103ca52; continue;
 }
 0x103ca52 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103caf5; continue; }
  pc = 0x103ca5f; continue;
 }
 0x103ca5f => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x103caf5; continue; }
  pc = 0x103ca6d; continue;
 }
 0x103ca6d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103caf5; continue; }
  pc = 0x103ca7a; continue;
 }
 0x103ca7a => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x103caf5; continue; }
  pc = 0x103ca7f; continue;
 }
 0x103ca7f => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d19u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103caf5; continue; }
  pc = 0x103ca8c; continue;
 }
 0x103ca8c => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x103caf5; continue; }
  pc = 0x103ca91; continue;
 }
 0x103ca91 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11de486u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x103caf5; continue; }
  pc = 0x103caa5; continue;
 }
 0x103caa5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f74c7u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103cad9; continue; }
  pc = 0x103cad1; continue;
 }
 0x103cad1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x103cad9; continue; }
  pc = 0x103cad7; continue;
 }
 0x103cad7 => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103cad9; continue;
 }
 0x103cad9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e40c0u32).wrapping_add(0x6u32); cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef36bu32)); cpu.w8(0x10ef36bu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x11de486u32.wrapping_add(cpu.eax), v); }
  pc = 0x103caf5; continue;
 }
 0x103caf5 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103ca52; continue; }
  pc = 0x103cb05; continue;
 }
 0x103cb05 => {
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cb0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cb0c); }
 let mut pc: u32 = 0x103cb0c;
 loop { match pc {
 0x103cb0c => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x103cb13; continue;
 }
 0x103cb13 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x103cbe5; continue; }
  pc = 0x103cb1d; continue;
 }
 0x103cb1d => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x103cbe5; continue; }
  pc = 0x103cb2f; continue;
 }
 0x103cb2f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103cb36); f_01031c81(cpu); if cpu.returned_to != 0x103cb36 { return; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x50u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.cf { pc = 0x103cb45; continue; }
  pc = 0x103cb40; continue;
 }
 0x103cb40 => {
  { let v: u32 = 0x7ffu32; cpu.eax = v; }
  pc = 0x103cb45; continue;
 }
 0x103cb45 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x111c986u32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 1u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  cpu.push(0x103cb6a); f_0103cbf1(cpu); if cpu.returned_to != 0x103cb6a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11b51aau32)); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x103cb9e; continue; }
  pc = 0x103cb8f; continue;
 }
 0x103cb8f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x33u32.wrapping_add(cpu.ebp))); cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x33u32.wrapping_add(cpu.ebp)), (cpu.ecx as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x103cb9e; continue; }
  pc = 0x103cb9a; continue;
 }
 0x103cb9a => {
  { let v: u8 = 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103cb9e; continue;
 }
 0x103cb9e => {
  { let v: u32 = cpu.r32(0x111a968u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x120u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103cbb9; continue; }
  pc = 0x103cbb3; continue;
 }
 0x103cbb3 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x140u32); cpu.ecx = v; }
  pc = 0x103cbb9; continue;
 }
 0x103cbb9 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111a96cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x92u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103cbd6; continue; }
  pc = 0x103cbd0; continue;
 }
 0x103cbd0 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x92u32); cpu.ebx = v; }
  pc = 0x103cbd6; continue;
 }
 0x103cbd6 => {
  { let v: u16 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x103cbe3); f_0103cf6c(cpu); if cpu.returned_to != 0x103cbe3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x103cbe5; continue;
 }
 0x103cbe5 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103cb13; continue; }
  pc = 0x103cbf0; continue;
 }
 0x103cbf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cbf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cbf1); }
 let mut pc: u32 = 0x103cbf1;
 loop { match pc {
 0x103cbf1 => {
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cc00); }
 let mut pc: u32 = 0x103cc00;
 loop { match pc {
 0x103cc00 => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc3d; continue; }
  pc = 0x103cc09; continue;
 }
 0x103cc09 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc3c; continue; }
  pc = 0x103cc12; continue;
 }
 0x103cc12 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc3c; continue; }
  pc = 0x103cc1b; continue;
 }
 0x103cc1b => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103cc25; continue; }
  pc = 0x103cc24; continue;
 }
 0x103cc24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103cc25 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103cc3c; continue; }
  pc = 0x103cc2e; continue;
 }
 0x103cc2e => {
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f7u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x11d02b1u32, v); }
  pc = 0x103cc3c; continue;
 }
 0x103cc3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103cc3d => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc5f; continue; }
  pc = 0x103cc46; continue;
 }
 0x103cc46 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc5f; continue; }
  pc = 0x103cc4f; continue;
 }
 0x103cc4f => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103cc5f; continue; }
  pc = 0x103cc58; continue;
 }
 0x103cc58 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110587cu32, v); }
  pc = 0x103cc5f; continue;
 }
 0x103cc5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cc60); }
 let mut pc: u32 = 0x103cc60;
 loop { match pc {
 0x103cc60 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103cc89; continue; }
  pc = 0x103cc69; continue;
 }
 0x103cc69 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc89; continue; }
  pc = 0x103cc72; continue;
 }
 0x103cc72 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cc89; continue; }
  pc = 0x103cc7b; continue;
 }
 0x103cc7b => {
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f8u32, v); }
  { let v: u8 = cpu.xor8(cpu.r8(0x110587bu32), 0x1u8); cpu.w8(0x110587bu32, v); }
  pc = 0x103cc89; continue;
 }
 0x103cc89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cc8a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103cc8a); } r_0103cc8a(cpu, 0x103cc8a); }
pub fn f_0103ccce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103ccce); } r_0103cc8a(cpu, 0x103ccce); }
pub fn f_0103cd49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103cd49); } r_0103cc8a(cpu, 0x103cd49); }
fn r_0103cc8a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103cc8a => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd87; continue; }
  pc = 0x103cc9c; continue;
 }
 0x103cc9c => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee944u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x103cccd; continue; }
  pc = 0x103ccad; continue;
 }
 0x103ccad => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cccd; continue; }
  pc = 0x103ccb6; continue;
 }
 0x103ccb6 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cccd; continue; }
  pc = 0x103ccbf; continue;
 }
 0x103ccbf => {
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fcbf8u32, v); }
  pc = 0x103cccd; continue;
 }
 0x103cccd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ccce => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd87; continue; }
  pc = 0x103cce0; continue;
 }
 0x103cce0 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee940u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x103cd0f; continue; }
  pc = 0x103ccf1; continue;
 }
 0x103ccf1 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd0f; continue; }
  pc = 0x103ccfa; continue;
 }
 0x103ccfa => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd0f; continue; }
  pc = 0x103cd03; continue;
 }
 0x103cd03 => {
  cpu.push(0x103cd08); f_0100881d(cpu); if cpu.returned_to != 0x103cd08 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9fau32, v); }
  pc = 0x103cd0f; continue;
 }
 0x103cd0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103cd49 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd87; continue; }
  pc = 0x103cd57; continue;
 }
 0x103cd57 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee938u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x103cd86; continue; }
  pc = 0x103cd68; continue;
 }
 0x103cd68 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd86; continue; }
  pc = 0x103cd71; continue;
 }
 0x103cd71 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd86; continue; }
  pc = 0x103cd7a; continue;
 }
 0x103cd7a => {
  cpu.push(0x103cd7f); f_01008832(cpu); if cpu.returned_to != 0x103cd7f { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9fcu32, v); }
  pc = 0x103cd86; continue;
 }
 0x103cd86 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103cd87 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cdf8; continue; }
  pc = 0x103cd90; continue;
 }
 0x103cd90 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cdf8; continue; }
  pc = 0x103cd99; continue;
 }
 0x103cd99 => {
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x103cdbe; continue; }
  pc = 0x103cda8; continue;
 }
 0x103cda8 => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x103cdbe; continue; }
  pc = 0x103cdb7; continue;
 }
 0x103cdb7 => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x103cdf8; continue;
 }
 0x103cdbe => {
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x103cdf8; continue; }
  pc = 0x103cdde; continue;
 }
 0x103cdde => {
  { let v: u8 = (cpu.eax as u8) ^ 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x103cdf8; continue; }
  pc = 0x103cdf3; continue;
 }
 0x103cdf3 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x103cdf8; continue;
 }
 0x103cdf8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cd10); }
 let mut pc: u32 = 0x103cd10;
 loop { match pc {
 0x103cd10 => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd48; continue; }
  pc = 0x103cd19; continue;
 }
 0x103cd19 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee93cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x103cd48; continue; }
  pc = 0x103cd2a; continue;
 }
 0x103cd2a => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd48; continue; }
  pc = 0x103cd33; continue;
 }
 0x103cd33 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cd48; continue; }
  pc = 0x103cd3c; continue;
 }
 0x103cd3c => {
  cpu.push(0x103cd41); f_01008847(cpu); if cpu.returned_to != 0x103cd41 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9fbu32, v); }
  pc = 0x103cd48; continue;
 }
 0x103cd48 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cdf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cdf9); }
 let mut pc: u32 = 0x103cdf9;
 loop { match pc {
 0x103cdf9 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x103ce75; continue; }
  pc = 0x103ce02; continue;
 }
 0x103ce02 => {
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11c5b26u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x103ce75; continue; }
  pc = 0x103ce0e; continue;
 }
 0x103ce0e => {
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x103ce33; continue; }
  pc = 0x103ce1d; continue;
 }
 0x103ce1d => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x103ce33; continue; }
  pc = 0x103ce2c; continue;
 }
 0x103ce2c => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x103ce6e; continue;
 }
 0x103ce33 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x103ce68; continue; }
  pc = 0x103ce5c; continue;
 }
 0x103ce5c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x103ce68; continue; }
  pc = 0x103ce62; continue;
 }
 0x103ce62 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x103ce6e; continue; }
  pc = 0x103ce68; continue;
 }
 0x103ce68 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x80u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x103ce6e; continue;
 }
 0x103ce6e => {
  { let v: u8 = 0x1u8; cpu.w8(0x110587bu32, v); }
  pc = 0x103ce75; continue;
 }
 0x103ce75 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ce77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ce77); }
 let mut pc: u32 = 0x103ce77;
 loop { match pc {
 0x103ce77 => {
  { cpu.sub8(cpu.r8(0x10fcbf5u32), 0x0u8); }
  if cpu.zf { pc = 0x103ce9e; continue; }
  pc = 0x103ce80; continue;
 }
 0x103ce80 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103ce9e; continue; }
  pc = 0x103ce89; continue;
 }
 0x103ce89 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103ce9e; continue; }
  pc = 0x103ce92; continue;
 }
 0x103ce92 => {
  cpu.push(0x103ce97); f_01008808(cpu); if cpu.returned_to != 0x103ce97 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f3u32, v); }
  pc = 0x103ce9e; continue;
 }
 0x103ce9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ce9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ce9f); }
 let mut pc: u32 = 0x103ce9f;
 loop { match pc {
 0x103ce9f => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cebf; continue; }
  pc = 0x103cea8; continue;
 }
 0x103cea8 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cebf; continue; }
  pc = 0x103ceb1; continue;
 }
 0x103ceb1 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x111e9f4u32), 0x1u8); cpu.w8(0x111e9f4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f0u32, v); }
  pc = 0x103cebf; continue;
 }
 0x103cebf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cec0); }
 let mut pc: u32 = 0x103cec0;
 loop { match pc {
 0x103cec0 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cee0; continue; }
  pc = 0x103cec9; continue;
 }
 0x103cec9 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cee0; continue; }
  pc = 0x103ced2; continue;
 }
 0x103ced2 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x111e9f5u32), 0x1u8); cpu.w8(0x111e9f5u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f1u32, v); }
  pc = 0x103cee0; continue;
 }
 0x103cee0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cee1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cee1); }
 let mut pc: u32 = 0x103cee1;
 loop { match pc {
 0x103cee1 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103cf01; continue; }
  pc = 0x103ceea; continue;
 }
 0x103ceea => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x103cf01; continue; }
  pc = 0x103cef3; continue;
 }
 0x103cef3 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x111e9f6u32), 0x1u8); cpu.w8(0x111e9f6u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x111e9f2u32, v); }
  pc = 0x103cf01; continue;
 }
 0x103cf01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cf02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cf02); }
 let mut pc: u32 = 0x103cf02;
 loop { match pc {
 0x103cf02 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x103cf0c; continue; }
  pc = 0x103cf0b; continue;
 }
 0x103cf0b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103cf0c => {
  cpu.push(0x103cf11); f_0103cb0c(cpu); if cpu.returned_to != 0x103cf11 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cf34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cf34); }
 let mut pc: u32 = 0x103cf34;
 loop { match pc {
 0x103cf34 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e40fcu32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x10e4100u32)); cpu.ebx = v; }
  cpu.push(0x103cf4c); f_01025059(cpu); if cpu.returned_to != 0x103cf4c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cf4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cf4e); }
 let mut pc: u32 = 0x103cf4e;
 loop { match pc {
 0x103cf4e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e40fcu32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x10e4100u32)); cpu.ebx = v; }
  cpu.push(0x103cf68); f_0102508f(cpu); if cpu.returned_to != 0x103cf68 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103cf6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103cf6c); }
 let mut pc: u32 = 0x103cf6c;
 loop { match pc {
 0x103cf6c => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e4104u32); cpu.push(v); }
  { let v = cpu.r32(0x10e40fcu32); cpu.push(v); }
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4100u32); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x111a968u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x120u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103cf9e; continue; }
  pc = 0x103cf99; continue;
 }
 0x103cf99 => {
  { let v: u32 = cpu.eax.wrapping_add(0x140u32); cpu.eax = v; }
  pc = 0x103cf9e; continue;
 }
 0x103cf9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a970u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4104u32, v); }
  { let v: u32 = cpu.r32(0x111a96cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4100u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x7u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x103cfcd; continue;
 }
 0x103cfcd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x2u16); }
  if cpu.cf { pc = 0x103cfe8; continue; }
  pc = 0x103cfd7; continue;
 }
 0x103cfd7 => {
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103cfe8; continue; }
  pc = 0x103cfe0; continue;
 }
 0x103cfe0 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e4380u32); cpu.push(0x103cfe7); cpu.call(t); if cpu.returned_to != 0x103cfe7 { return; }
  { cpu.popad(); }
  pc = 0x103cfe8; continue;
 }
 0x103cfe8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x8u32); }
  if !cpu.zf { cpu.poll(); pc = 0x103cfcd; continue; }
  pc = 0x103cfef; continue;
 }
 0x103cfef => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4100u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4104u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d009(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d009); }
 let mut pc: u32 = 0x103d009;
 loop { match pc {
 0x103d009 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x103d010; continue;
 }
 0x103d010 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x103d030; continue; }
  pc = 0x103d02a; continue;
 }
 0x103d02a => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x19u32); cpu.eax = v; }
  pc = 0x103d030; continue;
 }
 0x103d030 => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if cpu.cf { pc = 0x103d038; continue; }
  pc = 0x103d035; continue;
 }
 0x103d035 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x103d038; continue;
 }
 0x103d038 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103d010; continue; }
  pc = 0x103d03a; continue;
 }
 0x103d03a => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x111ea00u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d042(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d042); }
 let mut pc: u32 = 0x103d042;
 loop { match pc {
 0x103d042 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x103d049; continue;
 }
 0x103d049 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x103d062; continue; }
  pc = 0x103d05f; continue;
 }
 0x103d05f => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103d062; continue;
 }
 0x103d062 => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if cpu.cf { pc = 0x103d06a; continue; }
  pc = 0x103d067; continue;
 }
 0x103d067 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x103d06a; continue;
 }
 0x103d06a => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103d049; continue; }
  pc = 0x103d06c; continue;
 }
 0x103d06c => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x111e9feu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d074); }
 let mut pc: u32 = 0x103d074;
 loop { match pc {
 0x103d074 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x103d0fa; continue; }
  pc = 0x103d07e; continue;
 }
 0x103d07e => {
  cpu.push(0x103d083); f_010338d0(cpu); if cpu.returned_to != 0x103d083 { return; }
  cpu.push(0x103d088); f_010576b2(cpu); if cpu.returned_to != 0x103d088 { return; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x103d0c6; continue; }
  pc = 0x103d099; continue;
 }
 0x103d099 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x10e3df9u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103d0ba); f_0106f853(cpu); if cpu.returned_to != 0x103d0ba { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0xfff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfff8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d0fa; continue;
 }
 0x103d0c6 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x103d0fa; continue; }
  pc = 0x103d0d4; continue;
 }
 0x103d0d4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebp as u16), 3u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x10e3df9u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103d0f2); f_0106f853(cpu); if cpu.returned_to != 0x103d0f2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x103d0fa); f_0103d0fb(cpu); if cpu.returned_to != 0x103d0fa { return; }
  pc = 0x103d0fa; continue;
 }
 0x103d0fa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d0fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d0fb); }
 let mut pc: u32 = 0x103d0fb;
 loop { match pc {
 0x103d0fb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d0fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d0fc); }
 let mut pc: u32 = 0x103d0fc;
 loop { match pc {
 0x103d0fc => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x103d128; continue; }
  pc = 0x103d106; continue;
 }
 0x103d106 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 3u32); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x103d128); f_0106f853(cpu); if cpu.returned_to != 0x103d128 { return; }
  pc = 0x103d128; continue;
 }
 0x103d128 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d12a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d12a); }
 let mut pc: u32 = 0x103d12a;
 loop { match pc {
 0x103d12a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d12b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d12b); }
 let mut pc: u32 = 0x103d12b;
 loop { match pc {
 0x103d12b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee983u32, v); }
  { let v: u8 = 0x8u8; cpu.w8(0x10ee984u32, v); }
  { let v: u32 = 0x10f79c1u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  cpu.push(0x103d148); f_0103c5a4(cpu); if cpu.returned_to != 0x103d148 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d149(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d149); }
 let mut pc: u32 = 0x103d149;
 loop { match pc {
 0x103d149 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x112527cu32, v); }
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x103d2f7; continue; }
  pc = 0x103d161; continue;
 }
 0x103d161 => {
  { cpu.sub8(cpu.r8(0x1125281u32), 0x0u8); }
  if cpu.zf { pc = 0x103d2f7; continue; }
  pc = 0x103d16e; continue;
 }
 0x103d16e => {
  cpu.push(0x103d173); f_010576b2(cpu); if cpu.returned_to != 0x103d173 { return; }
  { let v: u32 = 0x11943fcu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1105670u32, v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x103d1a2; continue; }
  pc = 0x103d198; continue;
 }
 0x103d198 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1105670u32, v); }
  pc = 0x103d1a2; continue;
 }
 0x103d1a2 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x103d2f7; continue; }
  pc = 0x103d1aa; continue;
 }
 0x103d1aa => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x103d2f7; continue; }
  pc = 0x103d1b3; continue;
 }
 0x103d1b3 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x103d2f7; continue; }
  pc = 0x103d1c0; continue;
 }
 0x103d1c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x103d1da; continue;
 }
 0x103d1da => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x103d1f7; continue; }
  pc = 0x103d1de; continue;
 }
 0x103d1de => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x103d1f7; continue; }
  pc = 0x103d1e3; continue;
 }
 0x103d1e3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.poll(); pc = 0x103d1da; continue;
 }
 0x103d1f7 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x103d247; continue; }
  pc = 0x103d1ff; continue;
 }
 0x103d1ff => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103d20b; continue; }
  pc = 0x103d205; continue;
 }
 0x103d205 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x103d210; continue; }
  pc = 0x103d20b; continue;
 }
 0x103d20b => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  pc = 0x103d27a; continue;
 }
 0x103d210 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103d216); f_0103d3bd(cpu); if cpu.returned_to != 0x103d216 { return; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x103d27a; continue;
 }
 0x103d22a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x103d27a; continue;
 }
 0x103d247 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { cpu.poll(); pc = 0x103d22a; continue; }
  pc = 0x103d24f; continue;
 }
 0x103d24f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x103d256); f_01063324(cpu); if cpu.returned_to != 0x103d256 { return; }
  if !cpu.cf { pc = 0x103d25f; continue; }
  pc = 0x103d258; continue;
 }
 0x103d258 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x103d2f7; continue;
 }
 0x103d25f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x103d27a; continue;
 }
 0x103d27a => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(0x6u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x5au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x103d28f); f_010704c2(cpu); if cpu.returned_to != 0x103d28f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(0x6u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x1105670u32), 0x0u8); }
  if cpu.zf { pc = 0x103d2ed; continue; }
  pc = 0x103d2a1; continue;
 }
 0x103d2a1 => {
  { cpu.pushad(); }
  cpu.push(0x103d2a7); f_0103d2f8(cpu); if cpu.returned_to != 0x103d2a7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x103d2ed; continue; }
  pc = 0x103d2b5; continue;
 }
 0x103d2b5 => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if !cpu.zf { pc = 0x103d2c9; continue; }
  pc = 0x103d2c4; continue;
 }
 0x103d2c4 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if cpu.zf { pc = 0x103d2ed; continue; }
  pc = 0x103d2c9; continue;
 }
 0x103d2c9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x6u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x6u32); cpu.edx = v; }
  { let v: u32 = 0x5au32; cpu.ebp = v; }
  cpu.push(0x103d2df); f_01070516(cpu); if cpu.returned_to != 0x103d2df { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  pc = 0x103d2f2; continue;
 }
 0x103d2ed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  pc = 0x103d2f2; continue;
 }
 0x103d2f2 => {
  cpu.push(0x103d2f7); f_0103d424(cpu); if cpu.returned_to != 0x103d2f7 { return; }
  pc = 0x103d2f7; continue;
 }
 0x103d2f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d2f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d2f8); }
 let mut pc: u32 = 0x103d2f8;
 loop { match pc {
 0x103d2f8 => {
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x103d301; continue; }
  pc = 0x103d2ff; continue;
 }
 0x103d2ff => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103d301; continue;
 }
 0x103d301 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x103d316; continue; }
  pc = 0x103d305; continue;
 }
 0x103d305 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d3b9; continue;
 }
 0x103d316 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x103d35d; continue; }
  pc = 0x103d31c; continue;
 }
 0x103d31c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x103d33f; continue; }
  pc = 0x103d327; continue;
 }
 0x103d327 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x103d33f; continue; }
  pc = 0x103d33d; continue;
 }
 0x103d33d => {
  pc = 0x103d3bb; continue;
 }
 0x103d33f => {
  { cpu.sub16((cpu.eax as u16), 0xd80u16); }
  if cpu.zf { pc = 0x103d3bb; continue; }
  pc = 0x103d345; continue;
 }
 0x103d345 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d3b9; continue;
 }
 0x103d35d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x103d376; continue; }
  pc = 0x103d36e; continue;
 }
 0x103d36e => {
  { cpu.sub16((cpu.eax as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x103d3a1; continue; }
  pc = 0x103d374; continue;
 }
 0x103d374 => {
  pc = 0x103d3b9; continue;
 }
 0x103d376 => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x103d3bb; continue; }
  pc = 0x103d388; continue;
 }
 0x103d388 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  pc = 0x103d3b9; continue;
 }
 0x103d3a1 => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d3b9; continue;
 }
 0x103d3b9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d3bb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d3bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d3bd); }
 let mut pc: u32 = 0x103d3bd;
 loop { match pc {
 0x103d3bd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x103d3f3; continue; }
  pc = 0x103d3cd; continue;
 }
 0x103d3cd => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x103d3dd; continue; }
  pc = 0x103d3d4; continue;
 }
 0x103d3d4 => {
  { cpu.sub16((cpu.ecx as u16), 0xd80u16); }
  if cpu.zf { pc = 0x103d41a; continue; }
  pc = 0x103d3db; continue;
 }
 0x103d3db => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d3dd => {
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d3f3 => {
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.edx); cpu.esi = v; }
  cpu.push(0x103d404); f_010536af(cpu); if cpu.returned_to != 0x103d404 { return; }
  if cpu.cf { pc = 0x103d410; continue; }
  pc = 0x103d406; continue;
 }
 0x103d406 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d410 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d41a => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d424); }
 let mut pc: u32 = 0x103d424;
 loop { match pc {
 0x103d424 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x103d42c); f_010338d0(cpu); if cpu.returned_to != 0x103d42c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x103d43e; continue; }
  pc = 0x103d435; continue;
 }
 0x103d435 => {
  cpu.push(0x103d43a); f_01018cce(cpu); if cpu.returned_to != 0x103d43a { return; }
  if !cpu.cf { pc = 0x103d43e; continue; }
  pc = 0x103d43c; continue;
 }
 0x103d43c => {
  pc = 0x103d49e; continue;
 }
 0x103d43e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xa3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103d468; continue; }
  pc = 0x103d44c; continue;
 }
 0x103d44c => {
  { cpu.sub32(cpu.eax, 0xac3u32); }
  if cpu.zf { pc = 0x103d46a; continue; }
  pc = 0x103d453; continue;
 }
 0x103d453 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  pc = 0x103d542; continue;
 }
 0x103d468 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d46a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d6f0u32, v); }
  cpu.push(0x103d47a); f_010191d1(cpu); if cpu.returned_to != 0x103d47a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d6f0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103d4ae; continue; }
  pc = 0x103d486; continue;
 }
 0x103d486 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.poll(); pc = 0x103d453; continue;
 }
 0x103d48e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.poll(); pc = 0x103d453; continue;
 }
 0x103d496 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac3u32; cpu.eax = v; }
  cpu.poll(); pc = 0x103d453; continue;
 }
 0x103d49e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xacbu32; cpu.eax = v; }
  cpu.poll(); pc = 0x103d453; continue;
 }
 0x103d4a6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb11u32; cpu.eax = v; }
  cpu.poll(); pc = 0x103d453; continue;
 }
 0x103d4ae => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x103d4f2; continue; }
  pc = 0x103d4b4; continue;
 }
 0x103d4b4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103d4f2; continue; }
  pc = 0x103d4ba; continue;
 }
 0x103d4ba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x1cu8); }
  if cpu.zf { cpu.poll(); pc = 0x103d496; continue; }
  pc = 0x103d4de; continue;
 }
 0x103d4de => {
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x47u8); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x103d48e; continue; }
  pc = 0x103d4e7; continue;
 }
 0x103d4e7 => {
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x4du8); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x103d48e; continue; }
  pc = 0x103d4f0; continue;
 }
 0x103d4f0 => {
  cpu.poll(); pc = 0x103d486; continue;
 }
 0x103d4f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x10f746fu32.wrapping_add(cpu.eax)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x1cu8); }
  if cpu.zf { cpu.poll(); pc = 0x103d496; continue; }
  pc = 0x103d521; continue;
 }
 0x103d521 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10e3e93u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f746fu32.wrapping_add(cpu.eax)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x103d486; continue; }
  pc = 0x103d537; continue;
 }
 0x103d537 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x103d496; continue; }
  pc = 0x103d53d; continue;
 }
 0x103d53d => {
  cpu.poll(); pc = 0x103d4a6; continue;
 }
 0x103d542 => {
  { cpu.sub8(cpu.r8(0x11252a3u32), 0x0u8); }
  if cpu.zf { pc = 0x103d55c; continue; }
  pc = 0x103d54b; continue;
 }
 0x103d54b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11a37ecu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11a37eeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d55c; continue;
 }
 0x103d55c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1125274u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1125278u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112527cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11056a1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d580); }
 let mut pc: u32 = 0x103d580;
 loop { match pc {
 0x103d580 => {
  { let v: u32 = cpu.r32(0x1125274u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1125278u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112527cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x103d59c; continue; }
  pc = 0x103d596; continue;
 }
 0x103d596 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103d59c); cpu.call(t); if cpu.returned_to != 0x103d59c { return; }
  pc = 0x103d59c; continue;
 }
 0x103d59c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d59d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d59d); }
 let mut pc: u32 = 0x103d59d;
 loop { match pc {
 0x103d59d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d59e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d59e); }
 let mut pc: u32 = 0x103d59e;
 loop { match pc {
 0x103d59e => {
  { cpu.sub8(cpu.r8(0x10ee984u32), 0x0u8); }
  if !cpu.zf { pc = 0x103d5db; continue; }
  pc = 0x103d5a7; continue;
 }
 0x103d5a7 => {
  { let v = cpu.r16(0x10e3cf5u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3cf7u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x10ee986u32); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x10ee986u32, v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x103d5c7); f_01062caa(cpu); if cpu.returned_to != 0x103d5c7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee986u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3cf5u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3cf7u32, v); }
  pc = 0x103d5db; continue;
 }
 0x103d5db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d5dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103d5dc); } r_0103d5dc(cpu, 0x103d5dc); }
pub fn f_0103d61c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103d61c); } r_0103d5dc(cpu, 0x103d61c); }
fn r_0103d5dc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103d5dc => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x103d5ee; continue; }
  pc = 0x103d5e4; continue;
 }
 0x103d5e4 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d600; continue;
 }
 0x103d5ee => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103d5f4); f_01063324(cpu); if cpu.returned_to != 0x103d5f4 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x103d5f8; continue; }
  pc = 0x103d5f7; continue;
 }
 0x103d5f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d5f8 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d600; continue;
 }
 0x103d600 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103d61b); f_01062bdc(cpu); if cpu.returned_to != 0x103d61b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103d61c => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103d622); f_01062b13(cpu); if cpu.returned_to != 0x103d622 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x103d600; continue; }
  pc = 0x103d62b; continue;
 }
 0x103d62b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103d638); f_0106f7f8(cpu); if cpu.returned_to != 0x103d638 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d639(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d639); }
 let mut pc: u32 = 0x103d639;
 loop { match pc {
 0x103d639 => {
  { cpu.sub8(cpu.r8(0x112529fu32), 0x0u8); }
  if cpu.zf { pc = 0x103d79a; continue; }
  pc = 0x103d646; continue;
 }
 0x103d646 => {
  { cpu.sub8(cpu.r8(0x10ee92eu32), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x103d79a; continue; }
  pc = 0x103d653; continue;
 }
 0x103d653 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x103d67a; continue; }
  pc = 0x103d66d; continue;
 }
 0x103d66d => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d67a; continue;
 }
 0x103d67a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112528au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112528eu32, v); }
  { let v: u32 = cpu.r16(0x112528au32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x112528eu32) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x103d6ca); f_01018ffd(cpu); if cpu.returned_to != 0x103d6ca { return; }
  if cpu.cf { pc = 0x103d79a; continue; }
  pc = 0x103d6d0; continue;
 }
 0x103d6d0 => {
  { let v: u16 = cpu.r16(0x112528au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112528eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1125284u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1125286u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x112528cu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x1125288u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1125289u32, v); }
  { let v: u16 = cpu.r16(0x1125284u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125286u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u8 = cpu.r8(0x1125288u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103d72e; continue;
 }
 0x103d72e => {
  { let v: u8 = cpu.r8(0x1125288u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103d73a; continue;
 }
 0x103d73a => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x112529bu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x112529du32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x103d75c); f_0103db9f(cpu); if cpu.returned_to != 0x103d75c { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x112529du32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x112529bu32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x103d78a; continue; }
  pc = 0x103d76d; continue;
 }
 0x103d76d => {
  { cpu.sub8(cpu.r8(0x11252a1u32), 0x0u8); }
  if !cpu.zf { pc = 0x103d78a; continue; }
  pc = 0x103d776; continue;
 }
 0x103d776 => {
  { cpu.pushad(); }
  { let v: u32 = 0x4e7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103d782); cpu.call(t); if cpu.returned_to != 0x103d782 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x1125289u32, v); }
  pc = 0x103d78a; continue;
 }
 0x103d78a => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103d73a; continue; }
  pc = 0x103d792; continue;
 }
 0x103d792 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x103d72e; continue; }
  pc = 0x103d79a; continue;
 }
 0x103d79a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103d79b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103d79b); }
 let mut pc: u32 = 0x103d79b;
 loop { match pc {
 0x103d79b => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef37du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.sf { pc = 0x103db5d; continue; }
  pc = 0x103d7af; continue;
 }
 0x103d7af => {
  cpu.push(0x103d7b4); f_010338d0(cpu); if cpu.returned_to != 0x103d7b4 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103d7df; continue; }
  pc = 0x103d7bd; continue;
 }
 0x103d7bd => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103d7df; continue; }
  pc = 0x103d7c8; continue;
 }
 0x103d7c8 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x103d7ee; continue; }
  pc = 0x103d7cc; continue;
 }
 0x103d7cc => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x103d7fd; continue; }
  pc = 0x103d7d0; continue;
 }
 0x103d7d0 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d80c; continue;
 }
 0x103d7df => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d80c; continue;
 }
 0x103d7ee => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d80c; continue;
 }
 0x103d7fd => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d80c; continue;
 }
 0x103d80c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11252a4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11252a6u32, v); }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x10ef37du32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
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
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110ead8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x103d891; continue; }
  pc = 0x103d884; continue;
 }
 0x103d884 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d891; continue;
 }
 0x103d891 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112528au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112528eu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11252a0u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x103d8da; continue; }
  pc = 0x103d8d4; continue;
 }
 0x103d8d4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x103d8da; continue;
 }
 0x103d8da => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x112529au32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125298u32, v); }
  { let v: u32 = cpu.r16(0x112528au32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x112528eu32) as u32; cpu.edi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11252a4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11252a6u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x103d918); f_01018ffd(cpu); if cpu.returned_to != 0x103d918 { return; }
  if !cpu.cf { pc = 0x103daca; continue; }
  pc = 0x103d91e; continue;
 }
 0x103d91e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x11252a4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x11252a6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103d935); f_0103db5e(cpu); if cpu.returned_to != 0x103d935 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x1125244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125246u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x112529au32), 0x2u8); }
  if cpu.zf { pc = 0x103d95d; continue; }
  pc = 0x103d955; continue;
 }
 0x103d955 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d95d; continue;
 }
 0x103d95d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11252a4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11252a6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125298u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x5u8); }
  if !cpu.zf { pc = 0x103d98b; continue; }
  pc = 0x103d97b; continue;
 }
 0x103d97b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bde0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x112bde2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d9bb; continue;
 }
 0x103d98b => {
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x6u8); }
  if !cpu.zf { pc = 0x103d9a4; continue; }
  pc = 0x103d994; continue;
 }
 0x103d994 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112be00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x112be02u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d9bb; continue;
 }
 0x103d9a4 => {
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x4u8); }
  if !cpu.zf { pc = 0x103d9bb; continue; }
  pc = 0x103d9ad; continue;
 }
 0x103d9ad => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bdc0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x112bdc2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103d9bb; continue;
 }
 0x103d9bb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103d9cf); cpu.call(t); if cpu.returned_to != 0x103d9cf { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112529bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112529du32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  cpu.push(0x103d9f1); f_0108c1c5(cpu); if cpu.returned_to != 0x103d9f1 { return; }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x110eac4u32.wrapping_add(cpu.eax.wrapping_mul(4)))); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x103da30; continue; }
  pc = 0x103da04; continue;
 }
 0x103da04 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103da15); f_0103dda6(cpu); if cpu.returned_to != 0x103da15 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103da2f; continue; }
  pc = 0x103da1a; continue;
 }
 0x103da1a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103da2a); f_01019137(cpu); if cpu.returned_to != 0x103da2a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x103da61; continue; }
  pc = 0x103da2f; continue;
 }
 0x103da2f => {
  { cpu.popad(); }
  pc = 0x103da30; continue;
 }
 0x103da30 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103da53); cpu.call(t); if cpu.returned_to != 0x103da53 { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x112529bu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103da61 => {
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x5u8); }
  if !cpu.zf { pc = 0x103da8b; continue; }
  pc = 0x103da7b; continue;
 }
 0x103da7b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bde0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112bde2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103dabb; continue;
 }
 0x103da8b => {
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x6u8); }
  if !cpu.zf { pc = 0x103daa4; continue; }
  pc = 0x103da94; continue;
 }
 0x103da94 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112be00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112be02u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103dabb; continue;
 }
 0x103daa4 => {
  { cpu.sub8(cpu.r8(0x11252a0u32), 0x4u8); }
  if !cpu.zf { pc = 0x103dabb; continue; }
  pc = 0x103daad; continue;
 }
 0x103daad => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bdc0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112bdc2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103dabb; continue;
 }
 0x103dabb => {
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103daca => {
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1125298u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103dada); cpu.call(t); if cpu.returned_to != 0x103dada { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112529bu32, v); }
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x103daf5; continue; }
  pc = 0x103daf4; continue;
 }
 0x103daf4 => {
  {  }
  pc = 0x103daf5; continue;
 }
 0x103daf5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112529du32, v); }
  cpu.push(0x103db01); f_0103db9f(cpu); if cpu.returned_to != 0x103db01 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103db43; continue; }
  pc = 0x103db05; continue;
 }
 0x103db05 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103db1d); f_0103de84(cpu); if cpu.returned_to != 0x103db1d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103db43; continue; }
  pc = 0x103db21; continue;
 }
 0x103db21 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10f582du32; cpu.w32(0x10e415cu32, v); }
  cpu.push(0x103db3f); f_010590d3(cpu); if cpu.returned_to != 0x103db3f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x103db5d; continue; }
  pc = 0x103db43; continue;
 }
 0x103db43 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103db54); cpu.call(t); if cpu.returned_to != 0x103db54 { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x112529bu32, v); }
  pc = 0x103db5d; continue;
 }
 0x103db5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103db5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103db5e); }
 let mut pc: u32 = 0x103db5e;
 loop { match pc {
 0x103db5e => {
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if (cpu.sf == cpu.of) { pc = 0x103db82; continue; }
  pc = 0x103db64; continue;
 }
 0x103db64 => {
  { cpu.sub16((cpu.ebx as u16), 0x1fu16); }
  if (cpu.sf != cpu.of) { pc = 0x103db76; continue; }
  pc = 0x103db6a; continue;
 }
 0x103db6a => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103db7c; continue; }
  pc = 0x103db70; continue;
 }
 0x103db70 => {
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db76 => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db7c => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db82 => {
  { cpu.sub16((cpu.ebx as u16), 0x1fu16); }
  if (cpu.sf != cpu.of) { pc = 0x103db94; continue; }
  pc = 0x103db88; continue;
 }
 0x103db88 => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103db9a; continue; }
  pc = 0x103db8e; continue;
 }
 0x103db8e => {
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db94 => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db9a => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x103db9e; continue;
 }
 0x103db9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103db9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103db9f); }
 let mut pc: u32 = 0x103db9f;
 loop { match pc {
 0x103db9f => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11252a1u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11252a8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11252acu32, v); }
  cpu.push(0x103dbc4); f_0108c1c5(cpu); if cpu.returned_to != 0x103dbc4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11337cfu32), 0x0u8); }
  if !cpu.zf { pc = 0x103dbd8; continue; }
  pc = 0x103dbd1; continue;
 }
 0x103dbd1 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  pc = 0x103dbd8; continue;
 }
 0x103dbd8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x110eac4u32.wrapping_add(cpu.eax.wrapping_mul(4)))); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x103dd8e; continue; }
  pc = 0x103dbe6; continue;
 }
 0x103dbe6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x112529au32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1125298u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11252a8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11252acu32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11252a4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11252a6u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x103dc42); f_01018ffd(cpu); if cpu.returned_to != 0x103dc42 { return; }
  if !cpu.cf { pc = 0x103dd12; continue; }
  pc = 0x103dc48; continue;
 }
 0x103dc48 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11252a1u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x11252a4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x11252a6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103dc66); f_0103db5e(cpu); if cpu.returned_to != 0x103dc66 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x1125244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125246u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x112529au32), 0x2u8); }
  if cpu.zf { pc = 0x103dc8e; continue; }
  pc = 0x103dc86; continue;
 }
 0x103dc86 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103dc8e; continue;
 }
 0x103dc8e => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11252a4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11252a6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125298u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112529bu32, v); }
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x103dcc0; continue; }
  pc = 0x103dcbf; continue;
 }
 0x103dcbf => {
  {  }
  pc = 0x103dcc0; continue;
 }
 0x103dcc0 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112529du32, v); }
  { cpu.pushad(); }
  cpu.push(0x103dccd); f_0103dda6(cpu); if cpu.returned_to != 0x103dccd { return; }
  if cpu.cf { pc = 0x103dcf1; continue; }
  pc = 0x103dccf; continue;
 }
 0x103dccf => {
  { cpu.sub8(cpu.r8(0x11337cfu32), 0x0u8); }
  if cpu.zf { pc = 0x103dcf1; continue; }
  pc = 0x103dcd8; continue;
 }
 0x103dcd8 => {
  {  }
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x28u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x103dceb); f_010726f9(cpu); if cpu.returned_to != 0x103dceb { return; }
  { cpu.popad(); }
  pc = 0x103dd90; continue;
 }
 0x103dcf1 => {
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103dd12 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r16(0x11252a8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11252acu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11252a8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11252acu32) as u32; cpu.ebx = v; }
  cpu.push(0x103dd36); f_0103de84(cpu); if cpu.returned_to != 0x103dd36 { return; }
  if cpu.cf { pc = 0x103dd8e; continue; }
  pc = 0x103dd38; continue;
 }
 0x103dd38 => {
  { let v: u32 = cpu.r16(0x11252a8u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11252acu32) as u32; cpu.ebx = v; }
  { cpu.pushad(); }
  cpu.push(0x103dd4c); f_0103dda6(cpu); if cpu.returned_to != 0x103dd4c { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x103dd8e; continue; }
  pc = 0x103dd4f; continue;
 }
 0x103dd4f => {
  { let v: u32 = cpu.r32(0x11252a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11252acu32); cpu.ebx = v; }
  { cpu.pushad(); }
  cpu.push(0x103dd60); f_0101b70a(cpu); if cpu.returned_to != 0x103dd60 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x103dd8e; continue; }
  pc = 0x103dd63; continue;
 }
 0x103dd63 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103dd6a); f_01019137(cpu); if cpu.returned_to != 0x103dd6a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103dd8e; continue; }
  pc = 0x103dd6e; continue;
 }
 0x103dd6e => {
  { let v: u32 = cpu.r32(0x11252a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11252acu32); cpu.ebx = v; }
  { let v: u32 = 0x10f582du32; cpu.w32(0x10e415cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103dd8a); f_010590d3(cpu); if cpu.returned_to != 0x103dd8a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x103dd90; continue; }
  pc = 0x103dd8e; continue;
 }
 0x103dd8e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103dd90 => {
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x103dd97; continue; }
  pc = 0x103dd96; continue;
 }
 0x103dd96 => {
  {  }
  pc = 0x103dd97; continue;
 }
 0x103dd97 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112529bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112529du32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103dcf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103dcf7); }
 let mut pc: u32 = 0x103dcf7;
 loop { match pc {
 0x103dcf7 => {
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103dda6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103dda6); }
 let mut pc: u32 = 0x103dda6;
 loop { match pc {
 0x103dda6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x103ddb1; continue;
 }
 0x103ddb1 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x103de6b; continue; }
  pc = 0x103ddbb; continue;
 }
 0x103ddbb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x103de6b; continue; }
  pc = 0x103ddc5; continue;
 }
 0x103ddc5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.and32(cpu.edi, 0xfff0u32); cpu.edi = v; }
  cpu.push(0x103ddf0); f_01018ffd(cpu); if cpu.returned_to != 0x103ddf0 { return; }
  if cpu.cf { pc = 0x103de7b; continue; }
  pc = 0x103ddf6; continue;
 }
 0x103ddf6 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x20u32); cpu.esi = v; }
  cpu.push(0x103de18); f_01018ffd(cpu); if cpu.returned_to != 0x103de18 { return; }
  if cpu.cf { pc = 0x103de7b; continue; }
  pc = 0x103de1a; continue;
 }
 0x103de1a => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x103de3c); f_01018ffd(cpu); if cpu.returned_to != 0x103de3c { return; }
  if cpu.cf { pc = 0x103de7b; continue; }
  pc = 0x103de3e; continue;
 }
 0x103de3e => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x103de63); f_01018ffd(cpu); if cpu.returned_to != 0x103de63 { return; }
  if cpu.cf { pc = 0x103de7b; continue; }
  pc = 0x103de65; continue;
 }
 0x103de65 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x103de6b; continue;
 }
 0x103de6b => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103ddb1; continue; }
  pc = 0x103de78; continue;
 }
 0x103de78 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103de7b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
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
pub fn f_0103de84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103de84); }
 let mut pc: u32 = 0x103de84;
 loop { match pc {
 0x103de84 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x103de93; continue;
 }
 0x103de93 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103df5d; continue; }
  pc = 0x103de9e; continue;
 }
 0x103de9e => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x103df5d; continue; }
  pc = 0x103dea8; continue;
 }
 0x103dea8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd4bfu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x10c6120u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x11u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10fd4bfu32), 0x1u8); }
  if !cpu.zf { pc = 0x103dee2; continue; }
  pc = 0x103dedf; continue;
 }
 0x103dedf => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x103dee2; continue;
 }
 0x103dee2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4144u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e4138u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x10e413cu32)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x103df2c); f_01018ffd(cpu); if cpu.returned_to != 0x103df2c { return; }
  if cpu.cf { pc = 0x103df6b; continue; }
  pc = 0x103df2e; continue;
 }
 0x103df2e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x20u32); cpu.esi = v; }
  cpu.push(0x103df3a); f_01018ffd(cpu); if cpu.returned_to != 0x103df3a { return; }
  if cpu.cf { pc = 0x103df6b; continue; }
  pc = 0x103df3c; continue;
 }
 0x103df3c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x103df48); f_01018ffd(cpu); if cpu.returned_to != 0x103df48 { return; }
  if cpu.cf { pc = 0x103df6b; continue; }
  pc = 0x103df4a; continue;
 }
 0x103df4a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x103df59); f_01018ffd(cpu); if cpu.returned_to != 0x103df59 { return; }
  if cpu.cf { pc = 0x103df6b; continue; }
  pc = 0x103df5b; continue;
 }
 0x103df5b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103df5d; continue;
 }
 0x103df5d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103de93; continue; }
  pc = 0x103df68; continue;
 }
 0x103df68 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103df6b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103df71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103df71); }
 let mut pc: u32 = 0x103df71;
 loop { match pc {
 0x103df71 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103dfa8; continue; }
  pc = 0x103df7a; continue;
 }
 0x103df7a => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x103df84; continue;
 }
 0x103df84 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.sf { pc = 0x103dfa2; continue; }
  pc = 0x103df8a; continue;
 }
 0x103df8a => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x103dfa2; continue; }
  pc = 0x103df93; continue;
 }
 0x103df93 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103dfa2; continue; }
  pc = 0x103df99; continue;
 }
 0x103df99 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103dfa0); f_0103dfa9(cpu); if cpu.returned_to != 0x103dfa0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x103dfa2; continue;
 }
 0x103dfa2 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103df84; continue; }
  pc = 0x103dfa8; continue;
 }
 0x103dfa8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103dfa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103dfa9); }
 let mut pc: u32 = 0x103dfa9;
 loop { match pc {
 0x103dfa9 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103dfcd; continue; }
  pc = 0x103dfaf; continue;
 }
 0x103dfaf => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x103dfca); f_010324eb(cpu); if cpu.returned_to != 0x103dfca { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { cpu.poll(); pc = 0x103dfa9; continue; }
  pc = 0x103dfcd; continue;
 }
 0x103dfcd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103dfce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103dfce); }
 let mut pc: u32 = 0x103dfce;
 loop { match pc {
 0x103dfce => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103dfe0); f_0108c1c5(cpu); if cpu.returned_to != 0x103dfe0 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11337cfu32), 0x0u8); }
  if !cpu.zf { pc = 0x103dff2; continue; }
  pc = 0x103dfeb; continue;
 }
 0x103dfeb => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  pc = 0x103dff2; continue;
 }
 0x103dff2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x110eac4u32.wrapping_add(cpu.eax.wrapping_mul(4)))); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x103e03e; continue; }
  pc = 0x103dffd; continue;
 }
 0x103dffd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103e004); f_0103dda6(cpu); if cpu.returned_to != 0x103e004 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103e03e; continue; }
  pc = 0x103e008; continue;
 }
 0x103e008 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103e00f); f_0101b70a(cpu); if cpu.returned_to != 0x103e00f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x103e03e; continue; }
  pc = 0x103e013; continue;
 }
 0x103e013 => {
  { cpu.sub8(cpu.r8(0x11337cfu32), 0x0u8); }
  if !cpu.zf { pc = 0x103e040; continue; }
  pc = 0x103e01c; continue;
 }
 0x103e01c => {
  cpu.push(0x103e021); f_0103e042(cpu); if cpu.returned_to != 0x103e021 { return; }
  if !cpu.cf { pc = 0x103e03d; continue; }
  pc = 0x103e023; continue;
 }
 0x103e023 => {
  { let v: u8 = 0x5u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f582du32; cpu.w32(0x10e415cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103e036); f_010590d3(cpu); if cpu.returned_to != 0x103e036 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  pc = 0x103e03d; continue;
 }
 0x103e03d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e03e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e040 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e042(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e042); }
 let mut pc: u32 = 0x103e042;
 loop { match pc {
 0x103e042 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1125290u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1125292u32, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x103e07d); f_01018ffd(cpu); if cpu.returned_to != 0x103e07d { return; }
  if cpu.cf { pc = 0x103e082; continue; }
  pc = 0x103e07f; continue;
 }
 0x103e07f => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e082 => {
  { let v: u16 = cpu.r16(0x1125290u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125292u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3d1fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3d21u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x103e0a8); f_0103e0bc(cpu); if cpu.returned_to != 0x103e0a8 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16((cpu.ecx as u16), 0xffffu16); }
  if cpu.zf { cpu.poll(); pc = 0x103e07f; continue; }
  pc = 0x103e0b0; continue;
 }
 0x103e0b0 => {
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e0bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e0bc); }
 let mut pc: u32 = 0x103e0bc;
 loop { match pc {
 0x103e0bc => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1125214u32; cpu.ebp = v; }
  pc = 0x103e0ca; continue;
 }
 0x103e0ca => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x103e0df); f_01018ffd(cpu); if cpu.returned_to != 0x103e0df { return; }
  if cpu.cf { pc = 0x103e0fa; continue; }
  pc = 0x103e0e1; continue;
 }
 0x103e0e1 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x4u16); }
  if !cpu.zf { cpu.poll(); pc = 0x103e0ca; continue; }
  pc = 0x103e0f5; continue;
 }
 0x103e0f5 => {
  { let v: u16 = 0xffffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e0fa => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e104(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e104); }
 let mut pc: u32 = 0x103e104;
 loop { match pc {
 0x103e104 => {
  { let v: u32 = 0x10e6910u32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u16 = 0x230u16; cpu.w16(0x10e3d1bu32, v); }
  { let v: u16 = 0x80u16; cpu.w16(0x10e3d1du32, v); }
  { let v: u16 = 0x230u16; cpu.w16(0x10e3d1fu32, v); }
  { let v: u16 = 0x4e0u16; cpu.w16(0x10e3d21u32, v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2410u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d2bu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d2du32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d33u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d35u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d37u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d39u32, v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2414u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d2fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d31u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103e1c9; continue; }
  pc = 0x103e1b3; continue;
 }
 0x103e1b3 => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if cpu.cf { pc = 0x103e1c9; continue; }
  pc = 0x103e1bc; continue;
 }
 0x103e1bc => {
  { cpu.sub8(cpu.r8(0x10e3d7fu32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103e268; continue; }
  pc = 0x103e1c9; continue;
 }
 0x103e1c9 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x103e221; continue; }
  pc = 0x103e1ce; continue;
 }
 0x103e1ce => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x103e221; continue; }
  pc = 0x103e1d7; continue;
 }
 0x103e1d7 => {
  { cpu.sub8(cpu.r8(0x4204u32.wrapping_add(cpu.esi)), 0x15u8); }
  if !cpu.zf { pc = 0x103e1fc; continue; }
  pc = 0x103e1e0; continue;
 }
 0x103e1e0 => {
  { let v: u8 = 0x15u8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x15u8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e1fc; continue;
 }
 0x103e1fc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103e203); f_0103e48e(cpu); if cpu.returned_to != 0x103e203 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d1bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d1du32, v); }
  { let v: u32 = 0x680u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x680u32); cpu.ecx = v; }
  if !cpu.sf { pc = 0x103e22d; continue; }
  pc = 0x103e220; continue;
 }
 0x103e220 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e221 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103e1c9; continue; }
  pc = 0x103e223; continue;
 }
 0x103e223 => {
  { let v: u32 = 0x10e8910u32; cpu.esi = v; }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  pc = 0x103e22d; continue;
 }
 0x103e22d => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x103e265; continue; }
  pc = 0x103e232; continue;
 }
 0x103e232 => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x103e265; continue; }
  pc = 0x103e23b; continue;
 }
 0x103e23b => {
  { cpu.sub8(cpu.r8(0x4204u32.wrapping_add(cpu.esi)), 0x14u8); }
  if !cpu.zf { pc = 0x103e252; continue; }
  pc = 0x103e244; continue;
 }
 0x103e244 => {
  { let v: u8 = 0x3eu8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3eu8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e252; continue;
 }
 0x103e252 => {
  cpu.push(0x103e257); f_0103e48e(cpu); if cpu.returned_to != 0x103e257 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d1fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d21u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e265 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103e22d; continue; }
  pc = 0x103e267; continue;
 }
 0x103e267 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e268 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3d1bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3d1fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3d23u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3d27u32, v); }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11de596u32, v); }
  pc = 0x103e29c; continue;
 }
 0x103e29c => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x103e347; continue; }
  pc = 0x103e2a5; continue;
 }
 0x103e2a5 => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x103e347; continue; }
  pc = 0x103e2b2; continue;
 }
 0x103e2b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103e2b9); f_0103e464(cpu); if cpu.returned_to != 0x103e2b9 { return; }
  cpu.push(0x103e2be); f_0103e48e(cpu); if cpu.returned_to != 0x103e2be { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103e2cc); f_0103e39f(cpu); if cpu.returned_to != 0x103e2cc { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x103e346; continue; }
  pc = 0x103e2d3; continue;
 }
 0x103e2d3 => {
  cpu.push(0x103e2d8); f_0103e3e4(cpu); if cpu.returned_to != 0x103e2d8 { return; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x103e2ed; continue; }
  pc = 0x103e2dd; continue;
 }
 0x103e2dd => {
  { let v: u8 = 0x3eu8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3eu8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e346; continue;
 }
 0x103e2ed => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x103e310; continue; }
  pc = 0x103e2f2; continue;
 }
 0x103e2f2 => {
  { let v: u8 = 0x15u8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x15u8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e346; continue;
 }
 0x103e310 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x103e325; continue; }
  pc = 0x103e315; continue;
 }
 0x103e315 => {
  { let v: u8 = 0x40u8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x40u8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e346; continue;
 }
 0x103e325 => {
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x103e346; continue; }
  pc = 0x103e32a; continue;
 }
 0x103e32a => {
  { let v: u8 = 0x3du8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3du8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x41u8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x41u8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x103e346; continue;
 }
 0x103e346 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x103e347; continue;
 }
 0x103e347 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103e29c; continue; }
  pc = 0x103e34e; continue;
 }
 0x103e34e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e34f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e34f); }
 let mut pc: u32 = 0x103e34f;
 loop { match pc {
 0x103e34f => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x103e35b; continue;
 }
 0x103e35b => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103e368; continue; }
  pc = 0x103e361; continue;
 }
 0x103e361 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103e35b; continue; }
  pc = 0x103e366; continue;
 }
 0x103e366 => {
  pc = 0x103e39b; continue;
 }
 0x103e368 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x10e3d63u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x110d895u32.wrapping_add(cpu.ecx.wrapping_mul(2))) as u32; cpu.eax = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e39b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e39f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e39f); }
 let mut pc: u32 = 0x103e39f;
 loop { match pc {
 0x103e39f => {
  { let v: u8 = cpu.r8(0x4204u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x14u8); }
  if cpu.zf { pc = 0x103e3d5; continue; }
  pc = 0x103e3ab; continue;
 }
 0x103e3ab => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x103e3d0; continue; }
  pc = 0x103e3af; continue;
 }
 0x103e3af => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103e3cb; continue; }
  pc = 0x103e3b9; continue;
 }
 0x103e3b9 => {
  { cpu.sub8((cpu.eax as u8), 0x3cu8); }
  if cpu.zf { pc = 0x103e3da; continue; }
  pc = 0x103e3bd; continue;
 }
 0x103e3bd => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if !cpu.zf { pc = 0x103e3cb; continue; }
  pc = 0x103e3c7; continue;
 }
 0x103e3c7 => {
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x103e3df; continue; }
  pc = 0x103e3cb; continue;
 }
 0x103e3cb => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e3d0 => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e3d5 => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e3da => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e3df => {
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e3e4); }
 let mut pc: u32 = 0x103e3e4;
 loop { match pc {
 0x103e3e4 => {
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if cpu.zf { pc = 0x103e441; continue; }
  pc = 0x103e3ea; continue;
 }
 0x103e3ea => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x103e428; continue; }
  pc = 0x103e3f0; continue;
 }
 0x103e3f0 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x103e40f; continue; }
  pc = 0x103e3f6; continue;
 }
 0x103e3f6 => {
  { cpu.sub16(cpu.r16(0x10e3d27u32), 0x0u16); }
  if !cpu.zf { pc = 0x103e45f; continue; }
  pc = 0x103e400; continue;
 }
 0x103e400 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d27u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d29u32, v); }
  pc = 0x103e458; continue;
 }
 0x103e40f => {
  { cpu.sub16(cpu.r16(0x10e3d23u32), 0x0u16); }
  if !cpu.zf { pc = 0x103e45f; continue; }
  pc = 0x103e419; continue;
 }
 0x103e419 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d23u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d25u32, v); }
  pc = 0x103e458; continue;
 }
 0x103e428 => {
  { cpu.sub16(cpu.r16(0x10e3d1fu32), 0x0u16); }
  if !cpu.zf { pc = 0x103e45f; continue; }
  pc = 0x103e432; continue;
 }
 0x103e432 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d1fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d21u32, v); }
  pc = 0x103e458; continue;
 }
 0x103e441 => {
  { cpu.sub16(cpu.r16(0x10e3d1bu32), 0x0u16); }
  if !cpu.zf { pc = 0x103e45f; continue; }
  pc = 0x103e44b; continue;
 }
 0x103e44b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d1bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d1du32, v); }
  pc = 0x103e458; continue;
 }
 0x103e458 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11de596u32)); cpu.w8(0x11de596u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e45f => {
  { let v: u16 = 0xffffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e464); }
 let mut pc: u32 = 0x103e464;
 loop { match pc {
 0x103e464 => {
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)), 0xeeu8); }
  if !cpu.zf { pc = 0x103e476; continue; }
  pc = 0x103e46a; continue;
 }
 0x103e46a => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x103e476; continue; }
  pc = 0x103e473; continue;
 }
 0x103e473 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x103e464; continue;
 }
 0x103e476 => {
  { cpu.sub8(cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)), 0xeeu8); }
  if !cpu.zf { pc = 0x103e48d; continue; }
  pc = 0x103e47c; continue;
 }
 0x103e47c => {
  { cpu.and8(cpu.r8(0x3f80u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x103e48d; continue; }
  pc = 0x103e485; continue;
 }
 0x103e485 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x80u32); cpu.esi = v; }
  cpu.poll(); pc = 0x103e476; continue;
 }
 0x103e48d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e48e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e48e); }
 let mut pc: u32 = 0x103e48e;
 loop { match pc {
 0x103e48e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x10e6911u32); cpu.esi = v; }
  { let v: u8 = 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e4ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e4ac); }
 let mut pc: u32 = 0x103e4ac;
 loop { match pc {
 0x103e4ac => {
  cpu.push(0x103e4b1); f_01051303(cpu); if cpu.returned_to != 0x103e4b1 { return; }
  cpu.push(0x103e4b6); f_0105996f(cpu); if cpu.returned_to != 0x103e4b6 { return; }
  cpu.push(0x103e4bb); f_01048959(cpu); if cpu.returned_to != 0x103e4bb { return; }
  cpu.push(0x103e4c0); f_0104944f(cpu); if cpu.returned_to != 0x103e4c0 { return; }
  cpu.push(0x103e4c5); f_01072805(cpu); if cpu.returned_to != 0x103e4c5 { return; }
  cpu.push(0x103e4ca); f_01049d9b(cpu); if cpu.returned_to != 0x103e4ca { return; }
  cpu.push(0x103e4cf); f_01039b33(cpu); if cpu.returned_to != 0x103e4cf { return; }
  cpu.push(0x103e4d4); f_01049f64(cpu); if cpu.returned_to != 0x103e4d4 { return; }
  cpu.push(0x103e4d9); f_0103e7d6(cpu); if cpu.returned_to != 0x103e4d9 { return; }
  cpu.push(0x103e4de); f_01033502(cpu); if cpu.returned_to != 0x103e4de { return; }
  cpu.push(0x103e4e3); f_01066631(cpu); if cpu.returned_to != 0x103e4e3 { return; }
  cpu.push(0x103e4e8); f_0103e7c7(cpu); if cpu.returned_to != 0x103e4e8 { return; }
  cpu.push(0x103e4ed); f_0105c66f(cpu); if cpu.returned_to != 0x103e4ed { return; }
  cpu.push(0x103e4f2); f_0103e780(cpu); if cpu.returned_to != 0x103e4f2 { return; }
  cpu.push(0x103e4f7); f_0103e7b7(cpu); if cpu.returned_to != 0x103e4f7 { return; }
  cpu.push(0x103e4fc); f_0103e76e(cpu); if cpu.returned_to != 0x103e4fc { return; }
  cpu.push(0x103e501); f_0103e75c(cpu); if cpu.returned_to != 0x103e501 { return; }
  cpu.push(0x103e506); f_0104a6cc(cpu); if cpu.returned_to != 0x103e506 { return; }
  cpu.push(0x103e50b); f_0101bde6(cpu); if cpu.returned_to != 0x103e50b { return; }
  cpu.push(0x103e510); f_01077055(cpu); if cpu.returned_to != 0x103e510 { return; }
  cpu.push(0x103e515); f_0107706d(cpu); if cpu.returned_to != 0x103e515 { return; }
  cpu.push(0x103e51a); f_01077e51(cpu); if cpu.returned_to != 0x103e51a { return; }
  cpu.push(0x103e51f); f_0105d28c(cpu); if cpu.returned_to != 0x103e51f { return; }
  cpu.push(0x103e524); f_010867c0(cpu); if cpu.returned_to != 0x103e524 { return; }
  cpu.push(0x103e529); f_010027b8(cpu); if cpu.returned_to != 0x103e529 { return; }
  cpu.push(0x103e52e); f_01065b90(cpu); if cpu.returned_to != 0x103e52e { return; }
  cpu.push(0x103e533); f_0103f023(cpu); if cpu.returned_to != 0x103e533 { return; }
  { let v: u32 = 0x115f5b8u32; cpu.edi = v; }
  cpu.push(0x103e53d); f_0107281d(cpu); if cpu.returned_to != 0x103e53d { return; }
  { let v: u32 = 0x115f93cu32; cpu.edi = v; }
  cpu.push(0x103e547); f_0107281d(cpu); if cpu.returned_to != 0x103e547 { return; }
  { let v: u32 = 0x115fcc0u32; cpu.edi = v; }
  cpu.push(0x103e551); f_0107281d(cpu); if cpu.returned_to != 0x103e551 { return; }
  { let v: u32 = 0x1160044u32; cpu.edi = v; }
  cpu.push(0x103e55b); f_0107281d(cpu); if cpu.returned_to != 0x103e55b { return; }
  cpu.push(0x103e560); f_01072843(cpu); if cpu.returned_to != 0x103e560 { return; }
  cpu.push(0x103e565); f_0103e730(cpu); if cpu.returned_to != 0x103e565 { return; }
  cpu.push(0x103e56a); f_0103e6b5(cpu); if cpu.returned_to != 0x103e56a { return; }
  cpu.push(0x103e56f); f_010176af(cpu); if cpu.returned_to != 0x103e56f { return; }
  { let v: u16 = 0x30u16; cpu.w16(0x1122f98u32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x1122f94u32, v); }
  { let v: u16 = 0x32u16; cpu.w16(0x1122f96u32, v); }
  { let v: u16 = cpu.r16(0x1125282u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1125281u32, v); }
  { let v: u32 = 0x13ecu32; cpu.w32(0x11c62fcu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd44au32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c62fau32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c5f46u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c5f47u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee92du32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee92eu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11b5420u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11b5424u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e418cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e41a8u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee91bu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d07u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee988u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef203u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef206u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef20du32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef20bu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1123048u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef325u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e3fbcu32, v); }
  { let v: u32 = 0x1152d9cu32; cpu.w32(0x10e3f90u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3de9u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce1u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3debu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f74u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef371u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e41c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb12cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1123049u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110eab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110eab8u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10ef37du32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x110ed86u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x110ed88u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.w32(0x11de578u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.w32(0x11de57cu32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.w32(0x11de580u32, v); }
  { let v: u32 = 0xddu32; cpu.eax = v; }
  { let v = cpu.r32(0x116e72cu32); cpu.mul32(v); }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x116e72cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e6b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e6b5); }
 let mut pc: u32 = 0x103e6b5;
 loop { match pc {
 0x103e6b5 => {
  { let v: u32 = 0x10ea910u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x103e6cb); f_0103e722(cpu); if cpu.returned_to != 0x103e6cb { return; }
  { let v: u32 = 0x10ea910u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x103e6e1); f_0103e722(cpu); if cpu.returned_to != 0x103e6e1 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e690eu32); cpu.mul8(v); }
  { let v: u32 = 0x10ea890u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x103e703); f_0103e722(cpu); if cpu.returned_to != 0x103e703 { return; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x10ea90fu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x103e721); f_0103e722(cpu); if cpu.returned_to != 0x103e721 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e722); }
 let mut pc: u32 = 0x103e722;
 loop { match pc {
 0x103e722 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103e722; continue; }
  pc = 0x103e72f; continue;
 }
 0x103e72f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e730); }
 let mut pc: u32 = 0x103e730;
 loop { match pc {
 0x103e730 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x103e737; continue;
 }
 0x103e737 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103e737; continue; }
  pc = 0x103e748; continue;
 }
 0x103e748 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e749(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e749); }
 let mut pc: u32 = 0x103e749;
 loop { match pc {
 0x103e749 => {
  { let v: u32 = 0x116c0d8u32; cpu.edi = v; }
  { let v: u32 = 0x578u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e75c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e75c); }
 let mut pc: u32 = 0x103e75c;
 loop { match pc {
 0x103e75c => {
  { let v: u32 = 0x116b520u32; cpu.edi = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e76e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e76e); }
 let mut pc: u32 = 0x103e76e;
 loop { match pc {
 0x103e76e => {
  { let v: u32 = 0x116a9e0u32; cpu.edi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e780); }
 let mut pc: u32 = 0x103e780;
 loop { match pc {
 0x103e780 => {
  { let v: u32 = 0x116a8b4u32; cpu.edi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e791); }
 let mut pc: u32 = 0x103e791;
 loop { match pc {
 0x103e791 => {
  { let v: u32 = 0x10c09ccu32; cpu.edi = v; }
  { let v: u32 = 0x154u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7a4); }
 let mut pc: u32 = 0x103e7a4;
 loop { match pc {
 0x103e7a4 => {
  { let v: u32 = 0x10ef644u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7b7); }
 let mut pc: u32 = 0x103e7b7;
 loop { match pc {
 0x103e7b7 => {
  { let v: u32 = cpu.r32(0x10e4074u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7c7); }
 let mut pc: u32 = 0x103e7c7;
 loop { match pc {
 0x103e7c7 => {
  { let v: u32 = 0x10f5734u32; cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7d6); }
 let mut pc: u32 = 0x103e7d6;
 loop { match pc {
 0x103e7d6 => {
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0x2b20u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7e5); }
 let mut pc: u32 = 0x103e7e5;
 loop { match pc {
 0x103e7e5 => {
  { let v: u32 = 0x10c7adbu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x3840u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e7f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e7f7); }
 let mut pc: u32 = 0x103e7f7;
 loop { match pc {
 0x103e7f7 => {
  { let v: u32 = 0x114eed4u32; cpu.edi = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x10c09ccu32; cpu.esi = v; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103e824; continue;
 }
 0x103e824 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x103e82d; continue;
 }
 0x103e82d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x103e848; continue; }
  pc = 0x103e834; continue;
 }
 0x103e834 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1ffu32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103e82d; continue; }
  pc = 0x103e848; continue;
 }
 0x103e848 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x22u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x103e824; continue; }
  pc = 0x103e852; continue;
 }
 0x103e852 => {
  { let v: u32 = 0x10e6910u32; cpu.esi = v; }
  { let v: u32 = 0x114eed4u32; cpu.edi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  pc = 0x103e861; continue;
 }
 0x103e861 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1ffu32; cpu.eax = v; }
  { let v: u32 = 0x1194208u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103e8e9; continue; }
  pc = 0x103e87c; continue;
 }
 0x103e87c => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x10c09ccu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10e3f4cu32)); cpu.w32(0x10e3f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x10e6910u32); cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x10e6910u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x103e8ca; continue; }
  pc = 0x103e8c3; continue;
 }
 0x103e8c3 => {
  cpu.push(0x103e8c8); f_0105f215(cpu); if cpu.returned_to != 0x103e8c8 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103e8ca; continue;
 }
 0x103e8ca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x1au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x103e8db; continue; }
  pc = 0x103e8d3; continue;
 }
 0x103e8d3 => {
  cpu.push(0x103e8d8); f_0105f215(cpu); if cpu.returned_to != 0x103e8d8 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  pc = 0x103e8db; continue;
 }
 0x103e8db => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1152d54u32); }
  if !cpu.cf { pc = 0x103e8f1; continue; }
  pc = 0x103e8e9; continue;
 }
 0x103e8e9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103e861; continue; }
  pc = 0x103e8f1; continue;
 }
 0x103e8f1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e8f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e8f2); }
 let mut pc: u32 = 0x103e8f2;
 loop { match pc {
 0x103e8f2 => {
  cpu.push(0x103e8f7); f_0103ec28(cpu); if cpu.returned_to != 0x103e8f7 { return; }
  cpu.push(0x103e8fc); f_0100144a(cpu); if cpu.returned_to != 0x103e8fc { return; }
  { let v: u32 = 0x10f7d35u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x103e90c); f_010019bc(cpu); if cpu.returned_to != 0x103e90c { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x103e939; continue; }
  pc = 0x103e913; continue;
 }
 0x103e913 => {
  { let v: u8 = 0x32u8; cpu.w8(0x116e730u32, v); }
  { let v: u32 = 0x10f7d35u32; cpu.edx = v; }
  { let v: u32 = 0x116de88u32; cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef31eu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x103e936); f_01033bea(cpu); if cpu.returned_to != 0x103e936 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103e939 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e93b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e93b); }
 let mut pc: u32 = 0x103e93b;
 loop { match pc {
 0x103e93b => {
  cpu.push(0x103e940); f_0100144a(cpu); if cpu.returned_to != 0x103e940 { return; }
  cpu.push(0x103e945); f_01078eb2(cpu); if cpu.returned_to != 0x103e945 { return; }
  cpu.push(0x103e94a); f_0103e8f2(cpu); if cpu.returned_to != 0x103e94a { return; }
  cpu.push(0x103e94f); f_01046622(cpu); if cpu.returned_to != 0x103e94f { return; }
  cpu.push(0x103e954); f_0106257c(cpu); if cpu.returned_to != 0x103e954 { return; }
  cpu.push(0x103e959); f_0103e987(cpu); if cpu.returned_to != 0x103e959 { return; }
  cpu.push(0x103e95e); f_0103ebe1(cpu); if cpu.returned_to != 0x103e95e { return; }
  cpu.push(0x103e963); f_0103e9c8(cpu); if cpu.returned_to != 0x103e963 { return; }
  cpu.push(0x103e968); f_0103eb7c(cpu); if cpu.returned_to != 0x103e968 { return; }
  cpu.push(0x103e96d); f_0103e9ee(cpu); if cpu.returned_to != 0x103e96d { return; }
  cpu.push(0x103e972); f_0103eb9a(cpu); if cpu.returned_to != 0x103e972 { return; }
  cpu.push(0x103e977); f_0103ea16(cpu); if cpu.returned_to != 0x103e977 { return; }
  cpu.push(0x103e97c); f_0103ebc0(cpu); if cpu.returned_to != 0x103e97c { return; }
  cpu.push(0x103e981); f_0103ea3c(cpu); if cpu.returned_to != 0x103e981 { return; }
  cpu.push(0x103e986); f_0103ea62(cpu); if cpu.returned_to != 0x103e986 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e987); }
 let mut pc: u32 = 0x103e987;
 loop { match pc {
 0x103e987 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4bf2u32, v); }
  { let v: u32 = 0x10e6910u32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  cpu.push(0x103e9a0); f_0103903f(cpu); if cpu.returned_to != 0x103e9a0 { return; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4bf2u32, v); }
  { let v: u32 = 0x10ea910u32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  cpu.push(0x103e9bb); f_0103903f(cpu); if cpu.returned_to != 0x103e9bb { return; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x11b4bf2u32), cpu.edx); cpu.w32(0x11b4bf2u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e9c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e9c8); }
 let mut pc: u32 = 0x103e9c8;
 loop { match pc {
 0x103e9c8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4bfau32, v); }
  { let v: u32 = 0x10c5364u32; cpu.esi = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  cpu.push(0x103e9e1); f_0103903f(cpu); if cpu.returned_to != 0x103e9e1 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4bfau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103e9ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103e9ee); }
 let mut pc: u32 = 0x103e9ee;
 loop { match pc {
 0x103e9ee => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4c0eu32, v); }
  { let v: u32 = 0x1133418u32; cpu.esi = v; }
  { let v: u32 = 0x113368cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x103ea09); f_0103903f(cpu); if cpu.returned_to != 0x103ea09 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4c0eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ea16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ea16); }
 let mut pc: u32 = 0x103ea16;
 loop { match pc {
 0x103ea16 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4bf6u32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  cpu.push(0x103ea2f); f_0103903f(cpu); if cpu.returned_to != 0x103ea2f { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4bf6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ea3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ea3c); }
 let mut pc: u32 = 0x103ea3c;
 loop { match pc {
 0x103ea3c => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b4c0au32, v); }
  { let v: u32 = 0x10ef3d4u32; cpu.esi = v; }
  { let v: u32 = 0x138u32; cpu.ecx = v; }
  cpu.push(0x103ea55); f_0103903f(cpu); if cpu.returned_to != 0x103ea55 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11b4c0au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ea62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ea62); }
 let mut pc: u32 = 0x103ea62;
 loop { match pc {
 0x103ea62 => {
  { let v: u32 = 0x11252b0u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x11252b7u32, v); }
  { let v: u32 = 0x10f7fd8u32; cpu.ebx = v; }
  { let v: u32 = 0x11252b0u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103ea8d); f_01033bea(cpu); if cpu.returned_to != 0x103ea8d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252b7u32)); cpu.w8(0x11252b7u32, v); }
  { let v: u32 = 0x10f84d8u32; cpu.ebx = v; }
  { let v: u32 = 0x11252b0u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eaae); f_01033bea(cpu); if cpu.returned_to != 0x103eaae { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252b7u32)); cpu.w8(0x11252b7u32, v); }
  { let v: u32 = 0x10f89d8u32; cpu.ebx = v; }
  { let v: u32 = 0x11252b0u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eacf); f_01033bea(cpu); if cpu.returned_to != 0x103eacf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252b7u32)); cpu.w8(0x11252b7u32, v); }
  { let v: u32 = 0x10f8ed8u32; cpu.ebx = v; }
  { let v: u32 = 0x11252b0u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eaf0); f_01033bea(cpu); if cpu.returned_to != 0x103eaf0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x11252bdu32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x11252c4u32, v); }
  { let v: u32 = 0x10f93d8u32; cpu.ebx = v; }
  { let v: u32 = 0x11252bdu32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eb17); f_01033bea(cpu); if cpu.returned_to != 0x103eb17 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252c4u32)); cpu.w8(0x11252c4u32, v); }
  { let v: u32 = 0x10f98d8u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x11252bdu32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eb38); f_01033bea(cpu); if cpu.returned_to != 0x103eb38 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252c4u32)); cpu.w8(0x11252c4u32, v); }
  { let v: u32 = 0x10f9dd8u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x11252bdu32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eb59); f_01033bea(cpu); if cpu.returned_to != 0x103eb59 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11252c4u32)); cpu.w8(0x11252c4u32, v); }
  { let v: u32 = 0x10fa2d8u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x11252bdu32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eb7a); f_01033bea(cpu); if cpu.returned_to != 0x103eb7a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103eb7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103eb7c); }
 let mut pc: u32 = 0x103eb7c;
 loop { match pc {
 0x103eb7c => {
  { let v: u32 = 0x10f7d69u32; cpu.edx = v; }
  { let v: u32 = 0x1133418u32; cpu.ebx = v; }
  { let v: u32 = 0x113368cu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eb98); f_01033bea(cpu); if cpu.returned_to != 0x103eb98 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103eb9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103eb9a); }
 let mut pc: u32 = 0x103eb9a;
 loop { match pc {
 0x103eb9a => {
  cpu.push(0x103eb9f); f_0103e749(cpu); if cpu.returned_to != 0x103eb9f { return; }
  { let v: u32 = 0x10f7d76u32; cpu.edx = v; }
  { let v: u32 = 0x116c0d8u32; cpu.ebx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  cpu.push(0x103ebb3); f_0100144a(cpu); if cpu.returned_to != 0x103ebb3 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x103ebbe); f_01033bea(cpu); if cpu.returned_to != 0x103ebbe { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ebc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ebc0); }
 let mut pc: u32 = 0x103ebc0;
 loop { match pc {
 0x103ebc0 => {
  { let v: u32 = 0x10f7d8fu32; cpu.edx = v; }
  { let v: u32 = 0x10ef3d4u32; cpu.ebx = v; }
  { let v: u32 = 0x138u32; cpu.ecx = v; }
  cpu.push(0x103ebd4); f_0100144a(cpu); if cpu.returned_to != 0x103ebd4 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x103ebdf); f_01033bea(cpu); if cpu.returned_to != 0x103ebdf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ebe1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ebe1); }
 let mut pc: u32 = 0x103ebe1;
 loop { match pc {
 0x103ebe1 => {
  { let v: u32 = 0x10f7d5cu32; cpu.edx = v; }
  { let v: u32 = 0x10c5364u32; cpu.ebx = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  cpu.push(0x103ebf5); f_0100144a(cpu); if cpu.returned_to != 0x103ebf5 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x103ec00); f_01033bea(cpu); if cpu.returned_to != 0x103ec00 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ec02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ec02); }
 let mut pc: u32 = 0x103ec02;
 loop { match pc {
 0x103ec02 => {
  { let v: u32 = 0x10f7d2au32; cpu.edx = v; }
  { let v: u32 = 0x10c2394u32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103ec17); f_01033bea(cpu); if cpu.returned_to != 0x103ec17 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x103ec1d); f_0103ec28(cpu); if cpu.returned_to != 0x103ec1d { return; }
  cpu.push(0x103ec22); f_010369bd(cpu); if cpu.returned_to != 0x103ec22 { return; }
  cpu.push(0x103ec27); f_010840b4(cpu); if cpu.returned_to != 0x103ec27 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ec28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ec28); }
 let mut pc: u32 = 0x103ec28;
 loop { match pc {
 0x103ec28 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2394u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = 0x10f7c14u32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  cpu.push(0x103ec4d); f_0103eccb(cpu); if cpu.returned_to != 0x103ec4d { return; }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v: u32 = 0x10f7c2cu32; cpu.edi = v; }
  cpu.push(0x103ec5c); f_0103eccb(cpu); if cpu.returned_to != 0x103ec5c { return; }
  { let v: u32 = 0x2eu32; cpu.eax = v; }
  { let v: u32 = 0x10f7d35u32; cpu.edi = v; }
  cpu.push(0x103ec6b); f_0103eccb(cpu); if cpu.returned_to != 0x103ec6b { return; }
  { let v: u32 = 0x3bu32; cpu.eax = v; }
  { let v: u32 = 0x10f7d4fu32; cpu.edi = v; }
  cpu.push(0x103ec7a); f_0103eccb(cpu); if cpu.returned_to != 0x103ec7a { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x10f7d42u32; cpu.edi = v; }
  cpu.push(0x103ec89); f_0103eccb(cpu); if cpu.returned_to != 0x103ec89 { return; }
  { let v: u32 = 0x61u32; cpu.eax = v; }
  { let v: u32 = 0x10f7d5cu32; cpu.edi = v; }
  cpu.push(0x103ec98); f_0103eccb(cpu); if cpu.returned_to != 0x103ec98 { return; }
  { let v: u32 = 0x6fu32; cpu.eax = v; }
  { let v: u32 = 0x10f7d69u32; cpu.edi = v; }
  cpu.push(0x103eca7); f_0103eccb(cpu); if cpu.returned_to != 0x103eca7 { return; }
  { let v: u32 = 0x84u32; cpu.eax = v; }
  { let v: u32 = 0x10f7d76u32; cpu.edi = v; }
  cpu.push(0x103ecb6); f_0103eccb(cpu); if cpu.returned_to != 0x103ecb6 { return; }
  { let v: u32 = 0x91u32; cpu.eax = v; }
  { let v: u32 = 0x10f7d8fu32; cpu.edi = v; }
  cpu.push(0x103ecc5); f_0103eccb(cpu); if cpu.returned_to != 0x103ecc5 { return; }
  cpu.push(0x103ecca); f_0103ecd6(cpu); if cpu.returned_to != 0x103ecca { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103eccb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103eccb); }
 let mut pc: u32 = 0x103eccb;
 loop { match pc {
 0x103eccb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ecd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ecd6); }
 let mut pc: u32 = 0x103ecd6;
 loop { match pc {
 0x103ecd6 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2402u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11252cau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0x11c59d0u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ed01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ed01); }
 let mut pc: u32 = 0x103ed01;
 loop { match pc {
 0x103ed01 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1194709u32, v); }
  { let v: u32 = 0x119477cu32; cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6271u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6275u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6289u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c628du32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10fcd31u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11cc283u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3dafu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee968u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef36au32, v); }
  cpu.push(0x103ed97); f_0103d12a(cpu); if cpu.returned_to != 0x103ed97 { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4030u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee928u32, v); }
  { let v: u16 = 0x1au16; cpu.w16(0x10e3e37u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef305u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3e3bu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3e41u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee933u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11b517bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11b5177u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b25u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b2bu32, v); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d57u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d5bu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d55u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d59u32, v); }
  { let v: u32 = 0x32u32; cpu.w32(0x1105e60u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ee34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ee34); }
 let mut pc: u32 = 0x103ee34;
 loop { match pc {
 0x103ee34 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10fcd31u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b5447u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11c5afau32, v); }
  { let v: u32 = 0x119470au32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1194705u32, v); }
  { cpu.sub8(cpu.r8(0x1194709u32), 0x1u8); }
  if !cpu.zf { pc = 0x103ee7f; continue; }
  pc = 0x103ee68; continue;
 }
 0x103ee68 => {
  { let v: u32 = 0x824u32; cpu.w32(0x10fcd31u32, v); }
  cpu.push(0x103ee77); f_0103eede(cpu); if cpu.returned_to != 0x103ee77 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x1194705u32, v); }
  pc = 0x103eec7; continue;
 }
 0x103ee7f => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if !cpu.zf { pc = 0x103eec7; continue; }
  pc = 0x103ee88; continue;
 }
 0x103ee88 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d935u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d93fu32, v); }
  { let v: u32 = 0x110d931u32; cpu.edx = v; }
  { let v: u32 = 0x11a0f80u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x119470au32); cpu.ecx = v; }
  { let v: u32 = 0x119470au32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x103eeb8); f_01033bea(cpu); if cpu.returned_to != 0x103eeb8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x103eebe); f_0103ef0b(cpu); if cpu.returned_to != 0x103eebe { return; }
  if !cpu.cf { pc = 0x103eec7; continue; }
  pc = 0x103eec0; continue;
 }
 0x103eec0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1194709u32, v); }
  pc = 0x103eec7; continue;
 }
 0x103eec7 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if !cpu.zf { pc = 0x103eed7; continue; }
  pc = 0x103eed0; continue;
 }
 0x103eed0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3f39u32, v); }
  pc = 0x103eed7; continue;
 }
 0x103eed7 => {
  { let v: u32 = cpu.esi; cpu.w32(0x1194705u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103eede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103eede); }
 let mut pc: u32 = 0x103eede;
 loop { match pc {
 0x103eede => {
  { let v: u32 = 0x1003dc4u32; cpu.esi = v; }
  { let v: u32 = 0x1194762u32; cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194779u32, v); }
  { let v: u16 = cpu.r16(0x10e3cdfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x119477au32, v); }
  { let v: u32 = 0x119477cu32; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ef0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ef0b); }
 let mut pc: u32 = 0x103ef0b;
 loop { match pc {
 0x103ef0b => {
  { let v: u32 = 0x1003dc4u32; cpu.esi = v; }
  { let v: u32 = 0x1194762u32; cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  pc = 0x103ef1a; continue;
 }
 0x103ef1a => {
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103ef1a; continue; }
  pc = 0x103ef20; continue;
 }
 0x103ef20 => {
  { let v: u16 = cpu.r16(0x119477au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cdfu32, v); }
  { cpu.cf = false; }
  { let v: u32 = 0x119477cu32; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ef35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ef35); }
 let mut pc: u32 = 0x103ef35;
 loop { match pc {
 0x103ef35 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x103efb4; continue; }
  pc = 0x103ef49; continue;
 }
 0x103ef49 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10f639au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f63b7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f63b8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f63bbu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f63b9u32, v); }
  { let v: u8 = 0xf6u8; cpu.w8(0x10f5812u32, v); }
  { let v: u8 = 0x8u8; cpu.w8(0x10f5879u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10f7435u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10f7452u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10f7453u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x10f7456u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f66bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f66deu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f6813u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f6832u32, v); }
  pc = 0x103f01d; continue;
 }
 0x103efb4 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10f639au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f63b7u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f63b8u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f63bbu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f63b9u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10f5879u32, v); }
  { let v: u8 = 0xf6u8; cpu.w8(0x10f5812u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f7435u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f7452u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f7453u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10f7456u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f66bfu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f66deu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6813u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6832u32, v); }
  pc = 0x103f01d; continue;
 }
 0x103f01d => {
  cpu.push(0x103f022); f_0103f023(cpu); if cpu.returned_to != 0x103f022 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f023); }
 let mut pc: u32 = 0x103f023;
 loop { match pc {
 0x103f023 => {
  { let v: u32 = 0x10f6af4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6af4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6b37u32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6b37u32; cpu.edi = v; }
  { let v: u32 = 0x10f6b7au32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6b7au32; cpu.edi = v; }
  { let v: u32 = 0x10f6bbdu32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6bbdu32; cpu.edi = v; }
  { let v: u32 = 0x10f5e89u32; cpu.edi = v; }
  { let v: u32 = 0x10f5e46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5e89u32; cpu.edi = v; }
  { let v: u32 = 0x10f5eccu32; cpu.edi = v; }
  { let v: u32 = 0x10f5e46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5eccu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f0fu32; cpu.edi = v; }
  { let v: u32 = 0x10f5e46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5f0fu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f52u32; cpu.edi = v; }
  { let v: u32 = 0x10f5e46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5f52u32; cpu.edi = v; }
  { let v: u32 = 0x10f63d4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6391u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f63d4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6417u32; cpu.edi = v; }
  { let v: u32 = 0x10f6391u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6417u32; cpu.edi = v; }
  { let v: u32 = 0x10f645au32; cpu.edi = v; }
  { let v: u32 = 0x10f6391u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f645au32; cpu.edi = v; }
  { let v: u32 = 0x10f649du32; cpu.edi = v; }
  { let v: u32 = 0x10f6391u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f649du32; cpu.edi = v; }
  { let v: u32 = 0x10f6523u32; cpu.edi = v; }
  { let v: u32 = 0x10f64e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6523u32; cpu.edi = v; }
  { let v: u32 = 0x10f6566u32; cpu.edi = v; }
  { let v: u32 = 0x10f64e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6566u32; cpu.edi = v; }
  { let v: u32 = 0x10f65a9u32; cpu.edi = v; }
  { let v: u32 = 0x10f64e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f65a9u32; cpu.edi = v; }
  { let v: u32 = 0x10f65ecu32; cpu.edi = v; }
  { let v: u32 = 0x10f64e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f65ecu32; cpu.edi = v; }
  { let v: u32 = 0x10f6c43u32; cpu.edi = v; }
  { let v: u32 = 0x10f6c00u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6c43u32; cpu.edi = v; }
  { let v: u32 = 0x10f6c86u32; cpu.edi = v; }
  { let v: u32 = 0x10f6c00u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6c86u32; cpu.edi = v; }
  { let v: u32 = 0x10f6cc9u32; cpu.edi = v; }
  { let v: u32 = 0x10f6c00u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6cc9u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d0cu32; cpu.edi = v; }
  { let v: u32 = 0x10f6c00u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6d0cu32; cpu.edi = v; }
  { let v: u32 = 0x10f6af4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6af4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6b37u32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6b37u32; cpu.edi = v; }
  { let v: u32 = 0x10f6b7au32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6b7au32; cpu.edi = v; }
  { let v: u32 = 0x10f6bbdu32; cpu.edi = v; }
  { let v: u32 = 0x10f6ab1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6bbdu32; cpu.edi = v; }
  { let v: u32 = 0x10f6ff8u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x103f249); f_0103f67d(cpu); if cpu.returned_to != 0x103f249 { return; }
  { let v: u32 = 0x10f703cu32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f703cu32; cpu.edi = v; }
  cpu.push(0x103f264); f_0103f67d(cpu); if cpu.returned_to != 0x103f264 { return; }
  { let v: u32 = 0x10f7080u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f7080u32; cpu.edi = v; }
  cpu.push(0x103f27f); f_0103f687(cpu); if cpu.returned_to != 0x103f27f { return; }
  cpu.push(0x103f284); f_0103f67d(cpu); if cpu.returned_to != 0x103f284 { return; }
  { let v: u32 = 0x10f70c4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f70c4u32; cpu.edi = v; }
  cpu.push(0x103f29f); f_0103f67d(cpu); if cpu.returned_to != 0x103f29f { return; }
  { let v: u32 = 0x10f7108u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f7108u32; cpu.edi = v; }
  cpu.push(0x103f2ba); f_0103f691(cpu); if cpu.returned_to != 0x103f2ba { return; }
  cpu.push(0x103f2bf); f_0103f67d(cpu); if cpu.returned_to != 0x103f2bf { return; }
  { let v: u32 = 0x10f6ea4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6ee8u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6ee8u32; cpu.edi = v; }
  cpu.push(0x103f2eb); f_0103f682(cpu); if cpu.returned_to != 0x103f2eb { return; }
  { let v: u32 = 0x10f6f2cu32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6f70u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6f70u32; cpu.edi = v; }
  cpu.push(0x103f317); f_0103f68c(cpu); if cpu.returned_to != 0x103f317 { return; }
  { let v: u32 = 0x10f6fb4u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6d94u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6d94u32; cpu.edi = v; }
  cpu.push(0x103f343); f_0103f682(cpu); if cpu.returned_to != 0x103f343 { return; }
  { let v: u32 = 0x10f6dd8u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6dd8u32; cpu.edi = v; }
  cpu.push(0x103f35e); f_0103f687(cpu); if cpu.returned_to != 0x103f35e { return; }
  { let v: u32 = 0x10f6e1cu32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6e1cu32; cpu.edi = v; }
  cpu.push(0x103f379); f_0103f68c(cpu); if cpu.returned_to != 0x103f379 { return; }
  { let v: u32 = 0x10f6e60u32; cpu.edi = v; }
  { let v: u32 = 0x10f6d50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6e60u32; cpu.edi = v; }
  cpu.push(0x103f394); f_0103f691(cpu); if cpu.returned_to != 0x103f394 { return; }
  { let v: u32 = 0x10f695eu32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x103f3aa); f_0103f67d(cpu); if cpu.returned_to != 0x103f3aa { return; }
  { let v: u32 = 0x10f69a2u32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f69a2u32; cpu.edi = v; }
  cpu.push(0x103f3c5); f_0103f67d(cpu); if cpu.returned_to != 0x103f3c5 { return; }
  { let v: u32 = 0x10f69e6u32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f69e6u32; cpu.edi = v; }
  cpu.push(0x103f3e0); f_0103f687(cpu); if cpu.returned_to != 0x103f3e0 { return; }
  cpu.push(0x103f3e5); f_0103f67d(cpu); if cpu.returned_to != 0x103f3e5 { return; }
  { let v: u32 = 0x10f6a2au32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6a2au32; cpu.edi = v; }
  cpu.push(0x103f400); f_0103f67d(cpu); if cpu.returned_to != 0x103f400 { return; }
  { let v: u32 = 0x10f6a6eu32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6a6eu32; cpu.edi = v; }
  cpu.push(0x103f41b); f_0103f691(cpu); if cpu.returned_to != 0x103f41b { return; }
  cpu.push(0x103f420); f_0103f67d(cpu); if cpu.returned_to != 0x103f420 { return; }
  { let v: u32 = 0x10f66fau32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f66fau32; cpu.edi = v; }
  cpu.push(0x103f43b); f_0103f682(cpu); if cpu.returned_to != 0x103f43b { return; }
  cpu.push(0x103f440); f_0103f67d(cpu); if cpu.returned_to != 0x103f440 { return; }
  { let v: u32 = 0x10f673eu32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f673eu32; cpu.edi = v; }
  cpu.push(0x103f45b); f_0103f687(cpu); if cpu.returned_to != 0x103f45b { return; }
  cpu.push(0x103f460); f_0103f67d(cpu); if cpu.returned_to != 0x103f460 { return; }
  { let v: u32 = 0x10f6782u32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6782u32; cpu.edi = v; }
  cpu.push(0x103f47b); f_0103f68c(cpu); if cpu.returned_to != 0x103f47b { return; }
  cpu.push(0x103f480); f_0103f67d(cpu); if cpu.returned_to != 0x103f480 { return; }
  { let v: u32 = 0x10f67c6u32; cpu.edi = v; }
  { let v: u32 = 0x10f66b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f67c6u32; cpu.edi = v; }
  cpu.push(0x103f49b); f_0103f691(cpu); if cpu.returned_to != 0x103f49b { return; }
  cpu.push(0x103f4a0); f_0103f67d(cpu); if cpu.returned_to != 0x103f4a0 { return; }
  { let v: u32 = 0x10f684eu32; cpu.edi = v; }
  { let v: u32 = 0x10f680au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f684eu32; cpu.edi = v; }
  cpu.push(0x103f4bb); f_0103f682(cpu); if cpu.returned_to != 0x103f4bb { return; }
  cpu.push(0x103f4c0); f_0103f67d(cpu); if cpu.returned_to != 0x103f4c0 { return; }
  { let v: u32 = 0x10f6892u32; cpu.edi = v; }
  { let v: u32 = 0x10f680au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6892u32; cpu.edi = v; }
  cpu.push(0x103f4db); f_0103f67d(cpu); if cpu.returned_to != 0x103f4db { return; }
  { let v: u32 = 0x10f68d6u32; cpu.edi = v; }
  { let v: u32 = 0x10f680au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f68d6u32; cpu.edi = v; }
  cpu.push(0x103f4f6); f_0103f68c(cpu); if cpu.returned_to != 0x103f4f6 { return; }
  cpu.push(0x103f4fb); f_0103f67d(cpu); if cpu.returned_to != 0x103f4fb { return; }
  { let v: u32 = 0x10f691au32; cpu.edi = v; }
  { let v: u32 = 0x10f680au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f691au32; cpu.edi = v; }
  cpu.push(0x103f516); f_0103f67d(cpu); if cpu.returned_to != 0x103f516 { return; }
  { let v: u32 = 0x10f623eu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f623eu32; cpu.edi = v; }
  cpu.push(0x103f531); f_0103f67d(cpu); if cpu.returned_to != 0x103f531 { return; }
  { let v: u32 = 0x10f6282u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6282u32; cpu.edi = v; }
  cpu.push(0x103f54c); f_0103f67d(cpu); if cpu.returned_to != 0x103f54c { return; }
  { let v: u32 = 0x10f62c6u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f62c6u32; cpu.edi = v; }
  cpu.push(0x103f567); f_0103f687(cpu); if cpu.returned_to != 0x103f567 { return; }
  cpu.push(0x103f56c); f_0103f67d(cpu); if cpu.returned_to != 0x103f56c { return; }
  { let v: u32 = 0x10f630au32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f630au32; cpu.edi = v; }
  cpu.push(0x103f587); f_0103f67d(cpu); if cpu.returned_to != 0x103f587 { return; }
  { let v: u32 = 0x10f634eu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f634eu32; cpu.edi = v; }
  cpu.push(0x103f5a2); f_0103f691(cpu); if cpu.returned_to != 0x103f5a2 { return; }
  cpu.push(0x103f5a7); f_0103f67d(cpu); if cpu.returned_to != 0x103f5a7 { return; }
  { let v: u32 = 0x10f60eau32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f612eu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f612eu32; cpu.edi = v; }
  cpu.push(0x103f5d3); f_0103f682(cpu); if cpu.returned_to != 0x103f5d3 { return; }
  { let v: u32 = 0x10f6172u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f61b6u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f61b6u32; cpu.edi = v; }
  cpu.push(0x103f5ff); f_0103f68c(cpu); if cpu.returned_to != 0x103f5ff { return; }
  { let v: u32 = 0x10f61fau32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5fdau32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f5fdau32; cpu.edi = v; }
  cpu.push(0x103f62b); f_0103f682(cpu); if cpu.returned_to != 0x103f62b { return; }
  { let v: u32 = 0x10f601eu32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f601eu32; cpu.edi = v; }
  cpu.push(0x103f646); f_0103f687(cpu); if cpu.returned_to != 0x103f646 { return; }
  { let v: u32 = 0x10f6062u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f6062u32; cpu.edi = v; }
  cpu.push(0x103f661); f_0103f68c(cpu); if cpu.returned_to != 0x103f661 { return; }
  { let v: u32 = 0x10f60a6u32; cpu.edi = v; }
  { let v: u32 = 0x10f5f96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f60a6u32; cpu.edi = v; }
  cpu.push(0x103f67c); f_0103f691(cpu); if cpu.returned_to != 0x103f67c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
