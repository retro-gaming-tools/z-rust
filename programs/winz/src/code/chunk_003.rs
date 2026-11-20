#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004200a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4200a5); }
 let mut pc: u32 = 0x4200a5;
 loop { match pc {
 0x4200a5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x593009u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x592ff0u32, v); }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if cpu.zf { pc = 0x4200d7; continue; }
  pc = 0x4200be; continue;
 }
 0x4200be => {
  { cpu.sub32(cpu.ecx, 0x13a3u32); }
  if !cpu.zf { pc = 0x4200d7; continue; }
  pc = 0x4200c6; continue;
 }
 0x4200c6 => {
  { let v: u32 = 0x1bfcu32; cpu.w32(0x592ff0u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x581b67u32, v); }
  pc = 0x4200d7; continue;
 }
 0x4200d7 => {
  { let v: u32 = cpu.edx; cpu.w32(0x592ff4u32, v); }
  { let v: u32 = cpu.r32(0x592ff4u32).wrapping_add(cpu.eax); cpu.w32(0x592ff4u32, v); }
  pc = 0x4200e3; continue;
 }
 0x4200e3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x42013b; continue; }
  pc = 0x4200ec; continue;
 }
 0x4200ec => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4200f4; continue; }
  pc = 0x4200f1; continue;
 }
 0x4200f1 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4200f4; continue;
 }
 0x4200f4 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x592ff0u32)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x420106); cpu.call(t); if cpu.returned_to != 0x420106 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x593009u32)); cpu.w16(0x593009u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x59307bu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x2u8); }
  if !cpu.zf { pc = 0x420126; continue; }
  pc = 0x420124; continue;
 }
 0x420124 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x420126; continue;
 }
 0x420126 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x593005u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x593007u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x592ff4u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4200e3; continue; }
  pc = 0x42013b; continue;
 }
 0x42013b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042013c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42013c); }
 let mut pc: u32 = 0x42013c;
 loop { match pc {
 0x42013c => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x4201b2; continue; }
  pc = 0x42014a; continue;
 }
 0x42014a => {
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x4201ac; continue; }
  pc = 0x42016b; continue;
 }
 0x42016b => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x420174); f_0044c7f9(cpu); if cpu.returned_to != 0x420174 { return; }
  if cpu.cf { pc = 0x4201ac; continue; }
  pc = 0x420176; continue;
 }
 0x420176 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1u32); cpu.esi = v; }
  pc = 0x42017e; continue;
 }
 0x42017e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4201a7; continue; }
  pc = 0x420183; continue;
 }
 0x420183 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x4201a5); f_0042024a(cpu); if cpu.returned_to != 0x4201a5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4201a7; continue;
 }
 0x4201a7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42017e; continue; }
  pc = 0x4201ab; continue;
 }
 0x4201ab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4201ac => {
  cpu.push(0x4201b1); f_0042024a(cpu); if cpu.returned_to != 0x4201b1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4201b2 => {
  { cpu.sub16(cpu.r16(0x4d6888u32), 0x0u16); }
  if cpu.zf { pc = 0x420249; continue; }
  pc = 0x4201c0; continue;
 }
 0x4201c0 => {
  { cpu.sub8(cpu.r8(0x4e156au32), 0x4u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x420249; continue; }
  pc = 0x4201cd; continue;
 }
 0x4201cd => {
  cpu.push(0x4201d2); f_00447617(cpu); if cpu.returned_to != 0x4201d2 { return; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6888u32)); cpu.w16(0x4d6888u32, v); }
  { let v = cpu.r8(0x4d6888u32); cpu.mul8(v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6888u32)); cpu.w16(0x4d6888u32, v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x4b839cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x42022a; continue; }
  pc = 0x420226; continue;
 }
 0x420226 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x42022a; continue;
 }
 0x42022a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420231); f_00420301(cpu); if cpu.returned_to != 0x420231 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2ceu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x420249); cpu.call(t); if cpu.returned_to != 0x420249 { return; }
  pc = 0x420249; continue;
 }
 0x420249 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042024a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42024a); }
 let mut pc: u32 = 0x42024a;
 loop { match pc {
 0x42024a => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42026c); f_00420301(cpu); if cpu.returned_to != 0x42026c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x18u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f68u32); cpu.push(0x42028c); cpu.call(t); if cpu.returned_to != 0x42028c { return; }
  let t = cpu.r32(0x4d6f6cu32); cpu.push(0x420292); cpu.call(t); if cpu.returned_to != 0x420292 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f68u32); cpu.push(0x4202a4); cpu.call(t); if cpu.returned_to != 0x4202a4 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f6cu32); cpu.push(0x4202b3); cpu.call(t); if cpu.returned_to != 0x4202b3 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x12u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f68u32); cpu.push(0x4202c7); cpu.call(t); if cpu.returned_to != 0x4202c7 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f6cu32); cpu.push(0x4202d6); cpu.call(t); if cpu.returned_to != 0x4202d6 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f6cu32); cpu.push(0x4202e3); cpu.call(t); if cpu.returned_to != 0x4202e3 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f68u32); cpu.push(0x4202f6); cpu.call(t); if cpu.returned_to != 0x4202f6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx)); cpu.push(0x4202fd); cpu.call(t); if cpu.returned_to != 0x4202fd { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420301(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420301); }
 let mut pc: u32 = 0x420301;
 loop { match pc {
 0x420301 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x420329; continue; }
  pc = 0x420310; continue;
 }
 0x420310 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0x5ba147u32; cpu.w32(0x4d6af0u32, v); }
  cpu.push(0x420323); f_0046d7be(cpu); if cpu.returned_to != 0x420323 { return; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub((cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x420329; continue;
 }
 0x420329 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004204d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4204d6); }
 let mut pc: u32 = 0x4204d6;
 loop { match pc {
 0x4204d6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c6cfu32, v); }
  cpu.push(0x4204e2); f_004184ca(cpu); if cpu.returned_to != 0x4204e2 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4205db; continue; }
  pc = 0x4204ed; continue;
 }
 0x4204ed => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6c6cu32, v); }
  { let v: u32 = 0x5b9d10u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x2u32); }
  if cpu.zf { pc = 0x420536; continue; }
  pc = 0x420520; continue;
 }
 0x420520 => {
  { cpu.sub8(cpu.r8(0x58c440u32), 0x0u8); }
  if cpu.zf { pc = 0x420536; continue; }
  pc = 0x420529; continue;
 }
 0x420529 => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  pc = 0x420567; continue;
 }
 0x420536 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x420559; continue; }
  pc = 0x42054c; continue;
 }
 0x42054c => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x420559; continue;
 }
 0x420559 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x420567; continue;
 }
 0x420567 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x581c00u32, v); }
  cpu.push(0x420584); f_00476a47(cpu); if cpu.returned_to != 0x420584 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x581c00u32, v); }
  { let v: u32 = 0x4d6c88u32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x5b9d10u32; cpu.esi = v; }
  pc = 0x4205b0; continue;
 }
 0x4205b0 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x4205ce; continue; }
  pc = 0x4205b4; continue;
 }
 0x4205b4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4205ce; continue; }
  pc = 0x4205cb; continue;
 }
 0x4205cb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4205b0; continue; }
  pc = 0x4205ce; continue;
 }
 0x4205ce => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x4205db; continue;
 }
 0x4205db => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c6cfu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420978(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420978); }
 let mut pc: u32 = 0x420978;
 loop { match pc {
 0x420978 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420989); }
 let mut pc: u32 = 0x420989;
 loop { match pc {
 0x420989 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u32 = 0x4d954cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004209a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4209a1); }
 let mut pc: u32 = 0x4209a1;
 loop { match pc {
 0x4209a1 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4abefcu32; cpu.esi = v; }
  { let v: u32 = 0x5930d6u32; cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x4209b3; continue;
 }
 0x4209b3 => {
  { cpu.movs(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4209b3; continue; }
  pc = 0x4209b6; continue;
 }
 0x4209b6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004209ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4209ba); }
 let mut pc: u32 = 0x4209ba;
 loop { match pc {
 0x4209ba => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x5930d6u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x4209cc; continue;
 }
 0x4209cc => {
  { cpu.movs(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4209cc; continue; }
  pc = 0x4209cf; continue;
 }
 0x4209cf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004209d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4209d3); }
 let mut pc: u32 = 0x4209d3;
 loop { match pc {
 0x4209d3 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5932d6u32, v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x50u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4209e9; continue;
 }
 0x4209e9 => {
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4209e9; continue; }
  pc = 0x4209ee; continue;
 }
 0x4209ee => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004209f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4209f6); }
 let mut pc: u32 = 0x4209f6;
 loop { match pc {
 0x4209f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5932d6u32, v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420a0c; continue;
 }
 0x420a0c => {
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420a0c; continue; }
  pc = 0x420a11; continue;
 }
 0x420a11 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420a19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420a19); }
 let mut pc: u32 = 0x420a19;
 loop { match pc {
 0x420a19 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5932d6u32, v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420a2f; continue;
 }
 0x420a2f => {
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420a2f; continue; }
  pc = 0x420a34; continue;
 }
 0x420a34 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420a3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420a3c); }
 let mut pc: u32 = 0x420a3c;
 loop { match pc {
 0x420a3c => {
  { let v: u32 = 0x2u32; cpu.w32(0x5932d6u32, v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420a52; continue;
 }
 0x420a52 => {
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420a52; continue; }
  pc = 0x420a57; continue;
 }
 0x420a57 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420a5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420a5f); }
 let mut pc: u32 = 0x420a5f;
 loop { match pc {
 0x420a5f => {
  { let v: u32 = 0x3u32; cpu.w32(0x5932d6u32, v); }
  { let v: u32 = 0x4abefcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420a75; continue;
 }
 0x420a75 => {
  { cpu.stos(1, false); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420a75; continue; }
  pc = 0x420a7a; continue;
 }
 0x420a7a => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420a82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420a82); }
 let mut pc: u32 = 0x420a82;
 loop { match pc {
 0x420a82 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420a8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420a8b); }
 let mut pc: u32 = 0x420a8b;
 loop { match pc {
 0x420a8b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.w32(0x4d6bd8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x55b9b8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592ffcu32, v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x592ffeu32, v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x592ffdu32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x592ffcu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3fu8); }
  if (cpu.cf || cpu.zf) { pc = 0x420ad1; continue; }
  pc = 0x420acf; continue;
 }
 0x420acf => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420ad1; continue;
 }
 0x420ad1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592fffu32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x592ffeu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3fu8); }
  if (cpu.cf || cpu.zf) { pc = 0x420ae4; continue; }
  pc = 0x420ae2; continue;
 }
 0x420ae2 => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420ae4; continue;
 }
 0x420ae4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x593000u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x592ffdu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3fu8); }
  if (cpu.cf || cpu.zf) { pc = 0x420af7; continue; }
  pc = 0x420af5; continue;
 }
 0x420af5 => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420af7; continue;
 }
 0x420af7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x593001u32, v); }
  { let v: u8 = cpu.r8(0x592ffcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x420b0a; continue; }
  pc = 0x420b08; continue;
 }
 0x420b08 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420b0a; continue;
 }
 0x420b0a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x593002u32, v); }
  { let v: u8 = cpu.r8(0x592ffeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x420b1d; continue; }
  pc = 0x420b1b; continue;
 }
 0x420b1b => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420b1d; continue;
 }
 0x420b1d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x593003u32, v); }
  { let v: u8 = cpu.r8(0x592ffdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x420b30; continue; }
  pc = 0x420b2e; continue;
 }
 0x420b2e => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420b30; continue;
 }
 0x420b30 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x593004u32, v); }
  { let v: u32 = 0x55b9bbu32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  pc = 0x420b45; continue;
 }
 0x420b45 => {
  { let v: u16 = 0x0u16; cpu.w16(0x592ff8u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x592ffcu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x592ff8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x592ff8u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x592fffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x420b6c; continue; }
  pc = 0x420b6a; continue;
 }
 0x420b6a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420b6c; continue;
 }
 0x420b6c => {
  { let v = cpu.r8(0x593002u32); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x592ffeu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x592ff8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x592ff8u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x593000u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x420b95; continue; }
  pc = 0x420b93; continue;
 }
 0x420b93 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420b95; continue;
 }
 0x420b95 => {
  { let v = cpu.r8(0x593003u32); cpu.mul8(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x592ffdu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x592ff8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x592ff8u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x593001u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x420bbe; continue; }
  pc = 0x420bbc; continue;
 }
 0x420bbc => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x420bbe; continue;
 }
 0x420bbe => {
  { let v = cpu.r8(0x593004u32); cpu.mul8(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.cf { pc = 0x420bd9; continue; }
  pc = 0x420bcb; continue;
 }
 0x420bcb => {
  { cpu.sub16(cpu.r16(0x592ff8u32), 0x0u16); }
  if cpu.sf { pc = 0x420bd9; continue; }
  pc = 0x420bd5; continue;
 }
 0x420bd5 => {
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x420bd9; continue;
 }
 0x420bd9 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x3u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x420b45; continue; }
  pc = 0x420be6; continue;
 }
 0x420be6 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
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
pub fn f_00420d18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420d18); }
 let mut pc: u32 = 0x420d18;
 loop { match pc {
 0x420d18 => {
  { let v: u32 = 0x4adf5cu32; cpu.edi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x420d27; continue;
 }
 0x420d27 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x420d37; continue;
 }
 0x420d37 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420d3d); f_00420a8b(cpu); if cpu.returned_to != 0x420d3d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420d37; continue; }
  pc = 0x420d46; continue;
 }
 0x420d46 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x420d27; continue; }
  pc = 0x420d4c; continue;
 }
 0x420d4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420d4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420d4d); }
 let mut pc: u32 = 0x420d4d;
 loop { match pc {
 0x420d4d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420d59); f_004417b9(cpu); if cpu.returned_to != 0x420d59 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4ae75cu32; cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  cpu.push(0x420d7d); f_0044179f(cpu); if cpu.returned_to != 0x420d7d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420d7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420d7f); }
 let mut pc: u32 = 0x420d7f;
 loop { match pc {
 0x420d7f => {
  cpu.push(0x420d84); f_00420d4d(cpu); if cpu.returned_to != 0x420d84 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420d85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420d85); }
 let mut pc: u32 = 0x420d85;
 loop { match pc {
 0x420d85 => {
  { let v: u32 = cpu.r32(0x5ba958u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x30u32); cpu.esi = v; }
  { let v: u32 = 0x5932dcu32; cpu.edi = v; }
  { let v: u8 = 0x26u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4040404u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1eu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2bu32); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xfffffffcu32; cpu.edi = v; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x420dc6; continue;
 }
 0x420dc6 => {
  { let v: u8 = 0x26u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x420dc9; continue;
 }
 0x420dc9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x420dc9; continue; }
  pc = 0x420e1a; continue;
 }
 0x420e1a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x420dc6; continue; }
  pc = 0x420e20; continue;
 }
 0x420e20 => {
  { let v: u32 = cpu.r32(0x58e95cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58e95cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68f2u32, v); }
  { let v: u32 = 0x5932dcu32; cpu.esi = v; }
  let t = cpu.r32(0x4d6facu32); cpu.push(0x420e42); cpu.call(t); if cpu.returned_to != 0x420e42 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x58e95cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420e4c); }
 let mut pc: u32 = 0x420e4c;
 loop { match pc {
 0x420e4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420e4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420e4d); }
 let mut pc: u32 = 0x420e4d;
 loop { match pc {
 0x420e4d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420e4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420e4e); }
 let mut pc: u32 = 0x420e4e;
 loop { match pc {
 0x420e4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420e4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420e4f); }
 let mut pc: u32 = 0x420e4f;
 loop { match pc {
 0x420e4f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420e50); }
 let mut pc: u32 = 0x420e50;
 loop { match pc {
 0x420e50 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420e62); f_00447634(cpu); if cpu.returned_to != 0x420e62 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420e6f); f_00406c00(cpu); if cpu.returned_to != 0x420e6f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420ec7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420ec7); }
 let mut pc: u32 = 0x420ec7;
 loop { match pc {
 0x420ec7 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420ed9); f_00447634(cpu); if cpu.returned_to != 0x420ed9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x420ee9; continue; }
  pc = 0x420ee5; continue;
 }
 0x420ee5 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x420eeb; continue;
 }
 0x420ee9 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x420eeb; continue;
 }
 0x420eeb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420ef3); f_00406ce0(cpu); if cpu.returned_to != 0x420ef3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420ef8); }
 let mut pc: u32 = 0x420ef8;
 loop { match pc {
 0x420ef8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5893bcu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5893beu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5893c0u32, v); }
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if cpu.cf { pc = 0x420f14; continue; }
  pc = 0x420f13; continue;
 }
 0x420f13 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x420f14 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420f26); f_00447634(cpu); if cpu.returned_to != 0x420f26 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x420f36; continue; }
  pc = 0x420f32; continue;
 }
 0x420f32 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x420f38; continue;
 }
 0x420f36 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x420f38; continue;
 }
 0x420f38 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420f40); f_00406ce0(cpu); if cpu.returned_to != 0x420f40 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420f7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420f7d); }
 let mut pc: u32 = 0x420f7d;
 loop { match pc {
 0x420f7d => {
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if cpu.cf { pc = 0x420f85; continue; }
  pc = 0x420f84; continue;
 }
 0x420f84 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x420f85 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420f97); f_00447634(cpu); if cpu.returned_to != 0x420f97 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x420fa7; continue; }
  pc = 0x420fa3; continue;
 }
 0x420fa3 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x420fa9; continue;
 }
 0x420fa7 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x420fa9; continue;
 }
 0x420fa9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420fb1); f_00406ce0(cpu); if cpu.returned_to != 0x420fb1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420fb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420fb6); }
 let mut pc: u32 = 0x420fb6;
 loop { match pc {
 0x420fb6 => {
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if cpu.cf { pc = 0x420fbe; continue; }
  pc = 0x420fbd; continue;
 }
 0x420fbd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x420fbe => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x420fd0); f_00447634(cpu); if cpu.returned_to != 0x420fd0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x420fe0; continue; }
  pc = 0x420fdc; continue;
 }
 0x420fdc => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x420fe2; continue;
 }
 0x420fe0 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x420fe2; continue;
 }
 0x420fe2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x420fea); f_00406dc0(cpu); if cpu.returned_to != 0x420fea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00420fef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x420fef); }
 let mut pc: u32 = 0x420fef;
 loop { match pc {
 0x420fef => {
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if cpu.cf { pc = 0x420ff7; continue; }
  pc = 0x420ff6; continue;
 }
 0x420ff6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x420ff7 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x421009); f_00447634(cpu); if cpu.returned_to != 0x421009 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x421019; continue; }
  pc = 0x421015; continue;
 }
 0x421015 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x42101b; continue;
 }
 0x421019 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x42101b; continue;
 }
 0x42101b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x421023); f_00406c00(cpu); if cpu.returned_to != 0x421023 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421028); }
 let mut pc: u32 = 0x421028;
 loop { match pc {
 0x421028 => {
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if cpu.cf { pc = 0x421030; continue; }
  pc = 0x42102f; continue;
 }
 0x42102f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421030 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x421042); f_00447634(cpu); if cpu.returned_to != 0x421042 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x421052; continue; }
  pc = 0x42104e; continue;
 }
 0x42104e => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x421054; continue;
 }
 0x421052 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x421054; continue;
 }
 0x421054 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42105c); f_00406e10(cpu); if cpu.returned_to != 0x42105c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421064); }
 let mut pc: u32 = 0x421064;
 loop { match pc {
 0x421064 => {
  { cpu.sub8((cpu.eax as u8), 0x13u8); }
  if cpu.zf { pc = 0x42106d; continue; }
  pc = 0x421068; continue;
 }
 0x421068 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x42106e; continue; }
  pc = 0x42106c; continue;
 }
 0x42106c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42106d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42106e => {
  { cpu.pushad(); }
  cpu.push(0x421074); f_00404c60(cpu); if cpu.returned_to != 0x421074 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421076(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421076); }
 let mut pc: u32 = 0x421076;
 loop { match pc {
 0x421076 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004210c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4210c8); }
 let mut pc: u32 = 0x4210c8;
 loop { match pc {
 0x4210c8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4210d5); f_00407500(cpu); if cpu.returned_to != 0x4210d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x421107; continue; }
  pc = 0x4210e1; continue;
 }
 0x4210e1 => {
  cpu.push(0x4210e6); f_0042110c(cpu); if cpu.returned_to != 0x4210e6 { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x593b74u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x593b74u32); cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421107 => {
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042110c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42110c); } r_0042110c(cpu, 0x42110c); }
pub fn f_0042110e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42110e); } r_0042110c(cpu, 0x42110e); }
fn r_0042110c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x42110c => {
  { let v: u32 = 0x593b78u32; cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x421116; continue;
 }
 0x42110e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x421116; continue;
 }
 0x421116 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x421121; continue; }
  pc = 0x42111b; continue;
 }
 0x42111b => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x421116; continue;
 }
 0x421121 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421124); }
 let mut pc: u32 = 0x421124;
 loop { match pc {
 0x421124 => {
  { let v: u32 = 0x593b78u32; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421137); }
 let mut pc: u32 = 0x421137;
 loop { match pc {
 0x421137 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  cpu.push(0x421148); f_004210c8(cpu); if cpu.returned_to != 0x421148 { return; }
  if cpu.cf { pc = 0x42116d; continue; }
  pc = 0x42114a; continue;
 }
 0x42114a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x593b6cu32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x593b6eu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42116d => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421170); }
 let mut pc: u32 = 0x421170;
 loop { match pc {
 0x421170 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x593b6cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0x593b6eu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x421193; continue; }
  pc = 0x421180; continue;
 }
 0x421180 => {
  { let v: u16 = cpu.r16(0x593b6cu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593b6eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.push(0x421193); f_0042119b(cpu); if cpu.returned_to != 0x421193 { return; }
  pc = 0x421193; continue;
 }
 0x421193 => {
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042119b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42119b); }
 let mut pc: u32 = 0x42119b;
 loop { match pc {
 0x42119b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4211a9); f_00421124(cpu); if cpu.returned_to != 0x4211a9 { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4211af); f_00407510(cpu); if cpu.returned_to != 0x4211af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004211b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4211b6); }
 let mut pc: u32 = 0x4211b6;
 loop { match pc {
 0x4211b6 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x4211b8; continue;
 }
 0x4211b8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x24u8); }
  if cpu.zf { pc = 0x4211c5; continue; }
  pc = 0x4211bd; continue;
 }
 0x4211bd => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x4211c5; continue; }
  pc = 0x4211c2; continue;
 }
 0x4211c2 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x4211b8; continue;
 }
 0x4211c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4211cf); f_004030b0(cpu); if cpu.returned_to != 0x4211cf { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004211d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4211d4); }
 let mut pc: u32 = 0x4211d4;
 loop { match pc {
 0x4211d4 => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4211dd); f_0040d600(cpu); if cpu.returned_to != 0x4211dd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004211ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4211ec); }
 let mut pc: u32 = 0x4211ec;
 loop { match pc {
 0x4211ec => {
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4211f3); f_0040d640(cpu); if cpu.returned_to != 0x4211f3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421214(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421214); }
 let mut pc: u32 = 0x421214;
 loop { match pc {
 0x421214 => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42121d); f_0040d620(cpu); if cpu.returned_to != 0x42121d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042122d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42122d); }
 let mut pc: u32 = 0x42122d;
 loop { match pc {
 0x42122d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042122e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42122e); }
 let mut pc: u32 = 0x42122e;
 loop { match pc {
 0x42122e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042122f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42122f); }
 let mut pc: u32 = 0x42122f;
 loop { match pc {
 0x42122f => {
  { cpu.pushad(); }
  cpu.push(0x421235); f_00402fb0(cpu); if cpu.returned_to != 0x421235 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421241(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421241); }
 let mut pc: u32 = 0x421241;
 loop { match pc {
 0x421241 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x421253); f_00402fc0(cpu); if cpu.returned_to != 0x421253 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042125a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42125a); }
 let mut pc: u32 = 0x42125a;
 loop { match pc {
 0x42125a => {
  { cpu.pushad(); }
  cpu.push(0x421260); f_00402dd0(cpu); if cpu.returned_to != 0x421260 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421264(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421264); }
 let mut pc: u32 = 0x421264;
 loop { match pc {
 0x421264 => {
  { cpu.pushad(); }
  cpu.push(0x42126a); f_00402d30(cpu); if cpu.returned_to != 0x42126a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421286); }
 let mut pc: u32 = 0x421286;
 loop { match pc {
 0x421286 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x421295; continue; }
  pc = 0x42128a; continue;
 }
 0x42128a => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x421294; continue; }
  pc = 0x42128e; continue;
 }
 0x42128e => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x421293; continue; }
  pc = 0x421292; continue;
 }
 0x421292 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421293 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421294 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421295 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x42129c); f_00402f30(cpu); if cpu.returned_to != 0x42129c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004212a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4212a1); }
 let mut pc: u32 = 0x4212a1;
 loop { match pc {
 0x4212a1 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4212ab); f_00402de0(cpu); if cpu.returned_to != 0x4212ab { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x593b70u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x593b70u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004212c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4212c5); }
 let mut pc: u32 = 0x4212c5;
 loop { match pc {
 0x4212c5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5e2448u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5e244cu32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4ab14cu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x4212dd); f_00421ad7(cpu); if cpu.returned_to != 0x4212dd { return; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4ab14cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5e2448u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004212eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4212eb); }
 let mut pc: u32 = 0x4212eb;
 loop { match pc {
 0x4212eb => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x4212f7; continue;
 }
 0x4212f7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x4212fd; continue;
 }
 0x4212fd => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x421303; continue;
 }
 0x421303 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x421309; continue;
 }
 0x421309 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x42130f; continue;
 }
 0x42130f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x421315; continue;
 }
 0x421315 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x42131d; continue; }
  pc = 0x42131b; continue;
 }
 0x42131b => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x42131d; continue;
 }
 0x42131d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421320); }
 let mut pc: u32 = 0x421320;
 loop { match pc {
 0x421320 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x421326); f_004212eb(cpu); if cpu.returned_to != 0x421326 { return; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  cpu.push(0x42132e); f_004212eb(cpu); if cpu.returned_to != 0x42132e { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421334); }
 let mut pc: u32 = 0x421334;
 loop { match pc {
 0x421334 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42133a); f_004212eb(cpu); if cpu.returned_to != 0x42133a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x421341); f_004212eb(cpu); if cpu.returned_to != 0x421341 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x42134b); f_004212eb(cpu); if cpu.returned_to != 0x42134b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x421352); f_004212eb(cpu); if cpu.returned_to != 0x421352 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421365(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421365); } r_00421365(cpu, 0x421365); }
pub fn f_00421378(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421378); } r_00421365(cpu, 0x421378); }
pub fn f_0042138b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42138b); } r_00421365(cpu, 0x42138b); }
pub fn f_0042139e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42139e); } r_00421365(cpu, 0x42139e); }
pub fn f_004213b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4213b1); } r_00421365(cpu, 0x4213b1); }
pub fn f_004213c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4213c4); } r_00421365(cpu, 0x4213c4); }
pub fn f_004213d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4213d7); } r_00421365(cpu, 0x4213d7); }
pub fn f_004213ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4213ea); } r_00421365(cpu, 0x4213ea); }
pub fn f_004213fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4213fd); } r_00421365(cpu, 0x4213fd); }
pub fn f_00421410(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421410); } r_00421365(cpu, 0x421410); }
pub fn f_00421423(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421423); } r_00421365(cpu, 0x421423); }
pub fn f_00421436(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421436); } r_00421365(cpu, 0x421436); }
pub fn f_00421449(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421449); } r_00421365(cpu, 0x421449); }
pub fn f_0042145c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42145c); } r_00421365(cpu, 0x42145c); }
pub fn f_0042146f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42146f); } r_00421365(cpu, 0x42146f); }
pub fn f_00421482(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421482); } r_00421365(cpu, 0x421482); }
pub fn f_004214e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4214e4); } r_00421365(cpu, 0x4214e4); }
pub fn f_004214f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4214f2); } r_00421365(cpu, 0x4214f2); }
pub fn f_00421500(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421500); } r_00421365(cpu, 0x421500); }
pub fn f_0042150e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42150e); } r_00421365(cpu, 0x42150e); }
pub fn f_0042151c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42151c); } r_00421365(cpu, 0x42151c); }
pub fn f_0042152a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42152a); } r_00421365(cpu, 0x42152a); }
pub fn f_00421538(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421538); } r_00421365(cpu, 0x421538); }
pub fn f_00421546(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421546); } r_00421365(cpu, 0x421546); }
pub fn f_00421554(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421554); } r_00421365(cpu, 0x421554); }
pub fn f_00421562(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421562); } r_00421365(cpu, 0x421562); }
pub fn f_00421570(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421570); } r_00421365(cpu, 0x421570); }
pub fn f_0042157e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42157e); } r_00421365(cpu, 0x42157e); }
pub fn f_0042158c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42158c); } r_00421365(cpu, 0x42158c); }
pub fn f_0042159a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42159a); } r_00421365(cpu, 0x42159a); }
pub fn f_004215a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4215a8); } r_00421365(cpu, 0x4215a8); }
pub fn f_004215b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4215b6); } r_00421365(cpu, 0x4215b6); }
pub fn f_00421671(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421671); } r_00421365(cpu, 0x421671); }
pub fn f_00421892(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421892); } r_00421365(cpu, 0x421892); }
pub fn f_004218a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218a2); } r_00421365(cpu, 0x4218a2); }
pub fn f_004218b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218b2); } r_00421365(cpu, 0x4218b2); }
pub fn f_004218c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218c2); } r_00421365(cpu, 0x4218c2); }
pub fn f_004218d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218d2); } r_00421365(cpu, 0x4218d2); }
pub fn f_004218e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218e2); } r_00421365(cpu, 0x4218e2); }
pub fn f_004218f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4218f2); } r_00421365(cpu, 0x4218f2); }
pub fn f_00421902(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421902); } r_00421365(cpu, 0x421902); }
pub fn f_00421912(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421912); } r_00421365(cpu, 0x421912); }
pub fn f_00421916(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421916); } r_00421365(cpu, 0x421916); }
pub fn f_00421922(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421922); } r_00421365(cpu, 0x421922); }
pub fn f_00421932(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421932); } r_00421365(cpu, 0x421932); }
pub fn f_00421942(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421942); } r_00421365(cpu, 0x421942); }
pub fn f_00421952(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421952); } r_00421365(cpu, 0x421952); }
pub fn f_00421962(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421962); } r_00421365(cpu, 0x421962); }
pub fn f_00421972(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421972); } r_00421365(cpu, 0x421972); }
pub fn f_00421982(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x421982); } r_00421365(cpu, 0x421982); }
fn r_00421365(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x42135b => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xfu32); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x593aacu32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x421365 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421378 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42138b => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42139e => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4213b1 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4213c4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4213d7 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4213ea => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4213fd => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421410 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421423 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421436 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421449 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42145c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42146f => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421482 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4214e4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4214f2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421500 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42150e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42151c => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42152a => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421538 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421546 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421554 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421562 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421570 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42157e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42158c => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42159a => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4215a8 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4215b6 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421671 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x421679; continue;
 }
 0x421679 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x421680; continue; }
  pc = 0x42167f; continue;
 }
 0x42167f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x421680; continue;
 }
 0x421680 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421679; continue; }
  pc = 0x421688; continue;
 }
 0x421688 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x4216e4; continue; }
  pc = 0x42168e; continue;
 }
 0x42168e => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x4216a5; continue; }
  pc = 0x421693; continue;
 }
 0x421693 => {
  cpu.push(0x421698); f_004212eb(cpu); if cpu.returned_to != 0x421698 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4216a5 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x4216d0; continue; }
  pc = 0x4216aa; continue;
 }
 0x4216aa => {
  cpu.push(0x4216af); f_00421320(cpu); if cpu.returned_to != 0x4216af { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4216b9; continue;
 }
 0x4216b9 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x4216c0; continue; }
  pc = 0x4216bf; continue;
 }
 0x4216bf => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x4216c0; continue;
 }
 0x4216c0 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4216b9; continue; }
  pc = 0x4216c8; continue;
 }
 0x4216c8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x42135b; continue;
 }
 0x4216d0 => {
  cpu.push(0x4216d5); f_00421320(cpu); if cpu.returned_to != 0x4216d5 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4216dd); f_00421320(cpu); if cpu.returned_to != 0x4216dd { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  pc = 0x4216e4; continue;
 }
 0x4216e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421892 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218a2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218b2 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218c2 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218d2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218e2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4218f2 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421902 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421912 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x421916; continue;
 }
 0x421916 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421922 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421932 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421942 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421952 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421962 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421972 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421982 => {
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
pub fn f_004219ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4219ed); }
 let mut pc: u32 = 0x4219ed;
 loop { match pc {
 0x4219ed => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x4219f7; continue; }
  pc = 0x4219f5; continue;
 }
 0x4219f5 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4219f7; continue;
 }
 0x4219f7 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x4219fd; continue; }
  pc = 0x4219fb; continue;
 }
 0x4219fb => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4219fd; continue;
 }
 0x4219fd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a0a; continue; }
  pc = 0x421a08; continue;
 }
 0x421a08 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x421a0a; continue;
 }
 0x421a0a => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a10; continue; }
  pc = 0x421a0e; continue;
 }
 0x421a0e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x421a10; continue;
 }
 0x421a10 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a1e; continue; }
  pc = 0x421a1c; continue;
 }
 0x421a1c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x421a1e; continue;
 }
 0x421a1e => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a24; continue; }
  pc = 0x421a22; continue;
 }
 0x421a22 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x421a24; continue;
 }
 0x421a24 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a32; continue; }
  pc = 0x421a30; continue;
 }
 0x421a30 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x421a32; continue;
 }
 0x421a32 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x421a38; continue; }
  pc = 0x421a36; continue;
 }
 0x421a36 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x421a38; continue;
 }
 0x421a38 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421a3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421a3d); }
 let mut pc: u32 = 0x421a3d;
 loop { match pc {
 0x421843 => {
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x593b2cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x421851); cpu.call(t); if cpu.returned_to != 0x421851 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x593b2cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x421865); cpu.call(t); if cpu.returned_to != 0x421865 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 28u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x593b2cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x421879); cpu.call(t); if cpu.returned_to != 0x421879 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x593b2cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x42188b); cpu.call(t); if cpu.returned_to != 0x42188b { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x3c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421a3d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x421a45; continue;
 }
 0x421a45 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x421a4c; continue; }
  pc = 0x421a4b; continue;
 }
 0x421a4b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x421a4c; continue;
 }
 0x421a4c => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421a45; continue; }
  pc = 0x421a54; continue;
 }
 0x421a54 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x421ad6; continue; }
  pc = 0x421a5a; continue;
 }
 0x421a5a => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x421a98; continue; }
  pc = 0x421a5f; continue;
 }
 0x421a5f => {
  cpu.push(0x421a64); f_004212eb(cpu); if cpu.returned_to != 0x421a64 { return; }
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
 0x421a98 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x421ab0; continue; }
  pc = 0x421a9d; continue;
 }
 0x421a9d => {
  cpu.push(0x421aa2); f_00421320(cpu); if cpu.returned_to != 0x421aa2 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x421aa9); f_00421320(cpu); if cpu.returned_to != 0x421aa9 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x421843; continue;
 }
 0x421ab0 => {
  cpu.push(0x421ab5); f_00421671(cpu); if cpu.returned_to != 0x421ab5 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x421abd); f_00421671(cpu); if cpu.returned_to != 0x421abd { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x27eu32); cpu.edi = v; }
  cpu.push(0x421ac8); f_00421671(cpu); if cpu.returned_to != 0x421ac8 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x421ad0); f_00421671(cpu); if cpu.returned_to != 0x421ad0 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x282u32); cpu.edi = v; }
  pc = 0x421ad6; continue;
 }
 0x421ad6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421ad7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421ad7); }
 let mut pc: u32 = 0x421ad7;
 loop { match pc {
 0x421992 => {
  cpu.push(0x421997); f_004219ed(cpu); if cpu.returned_to != 0x421997 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219a2); f_004219ed(cpu); if cpu.returned_to != 0x4219a2 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219ad); f_004219ed(cpu); if cpu.returned_to != 0x4219ad { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219b8); f_004219ed(cpu); if cpu.returned_to != 0x4219b8 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  cpu.push(0x4219c5); f_004219ed(cpu); if cpu.returned_to != 0x4219c5 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219d0); f_004219ed(cpu); if cpu.returned_to != 0x4219d0 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219db); f_004219ed(cpu); if cpu.returned_to != 0x4219db { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x4219e6); f_004219ed(cpu); if cpu.returned_to != 0x4219e6 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x8c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421ad7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x421adf; continue;
 }
 0x421adf => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x421ae6; continue; }
  pc = 0x421ae5; continue;
 }
 0x421ae5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x421ae6; continue;
 }
 0x421ae6 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421adf; continue; }
  pc = 0x421aee; continue;
 }
 0x421aee => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x421c21; continue; }
  pc = 0x421af8; continue;
 }
 0x421af8 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x421bdc; continue; }
  pc = 0x421b01; continue;
 }
 0x421b01 => {
  cpu.push(0x421b06); f_004212eb(cpu); if cpu.returned_to != 0x421b06 { return; }
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
 0x421bdc => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x421bfb; continue; }
  pc = 0x421be1; continue;
 }
 0x421be1 => {
  cpu.push(0x421be6); f_00421320(cpu); if cpu.returned_to != 0x421be6 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x421bed); f_00421334(cpu); if cpu.returned_to != 0x421bed { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x421bf4); f_00421334(cpu); if cpu.returned_to != 0x421bf4 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x421992; continue;
 }
 0x421bfb => {
  cpu.push(0x421c00); f_00421a3d(cpu); if cpu.returned_to != 0x421c00 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x421c08); f_00421a3d(cpu); if cpu.returned_to != 0x421c08 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4fcu32); cpu.edi = v; }
  cpu.push(0x421c13); f_00421a3d(cpu); if cpu.returned_to != 0x421c13 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x421c1b); f_00421a3d(cpu); if cpu.returned_to != 0x421c1b { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x504u32); cpu.edi = v; }
  pc = 0x421c21; continue;
 }
 0x421c21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421c22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421c22); }
 let mut pc: u32 = 0x421c22;
 loop { match pc {
 0x421c22 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x593aa0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x421c3b); f_004701ee(cpu); if cpu.returned_to != 0x421c3b { return; }
  if cpu.cf { pc = 0x421c69; continue; }
  pc = 0x421c3d; continue;
 }
 0x421c3d => {
  { cpu.sub8(cpu.r8(0x5ceb68u32), 0xffu8); }
  if cpu.zf { pc = 0x421c69; continue; }
  pc = 0x421c46; continue;
 }
 0x421c46 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x593aa0u32, v); }
  { let v: u8 = cpu.r8(0x5ceb67u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x593aa4u32, v); }
  { let v: u8 = cpu.r8(0x5ceb68u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x593aa8u32, v); }
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x421c69 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if !cpu.zf { pc = 0x421c79; continue; }
  pc = 0x421c72; continue;
 }
 0x421c72 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = 0xedu8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x421c79; continue;
 }
 0x421c79 => {
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421c7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421c7c); }
 let mut pc: u32 = 0x421c7c;
 loop { match pc {
 0x421c7c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x421c8e; continue;
 }
 0x421c8e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421c8e; continue; }
  pc = 0x421ca5; continue;
 }
 0x421ca5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421cab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421cab); }
 let mut pc: u32 = 0x421cab;
 loop { match pc {
 0x421cab => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4eabc0u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4eabc4u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x421cc8; continue;
 }
 0x421cc8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421cc8; continue; }
  pc = 0x421cd5; continue;
 }
 0x421cd5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421cdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421cdb); }
 let mut pc: u32 = 0x421cdb;
 loop { match pc {
 0x421cdb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x421ced; continue;
 }
 0x421ced => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421ced; continue; }
  pc = 0x421cfa; continue;
 }
 0x421cfa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421d00); }
 let mut pc: u32 = 0x421d00;
 loop { match pc {
 0x421d00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x421d16; continue;
 }
 0x421d16 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421d16; continue; }
  pc = 0x421d30; continue;
 }
 0x421d30 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421d38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421d38); }
 let mut pc: u32 = 0x421d38;
 loop { match pc {
 0x421d38 => {
  { cpu.pushad(); }
  cpu.push(0x421d3e); f_004030c0(cpu); if cpu.returned_to != 0x421d3e { return; }
  pc = 0x421d3e; continue;
 }
 0x421d3e => {
  { let v = 0x575a7du32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = 0x575ab3u32; cpu.push(v); }
  cpu.push(0x421d52); f_00403620(cpu); if cpu.returned_to != 0x421d52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x421d72; continue; }
  pc = 0x421d59; continue;
 }
 0x421d59 => {
  { let v: u32 = 0x575ab3u32; cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x3u16); }
  if cpu.zf { pc = 0x421d7b; continue; }
  pc = 0x421d64; continue;
 }
 0x421d64 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x2u16); }
  if cpu.zf { pc = 0x421d77; continue; }
  pc = 0x421d6a; continue;
 }
 0x421d6a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x5u16); }
  if cpu.zf { pc = 0x421d79; continue; }
  pc = 0x421d70; continue;
 }
 0x421d70 => {
  cpu.poll(); pc = 0x421d3e; continue;
 }
 0x421d72 => {
  pc = 0x421e38; continue;
 }
 0x421d77 => {
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421d79 => {
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421d7b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x575d16u32); cpu.ebx = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x421d70; continue; }
  pc = 0x421d8c; continue;
 }
 0x421d8c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x575a7du32); cpu.esi = v; }
  { cpu.pushad(); }
  { let v = 0x575ab3u32; cpu.push(v); }
  cpu.push(0x421da3); f_0040b6c0(cpu); if cpu.returned_to != 0x421da3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  { cpu.popad(); }
  if !cpu.zf { cpu.poll(); pc = 0x421d70; continue; }
  pc = 0x421dad; continue;
 }
 0x421dad => {
  { let v: u8 = (cpu.eax as u8) & 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575d45u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x421dcf; continue; }
  pc = 0x421dbb; continue;
 }
 0x421dbb => {
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if cpu.zf { pc = 0x421de7; continue; }
  pc = 0x421dc1; continue;
 }
 0x421dc1 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x421e02; continue; }
  pc = 0x421dc7; continue;
 }
 0x421dc7 => {
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if cpu.zf { pc = 0x421e1d; continue; }
  pc = 0x421dcd; continue;
 }
 0x421dcd => {
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421dcf => {
  { let v: u32 = cpu.r32(0x590ab0u32); cpu.edi = v; }
  { let v: u32 = 0x590af8u32; cpu.ebx = v; }
  cpu.push(0x421ddf); f_00421e3a(cpu); if cpu.returned_to != 0x421ddf { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x590ab0u32, v); }
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421de7 => {
  { let v: u32 = cpu.r32(0x590ab4u32); cpu.edi = v; }
  { let v: u32 = 0x591078u32; cpu.ebx = v; }
  cpu.push(0x421df7); f_00421e3a(cpu); if cpu.returned_to != 0x421df7 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x590ab4u32, v); }
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421e02 => {
  { let v: u32 = cpu.r32(0x590ab8u32); cpu.edi = v; }
  { let v: u32 = 0x5915f8u32; cpu.ebx = v; }
  cpu.push(0x421e12); f_00421e3a(cpu); if cpu.returned_to != 0x421e12 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x590ab8u32, v); }
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421e1d => {
  { let v: u32 = cpu.r32(0x590abcu32); cpu.edi = v; }
  { let v: u32 = 0x591b78u32; cpu.ebx = v; }
  cpu.push(0x421e2d); f_00421e3a(cpu); if cpu.returned_to != 0x421e2d { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x590abcu32, v); }
  cpu.poll(); pc = 0x421d70; continue;
 }
 0x421e38 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421e3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421e3a); }
 let mut pc: u32 = 0x421e3a;
 loop { match pc {
 0x421e3a => {
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  pc = 0x421e40; continue;
 }
 0x421e40 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3ffu32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x421e40; continue; }
  pc = 0x421e54; continue;
 }
 0x421e54 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421e55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421e55); }
 let mut pc: u32 = 0x421e55;
 loop { match pc {
 0x421e55 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x421e62); f_0040b6e0(cpu); if cpu.returned_to != 0x421e62 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421e68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421e68); }
 let mut pc: u32 = 0x421e68;
 loop { match pc {
 0x421e68 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x421e72; continue;
 }
 0x421e72 => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x575d49u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x421e84; continue; }
  pc = 0x421e80; continue;
 }
 0x421e80 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x421e8f; continue;
 }
 0x421e84 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.sf { cpu.poll(); pc = 0x421e72; continue; }
  pc = 0x421e88; continue;
 }
 0x421e88 => {
  { let v: u32 = cpu.r32(0x575d49u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x421e8f; continue;
 }
 0x421e8f => {
  { let v: u32 = cpu.eax; cpu.w32(0x575d45u32, v); }
  { let v: u32 = 0x575a91u32; cpu.edi = v; }
  { let v: u16 = 0x3u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x575d16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x575d16u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x575d31u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x575d20u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x575d20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x575d16u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575d45u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56c62fu32, v); }
  { let v: u32 = 0x5df6d0u32; cpu.esi = v; }
  cpu.push(0x421ee5); f_00421f2d(cpu); if cpu.returned_to != 0x421ee5 { return; }
  { let v: u32 = 0x575f47u32; cpu.edi = v; }
  { let v: u32 = 0x575d45u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x421ef6; continue;
 }
 0x421ef6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x421f03; continue; }
  pc = 0x421f00; continue;
 }
 0x421f00 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  pc = 0x421f03; continue;
 }
 0x421f03 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421ef6; continue; }
  pc = 0x421f09; continue;
 }
 0x421f09 => {
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5766bau32, v); }
  cpu.push(0x421f1b); f_00429630(cpu); if cpu.returned_to != 0x421f1b { return; }
  { let v: u32 = 0x56d348u32; cpu.esi = v; }
  cpu.push(0x421f25); f_00422445(cpu); if cpu.returned_to != 0x421f25 { return; }
  cpu.push(0x421f2a); f_0040b600(cpu); if cpu.returned_to != 0x421f2a { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421f2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421f2d); }
 let mut pc: u32 = 0x421f2d;
 loop { match pc {
 0x421f2d => {
  { cpu.pushad(); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x421f33; continue;
 }
 0x421f33 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x421f37; continue;
 }
 0x421f37 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x421f52; continue; }
  pc = 0x421f3c; continue;
 }
 0x421f3c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, 0x6u32); }
  if !cpu.zf { cpu.poll(); pc = 0x421f37; continue; }
  pc = 0x421f43; continue;
 }
 0x421f43 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x421f52; continue; }
  pc = 0x421f48; continue;
 }
 0x421f48 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x35u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x421f52; continue;
 }
 0x421f52 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x421f33; continue; }
  pc = 0x421f58; continue;
 }
 0x421f58 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421f5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421f5a); }
 let mut pc: u32 = 0x421f5a;
 loop { match pc {
 0x421f5a => {
  { let v: u32 = cpu.r32(0x575d16u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421f60); }
 let mut pc: u32 = 0x421f60;
 loop { match pc {
 0x421f60 => {
  { let v: u8 = 0xffu8; cpu.w8(0x575d20u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00421fd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x421fd6); }
 let mut pc: u32 = 0x421fd6;
 loop { match pc {
 0x421fd6 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422042; continue; }
  pc = 0x421fe1; continue;
 }
 0x421fe1 => {
  cpu.push(0x421fe6); f_004010e0(cpu); if cpu.returned_to != 0x421fe6 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x421ff2); f_00401130(cpu); if cpu.returned_to != 0x421ff2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422042; continue; }
  pc = 0x422005; continue;
 }
 0x422005 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422011); f_004030c0(cpu); if cpu.returned_to != 0x422011 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422020); f_00401130(cpu); if cpu.returned_to != 0x422020 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422042; continue; }
  pc = 0x422033; continue;
 }
 0x422033 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42203f); f_004030c0(cpu); if cpu.returned_to != 0x42203f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422042; continue;
 }
 0x422042 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422044(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422044); }
 let mut pc: u32 = 0x422044;
 loop { match pc {
 0x422044 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4220b0; continue; }
  pc = 0x42204f; continue;
 }
 0x42204f => {
  cpu.push(0x422054); f_004010e0(cpu); if cpu.returned_to != 0x422054 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422060); f_00401130(cpu); if cpu.returned_to != 0x422060 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4220b0; continue; }
  pc = 0x422073; continue;
 }
 0x422073 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42207f); f_004030c0(cpu); if cpu.returned_to != 0x42207f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42208e); f_00401130(cpu); if cpu.returned_to != 0x42208e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4220b0; continue; }
  pc = 0x4220a1; continue;
 }
 0x4220a1 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4220ad); f_004030c0(cpu); if cpu.returned_to != 0x4220ad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4220b0; continue;
 }
 0x4220b0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004220b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4220b2); }
 let mut pc: u32 = 0x4220b2;
 loop { match pc {
 0x4220b2 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x42211e; continue; }
  pc = 0x4220bd; continue;
 }
 0x4220bd => {
  cpu.push(0x4220c2); f_004010e0(cpu); if cpu.returned_to != 0x4220c2 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x3bu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4220ce); f_00401130(cpu); if cpu.returned_to != 0x4220ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42211e; continue; }
  pc = 0x4220e1; continue;
 }
 0x4220e1 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4220ed); f_004030c0(cpu); if cpu.returned_to != 0x4220ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4220fc); f_00401130(cpu); if cpu.returned_to != 0x4220fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42211e; continue; }
  pc = 0x42210f; continue;
 }
 0x42210f => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42211b); f_004030c0(cpu); if cpu.returned_to != 0x42211b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x42211e; continue;
 }
 0x42211e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422120); }
 let mut pc: u32 = 0x422120;
 loop { match pc {
 0x422120 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x42218c; continue; }
  pc = 0x42212b; continue;
 }
 0x42212b => {
  cpu.push(0x422130); f_004010e0(cpu); if cpu.returned_to != 0x422130 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42213c); f_00401130(cpu); if cpu.returned_to != 0x42213c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42218c; continue; }
  pc = 0x42214f; continue;
 }
 0x42214f => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42215b); f_004030c0(cpu); if cpu.returned_to != 0x42215b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42216a); f_00401130(cpu); if cpu.returned_to != 0x42216a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42218c; continue; }
  pc = 0x42217d; continue;
 }
 0x42217d => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422189); f_004030c0(cpu); if cpu.returned_to != 0x422189 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x42218c; continue;
 }
 0x42218c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042218e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42218e); }
 let mut pc: u32 = 0x42218e;
 loop { match pc {
 0x42218e => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42223e; continue; }
  pc = 0x42219b; continue;
 }
 0x42219b => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422207; continue; }
  pc = 0x4221a6; continue;
 }
 0x4221a6 => {
  cpu.push(0x4221ab); f_004010e0(cpu); if cpu.returned_to != 0x4221ab { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4221b7); f_00401130(cpu); if cpu.returned_to != 0x4221b7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422207; continue; }
  pc = 0x4221ca; continue;
 }
 0x4221ca => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4221d6); f_004030c0(cpu); if cpu.returned_to != 0x4221d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4221e5); f_00401130(cpu); if cpu.returned_to != 0x4221e5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422207; continue; }
  pc = 0x4221f8; continue;
 }
 0x4221f8 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422204); f_004030c0(cpu); if cpu.returned_to != 0x422204 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x422207; continue;
 }
 0x422207 => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x5943d5u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x422216; continue;
 }
 0x422216 => {
  { let v: u8 = cpu.r8(0x4d6920u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4d6920u32, v); }
  { let v: u8 = cpu.r8(0x576501u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594437u32, v); }
  cpu.push(0x422237); f_00422240(cpu); if cpu.returned_to != 0x422237 { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x422216; continue; }
  pc = 0x422239; continue;
 }
 0x422239 => {
  cpu.push(0x42223e); f_0042223f(cpu); if cpu.returned_to != 0x42223e { return; }
  pc = 0x42223e; continue;
 }
 0x42223e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042223f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42223f); }
 let mut pc: u32 = 0x42223f;
 loop { match pc {
 0x42223f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422240); }
 let mut pc: u32 = 0x422240;
 loop { match pc {
 0x422240 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422445(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422445); }
 let mut pc: u32 = 0x422445;
 loop { match pc {
 0x422445 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x56d1a1u32), cpu.esi); }
  if cpu.zf { pc = 0x4224a6; continue; }
  pc = 0x42244e; continue;
 }
 0x42244e => {
  cpu.push(0x422453); f_004283df(cpu); if cpu.returned_to != 0x422453 { return; }
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (cpu.sf == cpu.of) { pc = 0x42247f; continue; }
  pc = 0x422458; continue;
 }
 0x422458 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d2f8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x56d1a1u32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x56d2e3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2e4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56d2dcu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42247f => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d2f8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x56d1a1u32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x56d2e3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2e4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56d2dcu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4224a6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x56d2dcu32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x56d2e3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2e4u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422570); }
 let mut pc: u32 = 0x422570;
 loop { match pc {
 0x422570 => {
  { let v: u32 = 0xc1u32; cpu.eax = v; }
  { let v: u32 = 0x5au32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x596fa6u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596fa6u32, v); }
  { let v: u16 = cpu.r16(0x596fa8u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596fa8u32, v); }
  { let v: u16 = cpu.r16(0x596faau32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596faau32, v); }
  { let v: u16 = cpu.r16(0x596facu32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596facu32, v); }
  { let v: u16 = cpu.r16(0x596fc2u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596fc2u32, v); }
  { let v: u16 = cpu.r16(0x596fc4u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596fc4u32, v); }
  { let v: u16 = cpu.r16(0x596fc6u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596fc6u32, v); }
  { let v: u16 = cpu.r16(0x596fc8u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596fc8u32, v); }
  { let v: u16 = cpu.r16(0x596fdeu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596fdeu32, v); }
  { let v: u16 = cpu.r16(0x596fe0u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596fe0u32, v); }
  { let v: u16 = cpu.r16(0x596fe2u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596fe2u32, v); }
  { let v: u16 = cpu.r16(0x596fe4u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596fe4u32, v); }
  { let v: u16 = cpu.r16(0x596ffau32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596ffau32, v); }
  { let v: u16 = cpu.r16(0x596ffcu32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x596ffcu32, v); }
  { let v: u16 = cpu.r16(0x596ffeu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x596ffeu32, v); }
  { let v: u16 = cpu.r16(0x597000u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x597000u32, v); }
  { let v: u16 = cpu.r16(0x597016u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x597016u32, v); }
  { let v: u16 = cpu.r16(0x597018u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x597018u32, v); }
  { let v: u16 = cpu.r16(0x59701au32).wrapping_add((cpu.eax as u16)); cpu.w16(0x59701au32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x59701cu32), (cpu.ebx as u16)); cpu.w16(0x59701cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042297f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42297f); }
 let mut pc: u32 = 0x42297f;
 loop { match pc {
 0x42297f => {
  { let v: u8 = 0x1u8; cpu.w8(0x581beeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422987); }
 let mut pc: u32 = 0x422987;
 loop { match pc {
 0x422987 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581beeu32, v); }
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if !cpu.zf { pc = 0x422998; continue; }
  pc = 0x422997; continue;
 }
 0x422997 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x422998 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f30u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422a12; continue; }
  pc = 0x4229b1; continue;
 }
 0x4229b1 => {
  cpu.push(0x4229b6); f_004010e0(cpu); if cpu.returned_to != 0x4229b6 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4229c2); f_00401130(cpu); if cpu.returned_to != 0x4229c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422a12; continue; }
  pc = 0x4229d5; continue;
 }
 0x4229d5 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4229e1); f_004030c0(cpu); if cpu.returned_to != 0x4229e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4229f0); f_00401130(cpu); if cpu.returned_to != 0x4229f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422a12; continue; }
  pc = 0x422a03; continue;
 }
 0x422a03 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422a0f); f_004030c0(cpu); if cpu.returned_to != 0x422a0f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422a12; continue;
 }
 0x422a12 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422a14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422a14); }
 let mut pc: u32 = 0x422a14;
 loop { match pc {
 0x422a14 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581beeu32, v); }
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if !cpu.zf { pc = 0x422a25; continue; }
  pc = 0x422a24; continue;
 }
 0x422a24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x422a25 => {
  { let v: u8 = 0x0u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575f30u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422a9f; continue; }
  pc = 0x422a3e; continue;
 }
 0x422a3e => {
  cpu.push(0x422a43); f_004010e0(cpu); if cpu.returned_to != 0x422a43 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422a4f); f_00401130(cpu); if cpu.returned_to != 0x422a4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422a9f; continue; }
  pc = 0x422a62; continue;
 }
 0x422a62 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422a6e); f_004030c0(cpu); if cpu.returned_to != 0x422a6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422a7d); f_00401130(cpu); if cpu.returned_to != 0x422a7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422a9f; continue; }
  pc = 0x422a90; continue;
 }
 0x422a90 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422a9c); f_004030c0(cpu); if cpu.returned_to != 0x422a9c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422a9f; continue;
 }
 0x422a9f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422aa1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422aa1); }
 let mut pc: u32 = 0x422aa1;
 loop { match pc {
 0x422aa1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581befu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422aa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422aa9); }
 let mut pc: u32 = 0x422aa9;
 loop { match pc {
 0x422aa9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581befu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23c8u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422b23; continue; }
  pc = 0x422ac2; continue;
 }
 0x422ac2 => {
  cpu.push(0x422ac7); f_004010e0(cpu); if cpu.returned_to != 0x422ac7 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422ad3); f_00401130(cpu); if cpu.returned_to != 0x422ad3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422b23; continue; }
  pc = 0x422ae6; continue;
 }
 0x422ae6 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422af2); f_004030c0(cpu); if cpu.returned_to != 0x422af2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b01); f_00401130(cpu); if cpu.returned_to != 0x422b01 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422b23; continue; }
  pc = 0x422b14; continue;
 }
 0x422b14 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b20); f_004030c0(cpu); if cpu.returned_to != 0x422b20 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422b23; continue;
 }
 0x422b23 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422b25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422b25); }
 let mut pc: u32 = 0x422b25;
 loop { match pc {
 0x422b25 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581befu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23c8u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422b9f; continue; }
  pc = 0x422b3e; continue;
 }
 0x422b3e => {
  cpu.push(0x422b43); f_004010e0(cpu); if cpu.returned_to != 0x422b43 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b4f); f_00401130(cpu); if cpu.returned_to != 0x422b4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422b9f; continue; }
  pc = 0x422b62; continue;
 }
 0x422b62 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b6e); f_004030c0(cpu); if cpu.returned_to != 0x422b6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b7d); f_00401130(cpu); if cpu.returned_to != 0x422b7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422b9f; continue; }
  pc = 0x422b90; continue;
 }
 0x422b90 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422b9c); f_004030c0(cpu); if cpu.returned_to != 0x422b9c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422b9f; continue;
 }
 0x422b9f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422ba1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422ba1); }
 let mut pc: u32 = 0x422ba1;
 loop { match pc {
 0x422ba1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581bf0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00422ba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x422ba9); }
 let mut pc: u32 = 0x422ba9;
 loop { match pc {
 0x422ba9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x581bf2u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x23u32); cpu.eax = v; }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x575f6eu32, v); }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422bcf); f_0040c270(cpu); if cpu.returned_to != 0x422bcf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x55b9b8u32; cpu.eax = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422beb); f_0040c240(cpu); if cpu.returned_to != 0x422beb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x422bf4); f_00417e3e(cpu); if cpu.returned_to != 0x422bf4 { return; }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x422c60; continue; }
  pc = 0x422bff; continue;
 }
 0x422bff => {
  cpu.push(0x422c04); f_004010e0(cpu); if cpu.returned_to != 0x422c04 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x37u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422c10); f_00401130(cpu); if cpu.returned_to != 0x422c10 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422c60; continue; }
  pc = 0x422c23; continue;
 }
 0x422c23 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422c2f); f_004030c0(cpu); if cpu.returned_to != 0x422c2f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x38u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422c3e); f_00401130(cpu); if cpu.returned_to != 0x422c3e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x422c60; continue; }
  pc = 0x422c51; continue;
 }
 0x422c51 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x422c5d); f_004030c0(cpu); if cpu.returned_to != 0x422c5d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x422c60; continue;
 }
 0x422c60 => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x581bf0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00424501(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x424501); }
 let mut pc: u32 = 0x424501;
 loop { match pc {
 0x424501 => {
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42450c); cpu.call(t); if cpu.returned_to != 0x42450c { return; }
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x0u8); }
  if cpu.zf { pc = 0x42458f; continue; }
  pc = 0x424515; continue;
 }
 0x424515 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a8fu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42452a); cpu.call(t); if cpu.returned_to != 0x42452a { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5943d5u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a94u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x0u8); }
  if cpu.zf { pc = 0x42458f; continue; }
  pc = 0x424542; continue;
 }
 0x424542 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a94u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424558); cpu.call(t); if cpu.returned_to != 0x424558 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x2u8); }
  if cpu.zf { pc = 0x42456d; continue; }
  pc = 0x424562; continue;
 }
 0x424562 => {
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x3u8); }
  if cpu.zf { pc = 0x42457f; continue; }
  pc = 0x42456b; continue;
 }
 0x42456b => {
  pc = 0x42458f; continue;
 }
 0x42456d => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42457d); cpu.call(t); if cpu.returned_to != 0x42457d { return; }
  pc = 0x42458f; continue;
 }
 0x42457f => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42458f); cpu.call(t); if cpu.returned_to != 0x42458f { return; }
  pc = 0x42458f; continue;
 }
 0x42458f => {
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x0u8); }
  if cpu.zf { pc = 0x424612; continue; }
  pc = 0x424598; continue;
 }
 0x424598 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  { let v: u32 = 0x1a8eu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4245ad); cpu.call(t); if cpu.returned_to != 0x4245ad { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5943d6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a97u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x0u8); }
  if cpu.zf { pc = 0x424612; continue; }
  pc = 0x4245c5; continue;
 }
 0x4245c5 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a97u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4245db); cpu.call(t); if cpu.returned_to != 0x4245db { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x2u8); }
  if cpu.zf { pc = 0x4245f0; continue; }
  pc = 0x4245e5; continue;
 }
 0x4245e5 => {
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x3u8); }
  if cpu.zf { pc = 0x424602; continue; }
  pc = 0x4245ee; continue;
 }
 0x4245ee => {
  pc = 0x424612; continue;
 }
 0x4245f0 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424600); cpu.call(t); if cpu.returned_to != 0x424600 { return; }
  pc = 0x424612; continue;
 }
 0x424602 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424612); cpu.call(t); if cpu.returned_to != 0x424612 { return; }
  pc = 0x424612; continue;
 }
 0x424612 => {
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x0u8); }
  if cpu.zf { pc = 0x424695; continue; }
  pc = 0x42461b; continue;
 }
 0x42461b => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x1a90u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424630); cpu.call(t); if cpu.returned_to != 0x424630 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5943d7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a9au32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x0u8); }
  if cpu.zf { pc = 0x424695; continue; }
  pc = 0x424648; continue;
 }
 0x424648 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a9au32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42465e); cpu.call(t); if cpu.returned_to != 0x42465e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x2u8); }
  if cpu.zf { pc = 0x424673; continue; }
  pc = 0x424668; continue;
 }
 0x424668 => {
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x3u8); }
  if cpu.zf { pc = 0x424685; continue; }
  pc = 0x424671; continue;
 }
 0x424671 => {
  pc = 0x424695; continue;
 }
 0x424673 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424683); cpu.call(t); if cpu.returned_to != 0x424683 { return; }
  pc = 0x424695; continue;
 }
 0x424685 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424695); cpu.call(t); if cpu.returned_to != 0x424695 { return; }
  pc = 0x424695; continue;
 }
 0x424695 => {
  { cpu.sub8(cpu.r8(0x594432u32), 0x0u8); }
  if cpu.zf { pc = 0x4246e8; continue; }
  pc = 0x42469e; continue;
 }
 0x42469e => {
  { cpu.sub8(cpu.r8(0x594432u32), 0x1u8); }
  if cpu.zf { pc = 0x4246d3; continue; }
  pc = 0x4246a7; continue;
 }
 0x4246a7 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1aa2u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4246bc); cpu.call(t); if cpu.returned_to != 0x4246bc { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1aa3u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4246d1); cpu.call(t); if cpu.returned_to != 0x4246d1 { return; }
  pc = 0x4246e8; continue;
 }
 0x4246d3 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1aa1u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4246e8); cpu.call(t); if cpu.returned_to != 0x4246e8 { return; }
  pc = 0x4246e8; continue;
 }
 0x4246e8 => {
  { cpu.sub8(cpu.r8(0x594436u32), 0x0u8); }
  if cpu.zf { pc = 0x424740; continue; }
  pc = 0x4246f1; continue;
 }
 0x4246f1 => {
  { let v: u32 = 0x1a6du32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x594436u32), 0x1u8); }
  if cpu.zf { pc = 0x42472b; continue; }
  pc = 0x4246ff; continue;
 }
 0x4246ff => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x16aeu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424714); cpu.call(t); if cpu.returned_to != 0x424714 { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x16afu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424729); cpu.call(t); if cpu.returned_to != 0x424729 { return; }
  pc = 0x424740; continue;
 }
 0x42472b => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x16b0u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x424740); cpu.call(t); if cpu.returned_to != 0x424740 { return; }
  pc = 0x424740; continue;
 }
 0x424740 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x590817u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x598124u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x42475e; continue; }
  pc = 0x424757; continue;
 }
 0x424757 => {
  { let v: u32 = 0x4ea761u32; cpu.esi = v; }
  pc = 0x424788; continue;
 }
 0x42475e => {
  { let v: u16 = 0xc200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  cpu.push(0x424776); f_0041b6f1(cpu); if cpu.returned_to != 0x424776 { return; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x424780; continue;
 }
 0x424780 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x424788; continue; }
  pc = 0x424785; continue;
 }
 0x424785 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x424780; continue; }
  pc = 0x424788; continue;
 }
 0x424788 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5943d4u32), 0x0u8); }
  if cpu.zf { pc = 0x4247a2; continue; }
  pc = 0x42479b; continue;
 }
 0x42479b => {
  cpu.push(0x4247a0); f_004283c1(cpu); if cpu.returned_to != 0x4247a0 { return; }
  pc = 0x4247a7; continue;
 }
 0x4247a2 => {
  cpu.push(0x4247a7); f_004283d0(cpu); if cpu.returned_to != 0x4247a7 { return; }
  pc = 0x4247a7; continue;
 }
 0x4247a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x594437u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if (cpu.sf != cpu.of) { pc = 0x4247d1; continue; }
  pc = 0x4247b4; continue;
 }
 0x4247b4 => {
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x54a7d0u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x54a7d1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x54a7d2u32, v); }
  pc = 0x4247df; continue;
 }
 0x4247d1 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x54a7d0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x54a7d1u32, v); }
  pc = 0x4247df; continue;
 }
 0x4247df => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x0u8); }
  if cpu.zf { pc = 0x4247fe; continue; }
  pc = 0x4247f7; continue;
 }
 0x4247f7 => {
  cpu.push(0x4247fc); f_004283c1(cpu); if cpu.returned_to != 0x4247fc { return; }
  pc = 0x424803; continue;
 }
 0x4247fe => {
  cpu.push(0x424803); f_004283d0(cpu); if cpu.returned_to != 0x424803 { return; }
  pc = 0x424803; continue;
 }
 0x424803 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d6920u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x31u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x54a7d0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x54a7d1u32, v); }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x0u8); }
  if cpu.zf { pc = 0x42483d; continue; }
  pc = 0x424836; continue;
 }
 0x424836 => {
  cpu.push(0x42483b); f_004283c1(cpu); if cpu.returned_to != 0x42483b { return; }
  pc = 0x424842; continue;
 }
 0x42483d => {
  cpu.push(0x424842); f_004283d0(cpu); if cpu.returned_to != 0x424842 { return; }
  pc = 0x424842; continue;
 }
 0x424842 => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x42484b); f_00427666(cpu); if cpu.returned_to != 0x42484b { return; }
  { let v: u16 = 0xccu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x70u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x0u8); }
  if cpu.zf { pc = 0x424868; continue; }
  pc = 0x424861; continue;
 }
 0x424861 => {
  cpu.push(0x424866); f_004283c1(cpu); if cpu.returned_to != 0x424866 { return; }
  pc = 0x42486d; continue;
 }
 0x424868 => {
  cpu.push(0x42486d); f_004283d0(cpu); if cpu.returned_to != 0x42486d { return; }
  pc = 0x42486d; continue;
 }
 0x42486d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00424f52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x424f52); }
 let mut pc: u32 = 0x424f52;
 loop { match pc {
 0x424f52 => {
  { let v: u32 = cpu.eax.wrapping_add(0x54a7d1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xcau32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xcau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d6u32), 0x0u8); }
  if cpu.zf { pc = 0x424f77; continue; }
  pc = 0x424f70; continue;
 }
 0x424f70 => {
  cpu.push(0x424f75); f_004283c1(cpu); if cpu.returned_to != 0x424f75 { return; }
  pc = 0x424f7c; continue;
 }
 0x424f77 => {
  cpu.push(0x424f7c); f_004283d0(cpu); if cpu.returned_to != 0x424f7c { return; }
  pc = 0x424f7c; continue;
 }
 0x424f7c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d6920u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x31u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x54a7d0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x54a7d1u32, v); }
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d5u32), 0x0u8); }
  if cpu.zf { pc = 0x424fb6; continue; }
  pc = 0x424faf; continue;
 }
 0x424faf => {
  cpu.push(0x424fb4); f_004283c1(cpu); if cpu.returned_to != 0x424fb4 { return; }
  pc = 0x424fbb; continue;
 }
 0x424fb6 => {
  cpu.push(0x424fbb); f_004283d0(cpu); if cpu.returned_to != 0x424fbb { return; }
  pc = 0x424fbb; continue;
 }
 0x424fbb => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x424fc4); f_00427666(cpu); if cpu.returned_to != 0x424fc4 { return; }
  { let v: u16 = 0xcau16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5943d7u32), 0x0u8); }
  if cpu.zf { pc = 0x424fe1; continue; }
  pc = 0x424fda; continue;
 }
 0x424fda => {
  cpu.push(0x424fdf); f_004283c1(cpu); if cpu.returned_to != 0x424fdf { return; }
  pc = 0x424fe6; continue;
 }
 0x424fe1 => {
  cpu.push(0x424fe6); f_004283d0(cpu); if cpu.returned_to != 0x424fe6 { return; }
  pc = 0x424fe6; continue;
 }
 0x424fe6 => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x7bu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5add00u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x59805du32), 0x0u8); }
  if cpu.zf { pc = 0x425005; continue; }
  pc = 0x424fff; continue;
 }
 0x424fff => {
  { let v: u32 = cpu.r32(0x5add04u32); cpu.esi = v; }
  pc = 0x425005; continue;
 }
 0x425005 => {
  { cpu.sub8(cpu.r8(0x5943d8u32), 0x0u8); }
  if cpu.zf { pc = 0x425015; continue; }
  pc = 0x42500e; continue;
 }
 0x42500e => {
  cpu.push(0x425013); f_004283c1(cpu); if cpu.returned_to != 0x425013 { return; }
  pc = 0x42501a; continue;
 }
 0x425015 => {
  cpu.push(0x42501a); f_004283d0(cpu); if cpu.returned_to != 0x42501a { return; }
  pc = 0x42501a; continue;
 }
 0x42501a => {
  { let v: u32 = 0x4e1fd2u32; cpu.esi = v; }
  cpu.push(0x425024); f_004283df(cpu); if cpu.returned_to != 0x425024 { return; }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42503f; continue; }
  pc = 0x425029; continue;
 }
 0x425029 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  cpu.push(0x425033); f_00428531(cpu); if cpu.returned_to != 0x425033 { return; }
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x8bu32; cpu.ebx = v; }
  pc = 0x42504e; continue;
 }
 0x42503f => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x8bu32; cpu.ebx = v; }
  { let v: u32 = 0x4e1fd2u32; cpu.esi = v; }
  pc = 0x42504e; continue;
 }
 0x42504e => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x1u8); }
  if cpu.zf { pc = 0x425067; continue; }
  pc = 0x425057; continue;
 }
 0x425057 => {
  { cpu.sub8(cpu.r8(0x5943d9u32), 0x0u8); }
  if !cpu.zf { pc = 0x425067; continue; }
  pc = 0x425060; continue;
 }
 0x425060 => {
  cpu.push(0x425065); f_004283d0(cpu); if cpu.returned_to != 0x425065 { return; }
  pc = 0x425096; continue;
 }
 0x425067 => {
  cpu.push(0x42506c); f_004283c1(cpu); if cpu.returned_to != 0x42506c { return; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x1u8); }
  if !cpu.zf { pc = 0x425096; continue; }
  pc = 0x425075; continue;
 }
 0x425075 => {
  { cpu.sub8(cpu.r8(0x59445cu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x425096; continue; }
  pc = 0x42507e; continue;
 }
 0x42507e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x56c9bbu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56c9bfu32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x425095); cpu.call(t); if cpu.returned_to != 0x425095 { return; }
  { cpu.popad(); }
  pc = 0x425096; continue;
 }
 0x425096 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425456(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425456); }
 let mut pc: u32 = 0x425456;
 loop { match pc {
 0x425456 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x4ea80fu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x1u8); }
  if cpu.zf { pc = 0x42547c; continue; }
  pc = 0x42546c; continue;
 }
 0x42546c => {
  { cpu.sub8(cpu.r8(0x5943dbu32), 0x0u8); }
  if !cpu.zf { pc = 0x42547c; continue; }
  pc = 0x425475; continue;
 }
 0x425475 => {
  cpu.push(0x42547a); f_004283d0(cpu); if cpu.returned_to != 0x42547a { return; }
  pc = 0x4254ab; continue;
 }
 0x42547c => {
  cpu.push(0x425481); f_004283c1(cpu); if cpu.returned_to != 0x425481 { return; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x1u8); }
  if !cpu.zf { pc = 0x4254ab; continue; }
  pc = 0x42548a; continue;
 }
 0x42548a => {
  { cpu.sub8(cpu.r8(0x59445cu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x4254ab; continue; }
  pc = 0x425493; continue;
 }
 0x425493 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x56c9bbu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56c9bfu32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4254aa); cpu.call(t); if cpu.returned_to != 0x4254aa { return; }
  { cpu.popad(); }
  pc = 0x4254ab; continue;
 }
 0x4254ab => {
  { let v: u32 = 0x4eaf70u32; cpu.esi = v; }
  cpu.push(0x4254b5); f_004283df(cpu); if cpu.returned_to != 0x4254b5 { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4254d0; continue; }
  pc = 0x4254ba; continue;
 }
 0x4254ba => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x4254c4); f_00428531(cpu); if cpu.returned_to != 0x4254c4 { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  pc = 0x4254df; continue;
 }
 0x4254d0 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = 0x4eaf70u32; cpu.esi = v; }
  pc = 0x4254df; continue;
 }
 0x4254df => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x2u8); }
  if cpu.zf { pc = 0x4254f8; continue; }
  pc = 0x4254e8; continue;
 }
 0x4254e8 => {
  { cpu.sub8(cpu.r8(0x5943dcu32), 0x0u8); }
  if !cpu.zf { pc = 0x4254f8; continue; }
  pc = 0x4254f1; continue;
 }
 0x4254f1 => {
  cpu.push(0x4254f6); f_004283d0(cpu); if cpu.returned_to != 0x4254f6 { return; }
  pc = 0x425527; continue;
 }
 0x4254f8 => {
  cpu.push(0x4254fd); f_004283c1(cpu); if cpu.returned_to != 0x4254fd { return; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x2u8); }
  if !cpu.zf { pc = 0x425527; continue; }
  pc = 0x425506; continue;
 }
 0x425506 => {
  { cpu.sub8(cpu.r8(0x59445cu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x425527; continue; }
  pc = 0x42550f; continue;
 }
 0x42550f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x56c9bbu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56c9bfu32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x425526); cpu.call(t); if cpu.returned_to != 0x425526 { return; }
  { cpu.popad(); }
  pc = 0x425527; continue;
 }
 0x425527 => {
  { let v: u32 = 0x590879u32; cpu.esi = v; }
  cpu.push(0x425531); f_004283df(cpu); if cpu.returned_to != 0x425531 { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42554c; continue; }
  pc = 0x425536; continue;
 }
 0x425536 => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x425540); f_00428531(cpu); if cpu.returned_to != 0x425540 { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  pc = 0x42555b; continue;
 }
 0x42554c => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x590879u32; cpu.esi = v; }
  pc = 0x42555b; continue;
 }
 0x42555b => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x3u8); }
  if cpu.zf { pc = 0x425574; continue; }
  pc = 0x425564; continue;
 }
 0x425564 => {
  { cpu.sub8(cpu.r8(0x5943ddu32), 0x0u8); }
  if !cpu.zf { pc = 0x425574; continue; }
  pc = 0x42556d; continue;
 }
 0x42556d => {
  cpu.push(0x425572); f_004283d0(cpu); if cpu.returned_to != 0x425572 { return; }
  pc = 0x4255a3; continue;
 }
 0x425574 => {
  cpu.push(0x425579); f_004283c1(cpu); if cpu.returned_to != 0x425579 { return; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x3u8); }
  if !cpu.zf { pc = 0x4255a3; continue; }
  pc = 0x425582; continue;
 }
 0x425582 => {
  { cpu.sub8(cpu.r8(0x59445cu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x4255a3; continue; }
  pc = 0x42558b; continue;
 }
 0x42558b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x56c9bbu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56c9bfu32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4255a2); cpu.call(t); if cpu.returned_to != 0x4255a2 { return; }
  { cpu.popad(); }
  pc = 0x4255a3; continue;
 }
 0x4255a3 => {
  { let v: u32 = 0x59806au32; cpu.esi = v; }
  cpu.push(0x4255ad); f_004283df(cpu); if cpu.returned_to != 0x4255ad { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4255c8; continue; }
  pc = 0x4255b2; continue;
 }
 0x4255b2 => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x4255bc); f_00428531(cpu); if cpu.returned_to != 0x4255bc { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  pc = 0x4255d7; continue;
 }
 0x4255c8 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x59806au32; cpu.esi = v; }
  pc = 0x4255d7; continue;
 }
 0x4255d7 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x4u8); }
  if cpu.zf { pc = 0x4255f0; continue; }
  pc = 0x4255e0; continue;
 }
 0x4255e0 => {
  { cpu.sub8(cpu.r8(0x5943deu32), 0x0u8); }
  if !cpu.zf { pc = 0x4255f0; continue; }
  pc = 0x4255e9; continue;
 }
 0x4255e9 => {
  cpu.push(0x4255ee); f_004283d0(cpu); if cpu.returned_to != 0x4255ee { return; }
  pc = 0x42561f; continue;
 }
 0x4255f0 => {
  cpu.push(0x4255f5); f_004283c1(cpu); if cpu.returned_to != 0x4255f5 { return; }
  { cpu.sub8(cpu.r8(0x594454u32), 0x4u8); }
  if !cpu.zf { pc = 0x42561f; continue; }
  pc = 0x4255fe; continue;
 }
 0x4255fe => {
  { cpu.sub8(cpu.r8(0x59445cu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x42561f; continue; }
  pc = 0x425607; continue;
 }
 0x425607 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x56c9bbu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56c9bfu32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42561e); cpu.call(t); if cpu.returned_to != 0x42561e { return; }
  { cpu.popad(); }
  pc = 0x42561f; continue;
 }
 0x42561f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004257b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4257b4); }
 let mut pc: u32 = 0x4257b4;
 loop { match pc {
 0x4257b4 => {
  cpu.push(0x4257b9); f_00425bac(cpu); if cpu.returned_to != 0x4257b9 { return; }
  { cpu.sub8(cpu.r8(0x594405u32), 0x0u8); }
  if !cpu.zf { pc = 0x4257c2; continue; }
  pc = 0x4257c2; continue;
 }
 0x4257c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594405u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004257d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4257d8); }
 let mut pc: u32 = 0x4257d8;
 loop { match pc {
 0x4257d8 => {
  { cpu.sub8(cpu.r8(0x5824b3u32), 0x0u8); }
  if cpu.zf { pc = 0x4257e2; continue; }
  pc = 0x4257e1; continue;
 }
 0x4257e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4257e2 => {
  { cpu.sub8(cpu.r8(0x594405u32), 0x0u8); }
  if !cpu.zf { pc = 0x4257eb; continue; }
  pc = 0x4257eb; continue;
 }
 0x4257eb => {
  cpu.push(0x4257f0); f_00425bac(cpu); if cpu.returned_to != 0x4257f0 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x594406u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425806(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425806); }
 let mut pc: u32 = 0x425806;
 loop { match pc {
 0x425806 => {
  cpu.push(0x42580b); f_00425bac(cpu); if cpu.returned_to != 0x42580b { return; }
  { cpu.sub8(cpu.r8(0x594405u32), 0x0u8); }
  if !cpu.zf { pc = 0x425814; continue; }
  pc = 0x425814; continue;
 }
 0x425814 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59440cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042582a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42582a); }
 let mut pc: u32 = 0x42582a;
 loop { match pc {
 0x42582a => {
  cpu.push(0x42582f); f_00425bac(cpu); if cpu.returned_to != 0x42582f { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x594407u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425845(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425845); }
 let mut pc: u32 = 0x425845;
 loop { match pc {
 0x425845 => {
  cpu.push(0x42584a); f_00425bac(cpu); if cpu.returned_to != 0x42584a { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x594408u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042587b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42587b); }
 let mut pc: u32 = 0x42587b;
 loop { match pc {
 0x42587b => {
  { cpu.sub8(cpu.r8(0x59440bu32), 0x0u8); }
  if !cpu.zf { pc = 0x425897; continue; }
  pc = 0x425884; continue;
 }
 0x425884 => {
  cpu.push(0x425889); f_00425bac(cpu); if cpu.returned_to != 0x425889 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  pc = 0x425897; continue;
 }
 0x425897 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425898(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425898); }
 let mut pc: u32 = 0x425898;
 loop { match pc {
 0x425898 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f5du32, v); }
  cpu.push(0x4258a4); f_00425bac(cpu); if cpu.returned_to != 0x4258a4 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x594405u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425925; continue; }
  pc = 0x4258c4; continue;
 }
 0x4258c4 => {
  cpu.push(0x4258c9); f_004010e0(cpu); if cpu.returned_to != 0x4258c9 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4258d5); f_00401130(cpu); if cpu.returned_to != 0x4258d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425925; continue; }
  pc = 0x4258e8; continue;
 }
 0x4258e8 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4258f4); f_004030c0(cpu); if cpu.returned_to != 0x4258f4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425903); f_00401130(cpu); if cpu.returned_to != 0x425903 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425925; continue; }
  pc = 0x425916; continue;
 }
 0x425916 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425922); f_004030c0(cpu); if cpu.returned_to != 0x425922 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x425925; continue;
 }
 0x425925 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425927(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425927); }
 let mut pc: u32 = 0x425927;
 loop { match pc {
 0x425927 => {
  { cpu.sub8(cpu.r8(0x5824b3u32), 0x0u8); }
  if cpu.zf { pc = 0x425931; continue; }
  pc = 0x425930; continue;
 }
 0x425930 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425931 => {
  cpu.push(0x425936); f_00425bac(cpu); if cpu.returned_to != 0x425936 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x594406u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4259b7; continue; }
  pc = 0x425956; continue;
 }
 0x425956 => {
  cpu.push(0x42595b); f_004010e0(cpu); if cpu.returned_to != 0x42595b { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425967); f_00401130(cpu); if cpu.returned_to != 0x425967 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4259b7; continue; }
  pc = 0x42597a; continue;
 }
 0x42597a => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425986); f_004030c0(cpu); if cpu.returned_to != 0x425986 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425995); f_00401130(cpu); if cpu.returned_to != 0x425995 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4259b7; continue; }
  pc = 0x4259a8; continue;
 }
 0x4259a8 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4259b4); f_004030c0(cpu); if cpu.returned_to != 0x4259b4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4259b7; continue;
 }
 0x4259b7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004259b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4259b9); }
 let mut pc: u32 = 0x4259b9;
 loop { match pc {
 0x4259b9 => {
  cpu.push(0x4259be); f_00425bac(cpu); if cpu.returned_to != 0x4259be { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x59440cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425a3f; continue; }
  pc = 0x4259de; continue;
 }
 0x4259de => {
  cpu.push(0x4259e3); f_004010e0(cpu); if cpu.returned_to != 0x4259e3 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4259ef); f_00401130(cpu); if cpu.returned_to != 0x4259ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425a3f; continue; }
  pc = 0x425a02; continue;
 }
 0x425a02 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425a0e); f_004030c0(cpu); if cpu.returned_to != 0x425a0e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425a1d); f_00401130(cpu); if cpu.returned_to != 0x425a1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425a3f; continue; }
  pc = 0x425a30; continue;
 }
 0x425a30 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425a3c); f_004030c0(cpu); if cpu.returned_to != 0x425a3c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x425a3f; continue;
 }
 0x425a3f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425a41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425a41); }
 let mut pc: u32 = 0x425a41;
 loop { match pc {
 0x425a41 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f5du32, v); }
  cpu.push(0x425a4d); f_00425bac(cpu); if cpu.returned_to != 0x425a4d { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x594407u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425ace; continue; }
  pc = 0x425a6d; continue;
 }
 0x425a6d => {
  cpu.push(0x425a72); f_004010e0(cpu); if cpu.returned_to != 0x425a72 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425a7e); f_00401130(cpu); if cpu.returned_to != 0x425a7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425ace; continue; }
  pc = 0x425a91; continue;
 }
 0x425a91 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425a9d); f_004030c0(cpu); if cpu.returned_to != 0x425a9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425aac); f_00401130(cpu); if cpu.returned_to != 0x425aac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425ace; continue; }
  pc = 0x425abf; continue;
 }
 0x425abf => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425acb); f_004030c0(cpu); if cpu.returned_to != 0x425acb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x425ace; continue;
 }
 0x425ace => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425ad0); }
 let mut pc: u32 = 0x425ad0;
 loop { match pc {
 0x425ad0 => {
  cpu.push(0x425ad5); f_00425bac(cpu); if cpu.returned_to != 0x425ad5 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x594408u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425b4f; continue; }
  pc = 0x425aee; continue;
 }
 0x425aee => {
  cpu.push(0x425af3); f_004010e0(cpu); if cpu.returned_to != 0x425af3 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425aff); f_00401130(cpu); if cpu.returned_to != 0x425aff { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425b4f; continue; }
  pc = 0x425b12; continue;
 }
 0x425b12 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425b1e); f_004030c0(cpu); if cpu.returned_to != 0x425b1e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425b2d); f_00401130(cpu); if cpu.returned_to != 0x425b2d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425b4f; continue; }
  pc = 0x425b40; continue;
 }
 0x425b40 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425b4c); f_004030c0(cpu); if cpu.returned_to != 0x425b4c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x425b4f; continue;
 }
 0x425b4f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425bac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425bac); }
 let mut pc: u32 = 0x425bac;
 loop { match pc {
 0x425bac => {
  { let v: u8 = 0x0u8; cpu.w8(0x594405u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594406u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594407u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594408u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594409u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425bdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425bdf); }
 let mut pc: u32 = 0x425bdf;
 loop { match pc {
 0x425bdf => {
  { let v: u8 = cpu.r8(0x59440du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x425bf6; continue; }
  pc = 0x425be8; continue;
 }
 0x425be8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59805cu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x594435u32, v); }
  pc = 0x425c01; continue;
 }
 0x425bf6 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x56c95eu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59805cu32, v); }
  pc = 0x425c01; continue;
 }
 0x425c01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425c02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425c02); }
 let mut pc: u32 = 0x425c02;
 loop { match pc {
 0x425c02 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425c0c; continue; }
  pc = 0x425c0b; continue;
 }
 0x425c0b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425c0c => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425c78; continue; }
  pc = 0x425c17; continue;
 }
 0x425c17 => {
  cpu.push(0x425c1c); f_004010e0(cpu); if cpu.returned_to != 0x425c1c { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425c28); f_00401130(cpu); if cpu.returned_to != 0x425c28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425c78; continue; }
  pc = 0x425c3b; continue;
 }
 0x425c3b => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425c47); f_004030c0(cpu); if cpu.returned_to != 0x425c47 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425c56); f_00401130(cpu); if cpu.returned_to != 0x425c56 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425c78; continue; }
  pc = 0x425c69; continue;
 }
 0x425c69 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425c75); f_004030c0(cpu); if cpu.returned_to != 0x425c75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x425c78; continue;
 }
 0x425c78 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425c9f; continue; }
  pc = 0x425c82; continue;
 }
 0x425c82 => {
  { let v: u8 = 0x3u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x59440du32)); cpu.w8(0x59440du32, v); }
  { cpu.sub8(cpu.r8(0x59440du32), 0x0u8); }
  if !cpu.sf { pc = 0x425c9f; continue; }
  pc = 0x425c98; continue;
 }
 0x425c98 => {
  { let v: u8 = 0x2u8; cpu.w8(0x59440du32, v); }
  pc = 0x425c9f; continue;
 }
 0x425c9f => {
  cpu.push(0x425ca4); f_00425bdf(cpu); if cpu.returned_to != 0x425ca4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425ca5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425ca5); }
 let mut pc: u32 = 0x425ca5;
 loop { match pc {
 0x425ca5 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425caf; continue; }
  pc = 0x425cae; continue;
 }
 0x425cae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425caf => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425d1b; continue; }
  pc = 0x425cba; continue;
 }
 0x425cba => {
  cpu.push(0x425cbf); f_004010e0(cpu); if cpu.returned_to != 0x425cbf { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425ccb); f_00401130(cpu); if cpu.returned_to != 0x425ccb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425d1b; continue; }
  pc = 0x425cde; continue;
 }
 0x425cde => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425cea); f_004030c0(cpu); if cpu.returned_to != 0x425cea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425cf9); f_00401130(cpu); if cpu.returned_to != 0x425cf9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425d1b; continue; }
  pc = 0x425d0c; continue;
 }
 0x425d0c => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425d18); f_004030c0(cpu); if cpu.returned_to != 0x425d18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x425d1b; continue;
 }
 0x425d1b => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425d42; continue; }
  pc = 0x425d25; continue;
 }
 0x425d25 => {
  { let v: u8 = 0x2u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x59440du32)); cpu.w8(0x59440du32, v); }
  { cpu.sub8(cpu.r8(0x59440du32), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x425d42; continue; }
  pc = 0x425d3b; continue;
 }
 0x425d3b => {
  { let v: u8 = 0x0u8; cpu.w8(0x59440du32, v); }
  pc = 0x425d42; continue;
 }
 0x425d42 => {
  cpu.push(0x425d47); f_00425bdf(cpu); if cpu.returned_to != 0x425d47 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425d48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425d48); }
 let mut pc: u32 = 0x425d48;
 loop { match pc {
 0x425d48 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425d5b; continue; }
  pc = 0x425d51; continue;
 }
 0x425d51 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x425d5b; continue; }
  pc = 0x425d5a; continue;
 }
 0x425d5a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425d5b => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425dc7; continue; }
  pc = 0x425d66; continue;
 }
 0x425d66 => {
  cpu.push(0x425d6b); f_004010e0(cpu); if cpu.returned_to != 0x425d6b { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425d77); f_00401130(cpu); if cpu.returned_to != 0x425d77 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425dc7; continue; }
  pc = 0x425d8a; continue;
 }
 0x425d8a => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425d96); f_004030c0(cpu); if cpu.returned_to != 0x425d96 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425da5); f_00401130(cpu); if cpu.returned_to != 0x425da5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425dc7; continue; }
  pc = 0x425db8; continue;
 }
 0x425db8 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425dc4); f_004030c0(cpu); if cpu.returned_to != 0x425dc4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x425dc7; continue;
 }
 0x425dc7 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425de5; continue; }
  pc = 0x425dd1; continue;
 }
 0x425dd1 => {
  { let v: u8 = 0x3u8; cpu.w8(0x59442fu32, v); }
  pc = 0x425dd8; continue;
 }
 0x425dd8 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x594410u32)); cpu.w8(0x594410u32, v); }
  { let v: u8 = cpu.r8(0x594410u32) & 0x3fu8; cpu.w8(0x594410u32, v); }
  pc = 0x425de5; continue;
 }
 0x425de5 => {
  cpu.push(0x425dea); f_00425eb2(cpu); if cpu.returned_to != 0x425dea { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x425dd8; continue; }
  pc = 0x425dee; continue;
 }
 0x425dee => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x425dd8; continue; }
  pc = 0x425dfc; continue;
 }
 0x425dfc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425dfd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425dfd); }
 let mut pc: u32 = 0x425dfd;
 loop { match pc {
 0x425dfd => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425e10; continue; }
  pc = 0x425e06; continue;
 }
 0x425e06 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x425e10; continue; }
  pc = 0x425e0f; continue;
 }
 0x425e0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425e10 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425e7c; continue; }
  pc = 0x425e1b; continue;
 }
 0x425e1b => {
  cpu.push(0x425e20); f_004010e0(cpu); if cpu.returned_to != 0x425e20 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425e2c); f_00401130(cpu); if cpu.returned_to != 0x425e2c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425e7c; continue; }
  pc = 0x425e3f; continue;
 }
 0x425e3f => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425e4b); f_004030c0(cpu); if cpu.returned_to != 0x425e4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425e5a); f_00401130(cpu); if cpu.returned_to != 0x425e5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425e7c; continue; }
  pc = 0x425e6d; continue;
 }
 0x425e6d => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425e79); f_004030c0(cpu); if cpu.returned_to != 0x425e79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x425e7c; continue;
 }
 0x425e7c => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425e9a; continue; }
  pc = 0x425e86; continue;
 }
 0x425e86 => {
  { let v: u8 = 0x2u8; cpu.w8(0x59442fu32, v); }
  pc = 0x425e8d; continue;
 }
 0x425e8d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594410u32)); cpu.w8(0x594410u32, v); }
  { let v: u8 = cpu.r8(0x594410u32) & 0x3fu8; cpu.w8(0x594410u32, v); }
  pc = 0x425e9a; continue;
 }
 0x425e9a => {
  cpu.push(0x425e9f); f_00425eb2(cpu); if cpu.returned_to != 0x425e9f { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x425e8d; continue; }
  pc = 0x425ea3; continue;
 }
 0x425ea3 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x425e8d; continue; }
  pc = 0x425eb1; continue;
 }
 0x425eb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425eb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425eb2); }
 let mut pc: u32 = 0x425eb2;
 loop { match pc {
 0x425eb2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425ed5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425ed5); }
 let mut pc: u32 = 0x425ed5;
 loop { match pc {
 0x425ed5 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425eee; continue; }
  pc = 0x425ede; continue;
 }
 0x425ede => {
  { cpu.sub8(cpu.r8(0x594436u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x425eee; continue; }
  pc = 0x425ee7; continue;
 }
 0x425ee7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594436u32, v); }
  pc = 0x425eee; continue;
 }
 0x425eee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425eef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425eef); }
 let mut pc: u32 = 0x425eef;
 loop { match pc {
 0x425eef => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x425f5b; continue; }
  pc = 0x425efa; continue;
 }
 0x425efa => {
  cpu.push(0x425eff); f_004010e0(cpu); if cpu.returned_to != 0x425eff { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425f0b); f_00401130(cpu); if cpu.returned_to != 0x425f0b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425f5b; continue; }
  pc = 0x425f1e; continue;
 }
 0x425f1e => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425f2a); f_004030c0(cpu); if cpu.returned_to != 0x425f2a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425f39); f_00401130(cpu); if cpu.returned_to != 0x425f39 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x425f5b; continue; }
  pc = 0x425f4c; continue;
 }
 0x425f4c => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x425f58); f_004030c0(cpu); if cpu.returned_to != 0x425f58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x425f5b; continue;
 }
 0x425f5b => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425f75; continue; }
  pc = 0x425f65; continue;
 }
 0x425f65 => {
  { cpu.sub8(cpu.r8(0x594436u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x425f75; continue; }
  pc = 0x425f6e; continue;
 }
 0x425f6e => {
  { let v: u8 = 0x2u8; cpu.w8(0x594436u32, v); }
  pc = 0x425f75; continue;
 }
 0x425f75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425f76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x425f76); }
 let mut pc: u32 = 0x425f76;
 loop { match pc {
 0x425f76 => {
  { cpu.sub8(cpu.r8(0x56d2dau32), 0x0u8); }
  if cpu.zf { pc = 0x425f89; continue; }
  pc = 0x425f7f; continue;
 }
 0x425f7f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425f89; continue; }
  pc = 0x425f88; continue;
 }
 0x425f88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425f89 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x425fb7; continue; }
  pc = 0x425f92; continue;
 }
 0x425f92 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.sub8(cpu.r8(0x594432u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x425fb7; continue; }
  pc = 0x425fa2; continue;
 }
 0x425fa2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  pc = 0x425fb7; continue;
 }
 0x425fb7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00425fb8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x425fb8); } r_00425fb8(cpu, 0x425fb8); }
pub fn f_00426000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x426000); } r_00425fb8(cpu, 0x426000); }
fn r_00425fb8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x425fb8 => {
  { cpu.sub8(cpu.r8(0x56d2dau32), 0x0u8); }
  if cpu.zf { pc = 0x425fcb; continue; }
  pc = 0x425fc1; continue;
 }
 0x425fc1 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x425fcb; continue; }
  pc = 0x425fca; continue;
 }
 0x425fca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x425fcb => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42606e; continue; }
  pc = 0x425fd8; continue;
 }
 0x425fd8 => {
  { cpu.sub8(cpu.r8(0x594432u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x42606e; continue; }
  pc = 0x425fe5; continue;
 }
 0x425fe5 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426051; continue; }
  pc = 0x425ff0; continue;
 }
 0x425ff0 => {
  cpu.push(0x425ff5); f_004010e0(cpu); if cpu.returned_to != 0x425ff5 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426001); f_00401130(cpu); if cpu.returned_to != 0x426001 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426051; continue; }
  pc = 0x426014; continue;
 }
 0x426000 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x58a304c4u32.wrapping_add(cpu.ebx))); cpu.w32(0x58a304c4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x61u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426051; continue; }
  pc = 0x426014; continue;
 }
 0x426014 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426020); f_004030c0(cpu); if cpu.returned_to != 0x426020 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42602f); f_00401130(cpu); if cpu.returned_to != 0x42602f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426051; continue; }
  pc = 0x426042; continue;
 }
 0x426042 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42604e); f_004030c0(cpu); if cpu.returned_to != 0x42604e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x426051; continue;
 }
 0x426051 => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  pc = 0x42606e; continue;
 }
 0x42606e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042606f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42606f); }
 let mut pc: u32 = 0x42606f;
 loop { match pc {
 0x42606f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x426079; continue; }
  pc = 0x426078; continue;
 }
 0x426078 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x426079 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4260a7; continue; }
  pc = 0x426082; continue;
 }
 0x426082 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { cpu.sub8(cpu.r8(0x594433u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4260a7; continue; }
  pc = 0x426092; continue;
 }
 0x426092 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594434u32, v); }
  pc = 0x4260a7; continue;
 }
 0x4260a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004260a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4260a8); }
 let mut pc: u32 = 0x4260a8;
 loop { match pc {
 0x4260a8 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4260b2; continue; }
  pc = 0x4260b1; continue;
 }
 0x4260b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4260b2 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426155; continue; }
  pc = 0x4260bf; continue;
 }
 0x4260bf => {
  { cpu.sub8(cpu.r8(0x594433u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x426155; continue; }
  pc = 0x4260cc; continue;
 }
 0x4260cc => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426138; continue; }
  pc = 0x4260d7; continue;
 }
 0x4260d7 => {
  cpu.push(0x4260dc); f_004010e0(cpu); if cpu.returned_to != 0x4260dc { return; }
  { cpu.pushad(); }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4260e8); f_00401130(cpu); if cpu.returned_to != 0x4260e8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426138; continue; }
  pc = 0x4260fb; continue;
 }
 0x4260fb => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426107); f_004030c0(cpu); if cpu.returned_to != 0x426107 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426116); f_00401130(cpu); if cpu.returned_to != 0x426116 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426138; continue; }
  pc = 0x426129; continue;
 }
 0x426129 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426135); f_004030c0(cpu); if cpu.returned_to != 0x426135 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x426138; continue;
 }
 0x426138 => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594434u32, v); }
  pc = 0x426155; continue;
 }
 0x426155 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426156(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426156); }
 let mut pc: u32 = 0x426156;
 loop { match pc {
 0x426156 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x426169; continue; }
  pc = 0x42615f; continue;
 }
 0x42615f => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x426169; continue; }
  pc = 0x426168; continue;
 }
 0x426168 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x426169 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4261a0; continue; }
  pc = 0x426172; continue;
 }
 0x426172 => {
  { cpu.sub8(cpu.r8(0x594434u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4261a0; continue; }
  pc = 0x42617b; continue;
 }
 0x42617b => {
  { cpu.sub32(cpu.r32(0x597880u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x4261a0; continue; }
  pc = 0x426184; continue;
 }
 0x426184 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x594434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  pc = 0x4261a0; continue;
 }
 0x4261a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004261a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4261a1); }
 let mut pc: u32 = 0x4261a1;
 loop { match pc {
 0x4261a1 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4261b4; continue; }
  pc = 0x4261aa; continue;
 }
 0x4261aa => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x4261b4; continue; }
  pc = 0x4261b3; continue;
 }
 0x4261b3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4261b4 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426286; continue; }
  pc = 0x4261c1; continue;
 }
 0x4261c1 => {
  { cpu.sub8(cpu.r8(0x594434u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x426286; continue; }
  pc = 0x4261ce; continue;
 }
 0x4261ce => {
  { cpu.sub32(cpu.r32(0x597880u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x426286; continue; }
  pc = 0x4261db; continue;
 }
 0x4261db => {
  { let v: u8 = 0x1u8; cpu.w8(0x59442du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x594434u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x426274; continue; }
  pc = 0x426207; continue;
 }
 0x426207 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426273; continue; }
  pc = 0x426212; continue;
 }
 0x426212 => {
  cpu.push(0x426217); f_004010e0(cpu); if cpu.returned_to != 0x426217 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x13u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426223); f_00401130(cpu); if cpu.returned_to != 0x426223 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426273; continue; }
  pc = 0x426236; continue;
 }
 0x426236 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426242); f_004030c0(cpu); if cpu.returned_to != 0x426242 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426251); f_00401130(cpu); if cpu.returned_to != 0x426251 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426273; continue; }
  pc = 0x426264; continue;
 }
 0x426264 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426270); f_004030c0(cpu); if cpu.returned_to != 0x426270 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x426273; continue;
 }
 0x426273 => {
  { cpu.popad(); }
  pc = 0x426274; continue;
 }
 0x426274 => {
  { let v: u16 = 0x0u16; cpu.w16(0x56d2b8u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x56d2bau32, v); }
  pc = 0x426286; continue;
 }
 0x426286 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426287(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426287); }
 let mut pc: u32 = 0x426287;
 loop { match pc {
 0x426287 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4262be; continue; }
  pc = 0x426290; continue;
 }
 0x426290 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4262be; continue; }
  pc = 0x426299; continue;
 }
 0x426299 => {
  { cpu.sub8(cpu.r8(0x56d2b3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4262be; continue; }
  pc = 0x4262a2; continue;
 }
 0x4262a2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2b3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  pc = 0x4262be; continue;
 }
 0x4262be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004262bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4262bf); }
 let mut pc: u32 = 0x4262bf;
 loop { match pc {
 0x4262bf => {
  { cpu.sub8(cpu.r8(0x56d2b3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4262e4; continue; }
  pc = 0x4262c8; continue;
 }
 0x4262c8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2b3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  pc = 0x4262e4; continue;
 }
 0x4262e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004262e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4262e5); }
 let mut pc: u32 = 0x4262e5;
 loop { match pc {
 0x4262e5 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4263a7; continue; }
  pc = 0x4262f2; continue;
 }
 0x4262f2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4263a7; continue; }
  pc = 0x4262ff; continue;
 }
 0x4262ff => {
  { cpu.sub8(cpu.r8(0x56d2b3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4263a7; continue; }
  pc = 0x42630c; continue;
 }
 0x42630c => {
  { let v: u8 = 0x1u8; cpu.w8(0x59442du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594432u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d2b3u32, v); }
  { let v: u32 = cpu.r32(0x5add08u32); cpu.esi = v; }
  cpu.push(0x42633a); f_00422445(cpu); if cpu.returned_to != 0x42633a { return; }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4263a6; continue; }
  pc = 0x426345; continue;
 }
 0x426345 => {
  cpu.push(0x42634a); f_004010e0(cpu); if cpu.returned_to != 0x42634a { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426356); f_00401130(cpu); if cpu.returned_to != 0x426356 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4263a6; continue; }
  pc = 0x426369; continue;
 }
 0x426369 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426375); f_004030c0(cpu); if cpu.returned_to != 0x426375 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426384); f_00401130(cpu); if cpu.returned_to != 0x426384 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4263a6; continue; }
  pc = 0x426397; continue;
 }
 0x426397 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4263a3); f_004030c0(cpu); if cpu.returned_to != 0x4263a3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4263a6; continue;
 }
 0x4263a6 => {
  { cpu.popad(); }
  pc = 0x4263a7; continue;
 }
 0x4263a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004263ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4263ed); }
 let mut pc: u32 = 0x4263ed;
 loop { match pc {
 0x4263ed => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4263f7; continue; }
  pc = 0x4263f6; continue;
 }
 0x4263f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4263f7 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426423; continue; }
  pc = 0x426400; continue;
 }
 0x426400 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594431u32, v); }
  pc = 0x426423; continue;
 }
 0x426423 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426424); }
 let mut pc: u32 = 0x426424;
 loop { match pc {
 0x426424 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x426437; continue; }
  pc = 0x42642d; continue;
 }
 0x42642d => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x426437; continue; }
  pc = 0x426436; continue;
 }
 0x426436 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x426437 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426463; continue; }
  pc = 0x426440; continue;
 }
 0x426440 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59442fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594431u32, v); }
  pc = 0x426463; continue;
 }
 0x426463 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426464); }
 let mut pc: u32 = 0x426464;
 loop { match pc {
 0x426464 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42646e; continue; }
  pc = 0x42646d; continue;
 }
 0x42646d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42646e => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42649a; continue; }
  pc = 0x426477; continue;
 }
 0x426477 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594431u32, v); }
  pc = 0x42649a; continue;
 }
 0x42649a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042649b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42649b); }
 let mut pc: u32 = 0x42649b;
 loop { match pc {
 0x42649b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4264a5; continue; }
  pc = 0x4264a4; continue;
 }
 0x4264a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4264a5 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4264d1; continue; }
  pc = 0x4264ae; continue;
 }
 0x4264ae => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59442fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x594431u32, v); }
  pc = 0x4264d1; continue;
 }
 0x4264d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004264d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4264d2); }
 let mut pc: u32 = 0x4264d2;
 loop { match pc {
 0x4264d2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4264dc; continue; }
  pc = 0x4264db; continue;
 }
 0x4264db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4264dc => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426548; continue; }
  pc = 0x4264e7; continue;
 }
 0x4264e7 => {
  cpu.push(0x4264ec); f_004010e0(cpu); if cpu.returned_to != 0x4264ec { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4264f8); f_00401130(cpu); if cpu.returned_to != 0x4264f8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426548; continue; }
  pc = 0x42650b; continue;
 }
 0x42650b => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426517); f_004030c0(cpu); if cpu.returned_to != 0x426517 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426526); f_00401130(cpu); if cpu.returned_to != 0x426526 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426548; continue; }
  pc = 0x426539; continue;
 }
 0x426539 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426545); f_004030c0(cpu); if cpu.returned_to != 0x426545 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426548; continue;
 }
 0x426548 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42656f; continue; }
  pc = 0x426552; continue;
 }
 0x426552 => {
  { let v: u8 = 0x3u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x594435u32)); cpu.w8(0x594435u32, v); }
  { cpu.sub8(cpu.r8(0x594435u32), 0x1u8); }
  if !cpu.zf { pc = 0x42656f; continue; }
  pc = 0x426568; continue;
 }
 0x426568 => {
  { let v: u8 = 0x4u8; cpu.w8(0x594435u32, v); }
  pc = 0x42656f; continue;
 }
 0x42656f => {
  cpu.push(0x426574); f_00425eb2(cpu); if cpu.returned_to != 0x426574 { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x426587; continue; }
  pc = 0x426578; continue;
 }
 0x426578 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x594410u32)); cpu.w8(0x594410u32, v); }
  { let v: u8 = cpu.r8(0x594410u32) & 0x3fu8; cpu.w8(0x594410u32, v); }
  cpu.poll(); pc = 0x42656f; continue;
 }
 0x426587 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x426578; continue; }
  pc = 0x426595; continue;
 }
 0x426595 => {
  { let v: u8 = cpu.r8(0x594435u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042659b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42659b); }
 let mut pc: u32 = 0x42659b;
 loop { match pc {
 0x42659b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4265a5; continue; }
  pc = 0x4265a4; continue;
 }
 0x4265a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4265a5 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426611; continue; }
  pc = 0x4265b0; continue;
 }
 0x4265b0 => {
  cpu.push(0x4265b5); f_004010e0(cpu); if cpu.returned_to != 0x4265b5 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4265c1); f_00401130(cpu); if cpu.returned_to != 0x4265c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426611; continue; }
  pc = 0x4265d4; continue;
 }
 0x4265d4 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4265e0); f_004030c0(cpu); if cpu.returned_to != 0x4265e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4265ef); f_00401130(cpu); if cpu.returned_to != 0x4265ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426611; continue; }
  pc = 0x426602; continue;
 }
 0x426602 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42660e); f_004030c0(cpu); if cpu.returned_to != 0x42660e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426611; continue;
 }
 0x426611 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426638; continue; }
  pc = 0x42661b; continue;
 }
 0x42661b => {
  { let v: u8 = 0x2u8; cpu.w8(0x594430u32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x594435u32)); cpu.w8(0x594435u32, v); }
  { cpu.sub8(cpu.r8(0x594435u32), 0x5u8); }
  if !cpu.zf { pc = 0x426638; continue; }
  pc = 0x426631; continue;
 }
 0x426631 => {
  { let v: u8 = 0x2u8; cpu.w8(0x594435u32, v); }
  pc = 0x426638; continue;
 }
 0x426638 => {
  cpu.push(0x42663d); f_00425eb2(cpu); if cpu.returned_to != 0x42663d { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x426650; continue; }
  pc = 0x426641; continue;
 }
 0x426641 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x594410u32)); cpu.w8(0x594410u32, v); }
  { let v: u8 = cpu.r8(0x594410u32) & 0x3fu8; cpu.w8(0x594410u32, v); }
  cpu.poll(); pc = 0x426638; continue;
 }
 0x426650 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x426641; continue; }
  pc = 0x42665e; continue;
 }
 0x42665e => {
  { let v: u8 = cpu.r8(0x594435u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426664(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426664); }
 let mut pc: u32 = 0x426664;
 loop { match pc {
 0x426664 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42666e; continue; }
  pc = 0x42666d; continue;
 }
 0x42666d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42666e => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4266da; continue; }
  pc = 0x426679; continue;
 }
 0x426679 => {
  cpu.push(0x42667e); f_004010e0(cpu); if cpu.returned_to != 0x42667e { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x15u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42668a); f_00401130(cpu); if cpu.returned_to != 0x42668a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4266da; continue; }
  pc = 0x42669d; continue;
 }
 0x42669d => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4266a9); f_004030c0(cpu); if cpu.returned_to != 0x4266a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4266b8); f_00401130(cpu); if cpu.returned_to != 0x4266b8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4266da; continue; }
  pc = 0x4266cb; continue;
 }
 0x4266cb => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4266d7); f_004030c0(cpu); if cpu.returned_to != 0x4266d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4266da; continue;
 }
 0x4266da => {
  { cpu.popad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x59443eu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x59440eu32)); cpu.w16(0x59440eu32, v); }
  { let v: u16 = cpu.and16(cpu.r16(0x59440eu32), 0xfu16); cpu.w16(0x59440eu32, v); }
  { let v: u16 = cpu.r16(0x59440eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004266f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4266f9); }
 let mut pc: u32 = 0x4266f9;
 loop { match pc {
 0x4266f9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x426703; continue; }
  pc = 0x426702; continue;
 }
 0x426702 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x426703 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x42676f; continue; }
  pc = 0x42670e; continue;
 }
 0x42670e => {
  cpu.push(0x426713); f_004010e0(cpu); if cpu.returned_to != 0x426713 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x15u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42671f); f_00401130(cpu); if cpu.returned_to != 0x42671f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42676f; continue; }
  pc = 0x426732; continue;
 }
 0x426732 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42673e); f_004030c0(cpu); if cpu.returned_to != 0x42673e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42674d); f_00401130(cpu); if cpu.returned_to != 0x42674d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42676f; continue; }
  pc = 0x426760; continue;
 }
 0x426760 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42676c); f_004030c0(cpu); if cpu.returned_to != 0x42676c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x42676f; continue;
 }
 0x42676f => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x59443eu32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x59440eu32)); cpu.w16(0x59440eu32, v); }
  { let v: u16 = cpu.and16(cpu.r16(0x59440eu32), 0xfu16); cpu.w16(0x59440eu32, v); }
  { let v: u16 = cpu.r16(0x59440eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004267c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4267c2); }
 let mut pc: u32 = 0x4267c2;
 loop { match pc {
 0x4267c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59443eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004267d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4267d2); } r_004267d2(cpu, 0x4267d2); }
pub fn f_00426800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x426800); } r_004267d2(cpu, 0x426800); }
fn r_004267d2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4267d2 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42687b; continue; }
  pc = 0x4267df; continue;
 }
 0x4267df => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x42684b; continue; }
  pc = 0x4267ea; continue;
 }
 0x4267ea => {
  cpu.push(0x4267ef); f_004010e0(cpu); if cpu.returned_to != 0x4267ef { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4267fb); f_00401130(cpu); if cpu.returned_to != 0x4267fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42684b; continue; }
  pc = 0x42680e; continue;
 }
 0x426800 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x61u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42684b; continue; }
  pc = 0x42680e; continue;
 }
 0x42680e => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42681a); f_004030c0(cpu); if cpu.returned_to != 0x42681a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426829); f_00401130(cpu); if cpu.returned_to != 0x426829 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42684b; continue; }
  pc = 0x42683c; continue;
 }
 0x42683c => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426848); f_004030c0(cpu); if cpu.returned_to != 0x426848 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x42684b; continue;
 }
 0x42684b => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x5943d6u32, v); }
  pc = 0x426853; continue;
 }
 0x426853 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594437u32)); cpu.w8(0x594437u32, v); }
  { cpu.sub8(cpu.r8(0x594437u32), 0x10u8); }
  if !cpu.zf { pc = 0x426869; continue; }
  pc = 0x426862; continue;
 }
 0x426862 => {
  { let v: u8 = 0x0u8; cpu.w8(0x594437u32, v); }
  pc = 0x426869; continue;
 }
 0x426869 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x594437u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d3f7u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x42687b; continue; }
  pc = 0x426879; continue;
 }
 0x426879 => {
  cpu.poll(); pc = 0x426853; continue;
 }
 0x42687b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042687c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42687c); }
 let mut pc: u32 = 0x42687c;
 loop { match pc {
 0x42687c => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426925; continue; }
  pc = 0x426889; continue;
 }
 0x426889 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4268f5; continue; }
  pc = 0x426894; continue;
 }
 0x426894 => {
  cpu.push(0x426899); f_004010e0(cpu); if cpu.returned_to != 0x426899 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4268a5); f_00401130(cpu); if cpu.returned_to != 0x4268a5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4268f5; continue; }
  pc = 0x4268b8; continue;
 }
 0x4268b8 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4268c4); f_004030c0(cpu); if cpu.returned_to != 0x4268c4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4268d3); f_00401130(cpu); if cpu.returned_to != 0x4268d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4268f5; continue; }
  pc = 0x4268e6; continue;
 }
 0x4268e6 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4268f2); f_004030c0(cpu); if cpu.returned_to != 0x4268f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4268f5; continue;
 }
 0x4268f5 => {
  { cpu.popad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x5943d6u32, v); }
  pc = 0x4268fd; continue;
 }
 0x4268fd => {
  { let v: u8 = cpu.dec8(cpu.r8(0x594437u32)); cpu.w8(0x594437u32, v); }
  { cpu.sub8(cpu.r8(0x594437u32), 0x0u8); }
  if !cpu.sf { pc = 0x426913; continue; }
  pc = 0x42690c; continue;
 }
 0x42690c => {
  { let v: u8 = 0xfu8; cpu.w8(0x594437u32, v); }
  pc = 0x426913; continue;
 }
 0x426913 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x594437u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d3f7u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x426925; continue; }
  pc = 0x426923; continue;
 }
 0x426923 => {
  cpu.poll(); pc = 0x4268fd; continue;
 }
 0x426925 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426926); }
 let mut pc: u32 = 0x426926;
 loop { match pc {
 0x426926 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4269d6; continue; }
  pc = 0x426933; continue;
 }
 0x426933 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x42699f; continue; }
  pc = 0x42693e; continue;
 }
 0x42693e => {
  cpu.push(0x426943); f_004010e0(cpu); if cpu.returned_to != 0x426943 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42694f); f_00401130(cpu); if cpu.returned_to != 0x42694f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42699f; continue; }
  pc = 0x426962; continue;
 }
 0x426962 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42696e); f_004030c0(cpu); if cpu.returned_to != 0x42696e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42697d); f_00401130(cpu); if cpu.returned_to != 0x42697d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42699f; continue; }
  pc = 0x426990; continue;
 }
 0x426990 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42699c); f_004030c0(cpu); if cpu.returned_to != 0x42699c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x42699f; continue;
 }
 0x42699f => {
  { cpu.popad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x5943d5u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x4269ae; continue;
 }
 0x4269ae => {
  { let v: u8 = cpu.r8(0x4d6920u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4d6920u32, v); }
  { let v: u8 = cpu.r8(0x576501u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594437u32, v); }
  cpu.push(0x4269cf); f_00422240(cpu); if cpu.returned_to != 0x4269cf { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x4269ae; continue; }
  pc = 0x4269d1; continue;
 }
 0x4269d1 => {
  cpu.push(0x4269d6); f_0042223f(cpu); if cpu.returned_to != 0x4269d6 { return; }
  pc = 0x4269d6; continue;
 }
 0x4269d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004269d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4269d7); }
 let mut pc: u32 = 0x4269d7;
 loop { match pc {
 0x4269d7 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426a88; continue; }
  pc = 0x4269e4; continue;
 }
 0x4269e4 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426a50; continue; }
  pc = 0x4269ef; continue;
 }
 0x4269ef => {
  cpu.push(0x4269f4); f_004010e0(cpu); if cpu.returned_to != 0x4269f4 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426a00); f_00401130(cpu); if cpu.returned_to != 0x426a00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426a50; continue; }
  pc = 0x426a13; continue;
 }
 0x426a13 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426a1f); f_004030c0(cpu); if cpu.returned_to != 0x426a1f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426a2e); f_00401130(cpu); if cpu.returned_to != 0x426a2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426a50; continue; }
  pc = 0x426a41; continue;
 }
 0x426a41 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426a4d); f_004030c0(cpu); if cpu.returned_to != 0x426a4d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426a50; continue;
 }
 0x426a50 => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x5943d7u32, v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x426a5d; continue;
 }
 0x426a5d => {
  { let v: u16 = cpu.r16(0x4d6842u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x426a72; continue; }
  pc = 0x426a6a; continue;
 }
 0x426a6a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x426a5d; continue; }
  pc = 0x426a70; continue;
 }
 0x426a70 => {
  pc = 0x426a88; continue;
 }
 0x426a72 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x426a88; continue;
 }
 0x426a88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426a89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426a89); }
 let mut pc: u32 = 0x426a89;
 loop { match pc {
 0x426a89 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426b3a; continue; }
  pc = 0x426a96; continue;
 }
 0x426a96 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5943d7u32, v); }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426b09; continue; }
  pc = 0x426aa8; continue;
 }
 0x426aa8 => {
  cpu.push(0x426aad); f_004010e0(cpu); if cpu.returned_to != 0x426aad { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426ab9); f_00401130(cpu); if cpu.returned_to != 0x426ab9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426b09; continue; }
  pc = 0x426acc; continue;
 }
 0x426acc => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426ad8); f_004030c0(cpu); if cpu.returned_to != 0x426ad8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426ae7); f_00401130(cpu); if cpu.returned_to != 0x426ae7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426b09; continue; }
  pc = 0x426afa; continue;
 }
 0x426afa => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426b06); f_004030c0(cpu); if cpu.returned_to != 0x426b06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426b09; continue;
 }
 0x426b09 => {
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x426b0f; continue;
 }
 0x426b0f => {
  { let v: u16 = cpu.r16(0x4d6842u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x426b24; continue; }
  pc = 0x426b1c; continue;
 }
 0x426b1c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x426b0f; continue; }
  pc = 0x426b22; continue;
 }
 0x426b22 => {
  pc = 0x426b3a; continue;
 }
 0x426b24 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x426b3a; continue;
 }
 0x426b3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426b3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426b3b); }
 let mut pc: u32 = 0x426b3b;
 loop { match pc {
 0x426b3b => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426bc9; continue; }
  pc = 0x426b48; continue;
 }
 0x426b48 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426bb4; continue; }
  pc = 0x426b53; continue;
 }
 0x426b53 => {
  cpu.push(0x426b58); f_004010e0(cpu); if cpu.returned_to != 0x426b58 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426b64); f_00401130(cpu); if cpu.returned_to != 0x426b64 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426bb4; continue; }
  pc = 0x426b77; continue;
 }
 0x426b77 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426b83); f_004030c0(cpu); if cpu.returned_to != 0x426b83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426b92); f_00401130(cpu); if cpu.returned_to != 0x426b92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426bb4; continue; }
  pc = 0x426ba5; continue;
 }
 0x426ba5 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426bb1); f_004030c0(cpu); if cpu.returned_to != 0x426bb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426bb4; continue;
 }
 0x426bb4 => {
  { cpu.popad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x5943d4u32, v); }
  { let v: u8 = cpu.r8(0x590817u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x426bc9; continue; }
  pc = 0x426bc7; continue;
 }
 0x426bc7 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x426bc9; continue;
 }
 0x426bc9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x590817u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426bcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426bcf); }
 let mut pc: u32 = 0x426bcf;
 loop { match pc {
 0x426bcf => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426c60; continue; }
  pc = 0x426bdc; continue;
 }
 0x426bdc => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426c48; continue; }
  pc = 0x426be7; continue;
 }
 0x426be7 => {
  cpu.push(0x426bec); f_004010e0(cpu); if cpu.returned_to != 0x426bec { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426bf8); f_00401130(cpu); if cpu.returned_to != 0x426bf8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426c48; continue; }
  pc = 0x426c0b; continue;
 }
 0x426c0b => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426c17); f_004030c0(cpu); if cpu.returned_to != 0x426c17 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426c26); f_00401130(cpu); if cpu.returned_to != 0x426c26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426c48; continue; }
  pc = 0x426c39; continue;
 }
 0x426c39 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426c45); f_004030c0(cpu); if cpu.returned_to != 0x426c45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426c48; continue;
 }
 0x426c48 => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x5943d4u32, v); }
  { let v: u8 = cpu.r8(0x590817u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x426c60; continue; }
  pc = 0x426c5b; continue;
 }
 0x426c5b => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  pc = 0x426c60; continue;
 }
 0x426c60 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x590817u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426c66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426c66); }
 let mut pc: u32 = 0x426c66;
 loop { match pc {
 0x426c66 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426c7d; continue; }
  pc = 0x426c6f; continue;
 }
 0x426c6f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d4u32, v); }
  pc = 0x426c7d; continue;
 }
 0x426c7d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426c7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426c7e); }
 let mut pc: u32 = 0x426c7e;
 loop { match pc {
 0x426c7e => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426c95; continue; }
  pc = 0x426c87; continue;
 }
 0x426c87 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d5u32, v); }
  pc = 0x426c95; continue;
 }
 0x426c95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426c96); }
 let mut pc: u32 = 0x426c96;
 loop { match pc {
 0x426c96 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426cad; continue; }
  pc = 0x426c9f; continue;
 }
 0x426c9f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d7u32, v); }
  pc = 0x426cad; continue;
 }
 0x426cad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426cae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426cae); }
 let mut pc: u32 = 0x426cae;
 loop { match pc {
 0x426cae => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426cc5; continue; }
  pc = 0x426cb7; continue;
 }
 0x426cb7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d6u32, v); }
  pc = 0x426cc5; continue;
 }
 0x426cc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426cc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426cc6); }
 let mut pc: u32 = 0x426cc6;
 loop { match pc {
 0x426cc6 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426cdd; continue; }
  pc = 0x426ccf; continue;
 }
 0x426ccf => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d9u32, v); }
  pc = 0x426cdd; continue;
 }
 0x426cdd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426cde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426cde); }
 let mut pc: u32 = 0x426cde;
 loop { match pc {
 0x426cde => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426cf5; continue; }
  pc = 0x426ce7; continue;
 }
 0x426ce7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943d8u32, v); }
  pc = 0x426cf5; continue;
 }
 0x426cf5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426cf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426cf6); }
 let mut pc: u32 = 0x426cf6;
 loop { match pc {
 0x426cf6 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426d0f; continue; }
  pc = 0x426cff; continue;
 }
 0x426cff => {
  { cpu.sub8(cpu.r8(0x5943dau32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x426d0f; continue; }
  pc = 0x426d08; continue;
 }
 0x426d08 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5943dau32, v); }
  pc = 0x426d0f; continue;
 }
 0x426d0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426d10); }
 let mut pc: u32 = 0x426d10;
 loop { match pc {
 0x426d10 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426d29; continue; }
  pc = 0x426d19; continue;
 }
 0x426d19 => {
  { cpu.sub8(cpu.r8(0x5943dau32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x426d29; continue; }
  pc = 0x426d22; continue;
 }
 0x426d22 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5943dau32, v); }
  pc = 0x426d29; continue;
 }
 0x426d29 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426d5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426d5e); }
 let mut pc: u32 = 0x426d5e;
 loop { match pc {
 0x426d5e => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426de2; continue; }
  pc = 0x426d67; continue;
 }
 0x426d67 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426dd3; continue; }
  pc = 0x426d72; continue;
 }
 0x426d72 => {
  cpu.push(0x426d77); f_004010e0(cpu); if cpu.returned_to != 0x426d77 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426d83); f_00401130(cpu); if cpu.returned_to != 0x426d83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426dd3; continue; }
  pc = 0x426d96; continue;
 }
 0x426d96 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426da2); f_004030c0(cpu); if cpu.returned_to != 0x426da2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426db1); f_00401130(cpu); if cpu.returned_to != 0x426db1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426dd3; continue; }
  pc = 0x426dc4; continue;
 }
 0x426dc4 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426dd0); f_004030c0(cpu); if cpu.returned_to != 0x426dd0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426dd3; continue;
 }
 0x426dd3 => {
  { cpu.popad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x5943d8u32, v); }
  { let v: u8 = cpu.xor8(cpu.r8(0x59805du32), 0x1u8); cpu.w8(0x59805du32, v); }
  pc = 0x426de2; continue;
 }
 0x426de2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426de3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426de3); }
 let mut pc: u32 = 0x426de3;
 loop { match pc {
 0x426de3 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426e67; continue; }
  pc = 0x426dec; continue;
 }
 0x426dec => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x426e58; continue; }
  pc = 0x426df7; continue;
 }
 0x426df7 => {
  cpu.push(0x426dfc); f_004010e0(cpu); if cpu.returned_to != 0x426dfc { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426e08); f_00401130(cpu); if cpu.returned_to != 0x426e08 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426e58; continue; }
  pc = 0x426e1b; continue;
 }
 0x426e1b => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426e27); f_004030c0(cpu); if cpu.returned_to != 0x426e27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426e36); f_00401130(cpu); if cpu.returned_to != 0x426e36 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x426e58; continue; }
  pc = 0x426e49; continue;
 }
 0x426e49 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x426e55); f_004030c0(cpu); if cpu.returned_to != 0x426e55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x426e58; continue;
 }
 0x426e58 => {
  { cpu.popad(); }
  { let v: u8 = 0x2u8; cpu.w8(0x5943d8u32, v); }
  { let v: u8 = cpu.xor8(cpu.r8(0x59805du32), 0x1u8); cpu.w8(0x59805du32, v); }
  pc = 0x426e67; continue;
 }
 0x426e67 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00426e78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x426e78); }
 let mut pc: u32 = 0x426e78;
 loop { match pc {
 0x426e78 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x426eee; continue; }
  pc = 0x426e81; continue;
 }
 0x426e81 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58212bu32, v); }
  { let v: u32 = 0x56d256u32; cpu.w32(0x56d423u32, v); }
  { let v: u32 = 0x4e1fd2u32; cpu.w32(0x56d427u32, v); }
  cpu.push(0x426ea1); f_0041c162(cpu); if cpu.returned_to != 0x426ea1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x594456u32, v); }
  { let v: u32 = 0x4e1fd2u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x59445du32, v); }
  pc = 0x426eb8; continue;
 }
 0x426eb8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x426ec7; continue; }
  pc = 0x426ebe; continue;
 }
 0x426ebe => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594456u32)); cpu.w8(0x594456u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x426eb8; continue; }
  pc = 0x426ec7; continue;
 }
 0x426ec7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x594457u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x426ee8); f_00402d70(cpu); if cpu.returned_to != 0x426ee8 { return; }
  cpu.push(0x426eed); f_00405320(cpu); if cpu.returned_to != 0x426eed { return; }
  { cpu.popad(); }
  pc = 0x426eee; continue;
 }
 0x426eee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004270e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4270e9); }
 let mut pc: u32 = 0x4270e9;
 loop { match pc {
 0x4270e9 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x427100; continue; }
  pc = 0x4270f2; continue;
 }
 0x4270f2 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943dbu32, v); }
  pc = 0x427100; continue;
 }
 0x427100 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427101(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427101); }
 let mut pc: u32 = 0x427101;
 loop { match pc {
 0x427101 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x427118; continue; }
  pc = 0x42710a; continue;
 }
 0x42710a => {
  { let v: u8 = 0x2u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943dcu32, v); }
  pc = 0x427118; continue;
 }
 0x427118 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427119(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427119); }
 let mut pc: u32 = 0x427119;
 loop { match pc {
 0x427119 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x427130; continue; }
  pc = 0x427122; continue;
 }
 0x427122 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943ddu32, v); }
  pc = 0x427130; continue;
 }
 0x427130 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427131(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427131); }
 let mut pc: u32 = 0x427131;
 loop { match pc {
 0x427131 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x427148; continue; }
  pc = 0x42713a; continue;
 }
 0x42713a => {
  { let v: u8 = 0x2u8; cpu.w8(0x5978c9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5943deu32, v); }
  pc = 0x427148; continue;
 }
 0x427148 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427149(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427149); }
 let mut pc: u32 = 0x427149;
 loop { match pc {
 0x427149 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d2b1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427151); }
 let mut pc: u32 = 0x427151;
 loop { match pc {
 0x427151 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42715b; continue; }
  pc = 0x42715a; continue;
 }
 0x42715a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42715b => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x427247; continue; }
  pc = 0x427168; continue;
 }
 0x427168 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x576c04u32), 0x0u16); }
  if cpu.zf { pc = 0x4271d4; continue; }
  pc = 0x427173; continue;
 }
 0x427173 => {
  cpu.push(0x427178); f_004010e0(cpu); if cpu.returned_to != 0x427178 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x427184); f_00401130(cpu); if cpu.returned_to != 0x427184 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4271d4; continue; }
  pc = 0x427197; continue;
 }
 0x427197 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4271a3); f_004030c0(cpu); if cpu.returned_to != 0x4271a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4271b2); f_00401130(cpu); if cpu.returned_to != 0x4271b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598058u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x598058u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4271d4; continue; }
  pc = 0x4271c5; continue;
 }
 0x4271c5 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4271d1); f_004030c0(cpu); if cpu.returned_to != 0x4271d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4271d4; continue;
 }
 0x4271d4 => {
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x58212bu32, v); }
  cpu.push(0x4271e1); f_0042649b(cpu); if cpu.returned_to != 0x4271e1 { return; }
  { let v: u32 = 0x56d23du32; cpu.w32(0x56d423u32, v); }
  { let v: u32 = 0x5943dfu32; cpu.w32(0x56d427u32, v); }
  cpu.push(0x4271fa); f_0041c162(cpu); if cpu.returned_to != 0x4271fa { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x594456u32, v); }
  { let v: u32 = 0x5943dfu32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x59445du32, v); }
  pc = 0x427211; continue;
 }
 0x427211 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x427220; continue; }
  pc = 0x427217; continue;
 }
 0x427217 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594456u32)); cpu.w8(0x594456u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x427211; continue; }
  pc = 0x427220; continue;
 }
 0x427220 => {
  { let v: u8 = 0x1u8; cpu.w8(0x594454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x594457u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x427241); f_00402d70(cpu); if cpu.returned_to != 0x427241 { return; }
  cpu.push(0x427246); f_00405320(cpu); if cpu.returned_to != 0x427246 { return; }
  { cpu.popad(); }
  pc = 0x427247; continue;
 }
 0x427247 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427248(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427248); }
 let mut pc: u32 = 0x427248;
 loop { match pc {
 0x427248 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4272c3; continue; }
  pc = 0x427251; continue;
 }
 0x427251 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58212bu32, v); }
  cpu.push(0x42725d); f_004270e9(cpu); if cpu.returned_to != 0x42725d { return; }
  { let v: u32 = 0x56d26cu32; cpu.w32(0x56d423u32, v); }
  { let v: u32 = 0x4ea80fu32; cpu.w32(0x56d427u32, v); }
  cpu.push(0x427276); f_0041c162(cpu); if cpu.returned_to != 0x427276 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x594456u32, v); }
  { let v: u32 = 0x4ea80fu32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x59445du32, v); }
  pc = 0x42728d; continue;
 }
 0x42728d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x42729c; continue; }
  pc = 0x427293; continue;
 }
 0x427293 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594456u32)); cpu.w8(0x594456u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42728d; continue; }
  pc = 0x42729c; continue;
 }
 0x42729c => {
  { let v: u8 = 0x1u8; cpu.w8(0x594454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x594457u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4272bd); f_00402d70(cpu); if cpu.returned_to != 0x4272bd { return; }
  cpu.push(0x4272c2); f_00405320(cpu); if cpu.returned_to != 0x4272c2 { return; }
  { cpu.popad(); }
  pc = 0x4272c3; continue;
 }
 0x4272c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004272c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4272c4); }
 let mut pc: u32 = 0x4272c4;
 loop { match pc {
 0x4272c4 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x42733f; continue; }
  pc = 0x4272cd; continue;
 }
 0x4272cd => {
  { let v: u8 = 0x0u8; cpu.w8(0x58212bu32, v); }
  cpu.push(0x4272d9); f_00427101(cpu); if cpu.returned_to != 0x4272d9 { return; }
  { let v: u32 = 0x56d28fu32; cpu.w32(0x56d423u32, v); }
  { let v: u32 = 0x4eaf70u32; cpu.w32(0x56d427u32, v); }
  cpu.push(0x4272f2); f_0041c162(cpu); if cpu.returned_to != 0x4272f2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x594456u32, v); }
  { let v: u32 = 0x4eaf70u32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x59445du32, v); }
  pc = 0x427309; continue;
 }
 0x427309 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x427318; continue; }
  pc = 0x42730f; continue;
 }
 0x42730f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594456u32)); cpu.w8(0x594456u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x427309; continue; }
  pc = 0x427318; continue;
 }
 0x427318 => {
  { let v: u8 = 0x2u8; cpu.w8(0x594454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x594457u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x427339); f_00402d70(cpu); if cpu.returned_to != 0x427339 { return; }
  cpu.push(0x42733e); f_00405320(cpu); if cpu.returned_to != 0x42733e { return; }
  { cpu.popad(); }
  pc = 0x42733f; continue;
 }
 0x42733f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427340); }
 let mut pc: u32 = 0x427340;
 loop { match pc {
 0x427340 => {
  { cpu.sub8(cpu.r8(0x594454u32), 0x0u8); }
  if !cpu.zf { pc = 0x4273bb; continue; }
  pc = 0x427349; continue;
 }
 0x427349 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58212bu32, v); }
  cpu.push(0x427355); f_00427119(cpu); if cpu.returned_to != 0x427355 { return; }
  { let v: u32 = 0x56d2a0u32; cpu.w32(0x56d423u32, v); }
  { let v: u32 = 0x590879u32; cpu.w32(0x56d427u32, v); }
  cpu.push(0x42736e); f_0041c162(cpu); if cpu.returned_to != 0x42736e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x594456u32, v); }
  { let v: u32 = 0x590879u32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x59445du32, v); }
  pc = 0x427385; continue;
 }
 0x427385 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x427394; continue; }
  pc = 0x42738b; continue;
 }
 0x42738b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x594456u32)); cpu.w8(0x594456u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x427385; continue; }
  pc = 0x427394; continue;
 }
 0x427394 => {
  { let v: u8 = 0x3u8; cpu.w8(0x594454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x594457u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4273b5); f_00402d70(cpu); if cpu.returned_to != 0x4273b5 { return; }
  cpu.push(0x4273ba); f_00405320(cpu); if cpu.returned_to != 0x4273ba { return; }
  { cpu.popad(); }
  pc = 0x4273bb; continue;
 }
 0x4273bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004273bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4273bc); }
 let mut pc: u32 = 0x4273bc;
 loop { match pc {
 0x4273bc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004275fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4275fd); }
 let mut pc: u32 = 0x4275fd;
 loop { match pc {
 0x4275fd => {
  { let v: u32 = 0x10d0u32; cpu.ebx = v; }
  cpu.push(0x427607); f_0041b922(cpu); if cpu.returned_to != 0x427607 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6au32, v); }
  { cpu.sub8(cpu.r8(0x5824d5u32), 0x0u8); }
  if cpu.zf { pc = 0x427630; continue; }
  pc = 0x427617; continue;
 }
 0x427617 => {
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  cpu.push(0x427628); f_00447961(cpu); if cpu.returned_to != 0x427628 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x427649; continue; }
  pc = 0x42762b; continue;
 }
 0x42762b => {
  f_0041c653(cpu); return;
 }
 0x427630 => {
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x427641); f_00447961(cpu); if cpu.returned_to != 0x427641 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x427649; continue; }
  pc = 0x427644; continue;
 }
 0x427644 => {
  f_0041c653(cpu); return;
 }
 0x427649 => {
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.esi = v; }
  { let v: u32 = 0x55b9b8u32; cpu.edi = v; }
  cpu.push(0x427659); f_0046c4e8(cpu); if cpu.returned_to != 0x427659 { return; }
  if !cpu.cf { pc = 0x427660; continue; }
  pc = 0x42765b; continue;
 }
 0x42765b => {
  f_0041c653(cpu); return;
 }
 0x427660 => {
  cpu.push(0x427665); f_0041b99b(cpu); if cpu.returned_to != 0x427665 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427666(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427666); }
 let mut pc: u32 = 0x427666;
 loop { match pc {
 0x427666 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42767f; continue;
 }
 0x42767f => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42768f; continue; }
  pc = 0x42768a; continue;
 }
 0x42768a => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x37u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x427692; continue;
 }
 0x42768f => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x427692; continue;
 }
 0x427692 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42767f; continue; }
  pc = 0x427697; continue;
 }
 0x427697 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427826); }
 let mut pc: u32 = 0x427826;
 loop { match pc {
 0x427826 => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x590af8u32; cpu.edi = v; }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x427837); f_0041494d(cpu); if cpu.returned_to != 0x427837 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427838); }
 let mut pc: u32 = 0x427838;
 loop { match pc {
 0x427838 => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x590af8u32; cpu.edi = v; }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x427849); f_0041494d(cpu); if cpu.returned_to != 0x427849 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x56d40fu32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x427864; continue; }
  pc = 0x42785a; continue;
 }
 0x42785a => {
  { let v: u32 = 0x590af8u32; cpu.eax = v; }
  cpu.push(0x427864); f_004278a3(cpu); if cpu.returned_to != 0x427864 { return; }
  pc = 0x427864; continue;
 }
 0x427864 => {
  { let v: u32 = cpu.r32(0x56d413u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x427878; continue; }
  pc = 0x42786e; continue;
 }
 0x42786e => {
  { let v: u32 = 0x591078u32; cpu.eax = v; }
  cpu.push(0x427878); f_004278a3(cpu); if cpu.returned_to != 0x427878 { return; }
  pc = 0x427878; continue;
 }
 0x427878 => {
  { let v: u32 = cpu.r32(0x56d417u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x42788c; continue; }
  pc = 0x427882; continue;
 }
 0x427882 => {
  { let v: u32 = 0x5915f8u32; cpu.eax = v; }
  cpu.push(0x42788c); f_004278a3(cpu); if cpu.returned_to != 0x42788c { return; }
  pc = 0x42788c; continue;
 }
 0x42788c => {
  { let v: u32 = cpu.r32(0x56d41bu32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x4278a0; continue; }
  pc = 0x427896; continue;
 }
 0x427896 => {
  { let v: u32 = 0x591b78u32; cpu.eax = v; }
  cpu.push(0x4278a0); f_004278a3(cpu); if cpu.returned_to != 0x4278a0 { return; }
  pc = 0x4278a0; continue;
 }
 0x4278a0 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004278a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4278a3); }
 let mut pc: u32 = 0x4278a3;
 loop { match pc {
 0x4278a3 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  pc = 0x4278a7; continue;
 }
 0x4278a7 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x4278b5; continue; }
  pc = 0x4278ab; continue;
 }
 0x4278ab => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi & cpu.ebp; cpu.edi = v; }
  cpu.poll(); pc = 0x4278a7; continue;
 }
 0x4278b5 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042798a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42798a); }
 let mut pc: u32 = 0x42798a;
 loop { match pc {
 0x42798a => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x4e1584u32, v); }
  { let v: u32 = cpu.r32(0x575d16u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575d45u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1559u32, v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b90u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427d93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427d93); }
 let mut pc: u32 = 0x427d93;
 loop { match pc {
 0x427d93 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x427da5; continue; }
  pc = 0x427d9c; continue;
 }
 0x427d9c => {
  { cpu.sub8(cpu.r8(0x4d67f6u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x427df3; continue; }
  pc = 0x427da5; continue;
 }
 0x427da5 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x427dcc; continue; }
  pc = 0x427dbb; continue;
 }
 0x427dbb => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x427dcc; continue;
 }
 0x427dcc => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x427ddd; continue; }
  pc = 0x427dd5; continue;
 }
 0x427dd5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x427ddd; continue;
 }
 0x427ddd => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6a00u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6a04u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x427df3 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6794u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6792u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x4e1559u32), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x427dcc; continue; }
  pc = 0x427e13; continue;
 }
 0x427e13 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x427dcc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427e19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427e19); }
 let mut pc: u32 = 0x427e19;
 loop { match pc {
 0x427e19 => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x4e158cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1584u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x427e42; continue; }
  pc = 0x427e39; continue;
 }
 0x427e39 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1585u32, v); }
  pc = 0x427e49; continue;
 }
 0x427e42 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1585u32, v); }
  pc = 0x427e49; continue;
 }
 0x427e49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427e4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427e4a); }
 let mut pc: u32 = 0x427e4a;
 loop { match pc {
 0x427e4a => {
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x100u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x427e5b); f_0042122e(cpu); if cpu.returned_to != 0x427e5b { return; }
  cpu.push(0x427e60); f_00417c92(cpu); if cpu.returned_to != 0x427e60 { return; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x427e69); f_00417dab(cpu); if cpu.returned_to != 0x427e69 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00427f22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x427f22); }
 let mut pc: u32 = 0x427f22;
 loop { match pc {
 0x427f22 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7d1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042801c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42801c); } r_0042801c(cpu, 0x42801c); }
pub fn f_004280b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4280b4); } r_0042801c(cpu, 0x4280b4); }
fn r_0042801c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x42801c => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c701u32, v); }
  { let v: u8 = cpu.r8(0x58c702u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4280b6; continue; }
  pc = 0x428036; continue;
 }
 0x428036 => {
  { let v: u32 = 0x4e15c8u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c701u32, v); }
  { let v: u32 = 0x513fc7u32; cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x428062; continue; }
  pc = 0x42805d; continue;
 }
 0x42805d => {
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x428062; continue;
 }
 0x428062 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e15c8u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5u32); cpu.esi = v; }
  { let v: u32 = 0x58c703u32; cpu.ebp = v; }
  pc = 0x428072; continue;
 }
 0x428072 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x4280a4; continue;
 }
 0x428077 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4280a1; continue; }
  pc = 0x428081; continue;
 }
 0x428081 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c7c7u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  pc = 0x4280a1; continue;
 }
 0x4280a1 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  pc = 0x4280a4; continue;
 }
 0x4280a4 => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.sf { cpu.poll(); pc = 0x428077; continue; }
  pc = 0x4280a8; continue;
 }
 0x4280a8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x4280b6; continue; }
  pc = 0x4280af; continue;
 }
 0x4280af => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x30u32); cpu.ebp = v; }
  pc = 0x4280b4; continue;
 }
 0x4280b4 => {
  cpu.poll(); pc = 0x428072; continue;
 }
 0x4280b6 => {
  { let v: u32 = 0x4e15c8u32; cpu.esi = v; }
  { let v: u32 = 0x4e16b8u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4280c5; continue;
 }
 0x4280c5 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4280e6; continue; }
  pc = 0x4280ca; continue;
 }
 0x4280ca => {
  { let v: u8 = 0x52u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d3098u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4280dc); f_00428165(cpu); if cpu.returned_to != 0x4280dc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x4280e4); f_004280fe(cpu); if cpu.returned_to != 0x4280e4 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4280e6; continue;
 }
 0x4280e6 => {
  { cpu.stos(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x77u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xefu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x4280fb; continue; }
  pc = 0x4280f6; continue;
 }
 0x4280f6 => {
  cpu.push(0x4280fb); f_0042812a(cpu); if cpu.returned_to != 0x4280fb { return; }
  pc = 0x4280fb; continue;
 }
 0x4280fb => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4280c5; continue; }
  pc = 0x4280fd; continue;
 }
 0x4280fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004280fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4280fe); }
 let mut pc: u32 = 0x4280fe;
 loop { match pc {
 0x4280fe => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x4e16b8u32); cpu.edi = v; }
  { let v: u32 = 0x4e1988u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
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
pub fn f_0042812a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42812a); }
 let mut pc: u32 = 0x42812a;
 loop { match pc {
 0x42812a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x4e16b9u32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x5d3098u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x466cu32); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x42814e; continue;
 }
 0x42814e => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x428158; continue; }
  pc = 0x428156; continue;
 }
 0x428156 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x428158; continue;
 }
 0x428158 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42814e; continue; }
  pc = 0x42815a; continue;
 }
 0x42815a => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e1898u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428165(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428165); }
 let mut pc: u32 = 0x428165;
 loop { match pc {
 0x428165 => {
  { let v: u32 = cpu.r16(0x5d3098u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x466cu32); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4281c3; continue; }
  pc = 0x428185; continue;
 }
 0x428185 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x4281be; continue; }
  pc = 0x42818b; continue;
 }
 0x42818b => {
  { let v: u16 = 0x2cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.mul16(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4281be; continue; }
  pc = 0x4281a2; continue;
 }
 0x4281a2 => {
  { let v: u8 = 0x2au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x55f0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4281be; continue; }
  pc = 0x4281ba; continue;
 }
 0x4281ba => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x4281be; continue;
 }
 0x4281be => {
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x428185; continue; }
  pc = 0x4281c3; continue;
 }
 0x4281c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004281c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4281c4); }
 let mut pc: u32 = 0x4281c4;
 loop { match pc {
 0x4281c4 => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.edi = v; }
  { let v: u32 = 0x186au32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.ebx = v; }
  { let v: u32 = 0x4ea9acu32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x4281ee); f_00447961(cpu); if cpu.returned_to != 0x4281ee { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fbbu32, v); }
  cpu.push(0x4281fb); f_0047ac69(cpu); if cpu.returned_to != 0x4281fb { return; }
  cpu.push(0x428200); f_0044925a(cpu); if cpu.returned_to != 0x428200 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428201); }
 let mut pc: u32 = 0x428201;
 loop { match pc {
 0x428201 => {
  { let v: u32 = cpu.eax; cpu.w32(0x56c9bbu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x56c9bfu32, v); }
  { let v = cpu.r16(0x4d6b9cu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6ba4u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6ba0u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6ba8u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6b9cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x56c80fu32)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x140u16); }
  if (cpu.sf != cpu.of) { pc = 0x428240; continue; }
  pc = 0x42823c; continue;
 }
 0x42823c => {
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x428240; continue;
 }
 0x428240 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6ba0u32, v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x56c813u32)); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), 0xc8u16); }
  if (cpu.sf != cpu.of) { pc = 0x42825e; continue; }
  pc = 0x42825a; continue;
 }
 0x42825a => {
  { let v: u16 = 0xc8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42825e; continue;
 }
 0x42825e => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  pc = 0x428268; continue;
 }
 0x428268 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x5d00e5u32), 0x54u16); }
  if !cpu.zf { pc = 0x428284; continue; }
  pc = 0x428277; continue;
 }
 0x428277 => {
  cpu.push(0x42827c); f_0041deda(cpu); if cpu.returned_to != 0x42827c { return; }
  if cpu.cf { pc = 0x4283a3; continue; }
  pc = 0x428282; continue;
 }
 0x428282 => {
  pc = 0x42828f; continue;
 }
 0x428284 => {
  cpu.push(0x428289); f_0041e6a9(cpu); if cpu.returned_to != 0x428289 { return; }
  if cpu.cf { pc = 0x4283a3; continue; }
  pc = 0x42828f; continue;
 }
 0x42828f => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x428373; continue; }
  pc = 0x428297; continue;
 }
 0x428297 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x56c76bu32, v); }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x4282ac; continue; }
  pc = 0x4282a2; continue;
 }
 0x4282a2 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x428339; continue;
 }
 0x4282ac => {
  { let v: u8 = 0xffu8; cpu.w8(0x58213fu32, v); }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { pc = 0x4282ec; continue; }
  pc = 0x4282b8; continue;
 }
 0x4282b8 => {
  { cpu.pushad(); }
  { let v: u32 = 0x45au32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x5d00e5u32), 0x54u16); }
  if cpu.zf { pc = 0x4282cd; continue; }
  pc = 0x4282c8; continue;
 }
 0x4282c8 => {
  { let v: u32 = 0x44eu32; cpu.ecx = v; }
  pc = 0x4282cd; continue;
 }
 0x4282cd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4282d9); cpu.call(t); if cpu.returned_to != 0x4282d9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x58213fu32, v); }
  pc = 0x4282ec; continue;
 }
 0x4282ec => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if (cpu.sf != cpu.of) { pc = 0x428339; continue; }
  pc = 0x4282f1; continue;
 }
 0x4282f1 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42830a; continue; }
  pc = 0x4282f6; continue;
 }
 0x4282f6 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf == cpu.of) { pc = 0x428311; continue; }
  pc = 0x4282fb; continue;
 }
 0x4282fb => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42830a; continue; }
  pc = 0x428300; continue;
 }
 0x428300 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x21u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x33fu32; cpu.ebp = v; }
  pc = 0x428319; continue;
 }
 0x42830a => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x428339; continue;
 }
 0x428311 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x379u32; cpu.ebp = v; }
  pc = 0x428319; continue;
 }
 0x428319 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d00e5u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42832f); cpu.call(t); if cpu.returned_to != 0x42832f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x428339; continue;
 }
 0x428339 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x56c76bu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58213fu32), 0xffu8); }
  if cpu.zf { pc = 0x428350; continue; }
  pc = 0x42834a; continue;
 }
 0x42834a => {
  { let v: u8 = cpu.r8(0x58213fu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x428350; continue;
 }
 0x428350 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x428363; continue; }
  pc = 0x428355; continue;
 }
 0x428355 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { pc = 0x428363; continue; }
  pc = 0x428359; continue;
 }
 0x428359 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x56c76cu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x428363; continue;
 }
 0x428363 => {
  { let v: u32 = cpu.eax; cpu.w32(0x56c9bbu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x56c9bfu32, v); }
  cpu.poll(); pc = 0x428268; continue;
 }
 0x428373 => {
  { cpu.popad(); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba8u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba0u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba4u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6b9cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6b9cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6ba0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4283a3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x578d6bu32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4283b4); cpu.call(t); if cpu.returned_to != 0x4283b4 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x581a04u32)); cpu.eax = v; }
  cpu.poll(); pc = 0x428363; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004283c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4283c1); }
 let mut pc: u32 = 0x4283c1;
 loop { match pc {
 0x4283c1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5d00e5u32, v); }
  cpu.push(0x4283cf); f_00428201(cpu); if cpu.returned_to != 0x4283cf { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004283d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4283d0); }
 let mut pc: u32 = 0x4283d0;
 loop { match pc {
 0x4283d0 => {
  { let v: u16 = 0x54u16; cpu.w16(0x5d00e5u32, v); }
  cpu.push(0x4283de); f_00428201(cpu); if cpu.returned_to != 0x4283de { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004283df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4283df); }
 let mut pc: u32 = 0x4283df;
 loop { match pc {
 0x4283df => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4283e4; continue;
 }
 0x4283e4 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.pushad(); }
  cpu.push(0x4283ef); f_0041deda(cpu); if cpu.returned_to != 0x4283ef { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x428427; continue; }
  pc = 0x4283f2; continue;
 }
 0x4283f2 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x428423; continue; }
  pc = 0x4283f6; continue;
 }
 0x4283f6 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x428409; continue; }
  pc = 0x4283fb; continue;
 }
 0x4283fb => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { pc = 0x428421; continue; }
  pc = 0x4283ff; continue;
 }
 0x4283ff => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x56c76cu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42841f; continue;
 }
 0x428409 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { pc = 0x428421; continue; }
  pc = 0x428411; continue;
 }
 0x428411 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x56c76cu32.wrapping_add(cpu.edx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42841f; continue;
 }
 0x42841f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x428421; continue;
 }
 0x428421 => {
  cpu.poll(); pc = 0x4283e4; continue;
 }
 0x428423 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x428427 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x581a04u32)); cpu.eax = v; }
  cpu.poll(); pc = 0x42841f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428531(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428531); }
 let mut pc: u32 = 0x428531;
 loop { match pc {
 0x428531 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x56d2b4u32, v); }
  pc = 0x428537; continue;
 }
 0x428537 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x42853f; continue; }
  pc = 0x42853c; continue;
 }
 0x42853c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x428537; continue;
 }
 0x42853f => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x42854a); f_0041deda(cpu); if cpu.returned_to != 0x42854a { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x428580; continue; }
  pc = 0x42854d; continue;
 }
 0x42854d => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42857e; continue; }
  pc = 0x428551; continue;
 }
 0x428551 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x428564; continue; }
  pc = 0x428556; continue;
 }
 0x428556 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x42853f; continue; }
  pc = 0x42855a; continue;
 }
 0x42855a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x56c76cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x42857a; continue;
 }
 0x428564 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x42853f; continue; }
  pc = 0x42856c; continue;
 }
 0x42856c => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x56c76cu32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x42857a; continue;
 }
 0x42857a => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x42853f; continue; }
  pc = 0x42857e; continue;
 }
 0x42857e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x428580 => {
  { let v: u32 = cpu.r32(0x581a04u32); cpu.eax = v; }
  cpu.poll(); pc = 0x42857a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004286fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4286fa); }
 let mut pc: u32 = 0x4286fa;
 loop { match pc {
 0x4286fa => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x428755; continue; }
  pc = 0x428703; continue;
 }
 0x428703 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x428761; continue; }
  pc = 0x42870c; continue;
 }
 0x42870c => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x42876d; continue; }
  pc = 0x428715; continue;
 }
 0x428715 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x428779; continue; }
  pc = 0x42871e; continue;
 }
 0x42871e => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x428785; continue; }
  pc = 0x428727; continue;
 }
 0x428727 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x428779; continue; }
  pc = 0x428730; continue;
 }
 0x428730 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x428779; continue; }
  pc = 0x428739; continue;
 }
 0x428739 => {
  pc = 0x428791; continue;
 }
 0x42873b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42873c => {
  { let v: u32 = 0x596f1au32; cpu.edi = v; }
  pc = 0x428741; continue;
 }
 0x428741 => {
  { cpu.sub32(cpu.edi, 0x5977a6u32); }
  if cpu.zf { cpu.poll(); pc = 0x42873b; continue; }
  pc = 0x428749; continue;
 }
 0x428749 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x428791; continue; }
  pc = 0x428753; continue;
 }
 0x428753 => {
  cpu.poll(); pc = 0x428741; continue;
 }
 0x428755 => {
  { let v: u32 = 0x59445eu32; cpu.esi = v; }
  { let v: u32 = 0x594ceau32; cpu.ebp = v; }
  cpu.poll(); pc = 0x42873c; continue;
 }
 0x428761 => {
  { let v: u32 = 0x594ceau32; cpu.esi = v; }
  { let v: u32 = 0x595576u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x42873c; continue;
 }
 0x42876d => {
  { let v: u32 = 0x595576u32; cpu.esi = v; }
  { let v: u32 = 0x595e02u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x42873c; continue;
 }
 0x428779 => {
  { let v: u32 = 0x595e02u32; cpu.esi = v; }
  { let v: u32 = 0x59668eu32; cpu.ebp = v; }
  cpu.poll(); pc = 0x42873c; continue;
 }
 0x428785 => {
  { let v: u32 = 0x59668eu32; cpu.esi = v; }
  { let v: u32 = 0x596f1au32; cpu.ebp = v; }
  cpu.poll(); pc = 0x42873c; continue;
 }
 0x428791 => {
  cpu.push(0x428796); f_00422570(cpu); if cpu.returned_to != 0x428796 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004289ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4289ec); }
 let mut pc: u32 = 0x4289ec;
 loop { match pc {
 0x4289ec => {
  { cpu.pushad(); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x4289f7); f_00473e2a(cpu); if cpu.returned_to != 0x4289f7 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x4ae77cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x428a1b); f_00428b4d(cpu); if cpu.returned_to != 0x428a1b { return; }
  cpu.push(0x428a20); f_0040d1f0(cpu); if cpu.returned_to != 0x428a20 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.pushad(); }
  cpu.push(0x428a2f); f_0040d850(cpu); if cpu.returned_to != 0x428a2f { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x5dfb00u32; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428a52); f_0040d500(cpu); if cpu.returned_to != 0x428a52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.push(0x428a5a); f_00450ada(cpu); if cpu.returned_to != 0x428a5a { return; }
  { let v: u32 = 0x4b7d84u32; cpu.edi = v; }
  cpu.push(0x428a64); f_00428b4d(cpu); if cpu.returned_to != 0x428a64 { return; }
  { let v: u32 = 0x4eaac2u32; cpu.eax = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x4b7d84u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428a84); f_0040d500(cpu); if cpu.returned_to != 0x428a84 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x4e1ff4u32; cpu.edi = v; }
  cpu.push(0x428a91); f_00428b4d(cpu); if cpu.returned_to != 0x428a91 { return; }
  { let v: u32 = 0x4eaaf1u32; cpu.eax = v; }
  { let v: u32 = 0x4e1ff4u32; cpu.ebx = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428ab1); f_0040d500(cpu); if cpu.returned_to != 0x428ab1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x522514u32; cpu.edi = v; }
  cpu.push(0x428abe); f_00428b4d(cpu); if cpu.returned_to != 0x428abe { return; }
  { let v: u32 = 0x4eaad8u32; cpu.eax = v; }
  { let v: u32 = 0x522514u32; cpu.ebx = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428ade); f_0040d500(cpu); if cpu.returned_to != 0x428ade { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.push(0x428ae6); f_0046c698(cpu); if cpu.returned_to != 0x428ae6 { return; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.edi = v; }
  cpu.push(0x428af1); f_00428b4d(cpu); if cpu.returned_to != 0x428af1 { return; }
  { let v: u32 = 0x4ea994u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428b12); f_0040d500(cpu); if cpu.returned_to != 0x428b12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.push(0x428b1a); f_0041b99b(cpu); if cpu.returned_to != 0x428b1a { return; }
  { let v: u32 = 0x4eaab5u32; cpu.eax = v; }
  { let v: u32 = 0x5242ccu32; cpu.edi = v; }
  cpu.push(0x428b29); f_00428b4d(cpu); if cpu.returned_to != 0x428b29 { return; }
  { let v: u32 = 0x4eaab5u32; cpu.eax = v; }
  { let v: u32 = 0x5242ccu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x5978ecu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428b49); f_0040d500(cpu); if cpu.returned_to != 0x428b49 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428ba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428ba9); }
 let mut pc: u32 = 0x428ba9;
 loop { match pc {
 0x428ba9 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x428bb4); f_00473e2a(cpu); if cpu.returned_to != 0x428bb4 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x428bbf); f_00450ada(cpu); if cpu.returned_to != 0x428bbf { return; }
  cpu.push(0x428bc4); f_00450c8d(cpu); if cpu.returned_to != 0x428bc4 { return; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x4ae77cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  cpu.push(0x428be7); f_00428ea6(cpu); if cpu.returned_to != 0x428be7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428bfc); f_0040d850(cpu); if cpu.returned_to != 0x428bfc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428c18); f_0040d1f0(cpu); if cpu.returned_to != 0x428c18 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x5dfb00u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428c31); f_0040d500(cpu); if cpu.returned_to != 0x428c31 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428c3f); f_0040d1f0(cpu); if cpu.returned_to != 0x428c3f { return; }
  cpu.push(0x428c44); f_00450ab7(cpu); if cpu.returned_to != 0x428c44 { return; }
  { let v: u32 = 0x4b7d84u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  cpu.push(0x428c5a); f_00428ea6(cpu); if cpu.returned_to != 0x428c5a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4eaac2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428c69); f_0040d8b0(cpu); if cpu.returned_to != 0x428c69 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428c7f); f_0040d1f0(cpu); if cpu.returned_to != 0x428c7f { return; }
  { let v: u32 = 0x4eaac2u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x4b7d84u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428c9d); f_0040d500(cpu); if cpu.returned_to != 0x428c9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428cab); f_0040d1f0(cpu); if cpu.returned_to != 0x428cab { return; }
  cpu.push(0x428cb0); f_00450a86(cpu); if cpu.returned_to != 0x428cb0 { return; }
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x208u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  cpu.push(0x428ccb); f_00428ea6(cpu); if cpu.returned_to != 0x428ccb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4eaaf1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428cda); f_0040d8b0(cpu); if cpu.returned_to != 0x428cda { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428cf0); f_0040d1f0(cpu); if cpu.returned_to != 0x428cf0 { return; }
  { let v: u32 = 0x4eaaf1u32; cpu.eax = v; }
  { let v: u32 = 0x4e1ff4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428d0e); f_0040d500(cpu); if cpu.returned_to != 0x428d0e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428d1c); f_0040d1f0(cpu); if cpu.returned_to != 0x428d1c { return; }
  cpu.push(0x428d21); f_00450a5e(cpu); if cpu.returned_to != 0x428d21 { return; }
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x428d3c); f_00428ea6(cpu); if cpu.returned_to != 0x428d3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4eaad8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428d4b); f_0040d8b0(cpu); if cpu.returned_to != 0x428d4b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428d61); f_0040d1f0(cpu); if cpu.returned_to != 0x428d61 { return; }
  { let v: u32 = 0x4eaad8u32; cpu.eax = v; }
  { let v: u32 = 0x522514u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428d7f); f_0040d500(cpu); if cpu.returned_to != 0x428d7f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428d8d); f_0040d1f0(cpu); if cpu.returned_to != 0x428d8d { return; }
  cpu.push(0x428d92); f_0046c698(cpu); if cpu.returned_to != 0x428d92 { return; }
  { let v: u32 = 0x4ea994u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f3u32, v); }
  cpu.push(0x428daf); f_00447961(cpu); if cpu.returned_to != 0x428daf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f3u32, v); }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x428dce); f_00428ea6(cpu); if cpu.returned_to != 0x428dce { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4ea994u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428ddd); f_0040d8b0(cpu); if cpu.returned_to != 0x428ddd { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428df3); f_0040d1f0(cpu); if cpu.returned_to != 0x428df3 { return; }
  { let v: u32 = 0x4ea994u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428e12); f_0040d500(cpu); if cpu.returned_to != 0x428e12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428e20); f_0040d1f0(cpu); if cpu.returned_to != 0x428e20 { return; }
  cpu.push(0x428e25); f_0041b99b(cpu); if cpu.returned_to != 0x428e25 { return; }
  cpu.push(0x428e2a); f_0040d1f0(cpu); if cpu.returned_to != 0x428e2a { return; }
  { let v: u32 = 0x4eaab5u32; cpu.edx = v; }
  { let v: u32 = 0x5242ccu32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6au32, v); }
  cpu.push(0x428e46); f_00447961(cpu); if cpu.returned_to != 0x428e46 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x8a9u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.ebp = v; }
  cpu.push(0x428e62); f_00428ea6(cpu); if cpu.returned_to != 0x428e62 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4eaab5u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428e71); f_0040d8b0(cpu); if cpu.returned_to != 0x428e71 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d67dau32, v); }
  cpu.push(0x428e7f); f_0040d1f0(cpu); if cpu.returned_to != 0x428e7f { return; }
  { let v: u32 = 0x4eaab5u32; cpu.eax = v; }
  { let v: u32 = 0x5242ccu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6a94u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428e9d); f_0040d500(cpu); if cpu.returned_to != 0x428e9d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.push(0x428ea5); f_00450ada(cpu); if cpu.returned_to != 0x428ea5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428ea6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428ea6); }
 let mut pc: u32 = 0x428ea6;
 loop { match pc {
 0x428ea6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x428ead); f_00415463(cpu); if cpu.returned_to != 0x428ead { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5978ecu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x5978ecu32; cpu.esi = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  cpu.push(0x428ecb); f_00428ed3(cpu); if cpu.returned_to != 0x428ecb { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x7u32); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xfffffff8u32); cpu.ecx = v; }
  f_00428ed3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428ed3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428ed3); }
 let mut pc: u32 = 0x428ed3;
 loop { match pc {
 0x428ed3 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5978e4u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5978e8u32, v); }
  pc = 0x428edf; continue;
 }
 0x428edf => {
  { let v: u32 = cpu.r32(0x5978e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5978e8u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x59805cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x428efb; continue; }
  pc = 0x428ef4; continue;
 }
 0x428ef4 => {
  cpu.push(0x428ef9); f_00428ffd(cpu); if cpu.returned_to != 0x428ef9 { return; }
  pc = 0x428f02; continue;
 }
 0x428efb => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x428f01); f_00428f35(cpu); if cpu.returned_to != 0x428f01 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x428f02; continue;
 }
 0x428f02 => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  pc = 0x428f07; continue;
 }
 0x428f07 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x428edf; continue; }
  pc = 0x428f0a; continue;
 }
 0x428f0a => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x428f10); f_00415156(cpu); if cpu.returned_to != 0x428f10 { return; }
  { cpu.pushad(); }
  cpu.push(0x428f16); f_0041c122(cpu); if cpu.returned_to != 0x428f16 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4b9382u32; cpu.esi = v; }
  cpu.push(0x428f21); f_00427826(cpu); if cpu.returned_to != 0x428f21 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { cpu.poll(); pc = 0x428f07; continue; }
  pc = 0x428f24; continue;
 }
 0x428f24 => {
  { let v: u32 = 0x4b9382u32; cpu.esi = v; }
  cpu.push(0x428f2e); f_00427838(cpu); if cpu.returned_to != 0x428f2e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428f32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428f32); }
 let mut pc: u32 = 0x428f32;
 loop { match pc {
 0x428f32 => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428f35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428f35); }
 let mut pc: u32 = 0x428f35;
 loop { match pc {
 0x428f35 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5978dcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5978d8u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x190u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = 0x4b9379u32; cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x428f58; continue; }
  pc = 0x428f56; continue;
 }
 0x428f56 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x428f58; continue;
 }
 0x428f58 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x428f67; continue; }
  pc = 0x428f63; continue;
 }
 0x428f63 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ebp))); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  pc = 0x428f67; continue;
 }
 0x428f67 => {
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x428f6c; continue;
 }
 0x428f6c => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x428ffc; continue; }
  pc = 0x428f74; continue;
 }
 0x428f74 => {
  { cpu.pushad(); }
  cpu.push(0x428f7a); f_00415156(cpu); if cpu.returned_to != 0x428f7a { return; }
  cpu.push(0x428f7f); f_0041c122(cpu); if cpu.returned_to != 0x428f7f { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  if !cpu.sf { pc = 0x428fa3; continue; }
  pc = 0x428f85; continue;
 }
 0x428f85 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5978d8u32).wrapping_sub(cpu.ecx); cpu.w32(0x5978d8u32, v); }
  cpu.push(0x428f94); f_00414ac9(cpu); if cpu.returned_to != 0x428f94 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x428fb8; continue;
 }
 0x428fa3 => {
  { cpu.pushad(); }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5978d8u32).wrapping_sub(cpu.ecx); cpu.w32(0x5978d8u32, v); }
  cpu.push(0x428fb4); f_00414ac9(cpu); if cpu.returned_to != 0x428fb4 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.add32(cpu.esi, 0x64u32); cpu.esi = v; }
  pc = 0x428fb8; continue;
 }
 0x428fb8 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x428f6c; continue; }
  pc = 0x428fbf; continue;
 }
 0x428fbf => {
  cpu.push(0x428fc4); f_00429171(cpu); if cpu.returned_to != 0x428fc4 { return; }
  { cpu.pushad(); }
  pc = 0x428fc5; continue;
 }
 0x428fc5 => {
  cpu.push(0x428fca); f_00415156(cpu); if cpu.returned_to != 0x428fca { return; }
  cpu.push(0x428fcf); f_0041c122(cpu); if cpu.returned_to != 0x428fcf { return; }
  { let v: u32 = 0x4b9379u32; cpu.esi = v; }
  cpu.push(0x428fd9); f_00427826(cpu); if cpu.returned_to != 0x428fd9 { return; }
  if !cpu.cf { cpu.poll(); pc = 0x428fc5; continue; }
  pc = 0x428fdb; continue;
 }
 0x428fdb => {
  { let v: u32 = 0x4b9379u32; cpu.esi = v; }
  cpu.push(0x428fe5); f_00427838(cpu); if cpu.returned_to != 0x428fe5 { return; }
  { let v: u32 = 0x4b9379u32; cpu.ecx = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ecx))); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ecx)), 0x7fu8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x428ff6; continue; }
  pc = 0x428ff3; continue;
 }
 0x428ff3 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ecx))); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  pc = 0x428ff6; continue;
 }
 0x428ff6 => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x428f67; continue;
 }
 0x428ffc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00428ffd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x428ffd); }
 let mut pc: u32 = 0x428ffd;
 loop { match pc {
 0x428ffd => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5978dcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5978d8u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = 0x4b9379u32; cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x429020; continue; }
  pc = 0x42901e; continue;
 }
 0x42901e => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x429020; continue;
 }
 0x429020 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x42902f; continue; }
  pc = 0x42902b; continue;
 }
 0x42902b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ebp))); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  pc = 0x42902f; continue;
 }
 0x42902f => {
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x429034; continue;
 }
 0x429034 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4290c0; continue; }
  pc = 0x42903c; continue;
 }
 0x42903c => {
  { cpu.pushad(); }
  cpu.push(0x429042); f_00415156(cpu); if cpu.returned_to != 0x429042 { return; }
  cpu.push(0x429047); f_0041c122(cpu); if cpu.returned_to != 0x429047 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x18u32); cpu.ecx = v; }
  if !cpu.sf { pc = 0x42906b; continue; }
  pc = 0x42904d; continue;
 }
 0x42904d => {
  { let v: u32 = cpu.ecx.wrapping_add(0x18u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5978d8u32).wrapping_sub(cpu.ecx); cpu.w32(0x5978d8u32, v); }
  cpu.push(0x42905c); f_004290c1(cpu); if cpu.returned_to != 0x42905c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x429080; continue;
 }
 0x42906b => {
  { cpu.pushad(); }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5978d8u32).wrapping_sub(cpu.ecx); cpu.w32(0x5978d8u32, v); }
  cpu.push(0x42907c); f_004290c1(cpu); if cpu.returned_to != 0x42907c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.add32(cpu.esi, 0x18u32); cpu.esi = v; }
  pc = 0x429080; continue;
 }
 0x429080 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429034; continue; }
  pc = 0x429083; continue;
 }
 0x429083 => {
  cpu.push(0x429088); f_00429171(cpu); if cpu.returned_to != 0x429088 { return; }
  { cpu.pushad(); }
  pc = 0x429089; continue;
 }
 0x429089 => {
  cpu.push(0x42908e); f_00415156(cpu); if cpu.returned_to != 0x42908e { return; }
  cpu.push(0x429093); f_0041c122(cpu); if cpu.returned_to != 0x429093 { return; }
  { let v: u32 = 0x4b9379u32; cpu.esi = v; }
  cpu.push(0x42909d); f_00427826(cpu); if cpu.returned_to != 0x42909d { return; }
  if !cpu.cf { cpu.poll(); pc = 0x429089; continue; }
  pc = 0x42909f; continue;
 }
 0x42909f => {
  { let v: u32 = 0x4b9379u32; cpu.esi = v; }
  cpu.push(0x4290a9); f_00427838(cpu); if cpu.returned_to != 0x4290a9 { return; }
  { let v: u32 = 0x4b9379u32; cpu.ecx = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ecx))); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ecx)), 0x7fu8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x4290ba; continue; }
  pc = 0x4290b7; continue;
 }
 0x4290b7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ecx))); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4290ba; continue;
 }
 0x4290ba => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x42902f; continue;
 }
 0x4290c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004290c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4290c1); }
 let mut pc: u32 = 0x4290c1;
 loop { match pc {
 0x4290c1 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = 0x597922u32; cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x4290cc; continue;
 }
 0x4290cc => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x4290d9; continue; }
  pc = 0x4290d4; continue;
 }
 0x4290d4 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4290d9; continue;
 }
 0x4290d9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x4290e8; continue; }
  pc = 0x4290e3; continue;
 }
 0x4290e3 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4290e8; continue;
 }
 0x4290e8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x4290f7; continue; }
  pc = 0x4290f2; continue;
 }
 0x4290f2 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4290f7; continue;
 }
 0x4290f7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x429106; continue; }
  pc = 0x429101; continue;
 }
 0x429101 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x429106; continue;
 }
 0x429106 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x429115; continue; }
  pc = 0x429110; continue;
 }
 0x429110 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x429115; continue;
 }
 0x429115 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x429124; continue; }
  pc = 0x42911f; continue;
 }
 0x42911f => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x429124; continue;
 }
 0x429124 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x429133; continue; }
  pc = 0x42912e; continue;
 }
 0x42912e => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x429133; continue;
 }
 0x429133 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x429142; continue; }
  pc = 0x42913d; continue;
 }
 0x42913d => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x429142; continue;
 }
 0x429142 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)) & 0xfu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4290cc; continue; }
  pc = 0x42915c; continue;
 }
 0x42915c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x597922u32; cpu.esi = v; }
  cpu.push(0x429170); f_00414ac9(cpu); if cpu.returned_to != 0x429170 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429171(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429171); }
 let mut pc: u32 = 0x429171;
 loop { match pc {
 0x429171 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x5978dcu32), 0x18u32); }
  if cpu.zf { pc = 0x4291a2; continue; }
  pc = 0x42917b; continue;
 }
 0x42917b => {
  { let v: u32 = cpu.r32(0x5978d8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.r32(0x5978dcu32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4291a1); f_004291a4(cpu); if cpu.returned_to != 0x4291a1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4291a2; continue;
 }
 0x4291a2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004291a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4291a4); }
 let mut pc: u32 = 0x4291a4;
 loop { match pc {
 0x4291a4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x59791eu32, v); }
  { let v: u32 = 0xb8u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4291e4; continue; }
  pc = 0x4291ba; continue;
 }
 0x4291ba => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x98u32; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4291cf); cpu.call(t); if cpu.returned_to != 0x4291cf { return; }
  { let v: u32 = 0xb8u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4292d6; continue; }
  pc = 0x4291e4; continue;
 }
 0x4291e4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.esi = v; }
  { let v: u32 = 0x59f458u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0xeu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x12u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x597958u32; cpu.esi = v; }
  { let v: u32 = 0x1c0u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x59f5bfu32; cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x59791eu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x429248; continue; }
  pc = 0x429227; continue;
 }
 0x429227 => {
  { cpu.sub32(cpu.eax, 0x64u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x429236; continue; }
  pc = 0x42922c; continue;
 }
 0x42922c => {
  { let v: u32 = 0x64u32; cpu.w32(0x59791eu32, v); }
  pc = 0x429236; continue;
 }
 0x429236 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x59791eu32); cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x70u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429236; continue; }
  pc = 0x429248; continue;
 }
 0x429248 => {
  { let v: u32 = cpu.r16(0x4d68f2u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68f2u32, v); }
  { let v: u32 = cpu.r8(0x58e95cu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x429276); f_004298e5(cpu); if cpu.returned_to != 0x429276 { return; }
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v: u32 = 0x98u32; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42928b); cpu.call(t); if cpu.returned_to != 0x42928b { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x429296); cpu.call(t); if cpu.returned_to != 0x429296 { return; }
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v: u32 = 0x98u32; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4292ab); cpu.call(t); if cpu.returned_to != 0x4292ab { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4292b6); cpu.call(t); if cpu.returned_to != 0x4292b6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68f2u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xb8u32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x4292d6; continue;
 }
 0x4292d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004292d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4292d8); }
 let mut pc: u32 = 0x4292d8;
 loop { match pc {
 0x4292d8 => {
  cpu.push(0x4292dd); f_004292de(cpu); if cpu.returned_to != 0x4292dd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004292de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4292de); }
 let mut pc: u32 = 0x4292de;
 loop { match pc {
 0x4292de => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x4292e6); f_004292fa(cpu); if cpu.returned_to != 0x4292e6 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x4292ec); f_004292fa(cpu); if cpu.returned_to != 0x4292ec { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x4292f2); f_004292fa(cpu); if cpu.returned_to != 0x4292f2 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x4292f8); f_004292fa(cpu); if cpu.returned_to != 0x4292f8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004292fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4292fa); }
 let mut pc: u32 = 0x4292fa;
 loop { match pc {
 0x4292fa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x581b77u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
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
pub fn f_00429336(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429336); }
 let mut pc: u32 = 0x429336;
 loop { match pc {
 0x429336 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f73u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x590f78u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b1cu32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x5914f8u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b20u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x591a78u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b24u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x591ff8u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b28u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f89u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429630); }
 let mut pc: u32 = 0x429630;
 loop { match pc {
 0x429630 => {
  { cpu.pushad(); }
  { let v: u32 = 0x56d348u32; cpu.edi = v; }
  { let v: u32 = 0x56d23du32; cpu.esi = v; }
  pc = 0x42963b; continue;
 }
 0x42963b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x429647; continue; }
  pc = 0x429641; continue;
 }
 0x429641 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x42963b; continue;
 }
 0x429647 => {
  { let v: u32 = 0x5a6e71u32; cpu.esi = v; }
  pc = 0x42964c; continue;
 }
 0x42964c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x429658; continue; }
  pc = 0x429652; continue;
 }
 0x429652 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x42964c; continue;
 }
 0x429658 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x56c62fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ae0b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x429667; continue;
 }
 0x429667 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x429673; continue; }
  pc = 0x42966d; continue;
 }
 0x42966d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x429667; continue;
 }
 0x429673 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004296c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4296c0); }
 let mut pc: u32 = 0x4296c0;
 loop { match pc {
 0x4296c0 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2980u32, v); }
  { let v: u16 = (cpu.eax as u16) & 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2984u32, v); }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4296e6); f_00420989(cpu); if cpu.returned_to != 0x4296e6 { return; }
  { let v: u32 = cpu.r32(0x5a2980u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2980u32, v); }
  { let v: u32 = cpu.r32(0x5a2984u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5a2984u32, v); }
  { let v: u16 = 0x17u16; cpu.w16(0x59c5abu32, v); }
  { cpu.sub32(cpu.r32(0x4d6bccu32), 0x190u32); }
  if cpu.zf { pc = 0x429722; continue; }
  pc = 0x429719; continue;
 }
 0x429719 => {
  { let v: u16 = 0x1cu16; cpu.w16(0x59c5abu32, v); }
  pc = 0x429722; continue;
 }
 0x429722 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x22u16; cpu.w16(0x59c5a9u32, v); }
  { let v = cpu.r32(0x5a2980u32); cpu.push(v); }
  pc = 0x429732; continue;
 }
 0x429732 => {
  cpu.push(0x429737); f_0042975f(cpu); if cpu.returned_to != 0x429737 { return; }
  { let v: u16 = cpu.dec16(cpu.r16(0x59c5a9u32)); cpu.w16(0x59c5a9u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x429732; continue; }
  pc = 0x429740; continue;
 }
 0x429740 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x5a2980u32, v); }
  { let v: u32 = cpu.r32(0x5a2984u32).wrapping_add(0x10u32); cpu.w32(0x5a2984u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0x59c5abu32)); cpu.w16(0x59c5abu32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x429722; continue; }
  pc = 0x42975d; continue;
 }
 0x42975d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042975f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42975f); }
 let mut pc: u32 = 0x42975f;
 loop { match pc {
 0x42975f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x429774; continue; }
  pc = 0x429770; continue;
 }
 0x429770 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x429774; continue;
 }
 0x429774 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4ac40eu32)); }
  if cpu.zf { pc = 0x429797; continue; }
  pc = 0x42977d; continue;
 }
 0x42977d => {
  { let v: u32 = cpu.shr32(cpu.eax, 6u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x5a2984u32); cpu.push(v); }
  { let v = cpu.r32(0x5a2980u32); cpu.push(v); }
  cpu.push(0x429793); f_004060b0(cpu); if cpu.returned_to != 0x429793 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x429797; continue;
 }
 0x429797 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5a2980u32), 0x10u32); cpu.w32(0x5a2980u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004297a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4297a8); }
 let mut pc: u32 = 0x4297a8;
 loop { match pc {
 0x4297a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429846(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429846); }
 let mut pc: u32 = 0x429846;
 loop { match pc {
 0x429846 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59c54du32, v); }
  { let v: u32 = 0x280u32; cpu.w32(0x59c555u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x59c55du32, v); }
  { let v: u32 = 0x280u32; cpu.w32(0x4d6ba4u32, v); }
  { cpu.sub8(cpu.r8(0x59c54cu32), 0x1u8); }
  if !cpu.zf { pc = 0x4298a1; continue; }
  pc = 0x429877; continue;
 }
 0x429877 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59c551u32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x59c559u32, v); }
  { let v: u32 = 0x19u32; cpu.w32(0x59c561u32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x4d6ba8u32, v); }
  pc = 0x4298c9; continue;
 }
 0x4298a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59c551u32, v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x59c559u32, v); }
  { let v: u32 = 0x1eu32; cpu.w32(0x59c561u32, v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x4d6ba8u32, v); }
  pc = 0x4298c9; continue;
 }
 0x4298c9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b9cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6ba0u32, v); }
  cpu.push(0x4298e2); f_004297a8(cpu); if cpu.returned_to != 0x4298e2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004298e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4298e3); }
 let mut pc: u32 = 0x4298e3;
 loop { match pc {
 0x4298e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004298e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4298e4); }
 let mut pc: u32 = 0x4298e4;
 loop { match pc {
 0x4298e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004298e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4298e5); }
 let mut pc: u32 = 0x4298e5;
 loop { match pc {
 0x4298e5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x4d68f2u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58e95cu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x42993e; continue; }
  pc = 0x4298f6; continue;
 }
 0x4298f6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x59813cu32; cpu.edi = v; }
  pc = 0x42991f; continue;
 }
 0x42991f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x429925; continue;
 }
 0x429925 => {
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x429925; continue; }
  pc = 0x42992b; continue;
 }
 0x42992b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42991f; continue; }
  pc = 0x429932; continue;
 }
 0x429932 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x59813cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x42993e; continue;
 }
 0x42993e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429954); }
 let mut pc: u32 = 0x429954;
 loop { match pc {
 0x429954 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x429966); f_00447634(cpu); if cpu.returned_to != 0x429966 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1f70u32), 0x0u8); }
  if !cpu.zf { pc = 0x429976; continue; }
  pc = 0x429972; continue;
 }
 0x429972 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x429978; continue;
 }
 0x429976 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x429978; continue;
 }
 0x429978 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429980); f_00406220(cpu); if cpu.returned_to != 0x429980 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429985); }
 let mut pc: u32 = 0x429985;
 loop { match pc {
 0x429985 => {
  f_00429954(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429987); }
 let mut pc: u32 = 0x429987;
 loop { match pc {
 0x429987 => {
  f_00429954(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429989); }
 let mut pc: u32 = 0x429989;
 loop { match pc {
 0x429989 => {
  f_00429954(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042998b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42998b); }
 let mut pc: u32 = 0x42998b;
 loop { match pc {
 0x42998b => {
  { let v: u8 = cpu.r8(0x4e1f70u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f70u32, v); }
  cpu.push(0x42999e); f_00429954(cpu); if cpu.returned_to != 0x42999e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4e1f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004299b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4299b0); }
 let mut pc: u32 = 0x4299b0;
 loop { match pc {
 0x4299b0 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4299cc; continue; }
  pc = 0x4299cb; continue;
 }
 0x4299cb => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x4299cc; continue;
 }
 0x4299cc => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4299d2; continue; }
  pc = 0x4299d0; continue;
 }
 0x4299d0 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x4299d2; continue;
 }
 0x4299d2 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.ebp; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebp = v; } }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x4299da; continue;
 }
 0x4299da => {
  cpu.push(0x4299df); f_00429a1d(cpu); if cpu.returned_to != 0x4299df { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4299da; continue; }
  pc = 0x4299e3; continue;
 }
 0x4299e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004299ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4299ef); }
 let mut pc: u32 = 0x4299ef;
 loop { match pc {
 0x4299ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004299f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4299f0); }
 let mut pc: u32 = 0x4299f0;
 loop { match pc {
 0x4299f0 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x4299fb); f_00429a1d(cpu); if cpu.returned_to != 0x4299fb { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004299fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4299fe); }
 let mut pc: u32 = 0x4299fe;
 loop { match pc {
 0x4299fe => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429a18); f_00405bd0(cpu); if cpu.returned_to != 0x429a18 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429a1d); }
 let mut pc: u32 = 0x429a1d;
 loop { match pc {
 0x429a1d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x429a83; continue; }
  pc = 0x429a37; continue;
 }
 0x429a37 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { pc = 0x429a3d; continue; }
  pc = 0x429a3b; continue;
 }
 0x429a3b => {
  pc = 0x429a83; continue;
 }
 0x429a3d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x429a3b; continue; }
  pc = 0x429a45; continue;
 }
 0x429a45 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x429a51; continue; }
  pc = 0x429a49; continue;
 }
 0x429a49 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x429a3b; continue; }
  pc = 0x429a4d; continue;
 }
 0x429a4d => {
  if cpu.sf { cpu.poll(); pc = 0x429a3b; continue; }
  pc = 0x429a4f; continue;
 }
 0x429a4f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x429a51; continue;
 }
 0x429a51 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4d6ba4u32)); cpu.ebp = v; }
  if (cpu.sf != cpu.of) { pc = 0x429a63; continue; }
  pc = 0x429a5d; continue;
 }
 0x429a5d => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebp); cpu.ecx = v; }
  if cpu.sf { cpu.poll(); pc = 0x429a3b; continue; }
  pc = 0x429a61; continue;
 }
 0x429a61 => {
  if cpu.zf { cpu.poll(); pc = 0x429a3b; continue; }
  pc = 0x429a63; continue;
 }
 0x429a63 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429a80); f_00405ae0(cpu); if cpu.returned_to != 0x429a80 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x429a83; continue;
 }
 0x429a83 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429a85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429a85); }
 let mut pc: u32 = 0x429a85;
 loop { match pc {
 0x429a85 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x429aeb; continue; }
  pc = 0x429a9f; continue;
 }
 0x429a9f => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x429aa5; continue; }
  pc = 0x429aa3; continue;
 }
 0x429aa3 => {
  pc = 0x429aeb; continue;
 }
 0x429aa5 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x429aa3; continue; }
  pc = 0x429aad; continue;
 }
 0x429aad => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { pc = 0x429ab9; continue; }
  pc = 0x429ab1; continue;
 }
 0x429ab1 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x429aa3; continue; }
  pc = 0x429ab5; continue;
 }
 0x429ab5 => {
  if cpu.sf { cpu.poll(); pc = 0x429aa3; continue; }
  pc = 0x429ab7; continue;
 }
 0x429ab7 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x429ab9; continue;
 }
 0x429ab9 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4d6ba8u32)); cpu.ebp = v; }
  if (cpu.sf != cpu.of) { pc = 0x429acb; continue; }
  pc = 0x429ac5; continue;
 }
 0x429ac5 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebp); cpu.ecx = v; }
  if cpu.sf { cpu.poll(); pc = 0x429aa3; continue; }
  pc = 0x429ac9; continue;
 }
 0x429ac9 => {
  if cpu.zf { cpu.poll(); pc = 0x429aa3; continue; }
  pc = 0x429acb; continue;
 }
 0x429acb => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429ae8); f_00405b60(cpu); if cpu.returned_to != 0x429ae8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x429aeb; continue;
 }
 0x429aeb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429af1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429af1); }
 let mut pc: u32 = 0x429af1;
 loop { match pc {
 0x429af1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x59c590u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x59c580u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x59c584u32); cpu.edx = v; }
  pc = 0x429b11; continue;
 }
 0x429b11 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x3fffu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x59813cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x429b39; continue; }
  pc = 0x429b25; continue;
 }
 0x429b25 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x59c588u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x59c58cu32)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429b11; continue; }
  pc = 0x429b37; continue;
 }
 0x429b37 => {
  pc = 0x429b49; continue;
 }
 0x429b39 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x59c588u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x59c58cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429b11; continue; }
  pc = 0x429b49; continue;
 }
 0x429b49 => {
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429b4c); }
 let mut pc: u32 = 0x429b4c;
 loop { match pc {
 0x429b4c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x59c59du32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x429c9c; continue; }
  pc = 0x429b6b; continue;
 }
 0x429b6b => {
  { let v: u32 = cpu.eax; cpu.w32(0x59c5a5u32, v); }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59c599u32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x429c9c; continue; }
  pc = 0x429b82; continue;
 }
 0x429b82 => {
  { let v = cpu.r32(0x59c59du32); cpu.mul32(v); }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59c595u32, v); }
  { let v: u32 = 0x59813cu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x59c59du32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v: u32 = 0xfcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59c599u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59c5a1u32, v); }
  { let v: u32 = cpu.r32(0x59c599u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x59c5a1u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x59c5a1u32)); cpu.edi = v; }
  pc = 0x429bd4; continue;
 }
 0x429bd4 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if !cpu.zf { pc = 0x429c19; continue; }
  pc = 0x429bd9; continue;
 }
 0x429bd9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x59c595u32); cpu.edx = v; }
  pc = 0x429be6; continue;
 }
 0x429be6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x429be6; continue; }
  pc = 0x429c03; continue;
 }
 0x429c03 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x59c5a1u32)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x59c5a5u32)); cpu.w32(0x59c5a5u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x429bd4; continue; }
  pc = 0x429c17; continue;
 }
 0x429c17 => {
  pc = 0x429c69; continue;
 }
 0x429c19 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x4abeecu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x59c595u32); cpu.edx = v; }
  pc = 0x429c2c; continue;
 }
 0x429c2c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x429c2c; continue; }
  pc = 0x429c50; continue;
 }
 0x429c50 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x59c5a1u32)); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x59c5a5u32)); cpu.w32(0x59c5a5u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x429bd4; continue; }
  pc = 0x429c69; continue;
 }
 0x429c69 => {
  { let v: u32 = cpu.r32(0x59c599u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x59c59du32)); cpu.ebx = v; }
  if !cpu.sf { pc = 0x429c79; continue; }
  pc = 0x429c77; continue;
 }
 0x429c77 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x429c79; continue;
 }
 0x429c79 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x429c7c; continue;
 }
 0x429c7c => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x59c5a1u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429c7c; continue; }
  pc = 0x429c8a; continue;
 }
 0x429c8a => {
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u16 = cpu.r16(0x59c59du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x59c599u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x429c9c => {
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429ca1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x429ca1); } r_00429ca1(cpu, 0x429ca1); }
pub fn f_00429ca3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x429ca3); } r_00429ca1(cpu, 0x429ca3); }
fn r_00429ca1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x429ca1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6b3cu32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4d9548u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x4d954au32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x429cca; continue;
 }
 0x429ca3 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6b3cu32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4d9548u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x4d954au32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x429cca; continue;
 }
 0x429cca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x429cce; continue;
 }
 0x429cce => {
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.lods(1, false); }
  { let v: u16 = (cpu.eax as u16) & 0x7f7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x7eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0x7f7fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429ce3; continue;
 }
 0x429ce3 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429ce7; continue;
 }
 0x429ce7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x429d2b; continue; }
  pc = 0x429cec; continue;
 }
 0x429cec => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x429d2b; continue; }
  pc = 0x429cf1; continue;
 }
 0x429cf1 => {
  { cpu.sub8((cpu.ebx as u8), 0x18u8); }
  if cpu.zf { pc = 0x429d33; continue; }
  pc = 0x429cf6; continue;
 }
 0x429cf6 => {
  { cpu.sub8((cpu.ebx as u8), 0x19u8); }
  if cpu.zf { pc = 0x429d33; continue; }
  pc = 0x429cfb; continue;
 }
 0x429cfb => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x429d2f; continue; }
  pc = 0x429d00; continue;
 }
 0x429d00 => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x429d2f; continue; }
  pc = 0x429d05; continue;
 }
 0x429d05 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429d09; continue;
 }
 0x429d09 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xbu8); }
  if cpu.zf { pc = 0x429d2b; continue; }
  pc = 0x429d0e; continue;
 }
 0x429d0e => {
  { cpu.sub8((cpu.ebx as u8), 0xbu8); }
  if cpu.zf { pc = 0x429d33; continue; }
  pc = 0x429d13; continue;
 }
 0x429d13 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429d17; continue;
 }
 0x429d17 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429d1b; continue;
 }
 0x429d1b => {
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x429d35; continue; }
  pc = 0x429d1f; continue;
 }
 0x429d1f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x429d2b; continue; }
  pc = 0x429d23; continue;
 }
 0x429d23 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x429d2b; continue; }
  pc = 0x429d27; continue;
 }
 0x429d27 => {
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x429d33; continue; }
  pc = 0x429d2b; continue;
 }
 0x429d2b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x429d35; continue;
 }
 0x429d2f => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x429d35; continue;
 }
 0x429d33 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x429d35; continue;
 }
 0x429d35 => {
  { let v: u32 = 0x4ea0a6u32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x429cce; continue; }
  pc = 0x429d3f; continue;
 }
 0x429d3f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x100u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x429cca; continue; }
  pc = 0x429d4f; continue;
 }
 0x429d4f => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429d51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429d51); }
 let mut pc: u32 = 0x429d51;
 loop { match pc {
 0x429d51 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429d69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429d69); }
 let mut pc: u32 = 0x429d69;
 loop { match pc {
 0x429d69 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x5a23c8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23c8u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x429d8c; continue; }
  pc = 0x429d82; continue;
 }
 0x429d82 => {
  cpu.push(0x429d87); f_00429e4a(cpu); if cpu.returned_to != 0x429d87 { return; }
  pc = 0x429e41; continue;
 }
 0x429d8c => {
  { let v: u16 = 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x429d95); f_00417dab(cpu); if cpu.returned_to != 0x429d95 { return; }
  let t = cpu.r32(0x4d6f88u32); cpu.push(0x429d9b); cpu.call(t); if cpu.returned_to != 0x429d9b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x429da8); f_00402d70(cpu); if cpu.returned_to != 0x429da8 { return; }
  cpu.push(0x429dad); f_00405320(cpu); if cpu.returned_to != 0x429dad { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5824d5u32, v); }
  { let v: u32 = 0x588b60u32; cpu.edx = v; }
  cpu.push(0x429dbf); f_004275fd(cpu); if cpu.returned_to != 0x429dbf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5824d5u32, v); }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x429dd0); cpu.call(t); if cpu.returned_to != 0x429dd0 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x429dde); f_00417ca0(cpu); if cpu.returned_to != 0x429dde { return; }
  { let v: u32 = 0x258u32; cpu.ecx = v; }
  pc = 0x429de3; continue;
 }
 0x429de3 => {
  cpu.push(0x429de8); f_0041c122(cpu); if cpu.returned_to != 0x429de8 { return; }
  if !cpu.cf { pc = 0x429dee; continue; }
  pc = 0x429dea; continue;
 }
 0x429dea => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x429df8; continue; }
  pc = 0x429dee; continue;
 }
 0x429dee => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f8cu32); cpu.push(0x429df5); cpu.call(t); if cpu.returned_to != 0x429df5 { return; }
  { cpu.popad(); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x429de3; continue; }
  pc = 0x429df8; continue;
 }
 0x429df8 => {
  { let v: u16 = 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x429e01); f_00417dab(cpu); if cpu.returned_to != 0x429e01 { return; }
  cpu.push(0x429e06); f_0043dabc(cpu); if cpu.returned_to != 0x429e06 { return; }
  cpu.push(0x429e0b); f_00430814(cpu); if cpu.returned_to != 0x429e0b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x588b80u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x588b81u32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x429e2d); f_0043db68(cpu); if cpu.returned_to != 0x429e2d { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x429e41); f_0043db68(cpu); if cpu.returned_to != 0x429e41 { return; }
  pc = 0x429e41; continue;
 }
 0x429e41 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23c8u32, v); }
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429e4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429e4a); }
 let mut pc: u32 = 0x429e4a;
 loop { match pc {
 0x429e4a => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.push(0x429e65); f_0043db68(cpu); if cpu.returned_to != 0x429e65 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00429e66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x429e66); }
 let mut pc: u32 = 0x429e66;
 loop { match pc {
 0x429e66 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x588b80u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x588b81u32, v); }
  { let v: u32 = 0x4abeecu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a3cu32, v); }
  cpu.push(0x429e84); f_0042122f(cpu); if cpu.returned_to != 0x429e84 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a88u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575d11u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af6u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5c9af5u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5c9af0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5c9af2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5c9af1u32, v); }
  cpu.push(0x429ebf); f_0044ff0c(cpu); if cpu.returned_to != 0x429ebf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5824aau32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x56c9dau32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6ac8u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6988u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6988u32, v); }
  { let v: u32 = 0x4d749au32; cpu.w32(0x54a7c8u32, v); }
  cpu.push(0x429ef8); f_0040d300(cpu); if cpu.returned_to != 0x429ef8 { return; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x429f01; continue; }
  pc = 0x429efd; continue;
 }
 0x429efd => {
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x429f01; continue;
 }
 0x429f01 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6756u32, v); }
  cpu.push(0x429f0c); f_00466f4d(cpu); if cpu.returned_to != 0x429f0c { return; }
  cpu.push(0x429f11); f_00448dcc(cpu); if cpu.returned_to != 0x429f11 { return; }
  cpu.push(0x429f16); f_00417ba8(cpu); if cpu.returned_to != 0x429f16 { return; }
  cpu.push(0x429f1b); f_0047ab2c(cpu); if cpu.returned_to != 0x429f1b { return; }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429f26); f_0040c270(cpu); if cpu.returned_to != 0x429f26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x429f2e); f_004173cf(cpu); if cpu.returned_to != 0x429f2e { return; }
  cpu.push(0x429f33); f_0041bef7(cpu); if cpu.returned_to != 0x429f33 { return; }
  { let v: u8 = 0x5u8; cpu.w8(0x5d31b4u32, v); }
  { let v: u16 = 0x5au16; cpu.w16(0x5d31b6u32, v); }
  cpu.push(0x429f48); f_004286fa(cpu); if cpu.returned_to != 0x429f48 { return; }
  cpu.push(0x429f4d); f_00447fd3(cpu); if cpu.returned_to != 0x429f4d { return; }
  cpu.push(0x429f52); f_004477ab(cpu); if cpu.returned_to != 0x429f52 { return; }
  { let v: u32 = 0x588856u32; cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429f64); f_0040d400(cpu); if cpu.returned_to != 0x429f64 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42a01c; continue; }
  pc = 0x429f70; continue;
 }
 0x429f70 => {
  { let v: u32 = 0x4ea924u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x429fb6; continue; }
  pc = 0x429f7e; continue;
 }
 0x429f7e => {
  { let v: u32 = 0x4ea930u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x429fb6; continue; }
  pc = 0x429f8c; continue;
 }
 0x429f8c => {
  { let v: u32 = 0x4ea93du32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x429fb6; continue; }
  pc = 0x429f9a; continue;
 }
 0x429f9a => {
  { let v: u32 = 0x4ea94au32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x429fb6; continue; }
  pc = 0x429fa8; continue;
 }
 0x429fa8 => {
  { let v: u32 = 0x4ea957u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x429fb6; continue; }
  pc = 0x429fb6; continue;
 }
 0x429fb6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x429fc3); f_0040d400(cpu); if cpu.returned_to != 0x429fc3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42a01c; continue; }
  pc = 0x429fcb; continue;
 }
 0x429fcb => {
  { let v: u32 = 0x0u32; cpu.w32(0x59f44au32, v); }
  cpu.push(0x429fda); f_00450ada(cpu); if cpu.returned_to != 0x429fda { return; }
  { let v: u32 = 0x581ab4u32; cpu.eax = v; }
  { let v: u32 = 0x6976656cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x429fea); f_00450c16(cpu); if cpu.returned_to != 0x429fea { return; }
  { let v: u32 = 0x581ab4u32; cpu.eax = v; }
  { let v: u32 = 0x6d6e666cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x429ffa); f_00415d93(cpu); if cpu.returned_to != 0x429ffa { return; }
  if cpu.cf { pc = 0x42a01c; continue; }
  pc = 0x429ffc; continue;
 }
 0x429ffc => {
  cpu.push(0x42a001); f_0041c60a(cpu); if cpu.returned_to != 0x42a001 { return; }
  cpu.push(0x42a006); f_0041c622(cpu); if cpu.returned_to != 0x42a006 { return; }
  cpu.push(0x42a00b); f_0047b363(cpu); if cpu.returned_to != 0x42a00b { return; }
  cpu.push(0x42a010); f_00430dc9(cpu); if cpu.returned_to != 0x42a010 { return; }
  cpu.push(0x42a015); f_00439d9d(cpu); if cpu.returned_to != 0x42a015 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42a01c => {
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042b632(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42b632); }
 let mut pc: u32 = 0x42b632;
 loop { match pc {
 0x42b632 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x42b65f; continue; }
  pc = 0x42b63b; continue;
 }
 0x42b63b => {
  { cpu.and8(cpu.r8(0x4e156au32), 0x4u8); }
  if cpu.zf { pc = 0x42b694; continue; }
  pc = 0x42b644; continue;
 }
 0x42b644 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42b659); cpu.call(t); if cpu.returned_to != 0x42b659 { return; }
  cpu.push(0x42b65e); f_0042b695(cpu); if cpu.returned_to != 0x42b65e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42b65f => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x42b694; continue; }
  pc = 0x42b668; continue;
 }
 0x42b668 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x3u8); }
  if cpu.zf { pc = 0x42b67a; continue; }
  pc = 0x42b671; continue;
 }
 0x42b671 => {
  { cpu.and8(cpu.r8(0x4e156au32), 0x4u8); }
  if cpu.zf { pc = 0x42b694; continue; }
  pc = 0x42b67a; continue;
 }
 0x42b67a => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42b68f); cpu.call(t); if cpu.returned_to != 0x42b68f { return; }
  cpu.push(0x42b694); f_0042b695(cpu); if cpu.returned_to != 0x42b694 { return; }
  pc = 0x42b694; continue;
 }
 0x42b694 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042b695(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42b695); }
 let mut pc: u32 = 0x42b695;
 loop { match pc {
 0x42b695 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x576e99u32)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42b6a9); f_00474aaf(cpu); if cpu.returned_to != 0x42b6a9 { return; }
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
  cpu.push(0x42b6cf); f_0041b6f1(cpu); if cpu.returned_to != 0x42b6cf { return; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d00ebu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x42b6f0); f_0041b6f1(cpu); if cpu.returned_to != 0x42b6f0 { return; }
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
  cpu.push(0x42b714); f_0041b6f1(cpu); if cpu.returned_to != 0x42b714 { return; }
  { let v: u16 = cpu.r16(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x42b742); f_0041cf66(cpu); if cpu.returned_to != 0x42b742 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042b792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42b792); }
 let mut pc: u32 = 0x42b792;
 loop { match pc {
 0x42b792 => {
  cpu.push(0x42b797); f_0042ba35(cpu); if cpu.returned_to != 0x42b797 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x42b7ab); f_00402d70(cpu); if cpu.returned_to != 0x42b7ab { return; }
  cpu.push(0x42b7b0); f_00405320(cpu); if cpu.returned_to != 0x42b7b0 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a23b6u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a23bau32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x42b7d5; continue; }
  pc = 0x42b7ce; continue;
 }
 0x42b7ce => {
  cpu.push(0x42b7d3); f_00417352(cpu); if cpu.returned_to != 0x42b7d3 { return; }
  pc = 0x42b803; continue;
 }
 0x42b7d5 => {
  { let v: u32 = 0x54a7d0u32; cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x42b7eb); f_0040bca0(cpu); if cpu.returned_to != 0x42b7eb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0xa0u32; cpu.w32(0x5a23b6u32, v); }
  { let v: u32 = 0x64u32; cpu.w32(0x5a23bau32, v); }
  pc = 0x42b803; continue;
 }
 0x42b803 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x42b810); f_00402d60(cpu); if cpu.returned_to != 0x42b810 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ba0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ba0d); }
 let mut pc: u32 = 0x42ba0d;
 loop { match pc {
 0x42ba0d => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u8 = 0xfau8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42ba2d); f_00413337(cpu); if cpu.returned_to != 0x42ba2d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ba35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ba35); }
 let mut pc: u32 = 0x42ba35;
 loop { match pc {
 0x42ba35 => {
  { cpu.pushad(); }
  cpu.push(0x42ba3b); f_004010e0(cpu); if cpu.returned_to != 0x42ba3b { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9dau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadfu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bc07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bc07); }
 let mut pc: u32 = 0x42bc07;
 loop { match pc {
 0x42bc07 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b66u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  cpu.push(0x42bc1d); f_0042ba35(cpu); if cpu.returned_to != 0x42bc1d { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5a23b6u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a23bau32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  cpu.push(0x42bc51); f_0042b792(cpu); if cpu.returned_to != 0x42bc51 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d39bu32, v); }
  cpu.push(0x42bc64); f_0043733b(cpu); if cpu.returned_to != 0x42bc64 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de71cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  { let v = cpu.flags(); cpu.push(v); }
  cpu.push(0x42bc7e); f_004152c0(cpu); if cpu.returned_to != 0x42bc7e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x42bc8b); f_00402d60(cpu); if cpu.returned_to != 0x42bc8b { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x575536u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  cpu.push(0x42bc9f); f_0041b2d5(cpu); if cpu.returned_to != 0x42bc9f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x42bcc4; continue; }
  pc = 0x42bcb3; continue;
 }
 0x42bcb3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581b66u32, v); }
  { cpu.sub8(cpu.r8(0x581b6fu32), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42bcc4; continue; }
  pc = 0x42bcc3; continue;
 }
 0x42bcc3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42bcc4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bccc); }
 let mut pc: u32 = 0x42bccc;
 loop { match pc {
 0x42bccc => {
  { cpu.sub8(cpu.r8(0x581b66u32), 0x0u8); }
  if cpu.zf { pc = 0x42bcfb; continue; }
  pc = 0x42bcd5; continue;
 }
 0x42bcd5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x42bce2); f_0042c853(cpu); if cpu.returned_to != 0x42bce2 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42bcf4; continue; }
  pc = 0x42bceb; continue;
 }
 0x42bceb => {
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42bcfb; continue; }
  pc = 0x42bcf4; continue;
 }
 0x42bcf4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b66u32, v); }
  pc = 0x42bcfb; continue;
 }
 0x42bcfb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bcfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bcfc); }
 let mut pc: u32 = 0x42bcfc;
 loop { match pc {
 0x42bcfc => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42bd2c; continue; }
  pc = 0x42bd05; continue;
 }
 0x42bd05 => {
  { cpu.sub8(cpu.r8(0x581b70u32), 0x0u8); }
  if cpu.zf { pc = 0x42bd2c; continue; }
  pc = 0x42bd0e; continue;
 }
 0x42bd0e => {
  cpu.push(0x42bd13); f_0042bf55(cpu); if cpu.returned_to != 0x42bd13 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42bd25; continue; }
  pc = 0x42bd1c; continue;
 }
 0x42bd1c => {
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42bd2c; continue; }
  pc = 0x42bd25; continue;
 }
 0x42bd25 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b70u32, v); }
  pc = 0x42bd2c; continue;
 }
 0x42bd2c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bd2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bd2d); }
 let mut pc: u32 = 0x42bd2d;
 loop { match pc {
 0x42bd2d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x42bd37; continue; }
  pc = 0x42bd36; continue;
 }
 0x42bd36 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42bd37 => {
  { cpu.sub8(cpu.r8(0x5824a9u32), 0x0u8); }
  if cpu.zf { pc = 0x42bd7b; continue; }
  pc = 0x42bd40; continue;
 }
 0x42bd40 => {
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d68aeu32, v); }
  { let v: u8 = 0xf1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42bd62); f_00413337(cpu); if cpu.returned_to != 0x42bd62 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42bda6; continue; }
  pc = 0x42bd72; continue;
 }
 0x42bd72 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5824a9u32, v); }
  pc = 0x42bda6; continue;
 }
 0x42bd7b => {
  { let v: u8 = cpu.r8(0x581b76u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42bda6; continue; }
  pc = 0x42bd88; continue;
 }
 0x42bd88 => {
  { let v: u32 = cpu.r32(0x5824abu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4b94d0u32), cpu.eax); }
  if cpu.cf { pc = 0x42bda6; continue; }
  pc = 0x42bd95; continue;
 }
 0x42bd95 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5824a9u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x13ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5824abu32, v); }
  pc = 0x42bda6; continue;
 }
 0x42bda6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bda7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bda7); }
 let mut pc: u32 = 0x42bda7;
 loop { match pc {
 0x42bda7 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x42bdb1; continue; }
  pc = 0x42bdb0; continue;
 }
 0x42bdb0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42bdb1 => {
  { cpu.sub32(cpu.r32(0x59f452u32), 0x0u32); }
  if cpu.zf { pc = 0x42bdfa; continue; }
  pc = 0x42bdba; continue;
 }
 0x42bdba => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d68aeu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68b0u32, v); }
  { let v: u8 = 0xe9u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42bde0); f_00413337(cpu); if cpu.returned_to != 0x42bde0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42bdfa; continue; }
  pc = 0x42bdf0; continue;
 }
 0x42bdf0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59f452u32, v); }
  pc = 0x42bdfa; continue;
 }
 0x42bdfa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bdfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bdfb); }
 let mut pc: u32 = 0x42bdfb;
 loop { match pc {
 0x42bdfb => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x42be05; continue; }
  pc = 0x42be04; continue;
 }
 0x42be04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42be05 => {
  { cpu.sub8(cpu.r8(0x5824e6u32), 0x0u8); }
  if cpu.zf { pc = 0x42be42; continue; }
  pc = 0x42be0e; continue;
 }
 0x42be0e => {
  { let v: u32 = cpu.r32(0x5820f1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68aeu32, v); }
  { let v: u8 = 0xf3u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42be2b); f_00413337(cpu); if cpu.returned_to != 0x42be2b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42be42; continue; }
  pc = 0x42be3b; continue;
 }
 0x42be3b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5824e6u32, v); }
  pc = 0x42be42; continue;
 }
 0x42be42 => {
  { cpu.sub8(cpu.r8(0x5820f5u32), 0x0u8); }
  if cpu.zf { pc = 0x42be97; continue; }
  pc = 0x42be4b; continue;
 }
 0x42be4b => {
  cpu.push(0x42be50); f_004372be(cpu); if cpu.returned_to != 0x42be50 { return; }
  { cpu.sub8(cpu.r8(0x5820f5u32), 0x0u8); }
  if !cpu.sf { pc = 0x42be83; continue; }
  pc = 0x42be59; continue;
 }
 0x42be59 => {
  { let v: u8 = 0xf2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42be6c); f_00413337(cpu); if cpu.returned_to != 0x42be6c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x42be97; continue; }
  pc = 0x42be7c; continue;
 }
 0x42be7c => {
  { let v: u8 = 0x2u8; cpu.w8(0x5820f5u32, v); }
  pc = 0x42be83; continue;
 }
 0x42be83 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5820f5u32)); cpu.w8(0x5820f5u32, v); }
  if !cpu.zf { pc = 0x42be95; continue; }
  pc = 0x42be8b; continue;
 }
 0x42be8b => {
  cpu.push(0x42be90); f_00414e29(cpu); if cpu.returned_to != 0x42be90 { return; }
  cpu.push(0x42be95); f_0041534f(cpu); if cpu.returned_to != 0x42be95 { return; }
  pc = 0x42be95; continue;
 }
 0x42be95 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42be97 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042be99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42be99); }
 let mut pc: u32 = 0x42be99;
 loop { match pc {
 0x42be99 => {
  { let v: u32 = cpu.r32(0x4d68aeu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5820f1u32)); }
  if cpu.zf { pc = 0x42bece; continue; }
  pc = 0x42bea6; continue;
 }
 0x42bea6 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5820f5u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5820f6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x42bec1); f_00402d70(cpu); if cpu.returned_to != 0x42bec1 { return; }
  cpu.push(0x42bec6); f_00405320(cpu); if cpu.returned_to != 0x42bec6 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d22au32, v); }
  pc = 0x42bece; continue;
 }
 0x42bece => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bf55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bf55); }
 let mut pc: u32 = 0x42bf55;
 loop { match pc {
 0x42bf55 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  cpu.push(0x42bf67); f_00421f5a(cpu); if cpu.returned_to != 0x42bf67 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = 0xf5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42bf80); f_00413337(cpu); if cpu.returned_to != 0x42bf80 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042bf88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42bf88); }
 let mut pc: u32 = 0x42bf88;
 loop { match pc {
 0x42bf88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x42bf95); f_00421f60(cpu); if cpu.returned_to != 0x42bf95 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x581b6fu32)); cpu.w8(0x581b6fu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d68aeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x42bfa7); f_004292fa(cpu); if cpu.returned_to != 0x42bfa7 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d68aeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x581b71u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x581b76u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x42bfcd; continue; }
  pc = 0x42bfbd; continue;
 }
 0x42bfbd => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x581b71u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x42bfbd; continue; }
  pc = 0x42bfc8; continue;
 }
 0x42bfc8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x581b76u32, v); }
  pc = 0x42bfcd; continue;
 }
 0x42bfcd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d68aeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x42bfd9); f_00466a97(cpu); if cpu.returned_to != 0x42bfd9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c1ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c1ec); }
 let mut pc: u32 = 0x42c1ec;
 loop { match pc {
 0x42c1ec => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c866u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c839u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  { let v: u32 = !cpu.r32(0x58c87au32); cpu.w32(0x58c87au32, v); }
  { let v: u32 = cpu.r32(0x58c87au32) | 0xb9u32; cpu.w32(0x58c87au32, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x200u32); }
  if cpu.zf { pc = 0x42c23c; continue; }
  pc = 0x42c222; continue;
 }
 0x42c222 => {
  { let v: u32 = cpu.r32(0x58c86au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c866u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c839u32, v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x42c23c; continue;
 }
 0x42c23c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x42c251; continue; }
  pc = 0x42c245; continue;
 }
 0x42c245 => {
  { let v: u32 = cpu.r32(0x58c87au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c872u32, v); }
  pc = 0x42c25b; continue;
 }
 0x42c251 => {
  { let v: u32 = cpu.r32(0x58c87au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c876u32, v); }
  pc = 0x42c25b; continue;
 }
 0x42c25b => {
  cpu.push(0x42c260); f_0041b345(cpu); if cpu.returned_to != 0x42c260 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c4c0); }
 let mut pc: u32 = 0x42c4c0;
 loop { match pc {
 0x42c4c0 => {
  cpu.push(0x42c4c5); f_004302c4(cpu); if cpu.returned_to != 0x42c4c5 { return; }
  cpu.push(0x42c4ca); f_004301f7(cpu); if cpu.returned_to != 0x42c4ca { return; }
  cpu.push(0x42c4cf); f_0044d980(cpu); if cpu.returned_to != 0x42c4cf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x42c4db); f_00471ec2(cpu); if cpu.returned_to != 0x42c4db { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c4dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c4dc); }
 let mut pc: u32 = 0x42c4dc;
 loop { match pc {
 0x42c4dc => {
  cpu.push(0x42c4e1); f_004302c4(cpu); if cpu.returned_to != 0x42c4e1 { return; }
  cpu.push(0x42c4e6); f_0042fad8(cpu); if cpu.returned_to != 0x42c4e6 { return; }
  cpu.push(0x42c4eb); f_0044d980(cpu); if cpu.returned_to != 0x42c4eb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x42c4f7); f_00471ec2(cpu); if cpu.returned_to != 0x42c4f7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c4f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c4f8); }
 let mut pc: u32 = 0x42c4f8;
 loop { match pc {
 0x42c4f8 => {
  cpu.push(0x42c4fd); f_004302c4(cpu); if cpu.returned_to != 0x42c4fd { return; }
  cpu.push(0x42c502); f_0042fc4a(cpu); if cpu.returned_to != 0x42c502 { return; }
  cpu.push(0x42c507); f_0044d980(cpu); if cpu.returned_to != 0x42c507 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x42c513); f_00471ec2(cpu); if cpu.returned_to != 0x42c513 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c514(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c514); }
 let mut pc: u32 = 0x42c514;
 loop { match pc {
 0x42c514 => {
  cpu.push(0x42c519); f_004302c4(cpu); if cpu.returned_to != 0x42c519 { return; }
  cpu.push(0x42c51e); f_0042fdbc(cpu); if cpu.returned_to != 0x42c51e { return; }
  cpu.push(0x42c523); f_0044d980(cpu); if cpu.returned_to != 0x42c523 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x42c52f); f_00471ec2(cpu); if cpu.returned_to != 0x42c52f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c853(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c853); }
 let mut pc: u32 = 0x42c853;
 loop { match pc {
 0x42c853 => {
  { let v: u8 = 0x5au8; cpu.w8(0x59f441u32, v); }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x42c896; continue; }
  pc = 0x42c870; continue;
 }
 0x42c870 => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), 0x8000u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = 0xfdu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x42c88f); f_00413337(cpu); if cpu.returned_to != 0x42c88f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  pc = 0x42c896; continue;
 }
 0x42c896 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c926); }
 let mut pc: u32 = 0x42c926;
 loop { match pc {
 0x42c926 => {
  { cpu.sub8(cpu.r8(0x59d9dbu32), 0x0u8); }
  if cpu.zf { pc = 0x42c9e4; continue; }
  pc = 0x42c933; continue;
 }
 0x42c933 => {
  { let v: u32 = cpu.r32(0x59d9f0u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x42c9e4; continue; }
  pc = 0x42c941; continue;
 }
 0x42c941 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x42c952; continue; }
  pc = 0x42c94e; continue;
 }
 0x42c94e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x42c952; continue;
 }
 0x42c952 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x42c9e4; continue; }
  pc = 0x42c95a; continue;
 }
 0x42c95a => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x42c9e4; continue; }
  pc = 0x42c963; continue;
 }
 0x42c963 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42c9db; continue; }
  pc = 0x42c98a; continue;
 }
 0x42c98a => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42c9db; continue; }
  pc = 0x42c99a; continue;
 }
 0x42c99a => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42c9db; continue; }
  pc = 0x42c9a0; continue;
 }
 0x42c9a0 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42c9db; continue; }
  pc = 0x42c9b0; continue;
 }
 0x42c9b0 => {
  { cpu.pushad(); }
  { let v: u32 = 0xfu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  let t = cpu.r32(0x4d6f54u32); cpu.push(0x42c9c2); cpu.call(t); if cpu.returned_to != 0x42c9c2 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x42c9d0); f_0041b6f1(cpu); if cpu.returned_to != 0x42c9d0 { return; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  cpu.push(0x42c9da); f_0041cbda(cpu); if cpu.returned_to != 0x42c9da { return; }
  { cpu.popad(); }
  pc = 0x42c9db; continue;
 }
 0x42c9db => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.poll(); pc = 0x42c952; continue;
 }
 0x42c9e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c9e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c9e5); }
 let mut pc: u32 = 0x42c9e5;
 loop { match pc {
 0x42c9e5 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042c9ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42c9ff); }
 let mut pc: u32 = 0x42c9ff;
 loop { match pc {
 0x42c9ff => {
  { let v: u32 = 0x4eab2cu32; cpu.esi = v; }
  cpu.push(0x42ca09); f_0047b0d0(cpu); if cpu.returned_to != 0x42ca09 { return; }
  { let v: u32 = 0x4eab44u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4eab34u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4eab44u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x4eab34u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  cpu.push(0x42ca3e); f_00450ef5(cpu); if cpu.returned_to != 0x42ca3e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042caaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42caaa); }
 let mut pc: u32 = 0x42caaa;
 loop { match pc {
 0x42caaa => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4e1fccu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x42cad9; continue; }
  pc = 0x42cabb; continue;
 }
 0x42cabb => {
  { let v: u32 = cpu.r32(0x5b3980u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  cpu.push(0x42cac7); f_00419d49(cpu); if cpu.returned_to != 0x42cac7 { return; }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4d675cu32), 0x20u16); cpu.w16(0x4d675cu32, v); }
  pc = 0x42cb16; continue;
 }
 0x42cad9 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42caea; continue; }
  pc = 0x42cae3; continue;
 }
 0x42cae3 => {
  cpu.push(0x42cae8); f_0041b24f(cpu); if cpu.returned_to != 0x42cae8 { return; }
  pc = 0x42cb16; continue;
 }
 0x42caea => {
  { cpu.sub16(cpu.r16(0x4d688au32), 0x0u16); }
  if cpu.zf { pc = 0x42cb16; continue; }
  pc = 0x42caf4; continue;
 }
 0x42caf4 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42cb16; continue; }
  pc = 0x42cafd; continue;
 }
 0x42cafd => {
  { let v: u16 = cpu.r16(0x4d688au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x42cb0f); f_0041a8a2(cpu); if cpu.returned_to != 0x42cb0f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  pc = 0x42cb16; continue;
 }
 0x42cb16 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cb32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cb32); }
 let mut pc: u32 = 0x42cb32;
 loop { match pc {
 0x42cb32 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42cb52); f_004712e3(cpu); if cpu.returned_to != 0x42cb52 { return; }
  if !cpu.cf { pc = 0x42cb55; continue; }
  pc = 0x42cb54; continue;
 }
 0x42cb54 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42cb55 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f9fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cb5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cb5d); }
 let mut pc: u32 = 0x42cb5d;
 loop { match pc {
 0x42cb5d => {
  cpu.push(0x42cb62); f_00450c16(cpu); if cpu.returned_to != 0x42cb62 { return; }
  { let v: u32 = 0x4b7d84u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x42cb6c; continue;
 }
 0x42cb6c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x42cb8f; continue; }
  pc = 0x42cb73; continue;
 }
 0x42cb73 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x25u8); }
  if cpu.zf { pc = 0x42cb8d; continue; }
  pc = 0x42cb85; continue;
 }
 0x42cb85 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x42cb8d); f_00448329(cpu); if cpu.returned_to != 0x42cb8d { return; }
  pc = 0x42cb8d; continue;
 }
 0x42cb8d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x42cb8f; continue;
 }
 0x42cb8f => {
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42cb6c; continue; }
  pc = 0x42cb94; continue;
 }
 0x42cb94 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cb95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cb95); }
 let mut pc: u32 = 0x42cb95;
 loop { match pc {
 0x42cb95 => {
  { cpu.sub8(cpu.r8(0x4e1fa9u32), 0x2u8); }
  if cpu.zf { pc = 0x42cbbc; continue; }
  pc = 0x42cb9e; continue;
 }
 0x42cb9e => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x4u8); }
  if cpu.zf { pc = 0x42cbbc; continue; }
  pc = 0x42cba7; continue;
 }
 0x42cba7 => {
  { cpu.sub8(cpu.r8(0x4e1faau32), 0x0u8); }
  if !cpu.zf { pc = 0x42cbbc; continue; }
  pc = 0x42cbb0; continue;
 }
 0x42cbb0 => {
  cpu.push(0x42cbb5); f_0042caaa(cpu); if cpu.returned_to != 0x42cbb5 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1faau32, v); }
  pc = 0x42cbbc; continue;
 }
 0x42cbbc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ccb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ccb1); }
 let mut pc: u32 = 0x42ccb1;
 loop { match pc {
 0x42ccb1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  cpu.push(0x42ccca); f_0042cd2e(cpu); if cpu.returned_to != 0x42ccca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
