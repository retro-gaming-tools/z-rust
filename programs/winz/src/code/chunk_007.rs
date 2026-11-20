#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00452584(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452584); }
 let mut pc: u32 = 0x452584;
 loop { match pc {
 0x452584 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x45259b); f_0047957f(cpu); if cpu.returned_to != 0x45259b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x4525b4); f_00481755(cpu); if cpu.returned_to != 0x4525b4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4525bd); f_00451950(cpu); if cpu.returned_to != 0x4525bd { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004525bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4525bf); }
 let mut pc: u32 = 0x4525bf;
 loop { match pc {
 0x4525bf => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x20u8); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x452791; continue; }
  pc = 0x4525e7; continue;
 }
 0x4525e7 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x452600; continue; }
  pc = 0x4525f4; continue;
 }
 0x4525f4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  pc = 0x452600; continue;
 }
 0x452600 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x452613; continue; }
  pc = 0x452606; continue;
 }
 0x452606 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x452613; continue; }
  pc = 0x45260d; continue;
 }
 0x45260d => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  pc = 0x452613; continue;
 }
 0x452613 => {
  cpu.push(0x452618); f_0047a294(cpu); if cpu.returned_to != 0x452618 { return; }
  cpu.push(0x45261d); f_00452584(cpu); if cpu.returned_to != 0x45261d { return; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x452791; continue; }
  pc = 0x452628; continue;
 }
 0x452628 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x452791; continue; }
  pc = 0x452633; continue;
 }
 0x452633 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x452647); f_00452792(cpu); if cpu.returned_to != 0x452647 { return; }
  if cpu.cf { pc = 0x452791; continue; }
  pc = 0x45264d; continue;
 }
 0x45264d => {
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x45266a; continue; }
  pc = 0x452660; continue;
 }
 0x452660 => {
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45266a; continue;
 }
 0x45266a => {
  cpu.push(0x45266f); f_004527fe(cpu); if cpu.returned_to != 0x45266f { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15bdu32, v); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x45268e); f_004527fe(cpu); if cpu.returned_to != 0x45268e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15bdu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x4526ae; continue; }
  pc = 0x45269d; continue;
 }
 0x45269d => {
  { let v: u8 = 0x41u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45272d; continue;
 }
 0x4526ae => {
  { let v: u8 = 0x5u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x75u32.wrapping_add(cpu.ebp)), 0x5cu8); }
  if (cpu.sf != cpu.of) { pc = 0x4526c9; continue; }
  pc = 0x4526ba; continue;
 }
 0x4526ba => {
  { let v: u8 = 0x39u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4526c9; continue;
 }
 0x4526c9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45271a; continue; }
  pc = 0x4526d0; continue;
 }
 0x4526d0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x452707; continue; }
  pc = 0x4526d7; continue;
 }
 0x4526d7 => {
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45272d; continue; }
  pc = 0x4526e1; continue;
 }
 0x4526e1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x4526f6; continue; }
  pc = 0x4526e7; continue;
 }
 0x4526e7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x4526f6; continue; }
  pc = 0x4526ed; continue;
 }
 0x4526ed => {
  { cpu.sub8(cpu.r8(0x56d188u32), 0x0u8); }
  if cpu.zf { pc = 0x45272d; continue; }
  pc = 0x4526f6; continue;
 }
 0x4526f6 => {
  { let v: u8 = 0x31u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45272d; continue;
 }
 0x452707 => {
  { let v: u8 = 0x47u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45272d; continue;
 }
 0x45271a => {
  { let v: u8 = 0x13u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4d6968u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45272d; continue;
 }
 0x45272d => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x452743); f_00452a38(cpu); if cpu.returned_to != 0x452743 { return; }
  if cpu.cf { pc = 0x452760; continue; }
  pc = 0x452745; continue;
 }
 0x452745 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x452760; continue; }
  pc = 0x45274b; continue;
 }
 0x45274b => {
  { let v: u8 = cpu.r8(0x77u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x452760; continue;
 }
 0x452760 => {
  cpu.push(0x452765); f_0045bbca(cpu); if cpu.returned_to != 0x452765 { return; }
  if !cpu.cf { pc = 0x452776; continue; }
  pc = 0x452767; continue;
 }
 0x452767 => {
  cpu.push(0x45276c); f_00452a56(cpu); if cpu.returned_to != 0x45276c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45277b; continue;
 }
 0x452776 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x55u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x45277b; continue;
 }
 0x45277b => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x452791; continue; }
  pc = 0x452787; continue;
 }
 0x452787 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x452791; continue;
 }
 0x452791 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452792); }
 let mut pc: u32 = 0x452792;
 loop { match pc {
 0x452792 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x4527fb; continue; }
  pc = 0x4527d0; continue;
 }
 0x4527d0 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5b3a58u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5b3a54u32, v); }
  { let v: u32 = cpu.shr32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5b3a54u32).wrapping_sub(cpu.edx); cpu.w32(0x5b3a54u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b3a54u32)); cpu.w32(0x5b3a54u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4527f8); f_00451ac6(cpu); if cpu.returned_to != 0x4527f8 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4527fb => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004527fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4527fe); }
 let mut pc: u32 = 0x4527fe;
 loop { match pc {
 0x4527fe => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x452808; continue;
 }
 0x452808 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x45281c; continue;
 }
 0x45281c => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x452977; continue; }
  pc = 0x452829; continue;
 }
 0x452829 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x452833; continue;
 }
 0x452833 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x26u8); }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x45283d; continue;
 }
 0x45283d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x452987; continue; }
  pc = 0x452847; continue;
 }
 0x452847 => {
  { cpu.pushad(); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x45285c); f_004782f8(cpu); if cpu.returned_to != 0x45285c { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x452977; continue; }
  pc = 0x452863; continue;
 }
 0x452863 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x452979; continue; }
  pc = 0x45286d; continue;
 }
 0x45286d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x45287a; continue;
 }
 0x45287a => {
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x452896; continue; }
  pc = 0x452884; continue;
 }
 0x452884 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x52bc03u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x452896; continue;
 }
 0x452896 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4528aa; continue; }
  pc = 0x45289f; continue;
 }
 0x45289f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x4528b7; continue; }
  pc = 0x4528aa; continue;
 }
 0x4528aa => {
  { cpu.sub8(cpu.r8(0x52bc2du32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x452977; continue; }
  pc = 0x4528b7; continue;
 }
 0x4528b7 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if !cpu.zf { pc = 0x452977; continue; }
  pc = 0x4528c1; continue;
 }
 0x4528c1 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4528ed; continue; }
  pc = 0x4528ca; continue;
 }
 0x4528ca => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x4528f7; continue; }
  pc = 0x4528d4; continue;
 }
 0x4528d4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4528de); f_004184ca(cpu); if cpu.returned_to != 0x4528de { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x452977; continue; }
  pc = 0x4528eb; continue;
 }
 0x4528eb => {
  pc = 0x4528f7; continue;
 }
 0x4528ed => {
  { cpu.sub8(cpu.r8(0x6cu32.wrapping_add(cpu.esi)), 0x55u8); }
  if cpu.cf { pc = 0x452977; continue; }
  pc = 0x4528f7; continue;
 }
 0x4528f7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x452901); f_00489968(cpu); if cpu.returned_to != 0x452901 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x45290f; continue; }
  pc = 0x45290a; continue;
 }
 0x45290a => {
  cpu.push(0x45290f); f_0045427c(cpu); if cpu.returned_to != 0x45290f { return; }
  pc = 0x45290f; continue;
 }
 0x45290f => {
  { let v: u8 = 0xfbu8; cpu.w8(0x5au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x66u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x4e15bdu32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.esi)) | 0x80u8; cpu.w8(0x67u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x452945; continue; }
  pc = 0x452933; continue;
 }
 0x452933 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x45293b); f_00430114(cpu); if cpu.returned_to != 0x45293b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x452945; continue; }
  pc = 0x45293e; continue;
 }
 0x45293e => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97bu32, v); }
  pc = 0x452945; continue;
 }
 0x452945 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x45294d); f_0045955b(cpu); if cpu.returned_to != 0x45294d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45295b); f_004679cb(cpu); if cpu.returned_to != 0x45295b { return; }
  if !cpu.cf { pc = 0x452970; continue; }
  pc = 0x45295d; continue;
 }
 0x45295d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x452970; continue; }
  pc = 0x452964; continue;
 }
 0x452964 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x452970; continue; }
  pc = 0x45296b; continue;
 }
 0x45296b => {
  cpu.push(0x452970); f_0041674d(cpu); if cpu.returned_to != 0x452970 { return; }
  pc = 0x452970; continue;
 }
 0x452970 => {
  { cpu.popad(); }
  cpu.push(0x452976); f_0047957f(cpu); if cpu.returned_to != 0x452976 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452977 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452979 => {
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x452980); f_00453ff6(cpu); if cpu.returned_to != 0x452980 { return; }
  { cpu.popad(); }
  cpu.push(0x452986); f_0047957f(cpu); if cpu.returned_to != 0x452986 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452987 => {
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x45298e); f_00453ff6(cpu); if cpu.returned_to != 0x45298e { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x452998); f_0047957f(cpu); if cpu.returned_to != 0x452998 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452a38); }
 let mut pc: u32 = 0x452a38;
 loop { match pc {
 0x452a38 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x452a54; continue; }
  pc = 0x452a3e; continue;
 }
 0x452a3e => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452a54 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452a56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452a56); }
 let mut pc: u32 = 0x452a56;
 loop { match pc {
 0x452a56 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x452a72; continue; }
  pc = 0x452a6e; continue;
 }
 0x452a6e => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x452a72; continue;
 }
 0x452a72 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x452ae2; continue; }
  pc = 0x452a7b; continue;
 }
 0x452a7b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x39u8); }
  if cpu.zf { pc = 0x452ae2; continue; }
  pc = 0x452a82; continue;
 }
 0x452a82 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x452aa0; continue; }
  pc = 0x452a95; continue;
 }
 0x452a95 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x452aa0; continue; }
  pc = 0x452a9e; continue;
 }
 0x452a9e => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x452aa0; continue;
 }
 0x452aa0 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x452aac; continue;
 }
 0x452aac => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x452ab9; continue; }
  pc = 0x452ab6; continue;
 }
 0x452ab6 => {
  { let v: u16 = cpu.shl16((cpu.edi as u16), 1u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x452ab9; continue;
 }
 0x452ab9 => {
  cpu.push(0x452abe); f_0045bd38(cpu); if cpu.returned_to != 0x452abe { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x452ac5); f_00481755(cpu); if cpu.returned_to != 0x452ac5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452ace); f_00451950(cpu); if cpu.returned_to != 0x452ace { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452ae2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x452ae9); f_00458258(cpu); if cpu.returned_to != 0x452ae9 { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 2u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x452b24; continue; }
  pc = 0x452af8; continue;
 }
 0x452af8 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x452b0e; continue; }
  pc = 0x452b03; continue;
 }
 0x452b03 => {
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 2u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x452b0e; continue; }
  pc = 0x452b0c; continue;
 }
 0x452b0c => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x452b0e; continue;
 }
 0x452b0e => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x452b24; continue; }
  pc = 0x452b21; continue;
 }
 0x452b21 => {
  { let v: u32 = cpu.shr32(cpu.edi, 2u32); cpu.edi = v; }
  pc = 0x452b24; continue;
 }
 0x452b24 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x452aac; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452b28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452b28); }
 let mut pc: u32 = 0x452b28;
 loop { match pc {
 0x452b28 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x452b95; continue; }
  pc = 0x452b2e; continue;
 }
 0x452b2e => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x452b96; continue; }
  pc = 0x452b3e; continue;
 }
 0x452b3e => {
  if !cpu.zf { pc = 0x452b95; continue; }
  pc = 0x452b40; continue;
 }
 0x452b40 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x452c4f; continue; }
  pc = 0x452b4b; continue;
 }
 0x452b4b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x452b5c); f_004679cb(cpu); if cpu.returned_to != 0x452b5c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x452b73; continue; }
  pc = 0x452b60; continue;
 }
 0x452b60 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59dadau32, v); }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59dad8u32, v); }
  pc = 0x452b73; continue;
 }
 0x452b73 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1c1u32, v); }
  cpu.push(0x452b7f); f_0045844b(cpu); if cpu.returned_to != 0x452b7f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1c1u32, v); }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452b95); f_004824fa(cpu); if cpu.returned_to != 0x452b95 { return; }
  pc = 0x452b95; continue;
 }
 0x452b95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452b96 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x452c3d; continue; }
  pc = 0x452bab; continue;
 }
 0x452bab => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x452c3d; continue; }
  pc = 0x452bb5; continue;
 }
 0x452bb5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x452c3d; continue; }
  pc = 0x452bd2; continue;
 }
 0x452bd2 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x452c24; continue; }
  pc = 0x452bd9; continue;
 }
 0x452bd9 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x452bec); f_0045bc6e(cpu); if cpu.returned_to != 0x452bec { return; }
  if cpu.cf { pc = 0x452c38; continue; }
  pc = 0x452bee; continue;
 }
 0x452bee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x452bf9); f_0048c288(cpu); if cpu.returned_to != 0x452bf9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x452c3d; continue; }
  pc = 0x452bfd; continue;
 }
 0x452bfd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x452c1b); f_00489e05(cpu); if cpu.returned_to != 0x452c1b { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x452c24; continue; }
  pc = 0x452c1e; continue;
 }
 0x452c1e => {
  cpu.push(0x452c23); f_0045300a(cpu); if cpu.returned_to != 0x452c23 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452c24 => {
  cpu.push(0x452c29); f_00452d5a(cpu); if cpu.returned_to != 0x452c29 { return; }
  if !cpu.cf { pc = 0x452c2c; continue; }
  pc = 0x452c2b; continue;
 }
 0x452c2b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452c2c => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  cpu.push(0x452c37); f_0045302a(cpu); if cpu.returned_to != 0x452c37 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452c38 => {
  cpu.push(0x452c3d); f_00454204(cpu); if cpu.returned_to != 0x452c3d { return; }
  pc = 0x452c3d; continue;
 }
 0x452c3d => {
  cpu.push(0x452c42); f_0043ffdb(cpu); if cpu.returned_to != 0x452c42 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x452c4e; continue; }
  pc = 0x452c49; continue;
 }
 0x452c49 => {
  cpu.push(0x452c4e); f_0043fced(cpu); if cpu.returned_to != 0x452c4e { return; }
  pc = 0x452c4e; continue;
 }
 0x452c4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452c4f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452c59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452c59); }
 let mut pc: u32 = 0x452c59;
 loop { match pc {
 0x452c59 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x452c99; continue; }
  pc = 0x452c5f; continue;
 }
 0x452c5f => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x452c9a; continue; }
  pc = 0x452c6f; continue;
 }
 0x452c6f => {
  if !cpu.zf { pc = 0x452c99; continue; }
  pc = 0x452c71; continue;
 }
 0x452c71 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x452d50; continue; }
  pc = 0x452c7c; continue;
 }
 0x452c7c => {
  cpu.push(0x452c81); f_0045844b(cpu); if cpu.returned_to != 0x452c81 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452c90); f_004824fa(cpu); if cpu.returned_to != 0x452c90 { return; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x452c99; continue; }
  pc = 0x452c94; continue;
 }
 0x452c94 => {
  { let v: u8 = 0x39u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x452c99; continue;
 }
 0x452c99 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452c9a => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x452d3e; continue; }
  pc = 0x452caa; continue;
 }
 0x452caa => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x452d3e; continue; }
  pc = 0x452cb4; continue;
 }
 0x452cb4 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x452d3e; continue; }
  pc = 0x452cd1; continue;
 }
 0x452cd1 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x452d25; continue; }
  pc = 0x452cd8; continue;
 }
 0x452cd8 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x452ceb); f_0045bc6e(cpu); if cpu.returned_to != 0x452ceb { return; }
  if cpu.cf { pc = 0x452d39; continue; }
  pc = 0x452ced; continue;
 }
 0x452ced => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x452cf9); f_0048c288(cpu); if cpu.returned_to != 0x452cf9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x452d3e; continue; }
  pc = 0x452cfe; continue;
 }
 0x452cfe => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x452d1c); f_00489e05(cpu); if cpu.returned_to != 0x452d1c { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x452d25; continue; }
  pc = 0x452d1f; continue;
 }
 0x452d1f => {
  cpu.push(0x452d24); f_0045300a(cpu); if cpu.returned_to != 0x452d24 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452d25 => {
  cpu.push(0x452d2a); f_00452d5a(cpu); if cpu.returned_to != 0x452d2a { return; }
  if !cpu.cf { pc = 0x452d2d; continue; }
  pc = 0x452d2c; continue;
 }
 0x452d2c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452d2d => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  cpu.push(0x452d38); f_0045302a(cpu); if cpu.returned_to != 0x452d38 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452d39 => {
  cpu.push(0x452d3e); f_00454204(cpu); if cpu.returned_to != 0x452d3e { return; }
  pc = 0x452d3e; continue;
 }
 0x452d3e => {
  cpu.push(0x452d43); f_0043ffdb(cpu); if cpu.returned_to != 0x452d43 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x452d4f; continue; }
  pc = 0x452d4a; continue;
 }
 0x452d4a => {
  cpu.push(0x452d4f); f_0043fced(cpu); if cpu.returned_to != 0x452d4f { return; }
  pc = 0x452d4f; continue;
 }
 0x452d4f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452d50 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452d5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452d5a); }
 let mut pc: u32 = 0x452d5a;
 loop { match pc {
 0x452d5a => {
  { cpu.sub8(cpu.r8(0x9au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x452d7d; continue; }
  pc = 0x452d64; continue;
 }
 0x452d64 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x452d72); f_0045300a(cpu); if cpu.returned_to != 0x452d72 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x6u8; cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452d7d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452d7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452d7f); }
 let mut pc: u32 = 0x452d7f;
 loop { match pc {
 0x452d7f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x452dbb; continue; }
  pc = 0x452d85; continue;
 }
 0x452d85 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x452dbc; continue; }
  pc = 0x452d91; continue;
 }
 0x452d91 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x452e72; continue; }
  pc = 0x452d9c; continue;
 }
 0x452d9c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x452dbb; continue; }
  pc = 0x452da7; continue;
 }
 0x452da7 => {
  cpu.push(0x452dac); f_0045850c(cpu); if cpu.returned_to != 0x452dac { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452dbb); f_004824fa(cpu); if cpu.returned_to != 0x452dbb { return; }
  pc = 0x452dbb; continue;
 }
 0x452dbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452dbc => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x452e60; continue; }
  pc = 0x452dcc; continue;
 }
 0x452dcc => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x452e60; continue; }
  pc = 0x452dd6; continue;
 }
 0x452dd6 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x452e60; continue; }
  pc = 0x452df3; continue;
 }
 0x452df3 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x452e47; continue; }
  pc = 0x452dfa; continue;
 }
 0x452dfa => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x452e0d); f_0045bc6e(cpu); if cpu.returned_to != 0x452e0d { return; }
  if cpu.cf { pc = 0x452e5b; continue; }
  pc = 0x452e0f; continue;
 }
 0x452e0f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x452e1b); f_0048c288(cpu); if cpu.returned_to != 0x452e1b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x452e60; continue; }
  pc = 0x452e20; continue;
 }
 0x452e20 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x452e3e); f_00489e05(cpu); if cpu.returned_to != 0x452e3e { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x452e47; continue; }
  pc = 0x452e41; continue;
 }
 0x452e41 => {
  cpu.push(0x452e46); f_0045300a(cpu); if cpu.returned_to != 0x452e46 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452e47 => {
  cpu.push(0x452e4c); f_00452d5a(cpu); if cpu.returned_to != 0x452e4c { return; }
  if !cpu.cf { pc = 0x452e4f; continue; }
  pc = 0x452e4e; continue;
 }
 0x452e4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452e4f => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  cpu.push(0x452e5a); f_0045302a(cpu); if cpu.returned_to != 0x452e5a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452e5b => {
  cpu.push(0x452e60); f_00454204(cpu); if cpu.returned_to != 0x452e60 { return; }
  pc = 0x452e60; continue;
 }
 0x452e60 => {
  cpu.push(0x452e65); f_0043ffdb(cpu); if cpu.returned_to != 0x452e65 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x452e71; continue; }
  pc = 0x452e6c; continue;
 }
 0x452e6c => {
  cpu.push(0x452e71); f_0043fced(cpu); if cpu.returned_to != 0x452e71 { return; }
  pc = 0x452e71; continue;
 }
 0x452e71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452e72 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452e78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452e78); }
 let mut pc: u32 = 0x452e78;
 loop { match pc {
 0x452e78 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x452f04; continue; }
  pc = 0x452e82; continue;
 }
 0x452e82 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x452f04; continue; }
  pc = 0x452e92; continue;
 }
 0x452e92 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x452ebf; continue; }
  pc = 0x452e94; continue;
 }
 0x452e94 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x452f3d; continue; }
  pc = 0x452e9f; continue;
 }
 0x452e9f => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452ea9); f_004585d0(cpu); if cpu.returned_to != 0x452ea9 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452eb8); f_004824fa(cpu); if cpu.returned_to != 0x452eb8 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x452f04; continue;
 }
 0x452ebf => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp))); cpu.w8(0x9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x9cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x452eeb; continue; }
  pc = 0x452eda; continue;
 }
 0x452eda => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x452ee7; continue; }
  pc = 0x452ee5; continue;
 }
 0x452ee5 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x452ee7; continue;
 }
 0x452ee7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x452eeb; continue;
 }
 0x452eeb => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x452f05; continue; }
  pc = 0x452ef7; continue;
 }
 0x452ef7 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x452f07; continue; }
  pc = 0x452f00; continue;
 }
 0x452f00 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x452f04; continue;
 }
 0x452f04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452f05 => {
  pc = 0x452f42; continue;
 }
 0x452f07 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x452f16); f_00453b70(cpu); if cpu.returned_to != 0x452f16 { return; }
  if !cpu.cf { pc = 0x452f28; continue; }
  pc = 0x452f18; continue;
 }
 0x452f18 => {
  { cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x452f27; continue; }
  pc = 0x452f22; continue;
 }
 0x452f22 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x452f27; continue;
 }
 0x452f27 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452f28 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  cpu.push(0x452f32); f_0046ab56(cpu); if cpu.returned_to != 0x452f32 { return; }
  cpu.push(0x452f37); f_00489968(cpu); if cpu.returned_to != 0x452f37 { return; }
  cpu.push(0x452f3c); f_0043ffdb(cpu); if cpu.returned_to != 0x452f3c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452f3d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452f42 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x453004; continue; }
  pc = 0x452f57; continue;
 }
 0x452f57 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x453004; continue; }
  pc = 0x452f61; continue;
 }
 0x452f61 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x453004; continue; }
  pc = 0x452f82; continue;
 }
 0x452f82 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x452fd6; continue; }
  pc = 0x452f89; continue;
 }
 0x452f89 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x452f9c); f_0045bc6e(cpu); if cpu.returned_to != 0x452f9c { return; }
  if cpu.cf { pc = 0x453004; continue; }
  pc = 0x452f9e; continue;
 }
 0x452f9e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x452faa); f_0048c288(cpu); if cpu.returned_to != 0x452faa { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x453004; continue; }
  pc = 0x452faf; continue;
 }
 0x452faf => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x452fcd); f_00489e05(cpu); if cpu.returned_to != 0x452fcd { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x452fd6; continue; }
  pc = 0x452fd0; continue;
 }
 0x452fd0 => {
  cpu.push(0x452fd5); f_0045300a(cpu); if cpu.returned_to != 0x452fd5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452fd6 => {
  cpu.push(0x452fdb); f_00452d5a(cpu); if cpu.returned_to != 0x452fdb { return; }
  if !cpu.cf { pc = 0x452fde; continue; }
  pc = 0x452fdd; continue;
 }
 0x452fdd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452fde => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x452ff4; continue; }
  pc = 0x452fee; continue;
 }
 0x452fee => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452ff4 => {
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x452ffe); f_0045302a(cpu); if cpu.returned_to != 0x452ffe { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453004 => {
  cpu.push(0x453009); f_0043ffdb(cpu); if cpu.returned_to != 0x453009 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045300a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45300a); }
 let mut pc: u32 = 0x45300a;
 loop { match pc {
 0x45300a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x453017); f_00458a0c(cpu); if cpu.returned_to != 0x453017 { return; }
  cpu.push(0x45301c); f_0048990c(cpu); if cpu.returned_to != 0x45301c { return; }
  cpu.push(0x453021); f_004895be(cpu); if cpu.returned_to != 0x453021 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045302a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45302a); }
 let mut pc: u32 = 0x45302a;
 loop { match pc {
 0x45302a => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x45303f); f_004527fe(cpu); if cpu.returned_to != 0x45303f { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x453083; continue; }
  pc = 0x453045; continue;
 }
 0x453045 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x453083; continue; }
  pc = 0x45304b; continue;
 }
 0x45304b => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x45305b); f_0041c743(cpu); if cpu.returned_to != 0x45305b { return; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  pc = 0x45306d; continue;
 }
 0x45306d => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x453078; continue; }
  pc = 0x453072; continue;
 }
 0x453072 => {
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x45307b; continue; }
  pc = 0x453076; continue;
 }
 0x453076 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x453078; continue;
 }
 0x453078 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x45306d; continue; }
  pc = 0x45307a; continue;
 }
 0x45307a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45307b => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x453082); f_0045b5ab(cpu); if cpu.returned_to != 0x453082 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453083 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x453092; continue; }
  pc = 0x453089; continue;
 }
 0x453089 => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x453091); f_0045b5ab(cpu); if cpu.returned_to != 0x453091 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453092 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045309c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45309c); } r_0045309c(cpu, 0x45309c); }
pub fn f_004531a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531a0); } r_0045309c(cpu, 0x4531a0); }
pub fn f_004531a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531a9); } r_0045309c(cpu, 0x4531a9); }
pub fn f_004531b9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531b9); } r_0045309c(cpu, 0x4531b9); }
pub fn f_004531c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531c2); } r_0045309c(cpu, 0x4531c2); }
pub fn f_004531d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531d2); } r_0045309c(cpu, 0x4531d2); }
pub fn f_004531db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531db); } r_0045309c(cpu, 0x4531db); }
pub fn f_004531eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531eb); } r_0045309c(cpu, 0x4531eb); }
pub fn f_004531f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4531f4); } r_0045309c(cpu, 0x4531f4); }
pub fn f_004532dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532dc); } r_0045309c(cpu, 0x4532dc); }
pub fn f_004532e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532e1); } r_0045309c(cpu, 0x4532e1); }
pub fn f_004532e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532e9); } r_0045309c(cpu, 0x4532e9); }
pub fn f_004532ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532ee); } r_0045309c(cpu, 0x4532ee); }
pub fn f_004532f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532f6); } r_0045309c(cpu, 0x4532f6); }
pub fn f_004532fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4532fb); } r_0045309c(cpu, 0x4532fb); }
pub fn f_00453303(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x453303); } r_0045309c(cpu, 0x453303); }
pub fn f_00453308(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x453308); } r_0045309c(cpu, 0x453308); }
fn r_0045309c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45309c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x45320c; continue; }
  pc = 0x4530a7; continue;
 }
 0x4530a7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x21u8); }
  if cpu.zf { pc = 0x45320c; continue; }
  pc = 0x4530b2; continue;
 }
 0x4530b2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x45320c; continue; }
  pc = 0x4530bd; continue;
 }
 0x4530bd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x453208; continue; }
  pc = 0x4530c8; continue;
 }
 0x4530c8 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x45310f; continue; }
  pc = 0x4530dd; continue;
 }
 0x4530dd => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x45310f; continue; }
  pc = 0x4530e1; continue;
 }
 0x4530e1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x453107; continue; }
  pc = 0x4530e7; continue;
 }
 0x4530e7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x4530f5; continue; }
  pc = 0x4530f0; continue;
 }
 0x4530f0 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4530fa; continue; }
  pc = 0x4530f5; continue;
 }
 0x4530f5 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4530fa; continue;
 }
 0x4530fa => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x453208; continue; }
  pc = 0x453105; continue;
 }
 0x453105 => {
  pc = 0x45310f; continue;
 }
 0x453107 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x453208; continue; }
  pc = 0x45310f; continue;
 }
 0x45310f => {
  cpu.push(0x453114); f_0045bbca(cpu); if cpu.returned_to != 0x453114 { return; }
  if cpu.cf { pc = 0x453204; continue; }
  pc = 0x45311a; continue;
 }
 0x45311a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x3ffu16); }
  if (cpu.cf || cpu.zf) { pc = 0x453141; continue; }
  pc = 0x45313c; continue;
 }
 0x45313c => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x453141; continue;
 }
 0x453141 => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1f4eu32), 0x0u8); }
  if !cpu.zf { pc = 0x453165; continue; }
  pc = 0x45315e; continue;
 }
 0x45315e => {
  { let v: u8 = 0x8u8; cpu.w8(0x4e1f4eu32, v); }
  pc = 0x453165; continue;
 }
 0x453165 => {
  { let v = cpu.r8(0x4e1f4eu32); cpu.div8(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575f28u32, v); }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x453182; continue;
 }
 0x453182 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  cpu.poll(); pc = cpu.r32(0x5b3a5cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x453194 => {
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x45319c; continue; }
  pc = 0x453199; continue;
 }
 0x453199 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x453182; continue; }
  pc = 0x45319c; continue;
 }
 0x45319c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x453204; continue;
 }
 0x4531a0 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531a9 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531b9 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531c2 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531d2 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531db => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531eb => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x4531f4 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6938u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x453194; continue;
 }
 0x453204 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453208 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45320c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x453312; continue; }
  pc = 0x453217; continue;
 }
 0x453217 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x453269; continue; }
  pc = 0x45322c; continue;
 }
 0x45322c => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x453269; continue; }
  pc = 0x453230; continue;
 }
 0x453230 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x453261; continue; }
  pc = 0x453236; continue;
 }
 0x453236 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x453244; continue; }
  pc = 0x45323f; continue;
 }
 0x45323f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453249; continue; }
  pc = 0x453244; continue;
 }
 0x453244 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x453249; continue;
 }
 0x453249 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x453312; continue; }
  pc = 0x45325f; continue;
 }
 0x45325f => {
  pc = 0x453269; continue;
 }
 0x453261 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x453312; continue; }
  pc = 0x453269; continue;
 }
 0x453269 => {
  cpu.push(0x45326e); f_0045bbca(cpu); if cpu.returned_to != 0x45326e { return; }
  if cpu.cf { pc = 0x45330e; continue; }
  pc = 0x453274; continue;
 }
 0x453274 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x3ffu16); }
  if (cpu.cf || cpu.zf) { pc = 0x45329b; continue; }
  pc = 0x453296; continue;
 }
 0x453296 => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x45329b; continue;
 }
 0x45329b => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1f4eu32), 0x0u8); }
  if !cpu.zf { pc = 0x4532bf; continue; }
  pc = 0x4532b8; continue;
 }
 0x4532b8 => {
  { let v: u8 = 0x8u8; cpu.w8(0x4e1f4eu32, v); }
  pc = 0x4532bf; continue;
 }
 0x4532bf => {
  { let v = cpu.r8(0x4e1f4eu32); cpu.div8(v); }
  { let v = cpu.r8(0x4d6934u32); cpu.mul8(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x5b3a7cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x4532dc => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x4532e1 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x4532e9 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x4532ee => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x4532f6 => {
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x4532fb => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x453303 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x453308 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45330e; continue;
 }
 0x45330e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453312 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453316(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453316); }
 let mut pc: u32 = 0x453316;
 loop { match pc {
 0x453316 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d26u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d39u32, v); }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x4535eb; continue; }
  pc = 0x45332f; continue;
 }
 0x45332f => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x4538c6; continue; }
  pc = 0x45333a; continue;
 }
 0x45333a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x453351; continue; }
  pc = 0x453341; continue;
 }
 0x453341 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x453351; continue; }
  pc = 0x453348; continue;
 }
 0x453348 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x453351; continue; }
  pc = 0x45334f; continue;
 }
 0x45334f => {
  pc = 0x453356; continue;
 }
 0x453351 => {
  cpu.push(0x453356); f_0046b449(cpu); if cpu.returned_to != 0x453356 { return; }
  pc = 0x453356; continue;
 }
 0x453356 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x45336e); f_0046c3d4(cpu); if cpu.returned_to != 0x45336e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x4535e9; continue; }
  pc = 0x453379; continue;
 }
 0x453379 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xadu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4533aa; continue; }
  pc = 0x453388; continue;
 }
 0x453388 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45339b); f_004481e0(cpu); if cpu.returned_to != 0x45339b { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.cf { pc = 0x45358c; continue; }
  pc = 0x4533a5; continue;
 }
 0x4533a5 => {
  pc = 0x453574; continue;
 }
 0x4533aa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x4533b4); f_0046e362(cpu); if cpu.returned_to != 0x4533b4 { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.cf { pc = 0x45355c; continue; }
  pc = 0x4533be; continue;
 }
 0x4533be => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x453456; continue; }
  pc = 0x4533c9; continue;
 }
 0x4533c9 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if !cpu.zf { pc = 0x453400; continue; }
  pc = 0x4533d3; continue;
 }
 0x4533d3 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x4533f5); f_00453cd6(cpu); if cpu.returned_to != 0x4533f5 { return; }
  if cpu.cf { pc = 0x45358c; continue; }
  pc = 0x4533fb; continue;
 }
 0x4533fb => {
  pc = 0x453568; continue;
 }
 0x453400 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45340c); f_004712e3(cpu); if cpu.returned_to != 0x45340c { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x4534d7; continue; }
  pc = 0x453413; continue;
 }
 0x453413 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x453568; continue; }
  pc = 0x453439; continue;
 }
 0x453439 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45358c; continue; }
  pc = 0x453444; continue;
 }
 0x453444 => {
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x1u8); }
  if !cpu.zf { pc = 0x453574; continue; }
  pc = 0x453451; continue;
 }
 0x453451 => {
  pc = 0x45358c; continue;
 }
 0x453456 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45348a; continue; }
  pc = 0x45345d; continue;
 }
 0x45345d => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x453469); f_004712e3(cpu); if cpu.returned_to != 0x453469 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x45348a; continue; }
  pc = 0x45346c; continue;
 }
 0x45346c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x45348a; continue; }
  pc = 0x453475; continue;
 }
 0x453475 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.zf { pc = 0x4534d7; continue; }
  pc = 0x45348a; continue;
 }
 0x45348a => {
  { cpu.pushad(); }
  cpu.push(0x453490); f_00470fc5(cpu); if cpu.returned_to != 0x453490 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x4534a0; continue; }
  pc = 0x453493; continue;
 }
 0x453493 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x4534d7; continue; }
  pc = 0x45349b; continue;
 }
 0x45349b => {
  pc = 0x45358c; continue;
 }
 0x4534a0 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x4534c2); f_00453a11(cpu); if cpu.returned_to != 0x4534c2 { return; }
  if cpu.cf { pc = 0x45358c; continue; }
  pc = 0x4534c8; continue;
 }
 0x4534c8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x4534d1); f_004713ce(cpu); if cpu.returned_to != 0x4534d1 { return; }
  if cpu.cf { pc = 0x453580; continue; }
  pc = 0x4534d7; continue;
 }
 0x4534d7 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5b3d26u32), 0xffu8); }
  if cpu.zf { pc = 0x45352c; continue; }
  pc = 0x4534f2; continue;
 }
 0x4534f2 => {
  { cpu.sub8(cpu.r8(0x5b3d26u32), 0x1u8); }
  if !cpu.zf { pc = 0x453516; continue; }
  pc = 0x4534fb; continue;
 }
 0x4534fb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xf4u8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x453513); f_0041b794(cpu); if cpu.returned_to != 0x453513 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x45352c; continue;
 }
 0x453516 => {
  { cpu.sub8(cpu.r8(0x5b3d26u32), 0x2u8); }
  if !cpu.zf { pc = 0x453521; continue; }
  pc = 0x45351f; continue;
 }
 0x45351f => {
  pc = 0x45352c; continue;
 }
 0x453521 => {
  { cpu.sub8(cpu.r8(0x5b3d26u32), 0x3u8); }
  if !cpu.zf { pc = 0x45352c; continue; }
  pc = 0x45352a; continue;
 }
 0x45352a => {
  pc = 0x45352c; continue;
 }
 0x45352c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4536a8; continue; }
  pc = 0x453537; continue;
 }
 0x453537 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x453546; continue; }
  pc = 0x453541; continue;
 }
 0x453541 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x45354b; continue; }
  pc = 0x453546; continue;
 }
 0x453546 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x45354b; continue;
 }
 0x45354b => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x453559); f_00489968(cpu); if cpu.returned_to != 0x453559 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45355c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d26u32, v); }
  cpu.poll(); pc = 0x4534d7; continue;
 }
 0x453568 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5b3d26u32, v); }
  cpu.poll(); pc = 0x4534d7; continue;
 }
 0x453574 => {
  { let v: u8 = 0xffu8; cpu.w8(0x5b3d26u32, v); }
  cpu.poll(); pc = 0x4534d7; continue;
 }
 0x453580 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5b3d26u32, v); }
  cpu.poll(); pc = 0x4534d7; continue;
 }
 0x45358c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x45359d; continue; }
  pc = 0x453598; continue;
 }
 0x453598 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4535a2; continue; }
  pc = 0x45359d; continue;
 }
 0x45359d => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4535a2; continue;
 }
 0x4535a2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4535af; continue; }
  pc = 0x4535a9; continue;
 }
 0x4535a9 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x4535e9; continue; }
  pc = 0x4535af; continue;
 }
 0x4535af => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4535b9); f_0046c3d4(cpu); if cpu.returned_to != 0x4535b9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x4536a8; continue; }
  pc = 0x4535c4; continue;
 }
 0x4535c4 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4535d3; continue; }
  pc = 0x4535ce; continue;
 }
 0x4535ce => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4535d8; continue; }
  pc = 0x4535d3; continue;
 }
 0x4535d3 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x4535d8; continue;
 }
 0x4535d8 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4535e6); f_00489968(cpu); if cpu.returned_to != 0x4535e6 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4535e9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4535eb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x453660; continue; }
  pc = 0x453602; continue;
 }
 0x453602 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x453631; continue; }
  pc = 0x45362c; continue;
 }
 0x45362c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x45366e; continue;
 }
 0x453631 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x45366b; continue; }
  pc = 0x453641; continue;
 }
 0x453641 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x45365e; continue; }
  pc = 0x45364a; continue;
 }
 0x45364a => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x45366b; continue; }
  pc = 0x45365e; continue;
 }
 0x45365e => {
  pc = 0x453678; continue;
 }
 0x453660 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x453668); f_00454204(cpu); if cpu.returned_to != 0x453668 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x45366b; continue;
 }
 0x45366b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x45366e; continue;
 }
 0x45366e => {
  cpu.push(0x453673); f_004539d9(cpu); if cpu.returned_to != 0x453673 { return; }
  pc = 0x45394e; continue;
 }
 0x453678 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x453680); f_0045bc6e(cpu); if cpu.returned_to != 0x453680 { return; }
  if cpu.cf { pc = 0x4536a6; continue; }
  pc = 0x453682; continue;
 }
 0x453682 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x453706; continue; }
  pc = 0x453689; continue;
 }
 0x453689 => {
  { let v = cpu.r32(0x5d6d3cu32); cpu.push(v); }
  { let v: u32 = 0x4e9e8eu32; cpu.w32(0x5d6d3cu32, v); }
  cpu.push(0x45369e); f_00489e00(cpu); if cpu.returned_to != 0x45369e { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x5d6d3cu32, v); }
  if !cpu.cf { pc = 0x453706; continue; }
  pc = 0x4536a6; continue;
 }
 0x4536a6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4536a8; continue;
 }
 0x4536a8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x4536bd; continue; }
  pc = 0x4536af; continue;
 }
 0x4536af => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4536bd); f_00489968(cpu); if cpu.returned_to != 0x4536bd { return; }
  pc = 0x4536bd; continue;
 }
 0x4536bd => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x4536df; continue; }
  pc = 0x4536da; continue;
 }
 0x4536da => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4536e4; continue; }
  pc = 0x4536df; continue;
 }
 0x4536df => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4536e4; continue;
 }
 0x4536e4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x4536ff; continue; }
  pc = 0x4536eb; continue;
 }
 0x4536eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x4536fd); f_0046ab56(cpu); if cpu.returned_to != 0x4536fd { return; }
  pc = 0x453704; continue;
 }
 0x4536ff => {
  cpu.push(0x453704); f_0044b9aa(cpu); if cpu.returned_to != 0x453704 { return; }
  pc = 0x453704; continue;
 }
 0x453704 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453706 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b3d27u32, v); }
  cpu.push(0x453719); f_004539be(cpu); if cpu.returned_to != 0x453719 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x45399e; continue; }
  pc = 0x453723; continue;
 }
 0x453723 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x453952; continue; }
  pc = 0x45372e; continue;
 }
 0x45372e => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.r32(0x4d6934u32); cpu.push(v); }
  cpu.push(0x453747); f_00453f9b(cpu); if cpu.returned_to != 0x453747 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6934u32, v); }
  if cpu.cf { pc = 0x4537ad; continue; }
  pc = 0x45374f; continue;
 }
 0x45374f => {
  cpu.push(0x453754); f_0046c3d4(cpu); if cpu.returned_to != 0x453754 { return; }
  if !cpu.cf { pc = 0x4537b1; continue; }
  pc = 0x453756; continue;
 }
 0x453756 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x45376e; continue; }
  pc = 0x45375d; continue;
 }
 0x45375d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x453775; continue; }
  pc = 0x453764; continue;
 }
 0x453764 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x453775; continue; }
  pc = 0x45376e; continue;
 }
 0x45376e => {
  cpu.push(0x453773); f_00489e00(cpu); if cpu.returned_to != 0x453773 { return; }
  if cpu.cf { pc = 0x453799; continue; }
  pc = 0x453775; continue;
 }
 0x453775 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x453799; continue; }
  pc = 0x45377c; continue;
 }
 0x45377c => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x453799; continue; }
  pc = 0x453787; continue;
 }
 0x453787 => {
  { cpu.sub8(cpu.r8(0x5b3d39u32), 0x0u8); }
  if !cpu.zf { pc = 0x453799; continue; }
  pc = 0x453790; continue;
 }
 0x453790 => {
  cpu.push(0x453795); f_004539d9(cpu); if cpu.returned_to != 0x453795 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453799 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4537a7); f_00451ac6(cpu); if cpu.returned_to != 0x4537a7 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  pc = 0x4537ad; continue;
 }
 0x4537ad => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4537b1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x4537c1; continue; }
  pc = 0x4537bc; continue;
 }
 0x4537bc => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4537c6; continue; }
  pc = 0x4537c1; continue;
 }
 0x4537c1 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4537c6; continue;
 }
 0x4537c6 => {
  cpu.push(0x4537cb); f_0046c3d4(cpu); if cpu.returned_to != 0x4537cb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x4538b6; continue; }
  pc = 0x4537d3; continue;
 }
 0x4537d3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x4537f8; continue; }
  pc = 0x4537da; continue;
 }
 0x4537da => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x4537eb; continue; }
  pc = 0x4537e1; continue;
 }
 0x4537e1 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x4537f8; continue; }
  pc = 0x4537eb; continue;
 }
 0x4537eb => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4538b6; continue; }
  pc = 0x4537f6; continue;
 }
 0x4537f6 => {
  pc = 0x453877; continue;
 }
 0x4537f8 => {
  cpu.push(0x4537fd); f_00489e00(cpu); if cpu.returned_to != 0x4537fd { return; }
  if cpu.cf { pc = 0x453816; continue; }
  pc = 0x4537ff; continue;
 }
 0x4537ff => {
  { cpu.sub8(cpu.r8(0x9au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x453877; continue; }
  pc = 0x453809; continue;
 }
 0x453809 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  pc = 0x4538b6; continue;
 }
 0x453816 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4536a6; continue; }
  pc = 0x453829; continue;
 }
 0x453829 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x45384d; continue; }
  pc = 0x453830; continue;
 }
 0x453830 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x45383d; continue; }
  pc = 0x453838; continue;
 }
 0x453838 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453842; continue; }
  pc = 0x45383d; continue;
 }
 0x45383d => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x453842; continue;
 }
 0x453842 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x45384d; continue; }
  pc = 0x453847; continue;
 }
 0x453847 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x453858; continue; }
  pc = 0x45384d; continue;
 }
 0x45384d => {
  cpu.push(0x453852); f_0045309c(cpu); if cpu.returned_to != 0x453852 { return; }
  if !cpu.cf { pc = 0x453858; continue; }
  pc = 0x453854; continue;
 }
 0x453854 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453858 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x453854; continue; }
  pc = 0x45385f; continue;
 }
 0x45385f => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45386d); f_00451ac6(cpu); if cpu.returned_to != 0x45386d { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453877 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebx)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x45388c; continue; }
  pc = 0x453885; continue;
 }
 0x453885 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4538af; continue; }
  pc = 0x45388c; continue;
 }
 0x45388c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x45389b; continue; }
  pc = 0x453896; continue;
 }
 0x453896 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4538a0; continue; }
  pc = 0x45389b; continue;
 }
 0x45389b => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x4538a0; continue;
 }
 0x4538a0 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4538ae); f_00489968(cpu); if cpu.returned_to != 0x4538ae { return; }
  { cpu.popad(); }
  pc = 0x4538af; continue;
 }
 0x4538af => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cc6a0u32, v); }
  pc = 0x4538b6; continue;
 }
 0x4538b6 => {
  cpu.push(0x4538bb); f_0046b345(cpu); if cpu.returned_to != 0x4538bb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4538c2); f_0045309c(cpu); if cpu.returned_to != 0x4538c2 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4538c6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x45393b; continue; }
  pc = 0x453903; continue;
 }
 0x453903 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x453932; continue; }
  pc = 0x453913; continue;
 }
 0x453913 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x45393b; continue; }
  pc = 0x45391c; continue;
 }
 0x45391c => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x453932; continue; }
  pc = 0x453930; continue;
 }
 0x453930 => {
  pc = 0x45393b; continue;
 }
 0x453932 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x453939); f_004539d9(cpu); if cpu.returned_to != 0x453939 { return; }
  pc = 0x45394e; continue;
 }
 0x45393b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x453706; continue; }
  pc = 0x453947; continue;
 }
 0x453947 => {
  cpu.push(0x45394c); f_00454204(cpu); if cpu.returned_to != 0x45394c { return; }
  pc = 0x4539a6; continue;
 }
 0x45394e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453952 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x453976; continue; }
  pc = 0x453960; continue;
 }
 0x453960 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x453976; continue; }
  pc = 0x453966; continue;
 }
 0x453966 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x453976; continue; }
  pc = 0x45396c; continue;
 }
 0x45396c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x453976; continue; }
  pc = 0x453972; continue;
 }
 0x453972 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x453976; continue;
 }
 0x453976 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45399e => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x4539a6; continue;
 }
 0x4539a6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004539be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4539be); }
 let mut pc: u32 = 0x4539be;
 loop { match pc {
 0x4539be => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x4539cc; continue; }
  pc = 0x4539cc; continue;
 }
 0x4539cc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4539d8; continue; }
  pc = 0x4539d2; continue;
 }
 0x4539d2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x4539d8; continue; }
  pc = 0x4539d8; continue;
 }
 0x4539d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004539d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4539d9); }
 let mut pc: u32 = 0x4539d9;
 loop { match pc {
 0x4539d9 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x453a00; continue; }
  pc = 0x4539e0; continue;
 }
 0x4539e0 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4539ef); f_0044b9aa(cpu); if cpu.returned_to != 0x4539ef { return; }
  cpu.push(0x4539f4); f_00489968(cpu); if cpu.returned_to != 0x4539f4 { return; }
  cpu.push(0x4539f9); f_0043ffdb(cpu); if cpu.returned_to != 0x4539f9 { return; }
  cpu.push(0x4539fe); f_0043f830(cpu); if cpu.returned_to != 0x4539fe { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453a00 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x453a0a); f_00489968(cpu); if cpu.returned_to != 0x453a0a { return; }
  cpu.push(0x453a0f); f_0043ffdb(cpu); if cpu.returned_to != 0x453a0f { return; }
  cpu.poll(); pc = 0x4539d9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453a11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453a11); }
 let mut pc: u32 = 0x453a11;
 loop { match pc {
 0x453a11 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x453b37; continue; }
  pc = 0x453a3d; continue;
 }
 0x453a3d => {
  { cpu.pushad(); }
  cpu.push(0x453a43); f_0046e31e(cpu); if cpu.returned_to != 0x453a43 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453a4a; continue;
 }
 0x453a4a => {
  cpu.push(0x453a4f); f_00453f75(cpu); if cpu.returned_to != 0x453a4f { return; }
  if !cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453a55; continue;
 }
 0x453a55 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x453a64); f_0041c808(cpu); if cpu.returned_to != 0x453a64 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b3a9cu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3aa0u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x453a93); f_00453f75(cpu); if cpu.returned_to != 0x453a93 { return; }
  if !cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453a99; continue;
 }
 0x453a99 => {
  { let v: u32 = 0x5b3a9cu32; cpu.esi = v; }
  { let v: u8 = 0x9u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x453aa0; continue;
 }
 0x453aa0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453ad6; continue; }
  pc = 0x453ac4; continue;
 }
 0x453ac4 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453ad6; continue; }
  pc = 0x453acd; continue;
 }
 0x453acd => {
  cpu.push(0x453ad2); f_00453f75(cpu); if cpu.returned_to != 0x453ad2 { return; }
  if !cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453ad4; continue;
 }
 0x453ad4 => {
  pc = 0x453b0c; continue;
 }
 0x453ad6 => {
  { let v: u32 = 0x71u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x453afd; continue; }
  pc = 0x453ae0; continue;
 }
 0x453ae0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x453afd; continue; }
  pc = 0x453ae7; continue;
 }
 0x453ae7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x453af5; continue; }
  pc = 0x453af0; continue;
 }
 0x453af0 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453afa; continue; }
  pc = 0x453af5; continue;
 }
 0x453af5 => {
  { let v: u32 = 0x5b355cu32; cpu.edx = v; }
  pc = 0x453afa; continue;
 }
 0x453afa => {
  { let v: u32 = cpu.add32(cpu.edx, 0x5u32); cpu.edx = v; }
  pc = 0x453afd; continue;
 }
 0x453afd => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x453b05); f_00453f75(cpu); if cpu.returned_to != 0x453b05 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453b0c; continue;
 }
 0x453b0c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x453aa0; continue; }
  pc = 0x453b13; continue;
 }
 0x453b13 => {
  cpu.push(0x453b18); f_00416876(cpu); if cpu.returned_to != 0x453b18 { return; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x453b33); f_004481e0(cpu); if cpu.returned_to != 0x453b33 { return; }
  if cpu.cf { pc = 0x453b37; continue; }
  pc = 0x453b35; continue;
 }
 0x453b35 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453b37 => {
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x57917eu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x578d7eu32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x400u32); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x453b67; continue; }
  pc = 0x453b5a; continue;
 }
 0x453b5a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  pc = 0x453b67; continue;
 }
 0x453b67 => {
  { let v: u32 = cpu.esi; cpu.w32(0x57917eu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453b70); }
 let mut pc: u32 = 0x453b70;
 loop { match pc {
 0x453b70 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x453cc8; continue; }
  pc = 0x453b9c; continue;
 }
 0x453b9c => {
  { cpu.pushad(); }
  cpu.push(0x453ba2); f_0046e31e(cpu); if cpu.returned_to != 0x453ba2 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453ba9; continue;
 }
 0x453ba9 => {
  { cpu.pushad(); }
  cpu.push(0x453baf); f_0046e362(cpu); if cpu.returned_to != 0x453baf { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x453cc4; continue; }
  pc = 0x453bb6; continue;
 }
 0x453bb6 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x453bc3); f_004712e3(cpu); if cpu.returned_to != 0x453bc3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453bd1; continue;
 }
 0x453bd1 => {
  { cpu.pushad(); }
  cpu.push(0x453bd7); f_004713ce(cpu); if cpu.returned_to != 0x453bd7 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x453cc6; continue; }
  pc = 0x453bde; continue;
 }
 0x453bde => {
  cpu.push(0x453be3); f_00453f4e(cpu); if cpu.returned_to != 0x453be3 { return; }
  if !cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453be9; continue;
 }
 0x453be9 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  cpu.push(0x453bf8); f_0041c808(cpu); if cpu.returned_to != 0x453bf8 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b3c04u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3c08u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x453c27); f_00453f4e(cpu); if cpu.returned_to != 0x453c27 { return; }
  if !cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453c2d; continue;
 }
 0x453c2d => {
  { let v: u32 = 0x5b3c04u32; cpu.esi = v; }
  { let v: u8 = 0x24u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x453c34; continue;
 }
 0x453c34 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453c6a; continue; }
  pc = 0x453c58; continue;
 }
 0x453c58 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453c6a; continue; }
  pc = 0x453c61; continue;
 }
 0x453c61 => {
  cpu.push(0x453c66); f_00453f4e(cpu); if cpu.returned_to != 0x453c66 { return; }
  if !cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453c68; continue;
 }
 0x453c68 => {
  pc = 0x453ca0; continue;
 }
 0x453c6a => {
  { let v: u32 = 0x71u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x453c91; continue; }
  pc = 0x453c74; continue;
 }
 0x453c74 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x453c91; continue; }
  pc = 0x453c7b; continue;
 }
 0x453c7b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x453c89; continue; }
  pc = 0x453c84; continue;
 }
 0x453c84 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453c8e; continue; }
  pc = 0x453c89; continue;
 }
 0x453c89 => {
  { let v: u32 = 0x5b355cu32; cpu.edx = v; }
  pc = 0x453c8e; continue;
 }
 0x453c8e => {
  { let v: u32 = cpu.add32(cpu.edx, 0x5u32); cpu.edx = v; }
  pc = 0x453c91; continue;
 }
 0x453c91 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x453c99); f_00453f4e(cpu); if cpu.returned_to != 0x453c99 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453ca0; continue;
 }
 0x453ca0 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x453c34; continue; }
  pc = 0x453ca7; continue;
 }
 0x453ca7 => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x453cc2); f_004481e0(cpu); if cpu.returned_to != 0x453cc2 { return; }
  if cpu.cf { pc = 0x453cc8; continue; }
  pc = 0x453cc4; continue;
 }
 0x453cc4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453cc6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453cc8 => {
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453cd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453cd6); }
 let mut pc: u32 = 0x453cd6;
 loop { match pc {
 0x453cd6 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x453e32; continue; }
  pc = 0x453d02; continue;
 }
 0x453d02 => {
  cpu.push(0x453d07); f_00453e83(cpu); if cpu.returned_to != 0x453d07 { return; }
  if !cpu.cf { pc = 0x453e32; continue; }
  pc = 0x453d0d; continue;
 }
 0x453d0d => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  cpu.push(0x453d1c); f_0041c808(cpu); if cpu.returned_to != 0x453d1c { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b3ae4u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3ae8u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x453d3b; continue; }
  pc = 0x453d34; continue;
 }
 0x453d34 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x453d3b; continue; }
  pc = 0x453d39; continue;
 }
 0x453d39 => {
  pc = 0x453d6e; continue;
 }
 0x453d3b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x453d4a; continue; }
  pc = 0x453d45; continue;
 }
 0x453d45 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453d4f; continue; }
  pc = 0x453d4a; continue;
 }
 0x453d4a => {
  { let v: u32 = 0x5b355cu32; cpu.edx = v; }
  pc = 0x453d4f; continue;
 }
 0x453d4f => {
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_sub(0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x453d64); f_00458258(cpu); if cpu.returned_to != 0x453d64 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x453da8; continue; }
  pc = 0x453d6e; continue;
 }
 0x453d6e => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x453d96); f_00453e83(cpu); if cpu.returned_to != 0x453d96 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  if !cpu.cf { pc = 0x453e32; continue; }
  pc = 0x453da8; continue;
 }
 0x453da8 => {
  { let v: u32 = 0x5b3ae4u32; cpu.esi = v; }
  { let v: u8 = 0x19u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x453daf; continue;
 }
 0x453daf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453de5; continue; }
  pc = 0x453dd3; continue;
 }
 0x453dd3 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x453de5; continue; }
  pc = 0x453ddc; continue;
 }
 0x453ddc => {
  cpu.push(0x453de1); f_00453e83(cpu); if cpu.returned_to != 0x453de1 { return; }
  if !cpu.cf { pc = 0x453e32; continue; }
  pc = 0x453de3; continue;
 }
 0x453de3 => {
  pc = 0x453e1d; continue;
 }
 0x453de5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x453df3; continue; }
  pc = 0x453dee; continue;
 }
 0x453dee => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x453df8; continue; }
  pc = 0x453df3; continue;
 }
 0x453df3 => {
  { let v: u32 = 0x5b355cu32; cpu.edx = v; }
  pc = 0x453df8; continue;
 }
 0x453df8 => {
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_sub(0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x453e0d); f_00458258(cpu); if cpu.returned_to != 0x453e0d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.cf { pc = 0x453e24; continue; }
  pc = 0x453e16; continue;
 }
 0x453e16 => {
  cpu.push(0x453e1b); f_00453e83(cpu); if cpu.returned_to != 0x453e1b { return; }
  if !cpu.cf { pc = 0x453e32; continue; }
  pc = 0x453e1d; continue;
 }
 0x453e1d => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x453daf; continue; }
  pc = 0x453e24; continue;
 }
 0x453e24 => {
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453e32 => {
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453e83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453e83); }
 let mut pc: u32 = 0x453e83;
 loop { match pc {
 0x453e83 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x453e8c); f_00453e97(cpu); if cpu.returned_to != 0x453e8c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x453e94; continue; }
  pc = 0x453e91; continue;
 }
 0x453e91 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453e94 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453e97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453e97); }
 let mut pc: u32 = 0x453e97;
 loop { match pc {
 0x453e97 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ceb6cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ceb70u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4ea600u32), 0x0u8); }
  if cpu.zf { pc = 0x453ec2; continue; }
  pc = 0x453ebd; continue;
 }
 0x453ebd => {
  { let v: u8 = cpu.r8(0x4ea600u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x453ec2; continue;
 }
 0x453ec2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5ceb74u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ceb78u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x453ed7; continue;
 }
 0x453ed7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x453f45; continue; }
  pc = 0x453edf; continue;
 }
 0x453edf => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x4b8aa4u32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453eef; continue;
 }
 0x453eef => {
  { cpu.sub32(cpu.edi, 0x4b8abdu32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453ef7; continue;
 }
 0x453ef7 => {
  { cpu.sub32(cpu.edi, 0x4b8b08u32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453eff; continue;
 }
 0x453eff => {
  { cpu.sub32(cpu.edi, 0x4b8b21u32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453f07; continue;
 }
 0x453f07 => {
  { cpu.sub32(cpu.edi, 0x4b8a8bu32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453f0f; continue;
 }
 0x453f0f => {
  { cpu.sub32(cpu.edi, 0x4b8ad6u32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453f17; continue;
 }
 0x453f17 => {
  { cpu.sub32(cpu.edi, 0x4b8aefu32); }
  if cpu.zf { pc = 0x453f21; continue; }
  pc = 0x453f1f; continue;
 }
 0x453f1f => {
  pc = 0x453f45; continue;
 }
 0x453f21 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x453f38); f_004714b9(cpu); if cpu.returned_to != 0x453f38 { return; }
  if !cpu.cf { pc = 0x453f44; continue; }
  pc = 0x453f3a; continue;
 }
 0x453f3a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453f44 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x453f45; continue;
 }
 0x453f45 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x453ed7; continue; }
  pc = 0x453f4b; continue;
 }
 0x453f4b => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453f4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453f4e); }
 let mut pc: u32 = 0x453f4e;
 loop { match pc {
 0x453f4e => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u8 = 0x8u8; cpu.w8(0x4ea600u32, v); }
  cpu.push(0x453f5c); f_00453e97(cpu); if cpu.returned_to != 0x453f5c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea600u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x453f69; continue; }
  pc = 0x453f66; continue;
 }
 0x453f66 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453f69 => {
  cpu.push(0x453f6e); f_0042e0fa(cpu); if cpu.returned_to != 0x453f6e { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x453f66; continue; }
  pc = 0x453f72; continue;
 }
 0x453f72 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453f75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453f75); }
 let mut pc: u32 = 0x453f75;
 loop { match pc {
 0x453f75 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x453f88); f_0042e0fa(cpu); if cpu.returned_to != 0x453f88 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x453f91; continue; }
  pc = 0x453f8c; continue;
 }
 0x453f8c => {
  pc = 0x453f94; continue;
 }
 0x453f91 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453f94 => {
  cpu.push(0x453f99); f_0046c3d4(cpu); if cpu.returned_to != 0x453f99 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453f9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x453f9b); }
 let mut pc: u32 = 0x453f9b;
 loop { match pc {
 0x453f9b => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x453ff4; continue; }
  pc = 0x453fa2; continue;
 }
 0x453fa2 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x453fb1); f_00458258(cpu); if cpu.returned_to != 0x453fb1 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x453fe7; continue; }
  pc = 0x453fb4; continue;
 }
 0x453fb4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x453fe4; continue; }
  pc = 0x453fbc; continue;
 }
 0x453fbc => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x453fe7; continue; }
  pc = 0x453fc7; continue;
 }
 0x453fc7 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x52bc2bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e15bdu32)); }
  if !cpu.zf { pc = 0x453fe7; continue; }
  pc = 0x453fdd; continue;
 }
 0x453fdd => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d39u32, v); }
  pc = 0x453fe4; continue;
 }
 0x453fe4 => {
  { cpu.cf = false; }
  pc = 0x453ff2; continue;
 }
 0x453fe7 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  pc = 0x453ff2; continue;
 }
 0x453ff2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x453ff4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00453ff6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x453ff6); } r_00453ff6(cpu, 0x453ff6); }
pub fn f_00454101(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454101); } r_00453ff6(cpu, 0x454101); }
fn r_00453ff6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x453ff6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x45400c; continue;
 }
 0x45400c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x454017; continue;
 }
 0x454017 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5824c9u32, v); }
  { cpu.sub8((cpu.ebx as u8), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x454074; continue; }
  pc = 0x454037; continue;
 }
 0x454037 => {
  { cpu.sub8(cpu.r8(0x5824c8u32), 0x0u8); }
  if cpu.zf { pc = 0x454049; continue; }
  pc = 0x454040; continue;
 }
 0x454040 => {
  { cpu.sub8(cpu.r8(0x5824c9u32), 0x1u8); }
  if cpu.zf { pc = 0x454074; continue; }
  pc = 0x454049; continue;
 }
 0x454049 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x454068); f_0046c3d4(cpu); if cpu.returned_to != 0x454068 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x454203; continue; }
  pc = 0x454074; continue;
 }
 0x454074 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4540b5; continue; }
  pc = 0x45407b; continue;
 }
 0x45407b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x454203; continue; }
  pc = 0x454086; continue;
 }
 0x454086 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x454091; continue;
 }
 0x454091 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x45409c; continue;
 }
 0x45409c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x4540aa; continue; }
  pc = 0x4540a3; continue;
 }
 0x4540a3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x4540b5; continue; }
  pc = 0x4540aa; continue;
 }
 0x4540aa => {
  cpu.push(0x4540af); f_00489e00(cpu); if cpu.returned_to != 0x4540af { return; }
  if cpu.cf { pc = 0x454203; continue; }
  pc = 0x4540b5; continue;
 }
 0x4540b5 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4540cb; continue; }
  pc = 0x4540be; continue;
 }
 0x4540be => {
  cpu.push(0x4540c3); f_0046b31b(cpu); if cpu.returned_to != 0x4540c3 { return; }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x4540c9; continue;
 }
 0x4540c9 => {
  pc = 0x4540dd; continue;
 }
 0x4540cb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4540dd; continue; }
  pc = 0x4540d2; continue;
 }
 0x4540d2 => {
  { cpu.sub8(cpu.r8(0x6cu32.wrapping_add(cpu.ebp)), 0x55u8); }
  if cpu.cf { pc = 0x454203; continue; }
  pc = 0x4540dd; continue;
 }
 0x4540dd => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x52bc2du32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x454203; continue; }
  pc = 0x4540f3; continue;
 }
 0x4540f3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x454101; continue; }
  pc = 0x4540fc; continue;
 }
 0x4540fc => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x454106; continue; }
  pc = 0x454101; continue;
 }
 0x454101 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x454106; continue;
 }
 0x454106 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45415e; continue; }
  pc = 0x45410d; continue;
 }
 0x45410d => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x45414d; continue; }
  pc = 0x45413b; continue;
 }
 0x45413b => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.edi)), 0xfb2eu16); }
  if !cpu.zf { pc = 0x45414d; continue; }
  pc = 0x454143; continue;
 }
 0x454143 => {
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x454203; continue; }
  pc = 0x45414d; continue;
 }
 0x45414d => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x4541a9; continue;
 }
 0x45415e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x45419a; continue; }
  pc = 0x454195; continue;
 }
 0x454195 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x45419f; continue; }
  pc = 0x45419a; continue;
 }
 0x45419a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x45419f; continue;
 }
 0x45419f => {
  { let v: u16 = 0xef1fu16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x4541a9; continue;
 }
 0x4541a9 => {
  { cpu.pushad(); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4541d5; continue; }
  pc = 0x4541b1; continue;
 }
 0x4541b1 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4541c0); f_004679cb(cpu); if cpu.returned_to != 0x4541c0 { return; }
  if !cpu.cf { pc = 0x4541d5; continue; }
  pc = 0x4541c2; continue;
 }
 0x4541c2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x4541d5; continue; }
  pc = 0x4541c8; continue;
 }
 0x4541c8 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x26u8); }
  if cpu.zf { pc = 0x4541d5; continue; }
  pc = 0x4541ce; continue;
 }
 0x4541ce => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x4541d5); f_0041674d(cpu); if cpu.returned_to != 0x4541d5 { return; }
  pc = 0x4541d5; continue;
 }
 0x4541d5 => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4541fb; continue; }
  pc = 0x4541ed; continue;
 }
 0x4541ed => {
  cpu.push(0x4541f2); f_00430114(cpu); if cpu.returned_to != 0x4541f2 { return; }
  if cpu.cf { pc = 0x4541fb; continue; }
  pc = 0x4541f4; continue;
 }
 0x4541f4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97bu32, v); }
  pc = 0x4541fb; continue;
 }
 0x4541fb => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x454202); f_0045955b(cpu); if cpu.returned_to != 0x454202 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x454203; continue;
 }
 0x454203 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454204); }
 let mut pc: u32 = 0x454204;
 loop { match pc {
 0x454204 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x42u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x454256; continue; }
  pc = 0x45421a; continue;
 }
 0x45421a => {
  { cpu.sub8(cpu.r8(0x43u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.cf { pc = 0x454256; continue; }
  pc = 0x454220; continue;
 }
 0x454220 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x43u32.wrapping_add(cpu.ebx))); cpu.w8(0x43u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454228; continue;
 }
 0x454228 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x45424d; continue; }
  pc = 0x45422d; continue;
 }
 0x45422d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x454249; continue; }
  pc = 0x454247; continue;
 }
 0x454247 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x454249; continue;
 }
 0x454249 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45424d; continue;
 }
 0x45424d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454228; continue; }
  pc = 0x454252; continue;
 }
 0x454252 => {
  { let v: u8 = 0xaau8; cpu.w8(0x42u32.wrapping_add(cpu.ebx), v); }
  pc = 0x454256; continue;
 }
 0x454256 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454258); }
 let mut pc: u32 = 0x454258;
 loop { match pc {
 0x454258 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x454264; continue; }
  pc = 0x45425f; continue;
 }
 0x45425f => {
  cpu.push(0x454264); f_0045427c(cpu); if cpu.returned_to != 0x454264 { return; }
  pc = 0x454264; continue;
 }
 0x454264 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454265); }
 let mut pc: u32 = 0x454265;
 loop { match pc {
 0x454265 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x45427b; continue; }
  pc = 0x45426c; continue;
 }
 0x45426c => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x454276; continue; }
  pc = 0x454270; continue;
 }
 0x454270 => {
  { cpu.sub8((cpu.eax as u8), 0xfbu8); }
  if cpu.zf { pc = 0x454276; continue; }
  pc = 0x454274; continue;
 }
 0x454274 => {
  pc = 0x45427b; continue;
 }
 0x454276 => {
  cpu.push(0x45427b); f_0045427c(cpu); if cpu.returned_to != 0x45427b { return; }
  pc = 0x45427b; continue;
 }
 0x45427b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045427c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45427c); }
 let mut pc: u32 = 0x45427c;
 loop { match pc {
 0x45427c => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454280); }
 let mut pc: u32 = 0x454280;
 loop { match pc {
 0x454280 => {
  cpu.push(0x454285); f_0045581c(cpu); if cpu.returned_to != 0x454285 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b99f8u32, v); }
  cpu.push(0x454291); f_00455412(cpu); if cpu.returned_to != 0x454291 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b99f8u32, v); }
  cpu.push(0x45429d); f_004549b7(cpu); if cpu.returned_to != 0x45429d { return; }
  cpu.push(0x4542a2); f_00454e87(cpu); if cpu.returned_to != 0x4542a2 { return; }
  cpu.push(0x4542a7); f_00454bee(cpu); if cpu.returned_to != 0x4542a7 { return; }
  cpu.push(0x4542ac); f_00455e4b(cpu); if cpu.returned_to != 0x4542ac { return; }
  cpu.push(0x4542b1); f_0045497f(cpu); if cpu.returned_to != 0x4542b1 { return; }
  cpu.push(0x4542b6); f_00454779(cpu); if cpu.returned_to != 0x4542b6 { return; }
  cpu.push(0x4542bb); f_00455319(cpu); if cpu.returned_to != 0x4542bb { return; }
  cpu.push(0x4542c0); f_004546fe(cpu); if cpu.returned_to != 0x4542c0 { return; }
  cpu.push(0x4542c5); f_00454494(cpu); if cpu.returned_to != 0x4542c5 { return; }
  cpu.push(0x4542ca); f_004542cb(cpu); if cpu.returned_to != 0x4542ca { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004542cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4542cb); }
 let mut pc: u32 = 0x4542cb;
 loop { match pc {
 0x4542cb => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4542d5; continue;
 }
 0x4542d5 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454377; continue; }
  pc = 0x4542df; continue;
 }
 0x4542df => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454377; continue; }
  pc = 0x4542e9; continue;
 }
 0x4542e9 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454377; continue; }
  pc = 0x4542f3; continue;
 }
 0x4542f3 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x454373; continue; }
  pc = 0x4542f9; continue;
 }
 0x4542f9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x454301); f_00479c13(cpu); if cpu.returned_to != 0x454301 { return; }
  { cpu.sub32(cpu.r32(0x5d39d4u32), 0x0u32); }
  if cpu.zf { pc = 0x45432a; continue; }
  pc = 0x45430a; continue;
 }
 0x45430a => {
  { let v: u32 = 0x5d31c8u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d39d4u32); cpu.ecx = v; }
  pc = 0x454315; continue;
 }
 0x454315 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x454327; continue; }
  pc = 0x45431e; continue;
 }
 0x45431e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454315; continue; }
  pc = 0x454325; continue;
 }
 0x454325 => {
  pc = 0x45432a; continue;
 }
 0x454327 => {
  { cpu.popad(); }
  pc = 0x454373; continue;
 }
 0x45432a => {
  { cpu.popad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454332); f_00478676(cpu); if cpu.returned_to != 0x454332 { return; }
  if cpu.cf { pc = 0x454381; continue; }
  pc = 0x454334; continue;
 }
 0x454334 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x454377; continue; }
  pc = 0x45434b; continue;
 }
 0x45434b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x454353); f_0044971e(cpu); if cpu.returned_to != 0x454353 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x454373; continue; }
  pc = 0x454356; continue;
 }
 0x454356 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.cf { pc = 0x454377; continue; }
  pc = 0x45435f; continue;
 }
 0x45435f => {
  cpu.push(0x454364); f_00454382(cpu); if cpu.returned_to != 0x454364 { return; }
  if cpu.cf { pc = 0x454377; continue; }
  pc = 0x454366; continue;
 }
 0x454366 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x454377; continue;
 }
 0x454373 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  pc = 0x454377; continue;
 }
 0x454377 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4542d5; continue; }
  pc = 0x454381; continue;
 }
 0x454381 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454382(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454382); }
 let mut pc: u32 = 0x454382;
 loop { match pc {
 0x454382 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = 0x5b59b4u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b99b8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b99b4u32, v); }
  pc = 0x4543a3; continue;
 }
 0x4543a3 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x45441d; continue; }
  pc = 0x4543a9; continue;
 }
 0x4543a9 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x45441d; continue; }
  pc = 0x4543af; continue;
 }
 0x4543af => {
  cpu.push(0x4543b4); f_00454b24(cpu); if cpu.returned_to != 0x4543b4 { return; }
  if cpu.cf { pc = 0x45441d; continue; }
  pc = 0x4543b6; continue;
 }
 0x4543b6 => {
  cpu.push(0x4543bb); f_00479c13(cpu); if cpu.returned_to != 0x4543bb { return; }
  { cpu.sub32(cpu.r32(0x5d39d4u32), 0x0u32); }
  if cpu.zf { pc = 0x45441d; continue; }
  pc = 0x4543c4; continue;
 }
 0x4543c4 => {
  { let v: u32 = 0x5d31c8u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d39d4u32); cpu.ebx = v; }
  pc = 0x4543cf; continue;
 }
 0x4543cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edx)), 0x1u8); }
  if cpu.zf { pc = 0x45441a; continue; }
  pc = 0x4543da; continue;
 }
 0x4543da => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x4543e2); f_00454b24(cpu); if cpu.returned_to != 0x4543e2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x45441a; continue; }
  pc = 0x4543e5; continue;
 }
 0x4543e5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5b99b4u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x454401; continue; }
  pc = 0x4543f0; continue;
 }
 0x4543f0 => {
  { let v: u32 = 0x5b59b4u32; cpu.ebx = v; }
  pc = 0x4543f5; continue;
 }
 0x4543f5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if cpu.zf { pc = 0x454418; continue; }
  pc = 0x4543fb; continue;
 }
 0x4543fb => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4543f5; continue; }
  pc = 0x454401; continue;
 }
 0x454401 => {
  { let v: u32 = cpu.r32(0x5b99b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b99b8u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b99b4u32)); cpu.w32(0x5b99b4u32, v); }
  pc = 0x454418; continue;
 }
 0x454418 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x45441a; continue;
 }
 0x45441a => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4543cf; continue; }
  pc = 0x45441d; continue;
 }
 0x45441d => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4543a3; continue; }
  pc = 0x454423; continue;
 }
 0x454423 => {
  { let v: u32 = cpu.r32(0x5b99b4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x454490; continue; }
  pc = 0x45442c; continue;
 }
 0x45442c => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x454447; continue; }
  pc = 0x454431; continue;
 }
 0x454431 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x454440); f_0041c808(cpu); if cpu.returned_to != 0x454440 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x454447; continue;
 }
 0x454447 => {
  { let v: u32 = cpu.r32(0x5b59b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  cpu.push(0x45448b); f_00413337(cpu); if cpu.returned_to != 0x45448b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454490 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454494(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454494); }
 let mut pc: u32 = 0x454494;
 loop { match pc {
 0x454494 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x45449e; continue;
 }
 0x45449e => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4544de; continue; }
  pc = 0x4544a4; continue;
 }
 0x4544a4 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x4544de; continue; }
  pc = 0x4544aa; continue;
 }
 0x4544aa => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x4544de; continue; }
  pc = 0x4544b0; continue;
 }
 0x4544b0 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x4544de; continue; }
  pc = 0x4544b6; continue;
 }
 0x4544b6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x4544bd); f_00478676(cpu); if cpu.returned_to != 0x4544bd { return; }
  if cpu.cf { pc = 0x4544de; continue; }
  pc = 0x4544bf; continue;
 }
 0x4544bf => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x4544d7; continue; }
  pc = 0x4544c5; continue;
 }
 0x4544c5 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x4544d7; continue; }
  pc = 0x4544cb; continue;
 }
 0x4544cb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x4544d7; continue; }
  pc = 0x4544d1; continue;
 }
 0x4544d1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x4544de; continue; }
  pc = 0x4544d7; continue;
 }
 0x4544d7 => {
  cpu.push(0x4544dc); f_004544e5(cpu); if cpu.returned_to != 0x4544dc { return; }
  if cpu.cf { pc = 0x4544e4; continue; }
  pc = 0x4544de; continue;
 }
 0x4544de => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x45449e; continue; }
  pc = 0x4544e4; continue;
 }
 0x4544e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004544e5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4544e5); } r_004544e5(cpu, 0x4544e5); }
pub fn f_00454552(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454552); } r_004544e5(cpu, 0x454552); }
fn r_004544e5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4544e5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5b99ecu32, v); }
  pc = 0x4544fc; continue;
 }
 0x4544fc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x4545da; continue; }
  pc = 0x454507; continue;
 }
 0x454507 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4545da; continue; }
  pc = 0x454511; continue;
 }
 0x454511 => {
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4545da; continue; }
  pc = 0x45451b; continue;
 }
 0x45451b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xc8u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x45452a; continue;
 }
 0x45452a => {
  { cpu.sub32(cpu.eax, 0x271u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454531; continue;
 }
 0x454531 => {
  { cpu.sub32(cpu.eax, 0x2d5u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454538; continue;
 }
 0x454538 => {
  { cpu.sub32(cpu.eax, 0x307u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x45453f; continue;
 }
 0x45453f => {
  { cpu.sub32(cpu.eax, 0x20du32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454546; continue;
 }
 0x454546 => {
  { cpu.sub32(cpu.eax, 0x226u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x45454d; continue;
 }
 0x45454d => {
  { cpu.sub32(cpu.eax, 0x23fu32); }
  pc = 0x454552; continue;
 }
 0x454552 => {
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454554; continue;
 }
 0x454554 => {
  { cpu.sub32(cpu.eax, 0x258u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x45455b; continue;
 }
 0x45455b => {
  { cpu.sub32(cpu.eax, 0x339u32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454562; continue;
 }
 0x454562 => {
  { cpu.sub32(cpu.eax, 0x36bu32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454569; continue;
 }
 0x454569 => {
  { cpu.sub32(cpu.eax, 0x39du32); }
  if cpu.zf { pc = 0x454575; continue; }
  pc = 0x454570; continue;
 }
 0x454570 => {
  { cpu.sub32(cpu.eax, 0x7du32); }
  if !cpu.zf { pc = 0x4545da; continue; }
  pc = 0x454575; continue;
 }
 0x454575 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454582); f_0047a2d9(cpu); if cpu.returned_to != 0x454582 { return; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x45458d); f_0045502b(cpu); if cpu.returned_to != 0x45458d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x4545da; continue; }
  pc = 0x454590; continue;
 }
 0x454590 => {
  { cpu.and8(cpu.r8(0x1du32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x45459f; continue; }
  pc = 0x454596; continue;
 }
 0x454596 => {
  cpu.push(0x45459b); f_0041c743(cpu); if cpu.returned_to != 0x45459b { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4545da; continue; }
  pc = 0x45459f; continue;
 }
 0x45459f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5b99ecu32)); }
  if !cpu.cf { pc = 0x4545da; continue; }
  pc = 0x4545ce; continue;
 }
 0x4545ce => {
  { let v: u32 = cpu.edx; cpu.w32(0x5b99ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5b99e8u32, v); }
  pc = 0x4545da; continue;
 }
 0x4545da => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4544fc; continue; }
  pc = 0x4545e4; continue;
 }
 0x4545e4 => {
  { cpu.sub32(cpu.r32(0x5b99ecu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x454638; continue; }
  pc = 0x4545ed; continue;
 }
 0x4545ed => {
  { let v: u32 = cpu.r32(0x5b99e8u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  cpu.push(0x454622); f_0047a2d9(cpu); if cpu.returned_to != 0x454622 { return; }
  cpu.push(0x454627); f_00478632(cpu); if cpu.returned_to != 0x454627 { return; }
  if cpu.cf { pc = 0x454638; continue; }
  pc = 0x454629; continue;
 }
 0x454629 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x454635); f_00413337(cpu); if cpu.returned_to != 0x454635 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454638 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045463b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45463b); }
 let mut pc: u32 = 0x45463b;
 loop { match pc {
 0x45463b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b99f7u32, v); }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x45464c; continue;
 }
 0x45464c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454674; continue; }
  pc = 0x454654; continue;
 }
 0x454654 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454674; continue; }
  pc = 0x45465a; continue;
 }
 0x45465a => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454674; continue; }
  pc = 0x454660; continue;
 }
 0x454660 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x454674; continue; }
  pc = 0x454668; continue;
 }
 0x454668 => {
  cpu.push(0x45466d); f_0045467e(cpu); if cpu.returned_to != 0x45466d { return; }
  if cpu.cf { pc = 0x454674; continue; }
  pc = 0x45466f; continue;
 }
 0x45466f => {
  cpu.push(0x454674); f_0045507f(cpu); if cpu.returned_to != 0x454674 { return; }
  pc = 0x454674; continue;
 }
 0x454674 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x45464c; continue; }
  pc = 0x45467d; continue;
 }
 0x45467d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045467e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45467e); }
 let mut pc: u32 = 0x45467e;
 loop { match pc {
 0x45467e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454686); f_004786ae(cpu); if cpu.returned_to != 0x454686 { return; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x4546ae; continue; }
  pc = 0x45468c; continue;
 }
 0x45468c => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x48u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6796u32)); }
  if !cpu.zf { pc = 0x4546ae; continue; }
  pc = 0x4546a5; continue;
 }
 0x4546a5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6798u32)); }
  if cpu.zf { pc = 0x4546f4; continue; }
  pc = 0x4546ae; continue;
 }
 0x4546ae => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x4546b5); f_00478676(cpu); if cpu.returned_to != 0x4546b5 { return; }
  if cpu.cf { pc = 0x4546f1; continue; }
  pc = 0x4546b7; continue;
 }
 0x4546b7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4546f1; continue; }
  pc = 0x4546be; continue;
 }
 0x4546be => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4546f1; continue; }
  pc = 0x4546c3; continue;
 }
 0x4546c3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6796u32)); }
  if !cpu.zf { pc = 0x4546f1; continue; }
  pc = 0x4546d3; continue;
 }
 0x4546d3 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x48u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6798u32)); }
  if cpu.zf { pc = 0x4546f4; continue; }
  pc = 0x4546e4; continue;
 }
 0x4546e4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6798u32)); }
  if cpu.zf { pc = 0x4546f4; continue; }
  pc = 0x4546f1; continue;
 }
 0x4546f1 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4546f4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b99f7u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004546fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4546fe); }
 let mut pc: u32 = 0x4546fe;
 loop { match pc {
 0x4546fe => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x454708; continue;
 }
 0x454708 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x45473d; continue; }
  pc = 0x45470e; continue;
 }
 0x45470e => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x45473d; continue; }
  pc = 0x454714; continue;
 }
 0x454714 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x45471b); f_00454f38(cpu); if cpu.returned_to != 0x45471b { return; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454723); f_00454fa7(cpu); if cpu.returned_to != 0x454723 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1f4u32); }
  if cpu.cf { pc = 0x45473d; continue; }
  pc = 0x45472e; continue;
 }
 0x45472e => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x45473d; continue; }
  pc = 0x454732; continue;
 }
 0x454732 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x45473b); f_00454744(cpu); if cpu.returned_to != 0x45473b { return; }
  pc = 0x454743; continue;
 }
 0x45473d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x454708; continue; }
  pc = 0x454743; continue;
 }
 0x454743 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454744); }
 let mut pc: u32 = 0x454744;
 loop { match pc {
 0x454744 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x45474b); f_00455719(cpu); if cpu.returned_to != 0x45474b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x5b99e4u32), 0x0u32); }
  if cpu.zf { pc = 0x454778; continue; }
  pc = 0x454756; continue;
 }
 0x454756 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x45475d); f_0045566e(cpu); if cpu.returned_to != 0x45475d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b99e0u32, v); }
  cpu.push(0x454769); f_00455617(cpu); if cpu.returned_to != 0x454769 { return; }
  if cpu.cf { pc = 0x454778; continue; }
  pc = 0x45476b; continue;
 }
 0x45476b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5b5234u32); cpu.esi = v; }
  cpu.push(0x454778); f_00454f84(cpu); if cpu.returned_to != 0x454778 { return; }
  pc = 0x454778; continue;
 }
 0x454778 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454779(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454779); }
 let mut pc: u32 = 0x454779;
 loop { match pc {
 0x454779 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454780; continue;
 }
 0x454780 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4547f6; continue; }
  pc = 0x454786; continue;
 }
 0x454786 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x4547f6; continue; }
  pc = 0x45478c; continue;
 }
 0x45478c => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x7u8); }
  if cpu.zf { pc = 0x454798; continue; }
  pc = 0x454792; continue;
 }
 0x454792 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x4547f6; continue; }
  pc = 0x454798; continue;
 }
 0x454798 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4547f2; continue; }
  pc = 0x45479e; continue;
 }
 0x45479e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x4547f2; continue; }
  pc = 0x4547a4; continue;
 }
 0x4547a4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x4547ab); f_004786ae(cpu); if cpu.returned_to != 0x4547ab { return; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4547f2; continue; }
  pc = 0x4547b1; continue;
 }
 0x4547b1 => {
  cpu.push(0x4547b6); f_0044971e(cpu); if cpu.returned_to != 0x4547b6 { return; }
  if cpu.cf { pc = 0x4547f2; continue; }
  pc = 0x4547b8; continue;
 }
 0x4547b8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x4547bf); f_00478676(cpu); if cpu.returned_to != 0x4547bf { return; }
  if cpu.cf { pc = 0x4547f6; continue; }
  pc = 0x4547c1; continue;
 }
 0x4547c1 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x4547cd; continue; }
  pc = 0x4547ca; continue;
 }
 0x4547ca => {
  { let v: u32 = cpu.ebp.wrapping_add(0xfffffff8u32); cpu.ebp = v; }
  pc = 0x4547cd; continue;
 }
 0x4547cd => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4547f2; continue; }
  pc = 0x4547d7; continue;
 }
 0x4547d7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x14u32.wrapping_add(cpu.esi))); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x14u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.cf { pc = 0x4547f6; continue; }
  pc = 0x4547e0; continue;
 }
 0x4547e0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x4547f2; continue;
 }
 0x4547f2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x4547f6; continue;
 }
 0x4547f6 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454780; continue; }
  pc = 0x4547fd; continue;
 }
 0x4547fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454937(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454937); }
 let mut pc: u32 = 0x454937;
 loop { match pc {
 0x454937 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x454948; continue;
 }
 0x454948 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45496d; continue; }
  pc = 0x45494e; continue;
 }
 0x45494e => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x45496d; continue; }
  pc = 0x454954; continue;
 }
 0x454954 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x45496d; continue; }
  pc = 0x45495a; continue;
 }
 0x45495a => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x45496d; continue; }
  pc = 0x454960; continue;
 }
 0x454960 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x19u32.wrapping_add(cpu.eax))); }
  if cpu.cf { pc = 0x45496d; continue; }
  pc = 0x454968; continue;
 }
 0x454968 => {
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x45496d; continue;
 }
 0x45496d => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454948; continue; }
  pc = 0x454974; continue;
 }
 0x454974 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x45497d; continue; }
  pc = 0x454979; continue;
 }
 0x454979 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45497d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045497f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45497f); }
 let mut pc: u32 = 0x45497f;
 loop { match pc {
 0x45497f => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454986; continue;
 }
 0x454986 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4549af; continue; }
  pc = 0x45498c; continue;
 }
 0x45498c => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if !cpu.zf { pc = 0x4549af; continue; }
  pc = 0x454992; continue;
 }
 0x454992 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x45499c); f_0047a0de(cpu); if cpu.returned_to != 0x45499c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x4549a4; continue; }
  pc = 0x4549a0; continue;
 }
 0x4549a0 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x4549af; continue; }
  pc = 0x4549a4; continue;
 }
 0x4549a4 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4549af; continue;
 }
 0x4549af => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454986; continue; }
  pc = 0x4549b6; continue;
 }
 0x4549b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004549b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4549b7); }
 let mut pc: u32 = 0x4549b7;
 loop { match pc {
 0x4549b7 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4549be; continue;
 }
 0x4549be => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4549da; continue; }
  pc = 0x4549c4; continue;
 }
 0x4549c4 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x4549da; continue; }
  pc = 0x4549ca; continue;
 }
 0x4549ca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4549d1); f_00454a54(cpu); if cpu.returned_to != 0x4549d1 { return; }
  if cpu.cf { pc = 0x4549d8; continue; }
  pc = 0x4549d3; continue;
 }
 0x4549d3 => {
  cpu.push(0x4549d8); f_004549e2(cpu); if cpu.returned_to != 0x4549d8 { return; }
  pc = 0x4549d8; continue;
 }
 0x4549d8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4549da; continue;
 }
 0x4549da => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4549be; continue; }
  pc = 0x4549e1; continue;
 }
 0x4549e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004549e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4549e2); }
 let mut pc: u32 = 0x4549e2;
 loop { match pc {
 0x4549e2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x454a52; continue; }
  pc = 0x4549ec; continue;
 }
 0x4549ec => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4549f2); f_00455644(cpu); if cpu.returned_to != 0x4549f2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b99e0u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x454a06; continue;
 }
 0x454a06 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a0c; continue;
 }
 0x454a0c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a12; continue;
 }
 0x454a12 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a18; continue;
 }
 0x454a18 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a1e; continue;
 }
 0x454a1e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a24; continue;
 }
 0x454a24 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a2a; continue;
 }
 0x454a2a => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454a4b; continue; }
  pc = 0x454a2f; continue;
 }
 0x454a2f => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x454a3e; continue; }
  pc = 0x454a33; continue;
 }
 0x454a33 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454a3b); f_00454aba(cpu); if cpu.returned_to != 0x454a3b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x454a4b; continue;
 }
 0x454a3e => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5b99e0u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x454a4b; continue; }
  pc = 0x454a49; continue;
 }
 0x454a49 => {
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x454a4b; continue;
 }
 0x454a4b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454a06; continue; }
  pc = 0x454a52; continue;
 }
 0x454a52 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454a54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454a54); }
 let mut pc: u32 = 0x454a54;
 loop { match pc {
 0x454a54 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454a5d); f_00455644(cpu); if cpu.returned_to != 0x454a5d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x454a68); f_00454ba0(cpu); if cpu.returned_to != 0x454a68 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.cf { pc = 0x454a89; continue; }
  pc = 0x454a6c; continue;
 }
 0x454a6c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x454a76); f_00454b5f(cpu); if cpu.returned_to != 0x454a76 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x454a7c); f_00455433(cpu); if cpu.returned_to != 0x454a7c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x454a87; continue; }
  pc = 0x454a7f; continue;
 }
 0x454a7f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x454a87); f_00454a8b(cpu); if cpu.returned_to != 0x454a87 { return; }
  pc = 0x454a87; continue;
 }
 0x454a87 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454a89 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454a8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454a8b); }
 let mut pc: u32 = 0x454a8b;
 loop { match pc {
 0x454a8b => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454a92; continue;
 }
 0x454a92 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454ab2; continue; }
  pc = 0x454a98; continue;
 }
 0x454a98 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454ab2; continue; }
  pc = 0x454a9e; continue;
 }
 0x454a9e => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454ab2; continue; }
  pc = 0x454aa4; continue;
 }
 0x454aa4 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454ab2; continue; }
  pc = 0x454aa9; continue;
 }
 0x454aa9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454ab1); f_00454aba(cpu); if cpu.returned_to != 0x454ab1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x454ab2; continue;
 }
 0x454ab2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454a92; continue; }
  pc = 0x454ab9; continue;
 }
 0x454ab9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454aba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454aba); }
 let mut pc: u32 = 0x454aba;
 loop { match pc {
 0x454aba => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454ac4); f_00478676(cpu); if cpu.returned_to != 0x454ac4 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x454acc); f_00454b24(cpu); if cpu.returned_to != 0x454acc { return; }
  if cpu.cf { pc = 0x454af1; continue; }
  pc = 0x454ace; continue;
 }
 0x454ace => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x454ad5); f_00479a1a(cpu); if cpu.returned_to != 0x454ad5 { return; }
  pc = 0x454ad5; continue;
 }
 0x454ad5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454af1 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  cpu.push(0x454b22); f_00413337(cpu); if cpu.returned_to != 0x454b22 { return; }
  cpu.poll(); pc = 0x454ad5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454b24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454b24); }
 let mut pc: u32 = 0x454b24;
 loop { match pc {
 0x454b24 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x454b5d; continue; }
  pc = 0x454b28; continue;
 }
 0x454b28 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x454b35; continue; }
  pc = 0x454b2e; continue;
 }
 0x454b2e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x454b5d; continue; }
  pc = 0x454b35; continue;
 }
 0x454b35 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x454b5b; continue; }
  pc = 0x454b3b; continue;
 }
 0x454b3b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x454b50; continue; }
  pc = 0x454b42; continue;
 }
 0x454b42 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x454b50; continue; }
  pc = 0x454b49; continue;
 }
 0x454b49 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x454b5b; continue; }
  pc = 0x454b50; continue;
 }
 0x454b50 => {
  { cpu.pushad(); }
  cpu.push(0x454b56); f_0046d525(cpu); if cpu.returned_to != 0x454b56 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x454b5d; continue; }
  pc = 0x454b5b; continue;
 }
 0x454b5b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454b5d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454b5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454b5f); }
 let mut pc: u32 = 0x454b5f;
 loop { match pc {
 0x454b5f => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x454b6a; continue;
 }
 0x454b6a => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b70; continue;
 }
 0x454b70 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b76; continue;
 }
 0x454b76 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b7c; continue;
 }
 0x454b7c => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b82; continue;
 }
 0x454b82 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b88; continue;
 }
 0x454b88 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b8e; continue;
 }
 0x454b8e => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454b97; continue; }
  pc = 0x454b93; continue;
 }
 0x454b93 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  pc = 0x454b97; continue;
 }
 0x454b97 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454b6a; continue; }
  pc = 0x454b9e; continue;
 }
 0x454b9e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454ba0); }
 let mut pc: u32 = 0x454ba0;
 loop { match pc {
 0x454ba0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x454bb2; continue;
 }
 0x454bb2 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bb8; continue;
 }
 0x454bb8 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bbe; continue;
 }
 0x454bbe => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bc4; continue;
 }
 0x454bc4 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bca; continue;
 }
 0x454bca => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bd0; continue;
 }
 0x454bd0 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bd6; continue;
 }
 0x454bd6 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454bde; continue; }
  pc = 0x454bdb; continue;
 }
 0x454bdb => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  pc = 0x454bde; continue;
 }
 0x454bde => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454bb2; continue; }
  pc = 0x454be5; continue;
 }
 0x454be5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
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
pub fn f_00454bee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454bee); }
 let mut pc: u32 = 0x454bee;
 loop { match pc {
 0x454bee => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x454bf8; continue;
 }
 0x454bf8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454c33; continue; }
  pc = 0x454c00; continue;
 }
 0x454c00 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454c33; continue; }
  pc = 0x454c06; continue;
 }
 0x454c06 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454c33; continue; }
  pc = 0x454c0c; continue;
 }
 0x454c0c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454c12); f_00454dda(cpu); if cpu.returned_to != 0x454c12 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x454c18); f_0045507f(cpu); if cpu.returned_to != 0x454c18 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454c1e); f_00454cca(cpu); if cpu.returned_to != 0x454c1e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454c29); f_00478676(cpu); if cpu.returned_to != 0x454c29 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x454c31); f_00455b74(cpu); if cpu.returned_to != 0x454c31 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x454c33; continue;
 }
 0x454c33 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454bf8; continue; }
  pc = 0x454c3c; continue;
 }
 0x454c3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454c3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454c3d); } r_00454c3d(cpu, 0x454c3d); }
pub fn f_00454c44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454c44); } r_00454c3d(cpu, 0x454c44); }
pub fn f_00454c48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454c48); } r_00454c3d(cpu, 0x454c48); }
fn r_00454c3d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x454c3d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x454c4d; continue;
 }
 0x454c44 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0xf0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xf0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x454c4d; continue;
 }
 0x454c48 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), ((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v = (cpu.r32(0x7400157eu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.sbb8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c59; continue;
 }
 0x454c4d => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c53; continue;
 }
 0x454c53 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c59; continue;
 }
 0x454c59 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c5f; continue;
 }
 0x454c5f => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c64; continue;
 }
 0x454c64 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x454c6d; continue; }
  pc = 0x454c6a; continue;
 }
 0x454c6a => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  pc = 0x454c6d; continue;
 }
 0x454c6d => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x454c4d; continue; }
  pc = 0x454c73; continue;
 }
 0x454c73 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x454c79); f_00455644(cpu); if cpu.returned_to != 0x454c79 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x454c81; continue; }
  pc = 0x454c7f; continue;
 }
 0x454c7f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454c81 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454c83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454c83); }
 let mut pc: u32 = 0x454c83;
 loop { match pc {
 0x454c83 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x454c86; continue;
 }
 0x454c86 => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x454c95); f_00454fea(cpu); if cpu.returned_to != 0x454c95 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x454cab; continue; }
  pc = 0x454c9a; continue;
 }
 0x454c9a => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x454ca4); f_00454fea(cpu); if cpu.returned_to != 0x454ca4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x454cab; continue; }
  pc = 0x454ca9; continue;
 }
 0x454ca9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454cab => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5b99f0u32); cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454cbd); f_00478676(cpu); if cpu.returned_to != 0x454cbd { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x454cc7); f_004553c3(cpu); if cpu.returned_to != 0x454cc7 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x454c86; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454cca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454cca); } r_00454cca(cpu, 0x454cca); }
pub fn f_00454d55(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454d55); } r_00454cca(cpu, 0x454d55); }
fn r_00454cca(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x454cca => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x454d97; continue; }
  pc = 0x454cd5; continue;
 }
 0x454cd5 => {
  cpu.push(0x454cda); f_00454c3d(cpu); if cpu.returned_to != 0x454cda { return; }
  if cpu.cf { pc = 0x454d36; continue; }
  pc = 0x454cdc; continue;
 }
 0x454cdc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x454ce7); f_00454fea(cpu); if cpu.returned_to != 0x454ce7 { return; }
  if cpu.cf { pc = 0x454d3f; continue; }
  pc = 0x454ce9; continue;
 }
 0x454ce9 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x454cf0); f_00454fea(cpu); if cpu.returned_to != 0x454cf0 { return; }
  if cpu.cf { pc = 0x454d3f; continue; }
  pc = 0x454cf2; continue;
 }
 0x454cf2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x454cfa); f_00479c13(cpu); if cpu.returned_to != 0x454cfa { return; }
  { let v: u8 = cpu.r8(0x5d39d4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x454d34; continue; }
  pc = 0x454d04; continue;
 }
 0x454d04 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5d31c8u32; cpu.esi = v; }
  pc = 0x454d0e; continue;
 }
 0x454d0e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454d2d; continue; }
  pc = 0x454d12; continue;
 }
 0x454d12 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454d1f); f_00478676(cpu); if cpu.returned_to != 0x454d1f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x454d29); f_004553c3(cpu); if cpu.returned_to != 0x454d29 { return; }
  { cpu.popad(); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x454d34; continue;
 }
 0x454d2d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454d0e; continue; }
  pc = 0x454d34; continue;
 }
 0x454d34 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454d36 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x454d3e); f_00454c83(cpu); if cpu.returned_to != 0x454d3e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454d3f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x454d47); f_00479c13(cpu); if cpu.returned_to != 0x454d47 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x5d39d4u32), 0x0u32); }
  if cpu.zf { pc = 0x454d6a; continue; }
  pc = 0x454d51; continue;
 }
 0x454d51 => {
  { let v: u8 = cpu.r8(0x5d39d4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d31c8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x454d5f; continue;
 }
 0x454d55 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x5d31c8u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x5d31c8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x454d5f; continue;
 }
 0x454d5f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.zf { pc = 0x454d6b; continue; }
  pc = 0x454d63; continue;
 }
 0x454d63 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454d5f; continue; }
  pc = 0x454d6a; continue;
 }
 0x454d6a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454d6b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454d73); f_00478676(cpu); if cpu.returned_to != 0x454d73 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x454d85); f_00479a1a(cpu); if cpu.returned_to != 0x454d85 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454d97 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { cpu.poll(); pc = 0x454d6a; continue; }
  pc = 0x454d9d; continue;
 }
 0x454d9d => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x454da7); f_00454fea(cpu); if cpu.returned_to != 0x454da7 { return; }
  if cpu.cf { cpu.poll(); pc = 0x454d6a; continue; }
  pc = 0x454da9; continue;
 }
 0x454da9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454db2); f_00478676(cpu); if cpu.returned_to != 0x454db2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.pushad(); }
  cpu.push(0x454dc0); f_0045557c(cpu); if cpu.returned_to != 0x454dc0 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x454dcd); f_00479a1a(cpu); if cpu.returned_to != 0x454dcd { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454dda(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454dda); } r_00454dda(cpu, 0x454dda); }
pub fn f_00454e47(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x454e47); } r_00454dda(cpu, 0x454e47); }
fn r_00454dda(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x454dda => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454de4; continue;
 }
 0x454de4 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454dea; continue;
 }
 0x454dea => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454df0; continue;
 }
 0x454df0 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454e0e; continue; }
  pc = 0x454df6; continue;
 }
 0x454df6 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x454e0e; continue; }
  pc = 0x454dfc; continue;
 }
 0x454dfc => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x454e0e; continue; }
  pc = 0x454e02; continue;
 }
 0x454e02 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x454e0e; continue; }
  pc = 0x454e08; continue;
 }
 0x454e08 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454e0e; continue;
 }
 0x454e0e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454e1c; continue;
 }
 0x454e1c => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454e21; continue;
 }
 0x454e21 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x454e67; continue; }
  pc = 0x454e27; continue;
 }
 0x454e27 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454e2b; continue;
 }
 0x454e2b => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x454e35); f_0045502b(cpu); if cpu.returned_to != 0x454e35 { return; }
  if cpu.cf { pc = 0x454e66; continue; }
  pc = 0x454e37; continue;
 }
 0x454e37 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454e3e); f_004786ae(cpu); if cpu.returned_to != 0x454e3e { return; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x454e66; continue; }
  pc = 0x454e44; continue;
 }
 0x454e44 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454e4b); f_00478676(cpu); if cpu.returned_to != 0x454e4b { return; }
  if cpu.cf { pc = 0x454e66; continue; }
  pc = 0x454e4d; continue;
 }
 0x454e47 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.cf { pc = 0x454e66; continue; }
  pc = 0x454e4d; continue;
 }
 0x454e4d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.esi = v; }
  cpu.push(0x454e5b); f_00455215(cpu); if cpu.returned_to != 0x454e5b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x454e66; continue;
 }
 0x454e66 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454e67 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x454e77); f_00478676(cpu); if cpu.returned_to != 0x454e77 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x454e7f); f_00455b74(cpu); if cpu.returned_to != 0x454e7f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x454e86); f_00454ed2(cpu); if cpu.returned_to != 0x454e86 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454e87); }
 let mut pc: u32 = 0x454e87;
 loop { match pc {
 0x454e87 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454e8e; continue;
 }
 0x454e8e => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454ebd; continue; }
  pc = 0x454e94; continue;
 }
 0x454e94 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454ebd; continue; }
  pc = 0x454e9a; continue;
 }
 0x454e9a => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454ebd; continue; }
  pc = 0x454ea0; continue;
 }
 0x454ea0 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x454ebd; continue; }
  pc = 0x454ea6; continue;
 }
 0x454ea6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x454ec5; continue; }
  pc = 0x454eaf; continue;
 }
 0x454eaf => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x454eb6); f_00454fa7(cpu); if cpu.returned_to != 0x454eb6 { return; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if (cpu.sf != cpu.of) { pc = 0x454ec5; continue; }
  pc = 0x454ebd; continue;
 }
 0x454ebd => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454e8e; continue; }
  pc = 0x454ec4; continue;
 }
 0x454ec4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x454ec5 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x454ebd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454ed2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454ed2); }
 let mut pc: u32 = 0x454ed2;
 loop { match pc {
 0x454ed2 => {
  { cpu.sub32(cpu.r32(0x19u32.wrapping_add(cpu.eax)), 0x1f4u32); }
  if cpu.cf { pc = 0x454f37; continue; }
  pc = 0x454edb; continue;
 }
 0x454edb => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x454ee2); f_00454fa7(cpu); if cpu.returned_to != 0x454ee2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x454f37; continue; }
  pc = 0x454ee6; continue;
 }
 0x454ee6 => {
  cpu.push(0x454eeb); f_00454f38(cpu); if cpu.returned_to != 0x454eeb { return; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x454f37; continue; }
  pc = 0x454eef; continue;
 }
 0x454eef => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x454ef8; continue;
 }
 0x454ef8 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454f30; continue; }
  pc = 0x454efe; continue;
 }
 0x454efe => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454f30; continue; }
  pc = 0x454f02; continue;
 }
 0x454f02 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454f30; continue; }
  pc = 0x454f08; continue;
 }
 0x454f08 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454f30; continue; }
  pc = 0x454f0e; continue;
 }
 0x454f0e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x454f30; continue; }
  pc = 0x454f14; continue;
 }
 0x454f14 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x454f26; continue; }
  pc = 0x454f18; continue;
 }
 0x454f18 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  if !cpu.sf { pc = 0x454f26; continue; }
  pc = 0x454f1f; continue;
 }
 0x454f1f => {
  cpu.push(0x454f24); f_00454f84(cpu); if cpu.returned_to != 0x454f24 { return; }
  pc = 0x454f37; continue;
 }
 0x454f26 => {
  cpu.push(0x454f2b); f_00454f84(cpu); if cpu.returned_to != 0x454f2b { return; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  pc = 0x454f30; continue;
 }
 0x454f30 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454ef8; continue; }
  pc = 0x454f37; continue;
 }
 0x454f37 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454f38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454f38); }
 let mut pc: u32 = 0x454f38;
 loop { match pc {
 0x454f38 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x454f47; continue;
 }
 0x454f47 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x454f76; continue; }
  pc = 0x454f4d; continue;
 }
 0x454f4d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454f54); f_00478676(cpu); if cpu.returned_to != 0x454f54 { return; }
  if cpu.cf { pc = 0x454f76; continue; }
  pc = 0x454f56; continue;
 }
 0x454f56 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x454f73; continue; }
  pc = 0x454f5c; continue;
 }
 0x454f5c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x454f76; continue; }
  pc = 0x454f62; continue;
 }
 0x454f62 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x454f76; continue; }
  pc = 0x454f68; continue;
 }
 0x454f68 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x454f76; continue; }
  pc = 0x454f6e; continue;
 }
 0x454f6e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x454f76; continue; }
  pc = 0x454f73; continue;
 }
 0x454f73 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  pc = 0x454f76; continue;
 }
 0x454f76 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454f47; continue; }
  pc = 0x454f7d; continue;
 }
 0x454f7d => {
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
pub fn f_00454f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454f84); }
 let mut pc: u32 = 0x454f84;
 loop { match pc {
 0x454f84 => {
  { let v: u8 = 0x4u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x454f9a); f_00478676(cpu); if cpu.returned_to != 0x454f9a { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x454fa5); f_00479a1a(cpu); if cpu.returned_to != 0x454fa5 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454fa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454fa7); }
 let mut pc: u32 = 0x454fa7;
 loop { match pc {
 0x454fa7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d39f8u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.r32(0x5d39f8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5d39f0u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x454fcd; continue; }
  pc = 0x454fc8; continue;
 }
 0x454fc8 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x454fcd; continue;
 }
 0x454fcd => {
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
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00454fea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x454fea); }
 let mut pc: u32 = 0x454fea;
 loop { match pc {
 0x454fea => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x524b80u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x454ff6; continue;
 }
 0x454ff6 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x455020; continue; }
  pc = 0x454ffc; continue;
 }
 0x454ffc => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x455020; continue; }
  pc = 0x455002; continue;
 }
 0x455002 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x10u8); }
  if cpu.zf { pc = 0x455020; continue; }
  pc = 0x455008; continue;
 }
 0x455008 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x11u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x455020; continue; }
  pc = 0x45500d; continue;
 }
 0x45500d => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x455020; continue; }
  pc = 0x455011; continue;
 }
 0x455011 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.ebx); }
  if !cpu.zf { pc = 0x455020; continue; }
  pc = 0x455016; continue;
 }
 0x455016 => {
  { let v: u32 = cpu.edi; cpu.w32(0x5b99f0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455020 => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x454ff6; continue; }
  pc = 0x455027; continue;
 }
 0x455027 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045502b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45502b); }
 let mut pc: u32 = 0x45502b;
 loop { match pc {
 0x45502b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x455037; continue;
 }
 0x455037 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45505e; continue; }
  pc = 0x45503d; continue;
 }
 0x45503d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x45505e; continue; }
  pc = 0x455043; continue;
 }
 0x455043 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x45505e; continue; }
  pc = 0x455049; continue;
 }
 0x455049 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x45505e; continue; }
  pc = 0x45504e; continue;
 }
 0x45504e => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x455058; continue; }
  pc = 0x455053; continue;
 }
 0x455053 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if !cpu.zf { pc = 0x45505e; continue; }
  pc = 0x455058; continue;
 }
 0x455058 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45505e => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x455037; continue; }
  pc = 0x455065; continue;
 }
 0x455065 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455069(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455069); }
 let mut pc: u32 = 0x455069;
 loop { match pc {
 0x455069 => {
  { cpu.pushad(); }
  pc = 0x45506a; continue;
 }
 0x45506a => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455074); f_0045502b(cpu); if cpu.returned_to != 0x455074 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x45507d; continue; }
  pc = 0x455077; continue;
 }
 0x455077 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x45506a; continue;
 }
 0x45507d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045507f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45507f); }
 let mut pc: u32 = 0x45507f;
 loop { match pc {
 0x45507f => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x455192; continue; }
  pc = 0x455089; continue;
 }
 0x455089 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x455189; continue; }
  pc = 0x455095; continue;
 }
 0x455095 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x455192; continue; }
  pc = 0x45509f; continue;
 }
 0x45509f => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x455192; continue; }
  pc = 0x4550a9; continue;
 }
 0x4550a9 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x455192; continue; }
  pc = 0x4550b3; continue;
 }
 0x4550b3 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x455192; continue; }
  pc = 0x4550bd; continue;
 }
 0x4550bd => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x4550d8; continue; }
  pc = 0x4550c3; continue;
 }
 0x4550c3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4550cd); f_00454ba0(cpu); if cpu.returned_to != 0x4550cd { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x455192; continue; }
  pc = 0x4550d8; continue;
 }
 0x4550d8 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4550e7; continue; }
  pc = 0x4550df; continue;
 }
 0x4550df => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x455190; continue; }
  pc = 0x4550e7; continue;
 }
 0x4550e7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4550f2; continue;
 }
 0x4550f2 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x45510c; continue; }
  pc = 0x4550f8; continue;
 }
 0x4550f8 => {
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x45510c; continue; }
  pc = 0x4550fc; continue;
 }
 0x4550fc => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x45510c; continue; }
  pc = 0x455100; continue;
 }
 0x455100 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x45510c; continue; }
  pc = 0x455106; continue;
 }
 0x455106 => {
  { cpu.popad(); }
  pc = 0x455190; continue;
 }
 0x45510c => {
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4550f2; continue; }
  pc = 0x455112; continue;
 }
 0x455112 => {
  { cpu.popad(); }
  cpu.push(0x455118); f_004551dd(cpu); if cpu.returned_to != 0x455118 { return; }
  if cpu.cf { pc = 0x455190; continue; }
  pc = 0x45511a; continue;
 }
 0x45511a => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x20u8); }
  if cpu.zf { pc = 0x45512f; continue; }
  pc = 0x455120; continue;
 }
 0x455120 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455128); f_00478676(cpu); if cpu.returned_to != 0x455128 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x455190; continue; }
  pc = 0x45512f; continue;
 }
 0x45512f => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x40u8); }
  if cpu.zf { pc = 0x45516a; continue; }
  pc = 0x455135; continue;
 }
 0x455135 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x45513d); f_00478676(cpu); if cpu.returned_to != 0x45513d { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x455151; continue; }
  pc = 0x455147; continue;
 }
 0x455147 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x455190; continue; }
  pc = 0x455151; continue;
 }
 0x455151 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x455190; continue; }
  pc = 0x455158; continue;
 }
 0x455158 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x45516a; continue; }
  pc = 0x45515f; continue;
 }
 0x45515f => {
  { cpu.pushad(); }
  cpu.push(0x455165); f_0046d525(cpu); if cpu.returned_to != 0x455165 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x455190; continue; }
  pc = 0x45516a; continue;
 }
 0x45516a => {
  cpu.push(0x45516f); f_00455194(cpu); if cpu.returned_to != 0x45516f { return; }
  if cpu.cf { pc = 0x455190; continue; }
  pc = 0x455171; continue;
 }
 0x455171 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x45517a); f_00478676(cpu); if cpu.returned_to != 0x45517a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.push(0x455186); f_00455286(cpu); if cpu.returned_to != 0x455186 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x455192; continue;
 }
 0x455189 => {
  cpu.push(0x45518e); f_0045467e(cpu); if cpu.returned_to != 0x45518e { return; }
  if !cpu.cf { cpu.poll(); pc = 0x455171; continue; }
  pc = 0x455190; continue;
 }
 0x455190 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455192 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455194(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455194); }
 let mut pc: u32 = 0x455194;
 loop { match pc {
 0x455194 => {
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x45519e); f_00478676(cpu); if cpu.returned_to != 0x45519e { return; }
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
pub fn f_004551dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4551dd); }
 let mut pc: u32 = 0x4551dd;
 loop { match pc {
 0x4551dd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x4551e8); f_004786ae(cpu); if cpu.returned_to != 0x4551e8 { return; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4551ef; continue;
 }
 0x4551ef => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x45520a; continue; }
  pc = 0x4551f4; continue;
 }
 0x4551f4 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { cpu.and8(cpu.r8(0x509317u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x45520a; continue; }
  pc = 0x455204; continue;
 }
 0x455204 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45520a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4551ef; continue; }
  pc = 0x45520f; continue;
 }
 0x45520f => {
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
pub fn f_00455215(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455215); } r_00455215(cpu, 0x455215); }
pub fn f_00455244(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455244); } r_00455215(cpu, 0x455244); }
pub fn f_00455246(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455246); } r_00455215(cpu, 0x455246); }
pub fn f_00455249(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455249); } r_00455215(cpu, 0x455249); }
pub fn f_0045524f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45524f); } r_00455215(cpu, 0x45524f); }
fn r_00455215(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x455215 => {
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
  if cpu.zf { pc = 0x455239; continue; }
  pc = 0x455232; continue;
 }
 0x455232 => {
  cpu.push(0x455237); f_00455286(cpu); if cpu.returned_to != 0x455237 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455239 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  pc = 0x455246; continue;
 }
 0x455244 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x455246; continue;
 }
 0x455246 => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d68b2u32, v); }
  pc = 0x45524f; continue;
 }
 0x455249 => {
  { let v: u8 = 0x68u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  { cpu.scas(1, 0); }
  { let v = 0x8966004du32; cpu.push(v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x4d68b0u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x455268); f_00413337(cpu); if cpu.returned_to != 0x455268 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45524f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x455268); f_00413337(cpu); if cpu.returned_to != 0x455268 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455271(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455271); }
 let mut pc: u32 = 0x455271;
 loop { match pc {
 0x455271 => {
  cpu.push(0x455276); f_0044aa9c(cpu); if cpu.returned_to != 0x455276 { return; }
  if cpu.cf { pc = 0x455284; continue; }
  pc = 0x455278; continue;
 }
 0x455278 => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x455284; continue; }
  pc = 0x455282; continue;
 }
 0x455282 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455284 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455286); }
 let mut pc: u32 = 0x455286;
 loop { match pc {
 0x455286 => {
  { cpu.pushad(); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x455294; continue; }
  pc = 0x45528d; continue;
 }
 0x45528d => {
  cpu.push(0x455292); f_0047ff60(cpu); if cpu.returned_to != 0x455292 { return; }
  if !cpu.cf { pc = 0x455296; continue; }
  pc = 0x455294; continue;
 }
 0x455294 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x455296; continue;
 }
 0x455296 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x45529d); f_00455271(cpu); if cpu.returned_to != 0x45529d { return; }
  if !cpu.cf { pc = 0x4552a1; continue; }
  pc = 0x45529f; continue;
 }
 0x45529f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4552a1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x4552d0; continue; }
  pc = 0x4552bd; continue;
 }
 0x4552bd => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x48u32); cpu.ebx = v; }
  pc = 0x4552d0; continue;
 }
 0x4552d0 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x455307; continue; }
  pc = 0x4552d7; continue;
 }
 0x4552d7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4552df); f_00458a0c(cpu); if cpu.returned_to != 0x4552df { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x4552f1; continue; }
  pc = 0x4552e8; continue;
 }
 0x4552e8 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x78u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4552f1); f_00458a7f(cpu); if cpu.returned_to != 0x4552f1 { return; }
  pc = 0x4552f1; continue;
 }
 0x4552f1 => {
  cpu.push(0x4552f6); f_0048990c(cpu); if cpu.returned_to != 0x4552f6 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x455307; continue; }
  pc = 0x4552fd; continue;
 }
 0x4552fd => {
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455307 => {
  cpu.push(0x45530c); f_00478632(cpu); if cpu.returned_to != 0x45530c { return; }
  if cpu.cf { pc = 0x455317; continue; }
  pc = 0x45530e; continue;
 }
 0x45530e => {
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x455317; continue;
 }
 0x455317 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455319(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455319); } r_00455319(cpu, 0x455319); }
pub fn f_00455355(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455355); } r_00455319(cpu, 0x455355); }
fn r_00455319(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x455319 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x455323; continue;
 }
 0x455323 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x45535e; continue; }
  pc = 0x45532d; continue;
 }
 0x45532d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x45535e; continue; }
  pc = 0x455334; continue;
 }
 0x455334 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x455345); f_0047a2d9(cpu); if cpu.returned_to != 0x455345 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x45535e; continue; }
  pc = 0x45534d; continue;
 }
 0x45534d => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  pc = 0x455355; continue;
 }
 0x455355 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x45535c); f_00455368(cpu); if cpu.returned_to != 0x45535c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x45535e; continue;
 }
 0x45535e => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455323; continue; }
  pc = 0x455367; continue;
 }
 0x455367 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455368); }
 let mut pc: u32 = 0x455368;
 loop { match pc {
 0x455368 => {
  cpu.push(0x45536d); f_00455271(cpu); if cpu.returned_to != 0x45536d { return; }
  if cpu.cf { pc = 0x4553c2; continue; }
  pc = 0x45536f; continue;
 }
 0x45536f => {
  cpu.push(0x455374); f_00489968(cpu); if cpu.returned_to != 0x455374 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4553c2; continue; }
  pc = 0x45537b; continue;
 }
 0x45537b => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x45539d); f_0043fd98(cpu); if cpu.returned_to != 0x45539d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x4553c2; continue; }
  pc = 0x4553ad; continue;
 }
 0x4553ad => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  cpu.push(0x4553bd); f_00458a0c(cpu); if cpu.returned_to != 0x4553bd { return; }
  cpu.push(0x4553c2); f_0048990c(cpu); if cpu.returned_to != 0x4553c2 { return; }
  pc = 0x4553c2; continue;
 }
 0x4553c2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004553c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4553c3); }
 let mut pc: u32 = 0x4553c3;
 loop { match pc {
 0x4553c3 => {
  { cpu.pushad(); }
  cpu.push(0x4553c9); f_00455271(cpu); if cpu.returned_to != 0x4553c9 { return; }
  if cpu.cf { pc = 0x455410; continue; }
  pc = 0x4553cb; continue;
 }
 0x4553cb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x4553fa; continue; }
  pc = 0x4553e7; continue;
 }
 0x4553e7 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  pc = 0x4553fa; continue;
 }
 0x4553fa => {
  { cpu.pushad(); }
  cpu.push(0x455400); f_004496ea(cpu); if cpu.returned_to != 0x455400 { return; }
  cpu.push(0x455405); f_004496fc(cpu); if cpu.returned_to != 0x455405 { return; }
  { cpu.popad(); }
  cpu.push(0x45540b); f_00458a0c(cpu); if cpu.returned_to != 0x45540b { return; }
  cpu.push(0x455410); f_0048990c(cpu); if cpu.returned_to != 0x455410 { return; }
  pc = 0x455410; continue;
 }
 0x455410 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455412); }
 let mut pc: u32 = 0x455412;
 loop { match pc {
 0x455412 => {
  { let v: u32 = 0x5b5034u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b99bcu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x455432; continue; }
  pc = 0x455421; continue;
 }
 0x455421 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x45542a); f_00455433(cpu); if cpu.returned_to != 0x45542a { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455421; continue; }
  pc = 0x455432; continue;
 }
 0x455432 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455433(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455433); } r_00455433(cpu, 0x455433); }
pub fn f_00455449(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455449); } r_00455433(cpu, 0x455449); }
pub fn f_0045544b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45544b); } r_00455433(cpu, 0x45544b); }
fn r_00455433(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x455433 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455439); f_00455719(cpu); if cpu.returned_to != 0x455439 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x45544c; continue; }
  pc = 0x45543c; continue;
 }
 0x45543c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455442); f_0045566e(cpu); if cpu.returned_to != 0x455442 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x455448); f_0045544e(cpu); if cpu.returned_to != 0x455448 { return; }
  if cpu.cf { pc = 0x45544c; continue; }
  pc = 0x45544a; continue;
 }
 0x455449 => {
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x45544b; continue;
 }
 0x45544a => {
  { cpu.cf = false; }
  pc = 0x45544b; continue;
 }
 0x45544b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45544c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045544e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45544e); } r_0045544e(cpu, 0x45544e); }
pub fn f_00455452(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455452); } r_0045544e(cpu, 0x455452); }
pub fn f_00455454(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455454); } r_0045544e(cpu, 0x455454); }
fn r_0045544e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45544e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455456); f_00455644(cpu); if cpu.returned_to != 0x455456 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b99e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455462); f_00455617(cpu); if cpu.returned_to != 0x455462 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x45546e; continue; }
  pc = 0x455465; continue;
 }
 0x455452 => {
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x5b99e0u32.wrapping_add(cpu.ebx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x5b99e0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455462); f_00455617(cpu); if cpu.returned_to != 0x455462 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x45546e; continue; }
  pc = 0x455465; continue;
 }
 0x455454 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5b99e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455462); f_00455617(cpu); if cpu.returned_to != 0x455462 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x45546e; continue; }
  pc = 0x455465; continue;
 }
 0x455465 => {
  cpu.push(0x45546a); f_0045551b(cpu); if cpu.returned_to != 0x45546a { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45546e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x455474); f_00454937(cpu); if cpu.returned_to != 0x455474 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x455497; continue; }
  pc = 0x455477; continue;
 }
 0x455477 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x19u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x455497; continue; }
  pc = 0x455482; continue;
 }
 0x455482 => {
  cpu.push(0x455487); f_004554fc(cpu); if cpu.returned_to != 0x455487 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x45548d); f_00455617(cpu); if cpu.returned_to != 0x45548d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x455497; continue; }
  pc = 0x455490; continue;
 }
 0x455490 => {
  cpu.push(0x455495); f_0045551b(cpu); if cpu.returned_to != 0x455495 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455497 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004554fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4554fc); }
 let mut pc: u32 = 0x4554fc;
 loop { match pc {
 0x4554fc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b99e4u32); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5b5234u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b99e4u32)); cpu.w32(0x5b99e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045551b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45551b); }
 let mut pc: u32 = 0x45551b;
 loop { match pc {
 0x45551b => {
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = 0x5b5234u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x455528; continue;
 }
 0x455528 => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.eax = v; }
  cpu.push(0x455532); f_00454ba0(cpu); if cpu.returned_to != 0x455532 { return; }
  { cpu.sub32(cpu.r32(0x5b99e0u32), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x455570; continue; }
  pc = 0x45553a; continue;
 }
 0x45553a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x455555); f_00478676(cpu); if cpu.returned_to != 0x455555 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { cpu.pushad(); }
  cpu.push(0x455566); f_00479a1a(cpu); if cpu.returned_to != 0x455566 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x455528; continue; }
  pc = 0x455570; continue;
 }
 0x455570 => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  cpu.push(0x45557b); f_0045557c(cpu); if cpu.returned_to != 0x45557b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045557c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45557c); }
 let mut pc: u32 = 0x45557c;
 loop { match pc {
 0x45557c => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x455616; continue; }
  pc = 0x455586; continue;
 }
 0x455586 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  cpu.push(0x455591); f_00479c13(cpu); if cpu.returned_to != 0x455591 { return; }
  { let v: u32 = cpu.r32(0x5d39d4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x455616; continue; }
  pc = 0x45559b; continue;
 }
 0x45559b => {
  { let v: u32 = 0x5d31c8u32; cpu.esi = v; }
  pc = 0x4555a0; continue;
 }
 0x4555a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4555ac; continue;
 }
 0x4555ac => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x455603; continue; }
  pc = 0x4555b3; continue;
 }
 0x4555b3 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x455603; continue; }
  pc = 0x4555ba; continue;
 }
 0x4555ba => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x455603; continue; }
  pc = 0x4555c1; continue;
 }
 0x4555c1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x455603; continue; }
  pc = 0x4555c7; continue;
 }
 0x4555c7 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x4555d9; continue; }
  pc = 0x4555ce; continue;
 }
 0x4555ce => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x455603; continue; }
  pc = 0x4555d9; continue;
 }
 0x4555d9 => {
  { cpu.pushad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x4555f3); f_00478676(cpu); if cpu.returned_to != 0x4555f3 { return; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x455602); f_00479a1a(cpu); if cpu.returned_to != 0x455602 { return; }
  { cpu.popad(); }
  pc = 0x455603; continue;
 }
 0x455603 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x4555ac; continue; }
  pc = 0x45560e; continue;
 }
 0x45560e => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4555a0; continue; }
  pc = 0x455616; continue;
 }
 0x455616 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455617(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455617); }
 let mut pc: u32 = 0x455617;
 loop { match pc {
 0x455617 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5b5234u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b99e4u32); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x455627; continue;
 }
 0x455627 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5b99e0u32)); }
  if !cpu.cf { pc = 0x45563f; continue; }
  pc = 0x455636; continue;
 }
 0x455636 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455627; continue; }
  pc = 0x45563c; continue;
 }
 0x45563c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45563f => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455644(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455644); }
 let mut pc: u32 = 0x455644;
 loop { match pc {
 0x455644 => {
  cpu.push(0x455649); f_0045565f(cpu); if cpu.returned_to != 0x455649 { return; }
  if cpu.cf { pc = 0x455659; continue; }
  pc = 0x45564b; continue;
 }
 0x45564b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x19u32.wrapping_add(cpu.eax))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455659 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045565f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45565f); }
 let mut pc: u32 = 0x45565f;
 loop { match pc {
 0x45565f => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x45566b; continue; }
  pc = 0x455668; continue;
 }
 0x455668 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45566b => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045566e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45566e); }
 let mut pc: u32 = 0x45566e;
 loop { match pc {
 0x45566e => {
  { cpu.sub32(cpu.r32(0x5b99e4u32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4556af; continue; }
  pc = 0x455677; continue;
 }
 0x455677 => {
  cpu.push(0x45567c); f_004556b0(cpu); if cpu.returned_to != 0x45567c { return; }
  { let v: u8 = cpu.r8(0x5b99e4u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x455682; continue;
 }
 0x455682 => {
  { let v: u8 = cpu.r8(0x5b99e4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5b5234u32; cpu.esi = v; }
  pc = 0x45568f; continue;
 }
 0x45568f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.cf || cpu.zf) { pc = 0x4556a4; continue; }
  pc = 0x455697; continue;
 }
 0x455697 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x4556a4; continue;
 }
 0x4556a4 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x45568f; continue; }
  pc = 0x4556ab; continue;
 }
 0x4556ab => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x455682; continue; }
  pc = 0x4556af; continue;
 }
 0x4556af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004556b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4556b0); }
 let mut pc: u32 = 0x4556b0;
 loop { match pc {
 0x4556b0 => {
  { let v: u32 = 0x5b5234u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5b99e4u32); cpu.ecx = v; }
  pc = 0x4556bb; continue;
 }
 0x4556bb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x4556ce); f_00478676(cpu); if cpu.returned_to != 0x4556ce { return; }
  if cpu.cf { pc = 0x455710; continue; }
  pc = 0x4556d0; continue;
 }
 0x4556d0 => {
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
  pc = 0x455710; continue;
 }
 0x455710 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4556bb; continue; }
  pc = 0x455718; continue;
 }
 0x455718 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455719(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455719); }
 let mut pc: u32 = 0x455719;
 loop { match pc {
 0x455719 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x5b5234u32; cpu.edi = v; }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b99e4u32, v); }
  pc = 0x455734; continue;
 }
 0x455734 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x45573e; continue;
 }
 0x45573e => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x455748; continue;
 }
 0x455748 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x455752; continue;
 }
 0x455752 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x45575c; continue;
 }
 0x45575c => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x455766; continue;
 }
 0x455766 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x455805; continue; }
  pc = 0x455770; continue;
 }
 0x455770 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edx)), 0x20u8); }
  if cpu.zf { pc = 0x455789; continue; }
  pc = 0x455776; continue;
 }
 0x455776 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x45577d); f_00478676(cpu); if cpu.returned_to != 0x45577d { return; }
  if cpu.cf { pc = 0x455805; continue; }
  pc = 0x455783; continue;
 }
 0x455783 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x455789; continue;
 }
 0x455789 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edx)), 0x40u8); }
  if cpu.zf { pc = 0x4557ca; continue; }
  pc = 0x45578f; continue;
 }
 0x45578f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455796); f_00478676(cpu); if cpu.returned_to != 0x455796 { return; }
  if cpu.cf { pc = 0x455805; continue; }
  pc = 0x455798; continue;
 }
 0x455798 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x4557a7; continue; }
  pc = 0x45579e; continue;
 }
 0x45579e => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x455805; continue; }
  pc = 0x4557a7; continue;
 }
 0x4557a7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x22u8); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x4557ad; continue;
 }
 0x4557ad => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x4557bf; continue; }
  pc = 0x4557b3; continue;
 }
 0x4557b3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x4557bf; continue; }
  pc = 0x4557b9; continue;
 }
 0x4557b9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4557ca; continue; }
  pc = 0x4557bf; continue;
 }
 0x4557bf => {
  { cpu.pushad(); }
  cpu.push(0x4557c5); f_0046d525(cpu); if cpu.returned_to != 0x4557c5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x455805; continue; }
  pc = 0x4557ca; continue;
 }
 0x4557ca => {
  { cpu.sub8(cpu.r8(0x5b99f8u32), 0x0u8); }
  if cpu.zf { pc = 0x4557fa; continue; }
  pc = 0x4557d3; continue;
 }
 0x4557d3 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x4557dc); f_00479c13(cpu); if cpu.returned_to != 0x4557dc { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d39d4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4557f6; continue; }
  pc = 0x4557e7; continue;
 }
 0x4557e7 => {
  { let v: u32 = 0x5d31c8u32; cpu.eax = v; }
  pc = 0x4557ec; continue;
 }
 0x4557ec => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); }
  if cpu.zf { pc = 0x4557f9; continue; }
  pc = 0x4557f0; continue;
 }
 0x4557f0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4557ec; continue; }
  pc = 0x4557f6; continue;
 }
 0x4557f6 => {
  { cpu.popad(); }
  pc = 0x455805; continue;
 }
 0x4557f9 => {
  { cpu.popad(); }
  pc = 0x4557fa; continue;
 }
 0x4557fa => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b99e4u32)); cpu.w32(0x5b99e4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x455805; continue;
 }
 0x455805 => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455734; continue; }
  pc = 0x45580f; continue;
 }
 0x45580f => {
  { cpu.sub32(cpu.r32(0x5b99e4u32), 0x0u32); }
  if cpu.zf { pc = 0x45581a; continue; }
  pc = 0x455818; continue;
 }
 0x455818 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45581a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045581c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45581c); }
 let mut pc: u32 = 0x45581c;
 loop { match pc {
 0x45581c => {
  cpu.push(0x455821); f_0045597b(cpu); if cpu.returned_to != 0x455821 { return; }
  cpu.push(0x455826); f_004558db(cpu); if cpu.returned_to != 0x455826 { return; }
  cpu.push(0x45582b); f_004558a2(cpu); if cpu.returned_to != 0x45582b { return; }
  cpu.push(0x455830); f_00455884(cpu); if cpu.returned_to != 0x455830 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455884(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455884); }
 let mut pc: u32 = 0x455884;
 loop { match pc {
 0x455884 => {
  { let v: u32 = 0x5b5034u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b99bcu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4558a1; continue; }
  pc = 0x455893; continue;
 }
 0x455893 => {
  { let v: u32 = 0xffffffc0u32; cpu.eax = v; }
  pc = 0x455898; continue;
 }
 0x455898 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)) & cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455898; continue; }
  pc = 0x4558a1; continue;
 }
 0x4558a1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004558a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4558a2); }
 let mut pc: u32 = 0x4558a2;
 loop { match pc {
 0x4558a2 => {
  { let v: u8 = cpu.r8(0x5b99bcu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4558da; continue; }
  pc = 0x4558ad; continue;
 }
 0x4558ad => {
  { let v: u32 = 0x5b5034u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5b99bcu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4558ba; continue;
 }
 0x4558ba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.cf { pc = 0x4558cf; continue; }
  pc = 0x4558c2; continue;
 }
 0x4558c2 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x4558cf; continue;
 }
 0x4558cf => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4558ba; continue; }
  pc = 0x4558d6; continue;
 }
 0x4558d6 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x4558ad; continue; }
  pc = 0x4558da; continue;
 }
 0x4558da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004558db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4558db); }
 let mut pc: u32 = 0x4558db;
 loop { match pc {
 0x4558db => {
  { cpu.sub8(cpu.r8(0x5b99f6u32), 0x0u8); }
  if !cpu.zf { pc = 0x45597a; continue; }
  pc = 0x4558e8; continue;
 }
 0x4558e8 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x4558f2; continue;
 }
 0x4558f2 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x455916; continue; }
  pc = 0x4558f9; continue;
 }
 0x4558f9 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x455916; continue; }
  pc = 0x4558ff; continue;
 }
 0x4558ff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x45591c; continue;
 }
 0x455916 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4558f2; continue; }
  pc = 0x45591c; continue;
 }
 0x45591c => {
  { let v: u32 = 0x5b5034u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b99bcu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x45597a; continue; }
  pc = 0x45592b; continue;
 }
 0x45592b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x4d6bd8u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x4d6bdcu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3ffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x45595e; continue; }
  pc = 0x455959; continue;
 }
 0x455959 => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x45595e; continue;
 }
 0x45595e => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x45592b; continue; }
  pc = 0x45597a; continue;
 }
 0x45597a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045597b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45597b); }
 let mut pc: u32 = 0x45597b;
 loop { match pc {
 0x45597b => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x5b5034u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b99bcu32, v); }
  pc = 0x455996; continue;
 }
 0x455996 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x4559c2; continue; }
  pc = 0x45599d; continue;
 }
 0x45599d => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x4559c2; continue; }
  pc = 0x4559a3; continue;
 }
 0x4559a3 => {
  cpu.push(0x4559a8); f_004559c9(cpu); if cpu.returned_to != 0x4559a8 { return; }
  if cpu.cf { pc = 0x4559c2; continue; }
  pc = 0x4559aa; continue;
 }
 0x4559aa => {
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x4559b4; continue; }
  pc = 0x4559b2; continue;
 }
 0x4559b2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4559b4; continue;
 }
 0x4559b4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b99bcu32)); cpu.w32(0x5b99bcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4559c2; continue;
 }
 0x4559c2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455996; continue; }
  pc = 0x4559c8; continue;
 }
 0x4559c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004559c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4559c9); }
 let mut pc: u32 = 0x4559c9;
 loop { match pc {
 0x4559c9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4559db); f_00454fea(cpu); if cpu.returned_to != 0x4559db { return; }
  if cpu.cf { pc = 0x4559fd; continue; }
  pc = 0x4559dd; continue;
 }
 0x4559dd => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4559e4); f_00454fea(cpu); if cpu.returned_to != 0x4559e4 { return; }
  if cpu.cf { pc = 0x4559fd; continue; }
  pc = 0x4559e6; continue;
 }
 0x4559e6 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4559ed); f_00454fea(cpu); if cpu.returned_to != 0x4559ed { return; }
  if cpu.cf { pc = 0x4559fd; continue; }
  pc = 0x4559ef; continue;
 }
 0x4559ef => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4559f6); f_00454fea(cpu); if cpu.returned_to != 0x4559f6 { return; }
  if cpu.cf { pc = 0x4559fd; continue; }
  pc = 0x4559f8; continue;
 }
 0x4559f8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4559fd => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455b74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455b74); }
 let mut pc: u32 = 0x455b74;
 loop { match pc {
 0x455b74 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455b7f; continue;
 }
 0x455b7f => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455b89; continue;
 }
 0x455b89 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455b93; continue;
 }
 0x455b93 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x455b9c); f_0045502b(cpu); if cpu.returned_to != 0x455b9c { return; }
  if !cpu.cf { pc = 0x455bcf; continue; }
  pc = 0x455b9e; continue;
 }
 0x455b9e => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x455ba5); f_00478676(cpu); if cpu.returned_to != 0x455ba5 { return; }
  if cpu.cf { pc = 0x455d0a; continue; }
  pc = 0x455bab; continue;
 }
 0x455bab => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x455bbb; continue; }
  pc = 0x455bb1; continue;
 }
 0x455bb1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455bbb; continue;
 }
 0x455bbb => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x455b93; continue;
 }
 0x455bcf => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x455bf2; continue; }
  pc = 0x455bd5; continue;
 }
 0x455bd5 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x455bf2; continue; }
  pc = 0x455bdb; continue;
 }
 0x455bdb => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455be2); f_004786ae(cpu); if cpu.returned_to != 0x455be2 { return; }
  if cpu.cf { pc = 0x455d0a; continue; }
  pc = 0x455be8; continue;
 }
 0x455be8 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x455d0a; continue; }
  pc = 0x455bf2; continue;
 }
 0x455bf2 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455bfc; continue;
 }
 0x455bfc => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455c06; continue;
 }
 0x455c06 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455c14; continue;
 }
 0x455c14 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455c1e; continue;
 }
 0x455c1e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x455c26); f_0047a0de(cpu); if cpu.returned_to != 0x455c26 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x455d0a; continue; }
  pc = 0x455c31; continue;
 }
 0x455c31 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x455ce3; continue; }
  pc = 0x455c39; continue;
 }
 0x455c39 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455c44; continue;
 }
 0x455c44 => {
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x455c5e; continue; }
  pc = 0x455c4a; continue;
 }
 0x455c4a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x455c5e; continue; }
  pc = 0x455c51; continue;
 }
 0x455c51 => {
  cpu.push(0x455c56); f_0046d525(cpu); if cpu.returned_to != 0x455c56 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455c5e; continue;
 }
 0x455c5e => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x455c79); f_0047f60f(cpu); if cpu.returned_to != 0x455c79 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { pc = 0x455c83; continue; }
  pc = 0x455c81; continue;
 }
 0x455c81 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x455c83; continue;
 }
 0x455c83 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { cpu.pushad(); }
  cpu.push(0x455cb6); f_00413337(cpu); if cpu.returned_to != 0x455cb6 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x455cbd); f_00478632(cpu); if cpu.returned_to != 0x455cbd { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)) | 0x4u8; cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x455cd6); f_0047a2d9(cpu); if cpu.returned_to != 0x455cd6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x18u8); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x455ce3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x455d0a; continue; }
  pc = 0x455cea; continue;
 }
 0x455cea => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { cpu.poll(); pc = 0x455c39; continue; }
  pc = 0x455cf2; continue;
 }
 0x455cf2 => {
  pc = 0x455d0a; continue;
 }
 0x455d0a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455d0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455d0c); } r_00455d0c(cpu, 0x455d0c); }
pub fn f_00455d25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455d25); } r_00455d0c(cpu, 0x455d25); }
pub fn f_00455d3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455d3e); } r_00455d0c(cpu, 0x455d3e); }
pub fn f_00455d71(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455d71); } r_00455d0c(cpu, 0x455d71); }
pub fn f_00455da4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455da4); } r_00455d0c(cpu, 0x455da4); }
fn r_00455d0c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x455d0c => {
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455e00u32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455d25 => {
  { let v: u32 = 0x455e00u32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455d3e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x455d5b; continue; }
  pc = 0x455d45; continue;
 }
 0x455d45 => {
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455e00u32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455d5b => {
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455dfeu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455d71 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x455d8e; continue; }
  pc = 0x455d78; continue;
 }
 0x455d78 => {
  { let v: u32 = 0x455e00u32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455d8e => {
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455dfeu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455da4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x455dc1; continue; }
  pc = 0x455dab; continue;
 }
 0x455dab => {
  { let v: u32 = 0x455e00u32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455dc1 => {
  { let v: u32 = 0x455ffbu32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x455dfeu32; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x455dd7; continue;
 }
 0x455dd7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x455de2); f_0041c743(cpu); if cpu.returned_to != 0x455de2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5d627cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x455df8; continue; }
  pc = 0x455df2; continue;
 }
 0x455df2 => {
  cpu.poll(); pc = cpu.r32(0x4d6bd8u32); continue;
 }
 0x455df8 => {
  cpu.poll(); pc = cpu.r32(0x4d6bdcu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455dfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455dfe); }
 let mut pc: u32 = 0x455dfe;
 loop { match pc {
 0x455dfe => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455e00); }
 let mut pc: u32 = 0x455e00;
 loop { match pc {
 0x455e00 => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { cpu.pushad(); }
  cpu.push(0x455e48); f_00413337(cpu); if cpu.returned_to != 0x455e48 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455e4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x455e4b); }
 let mut pc: u32 = 0x455e4b;
 loop { match pc {
 0x455e4b => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x455e55; continue;
 }
 0x455e55 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e5f; continue;
 }
 0x455e5f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e69; continue;
 }
 0x455e69 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e73; continue;
 }
 0x455e73 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x455e83; continue; }
  pc = 0x455e79; continue;
 }
 0x455e79 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e83; continue;
 }
 0x455e83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e8e; continue;
 }
 0x455e8e => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455e98; continue;
 }
 0x455e98 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455e9f); f_00478676(cpu); if cpu.returned_to != 0x455e9f { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x455ee6; continue; }
  pc = 0x455ea5; continue;
 }
 0x455ea5 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x455ee6; continue; }
  pc = 0x455eab; continue;
 }
 0x455eab => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455eb1; continue;
 }
 0x455eb1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455eb7; continue;
 }
 0x455eb7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1eu8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ebd; continue;
 }
 0x455ebd => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x6u8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ec3; continue;
 }
 0x455ec3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1au8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ec9; continue;
 }
 0x455ec9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x19u8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ecf; continue;
 }
 0x455ecf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1eu8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ed5; continue;
 }
 0x455ed5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1bu8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455edb; continue;
 }
 0x455edb => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x18u8); }
  if cpu.zf { pc = 0x455ef9; continue; }
  pc = 0x455ee1; continue;
 }
 0x455ee1 => {
  pc = 0x455ff0; continue;
 }
 0x455ee6 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x455eef); f_0046d525(cpu); if cpu.returned_to != 0x455eef { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455ef9; continue;
 }
 0x455ef9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455f05; continue;
 }
 0x455f05 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455f11; continue;
 }
 0x455f11 => {
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
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6be4u32, v); }
  cpu.push(0x455f50); f_0045be05(cpu); if cpu.returned_to != 0x455f50 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x55755du32; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x455f59; continue;
 }
 0x455f59 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x455fac; continue; }
  pc = 0x455f5e; continue;
 }
 0x455f5e => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x455fa9; continue; }
  pc = 0x455f75; continue;
 }
 0x455f75 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6bd8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x455fa7; continue; }
  pc = 0x455f85; continue;
 }
 0x455f85 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6be0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x455fa7; continue; }
  pc = 0x455f8d; continue;
 }
 0x455f8d => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6bdcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x455fa7; continue; }
  pc = 0x455f99; continue;
 }
 0x455f99 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6be4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x455fa7; continue; }
  pc = 0x455fa1; continue;
 }
 0x455fa1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x455fa9; continue;
 }
 0x455fa7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x455fa9; continue;
 }
 0x455fa9 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x455f59; continue;
 }
 0x455fac => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x455ff0; continue; }
  pc = 0x455fb0; continue;
 }
 0x455fb0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x455fb9; continue;
 }
 0x455fb9 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x455fe9; continue; }
  pc = 0x455fc2; continue;
 }
 0x455fc2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.sf { pc = 0x455fe9; continue; }
  pc = 0x455fc9; continue;
 }
 0x455fc9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x455fe9; continue; }
  pc = 0x455fcf; continue;
 }
 0x455fcf => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x455fda); f_00478676(cpu); if cpu.returned_to != 0x455fda { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x455fe1); f_00455ffb(cpu); if cpu.returned_to != 0x455fe1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x6u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  pc = 0x455ff0; continue;
 }
 0x455fe9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x455fb9; continue; }
  pc = 0x455ff0; continue;
 }
 0x455ff0 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x455e55; continue; }
  pc = 0x455ffa; continue;
 }
 0x455ffa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00455ffb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x455ffb); } r_00455ffb(cpu, 0x455ffb); }
pub fn f_00456000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x456000); } r_00455ffb(cpu, 0x456000); }
fn r_00455ffb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x455ffb => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  pc = 0x456000; continue;
 }
 0x456000 => {
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0xd80u16; cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b2u32, v); }
  { cpu.pushad(); }
  cpu.push(0x456043); f_00413337(cpu); if cpu.returned_to != 0x456043 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456850); }
 let mut pc: u32 = 0x456850;
 loop { match pc {
 0x456850 => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x0u32); }
  if !cpu.zf { pc = 0x456877; continue; }
  pc = 0x456859; continue;
 }
 0x456859 => {
  { let v: u32 = 0xddu32; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = 0x5b388bu32; cpu.w32(0x5b3590u32, v); }
  { let v: u32 = 0x4d2u32; cpu.w32(0x5b3594u32, v); }
  pc = 0x456877; continue;
 }
 0x456877 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456878(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456878); }
 let mut pc: u32 = 0x456878;
 loop { match pc {
 0x456878 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x4568a5); f_004714a5(cpu); if cpu.returned_to != 0x4568a5 { return; }
  if !cpu.cf { pc = 0x4568b9; continue; }
  pc = 0x4568a7; continue;
 }
 0x4568a7 => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0x0u8); }
  if cpu.zf { pc = 0x456964; continue; }
  pc = 0x4568b4; continue;
 }
 0x4568b4 => {
  pc = 0x456961; continue;
 }
 0x4568b9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x4568db); f_004714a5(cpu); if cpu.returned_to != 0x4568db { return; }
  if !cpu.cf { pc = 0x4568e8; continue; }
  pc = 0x4568dd; continue;
 }
 0x4568dd => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if cpu.zf { pc = 0x456964; continue; }
  pc = 0x4568e6; continue;
 }
 0x4568e6 => {
  pc = 0x456961; continue;
 }
 0x4568e8 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x456961; continue; }
  pc = 0x4568f1; continue;
 }
 0x4568f1 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456961; continue; }
  pc = 0x4568fb; continue;
 }
 0x4568fb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x45691d); f_004714a5(cpu); if cpu.returned_to != 0x45691d { return; }
  if !cpu.cf { pc = 0x45692a; continue; }
  pc = 0x45691f; continue;
 }
 0x45691f => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if cpu.zf { pc = 0x456964; continue; }
  pc = 0x456928; continue;
 }
 0x456928 => {
  pc = 0x456961; continue;
 }
 0x45692a => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456961; continue; }
  pc = 0x456934; continue;
 }
 0x456934 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x456956); f_004714a5(cpu); if cpu.returned_to != 0x456956 { return; }
  if !cpu.cf { pc = 0x456961; continue; }
  pc = 0x456958; continue;
 }
 0x456958 => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0x0u8); }
  if cpu.zf { pc = 0x456964; continue; }
  pc = 0x456961; continue;
 }
 0x456961 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x456964 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456a91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456a91); }
 let mut pc: u32 = 0x456a91;
 loop { match pc {
 0x456a91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456b09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456b09); }
 let mut pc: u32 = 0x456b09;
 loop { match pc {
 0x456b09 => {
  { let v: u32 = 0x56d39du32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x456b13; continue;
 }
 0x456b13 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x456b22; continue; }
  pc = 0x456b1b; continue;
 }
 0x456b1b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x456b13; continue; }
  pc = 0x456b20; continue;
 }
 0x456b20 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x456b22 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456b24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456b24); }
 let mut pc: u32 = 0x456b24;
 loop { match pc {
 0x456b24 => {
  { let v: u32 = 0x56d39du32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x456b2e; continue;
 }
 0x456b2e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x456b2e; continue; }
  pc = 0x456b39; continue;
 }
 0x456b39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456b3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456b3a); }
 let mut pc: u32 = 0x456b3a;
 loop { match pc {
 0x456b3a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b9a2cu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b9a2eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x5b9a0cu32; cpu.esi = v; }
  pc = 0x456b57; continue;
 }
 0x456b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b9a2cu32)); }
  if !cpu.zf { pc = 0x456b78; continue; }
  pc = 0x456b6f; continue;
 }
 0x456b6f => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b9a2eu32)); }
  if cpu.zf { pc = 0x456b85; continue; }
  pc = 0x456b78; continue;
 }
 0x456b78 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x456b57; continue; }
  pc = 0x456b81; continue;
 }
 0x456b81 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x456b85 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456b9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456b9c); }
 let mut pc: u32 = 0x456b9c;
 loop { match pc {
 0x456b9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456b9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456b9e); }
 let mut pc: u32 = 0x456b9e;
 loop { match pc {
 0x456b9e => {
  cpu.push(0x456ba3); f_0045700d(cpu); if cpu.returned_to != 0x456ba3 { return; }
  if cpu.cf { pc = 0x456ca0; continue; }
  pc = 0x456ba9; continue;
 }
 0x456ba9 => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if cpu.zf { pc = 0x456ca0; continue; }
  pc = 0x456bb6; continue;
 }
 0x456bb6 => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0x2u8); }
  if cpu.zf { pc = 0x456ca0; continue; }
  pc = 0x456bc3; continue;
 }
 0x456bc3 => {
  cpu.push(0x456bc8); f_0044d980(cpu); if cpu.returned_to != 0x456bc8 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x56d30au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e156fu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d306u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fccu32, v); }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if !cpu.zf { pc = 0x456c08; continue; }
  pc = 0x456bf5; continue;
 }
 0x456bf5 => {
  cpu.push(0x456bfa); f_0044d980(cpu); if cpu.returned_to != 0x456bfa { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3962u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5824a9u32, v); }
  pc = 0x456c08; continue;
 }
 0x456c08 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x456c56; continue; }
  pc = 0x456c11; continue;
 }
 0x456c11 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581bddu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x456c2c; continue; }
  pc = 0x456c1d; continue;
 }
 0x456c1d => {
  { let v: u32 = 0x581bd9u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581bddu32, v); }
  pc = 0x456c2c; continue;
 }
 0x456c2c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456c56; continue; }
  pc = 0x456c38; continue;
 }
 0x456c38 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456c4b); f_0047a2d9(cpu); if cpu.returned_to != 0x456c4b { return; }
  { let v: u32 = cpu.r32(0x56d30au32); cpu.eax = v; }
  cpu.push(0x456c55); f_0047232c(cpu); if cpu.returned_to != 0x456c55 { return; }
  { cpu.popad(); }
  pc = 0x456c56; continue;
 }
 0x456c56 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456c69); f_00456fce(cpu); if cpu.returned_to != 0x456c69 { return; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x456c70); f_00456b9c(cpu); if cpu.returned_to != 0x456c70 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x456c77); f_0045165b(cpu); if cpu.returned_to != 0x456c77 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456c8a); f_00456fa5(cpu); if cpu.returned_to != 0x456c8a { return; }
  cpu.push(0x456c8f); f_00456a92(cpu); if cpu.returned_to != 0x456c8f { return; }
  cpu.push(0x456c94); f_0045700d(cpu); if cpu.returned_to != 0x456c94 { return; }
  if cpu.cf { pc = 0x456ca0; continue; }
  pc = 0x456c96; continue;
 }
 0x456c96 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  pc = 0x456ca0; continue;
 }
 0x456ca0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456ca1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456ca1); }
 let mut pc: u32 = 0x456ca1;
 loop { match pc {
 0x456ca1 => {
  cpu.push(0x456ca6); f_0045700d(cpu); if cpu.returned_to != 0x456ca6 { return; }
  if cpu.cf { pc = 0x456da5; continue; }
  pc = 0x456cac; continue;
 }
 0x456cac => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0x0u8); }
  if cpu.zf { pc = 0x456da5; continue; }
  pc = 0x456cb9; continue;
 }
 0x456cb9 => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0x2u8); }
  if cpu.zf { pc = 0x456da5; continue; }
  pc = 0x456cc6; continue;
 }
 0x456cc6 => {
  cpu.push(0x456ccb); f_0044d980(cpu); if cpu.returned_to != 0x456ccb { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fcdu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x56d30au32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e156fu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d307u32, v); }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x1u8); }
  if !cpu.zf { pc = 0x456d0b; continue; }
  pc = 0x456cf8; continue;
 }
 0x456cf8 => {
  cpu.push(0x456cfd); f_0044d980(cpu); if cpu.returned_to != 0x456cfd { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3962u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5824a9u32, v); }
  pc = 0x456d0b; continue;
 }
 0x456d0b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x456d54; continue; }
  pc = 0x456d14; continue;
 }
 0x456d14 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581bddu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x456d2f; continue; }
  pc = 0x456d20; continue;
 }
 0x456d20 => {
  { let v: u32 = 0x581bd9u32; cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581bddu32, v); }
  pc = 0x456d2f; continue;
 }
 0x456d2f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456d54; continue; }
  pc = 0x456d3b; continue;
 }
 0x456d3b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456d4e); f_0047a2d9(cpu); if cpu.returned_to != 0x456d4e { return; }
  cpu.push(0x456d53); f_0047232c(cpu); if cpu.returned_to != 0x456d53 { return; }
  { cpu.popad(); }
  pc = 0x456d54; continue;
 }
 0x456d54 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456d6b); f_00456fce(cpu); if cpu.returned_to != 0x456d6b { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x456d75); f_00456b9c(cpu); if cpu.returned_to != 0x456d75 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x456d7c); f_0045165b(cpu); if cpu.returned_to != 0x456d7c { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456d8f); f_00456fa5(cpu); if cpu.returned_to != 0x456d8f { return; }
  cpu.push(0x456d94); f_00456a92(cpu); if cpu.returned_to != 0x456d94 { return; }
  cpu.push(0x456d99); f_0045700d(cpu); if cpu.returned_to != 0x456d99 { return; }
  if cpu.cf { pc = 0x456da5; continue; }
  pc = 0x456d9b; continue;
 }
 0x456d9b => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  pc = 0x456da5; continue;
 }
 0x456da5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456da6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456da6); }
 let mut pc: u32 = 0x456da6;
 loop { match pc {
 0x456da6 => {
  cpu.push(0x456dab); f_0045700d(cpu); if cpu.returned_to != 0x456dab { return; }
  if cpu.cf { pc = 0x456ea3; continue; }
  pc = 0x456db1; continue;
 }
 0x456db1 => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if cpu.zf { pc = 0x456ea3; continue; }
  pc = 0x456dbe; continue;
 }
 0x456dbe => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0x2u8); }
  if cpu.zf { pc = 0x456ea3; continue; }
  pc = 0x456dcb; continue;
 }
 0x456dcb => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fceu32, v); }
  cpu.push(0x456dd7); f_0044d980(cpu); if cpu.returned_to != 0x456dd7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x56d30au32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x4e156fu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d308u32, v); }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x2u8); }
  if !cpu.zf { pc = 0x456e10; continue; }
  pc = 0x456dfd; continue;
 }
 0x456dfd => {
  cpu.push(0x456e02); f_0044d980(cpu); if cpu.returned_to != 0x456e02 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5824a9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3962u32, v); }
  pc = 0x456e10; continue;
 }
 0x456e10 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x456e59; continue; }
  pc = 0x456e19; continue;
 }
 0x456e19 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581bddu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x456e34; continue; }
  pc = 0x456e25; continue;
 }
 0x456e25 => {
  { let v: u32 = 0x581bd9u32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581bddu32, v); }
  pc = 0x456e34; continue;
 }
 0x456e34 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456e59; continue; }
  pc = 0x456e40; continue;
 }
 0x456e40 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456e53); f_0047a2d9(cpu); if cpu.returned_to != 0x456e53 { return; }
  cpu.push(0x456e58); f_0047232c(cpu); if cpu.returned_to != 0x456e58 { return; }
  { cpu.popad(); }
  pc = 0x456e59; continue;
 }
 0x456e59 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456e6c); f_00456fce(cpu); if cpu.returned_to != 0x456e6c { return; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x456e73); f_00456b9c(cpu); if cpu.returned_to != 0x456e73 { return; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x456e7a); f_0045165b(cpu); if cpu.returned_to != 0x456e7a { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456e8d); f_00456fa5(cpu); if cpu.returned_to != 0x456e8d { return; }
  cpu.push(0x456e92); f_00456a92(cpu); if cpu.returned_to != 0x456e92 { return; }
  cpu.push(0x456e97); f_0045700d(cpu); if cpu.returned_to != 0x456e97 { return; }
  if cpu.cf { pc = 0x456ea3; continue; }
  pc = 0x456e99; continue;
 }
 0x456e99 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  pc = 0x456ea3; continue;
 }
 0x456ea3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456ea4); }
 let mut pc: u32 = 0x456ea4;
 loop { match pc {
 0x456ea4 => {
  cpu.push(0x456ea9); f_0045700d(cpu); if cpu.returned_to != 0x456ea9 { return; }
  if cpu.cf { pc = 0x456fa4; continue; }
  pc = 0x456eaf; continue;
 }
 0x456eaf => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0x0u8); }
  if cpu.zf { pc = 0x456fa4; continue; }
  pc = 0x456ebc; continue;
 }
 0x456ebc => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0x2u8); }
  if cpu.zf { pc = 0x456fa4; continue; }
  pc = 0x456ec9; continue;
 }
 0x456ec9 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fcfu32, v); }
  cpu.push(0x456ed5); f_0044d980(cpu); if cpu.returned_to != 0x456ed5 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x56d30au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e156fu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d309u32, v); }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x3u8); }
  if !cpu.zf { pc = 0x456f0e; continue; }
  pc = 0x456efb; continue;
 }
 0x456efb => {
  cpu.push(0x456f00); f_0044d980(cpu); if cpu.returned_to != 0x456f00 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5824a9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3962u32, v); }
  pc = 0x456f0e; continue;
 }
 0x456f0e => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x456f57; continue; }
  pc = 0x456f17; continue;
 }
 0x456f17 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581bddu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x456f32; continue; }
  pc = 0x456f23; continue;
 }
 0x456f23 => {
  { let v: u32 = 0x581bd9u32; cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581bddu32, v); }
  pc = 0x456f32; continue;
 }
 0x456f32 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x456f57; continue; }
  pc = 0x456f3e; continue;
 }
 0x456f3e => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456f51); f_0047a2d9(cpu); if cpu.returned_to != 0x456f51 { return; }
  cpu.push(0x456f56); f_0047232c(cpu); if cpu.returned_to != 0x456f56 { return; }
  { cpu.popad(); }
  pc = 0x456f57; continue;
 }
 0x456f57 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456f6a); f_00456fce(cpu); if cpu.returned_to != 0x456f6a { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x456f74); f_00456b9c(cpu); if cpu.returned_to != 0x456f74 { return; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x456f7b); f_0045165b(cpu); if cpu.returned_to != 0x456f7b { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456f8e); f_00456fa5(cpu); if cpu.returned_to != 0x456f8e { return; }
  cpu.push(0x456f93); f_00456a92(cpu); if cpu.returned_to != 0x456f93 { return; }
  cpu.push(0x456f98); f_0045700d(cpu); if cpu.returned_to != 0x456f98 { return; }
  if cpu.cf { pc = 0x456fa4; continue; }
  pc = 0x456f9a; continue;
 }
 0x456f9a => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  pc = 0x456fa4; continue;
 }
 0x456fa4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456fa5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456fa5); }
 let mut pc: u32 = 0x456fa5;
 loop { match pc {
 0x456fa5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x456fb8); f_004712e3(cpu); if cpu.returned_to != 0x456fb8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  if !cpu.cf { pc = 0x456fcc; continue; }
  pc = 0x456fc1; continue;
 }
 0x456fc1 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x456fcc; continue;
 }
 0x456fcc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00456fce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x456fce); }
 let mut pc: u32 = 0x456fce;
 loop { match pc {
 0x456fce => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x456fd7); f_00420989(cpu); if cpu.returned_to != 0x456fd7 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  pc = 0x456fe4; continue;
 }
 0x456fe4 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x456feb; continue;
 }
 0x456feb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x456ff7; continue; }
  pc = 0x456ff5; continue;
 }
 0x456ff5 => {
  if !cpu.zf { pc = 0x456fff; continue; }
  pc = 0x456ff7; continue;
 }
 0x456ff7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x13u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x456fff; continue;
 }
 0x456fff => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x456feb; continue; }
  pc = 0x457002; continue;
 }
 0x457002 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x456fe4; continue; }
  pc = 0x45700c; continue;
 }
 0x45700c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045700d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45700d); }
 let mut pc: u32 = 0x45700d;
 loop { match pc {
 0x45700d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x457021; continue; }
  pc = 0x457016; continue;
 }
 0x457016 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x45702d; continue; }
  pc = 0x45701f; continue;
 }
 0x45701f => {
  pc = 0x45702b; continue;
 }
 0x457021 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x45702d; continue; }
  pc = 0x45702b; continue;
 }
 0x45702b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45702d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045702f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45702f); }
 let mut pc: u32 = 0x45702f;
 loop { match pc {
 0x45702f => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if !cpu.zf { pc = 0x457054; continue; }
  pc = 0x45704d; continue;
 }
 0x45704d => {
  { let v: u32 = 0x6a6u32; cpu.ecx = v; }
  pc = 0x457064; continue;
 }
 0x457054 => {
  { let v: u32 = cpu.r8(0x4e156au32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x6a4u32); cpu.ecx = v; }
  pc = 0x457064; continue;
 }
 0x457064 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x457079); f_00486542(cpu); if cpu.returned_to != 0x457079 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4570d6; continue; }
  pc = 0x457082; continue;
 }
 0x457082 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x4570d6; continue; }
  pc = 0x45708c; continue;
 }
 0x45708c => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if !cpu.zf { pc = 0x4570b1; continue; }
  pc = 0x4570aa; continue;
 }
 0x4570aa => {
  { let v: u32 = 0x6a6u32; cpu.ecx = v; }
  pc = 0x4570c1; continue;
 }
 0x4570b1 => {
  { let v: u32 = cpu.r8(0x4e156au32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x6a4u32); cpu.ecx = v; }
  pc = 0x4570c1; continue;
 }
 0x4570c1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x4570d6); f_00486542(cpu); if cpu.returned_to != 0x4570d6 { return; }
  pc = 0x4570d6; continue;
 }
 0x4570d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004570d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4570d7); }
 let mut pc: u32 = 0x4570d7;
 loop { match pc {
 0x4570d7 => {
  { let v: u16 = cpu.r16(0x4d9548u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d954au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x457105; continue; }
  pc = 0x4570fd; continue;
 }
 0x4570fd => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x457105; continue;
 }
 0x457105 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457106(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457106); }
 let mut pc: u32 = 0x457106;
 loop { match pc {
 0x457106 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6760u32, v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x457175; continue; }
  pc = 0x457136; continue;
 }
 0x457136 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x2u8); }
  if cpu.zf { pc = 0x457157; continue; }
  pc = 0x45713f; continue;
 }
 0x45713f => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x3u8); }
  if cpu.zf { pc = 0x457166; continue; }
  pc = 0x457148; continue;
 }
 0x457148 => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x457175; continue;
 }
 0x457157 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x457175; continue;
 }
 0x457166 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x457175; continue;
 }
 0x457175 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x45718e; continue; }
  pc = 0x457186; continue;
 }
 0x457186 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x45718e; continue;
 }
 0x45718e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d675cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045719c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45719c); }
 let mut pc: u32 = 0x45719c;
 loop { match pc {
 0x45719c => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eab34u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d307u32), 0x0u8); }
  if !cpu.zf { pc = 0x4571c0; continue; }
  pc = 0x4571b9; continue;
 }
 0x4571b9 => {
  { let v: u16 = cpu.r16(0x4eab3eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4571c0; continue;
 }
 0x4571c0 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4571c9); f_004572b7(cpu); if cpu.returned_to != 0x4571c9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x4571d0); f_00457320(cpu); if cpu.returned_to != 0x4571d0 { return; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eab34u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if !cpu.zf { pc = 0x4571f4; continue; }
  pc = 0x4571ed; continue;
 }
 0x4571ed => {
  { let v: u16 = cpu.r16(0x4eab3eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4571f4; continue;
 }
 0x4571f4 => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  cpu.push(0x4571fe); f_004572b7(cpu); if cpu.returned_to != 0x4571fe { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4572b6; continue; }
  pc = 0x45720b; continue;
 }
 0x45720b => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4572b6; continue; }
  pc = 0x457219; continue;
 }
 0x457219 => {
  { cpu.sub8(cpu.r8(0x5b3a3du32), 0x3u8); }
  if cpu.cf { pc = 0x4572b6; continue; }
  pc = 0x457226; continue;
 }
 0x457226 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eab34u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if !cpu.zf { pc = 0x45724a; continue; }
  pc = 0x457243; continue;
 }
 0x457243 => {
  { let v: u16 = cpu.r16(0x4eab3eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45724a; continue;
 }
 0x45724a => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x457256); f_004572b7(cpu); if cpu.returned_to != 0x457256 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4572b6; continue; }
  pc = 0x457262; continue;
 }
 0x457262 => {
  { cpu.sub8(cpu.r8(0x5b3a3du32), 0x4u8); }
  if cpu.cf { pc = 0x4572b6; continue; }
  pc = 0x45726b; continue;
 }
 0x45726b => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x457292; continue; }
  pc = 0x457282; continue;
 }
 0x457282 => {
  { let v: u16 = cpu.r16(0x4eab34u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d309u32), 0x0u8); }
  if !cpu.zf { pc = 0x457299; continue; }
  pc = 0x457292; continue;
 }
 0x457292 => {
  { let v: u16 = cpu.r16(0x4eab3eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x457299; continue;
 }
 0x457299 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4572a5); f_004572b7(cpu); if cpu.returned_to != 0x4572a5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x4572b6; continue; }
  pc = 0x4572b1; continue;
 }
 0x4572b1 => {
  cpu.push(0x4572b6); f_00457352(cpu); if cpu.returned_to != 0x4572b6 { return; }
  pc = 0x4572b6; continue;
 }
 0x4572b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004572b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4572b7); }
 let mut pc: u32 = 0x4572b7;
 loop { match pc {
 0x4572b7 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4572c0); f_00457303(cpu); if cpu.returned_to != 0x4572c0 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x4572cb); f_00457303(cpu); if cpu.returned_to != 0x4572cb { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x4572dd); f_00457303(cpu); if cpu.returned_to != 0x4572dd { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x4572e8); f_00457303(cpu); if cpu.returned_to != 0x4572e8 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x70u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x4572f7); f_00457303(cpu); if cpu.returned_to != 0x4572f7 { return; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x457302); f_00457303(cpu); if cpu.returned_to != 0x457302 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457303(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457303); }
 let mut pc: u32 = 0x457303;
 loop { match pc {
 0x457303 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x45731b); cpu.call(t); if cpu.returned_to != 0x45731b { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457320); }
 let mut pc: u32 = 0x457320;
 loop { match pc {
 0x457320 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x90u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67ccu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x457351; continue; }
  pc = 0x457334; continue;
 }
 0x457334 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4eab3cu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x457351); cpu.call(t); if cpu.returned_to != 0x457351 { return; }
  pc = 0x457351; continue;
 }
 0x457351 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457352); }
 let mut pc: u32 = 0x457352;
 loop { match pc {
 0x457352 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x90u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67ccu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x457383; continue; }
  pc = 0x457366; continue;
 }
 0x457366 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4eab3cu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x457383); cpu.call(t); if cpu.returned_to != 0x457383 { return; }
  pc = 0x457383; continue;
 }
 0x457383 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457384(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457384); }
 let mut pc: u32 = 0x457384;
 loop { match pc {
 0x457384 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x4574bc; continue; }
  pc = 0x457391; continue;
 }
 0x457391 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6a08u32)); cpu.w32(0x4d6a08u32, v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x2u32); }
  if !cpu.zf { pc = 0x4573e2; continue; }
  pc = 0x4573a0; continue;
 }
 0x4573a0 => {
  { cpu.pushad(); }
  cpu.push(0x4573a6); f_004010e0(cpu); if cpu.returned_to != 0x4573a6 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x4e156fu32)); }
  if !cpu.zf { pc = 0x4573c5; continue; }
  pc = 0x4573b9; continue;
 }
 0x4573b9 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4573c3); f_00415b6c(cpu); if cpu.returned_to != 0x4573c3 { return; }
  pc = 0x4573e2; continue;
 }
 0x4573c5 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4573d8; continue; }
  pc = 0x4573ce; continue;
 }
 0x4573ce => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4573e2; continue; }
  pc = 0x4573d8; continue;
 }
 0x4573d8 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4573e2); f_00415ab4(cpu); if cpu.returned_to != 0x4573e2 { return; }
  pc = 0x4573e2; continue;
 }
 0x4573e2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x1u8); }
  if cpu.zf { pc = 0x457411; continue; }
  pc = 0x4573f0; continue;
 }
 0x4573f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x2u8); }
  if cpu.zf { pc = 0x457411; continue; }
  pc = 0x4573fe; continue;
 }
 0x4573fe => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x3u8); }
  if cpu.zf { pc = 0x457411; continue; }
  pc = 0x45740c; continue;
 }
 0x45740c => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x457411; continue;
 }
 0x457411 => {
  { let v: u32 = cpu.eax; cpu.w32(0x59f446u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x45742b; continue; }
  pc = 0x45741f; continue;
 }
 0x45741f => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x457442; continue; }
  pc = 0x457424; continue;
 }
 0x457424 => {
  cpu.push(0x457429); f_0042ec8e(cpu); if cpu.returned_to != 0x457429 { return; }
  pc = 0x457442; continue;
 }
 0x45742b => {
  cpu.push(0x457430); f_0042ec8e(cpu); if cpu.returned_to != 0x457430 { return; }
  { let v: u32 = cpu.r32(0x59f446u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56d30au32); cpu.ebx = v; }
  cpu.push(0x457440); f_0042eaf0(cpu); if cpu.returned_to != 0x457440 { return; }
  pc = 0x457442; continue;
 }
 0x457442 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x2u8); }
  if cpu.zf { pc = 0x4574be; continue; }
  pc = 0x457461; continue;
 }
 0x457461 => {
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x3u8); }
  if cpu.zf { pc = 0x4574d2; continue; }
  pc = 0x45746a; continue;
 }
 0x45746a => {
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x4u8); }
  if cpu.zf { pc = 0x4574e1; continue; }
  pc = 0x457473; continue;
 }
 0x457473 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x2u32); }
  if cpu.zf { pc = 0x45756e; continue; }
  pc = 0x457480; continue;
 }
 0x457480 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x3u32); }
  if cpu.zf { pc = 0x457820; continue; }
  pc = 0x45748d; continue;
 }
 0x45748d => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0xfu32); }
  if cpu.zf { pc = 0x4575b6; continue; }
  pc = 0x45749a; continue;
 }
 0x45749a => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x10u32); }
  if cpu.zf { pc = 0x4576ac; continue; }
  pc = 0x4574a7; continue;
 }
 0x4574a7 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x11u32); }
  if cpu.zf { pc = 0x45751b; continue; }
  pc = 0x4574b0; continue;
 }
 0x4574b0 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0xaau32); }
  if cpu.zf { pc = 0x4574f0; continue; }
  pc = 0x4574bc; continue;
 }
 0x4574bc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4574be => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x457473; continue;
 }
 0x4574d2 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x457473; continue;
 }
 0x4574e1 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x457473; continue;
 }
 0x4574f0 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x45750f; continue; }
  pc = 0x4574f9; continue;
 }
 0x4574f9 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x45750f; continue; }
  pc = 0x457503; continue;
 }
 0x457503 => {
  cpu.push(0x457508); f_004578cd(cpu); if cpu.returned_to != 0x457508 { return; }
  cpu.push(0x45750d); f_0041b2d5(cpu); if cpu.returned_to != 0x45750d { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45750f => {
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45751b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e156fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x457536; continue; }
  pc = 0x45752c; continue;
 }
 0x45752c => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x45754a; continue; }
  pc = 0x457536; continue;
 }
 0x457536 => {
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c635u32, v); }
  pc = 0x45754a; continue;
 }
 0x45754a => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d306u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56d302u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e156eu32, v); }
  { let v: u8 = 0x14u8; cpu.w8(0x59d9dcu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x59d9ddu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45756e => {
  cpu.push(0x457573); f_0041879e(cpu); if cpu.returned_to != 0x457573 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e156fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1559u32, v); }
  cpu.push(0x457591); f_00457106(cpu); if cpu.returned_to != 0x457591 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1559u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4575b4; continue; }
  pc = 0x4575a0; continue;
 }
 0x4575a0 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x4575b4; continue; }
  pc = 0x4575aa; continue;
 }
 0x4575aa => {
  { let v: u32 = 0x4ac6d0u32; cpu.esi = v; }
  cpu.push(0x4575b4); f_0041b447(cpu); if cpu.returned_to != 0x4575b4 { return; }
  pc = 0x4575b4; continue;
 }
 0x4575b4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4575b6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  cpu.push(0x4575c2); f_004679cb(cpu); if cpu.returned_to != 0x4575c2 { return; }
  if cpu.cf { pc = 0x4575fb; continue; }
  pc = 0x4575c4; continue;
 }
 0x4575c4 => {
  { cpu.sub8(cpu.r8(0x4eab2au32), 0x0u8); }
  if !cpu.zf { pc = 0x4575fb; continue; }
  pc = 0x4575cd; continue;
 }
 0x4575cd => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x4575f3; continue; }
  pc = 0x4575e3; continue;
 }
 0x4575e3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4575ea); f_00401130(cpu); if cpu.returned_to != 0x4575ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x4575f3; continue;
 }
 0x4575f3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2au32, v); }
  pc = 0x4575fb; continue;
 }
 0x4575fb => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x1dau16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x457643; continue; }
  pc = 0x45762c; continue;
 }
 0x45762c => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x457648; continue; }
  pc = 0x457635; continue;
 }
 0x457635 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x45764d; continue; }
  pc = 0x45763e; continue;
 }
 0x45763e => {
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  pc = 0x457650; continue;
 }
 0x457643 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x457650; continue;
 }
 0x457648 => {
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  pc = 0x457650; continue;
 }
 0x45764d => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x457650; continue;
 }
 0x457650 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x457653; continue;
 }
 0x457653 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x45765d); f_0041c743(cpu); if cpu.returned_to != 0x45765d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x457664); f_0041c743(cpu); if cpu.returned_to != 0x457664 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x3f1fu32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x3fu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x4576a5); f_00483b00(cpu); if cpu.returned_to != 0x4576a5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x457653; continue; }
  pc = 0x4576aa; continue;
 }
 0x4576aa => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4576ac => {
  cpu.push(0x4576b1); f_004679cb(cpu); if cpu.returned_to != 0x4576b1 { return; }
  if cpu.cf { pc = 0x4576ea; continue; }
  pc = 0x4576b3; continue;
 }
 0x4576b3 => {
  { cpu.sub8(cpu.r8(0x4eab2au32), 0x0u8); }
  if !cpu.zf { pc = 0x4576ea; continue; }
  pc = 0x4576bc; continue;
 }
 0x4576bc => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x4576e2; continue; }
  pc = 0x4576d2; continue;
 }
 0x4576d2 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4576d9); f_00401130(cpu); if cpu.returned_to != 0x4576d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x4576e2; continue;
 }
 0x4576e2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2au32, v); }
  pc = 0x4576ea; continue;
 }
 0x4576ea => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x6a7u16; cpu.w16(0x4d6960u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x457732; continue; }
  pc = 0x45771b; continue;
 }
 0x45771b => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x457737; continue; }
  pc = 0x457724; continue;
 }
 0x457724 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x45773c; continue; }
  pc = 0x45772d; continue;
 }
 0x45772d => {
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  pc = 0x45773f; continue;
 }
 0x457732 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x45773f; continue;
 }
 0x457737 => {
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  pc = 0x45773f; continue;
 }
 0x45773c => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x45773f; continue;
 }
 0x45773f => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x457742; continue;
 }
 0x457742 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x45774b); f_0041c743(cpu); if cpu.returned_to != 0x45774b { return; }
  { let v: u32 = cpu.eax & 0x301u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x56d3edu32.wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  cpu.push(0x457768); f_0041c743(cpu); if cpu.returned_to != 0x457768 { return; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x457772); f_0041c743(cpu); if cpu.returned_to != 0x457772 { return; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x457780); f_004577ff(cpu); if cpu.returned_to != 0x457780 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x457742; continue; }
  pc = 0x457785; continue;
 }
 0x457785 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = 0x4dbu16; cpu.w16(0x4d6960u32, v); }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x1u8); }
  if cpu.zf { pc = 0x4577e0; continue; }
  pc = 0x4577b3; continue;
 }
 0x4577b3 => {
  { let v: u16 = 0x4dfu16; cpu.w16(0x4d6960u32, v); }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x2u8); }
  if cpu.zf { pc = 0x4577e0; continue; }
  pc = 0x4577c5; continue;
 }
 0x4577c5 => {
  { let v: u16 = 0x4e7u16; cpu.w16(0x4d6960u32, v); }
  { cpu.sub8(cpu.r8(0x4e156fu32), 0x3u8); }
  if cpu.zf { pc = 0x4577e0; continue; }
  pc = 0x4577d7; continue;
 }
 0x4577d7 => {
  { let v: u16 = 0x4ebu16; cpu.w16(0x4d6960u32, v); }
  pc = 0x4577e0; continue;
 }
 0x4577e0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x53u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x3bu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  cpu.push(0x4577fd); f_00482f3b(cpu); if cpu.returned_to != 0x4577fd { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x457820 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xa0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0xa0u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x59f446u32); cpu.esi = v; }
  cpu.push(0x45783b); f_0045c5fe(cpu); if cpu.returned_to != 0x45783b { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004577ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4577ff); }
 let mut pc: u32 = 0x4577ff;
 loop { match pc {
 0x4577ff => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  cpu.push(0x45781e); f_00482f3b(cpu); if cpu.returned_to != 0x45781e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045783d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45783d); }
 let mut pc: u32 = 0x45783d;
 loop { match pc {
 0x45783d => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d315u32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x45784e; continue;
 }
 0x45784e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x4578b8; continue; }
  pc = 0x457857; continue;
 }
 0x457857 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x4578b8; continue; }
  pc = 0x45785e; continue;
 }
 0x45785e => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x457872; continue; }
  pc = 0x457865; continue;
 }
 0x457865 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x45786d); f_004586f4(cpu); if cpu.returned_to != 0x45786d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4578b8; continue;
 }
 0x457872 => {
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x4578b8; continue; }
  pc = 0x45787b; continue;
 }
 0x45787b => {
  { let v: u8 = 0xf8u8; cpu.w8(0x4e1f4cu32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x4578a9); f_00413337(cpu); if cpu.returned_to != 0x4578a9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { cpu.popad(); }
  pc = 0x4578b8; continue;
 }
 0x4578b8 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x45784e; continue; }
  pc = 0x4578c5; continue;
 }
 0x4578c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d315u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004578cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4578cd); }
 let mut pc: u32 = 0x4578cd;
 loop { match pc {
 0x4578cd => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e156fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004578df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4578df); }
 let mut pc: u32 = 0x4578df;
 loop { match pc {
 0x4578df => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d67ceu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004578e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4578e9); }
 let mut pc: u32 = 0x4578e9;
 loop { match pc {
 0x4578e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045796a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45796a); }
 let mut pc: u32 = 0x45796a;
 loop { match pc {
 0x45796a => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x45797d; continue; }
  pc = 0x457973; continue;
 }
 0x457973 => {
  { let v: u32 = 0x4d67ccu32; cpu.esi = v; }
  cpu.push(0x45797d); f_0045797e(cpu); if cpu.returned_to != 0x45797d { return; }
  pc = 0x45797d; continue;
 }
 0x45797d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045797e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45797e); }
 let mut pc: u32 = 0x45797e;
 loop { match pc {
 0x45797e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x457995; continue; }
  pc = 0x457987; continue;
 }
 0x457987 => {
  { let v: u16 = cpu.add16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x457999; continue; }
  pc = 0x45798c; continue;
 }
 0x45798c => {
  if cpu.sf { pc = 0x457996; continue; }
  pc = 0x45798e; continue;
 }
 0x45798e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x457999; continue; }
  pc = 0x457993; continue;
 }
 0x457993 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x457996; continue; }
  pc = 0x457995; continue;
 }
 0x457995 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x457996 => {
  { let v: u16 = cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x457999; continue;
 }
 0x457999 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004579a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4579a4); }
 let mut pc: u32 = 0x4579a4;
 loop { match pc {
 0x4579a4 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4579ae; continue;
 }
 0x4579ae => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4579ce; continue; }
  pc = 0x4579b5; continue;
 }
 0x4579b5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x4579ce; continue; }
  pc = 0x4579bd; continue;
 }
 0x4579bd => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x4579ce; continue; }
  pc = 0x4579c6; continue;
 }
 0x4579c6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4579ce; continue;
 }
 0x4579ce => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4579ae; continue; }
  pc = 0x4579d7; continue;
 }
 0x4579d7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004579d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4579d8); }
 let mut pc: u32 = 0x4579d8;
 loop { match pc {
 0x4579d8 => {
  cpu.push(0x4579dd); f_004579a4(cpu); if cpu.returned_to != 0x4579dd { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1cdu32, v); }
  cpu.push(0x4579e9); f_0045bc08(cpu); if cpu.returned_to != 0x4579e9 { return; }
  cpu.push(0x4579ee); f_00448d7e(cpu); if cpu.returned_to != 0x4579ee { return; }
  cpu.push(0x4579f3); f_0047e2e9(cpu); if cpu.returned_to != 0x4579f3 { return; }
  cpu.push(0x4579f8); f_0046a3a3(cpu); if cpu.returned_to != 0x4579f8 { return; }
  cpu.push(0x4579fd); f_0046a31c(cpu); if cpu.returned_to != 0x4579fd { return; }
  cpu.push(0x457a02); f_0044668e(cpu); if cpu.returned_to != 0x457a02 { return; }
  cpu.push(0x457a07); f_004800e7(cpu); if cpu.returned_to != 0x457a07 { return; }
  cpu.push(0x457a0c); f_0047a26f(cpu); if cpu.returned_to != 0x457a0c { return; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bdu32, v); }
  pc = 0x457a1d; continue;
 }
 0x457a1d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x457a47; continue; }
  pc = 0x457a24; continue;
 }
 0x457a24 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x457a31); f_00457a6c(cpu); if cpu.returned_to != 0x457a31 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { pc = 0x457a37; continue; }
  pc = 0x457a36; continue;
 }
 0x457a36 => {
  {  }
  pc = 0x457a37; continue;
 }
 0x457a37 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0xffu16); }
  if (cpu.sf != cpu.of) { pc = 0x457a47; continue; }
  pc = 0x457a40; continue;
 }
 0x457a40 => {
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x457a47; continue;
 }
 0x457a47 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e15bdu32)); cpu.w8(0x4e15bdu32, v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x457a1d; continue; }
  pc = 0x457a5d; continue;
 }
 0x457a5d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cf455u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fc6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457a6c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x457a6c); } r_00457a6c(cpu, 0x457a6c); }
pub fn f_0048658b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48658b); } r_00457a6c(cpu, 0x48658b); }
pub fn f_00486be8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x486be8); } r_00457a6c(cpu, 0x486be8); }
pub fn f_00487628(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x487628); } r_00457a6c(cpu, 0x487628); }
fn r_00457a6c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x457a6c => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x3du8); }
  if !cpu.zf { pc = 0x457acc; continue; }
  pc = 0x457a89; continue;
 }
 0x457a89 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { pc = 0x457aca; continue; }
  pc = 0x457aa1; continue;
 }
 0x457aa1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xe7u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x457ac4); f_0045c2d0(cpu); if cpu.returned_to != 0x457ac4 { return; }
  cpu.push(0x457ac9); f_0045c294(cpu); if cpu.returned_to != 0x457ac9 { return; }
  { cpu.popad(); }
  pc = 0x457aca; continue;
 }
 0x457aca => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x457acc; continue;
 }
 0x457acc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x457ad8; continue; }
  pc = 0x457ad3; continue;
 }
 0x457ad3 => {
  cpu.push(0x457ad8); f_0045c73e(cpu); if cpu.returned_to != 0x457ad8 { return; }
  pc = 0x457ad8; continue;
 }
 0x457ad8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5d6758u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x32u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x457af2; continue; }
  pc = 0x457aeb; continue;
 }
 0x457aeb => {
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x457af2; continue;
 }
 0x457af2 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x457b22; continue; }
  pc = 0x457afc; continue;
 }
 0x457afc => {
  { let v: u8 = cpu.dec8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp))); cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1cdu32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xebu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x52bbf9u32.wrapping_add(cpu.ebx)) & 0xdfu8; cpu.w8(0x52bbf9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x457b22; continue;
 }
 0x457b22 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b29; continue;
 }
 0x457b29 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.sf { pc = 0x457b94; continue; }
  pc = 0x457b40; continue;
 }
 0x457b40 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x6cu32.wrapping_add(cpu.ebp))); cpu.w8(0x6cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x457b4b; continue; }
  pc = 0x457b46; continue;
 }
 0x457b46 => {
  { let v: u8 = 0xffu8; cpu.w8(0x6cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x457b4b; continue;
 }
 0x457b4b => {
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x457b5b; continue; }
  pc = 0x457b51; continue;
 }
 0x457b51 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3fu32.wrapping_add(cpu.eax))); cpu.w8(0x3fu32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x457b5b; continue; }
  pc = 0x457b56; continue;
 }
 0x457b56 => {
  cpu.push(0x457b5b); f_00480193(cpu); if cpu.returned_to != 0x457b5b { return; }
  pc = 0x457b5b; continue;
 }
 0x457b5b => {
  { cpu.sub8(cpu.r8(0x40u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x457b6f; continue; }
  pc = 0x457b61; continue;
 }
 0x457b61 => {
  if cpu.sf { pc = 0x457b6f; continue; }
  pc = 0x457b63; continue;
 }
 0x457b63 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x40u32.wrapping_add(cpu.eax))); cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x457b6f; continue; }
  pc = 0x457b68; continue;
 }
 0x457b68 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x457b6f); f_004167f5(cpu); if cpu.returned_to != 0x457b6f { return; }
  pc = 0x457b6f; continue;
 }
 0x457b6f => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b75; continue;
 }
 0x457b75 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b7c; continue;
 }
 0x457b7c => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b82; continue;
 }
 0x457b82 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b89; continue;
 }
 0x457b89 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x13u8); }
  if cpu.zf { pc = 0x457b94; continue; }
  pc = 0x457b90; continue;
 }
 0x457b90 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.eax), v); }
  pc = 0x457b94; continue;
 }
 0x457b94 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x457b9d; continue; }
  pc = 0x457b9b; continue;
 }
 0x457b9b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x457b9d => {
  { cpu.sub8(cpu.r8(0x4e1fc3u32), 0x0u8); }
  if cpu.zf { pc = 0x457bab; continue; }
  pc = 0x457ba6; continue;
 }
 0x457ba6 => {
  cpu.push(0x457bab); f_00457bf1(cpu); if cpu.returned_to != 0x457bab { return; }
  pc = 0x457bab; continue;
 }
 0x457bab => {
  cpu.push(0x457bb0); f_0048afc8(cpu); if cpu.returned_to != 0x457bb0 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x78u32.wrapping_add(cpu.ebp))); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x457bbf; continue; }
  pc = 0x457bbb; continue;
 }
 0x457bbb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x78u32.wrapping_add(cpu.ebp))); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x457bbf; continue;
 }
 0x457bbf => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  cpu.push(0x457bce); f_0044a9a4(cpu); if cpu.returned_to != 0x457bce { return; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  let t = cpu.r32(0x5b9b70u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x457bdb); cpu.call(t); if cpu.returned_to != 0x457bdb { return; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x457bec; continue; }
  pc = 0x457be2; continue;
 }
 0x457be2 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x457bec); f_0044bc7b(cpu); if cpu.returned_to != 0x457bec { return; }
  pc = 0x457bec; continue;
 }
 0x457bec => {
  pc = 0x48658c; continue;
 }
 0x48658b => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x48658c; continue;
 }
 0x48658c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d6a34u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6936u32, v); }
  cpu.push(0x4865b5); f_00486fa1(cpu); if cpu.returned_to != 0x4865b5 { return; }
  if cpu.cf { pc = 0x486a99; continue; }
  pc = 0x4865bb; continue;
 }
 0x4865bb => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x486a99; continue; }
  pc = 0x4865d1; continue;
 }
 0x4865d1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x486a99; continue; }
  pc = 0x4865e2; continue;
 }
 0x4865e2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x486a99; continue; }
  pc = 0x4865fc; continue;
 }
 0x4865fc => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x486a99; continue; }
  pc = 0x48660d; continue;
 }
 0x48660d => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x486a99; continue; }
  pc = 0x48661c; continue;
 }
 0x48661c => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x486645; continue; }
  pc = 0x486625; continue;
 }
 0x486625 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x486645; continue; }
  pc = 0x486635; continue;
 }
 0x486635 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x486651; continue; }
  pc = 0x486645; continue;
 }
 0x486645 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x486a99; continue;
 }
 0x486651 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x486663; continue;
 }
 0x486663 => {
  { cpu.sub8((cpu.edx as u8), 0x1fu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x48666c; continue;
 }
 0x48666c => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x486675; continue;
 }
 0x486675 => {
  { cpu.sub8((cpu.edx as u8), 0x3eu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x48667e; continue;
 }
 0x48667e => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x486687; continue;
 }
 0x486687 => {
  { cpu.sub8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x486690; continue;
 }
 0x486690 => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x486699; continue;
 }
 0x486699 => {
  { cpu.sub8((cpu.edx as u8), 0x42u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866a2; continue;
 }
 0x4866a2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x4866df; continue; }
  pc = 0x4866a9; continue;
 }
 0x4866a9 => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866b2; continue;
 }
 0x4866b2 => {
  { cpu.sub8((cpu.edx as u8), 0x32u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866bb; continue;
 }
 0x4866bb => {
  { cpu.sub8((cpu.edx as u8), 0x2fu8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866c4; continue;
 }
 0x4866c4 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866cd; continue;
 }
 0x4866cd => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866d6; continue;
 }
 0x4866d6 => {
  { cpu.sub8((cpu.edx as u8), 0x38u8); }
  if cpu.zf { pc = 0x486a9b; continue; }
  pc = 0x4866df; continue;
 }
 0x4866df => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x44u8); }
  if cpu.zf { pc = 0x4867e8; continue; }
  pc = 0x4866ea; continue;
 }
 0x4866ea => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a35u32, v); }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a37u32, v); }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x9u8); }
  if cpu.zf { pc = 0x487629; continue; }
  pc = 0x486716; continue;
 }
 0x486716 => {
  { cpu.sub8((cpu.ebx as u8), 0x43u8); }
  if !cpu.zf { pc = 0x4868db; continue; }
  pc = 0x48671f; continue;
 }
 0x48671f => {
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x48674c; continue; }
  pc = 0x486747; continue;
 }
 0x486747 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48674c; continue;
 }
 0x48674c => {
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x486a99; continue; }
  pc = 0x486765; continue;
 }
 0x486765 => {
  { let v: u16 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6a35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6a37u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48677f; continue;
 }
 0x48677f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x178au16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x486797; continue; }
  pc = 0x486791; continue;
 }
 0x486791 => {
  { let v: u16 = 0x178bu16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x486797; continue;
 }
 0x486797 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48677f; continue; }
  pc = 0x4867ae; continue;
 }
 0x4867ae => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  { cpu.and8(cpu.r8(0x7cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x486a99; continue; }
  pc = 0x4867bf; continue;
 }
 0x4867bf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x178cu16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x486a99; continue;
 }
 0x4867e8 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a35u32, v); }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a37u32, v); }
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x4u8); }
  if cpu.zf { pc = 0x486817; continue; }
  pc = 0x48680c; continue;
 }
 0x48680c => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x486867; continue; }
  pc = 0x486815; continue;
 }
 0x486815 => {
  pc = 0x486820; continue;
 }
 0x486817 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x486867; continue; }
  pc = 0x486820; continue;
 }
 0x486820 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x5d6a35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x5d6a37u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x486858; continue; }
  pc = 0x486853; continue;
 }
 0x486853 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x486858; continue;
 }
 0x486858 => {
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  { cpu.popad(); }
  pc = 0x486867; continue;
 }
 0x486867 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x5d6a35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x5d6a37u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x7u16); }
  if (cpu.sf == cpu.of) { pc = 0x486890; continue; }
  pc = 0x48688a; continue;
 }
 0x48688a => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486894; continue;
 }
 0x486890 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486894; continue;
 }
 0x486894 => {
  { cpu.stos(2, false); }
  { let v: u32 = 0x44u32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x4u8); }
  if cpu.zf { pc = 0x4868bb; continue; }
  pc = 0x4868b5; continue;
 }
 0x4868b5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4868bb; continue;
 }
 0x4868bb => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x486a99; continue;
 }
 0x4868db => {
  { cpu.sub8((cpu.ebx as u8), 0x12u8); }
  if cpu.zf { pc = 0x486a99; continue; }
  pc = 0x4868e4; continue;
 }
 0x4868e4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487629; continue; }
  pc = 0x4868ef; continue;
 }
 0x4868ef => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x4868fa; continue;
 }
 0x4868fa => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x32u8); }
  if !cpu.zf { pc = 0x48696e; continue; }
  pc = 0x486901; continue;
 }
 0x486901 => {
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x486911; continue;
 }
 0x486911 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d69d4u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x486942; continue;
 }
 0x486942 => {
  { let v: u16 = cpu.r16(0x81u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x486959; continue; }
  pc = 0x486950; continue;
 }
 0x486950 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x97fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486a7a; continue;
 }
 0x486959 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x977u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x486a7a; continue;
 }
 0x48696e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x33u8); }
  if !cpu.zf { pc = 0x4869d2; continue; }
  pc = 0x486975; continue;
 }
 0x486975 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d69f4u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x4869a6; continue;
 }
 0x4869a6 => {
  { let v: u16 = cpu.r16(0x81u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x4869bd; continue; }
  pc = 0x4869b4; continue;
 }
 0x4869b4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x97bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486a7a; continue;
 }
 0x4869bd => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x977u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x486a7a; continue;
 }
 0x4869d2 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x34u8); }
  if !cpu.zf { pc = 0x486a2c; continue; }
  pc = 0x4869d9; continue;
 }
 0x4869d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6a14u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x486a06; continue;
 }
 0x486a06 => {
  { let v: u16 = cpu.r16(0x81u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x486a1a; continue; }
  pc = 0x486a14; continue;
 }
 0x486a14 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x983u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486a7a; continue;
 }
 0x486a1a => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x977u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x486a7a; continue;
 }
 0x486a2c => {
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x486a5e; continue; }
  pc = 0x486a38; continue;
 }
 0x486a38 => {
  { let v: u16 = cpu.r16(0x81u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x486a4c; continue; }
  pc = 0x486a46; continue;
 }
 0x486a46 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x977u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486a7a; continue;
 }
 0x486a4c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x977u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)).wrapping_add(0xcu16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x486a7a; continue;
 }
 0x486a5e => {
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486a7a; continue;
 }
 0x486a7a => {
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x486a8b; continue; }
  pc = 0x486a86; continue;
 }
 0x486a86 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x486a8b; continue;
 }
 0x486a8b => {
  { let v: u8 = cpu.r8(0x73u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x486a99; continue;
 }
 0x486a99 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486a9b => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4d8236u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a35u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d6a37u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x4d6bd8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x486ade; continue;
 }
 0x486ade => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x486aff; continue; }
  pc = 0x486af8; continue;
 }
 0x486af8 => {
  cpu.push(0x486afd); f_004713ce(cpu); if cpu.returned_to != 0x486afd { return; }
  if cpu.cf { pc = 0x486b04; continue; }
  pc = 0x486aff; continue;
 }
 0x486aff => {
  cpu.push(0x486b04); f_004712e3(cpu); if cpu.returned_to != 0x486b04 { return; }
  pc = 0x486b04; continue;
 }
 0x486b04 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x486b46; continue; }
  pc = 0x486b0b; continue;
 }
 0x486b0b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x486b21; continue; }
  pc = 0x486b16; continue;
 }
 0x486b16 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x486b21; continue; }
  pc = 0x486b1d; continue;
 }
 0x486b1d => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x486b21; continue;
 }
 0x486b21 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x486ade; continue; }
  pc = 0x486b2a; continue;
 }
 0x486b2a => {
  cpu.push(0x486b2f); f_004874c8(cpu); if cpu.returned_to != 0x486b2f { return; }
  if cpu.cf { pc = 0x486c26; continue; }
  pc = 0x486b35; continue;
 }
 0x486b35 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x486b40; continue;
 }
 0x486b40 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = 0x4866df; continue;
 }
 0x486b46 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ecx = v; }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ecx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x486bca; continue; }
  pc = 0x486b69; continue;
 }
 0x486b69 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6a04u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x486ba1; continue; }
  pc = 0x486b75; continue;
 }
 0x486b75 => {
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4b8b8au32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  pc = 0x486c26; continue;
 }
 0x486ba1 => {
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4b8b8cu32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  pc = 0x486c26; continue;
 }
 0x486bca => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6a04u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x486bff; continue; }
  pc = 0x486bd6; continue;
 }
 0x486bd6 => {
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4b8b86u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  pc = 0x486c26; continue;
 }
 0x486be8 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4b8b86u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  pc = 0x486c26; continue;
 }
 0x486bff => {
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4b8b88u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  pc = 0x486c26; continue;
 }
 0x486c26 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if cpu.zf { cpu.poll(); pc = 0x486b40; continue; }
  pc = 0x486c33; continue;
 }
 0x486c33 => {
  { cpu.sub8((cpu.ebx as u8), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x486b40; continue; }
  pc = 0x486c3c; continue;
 }
 0x486c3c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x486b40; continue; }
  pc = 0x486c47; continue;
 }
 0x486c47 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x486c76; continue; }
  pc = 0x486c71; continue;
 }
 0x486c71 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x486c76; continue;
 }
 0x486c76 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487628 => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x487629; continue;
 }
 0x487629 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x48798b; continue; }
  pc = 0x487634; continue;
 }
 0x487634 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x487c45; continue; }
  pc = 0x48763f; continue;
 }
 0x48763f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x4882eb; continue; }
  pc = 0x48764a; continue;
 }
 0x48764a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x487822; continue; }
  pc = 0x487655; continue;
 }
 0x487655 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x487aee; continue; }
  pc = 0x487660; continue;
 }
 0x487660 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x487e7b; continue; }
  pc = 0x48766b; continue;
 }
 0x48766b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x487c44; continue; }
  pc = 0x487676; continue;
 }
 0x487676 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x5d67e4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1e9u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48770d; continue; }
  pc = 0x487708; continue;
 }
 0x487708 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487712; continue; }
  pc = 0x48770d; continue;
 }
 0x48770d => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487712; continue;
 }
 0x487712 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x487724; continue; }
  pc = 0x487720; continue;
 }
 0x487720 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x487724; continue;
 }
 0x487724 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487740); f_004880f4(cpu); if cpu.returned_to != 0x487740 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.push(0x487748); f_004882df(cpu); if cpu.returned_to != 0x487748 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487756; continue; }
  pc = 0x487751; continue;
 }
 0x487751 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48775b; continue; }
  pc = 0x487756; continue;
 }
 0x487756 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48775b; continue;
 }
 0x48775b => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d67a3u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x202u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4877b0); f_004880f4(cpu); if cpu.returned_to != 0x4877b0 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4877f5); f_004880f4(cpu); if cpu.returned_to != 0x4877f5 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487800); f_00488118(cpu); if cpu.returned_to != 0x487800 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x487806); f_004882e5(cpu); if cpu.returned_to != 0x487806 { return; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48781a; continue; }
  pc = 0x48780d; continue;
 }
 0x48780d => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48781a; continue;
 }
 0x48781a => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487822 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x5d67f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14ccu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4878bd; continue; }
  pc = 0x4878b8; continue;
 }
 0x4878b8 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4878c2; continue; }
  pc = 0x4878bd; continue;
 }
 0x4878bd => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x4878c2; continue;
 }
 0x4878c2 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x4878d0; continue; }
  pc = 0x4878d0; continue;
 }
 0x4878d0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4878ec); f_004880f4(cpu); if cpu.returned_to != 0x4878ec { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.push(0x4878f4); f_004882df(cpu); if cpu.returned_to != 0x4878f4 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487902; continue; }
  pc = 0x4878fd; continue;
 }
 0x4878fd => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487907; continue; }
  pc = 0x487902; continue;
 }
 0x487902 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487907; continue;
 }
 0x487907 => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d67b3u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14e4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x48795c); f_004880f4(cpu); if cpu.returned_to != 0x48795c { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487969); f_00488118(cpu); if cpu.returned_to != 0x487969 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x48796f); f_004882e5(cpu); if cpu.returned_to != 0x48796f { return; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x487983; continue; }
  pc = 0x487976; continue;
 }
 0x487976 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487983; continue;
 }
 0x487983 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48798b => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6804u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x166au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487a26; continue; }
  pc = 0x487a21; continue;
 }
 0x487a21 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487a2b; continue; }
  pc = 0x487a26; continue;
 }
 0x487a26 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487a2b; continue;
 }
 0x487a2b => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x487a3d; continue; }
  pc = 0x487a39; continue;
 }
 0x487a39 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x487a3d; continue;
 }
 0x487a3d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487a59); f_004880f4(cpu); if cpu.returned_to != 0x487a59 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487a6a; continue; }
  pc = 0x487a65; continue;
 }
 0x487a65 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487a6f; continue; }
  pc = 0x487a6a; continue;
 }
 0x487a6a => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487a6f; continue;
 }
 0x487a6f => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d67c3u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1676u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487ac4); f_004880f4(cpu); if cpu.returned_to != 0x487ac4 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487ad1); f_00488118(cpu); if cpu.returned_to != 0x487ad1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x487ae6; continue; }
  pc = 0x487ad9; continue;
 }
 0x487ad9 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487ae6; continue;
 }
 0x487ae6 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487aee => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6814u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x162au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487b82; continue; }
  pc = 0x487b7d; continue;
 }
 0x487b7d => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487b87; continue; }
  pc = 0x487b82; continue;
 }
 0x487b82 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487b87; continue;
 }
 0x487b87 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x487b95; continue; }
  pc = 0x487b95; continue;
 }
 0x487b95 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487bb1); f_004880f4(cpu); if cpu.returned_to != 0x487bb1 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.push(0x487bb9); f_004882df(cpu); if cpu.returned_to != 0x487bb9 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487bc7; continue; }
  pc = 0x487bc2; continue;
 }
 0x487bc2 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487bcc; continue; }
  pc = 0x487bc7; continue;
 }
 0x487bc7 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487bcc; continue;
 }
 0x487bcc => {
  { let v: u32 = cpu.r8(0x4e156au32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6874u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1642u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487c23); f_004880f4(cpu); if cpu.returned_to != 0x487c23 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x487c3c; continue; }
  pc = 0x487c2f; continue;
 }
 0x487c2f => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487c3c; continue;
 }
 0x487c3c => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487c44 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487c45 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487c97; continue; }
  pc = 0x487c92; continue;
 }
 0x487c92 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487c9c; continue; }
  pc = 0x487c97; continue;
 }
 0x487c97 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487c9c; continue;
 }
 0x487c9c => {
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x487cf0; continue; }
  pc = 0x487cb9; continue;
 }
 0x487cb9 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x487cf0; continue; }
  pc = 0x487cbe; continue;
 }
 0x487cbe => {
  { let v: u16 = cpu.r16(0x5d6964u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487ced); f_004880f4(cpu); if cpu.returned_to != 0x487ced { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x487cf0; continue;
 }
 0x487cf0 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487d2b; continue; }
  pc = 0x487d26; continue;
 }
 0x487d26 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487d30; continue; }
  pc = 0x487d2b; continue;
 }
 0x487d2b => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487d30; continue;
 }
 0x487d30 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x487d44; continue; }
  pc = 0x487d36; continue;
 }
 0x487d36 => {
  { let v: u32 = cpu.r8(0x4e156au32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  pc = 0x487d7f; continue;
 }
 0x487d44 => {
  { let v: u32 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x487d5a); f_00420989(cpu); if cpu.returned_to != 0x487d5a { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x487d7c; continue; }
  pc = 0x487d6a; continue;
 }
 0x487d6a => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x487d7c; continue; }
  pc = 0x487d6f; continue;
 }
 0x487d6f => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if !cpu.zf { pc = 0x487d7c; continue; }
  pc = 0x487d74; continue;
 }
 0x487d74 => {
  { let v: u32 = cpu.ebx & 0x2u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x487d7f; continue;
 }
 0x487d7c => {
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  pc = 0x487d7f; continue;
 }
 0x487d7f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x15e9u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487d94; continue; }
  pc = 0x487d8f; continue;
 }
 0x487d8f => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487d99; continue; }
  pc = 0x487d94; continue;
 }
 0x487d94 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487d99; continue;
 }
 0x487d99 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x487da7; continue; }
  pc = 0x487da7; continue;
 }
 0x487da7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487dc3); f_004880f4(cpu); if cpu.returned_to != 0x487dc3 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487dd4; continue; }
  pc = 0x487dcf; continue;
 }
 0x487dcf => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487dd9; continue; }
  pc = 0x487dd4; continue;
 }
 0x487dd4 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487dd9; continue;
 }
 0x487dd9 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d69a4u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68d0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68d2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5d69b4u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x5d69b6u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x161au32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487e57); f_004880f4(cpu); if cpu.returned_to != 0x487e57 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.push(0x487e5f); f_004882df(cpu); if cpu.returned_to != 0x487e5f { return; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x487e73; continue; }
  pc = 0x487e66; continue;
 }
 0x487e66 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487e73; continue;
 }
 0x487e73 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487e7b => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x487f65; continue; }
  pc = 0x487ecf; continue;
 }
 0x487ecf => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if cpu.cf { pc = 0x487f65; continue; }
  pc = 0x487ed8; continue;
 }
 0x487ed8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487ee6; continue; }
  pc = 0x487ee1; continue;
 }
 0x487ee1 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487eeb; continue; }
  pc = 0x487ee6; continue;
 }
 0x487ee6 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487eeb; continue;
 }
 0x487eeb => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5d6984u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x5d6974u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x487f3e; continue; }
  pc = 0x487f3a; continue;
 }
 0x487f3a => {
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x487f3e; continue;
 }
 0x487f3e => {
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16d5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487f62); f_004880f4(cpu); if cpu.returned_to != 0x487f62 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x487f65; continue;
 }
 0x487f65 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d0u32, v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68d2u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16b5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487fab; continue; }
  pc = 0x487fa6; continue;
 }
 0x487fa6 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487fb0; continue; }
  pc = 0x487fab; continue;
 }
 0x487fab => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487fb0; continue;
 }
 0x487fb0 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x487fbe; continue; }
  pc = 0x487fbe; continue;
 }
 0x487fbe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x487fda); f_004880f4(cpu); if cpu.returned_to != 0x487fda { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x487feb; continue; }
  pc = 0x487fe6; continue;
 }
 0x487fe6 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x487ff0; continue; }
  pc = 0x487feb; continue;
 }
 0x487feb => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x487ff0; continue;
 }
 0x487ff0 => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d6974u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16cdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x48805d; continue; }
  pc = 0x488047; continue;
 }
 0x488047 => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if cpu.cf { pc = 0x48805d; continue; }
  pc = 0x48804c; continue;
 }
 0x48804c => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x488056); f_004880f4(cpu); if cpu.returned_to != 0x488056 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4880d8; continue;
 }
 0x48805d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x488067); f_004880f4(cpu); if cpu.returned_to != 0x488067 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48807a; continue; }
  pc = 0x488075; continue;
 }
 0x488075 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48807f; continue; }
  pc = 0x48807a; continue;
 }
 0x48807a => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48807f; continue;
 }
 0x48807f => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6984u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x4880b1; continue; }
  pc = 0x4880ad; continue;
 }
 0x4880ad => {
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4880b1; continue;
 }
 0x4880b1 => {
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16d5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4880d5); f_004880f4(cpu); if cpu.returned_to != 0x4880d5 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x4880d8; continue;
 }
 0x4880d8 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4880ec; continue; }
  pc = 0x4880df; continue;
 }
 0x4880df => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4880ec; continue;
 }
 0x4880ec => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4882eb => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ceu32, v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68ccu32, v); }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)).wrapping_add(0x8u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4d8236u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x4d6ab4u32).wrapping_add(0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x5d6924u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x740u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48837b; continue; }
  pc = 0x488376; continue;
 }
 0x488376 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x488380; continue; }
  pc = 0x48837b; continue;
 }
 0x48837b => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x488380; continue;
 }
 0x488380 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { pc = 0x48838e; continue; }
  pc = 0x48838e; continue;
 }
 0x48838e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4883aa); f_004880f4(cpu); if cpu.returned_to != 0x4883aa { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4883bb; continue; }
  pc = 0x4883b6; continue;
 }
 0x4883b6 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4883c0; continue; }
  pc = 0x4883bb; continue;
 }
 0x4883bb => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x4883c0; continue;
 }
 0x4883c0 => {
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5cc724u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = 0x5cc764u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x9u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x758u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x94u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x488433); f_004880f4(cpu); if cpu.returned_to != 0x488433 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48844a; continue; }
  pc = 0x48843d; continue;
 }
 0x48843d => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48844a; continue;
 }
 0x48844a => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457bf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457bf1); }
 let mut pc: u32 = 0x457bf1;
 loop { match pc {
 0x457bf1 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457bfd; continue;
 }
 0x457bfd => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c08; continue;
 }
 0x457c08 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c13; continue;
 }
 0x457c13 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x30u8); }
  if !cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c21; continue;
 }
 0x457c21 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x457c4f; continue; }
  pc = 0x457c28; continue;
 }
 0x457c28 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x457c4f; continue; }
  pc = 0x457c2f; continue;
 }
 0x457c2f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x457c4f; continue; }
  pc = 0x457c36; continue;
 }
 0x457c36 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x11u8); }
  if cpu.zf { pc = 0x457c4f; continue; }
  pc = 0x457c3d; continue;
 }
 0x457c3d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x457c4f; continue; }
  pc = 0x457c44; continue;
 }
 0x457c44 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c4f; continue;
 }
 0x457c4f => {
  { cpu.sub8(cpu.r8(0x7au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c5a; continue;
 }
 0x457c5a => {
  { cpu.sub16(cpu.r16(0x4d677eu32), 0x0u16); }
  if !cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c68; continue;
 }
 0x457c68 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7au32.wrapping_add(cpu.ebp))); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c77; continue;
 }
 0x457c77 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c82; continue;
 }
 0x457c82 => {
  cpu.push(0x457c87); f_0041c743(cpu); if cpu.returned_to != 0x457c87 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457c9b; continue;
 }
 0x457c9b => {
  cpu.push(0x457ca0); f_0041c743(cpu); if cpu.returned_to != 0x457ca0 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x457cb1; continue; }
  pc = 0x457caa; continue;
 }
 0x457caa => {
  cpu.push(0x457caf); f_0045aaf5(cpu); if cpu.returned_to != 0x457caf { return; }
  if !cpu.cf { pc = 0x457d04; continue; }
  pc = 0x457cb1; continue;
 }
 0x457cb1 => {
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if cpu.zf { pc = 0x457cbe; continue; }
  pc = 0x457cb7; continue;
 }
 0x457cb7 => {
  cpu.push(0x457cbc); f_0045aa00(cpu); if cpu.returned_to != 0x457cbc { return; }
  if !cpu.cf { pc = 0x457d04; continue; }
  pc = 0x457cbe; continue;
 }
 0x457cbe => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x457ccb; continue; }
  pc = 0x457cc4; continue;
 }
 0x457cc4 => {
  cpu.push(0x457cc9); f_0045abb2(cpu); if cpu.returned_to != 0x457cc9 { return; }
  if !cpu.cf { pc = 0x457d04; continue; }
  pc = 0x457ccb; continue;
 }
 0x457ccb => {
  { let v: u32 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x457d04; continue; }
  pc = 0x457cd4; continue;
 }
 0x457cd4 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x41u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if cpu.cf { pc = 0x457cef; continue; }
  pc = 0x457cea; continue;
 }
 0x457cea => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x457cef; continue;
 }
 0x457cef => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5ba0f7u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x457cfe); f_0041c743(cpu); if cpu.returned_to != 0x457cfe { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x457d04); cpu.call(t); if cpu.returned_to != 0x457d04 { return; }
  pc = 0x457d04; continue;
 }
 0x457d04 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457d06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457d06); }
 let mut pc: u32 = 0x457d06;
 loop { match pc {
 0x457d06 => {
  cpu.push(0x457d0b); f_0041c743(cpu); if cpu.returned_to != 0x457d0b { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x457d17; continue; }
  pc = 0x457d11; continue;
 }
 0x457d11 => {
  cpu.push(0x457d16); f_0045ae9b(cpu); if cpu.returned_to != 0x457d16 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x457d17 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5ba040u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x2au8); }
  if cpu.zf { pc = 0x457f70; continue; }
  pc = 0x457d39; continue;
 }
 0x457d39 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if cpu.zf { pc = 0x457f70; continue; }
  pc = 0x457d42; continue;
 }
 0x457d42 => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if cpu.zf { pc = 0x457f70; continue; }
  pc = 0x457d4b; continue;
 }
 0x457d4b => {
  { cpu.sub8((cpu.edx as u8), 0x27u8); }
  if cpu.zf { pc = 0x457f70; continue; }
  pc = 0x457d54; continue;
 }
 0x457d54 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x457f70; continue; }
  pc = 0x457d5f; continue;
 }
 0x457d5f => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bc03u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5ba05eu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5ba060u32, v); }
  cpu.push(0x457d9f); f_0041c743(cpu); if cpu.returned_to != 0x457d9f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba05eu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x5ba05eu32, v); }
  cpu.push(0x457db3); f_0041c743(cpu); if cpu.returned_to != 0x457db3 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba060u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x5ba060u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5ba1bcu32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x5ba0d6u32, v); }
  { let v: u32 = 0x5ba0c6u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = 0x5ba040u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5ba058u32, v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x457dfa); f_0041c808(cpu); if cpu.returned_to != 0x457dfa { return; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16) | 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5ba05cu32, v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x457e15); f_0041c808(cpu); if cpu.returned_to != 0x457e15 { return; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5ba1bau32, v); }
  cpu.push(0x457e23); f_0041c743(cpu); if cpu.returned_to != 0x457e23 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  pc = 0x457e26; continue;
 }
 0x457e26 => {
  cpu.push(0x457e2b); f_00457fa6(cpu); if cpu.returned_to != 0x457e2b { return; }
  if cpu.cf { pc = 0x457f70; continue; }
  pc = 0x457e31; continue;
 }
 0x457e31 => {
  { let v: u8 = 0x19u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x19u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5ba058u32); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5ba058u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5ba05cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5ba0b6u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba05eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba060u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x457e89); f_0045803b(cpu); if cpu.returned_to != 0x457e89 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x457e95; continue; }
  pc = 0x457e8c; continue;
 }
 0x457e8c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x457f70; continue;
 }
 0x457e95 => {
  { let v: u8 = 0xfu8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x457ea5); f_00489637(cpu); if cpu.returned_to != 0x457ea5 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x457ebb; continue; }
  pc = 0x457ead; continue;
 }
 0x457ead => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x3fu32.wrapping_add(cpu.ebx), v); }
  pc = 0x457ebb; continue;
 }
 0x457ebb => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x457f70; continue; }
  pc = 0x457ed3; continue;
 }
 0x457ed3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x5ba1bau32), 0x1u16); }
  if !cpu.zf { pc = 0x457ee9; continue; }
  pc = 0x457ee7; continue;
 }
 0x457ee7 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x457ee9; continue;
 }
 0x457ee9 => {
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x457e26; continue; }
  pc = 0x457ef5; continue;
 }
 0x457ef5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5ba05cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5ba0b6u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba05eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5ba060u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x457f30); f_0045803b(cpu); if cpu.returned_to != 0x457f30 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x457f70; continue; }
  pc = 0x457f33; continue;
 }
 0x457f33 => {
  { let v: u8 = 0xfu8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x457f3e); f_00489637(cpu); if cpu.returned_to != 0x457f3e { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x457f50; continue; }
  pc = 0x457f46; continue;
 }
 0x457f46 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x457f50; continue;
 }
 0x457f50 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x457f70; continue; }
  pc = 0x457f54; continue;
 }
 0x457f54 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5ba1bcu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.esi)), 0x10u8); cpu.w8(0x7bu32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x457f70 => {
  { let v: u32 = 0x5ba040u32; cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x457f7a; continue;
 }
 0x457f7a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x457fa4; continue; }
  pc = 0x457f81; continue;
 }
 0x457f81 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x87654321u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x457f7a; continue; }
  pc = 0x457fa4; continue;
 }
 0x457fa4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00457fa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x457fa6); }
 let mut pc: u32 = 0x457fa6;
 loop { match pc {
 0x457fa6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bc03u32); cpu.eax = v; }
  { let v: u32 = 0x5ba0c6u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  pc = 0x457fc1; continue;
 }
 0x457fc1 => {
  { cpu.sub8(cpu.r8(0x5ba0d6u32), 0x0u8); }
  if cpu.zf { pc = 0x458036; continue; }
  pc = 0x457fca; continue;
 }
 0x457fca => {
  if cpu.sf { pc = 0x458036; continue; }
  pc = 0x457fcc; continue;
 }
 0x457fcc => {
  cpu.push(0x457fd1); f_0041c743(cpu); if cpu.returned_to != 0x457fd1 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  pc = 0x457fd8; continue;
 }
 0x457fd8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x457fe4; continue; }
  pc = 0x457fde; continue;
 }
 0x457fde => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  cpu.poll(); pc = 0x457fd8; continue;
 }
 0x457fe4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5ba0d6u32)); cpu.w8(0x5ba0d6u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x457fc1; continue; }
  pc = 0x458000; continue;
 }
 0x458000 => {
  { let v: u16 = (cpu.eax as u8) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x457fc1; continue; }
  pc = 0x458015; continue;
 }
 0x458015 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x457fc1; continue; }
  pc = 0x45802b; continue;
 }
 0x45802b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x457fc1; continue; }
  pc = 0x458031; continue;
 }
 0x458031 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458036 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045803b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45803b); } r_0045803b(cpu, 0x45803b); }
pub fn f_00458074(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458074); } r_0045803b(cpu, 0x458074); }
fn r_0045803b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45803b => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x57557fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x575581u32, v); }
  cpu.push(0x45804e); f_00420989(cpu); if cpu.returned_to != 0x45804e { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x458060; continue; }
  pc = 0x45805c; continue;
 }
 0x45805c => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if !cpu.zf { pc = 0x4580c9; continue; }
  pc = 0x458060; continue;
 }
 0x458060 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x45806d; continue; }
  pc = 0x458069; continue;
 }
 0x458069 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if !cpu.zf { pc = 0x4580c9; continue; }
  pc = 0x45806d; continue;
 }
 0x45806d => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x45807d; continue; }
  pc = 0x458079; continue;
 }
 0x458074 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4580b2; continue; }
  pc = 0x458076; continue;
 }
 0x458076 => {
  { let v: u8 = cpu.add8(cpu.r8(0x3cu32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), ((cpu.edx >> 8) as u8)); cpu.w8(0x3cu32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.push(0x2b); }
  if !cpu.zf { pc = 0x4580c9; continue; }
  pc = 0x45807d; continue;
 }
 0x458079 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if !cpu.zf { pc = 0x4580c9; continue; }
  pc = 0x45807d; continue;
 }
 0x45807d => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x45808d; continue; }
  pc = 0x458089; continue;
 }
 0x458089 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if !cpu.zf { pc = 0x4580c9; continue; }
  pc = 0x45808d; continue;
 }
 0x45808d => {
  { let v: u16 = cpu.r16(0x57557fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x575581u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45809f); f_004716d0(cpu); if cpu.returned_to != 0x45809f { return; }
  if cpu.cf { pc = 0x4580c9; continue; }
  pc = 0x4580a1; continue;
 }
 0x4580a1 => {
  { let v: u32 = 0x5ce994u32; cpu.w32(0x4d6af0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x4580c0); f_0046d630(cpu); if cpu.returned_to != 0x4580c0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x4580c9; continue; }
  pc = 0x4580c6; continue;
 }
 0x4580b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x58c889u32); cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x458113; continue; }
  pc = 0x4580be; continue;
 }
 0x4580be => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x4580c9; continue; }
  pc = 0x4580c6; continue;
 }
 0x4580c6 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4580c9 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458113 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x458142; continue; }
  pc = 0x458131; continue;
 }
 0x458131 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x45821f; continue; }
  pc = 0x45813d; continue;
 }
 0x45813d => {
  pc = 0x458218; continue;
 }
 0x458142 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x458226; continue; }
  pc = 0x45814b; continue;
 }
 0x45814b => {
  pc = 0x45821f; continue;
 }
 0x458218 => {
  { let v: u8 = 0x1u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x458247; continue;
 }
 0x45821f => {
  { let v: u8 = 0x2u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x458247; continue;
 }
 0x458226 => {
  { let v: u8 = 0x3u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x458247; continue;
 }
 0x458247 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x458256; continue; }
  pc = 0x45824f; continue;
 }
 0x45824f => {
  { let v: u16 = 0x1u16; cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x458256; continue;
 }
 0x458256 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458258); }
 let mut pc: u32 = 0x458258;
 loop { match pc {
 0x458258 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x400u16); }
  if (cpu.sf == cpu.of) { pc = 0x45828d; continue; }
  pc = 0x458282; continue;
 }
 0x458282 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45828d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458290); }
 let mut pc: u32 = 0x458290;
 loop { match pc {
 0x458290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let a = cpu.edx; let b = cpu.ecx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let a = cpu.ecx; let b = cpu.edx; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x4582c4; continue; }
  pc = 0x4582bf; continue;
 }
 0x4582bf => {
  { let v: u32 = 0x3ffu32; cpu.ecx = v; }
  pc = 0x4582c4; continue;
 }
 0x4582c4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x4d6accu32)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004582d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4582d2); }
 let mut pc: u32 = 0x4582d2;
 loop { match pc {
 0x4582d2 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x458321; continue; }
  pc = 0x4582d8; continue;
 }
 0x4582d8 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x45830a; continue; }
  pc = 0x4582e3; continue;
 }
 0x4582e3 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x458311; continue; }
  pc = 0x4582e5; continue;
 }
 0x4582e5 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x458322; continue; }
  pc = 0x4582ec; continue;
 }
 0x4582ec => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4582f6); f_00458357(cpu); if cpu.returned_to != 0x4582f6 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x458302); f_004824fa(cpu); if cpu.returned_to != 0x458302 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x458321; continue; }
  pc = 0x458309; continue;
 }
 0x458309 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45830a => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x458321; continue;
 }
 0x458311 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x458327; continue; }
  pc = 0x45831d; continue;
 }
 0x45831d => {
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x458321; continue;
 }
 0x458321 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458322 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458327 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x458330; continue; }
  pc = 0x458330; continue;
 }
 0x458330 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x458337); f_0043fede(cpu); if cpu.returned_to != 0x458337 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x458351; continue; }
  pc = 0x45833c; continue;
 }
 0x45833c => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x458351; continue; }
  pc = 0x458344; continue;
 }
 0x458344 => {
  cpu.push(0x458349); f_004782f8(cpu); if cpu.returned_to != 0x458349 { return; }
  if cpu.cf { pc = 0x458351; continue; }
  pc = 0x45834b; continue;
 }
 0x45834b => {
  cpu.push(0x458350); f_004525bf(cpu); if cpu.returned_to != 0x458350 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458351 => {
  cpu.push(0x458356); f_0043ffdb(cpu); if cpu.returned_to != 0x458356 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458357(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458357); }
 let mut pc: u32 = 0x458357;
 loop { match pc {
 0x458357 => {
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5da9c4u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5da9c6u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x4583c5; continue; }
  pc = 0x45838d; continue;
 }
 0x45838d => {
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x4583b9; continue; }
  pc = 0x458396; continue;
 }
 0x458396 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x4583b0); f_00452a56(cpu); if cpu.returned_to != 0x4583b0 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4583b9; continue;
 }
 0x4583b9 => {
  { let v: u16 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4583fd; continue;
 }
 0x4583c5 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4583ed; continue; }
  pc = 0x4583e2; continue;
 }
 0x4583e2 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x4583ed; continue; }
  pc = 0x4583eb; continue;
 }
 0x4583eb => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4583ed; continue;
 }
 0x4583ed => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x4583fd); f_0045bd38(cpu); if cpu.returned_to != 0x4583fd { return; }
  pc = 0x4583fd; continue;
 }
 0x4583fd => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x45840a; continue; }
  pc = 0x458406; continue;
 }
 0x458406 => {
  { let v: u16 = cpu.add16((cpu.edx as u16), 0xcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x45840a; continue;
 }
 0x45840a => {
  { let v: u32 = cpu.r8(0x4e1f4eu32) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = 0x1du8; cpu.w8(0x5d5f23u32, v); }
  cpu.push(0x458427); f_00480e87(cpu); if cpu.returned_to != 0x458427 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458429); }
 let mut pc: u32 = 0x458429;
 loop { match pc {
 0x458429 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x458437; continue; }
  pc = 0x458432; continue;
 }
 0x458432 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x45843c; continue; }
  pc = 0x458437; continue;
 }
 0x458437 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x45843c; continue;
 }
 0x45843c => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045844b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45844b); }
 let mut pc: u32 = 0x45844b;
 loop { match pc {
 0x45844b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5ba1c1u32), 0x0u8); }
  if !cpu.zf { pc = 0x45849c; continue; }
  pc = 0x45845e; continue;
 }
 0x45845e => {
  { cpu.sub8(cpu.r8(0x4eab2bu32), 0x0u8); }
  if !cpu.zf { pc = 0x45849c; continue; }
  pc = 0x458467; continue;
 }
 0x458467 => {
  cpu.push(0x45846c); f_004679cb(cpu); if cpu.returned_to != 0x45846c { return; }
  if cpu.cf { pc = 0x45849c; continue; }
  pc = 0x45846e; continue;
 }
 0x45846e => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x458494; continue; }
  pc = 0x458484; continue;
 }
 0x458484 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x45848b); f_00401130(cpu); if cpu.returned_to != 0x45848b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x458494; continue;
 }
 0x458494 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2bu32, v); }
  pc = 0x45849c; continue;
 }
 0x45849c => {
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x4584c9; continue; }
  pc = 0x4584a5; continue;
 }
 0x4584a5 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x4584c2); f_00452a56(cpu); if cpu.returned_to != 0x4584c2 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4584c9; continue;
 }
 0x4584c9 => {
  { let v: u32 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d5ec8u32, v); }
  cpu.push(0x4584fe); f_004820da(cpu); if cpu.returned_to != 0x4584fe { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5ec8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x45850b; continue; }
  pc = 0x458508; continue;
 }
 0x458508 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xbu32.wrapping_add(cpu.eax))); cpu.w8(0xbu32.wrapping_add(cpu.eax), v); }
  pc = 0x45850b; continue;
 }
 0x45850b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045850c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45850c); }
 let mut pc: u32 = 0x45850c;
 loop { match pc {
 0x45850c => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45851b); f_004679cb(cpu); if cpu.returned_to != 0x45851b { return; }
  if cpu.cf { pc = 0x458544; continue; }
  pc = 0x45851d; continue;
 }
 0x45851d => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x458543; continue; }
  pc = 0x458533; continue;
 }
 0x458533 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x45853a); f_00401130(cpu); if cpu.returned_to != 0x45853a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x458543; continue;
 }
 0x458543 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x458544; continue;
 }
 0x458544 => {
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x458571; continue; }
  pc = 0x45854d; continue;
 }
 0x45854d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x45856a); f_00452a56(cpu); if cpu.returned_to != 0x45856a { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  pc = 0x458571; continue;
 }
 0x458571 => {
  { let v: u32 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d5ec8u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d5f21u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1c1u32, v); }
  cpu.push(0x4585b4); f_004820da(cpu); if cpu.returned_to != 0x4585b4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1c1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5f21u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5ec8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4585cf; continue; }
  pc = 0x4585cc; continue;
 }
 0x4585cc => {
  { let v: u8 = cpu.inc8(cpu.r8(0xbu32.wrapping_add(cpu.eax))); cpu.w8(0xbu32.wrapping_add(cpu.eax), v); }
  pc = 0x4585cf; continue;
 }
 0x4585cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004585d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4585d0); }
 let mut pc: u32 = 0x4585d0;
 loop { match pc {
 0x4585d0 => {
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5b9cecu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5b9ceeu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x458646; continue; }
  pc = 0x45860e; continue;
 }
 0x45860e => {
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x45863a; continue; }
  pc = 0x458617; continue;
 }
 0x458617 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x458631); f_00452a56(cpu); if cpu.returned_to != 0x458631 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x45863a; continue;
 }
 0x45863a => {
  { let v: u16 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x458689; continue;
 }
 0x458646 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x458657); f_00453b70(cpu); if cpu.returned_to != 0x458657 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x4586ad; continue; }
  pc = 0x45865b; continue;
 }
 0x45865b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x458679; continue; }
  pc = 0x45866e; continue;
 }
 0x45866e => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x458679; continue; }
  pc = 0x458677; continue;
 }
 0x458677 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x458679; continue;
 }
 0x458679 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x458689); f_0045bd38(cpu); if cpu.returned_to != 0x458689 { return; }
  pc = 0x458689; continue;
 }
 0x458689 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x458696; continue; }
  pc = 0x458692; continue;
 }
 0x458692 => {
  { let v: u16 = cpu.add16((cpu.edx as u16), 0xcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x458696; continue;
 }
 0x458696 => {
  { let v: u32 = cpu.r8(0x4e1f4eu32) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  cpu.push(0x4586ac); f_004810cb(cpu); if cpu.returned_to != 0x4586ac { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4586ad; continue;
 }
 0x4586ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004586f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4586f4); }
 let mut pc: u32 = 0x4586f4;
 loop { match pc {
 0x4586f4 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x4586fc); f_00458cb3(cpu); if cpu.returned_to != 0x4586fc { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458731(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458731); } r_00458731(cpu, 0x458731); }
pub fn f_0045883e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45883e); } r_00458731(cpu, 0x45883e); }
fn r_00458731(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x458731 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x45873f; continue;
 }
 0x45873f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x45874a; continue;
 }
 0x45874a => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458755; continue;
 }
 0x458755 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458760; continue;
 }
 0x458760 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x45876b; continue;
 }
 0x45876b => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458782; continue;
 }
 0x458782 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1e1u16); }
  if cpu.zf { pc = 0x4589c6; continue; }
  pc = 0x458790; continue;
 }
 0x458790 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1471u16); }
  if cpu.zf { pc = 0x45890a; continue; }
  pc = 0x45879b; continue;
 }
 0x45879b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1447u16); }
  if cpu.zf { pc = 0x458951; continue; }
  pc = 0x4587a6; continue;
 }
 0x4587a6 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x684u16); }
  if cpu.zf { pc = 0x458871; continue; }
  pc = 0x4587b1; continue;
 }
 0x4587b1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x674u16); }
  if cpu.zf { pc = 0x45885f; continue; }
  pc = 0x4587bc; continue;
 }
 0x4587bc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1698u16); }
  if cpu.zf { pc = 0x458883; continue; }
  pc = 0x4587c7; continue;
 }
 0x4587c7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1444u16); }
  if cpu.zf { pc = 0x4588a0; continue; }
  pc = 0x4587d2; continue;
 }
 0x4587d2 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1443u16); }
  if cpu.zf { pc = 0x4588b2; continue; }
  pc = 0x4587dd; continue;
 }
 0x4587dd => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1443u16); }
  if cpu.zf { pc = 0x4588b2; continue; }
  pc = 0x4587e8; continue;
 }
 0x4587e8 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1448u16); }
  if cpu.zf { pc = 0x4588c4; continue; }
  pc = 0x4587f3; continue;
 }
 0x4587f3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x16ddu16); }
  if cpu.zf { pc = 0x4588fb; continue; }
  pc = 0x4587fe; continue;
 }
 0x4587fe => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1472u16); }
  if cpu.zf { pc = 0x458919; continue; }
  pc = 0x458809; continue;
 }
 0x458809 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x15fu16); }
  if cpu.zf { pc = 0x458830; continue; }
  pc = 0x458810; continue;
 }
 0x458810 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x160u16); }
  if cpu.zf { pc = 0x458830; continue; }
  pc = 0x458817; continue;
 }
 0x458817 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x161u16); }
  if !cpu.zf { pc = 0x458895; continue; }
  pc = 0x45881e; continue;
 }
 0x45881e => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458830 => {
  cpu.push(0x458835); f_0046d525(cpu); if cpu.returned_to != 0x458835 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x45883d; continue;
 }
 0x45883d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x458856; continue; }
  pc = 0x458844; continue;
 }
 0x45883e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x458856; continue; }
  pc = 0x458844; continue;
 }
 0x458844 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x458856; continue; }
  pc = 0x45884b; continue;
 }
 0x45884b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458856; continue;
 }
 0x458856 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x45897b; continue;
 }
 0x45885f => {
  { let v: u16 = 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458871 => {
  { let v: u16 = 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458883 => {
  { let v: u16 = 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458895 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1440u16); }
  if !cpu.zf { pc = 0x458960; continue; }
  pc = 0x4588a0; continue;
 }
 0x4588a0 => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x4588b2 => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x4588c4 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x4588cf; continue;
 }
 0x4588cf => {
  cpu.push(0x4588d4); f_0045bf77(cpu); if cpu.returned_to != 0x4588d4 { return; }
  if !cpu.cf { pc = 0x4589bf; continue; }
  pc = 0x4588da; continue;
 }
 0x4588da => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x4588e2; continue;
 }
 0x4588e2 => {
  { let v: u16 = 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x56d525u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x4588fb => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x45890a => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458919 => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458951 => {
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x458960 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458967; continue;
 }
 0x458967 => {
  { let v: u16 = 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x15eu16); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458976; continue;
 }
 0x458976 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45897b; continue;
 }
 0x45897b => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458981; continue;
 }
 0x458981 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x4589bf; continue; }
  pc = 0x458988; continue;
 }
 0x458988 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4589a1); f_004895c7(cpu); if cpu.returned_to != 0x4589a1 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x89u32.wrapping_add(cpu.ebp))); cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x56d525u32), 0x0u8); }
  if cpu.zf { pc = 0x4589b1; continue; }
  pc = 0x4589b1; continue;
 }
 0x4589b1 => {
  cpu.push(0x4589b6); f_00458a7f(cpu); if cpu.returned_to != 0x4589b6 { return; }
  if cpu.cf { pc = 0x4589bf; continue; }
  pc = 0x4589b8; continue;
 }
 0x4589b8 => {
  cpu.push(0x4589bd); f_0048990c(cpu); if cpu.returned_to != 0x4589bd { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4589bf => {
  { let v: u8 = 0xffu8; cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4589c6 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x458a05; continue; }
  pc = 0x4589cc; continue;
 }
 0x4589cc => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x458a05; continue; }
  pc = 0x4589d3; continue;
 }
 0x4589d3 => {
  { cpu.sub8(cpu.r8(0x1fu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x458a05; continue; }
  pc = 0x4589d9; continue;
 }
 0x4589d9 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4589f7); f_004895c7(cpu); if cpu.returned_to != 0x4589f7 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x89u32.wrapping_add(cpu.ebp))); cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x458a03); f_00489637(cpu); if cpu.returned_to != 0x458a03 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458a05 => {
  { let v: u8 = 0xffu8; cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458a0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458a0c); } r_00458a0c(cpu, 0x458a0c); }
pub fn f_00458a3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458a3e); } r_00458a0c(cpu, 0x458a3e); }
pub fn f_00458a7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458a7f); } r_00458a0c(cpu, 0x458a7f); }
fn r_00458a0c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x458a0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1b9u32, v); }
  pc = 0x458a13; continue;
 }
 0x458a13 => {
  { let v: u32 = 0x4ba552u32; cpu.edi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x458a1d; continue;
 }
 0x458a1d => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x458a2a; continue; }
  pc = 0x458a23; continue;
 }
 0x458a23 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x458a1d; continue; }
  pc = 0x458a28; continue;
 }
 0x458a28 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458a2a => {
  { cpu.sub8(cpu.r8(0x5ba1b9u32), 0x0u8); }
  if cpu.zf { pc = 0x458a5a; continue; }
  pc = 0x458a33; continue;
 }
 0x458a33 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x458a3a); f_00489968(cpu); if cpu.returned_to != 0x458a3a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x458a52); f_0042f53c(cpu); if cpu.returned_to != 0x458a52 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458a3e => {
  { cpu.pop(); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x458a52); f_0042f53c(cpu); if cpu.returned_to != 0x458a52 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458a5a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  cpu.push(0x458a67); f_004899b0(cpu); if cpu.returned_to != 0x458a67 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x458a77); f_0042f53c(cpu); if cpu.returned_to != 0x458a77 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458a7f => {
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1b9u32, v); }
  cpu.poll(); pc = 0x458a13; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458a8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458a8b); } r_00458a8b(cpu, 0x458a8b); }
pub fn f_00458b3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x458b3e); } r_00458a8b(cpu, 0x458b3e); }
fn r_00458a8b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x458a8b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x458b2b; continue; }
  pc = 0x458a95; continue;
 }
 0x458a95 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x458aaa; continue; }
  pc = 0x458aa5; continue;
 }
 0x458aa5 => {
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)).wrapping_add(0x2u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x458aaa; continue;
 }
 0x458aaa => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x458b2b; continue; }
  pc = 0x458ab1; continue;
 }
 0x458ab1 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x458ab8); f_0043fede(cpu); if cpu.returned_to != 0x458ab8 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x458b2c; continue; }
  pc = 0x458abd; continue;
 }
 0x458abd => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x458b2c; continue; }
  pc = 0x458ac2; continue;
 }
 0x458ac2 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x458af3; continue; }
  pc = 0x458ad7; continue;
 }
 0x458ad7 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x40u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x458af3; continue; }
  pc = 0x458ae4; continue;
 }
 0x458ae4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x458aec); f_0041c743(cpu); if cpu.returned_to != 0x458aec { return; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  pc = 0x458af3; continue;
 }
 0x458af3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x458b09; continue; }
  pc = 0x458b02; continue;
 }
 0x458b02 => {
  cpu.push(0x458b07); f_00458cb3(cpu); if cpu.returned_to != 0x458b07 { return; }
  pc = 0x458b2c; continue;
 }
 0x458b09 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xf0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x458b2c; continue; }
  pc = 0x458b17; continue;
 }
 0x458b17 => {
  { let v: u8 = cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x458b1f; continue; }
  pc = 0x458b1c; continue;
 }
 0x458b1c => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x458b1f; continue;
 }
 0x458b1f => {
  { let v: u8 = cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.edi)), 0x5u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x458b2c; continue; }
  pc = 0x458b25; continue;
 }
 0x458b25 => {
  { let v: u8 = 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x458b2c; continue;
 }
 0x458b2b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458b2c => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x458b33); f_0043fede(cpu); if cpu.returned_to != 0x458b33 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x458b4a; continue; }
  pc = 0x458b38; continue;
 }
 0x458b38 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x458b4a; continue; }
  pc = 0x458b3d; continue;
 }
 0x458b3d => {
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458b3e => {
  { let v: u32 = 0xc63e0000u32; cpu.w32(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458b4a => {
  cpu.push(0x458b4f); f_0043ffdb(cpu); if cpu.returned_to != 0x458b4f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458b50); }
 let mut pc: u32 = 0x458b50;
 loop { match pc {
 0x458b50 => {
  cpu.push(0x458b55); f_0046a867(cpu); if cpu.returned_to != 0x458b55 { return; }
  if !cpu.cf { pc = 0x458b61; continue; }
  pc = 0x458b57; continue;
 }
 0x458b57 => {
  { let v: u8 = cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x458b61); f_0043ffdb(cpu); if cpu.returned_to != 0x458b61 { return; }
  pc = 0x458b61; continue;
 }
 0x458b61 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458b62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458b62); }
 let mut pc: u32 = 0x458b62;
 loop { match pc {
 0x458b62 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x458b7d; continue; }
  pc = 0x458b67; continue;
 }
 0x458b67 => {
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if cpu.zf { pc = 0x458b7d; continue; }
  pc = 0x458b6c; continue;
 }
 0x458b6c => {
  { cpu.sub8((cpu.ecx as u8), 0x16u8); }
  if cpu.zf { pc = 0x458b7d; continue; }
  pc = 0x458b71; continue;
 }
 0x458b71 => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { pc = 0x458b7d; continue; }
  pc = 0x458b76; continue;
 }
 0x458b76 => {
  cpu.push(0x458b7b); f_00458bd0(cpu); if cpu.returned_to != 0x458b7b { return; }
  if cpu.cf { pc = 0x458bcf; continue; }
  pc = 0x458b7d; continue;
 }
 0x458b7d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x458b84); f_00448783(cpu); if cpu.returned_to != 0x458b84 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x458bcf; continue; }
  pc = 0x458b88; continue;
 }
 0x458b88 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4b839cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { cpu.cf = false; }
  pc = 0x458bcf; continue;
 }
 0x458bcf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458bd0); }
 let mut pc: u32 = 0x458bd0;
 loop { match pc {
 0x458bd0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x458bec); f_00420989(cpu); if cpu.returned_to != 0x458bec { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458c3b; continue; }
  pc = 0x458c04; continue;
 }
 0x458c04 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458c3b; continue; }
  pc = 0x458c13; continue;
 }
 0x458c13 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458c3b; continue; }
  pc = 0x458c25; continue;
 }
 0x458c25 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458c3b; continue; }
  pc = 0x458c37; continue;
 }
 0x458c37 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458c3b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x458c42); f_00448195(cpu); if cpu.returned_to != 0x458c42 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458c44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458c44); }
 let mut pc: u32 = 0x458c44;
 loop { match pc {
 0x458c44 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x458c60); f_00420989(cpu); if cpu.returned_to != 0x458c60 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458caf; continue; }
  pc = 0x458c78; continue;
 }
 0x458c78 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458caf; continue; }
  pc = 0x458c87; continue;
 }
 0x458c87 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458caf; continue; }
  pc = 0x458c99; continue;
 }
 0x458c99 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56cab3u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x458caf; continue; }
  pc = 0x458cab; continue;
 }
 0x458cab => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458caf => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458cb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458cb3); }
 let mut pc: u32 = 0x458cb3;
 loop { match pc {
 0x458cb3 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x458dd7; continue; }
  pc = 0x458cbd; continue;
 }
 0x458cbd => {
  cpu.push(0x458cc2); f_0045955b(cpu); if cpu.returned_to != 0x458cc2 { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.edi)), 0x20u8); }
  if cpu.zf { pc = 0x458cd0; continue; }
  pc = 0x458cc8; continue;
 }
 0x458cc8 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.edi)), 0x3bu8); }
  if cpu.zf { pc = 0x458cd0; continue; }
  pc = 0x458cce; continue;
 }
 0x458cce => {
  pc = 0x458cd5; continue;
 }
 0x458cd0 => {
  cpu.push(0x458cd5); f_00458e9a(cpu); if cpu.returned_to != 0x458cd5 { return; }
  pc = 0x458cd5; continue;
 }
 0x458cd5 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x458cf9; continue; }
  pc = 0x458ce0; continue;
 }
 0x458ce0 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x458cf9; continue; }
  pc = 0x458cf5; continue;
 }
 0x458cf5 => {
  { let v: u8 = 0xffu8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x458cf9; continue;
 }
 0x458cf9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.edi)), 0x24u8); }
  if !cpu.zf { pc = 0x458d72; continue; }
  pc = 0x458d00; continue;
 }
 0x458d00 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x458d72; continue; }
  pc = 0x458d0a; continue;
 }
 0x458d0a => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x458d32; continue; }
  pc = 0x458d14; continue;
 }
 0x458d14 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x458d25; continue; }
  pc = 0x458d25; continue;
 }
 0x458d25 => {
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x458d49; continue;
 }
 0x458d32 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x458d43; continue; }
  pc = 0x458d43; continue;
 }
 0x458d43 => {
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x458d49; continue;
 }
 0x458d49 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x458d72; continue; }
  pc = 0x458d50; continue;
 }
 0x458d50 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x458d67; continue; }
  pc = 0x458d5a; continue;
 }
 0x458d5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x458d72; continue;
 }
 0x458d67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x458d72; continue;
 }
 0x458d72 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if !cpu.zf { pc = 0x458da6; continue; }
  pc = 0x458d78; continue;
 }
 0x458d78 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x458d89); f_00458dd8(cpu); if cpu.returned_to != 0x458d89 { return; }
  if !cpu.cf { pc = 0x458da4; continue; }
  pc = 0x458d8b; continue;
 }
 0x458d8b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x458d99; continue; }
  pc = 0x458d94; continue;
 }
 0x458d94 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x458d9e; continue; }
  pc = 0x458d99; continue;
 }
 0x458d99 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x458d9e; continue;
 }
 0x458d9e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x458da4; continue;
 }
 0x458da4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x458da6; continue;
 }
 0x458da6 => {
  cpu.push(0x458dab); f_00458ee1(cpu); if cpu.returned_to != 0x458dab { return; }
  cpu.push(0x458db0); f_0045948c(cpu); if cpu.returned_to != 0x458db0 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x458db9); f_0046a9bc(cpu); if cpu.returned_to != 0x458db9 { return; }
  { cpu.popad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x581bffu32, v); }
  cpu.push(0x458dc8); f_0047f7c2(cpu); if cpu.returned_to != 0x458dc8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x581bffu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x458dd6); f_00489968(cpu); if cpu.returned_to != 0x458dd6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x458dd7; continue;
 }
 0x458dd7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458dd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458dd8); }
 let mut pc: u32 = 0x458dd8;
 loop { match pc {
 0x458dd8 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xc8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x458e05); f_004714a5(cpu); if cpu.returned_to != 0x458e05 { return; }
  if cpu.cf { pc = 0x458e97; continue; }
  pc = 0x458e0b; continue;
 }
 0x458e0b => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xc8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x458e2d); f_004714a5(cpu); if cpu.returned_to != 0x458e2d { return; }
  if cpu.cf { pc = 0x458e97; continue; }
  pc = 0x458e2f; continue;
 }
 0x458e2f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x458e94; continue; }
  pc = 0x458e38; continue;
 }
 0x458e38 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x458e94; continue; }
  pc = 0x458e42; continue;
 }
 0x458e42 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xc8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x458e64); f_004714a5(cpu); if cpu.returned_to != 0x458e64 { return; }
  if cpu.cf { pc = 0x458e97; continue; }
  pc = 0x458e66; continue;
 }
 0x458e66 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x458e94; continue; }
  pc = 0x458e70; continue;
 }
 0x458e70 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xc8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x458e92); f_004714a5(cpu); if cpu.returned_to != 0x458e92 { return; }
  if cpu.cf { pc = 0x458e97; continue; }
  pc = 0x458e94; continue;
 }
 0x458e94 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x458e97 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458e9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458e9a); }
 let mut pc: u32 = 0x458e9a;
 loop { match pc {
 0x458e9a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x458eb4; continue;
 }
 0x458eb4 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x458ed9; continue; }
  pc = 0x458eb9; continue;
 }
 0x458eb9 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x458ed9; continue; }
  pc = 0x458ecc; continue;
 }
 0x458ecc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x458ed5); f_0045948c(cpu); if cpu.returned_to != 0x458ed5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x458ed9; continue;
 }
 0x458ed9 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x458eb4; continue; }
  pc = 0x458edc; continue;
 }
 0x458edc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00458ee1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x458ee1); }
 let mut pc: u32 = 0x458ee1;
 loop { match pc {
 0x458ee1 => {
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1c8u32, v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x459406; continue; }
  pc = 0x458ef7; continue;
 }
 0x458ef7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x459401; continue; }
  pc = 0x458f05; continue;
 }
 0x458f05 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x458f14; continue; }
  pc = 0x458f12; continue;
 }
 0x458f12 => {
  pc = 0x458f24; continue;
 }
 0x458f14 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d6c3cu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d6c3eu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x458f24; continue;
 }
 0x458f24 => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f29; continue;
 }
 0x458f29 => {
  { cpu.sub8((cpu.ecx as u8), 0x19u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f2e; continue;
 }
 0x458f2e => {
  { cpu.sub8((cpu.ecx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f33; continue;
 }
 0x458f33 => {
  { cpu.sub8((cpu.ecx as u8), 0x18u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f38; continue;
 }
 0x458f38 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x458f69; continue; }
  pc = 0x458f3d; continue;
 }
 0x458f3d => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f42; continue;
 }
 0x458f42 => {
  { cpu.sub8((cpu.ecx as u8), 0x24u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f47; continue;
 }
 0x458f47 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f4c; continue;
 }
 0x458f4c => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f51; continue;
 }
 0x458f51 => {
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f56; continue;
 }
 0x458f56 => {
  { cpu.sub8((cpu.ecx as u8), 0x16u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f5b; continue;
 }
 0x458f5b => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { pc = 0x458f65; continue; }
  pc = 0x458f60; continue;
 }
 0x458f60 => {
  pc = 0x4592a3; continue;
 }
 0x458f65 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x458f69; continue;
 }
 0x458f69 => {
  { cpu.sub8(cpu.r8(0x5d5f20u32), 0x0u8); }
  if cpu.zf { pc = 0x458f7e; continue; }
  pc = 0x458f72; continue;
 }
 0x458f72 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x458f8b; continue; }
  pc = 0x458f78; continue;
 }
 0x458f78 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x458f8b; continue; }
  pc = 0x458f7e; continue;
 }
 0x458f7e => {
  { cpu.sub8(cpu.r8(0x4e1f80u32), 0x0u8); }
  if !cpu.zf { pc = 0x459406; continue; }
  pc = 0x458f8b; continue;
 }
 0x458f8b => {
  { cpu.sub8(cpu.r8(0x56d315u32), 0x0u8); }
  if !cpu.zf { pc = 0x4592a3; continue; }
  pc = 0x458f98; continue;
 }
 0x458f98 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x458fa6; continue; }
  pc = 0x458fa1; continue;
 }
 0x458fa1 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x458fab; continue; }
  pc = 0x458fa6; continue;
 }
 0x458fa6 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x458fab; continue;
 }
 0x458fab => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x4592a3; continue; }
  pc = 0x458fb8; continue;
 }
 0x458fb8 => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x459041; continue; }
  pc = 0x458fc1; continue;
 }
 0x458fc1 => {
  { cpu.sub8((cpu.ecx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x4591c2; continue; }
  pc = 0x458fca; continue;
 }
 0x458fca => {
  { cpu.sub8((cpu.ecx as u8), 0x19u8); }
  if cpu.zf { pc = 0x459175; continue; }
  pc = 0x458fd3; continue;
 }
 0x458fd3 => {
  { cpu.sub8((cpu.ecx as u8), 0x18u8); }
  if cpu.zf { pc = 0x45920f; continue; }
  pc = 0x458fdc; continue;
 }
 0x458fdc => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if cpu.zf { pc = 0x459259; continue; }
  pc = 0x458fe5; continue;
 }
 0x458fe5 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x459128; continue; }
  pc = 0x458fee; continue;
 }
 0x458fee => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x45908e; continue; }
  pc = 0x458ff7; continue;
 }
 0x458ff7 => {
  { cpu.sub8((cpu.ecx as u8), 0x24u8); }
  if cpu.zf { pc = 0x4590db; continue; }
  pc = 0x459000; continue;
 }
 0x459000 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x459019; continue; }
  pc = 0x459005; continue;
 }
 0x459005 => {
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if cpu.zf { pc = 0x459023; continue; }
  pc = 0x45900a; continue;
 }
 0x45900a => {
  { cpu.sub8((cpu.ecx as u8), 0x16u8); }
  if cpu.zf { pc = 0x45902d; continue; }
  pc = 0x45900f; continue;
 }
 0x45900f => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { pc = 0x459037; continue; }
  pc = 0x459014; continue;
 }
 0x459014 => {
  pc = 0x4592a3; continue;
 }
 0x459019 => {
  cpu.push(0x45901e); f_0045943e(cpu); if cpu.returned_to != 0x45901e { return; }
  pc = 0x4592d5; continue;
 }
 0x459023 => {
  cpu.push(0x459028); f_0045943e(cpu); if cpu.returned_to != 0x459028 { return; }
  pc = 0x4592d5; continue;
 }
 0x45902d => {
  cpu.push(0x459032); f_0045943e(cpu); if cpu.returned_to != 0x459032 { return; }
  pc = 0x4592d5; continue;
 }
 0x459037 => {
  cpu.push(0x45903c); f_0045943e(cpu); if cpu.returned_to != 0x45903c { return; }
  pc = 0x4592d5; continue;
 }
 0x459041 => {
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x45906e; continue; }
  pc = 0x459069; continue;
 }
 0x459069 => {
  cpu.push(0x45906e); f_004695ff(cpu); if cpu.returned_to != 0x45906e { return; }
  pc = 0x45906e; continue;
 }
 0x45906e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x45908e => {
  { let v: u8 = 0x1fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x4590bb; continue; }
  pc = 0x4590b6; continue;
 }
 0x4590b6 => {
  cpu.push(0x4590bb); f_004695ff(cpu); if cpu.returned_to != 0x4590bb { return; }
  pc = 0x4590bb; continue;
 }
 0x4590bb => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x4590db => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x459108; continue; }
  pc = 0x459103; continue;
 }
 0x459103 => {
  cpu.push(0x459108); f_004695ff(cpu); if cpu.returned_to != 0x459108 { return; }
  pc = 0x459108; continue;
 }
 0x459108 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x459128 => {
  { let v: u8 = 0x25u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x459155; continue; }
  pc = 0x459150; continue;
 }
 0x459150 => {
  cpu.push(0x459155); f_004695ff(cpu); if cpu.returned_to != 0x459155 { return; }
  pc = 0x459155; continue;
 }
 0x459155 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x459175 => {
  { let v: u8 = 0x1au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x4591a2; continue; }
  pc = 0x45919d; continue;
 }
 0x45919d => {
  cpu.push(0x4591a2); f_004695ff(cpu); if cpu.returned_to != 0x4591a2 { return; }
  pc = 0x4591a2; continue;
 }
 0x4591a2 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x4591c2 => {
  { let v: u8 = 0x1du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x4591ef; continue; }
  pc = 0x4591ea; continue;
 }
 0x4591ea => {
  cpu.push(0x4591ef); f_004695ff(cpu); if cpu.returned_to != 0x4591ef { return; }
  pc = 0x4591ef; continue;
 }
 0x4591ef => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x45920f => {
  { let v: u8 = 0x18u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x45923c; continue; }
  pc = 0x459237; continue;
 }
 0x459237 => {
  cpu.push(0x45923c); f_004695ff(cpu); if cpu.returned_to != 0x45923c { return; }
  pc = 0x45923c; continue;
 }
 0x45923c => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x459259 => {
  { let v: u8 = 0x21u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x459286; continue; }
  pc = 0x459281; continue;
 }
 0x459281 => {
  cpu.push(0x459286); f_004695ff(cpu); if cpu.returned_to != 0x459286 { return; }
  pc = 0x459286; continue;
 }
 0x459286 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4592d0; continue;
 }
 0x4592a3 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x4592aa; continue; }
  pc = 0x4592a8; continue;
 }
 0x4592a8 => {
  { let v: u8 = 0x41u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4592aa; continue;
 }
 0x4592aa => {
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if !cpu.zf { pc = 0x4592b1; continue; }
  pc = 0x4592af; continue;
 }
 0x4592af => {
  { let v: u8 = 0x3fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4592b1; continue;
 }
 0x4592b1 => {
  { cpu.sub8((cpu.ecx as u8), 0x16u8); }
  if !cpu.zf { pc = 0x4592b8; continue; }
  pc = 0x4592b6; continue;
 }
 0x4592b6 => {
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4592b8; continue;
 }
 0x4592b8 => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x4592bf; continue; }
  pc = 0x4592bd; continue;
 }
 0x4592bd => {
  { let v: u8 = 0x42u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4592bf; continue;
 }
 0x4592bf => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x4592c9; continue; }
  pc = 0x4592c4; continue;
 }
 0x4592c4 => {
  cpu.push(0x4592c9); f_00469742(cpu); if cpu.returned_to != 0x4592c9 { return; }
  pc = 0x4592c9; continue;
 }
 0x4592c9 => {
  { cpu.sub8((cpu.ecx as u8), 0x18u8); }
  if !cpu.zf { pc = 0x4592d0; continue; }
  pc = 0x4592ce; continue;
 }
 0x4592ce => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4592d0; continue;
 }
 0x4592d0 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x4592d5; continue; }
  pc = 0x4592d5; continue;
 }
 0x4592d5 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4592db); f_00458b62(cpu); if cpu.returned_to != 0x4592db { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x459401; continue; }
  pc = 0x4592e2; continue;
 }
 0x4592e2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x576c06u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.ecx as u8), 0x1fu8); }
  if !cpu.zf { pc = 0x45930b; continue; }
  pc = 0x459307; continue;
 }
 0x459307 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  pc = 0x45930b; continue;
 }
 0x45930b => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1du32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5b9d0cu32, v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x19u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1fu8); }
  if !cpu.zf { pc = 0x459355; continue; }
  pc = 0x459355; continue;
 }
 0x459355 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x19u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x459365; continue; }
  pc = 0x459363; continue;
 }
 0x459363 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x459365; continue;
 }
 0x459365 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x5ba1c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x459401; continue; }
  pc = 0x459390; continue;
 }
 0x459390 => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x459395; continue;
 }
 0x459395 => {
  { cpu.sub8((cpu.ecx as u8), 0x19u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x45939a; continue;
 }
 0x45939a => {
  { cpu.sub8((cpu.ecx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x45939f; continue;
 }
 0x45939f => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x4593a4; continue;
 }
 0x4593a4 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x4593a9; continue;
 }
 0x4593a9 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x4593ae; continue;
 }
 0x4593ae => {
  { cpu.sub8((cpu.ecx as u8), 0x24u8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x4593b3; continue;
 }
 0x4593b3 => {
  { cpu.sub8((cpu.ecx as u8), 0x1bu8); }
  if cpu.zf { pc = 0x4593e2; continue; }
  pc = 0x4593b8; continue;
 }
 0x4593b8 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x459416; continue; }
  pc = 0x4593bd; continue;
 }
 0x4593bd => {
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if cpu.zf { pc = 0x45941a; continue; }
  pc = 0x4593c2; continue;
 }
 0x4593c2 => {
  { cpu.sub8((cpu.ecx as u8), 0x16u8); }
  if cpu.zf { pc = 0x45941e; continue; }
  pc = 0x4593c7; continue;
 }
 0x4593c7 => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { pc = 0x459422; continue; }
  pc = 0x4593cc; continue;
 }
 0x4593cc => {
  { cpu.sub8((cpu.ecx as u8), 0x41u8); }
  if cpu.zf { pc = 0x459424; continue; }
  pc = 0x4593d1; continue;
 }
 0x4593d1 => {
  { cpu.sub8((cpu.ecx as u8), 0x3fu8); }
  if cpu.zf { pc = 0x459424; continue; }
  pc = 0x4593d6; continue;
 }
 0x4593d6 => {
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x459424; continue; }
  pc = 0x4593db; continue;
 }
 0x4593db => {
  { cpu.sub8((cpu.ecx as u8), 0x42u8); }
  if cpu.zf { pc = 0x459424; continue; }
  pc = 0x4593e0; continue;
 }
 0x4593e0 => {
  pc = 0x459401; continue;
 }
 0x4593e2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x120u16; cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4593f2); f_0041c743(cpu); if cpu.returned_to != 0x4593f2 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x459400); f_0048bcda(cpu); if cpu.returned_to != 0x459400 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x459401; continue;
 }
 0x459401 => {
  { let v: u8 = 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459406 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.edi)), 0x20u8); }
  if !cpu.zf { pc = 0x45940d; continue; }
  pc = 0x45940c; continue;
 }
 0x45940c => {
  {  }
  pc = 0x45940d; continue;
 }
 0x45940d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459416 => {
  { let v: u8 = 0x41u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x459424; continue;
 }
 0x45941a => {
  { let v: u8 = 0x3fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x459424; continue;
 }
 0x45941e => {
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x459424; continue;
 }
 0x459422 => {
  { let v: u8 = 0x42u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x459424; continue;
 }
 0x459424 => {
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x120u16; cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x459434); f_0041c743(cpu); if cpu.returned_to != 0x459434 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x459401; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045943e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45943e); }
 let mut pc: u32 = 0x45943e;
 loop { match pc {
 0x45943e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bdcu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6be4u32); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cc621u32), 0x0u8); }
  if !cpu.zf { pc = 0x459469; continue; }
  pc = 0x459464; continue;
 }
 0x459464 => {
  cpu.push(0x459469); f_004695ff(cpu); if cpu.returned_to != 0x459469 { return; }
  pc = 0x459469; continue;
 }
 0x459469 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1c8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045948c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45948c); }
 let mut pc: u32 = 0x45948c;
 loop { match pc {
 0x45948c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0xafu32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x459513; continue; }
  pc = 0x45949a; continue;
 }
 0x45949a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4594b3; continue; }
  pc = 0x4594a2; continue;
 }
 0x4594a2 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4594b3; continue; }
  pc = 0x4594a7; continue;
 }
 0x4594a7 => {
  { cpu.pushad(); }
  cpu.push(0x4594ad); f_0046a9bc(cpu); if cpu.returned_to != 0x4594ad { return; }
  { cpu.popad(); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x4594b3; continue;
 }
 0x4594b3 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4594bb); f_0046d55d(cpu); if cpu.returned_to != 0x4594bb { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)) & 0xfbu8; cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e1f80u32), 0x0u8); }
  if !cpu.zf { pc = 0x459513; continue; }
  pc = 0x4594cd; continue;
 }
 0x4594cd => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4594d3); f_0045957f(cpu); if cpu.returned_to != 0x4594d3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x459513; continue; }
  pc = 0x4594d6; continue;
 }
 0x4594d6 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5ba1b7u32), 0x0u8); }
  if cpu.zf { pc = 0x4594e7; continue; }
  pc = 0x4594e0; continue;
 }
 0x4594e0 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x4594ef; continue;
 }
 0x4594e7 => {
  cpu.push(0x4594ec); f_0041c743(cpu); if cpu.returned_to != 0x4594ec { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  pc = 0x4594ef; continue;
 }
 0x4594ef => {
  { let v: u32 = 0x5b9fb0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(2, false); }
  { cpu.lods(2, false); }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), (cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  pc = 0x459513; continue;
 }
 0x459513 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045955b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45955b); }
 let mut pc: u32 = 0x45955b;
 loop { match pc {
 0x45955b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x45957c; continue; }
  pc = 0x459563; continue;
 }
 0x459563 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)) & 0xf7u8; cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u8 = cpu.and8(cpu.r8(0x559761u32.wrapping_add(cpu.eax)), 0xfeu8); cpu.w8(0x559761u32.wrapping_add(cpu.eax), v); }
  pc = 0x45957c; continue;
 }
 0x45957c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045957f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45957f); }
 let mut pc: u32 = 0x45957f;
 loop { match pc {
 0x45957f => {
  { let v: u32 = 0x4d737cu32; cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  pc = 0x459589; continue;
 }
 0x459589 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x459596; continue; }
  pc = 0x45958f; continue;
 }
 0x45958f => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x459589; continue; }
  pc = 0x459594; continue;
 }
 0x459594 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459596 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459598(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459598); }
 let mut pc: u32 = 0x459598;
 loop { match pc {
 0x459598 => {
  { let v: u32 = 0x4d737cu32; cpu.edi = v; }
  { let v: u32 = 0x1eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004595aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4595aa); }
 let mut pc: u32 = 0x4595aa;
 loop { match pc {
 0x4595aa => {
  { let v: u32 = 0x4d737cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  pc = 0x4595ba; continue;
 }
 0x4595ba => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x459668; continue; }
  pc = 0x4595c8; continue;
 }
 0x4595c8 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4595e2; continue; }
  pc = 0x4595dd; continue;
 }
 0x4595dd => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x459650; continue;
 }
 0x4595e2 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x45961a; continue; }
  pc = 0x4595eb; continue;
 }
 0x4595eb => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x459650; continue;
 }
 0x45961a => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x459650; continue; }
  pc = 0x45962c; continue;
 }
 0x45962c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x459650; continue; }
  pc = 0x459635; continue;
 }
 0x459635 => {
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x459650; continue;
 }
 0x459650 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x459668; continue; }
  pc = 0x459659; continue;
 }
 0x459659 => {
  { let v: u16 = cpu.inc16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.esi))); cpu.w16(0xfffffff8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0xfffffffau32.wrapping_add(cpu.esi))); cpu.w8(0xfffffffau32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x459668; continue; }
  pc = 0x459662; continue;
 }
 0x459662 => {
  { let v: u16 = 0xffffu16; cpu.w16(0xfffffff8u32.wrapping_add(cpu.esi), v); }
  pc = 0x459668; continue;
 }
 0x459668 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4595ba; continue; }
  pc = 0x459673; continue;
 }
 0x459673 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459674); }
 let mut pc: u32 = 0x459674;
 loop { match pc {
 0x459674 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x45967d); f_00481755(cpu); if cpu.returned_to != 0x45967d { return; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x400u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x4596c5; continue; }
  pc = 0x4596a0; continue;
 }
 0x4596a0 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4596ae; continue;
 }
 0x4596ae => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4596c3; continue; }
  pc = 0x4596b3; continue;
 }
 0x4596b3 => {
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x5092c1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x7fu8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x4596c3; continue;
 }
 0x4596c3 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4596ae; continue; }
  pc = 0x4596c5; continue;
 }
 0x4596c5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004596ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4596ca); }
 let mut pc: u32 = 0x4596ca;
 loop { match pc {
 0x4596ca => {
  { let v: u32 = 0x52300cu32; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x4e1fa7u32) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x459705; continue; }
  pc = 0x4596da; continue;
 }
 0x4596da => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  pc = 0x4596e4; continue;
 }
 0x4596e4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x4596ed; continue; }
  pc = 0x4596eb; continue;
 }
 0x4596eb => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x4596ed; continue;
 }
 0x4596ed => {
  { cpu.sub32(cpu.edx, 0x18u32); }
  if (cpu.sf == cpu.of) { pc = 0x459700; continue; }
  pc = 0x4596f2; continue;
 }
 0x4596f2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x4596fb; continue; }
  pc = 0x4596f9; continue;
 }
 0x4596f9 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x4596fb; continue;
 }
 0x4596fb => {
  { cpu.sub32(cpu.edx, 0x18u32); }
  if (cpu.sf != cpu.of) { pc = 0x459721; continue; }
  pc = 0x459700; continue;
 }
 0x459700 => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4596e4; continue; }
  pc = 0x459705; continue;
 }
 0x459705 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if !cpu.zf { pc = 0x459717; continue; }
  pc = 0x45970c; continue;
 }
 0x45970c => {
  { let v: u8 = 0x2fu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459717 => {
  { let v: u8 = 0x18u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459721; continue;
 }
 0x459721 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459722); }
 let mut pc: u32 = 0x459722;
 loop { match pc {
 0x459722 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45973b; continue; }
  pc = 0x45972b; continue;
 }
 0x45972b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if (cpu.sf != cpu.of) { pc = 0x45973b; continue; }
  pc = 0x459736; continue;
 }
 0x459736 => {
  cpu.push(0x45973b); f_0043ffdb(cpu); if cpu.returned_to != 0x45973b { return; }
  pc = 0x45973b; continue;
 }
 0x45973b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045973c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45973c); }
 let mut pc: u32 = 0x45973c;
 loop { match pc {
 0x45973c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x459747; continue;
 }
 0x459747 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x45974e; continue;
 }
 0x45974e => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x459756; continue;
 }
 0x459756 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x459767; continue;
 }
 0x459767 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x45976d; continue;
 }
 0x45976d => {
  { cpu.sub8(cpu.r8(0x5756cdu32), 0x0u8); }
  if !cpu.zf { pc = 0x45977c; continue; }
  pc = 0x459776; continue;
 }
 0x459776 => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x4597b4; continue; }
  pc = 0x45977c; continue;
 }
 0x45977c => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45978a; continue; }
  pc = 0x459782; continue;
 }
 0x459782 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45978a; continue; }
  pc = 0x459788; continue;
 }
 0x459788 => {
  pc = 0x4597b4; continue;
 }
 0x45978a => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x459792); f_0043fede(cpu); if cpu.returned_to != 0x459792 { return; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x4597b0; continue; }
  pc = 0x45979f; continue;
 }
 0x45979f => {
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4597b0; continue; }
  pc = 0x4597ae; continue;
 }
 0x4597ae => {
  pc = 0x4597b4; continue;
 }
 0x4597b0 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4597b4 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004597be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4597be); }
 let mut pc: u32 = 0x4597be;
 loop { match pc {
 0x4597be => {
  { let v: u8 = cpu.r8(0x86u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4597eb; continue; }
  pc = 0x4597c9; continue;
 }
 0x4597c9 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4597eb; continue; }
  pc = 0x4597cf; continue;
 }
 0x4597cf => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x4597e1; continue; }
  pc = 0x4597d7; continue;
 }
 0x4597d7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4597e1 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4597eb => {
  { let v: u8 = cpu.r8(0x85u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004597f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4597f7); }
 let mut pc: u32 = 0x4597f7;
 loop { match pc {
 0x4597f7 => {
  { cpu.sub16(cpu.r16(0x7fu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x45989d; continue; }
  pc = 0x459804; continue;
 }
 0x459804 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x7fu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x7fu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x165u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x459861; continue; }
  pc = 0x459841; continue;
 }
 0x459841 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x459861; continue; }
  pc = 0x459851; continue;
 }
 0x459851 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x45986a; continue; }
  pc = 0x459861; continue;
 }
 0x459861 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x45989d; continue;
 }
 0x45986a => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x7fu32.wrapping_add(cpu.ebp))); cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x7fu32.wrapping_add(cpu.ebp)), 0x8u16); }
  if !cpu.zf { pc = 0x45989d; continue; }
  pc = 0x459896; continue;
 }
 0x459896 => {
  { let v: u16 = 0x0u16; cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45989d; continue;
 }
 0x45989d => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45990c; continue; }
  pc = 0x4598a6; continue;
 }
 0x4598a6 => {
  cpu.push(0x4598ab); f_0045973c(cpu); if cpu.returned_to != 0x4598ab { return; }
  if cpu.cf { pc = 0x459907; continue; }
  pc = 0x4598ad; continue;
 }
 0x4598ad => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x4598c4; continue; }
  pc = 0x4598b4; continue;
 }
 0x4598b4 => {
  { cpu.sub16(cpu.r16(0x7fu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x4598c4; continue; }
  pc = 0x4598bd; continue;
 }
 0x4598bd => {
  { let v: u16 = 0x1u16; cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4598c4; continue;
 }
 0x4598c4 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xau8); }
  if cpu.zf { pc = 0x4598d1; continue; }
  pc = 0x4598cb; continue;
 }
 0x4598cb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45990c; continue;
 }
 0x4598d1 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45990c; continue; }
  pc = 0x4598d7; continue;
 }
 0x4598d7 => {
  cpu.push(0x4598dc); f_0041c743(cpu); if cpu.returned_to != 0x4598dc { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x19u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x459907; continue; }
  pc = 0x4598ef; continue;
 }
 0x4598ef => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45990c; continue;
 }
 0x459907 => {
  cpu.push(0x45990c); f_0043ffdb(cpu); if cpu.returned_to != 0x45990c { return; }
  pc = 0x45990c; continue;
 }
 0x45990c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045990d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45990d); }
 let mut pc: u32 = 0x45990d;
 loop { match pc {
 0x45990d => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459966; continue; }
  pc = 0x459916; continue;
 }
 0x459916 => {
  cpu.push(0x45991b); f_0045973c(cpu); if cpu.returned_to != 0x45991b { return; }
  if cpu.cf { pc = 0x459961; continue; }
  pc = 0x45991d; continue;
 }
 0x45991d => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45993c; continue; }
  pc = 0x459924; continue;
 }
 0x459924 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x459966; continue; }
  pc = 0x45992f; continue;
 }
 0x45992f => {
  cpu.push(0x459934); f_0041c743(cpu); if cpu.returned_to != 0x459934 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45993c; continue;
 }
 0x45993c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459966; continue; }
  pc = 0x459942; continue;
 }
 0x459942 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x459961; continue; }
  pc = 0x459949; continue;
 }
 0x459949 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459966; continue;
 }
 0x459961 => {
  cpu.push(0x459966); f_0043ffdb(cpu); if cpu.returned_to != 0x459966 { return; }
  pc = 0x459966; continue;
 }
 0x459966 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459967); }
 let mut pc: u32 = 0x459967;
 loop { match pc {
 0x459967 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x4599d2; continue; }
  pc = 0x459970; continue;
 }
 0x459970 => {
  cpu.push(0x459975); f_0045973c(cpu); if cpu.returned_to != 0x459975 { return; }
  if cpu.cf { pc = 0x4599cd; continue; }
  pc = 0x459977; continue;
 }
 0x459977 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x459987; continue; }
  pc = 0x45997e; continue;
 }
 0x45997e => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x459987; continue; }
  pc = 0x459985; continue;
 }
 0x459985 => {
  pc = 0x45999e; continue;
 }
 0x459987 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x4599d2; continue; }
  pc = 0x45998d; continue;
 }
 0x45998d => {
  cpu.push(0x459992); f_0041c743(cpu); if cpu.returned_to != 0x459992 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45999e; continue;
 }
 0x45999e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if (cpu.sf != cpu.of) { pc = 0x4599d2; continue; }
  pc = 0x4599a9; continue;
 }
 0x4599a9 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x4599cd; continue; }
  pc = 0x4599b0; continue;
 }
 0x4599b0 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xau8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x4599d2; continue;
 }
 0x4599cd => {
  cpu.push(0x4599d2); f_0043ffdb(cpu); if cpu.returned_to != 0x4599d2 { return; }
  pc = 0x4599d2; continue;
 }
 0x4599d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004599d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4599d3); }
 let mut pc: u32 = 0x4599d3;
 loop { match pc {
 0x4599d3 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459a3e; continue; }
  pc = 0x4599dc; continue;
 }
 0x4599dc => {
  cpu.push(0x4599e1); f_0045973c(cpu); if cpu.returned_to != 0x4599e1 { return; }
  if cpu.cf { pc = 0x459a39; continue; }
  pc = 0x4599e3; continue;
 }
 0x4599e3 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4599f3; continue; }
  pc = 0x4599ea; continue;
 }
 0x4599ea => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4599f3; continue; }
  pc = 0x4599f1; continue;
 }
 0x4599f1 => {
  pc = 0x459a0a; continue;
 }
 0x4599f3 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459a3e; continue; }
  pc = 0x4599f9; continue;
 }
 0x4599f9 => {
  cpu.push(0x4599fe); f_0041c743(cpu); if cpu.returned_to != 0x4599fe { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x459a0a; continue;
 }
 0x459a0a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if (cpu.sf != cpu.of) { pc = 0x459a3e; continue; }
  pc = 0x459a15; continue;
 }
 0x459a15 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x459a39; continue; }
  pc = 0x459a1c; continue;
 }
 0x459a1c => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xau8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x459a3e; continue;
 }
 0x459a39 => {
  cpu.push(0x459a3e); f_0043ffdb(cpu); if cpu.returned_to != 0x459a3e { return; }
  pc = 0x459a3e; continue;
 }
 0x459a3e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459a3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459a3f); }
 let mut pc: u32 = 0x459a3f;
 loop { match pc {
 0x459a3f => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459b16; continue; }
  pc = 0x459a4c; continue;
 }
 0x459a4c => {
  cpu.push(0x459a51); f_0045973c(cpu); if cpu.returned_to != 0x459a51 { return; }
  if cpu.cf { pc = 0x459b11; continue; }
  pc = 0x459a57; continue;
 }
 0x459a57 => {
  { cpu.and8(cpu.r8(0x7cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x459ad8; continue; }
  pc = 0x459a5e; continue;
 }
 0x459a5e => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x459a98; continue; }
  pc = 0x459a65; continue;
 }
 0x459a65 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x459a6e; continue; }
  pc = 0x459a6c; continue;
 }
 0x459a6c => {
  pc = 0x459a98; continue;
 }
 0x459a6e => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x459a79; continue; }
  pc = 0x459a75; continue;
 }
 0x459a75 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459a79; continue;
 }
 0x459a79 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459b16; continue; }
  pc = 0x459a87; continue;
 }
 0x459a87 => {
  cpu.push(0x459a8c); f_0041c743(cpu); if cpu.returned_to != 0x459a8c { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x459a98; continue;
 }
 0x459a98 => {
  { let v: u8 = 0x2u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x459aa2); f_0041c743(cpu); if cpu.returned_to != 0x459aa2 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x459aad; continue; }
  pc = 0x459aa8; continue;
 }
 0x459aa8 => {
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459aad; continue;
 }
 0x459aad => {
  { let v: u8 = cpu.inc8(cpu.r8(0x81u32.wrapping_add(cpu.ebp))); cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if (cpu.sf != cpu.of) { pc = 0x459ac6; continue; }
  pc = 0x459abe; continue;
 }
 0x459abe => {
  { let v: u8 = 0xffu8; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459ac6; continue;
 }
 0x459ac6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459b16; continue; }
  pc = 0x459acc; continue;
 }
 0x459acc => {
  { let v: u8 = 0x4u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x7cu32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x459b16; continue;
 }
 0x459ad8 => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x459afb; continue; }
  pc = 0x459ae2; continue;
 }
 0x459ae2 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x81u32.wrapping_add(cpu.ebp))); cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x459afb; continue; }
  pc = 0x459af3; continue;
 }
 0x459af3 => {
  { let v: u8 = 0xffu8; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459afb; continue;
 }
 0x459afb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x459b0b; continue; }
  pc = 0x459b06; continue;
 }
 0x459b06 => {
  { let v: u8 = 0x4u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459b0b; continue;
 }
 0x459b0b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459b16; continue; }
  pc = 0x459b11; continue;
 }
 0x459b11 => {
  cpu.push(0x459b16); f_0043ffdb(cpu); if cpu.returned_to != 0x459b16 { return; }
  pc = 0x459b16; continue;
 }
 0x459b16 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459b17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459b17); }
 let mut pc: u32 = 0x459b17;
 loop { match pc {
 0x459b17 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459bd1; continue; }
  pc = 0x459b24; continue;
 }
 0x459b24 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x459b4d; continue; }
  pc = 0x459b44; continue;
 }
 0x459b44 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x459b54; continue; }
  pc = 0x459b4b; continue;
 }
 0x459b4b => {
  pc = 0x459b8e; continue;
 }
 0x459b4d => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xau8); }
  if !cpu.zf { pc = 0x459b8e; continue; }
  pc = 0x459b54; continue;
 }
 0x459b54 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x459b64); f_004679cb(cpu); if cpu.returned_to != 0x459b64 { return; }
  if cpu.cf { pc = 0x459b8d; continue; }
  pc = 0x459b66; continue;
 }
 0x459b66 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x67u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x459b8c; continue; }
  pc = 0x459b7c; continue;
 }
 0x459b7c => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x459b83); f_00401130(cpu); if cpu.returned_to != 0x459b83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x459b8c; continue;
 }
 0x459b8c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x459b8d; continue;
 }
 0x459b8d => {
  { cpu.popad(); }
  pc = 0x459b8e; continue;
 }
 0x459b8e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x459bb3; continue; }
  pc = 0x459baa; continue;
 }
 0x459baa => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xeu8); }
  if !cpu.zf { pc = 0x459bd1; continue; }
  pc = 0x459bb1; continue;
 }
 0x459bb1 => {
  pc = 0x459bba; continue;
 }
 0x459bb3 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x11u8); }
  if !cpu.zf { pc = 0x459bd1; continue; }
  pc = 0x459bba; continue;
 }
 0x459bba => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f80u32, v); }
  cpu.push(0x459bc9); f_00458cb3(cpu); if cpu.returned_to != 0x459bc9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f80u32, v); }
  { cpu.popad(); }
  pc = 0x459bd1; continue;
 }
 0x459bd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459bd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459bd2); }
 let mut pc: u32 = 0x459bd2;
 loop { match pc {
 0x459bd2 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459c0a; continue; }
  pc = 0x459bdb; continue;
 }
 0x459bdb => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x459c05; continue; }
  pc = 0x459be5; continue;
 }
 0x459be5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x459c0a; continue; }
  pc = 0x459bf3; continue;
 }
 0x459bf3 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459c0a; continue;
 }
 0x459c05 => {
  cpu.push(0x459c0a); f_0043ffdb(cpu); if cpu.returned_to != 0x459c0a { return; }
  pc = 0x459c0a; continue;
 }
 0x459c0a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459c0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x459c0b); } r_00459c0b(cpu, 0x459c0b); }
pub fn f_00459c4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x459c4b); } r_00459c0b(cpu, 0x459c4b); }
fn r_00459c0b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x459c0b => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459c81; continue; }
  pc = 0x459c14; continue;
 }
 0x459c14 => {
  cpu.push(0x459c19); f_0045973c(cpu); if cpu.returned_to != 0x459c19 { return; }
  if cpu.cf { pc = 0x459c3e; continue; }
  pc = 0x459c1b; continue;
 }
 0x459c1b => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x459c44; continue; }
  pc = 0x459c22; continue;
 }
 0x459c22 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x459c81; continue; }
  pc = 0x459c2d; continue;
 }
 0x459c2d => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x459c3d); f_00483e27(cpu); if cpu.returned_to != 0x459c3d { return; }
  { cpu.popad(); }
  pc = 0x459c3e; continue;
 }
 0x459c3e => {
  cpu.push(0x459c43); f_0043ffdb(cpu); if cpu.returned_to != 0x459c43 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459c44 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x459c7c; continue; }
  pc = 0x459c53; continue;
 }
 0x459c4b => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x459c7c; continue; }
  pc = 0x459c53; continue;
 }
 0x459c53 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x459c81; continue; }
  pc = 0x459c5a; continue;
 }
 0x459c5a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  cpu.push(0x459c7a); f_0047bbe2(cpu); if cpu.returned_to != 0x459c7a { return; }
  pc = 0x459c81; continue;
 }
 0x459c7c => {
  { let v: u8 = 0x2u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459c81; continue;
 }
 0x459c81 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459c82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459c82); }
 let mut pc: u32 = 0x459c82;
 loop { match pc {
 0x459c82 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459cda; continue; }
  pc = 0x459c8b; continue;
 }
 0x459c8b => {
  cpu.push(0x459c90); f_0045973c(cpu); if cpu.returned_to != 0x459c90 { return; }
  if cpu.cf { pc = 0x459cd5; continue; }
  pc = 0x459c92; continue;
 }
 0x459c92 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x459c9f; continue; }
  pc = 0x459c99; continue;
 }
 0x459c99 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459cda; continue;
 }
 0x459c9f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x459cda; continue; }
  pc = 0x459ca5; continue;
 }
 0x459ca5 => {
  cpu.push(0x459caa); f_0041c743(cpu); if cpu.returned_to != 0x459caa { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x459cd5; continue; }
  pc = 0x459cbd; continue;
 }
 0x459cbd => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459cda; continue;
 }
 0x459cd5 => {
  cpu.push(0x459cda); f_0043ffdb(cpu); if cpu.returned_to != 0x459cda { return; }
  pc = 0x459cda; continue;
 }
 0x459cda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459cdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459cdb); }
 let mut pc: u32 = 0x459cdb;
 loop { match pc {
 0x459cdb => {
  cpu.push(0x459ce0); f_0045973c(cpu); if cpu.returned_to != 0x459ce0 { return; }
  if !cpu.cf { pc = 0x459cf1; continue; }
  pc = 0x459ce2; continue;
 }
 0x459ce2 => {
  cpu.push(0x459ce7); f_00459e8b(cpu); if cpu.returned_to != 0x459ce7 { return; }
  cpu.push(0x459cec); f_00459ec0(cpu); if cpu.returned_to != 0x459cec { return; }
  pc = 0x459e7c; continue;
 }
 0x459cf1 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x459e46; continue; }
  pc = 0x459cfc; continue;
 }
 0x459cfc => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x459e46; continue; }
  pc = 0x459d06; continue;
 }
 0x459d06 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x459d15); f_00420989(cpu); if cpu.returned_to != 0x459d15 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x459d22); f_0045a6d5(cpu); if cpu.returned_to != 0x459d22 { return; }
  if !cpu.cf { pc = 0x459e46; continue; }
  pc = 0x459d28; continue;
 }
 0x459d28 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x459d66; continue; }
  pc = 0x459d2f; continue;
 }
 0x459d2f => {
  cpu.push(0x459d34); f_0045a01a(cpu); if cpu.returned_to != 0x459d34 { return; }
  if cpu.cf { pc = 0x459e8a; continue; }
  pc = 0x459d3a; continue;
 }
 0x459d3a => {
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x7bu32; cpu.ebx = v; }
  cpu.push(0x459d46); f_0045a14e(cpu); if cpu.returned_to != 0x459d46 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  cpu.push(0x459d52); f_0045a14e(cpu); if cpu.returned_to != 0x459d52 { return; }
  cpu.push(0x459d57); f_0045a116(cpu); if cpu.returned_to != 0x459d57 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x459e8a; continue;
 }
 0x459d66 => {
  cpu.push(0x459d6b); f_0045a0d3(cpu); if cpu.returned_to != 0x459d6b { return; }
  if cpu.cf { pc = 0x459e46; continue; }
  pc = 0x459d71; continue;
 }
 0x459d71 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x459e8a; continue; }
  pc = 0x459d7c; continue;
 }
 0x459d7c => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x459e8a; continue; }
  pc = 0x459d87; continue;
 }
 0x459d87 => {
  cpu.push(0x459d8c); f_00459f3e(cpu); if cpu.returned_to != 0x459d8c { return; }
  { cpu.sub8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x459e46; continue; }
  pc = 0x459d9a; continue;
 }
 0x459d9a => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x459e8a; continue; }
  pc = 0x459da7; continue;
 }
 0x459da7 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x459db7; continue; }
  pc = 0x459dae; continue;
 }
 0x459dae => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x459e8a; continue;
 }
 0x459db7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x5ba0efu32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693au32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x459e8a; continue; }
  pc = 0x459ddd; continue;
 }
 0x459ddd => {
  { let v: u8 = cpu.r8(0x84u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x459e46; continue; }
  pc = 0x459df1; continue;
 }
 0x459df1 => {
  { let v: u8 = (cpu.eax as u8) | (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x84u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  cpu.push(0x459e0b); f_0045a14e(cpu); if cpu.returned_to != 0x459e0b { return; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if !cpu.zf { pc = 0x459e30; continue; }
  pc = 0x459e26; continue;
 }
 0x459e26 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x459e30; continue;
 }
 0x459e30 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.esi)) | 0x10u8; cpu.w8(0x7bu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x459e39); f_0041c743(cpu); if cpu.returned_to != 0x459e39 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.esi), v); }
  pc = 0x459e8a; continue;
 }
 0x459e46 => {
  cpu.push(0x459e4b); f_00459ec0(cpu); if cpu.returned_to != 0x459e4b { return; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x459e5e; continue; }
  pc = 0x459e59; continue;
 }
 0x459e59 => {
  cpu.push(0x459e5e); f_0045a796(cpu); if cpu.returned_to != 0x459e5e { return; }
  pc = 0x459e5e; continue;
 }
 0x459e5e => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x459e6b); f_0041c808(cpu); if cpu.returned_to != 0x459e6b { return; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if !cpu.zf { pc = 0x459e7c; continue; }
  pc = 0x459e71; continue;
 }
 0x459e71 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  cpu.push(0x459e7b); f_0045a8ca(cpu); if cpu.returned_to != 0x459e7b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x459e7c; continue;
 }
 0x459e7c => {
  { cpu.pushad(); }
  cpu.push(0x459e82); f_0045af8c(cpu); if cpu.returned_to != 0x459e82 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x459e8a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459e8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459e8b); }
 let mut pc: u32 = 0x459e8b;
 loop { match pc {
 0x459e8b => {
  { cpu.and8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x459ebf; continue; }
  pc = 0x459e95; continue;
 }
 0x459e95 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x459eb8; continue; }
  pc = 0x459eae; continue;
 }
 0x459eae => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  cpu.push(0x459eb8); f_00459fea(cpu); if cpu.returned_to != 0x459eb8 { return; }
  pc = 0x459eb8; continue;
 }
 0x459eb8 => {
  { cpu.pushad(); }
  cpu.push(0x459ebe); f_004681b7(cpu); if cpu.returned_to != 0x459ebe { return; }
  { cpu.popad(); }
  pc = 0x459ebf; continue;
 }
 0x459ebf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459ec0); }
 let mut pc: u32 = 0x459ec0;
 loop { match pc {
 0x459ec0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x35u32; cpu.ebx = v; }
  cpu.push(0x459ecd); f_0045a14e(cpu); if cpu.returned_to != 0x459ecd { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  cpu.push(0x459edc); f_0045a18f(cpu); if cpu.returned_to != 0x459edc { return; }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  cpu.push(0x459ee8); f_0045a14e(cpu); if cpu.returned_to != 0x459ee8 { return; }
  cpu.push(0x459eed); f_0041c743(cpu); if cpu.returned_to != 0x459eed { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  cpu.push(0x459ef9); f_0045a14e(cpu); if cpu.returned_to != 0x459ef9 { return; }
  cpu.push(0x459efe); f_0041c743(cpu); if cpu.returned_to != 0x459efe { return; }
  { let v: u8 = (cpu.eax as u8) & 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x7au32; cpu.ebx = v; }
  cpu.push(0x459f0a); f_0045a14e(cpu); if cpu.returned_to != 0x459f0a { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x39u32; cpu.ebx = v; }
  cpu.push(0x459f16); f_0045a14e(cpu); if cpu.returned_to != 0x459f16 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  cpu.push(0x459f22); f_0045a14e(cpu); if cpu.returned_to != 0x459f22 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  cpu.push(0x459f2e); f_0045a14e(cpu); if cpu.returned_to != 0x459f2e { return; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x36u32; cpu.ebx = v; }
  cpu.push(0x459f3c); f_0045a18f(cpu); if cpu.returned_to != 0x459f3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459f3e); }
 let mut pc: u32 = 0x459f3e;
 loop { match pc {
 0x459f3e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x459fe5; continue; }
  pc = 0x459f50; continue;
 }
 0x459f50 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = 0x5ba0d7u32; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x9c2u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x459f90; continue; }
  pc = 0x459f8a; continue;
 }
 0x459f8a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x459f95; continue;
 }
 0x459f90 => {
  cpu.push(0x459f95); f_00459fea(cpu); if cpu.returned_to != 0x459f95 { return; }
  pc = 0x459f95; continue;
 }
 0x459f95 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x459fc4; continue; }
  pc = 0x459fa4; continue;
 }
 0x459fa4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x459fc4; continue; }
  pc = 0x459fb4; continue;
 }
 0x459fb4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x459fcd; continue; }
  pc = 0x459fc4; continue;
 }
 0x459fc4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x459fe5; continue;
 }
 0x459fcd => {
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x459fe5; continue;
 }
 0x459fe5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00459fea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x459fea); }
 let mut pc: u32 = 0x459fea;
 loop { match pc {
 0x459fea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r8(0x84u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = ((cpu.eax >> 8) as u8) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x8u16); }
  if !cpu.zf { pc = 0x45a017; continue; }
  pc = 0x45a00f; continue;
 }
 0x45a00f => {
  { let v: u8 = cpu.or8(cpu.r8(0x84u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a017; continue;
 }
 0x45a017 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a01a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a01a); }
 let mut pc: u32 = 0x45a01a;
 loop { match pc {
 0x45a01a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x45a052; continue; }
  pc = 0x45a023; continue;
 }
 0x45a023 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45a02d; continue;
 }
 0x45a02d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if cpu.zf { pc = 0x45a05a; continue; }
  pc = 0x45a039; continue;
 }
 0x45a039 => {
  { cpu.sub32(cpu.esi, 0x0u32); }
  if !cpu.zf { pc = 0x45a040; continue; }
  pc = 0x45a03e; continue;
 }
 0x45a03e => {
  pc = 0x45a056; continue;
 }
 0x45a040 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x45a056; continue; }
  pc = 0x45a048; continue;
 }
 0x45a048 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x45a052; continue; }
  pc = 0x45a04e; continue;
 }
 0x45a04e => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x45a02d; continue; }
  pc = 0x45a052; continue;
 }
 0x45a052 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a056 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a05a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x45a063; continue;
 }
 0x45a063 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x45a06c); f_0045a083(cpu); if cpu.returned_to != 0x45a06c { return; }
  if cpu.cf { pc = 0x45a07a; continue; }
  pc = 0x45a06e; continue;
 }
 0x45a06e => {
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if cpu.zf { pc = 0x45a07a; continue; }
  pc = 0x45a076; continue;
 }
 0x45a076 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x45a063; continue; }
  pc = 0x45a07a; continue;
 }
 0x45a07a => {
  cpu.push(0x45a07f); f_0045a083(cpu); if cpu.returned_to != 0x45a07f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a083(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a083); }
 let mut pc: u32 = 0x45a083;
 loop { match pc {
 0x45a083 => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4d6bf4u32), cpu.esi); }
  if cpu.zf { pc = 0x45a0d1; continue; }
  pc = 0x45a098; continue;
 }
 0x45a098 => {
  { let v: u32 = 0x35u32; cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a0b3); f_0041c743(cpu); if cpu.returned_to != 0x45a0b3 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xffu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a0d1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a0d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a0d3); }
 let mut pc: u32 = 0x45a0d3;
 loop { match pc {
 0x45a0d3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if !cpu.zf { pc = 0x45a10e; continue; }
  pc = 0x45a0dc; continue;
 }
 0x45a0dc => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x45a0e6; continue;
 }
 0x45a0e6 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if !cpu.zf { pc = 0x45a0fc; continue; }
  pc = 0x45a0f2; continue;
 }
 0x45a0f2 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a0fc; continue;
 }
 0x45a0fc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x45a112; continue; }
  pc = 0x45a104; continue;
 }
 0x45a104 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x19u8); }
  if !cpu.zf { pc = 0x45a10e; continue; }
  pc = 0x45a10a; continue;
 }
 0x45a10a => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x45a0e6; continue; }
  pc = 0x45a10e; continue;
 }
 0x45a10e => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a112 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a116(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a116); }
 let mut pc: u32 = 0x45a116;
 loop { match pc {
 0x45a116 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x5ba0efu32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x45a130; continue;
 }
 0x45a130 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x45a14b; continue; }
  pc = 0x45a13c; continue;
 }
 0x45a13c => {
  { let v: u32 = cpu.r16(0x7fu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.poll(); pc = 0x45a130; continue;
 }
 0x45a14b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a14e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a14e); }
 let mut pc: u32 = 0x45a14e;
 loop { match pc {
 0x45a14e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x45a15f; continue;
 }
 0x45a15f => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x45a18a; continue; }
  pc = 0x45a164; continue;
 }
 0x45a164 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if !cpu.zf { pc = 0x45a17a; continue; }
  pc = 0x45a170; continue;
 }
 0x45a170 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a17a; continue;
 }
 0x45a17a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x45a18a; continue; }
  pc = 0x45a182; continue;
 }
 0x45a182 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x45a15f; continue;
 }
 0x45a18a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a18f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a18f); }
 let mut pc: u32 = 0x45a18f;
 loop { match pc {
 0x45a18f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x45a1a1; continue;
 }
 0x45a1a1 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x45a1cd; continue; }
  pc = 0x45a1a6; continue;
 }
 0x45a1a6 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x87654321u32); }
  if !cpu.zf { pc = 0x45a1bc; continue; }
  pc = 0x45a1b2; continue;
 }
 0x45a1b2 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a1bc; continue;
 }
 0x45a1bc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x45a1cd; continue; }
  pc = 0x45a1c4; continue;
 }
 0x45a1c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x45a1a1; continue;
 }
 0x45a1cd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a213); }
 let mut pc: u32 = 0x45a213;
 loop { match pc {
 0x45a213 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a2f6; continue; }
  pc = 0x45a220; continue;
 }
 0x45a220 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0xau8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x45a27c; continue; }
  pc = 0x45a227; continue;
 }
 0x45a227 => {
  { let v: u8 = cpu.r8(0x7fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x45a231; continue; }
  pc = 0x45a22f; continue;
 }
 0x45a22f => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45a231; continue;
 }
 0x45a231 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a242); f_0043fede(cpu); if cpu.returned_to != 0x45a242 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x45a27c; continue; }
  pc = 0x45a24c; continue;
 }
 0x45a24c => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x45a27c; continue; }
  pc = 0x45a252; continue;
 }
 0x45a252 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x45a276); f_00481755(cpu); if cpu.returned_to != 0x45a276 { return; }
  cpu.push(0x45a27b); f_0045add0(cpu); if cpu.returned_to != 0x45a27b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a27c => {
  cpu.push(0x45a281); f_0045973c(cpu); if cpu.returned_to != 0x45a281 { return; }
  if cpu.cf { pc = 0x45a2f1; continue; }
  pc = 0x45a283; continue;
 }
 0x45a283 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a290; continue; }
  pc = 0x45a28a; continue;
 }
 0x45a28a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a2f6; continue;
 }
 0x45a290 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x45a2b8; continue; }
  pc = 0x45a2af; continue;
 }
 0x45a2af => {
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x45a2b8; continue; }
  pc = 0x45a2b6; continue;
 }
 0x45a2b6 => {
  pc = 0x45a2bd; continue;
 }
 0x45a2b8 => {
  { let v: u8 = 0x14u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a2bd; continue;
 }
 0x45a2bd => {
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45a2ca; continue; }
  pc = 0x45a2c4; continue;
 }
 0x45a2c4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45a2cc; continue;
 }
 0x45a2ca => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45a2cc; continue;
 }
 0x45a2cc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45a2f6; continue; }
  pc = 0x45a2d7; continue;
 }
 0x45a2d7 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a2f1; continue; }
  pc = 0x45a2de; continue;
 }
 0x45a2de => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a2f6; continue;
 }
 0x45a2f1 => {
  cpu.push(0x45a2f6); f_0043ffdb(cpu); if cpu.returned_to != 0x45a2f6 { return; }
  pc = 0x45a2f6; continue;
 }
 0x45a2f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a2f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a2f7); }
 let mut pc: u32 = 0x45a2f7;
 loop { match pc {
 0x45a2f7 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a33b; continue; }
  pc = 0x45a300; continue;
 }
 0x45a300 => {
  cpu.push(0x45a305); f_0045973c(cpu); if cpu.returned_to != 0x45a305 { return; }
  if cpu.cf { pc = 0x45a336; continue; }
  pc = 0x45a307; continue;
 }
 0x45a307 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x45a33b; continue; }
  pc = 0x45a317; continue;
 }
 0x45a317 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a336; continue; }
  pc = 0x45a31e; continue;
 }
 0x45a31e => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a33b; continue;
 }
 0x45a336 => {
  cpu.push(0x45a33b); f_0043ffdb(cpu); if cpu.returned_to != 0x45a33b { return; }
  pc = 0x45a33b; continue;
 }
 0x45a33b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a33c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a33c); }
 let mut pc: u32 = 0x45a33c;
 loop { match pc {
 0x45a33c => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a3a3; continue; }
  pc = 0x45a345; continue;
 }
 0x45a345 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5756cdu32, v); }
  cpu.push(0x45a351); f_0045973c(cpu); if cpu.returned_to != 0x45a351 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5756cdu32, v); }
  if cpu.cf { pc = 0x45a384; continue; }
  pc = 0x45a35a; continue;
 }
 0x45a35a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45a3a3; continue; }
  pc = 0x45a365; continue;
 }
 0x45a365 => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a384; continue; }
  pc = 0x45a36c; continue;
 }
 0x45a36c => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a3a3; continue;
 }
 0x45a384 => {
  cpu.push(0x45a389); f_0043ffdb(cpu); if cpu.returned_to != 0x45a389 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x45a3a3; continue; }
  pc = 0x45a390; continue;
 }
 0x45a390 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a3d9du32, v); }
  cpu.push(0x45a39c); f_0043fced(cpu); if cpu.returned_to != 0x45a39c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a3d9du32, v); }
  pc = 0x45a3a3; continue;
 }
 0x45a3a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a3a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a3a4); }
 let mut pc: u32 = 0x45a3a4;
 loop { match pc {
 0x45a3a4 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a3e3; continue; }
  pc = 0x45a3ad; continue;
 }
 0x45a3ad => {
  cpu.push(0x45a3b2); f_0045973c(cpu); if cpu.returned_to != 0x45a3b2 { return; }
  if cpu.cf { pc = 0x45a3de; continue; }
  pc = 0x45a3b4; continue;
 }
 0x45a3b4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45a3e3; continue; }
  pc = 0x45a3bf; continue;
 }
 0x45a3bf => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a3de; continue; }
  pc = 0x45a3c6; continue;
 }
 0x45a3c6 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a3e3; continue;
 }
 0x45a3de => {
  cpu.push(0x45a3e3); f_0043ffdb(cpu); if cpu.returned_to != 0x45a3e3 { return; }
  pc = 0x45a3e3; continue;
 }
 0x45a3e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a3e4); }
 let mut pc: u32 = 0x45a3e4;
 loop { match pc {
 0x45a3e4 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a423; continue; }
  pc = 0x45a3ed; continue;
 }
 0x45a3ed => {
  cpu.push(0x45a3f2); f_0045973c(cpu); if cpu.returned_to != 0x45a3f2 { return; }
  if cpu.cf { pc = 0x45a41e; continue; }
  pc = 0x45a3f4; continue;
 }
 0x45a3f4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45a423; continue; }
  pc = 0x45a3ff; continue;
 }
 0x45a3ff => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a41e; continue; }
  pc = 0x45a406; continue;
 }
 0x45a406 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a423; continue;
 }
 0x45a41e => {
  cpu.push(0x45a423); f_0043ffdb(cpu); if cpu.returned_to != 0x45a423 { return; }
  pc = 0x45a423; continue;
 }
 0x45a423 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a424); }
 let mut pc: u32 = 0x45a424;
 loop { match pc {
 0x45a424 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a498; continue; }
  pc = 0x45a42d; continue;
 }
 0x45a42d => {
  cpu.push(0x45a432); f_0045973c(cpu); if cpu.returned_to != 0x45a432 { return; }
  if cpu.cf { pc = 0x45a479; continue; }
  pc = 0x45a434; continue;
 }
 0x45a434 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x5u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x45a498; continue; }
  pc = 0x45a44f; continue;
 }
 0x45a44f => {
  { cpu.and8(cpu.r8(0x7bu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x45a479; continue; }
  pc = 0x45a456; continue;
 }
 0x45a456 => {
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) & 0xf8u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) | (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a498; continue;
 }
 0x45a479 => {
  cpu.push(0x45a47e); f_0043ffdb(cpu); if cpu.returned_to != 0x45a47e { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x45a498; continue; }
  pc = 0x45a485; continue;
 }
 0x45a485 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a3d9du32, v); }
  cpu.push(0x45a491); f_0043fced(cpu); if cpu.returned_to != 0x45a491 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a3d9du32, v); }
  pc = 0x45a498; continue;
 }
 0x45a498 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a499(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a499); }
 let mut pc: u32 = 0x45a499;
 loop { match pc {
 0x45a499 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a4c8; continue; }
  pc = 0x45a4a2; continue;
 }
 0x45a4a2 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x45a4b6; continue; }
  pc = 0x45a4a9; continue;
 }
 0x45a4a9 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a4b6; continue; }
  pc = 0x45a4b0; continue;
 }
 0x45a4b0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a4c8; continue;
 }
 0x45a4b6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x45a4c3; continue; }
  pc = 0x45a4c1; continue;
 }
 0x45a4c1 => {
  if !cpu.zf { pc = 0x45a4c8; continue; }
  pc = 0x45a4c3; continue;
 }
 0x45a4c3 => {
  cpu.push(0x45a4c8); f_0043ffdb(cpu); if cpu.returned_to != 0x45a4c8 { return; }
  pc = 0x45a4c8; continue;
 }
 0x45a4c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a4c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a4c9); }
 let mut pc: u32 = 0x45a4c9;
 loop { match pc {
 0x45a4c9 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a4f8; continue; }
  pc = 0x45a4d2; continue;
 }
 0x45a4d2 => {
  cpu.push(0x45a4d7); f_0045973c(cpu); if cpu.returned_to != 0x45a4d7 { return; }
  if cpu.cf { pc = 0x45a4f3; continue; }
  pc = 0x45a4d9; continue;
 }
 0x45a4d9 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x45a4ed; continue; }
  pc = 0x45a4e0; continue;
 }
 0x45a4e0 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a4f3; continue; }
  pc = 0x45a4e7; continue;
 }
 0x45a4e7 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a4f8; continue;
 }
 0x45a4ed => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a4f8; continue;
 }
 0x45a4f3 => {
  cpu.push(0x45a4f8); f_0043ffdb(cpu); if cpu.returned_to != 0x45a4f8 { return; }
  pc = 0x45a4f8; continue;
 }
 0x45a4f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a4f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a4f9); }
 let mut pc: u32 = 0x45a4f9;
 loop { match pc {
 0x45a4f9 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x45a522; continue; }
  pc = 0x45a500; continue;
 }
 0x45a500 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a52f; continue; }
  pc = 0x45a507; continue;
 }
 0x45a507 => {
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x45a51c; continue; }
  pc = 0x45a513; continue;
 }
 0x45a513 => {
  { let v: u16 = ((cpu.eax >> 8) as u8) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a51c; continue;
 }
 0x45a51c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a534; continue;
 }
 0x45a522 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.add16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x3u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a534; continue;
 }
 0x45a52f => {
  cpu.push(0x45a534); f_0043ffdb(cpu); if cpu.returned_to != 0x45a534 { return; }
  pc = 0x45a534; continue;
 }
 0x45a534 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a535(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a535); }
 let mut pc: u32 = 0x45a535;
 loop { match pc {
 0x45a535 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a570; continue; }
  pc = 0x45a53e; continue;
 }
 0x45a53e => {
  cpu.push(0x45a543); f_0045973c(cpu); if cpu.returned_to != 0x45a543 { return; }
  if cpu.cf { pc = 0x45a571; continue; }
  pc = 0x45a545; continue;
 }
 0x45a545 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x45a570; continue; }
  pc = 0x45a550; continue;
 }
 0x45a550 => {
  { let v: u8 = 0x26u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a564); f_0041c743(cpu); if cpu.returned_to != 0x45a564 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a570; continue;
 }
 0x45a570 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a571 => {
  cpu.push(0x45a576); f_0043ffdb(cpu); if cpu.returned_to != 0x45a576 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a577(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a577); }
 let mut pc: u32 = 0x45a577;
 loop { match pc {
 0x45a577 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a5a4; continue; }
  pc = 0x45a580; continue;
 }
 0x45a580 => {
  cpu.push(0x45a585); f_0045973c(cpu); if cpu.returned_to != 0x45a585 { return; }
  if cpu.cf { pc = 0x45a59f; continue; }
  pc = 0x45a587; continue;
 }
 0x45a587 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) & 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a595); f_0041c743(cpu); if cpu.returned_to != 0x45a595 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x45a5a4; continue; }
  pc = 0x45a59f; continue;
 }
 0x45a59f => {
  cpu.push(0x45a5a4); f_0043ffdb(cpu); if cpu.returned_to != 0x45a5a4 { return; }
  pc = 0x45a5a4; continue;
 }
 0x45a5a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a5a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a5a5); }
 let mut pc: u32 = 0x45a5a5;
 loop { match pc {
 0x45a5a5 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a5c8; continue; }
  pc = 0x45a5ae; continue;
 }
 0x45a5ae => {
  cpu.push(0x45a5b3); f_0045973c(cpu); if cpu.returned_to != 0x45a5b3 { return; }
  if cpu.cf { pc = 0x45a5c3; continue; }
  pc = 0x45a5b5; continue;
 }
 0x45a5b5 => {
  { let v: u8 = cpu.r8(0x7fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45a5c8; continue; }
  pc = 0x45a5c3; continue;
 }
 0x45a5c3 => {
  cpu.push(0x45a5c8); f_0043ffdb(cpu); if cpu.returned_to != 0x45a5c8 { return; }
  pc = 0x45a5c8; continue;
 }
 0x45a5c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a5c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a5c9); }
 let mut pc: u32 = 0x45a5c9;
 loop { match pc {
 0x45a5c9 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a606; continue; }
  pc = 0x45a5d2; continue;
 }
 0x45a5d2 => {
  cpu.push(0x45a5d7); f_0045973c(cpu); if cpu.returned_to != 0x45a5d7 { return; }
  if cpu.cf { pc = 0x45a601; continue; }
  pc = 0x45a5d9; continue;
 }
 0x45a5d9 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x45a5f4; continue; }
  pc = 0x45a5ee; continue;
 }
 0x45a5ee => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a606; continue;
 }
 0x45a5f4 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a601; continue; }
  pc = 0x45a5fb; continue;
 }
 0x45a5fb => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a606; continue;
 }
 0x45a601 => {
  cpu.push(0x45a606); f_0043ffdb(cpu); if cpu.returned_to != 0x45a606 { return; }
  pc = 0x45a606; continue;
 }
 0x45a606 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a653(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a653); }
 let mut pc: u32 = 0x45a653;
 loop { match pc {
 0x45a653 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45a69f; continue; }
  pc = 0x45a65c; continue;
 }
 0x45a65c => {
  cpu.push(0x45a661); f_0045973c(cpu); if cpu.returned_to != 0x45a661 { return; }
  if cpu.cf { pc = 0x45a69a; continue; }
  pc = 0x45a663; continue;
 }
 0x45a663 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x45a684; continue; }
  pc = 0x45a66a; continue;
 }
 0x45a66a => {
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x45a677; continue; }
  pc = 0x45a671; continue;
 }
 0x45a671 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a69f; continue;
 }
 0x45a677 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x45a69a; continue; }
  pc = 0x45a682; continue;
 }
 0x45a682 => {
  pc = 0x45a69f; continue;
 }
 0x45a684 => {
  { cpu.sub8(cpu.r8(0x7du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a691; continue; }
  pc = 0x45a68b; continue;
 }
 0x45a68b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x7du32.wrapping_add(cpu.ebp))); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a69f; continue;
 }
 0x45a691 => {
  { let v: u16 = 0x0u16; cpu.w16(0x7fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a69f; continue;
 }
 0x45a69a => {
  cpu.push(0x45a69f); f_0043ffdb(cpu); if cpu.returned_to != 0x45a69f { return; }
  pc = 0x45a69f; continue;
 }
 0x45a69f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a6a0); }
 let mut pc: u32 = 0x45a6a0;
 loop { match pc {
 0x45a6a0 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45a6b7); f_00420989(cpu); if cpu.returned_to != 0x45a6b7 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x45a6d3; continue; }
  pc = 0x45a6c5; continue;
 }
 0x45a6c5 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x45a6d3; continue; }
  pc = 0x45a6c9; continue;
 }
 0x45a6c9 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x45a6d3; continue; }
  pc = 0x45a6cd; continue;
 }
 0x45a6cd => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x45a6d3; continue; }
  pc = 0x45a6d1; continue;
 }
 0x45a6d1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a6d3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a6d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a6d5); }
 let mut pc: u32 = 0x45a6d5;
 loop { match pc {
 0x45a6d5 => {
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x45a6e9; continue; }
  pc = 0x45a6db; continue;
 }
 0x45a6db => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x45a6e9; continue; }
  pc = 0x45a6df; continue;
 }
 0x45a6df => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x45a6e9; continue; }
  pc = 0x45a6e3; continue;
 }
 0x45a6e3 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x45a6e9; continue; }
  pc = 0x45a6e7; continue;
 }
 0x45a6e7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a6e9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a6eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a6eb); }
 let mut pc: u32 = 0x45a6eb;
 loop { match pc {
 0x45a6eb => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x45a72b; continue; }
  pc = 0x45a6f3; continue;
 }
 0x45a6f3 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc04u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x45a704; continue;
 }
 0x45a704 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x45a721; continue; }
  pc = 0x45a709; continue;
 }
 0x45a709 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x45a729; continue; }
  pc = 0x45a71b; continue;
 }
 0x45a71b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x45a729; continue; }
  pc = 0x45a721; continue;
 }
 0x45a721 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x45a704; continue; }
  pc = 0x45a726; continue;
 }
 0x45a726 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a729 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x45a72b; continue;
 }
 0x45a72b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a72e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a72e); }
 let mut pc: u32 = 0x45a72e;
 loop { match pc {
 0x45a72e => {
  cpu.push(0x45a733); f_0045a6a0(cpu); if cpu.returned_to != 0x45a733 { return; }
  if cpu.cf { pc = 0x45a794; continue; }
  pc = 0x45a735; continue;
 }
 0x45a735 => {
  { cpu.pushad(); }
  cpu.push(0x45a73b); f_0045a6eb(cpu); if cpu.returned_to != 0x45a73b { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a742; continue;
 }
 0x45a742 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a749; continue;
 }
 0x45a749 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a750; continue;
 }
 0x45a750 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a757; continue;
 }
 0x45a757 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x11u8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a75e; continue;
 }
 0x45a75e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x45a768; continue; }
  pc = 0x45a765; continue;
 }
 0x45a765 => {
  { cpu.popad(); }
  pc = 0x45a794; continue;
 }
 0x45a768 => {
  { let v: u8 = 0x21u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a794 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a796(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a796); }
 let mut pc: u32 = 0x45a796;
 loop { match pc {
 0x45a796 => {
  cpu.push(0x45a79b); f_0045a6a0(cpu); if cpu.returned_to != 0x45a79b { return; }
  if cpu.cf { pc = 0x45a7ac; continue; }
  pc = 0x45a79d; continue;
 }
 0x45a79d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45a7ae; continue; }
  pc = 0x45a7a4; continue;
 }
 0x45a7a4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45a7ae; continue; }
  pc = 0x45a7ab; continue;
 }
 0x45a7ab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a7ac => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a7ae => {
  cpu.push(0x45a7b3); f_0041c743(cpu); if cpu.returned_to != 0x45a7b3 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x45a81f; continue; }
  pc = 0x45a7b9; continue;
 }
 0x45a7b9 => {
  cpu.push(0x45a7be); f_0045a6a0(cpu); if cpu.returned_to != 0x45a7be { return; }
  if cpu.cf { pc = 0x45a7ea; continue; }
  pc = 0x45a7c0; continue;
 }
 0x45a7c0 => {
  { let v: u8 = 0x21u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a7ea => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a81f => {
  cpu.push(0x45a824); f_0045a6a0(cpu); if cpu.returned_to != 0x45a824 { return; }
  if cpu.cf { pc = 0x45a862; continue; }
  pc = 0x45a826; continue;
 }
 0x45a826 => {
  { let v: u8 = 0x24u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a862 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a7ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a7ec); }
 let mut pc: u32 = 0x45a7ec;
 loop { match pc {
 0x45a7ec => {
  cpu.push(0x45a7f1); f_0045a6a0(cpu); if cpu.returned_to != 0x45a7f1 { return; }
  if cpu.cf { pc = 0x45a81d; continue; }
  pc = 0x45a7f3; continue;
 }
 0x45a7f3 => {
  { let v: u8 = 0x2du8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a81d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a864); }
 let mut pc: u32 = 0x45a864;
 loop { match pc {
 0x45a864 => {
  cpu.push(0x45a869); f_0045a6a0(cpu); if cpu.returned_to != 0x45a869 { return; }
  if cpu.cf { pc = 0x45a895; continue; }
  pc = 0x45a86b; continue;
 }
 0x45a86b => {
  { let v: u8 = 0x23u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a895 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a897(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a897); }
 let mut pc: u32 = 0x45a897;
 loop { match pc {
 0x45a897 => {
  cpu.push(0x45a89c); f_0045a6a0(cpu); if cpu.returned_to != 0x45a89c { return; }
  if cpu.cf { pc = 0x45a8c8; continue; }
  pc = 0x45a89e; continue;
 }
 0x45a89e => {
  { let v: u8 = 0x27u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a8c8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a8ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a8ca); }
 let mut pc: u32 = 0x45a8ca;
 loop { match pc {
 0x45a8ca => {
  cpu.push(0x45a8cf); f_0045a6a0(cpu); if cpu.returned_to != 0x45a8cf { return; }
  if cpu.cf { pc = 0x45a8fb; continue; }
  pc = 0x45a8d1; continue;
 }
 0x45a8d1 => {
  { let v: u8 = 0x20u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a8fb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a8fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a8fd); }
 let mut pc: u32 = 0x45a8fd;
 loop { match pc {
 0x45a8fd => {
  cpu.push(0x45a902); f_0045a6a0(cpu); if cpu.returned_to != 0x45a902 { return; }
  if cpu.cf { pc = 0x45a94b; continue; }
  pc = 0x45a904; continue;
 }
 0x45a904 => {
  { let v: u8 = 0x1au8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45a930); f_0041c808(cpu); if cpu.returned_to != 0x45a930 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a93e); f_0041c743(cpu); if cpu.returned_to != 0x45a93e { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a94b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a94d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45a94d); }
 let mut pc: u32 = 0x45a94d;
 loop { match pc {
 0x45a94d => {
  cpu.push(0x45a952); f_0045a6a0(cpu); if cpu.returned_to != 0x45a952 { return; }
  if cpu.cf { pc = 0x45a98f; continue; }
  pc = 0x45a954; continue;
 }
 0x45a954 => {
  { let v: u8 = 0x1bu8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45a980); f_0041c808(cpu); if cpu.returned_to != 0x45a980 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a98f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045a991(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45a991); } r_0045a991(cpu, 0x45a991); }
pub fn f_0045a9ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45a9ed); } r_0045a991(cpu, 0x45a9ed); }
fn r_0045a991(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45a991 => {
  cpu.push(0x45a996); f_0045a6a0(cpu); if cpu.returned_to != 0x45a996 { return; }
  if cpu.cf { pc = 0x45a9eb; continue; }
  pc = 0x45a998; continue;
 }
 0x45a998 => {
  cpu.push(0x45a99d); f_0041c743(cpu); if cpu.returned_to != 0x45a99d { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x45a9ed; continue; }
  pc = 0x45a9a3; continue;
 }
 0x45a9a3 => {
  { let v: u8 = 0x1cu8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45a9ab; continue;
 }
 0x45a9ab => {
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45a9cf); f_0041c808(cpu); if cpu.returned_to != 0x45a9cf { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45a9dd); f_0041c743(cpu); if cpu.returned_to != 0x45a9dd { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a9eb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45a9ed => {
  cpu.push(0x45a9f2); f_0045a6a0(cpu); if cpu.returned_to != 0x45a9f2 { return; }
  if cpu.cf { pc = 0x45a9fe; continue; }
  pc = 0x45a9f4; continue;
 }
 0x45a9f4 => {
  { let v: u8 = 0x1eu8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x45a9ab; continue;
 }
 0x45a9fe => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045aa00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45aa00); }
 let mut pc: u32 = 0x45aa00;
 loop { match pc {
 0x45aa00 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45aa13; continue; }
  pc = 0x45aa08; continue;
 }
 0x45aa08 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x45aaba; continue; }
  pc = 0x45aa13; continue;
 }
 0x45aa13 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x45aa3a; continue; }
  pc = 0x45aa2f; continue;
 }
 0x45aa2f => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x45aa3a; continue; }
  pc = 0x45aa34; continue;
 }
 0x45aa34 => {
  pc = 0x45aaba; continue;
 }
 0x45aa3a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45aa51); f_00420989(cpu); if cpu.returned_to != 0x45aa51 { return; }
  cpu.push(0x45aa56); f_0045aabd(cpu); if cpu.returned_to != 0x45aa56 { return; }
  if cpu.cf { pc = 0x45aaba; continue; }
  pc = 0x45aa58; continue;
 }
 0x45aa58 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.push(0x45aa5e); f_0045aabd(cpu); if cpu.returned_to != 0x45aa5e { return; }
  if cpu.cf { pc = 0x45aaba; continue; }
  pc = 0x45aa60; continue;
 }
 0x45aa60 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x45aa67); f_0045aabd(cpu); if cpu.returned_to != 0x45aa67 { return; }
  if cpu.cf { pc = 0x45aaba; continue; }
  pc = 0x45aa69; continue;
 }
 0x45aa69 => {
  { let v: u8 = 0x43u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45aa9d); f_0041c808(cpu); if cpu.returned_to != 0x45aa9d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45aaab); f_0041c743(cpu); if cpu.returned_to != 0x45aaab { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45aaba => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045aabd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45aabd); }
 let mut pc: u32 = 0x45aabd;
 loop { match pc {
 0x45aabd => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45aac8); f_0045a6d5(cpu); if cpu.returned_to != 0x45aac8 { return; }
  if cpu.cf { pc = 0x45aaf3; continue; }
  pc = 0x45aaca; continue;
 }
 0x45aaca => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45aad5); f_0045a6d5(cpu); if cpu.returned_to != 0x45aad5 { return; }
  if cpu.cf { pc = 0x45aaf3; continue; }
  pc = 0x45aad7; continue;
 }
 0x45aad7 => {
  { let v: u8 = cpu.r8(0x4100u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45aae2); f_0045a6d5(cpu); if cpu.returned_to != 0x45aae2 { return; }
  if cpu.cf { pc = 0x45aaf3; continue; }
  pc = 0x45aae4; continue;
 }
 0x45aae4 => {
  { let v: u8 = cpu.r8(0x4180u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45aaef); f_0045a6d5(cpu); if cpu.returned_to != 0x45aaef { return; }
  if cpu.cf { pc = 0x45aaf3; continue; }
  pc = 0x45aaf1; continue;
 }
 0x45aaf1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45aaf3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045aaf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45aaf5); }
 let mut pc: u32 = 0x45aaf5;
 loop { match pc {
 0x45aaf5 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x45ab1d; continue; }
  pc = 0x45ab12; continue;
 }
 0x45ab12 => {
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x45ab1d; continue; }
  pc = 0x45ab17; continue;
 }
 0x45ab17 => {
  pc = 0x45abaf; continue;
 }
 0x45ab1d => {
  { let v: u32 = 0x5ba1c2u32; cpu.esi = v; }
  pc = 0x45ab22; continue;
 }
 0x45ab22 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x45abaf; continue; }
  pc = 0x45ab2c; continue;
 }
 0x45ab2c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x45ab35; continue; }
  pc = 0x45ab32; continue;
 }
 0x45ab32 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x45ab22; continue;
 }
 0x45ab35 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45ab4c); f_00420989(cpu); if cpu.returned_to != 0x45ab4c { return; }
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45ab57); f_0045a6d5(cpu); if cpu.returned_to != 0x45ab57 { return; }
  if cpu.cf { pc = 0x45abaf; continue; }
  pc = 0x45ab59; continue;
 }
 0x45ab59 => {
  { let v: u8 = cpu.r8(0x3f00u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45ab64); f_0045a6d5(cpu); if cpu.returned_to != 0x45ab64 { return; }
  if cpu.cf { pc = 0x45abaf; continue; }
  pc = 0x45ab66; continue;
 }
 0x45ab66 => {
  { let v: u8 = 0x44u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45ab92); f_0041c808(cpu); if cpu.returned_to != 0x45ab92 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45aba0); f_0041c743(cpu); if cpu.returned_to != 0x45aba0 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45abaf => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045abb2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45abb2); } r_0045abb2(cpu, 0x45abb2); }
pub fn f_0045abe8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45abe8); } r_0045abb2(cpu, 0x45abe8); }
fn r_0045abb2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45abb2 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x45abd1; continue; }
  pc = 0x45abcf; continue;
 }
 0x45abcf => {
  pc = 0x45ac3e; continue;
 }
 0x45abd1 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45abe8); f_00420989(cpu); if cpu.returned_to != 0x45abe8 { return; }
  pc = 0x45abe8; continue;
 }
 0x45abe8 => {
  { let v: u8 = cpu.r8(0x4100u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45abf3); f_0045a6d5(cpu); if cpu.returned_to != 0x45abf3 { return; }
  if cpu.cf { pc = 0x45ac3e; continue; }
  pc = 0x45abf5; continue;
 }
 0x45abf5 => {
  { let v: u8 = 0x45u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45ac21); f_0041c808(cpu); if cpu.returned_to != 0x45ac21 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45ac2f); f_0041c743(cpu); if cpu.returned_to != 0x45ac2f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ac3e => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ac41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ac41); }
 let mut pc: u32 = 0x45ac41;
 loop { match pc {
 0x45ac41 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x45ac4f; continue; }
  pc = 0x45ac48; continue;
 }
 0x45ac48 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45ac74; continue; }
  pc = 0x45ac4f; continue;
 }
 0x45ac4f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x46u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45ac74; continue;
 }
 0x45ac74 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ac75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ac75); }
 let mut pc: u32 = 0x45ac75;
 loop { match pc {
 0x45ac75 => {
  cpu.push(0x45ac7a); f_0045a6a0(cpu); if cpu.returned_to != 0x45ac7a { return; }
  if cpu.cf { pc = 0x45acc3; continue; }
  pc = 0x45ac7c; continue;
 }
 0x45ac7c => {
  { let v: u8 = 0x1du8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x45aca8); f_0041c808(cpu); if cpu.returned_to != 0x45aca8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45acb6); f_0041c743(cpu); if cpu.returned_to != 0x45acb6 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45acc3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045acc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45acc5); }
 let mut pc: u32 = 0x45acc5;
 loop { match pc {
 0x45acc5 => {
  cpu.push(0x45acca); f_0045a6a0(cpu); if cpu.returned_to != 0x45acca { return; }
  if cpu.cf { pc = 0x45ad0f; continue; }
  pc = 0x45accc; continue;
 }
 0x45accc => {
  { let v: u8 = 0x1fu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45ace0); f_0041c743(cpu); if cpu.returned_to != 0x45ace0 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x14u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ad0f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ad11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ad11); }
 let mut pc: u32 = 0x45ad11;
 loop { match pc {
 0x45ad11 => {
  cpu.push(0x45ad16); f_0045a6a0(cpu); if cpu.returned_to != 0x45ad16 { return; }
  if cpu.cf { pc = 0x45ad38; continue; }
  pc = 0x45ad18; continue;
 }
 0x45ad18 => {
  { let v: u8 = 0x22u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ad38 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ad3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ad3a); }
 let mut pc: u32 = 0x45ad3a;
 loop { match pc {
 0x45ad3a => {
  cpu.push(0x45ad3f); f_0045a6a0(cpu); if cpu.returned_to != 0x45ad3f { return; }
  if cpu.cf { pc = 0x45ad6b; continue; }
  pc = 0x45ad41; continue;
 }
 0x45ad41 => {
  { let v: u8 = 0x25u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ad6b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ad6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ad6d); }
 let mut pc: u32 = 0x45ad6d;
 loop { match pc {
 0x45ad6d => {
  cpu.push(0x45ad72); f_0045a6a0(cpu); if cpu.returned_to != 0x45ad72 { return; }
  if cpu.cf { pc = 0x45adaa; continue; }
  pc = 0x45ad74; continue;
 }
 0x45ad74 => {
  { let v: u8 = 0x26u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x96u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45ada5); f_0041c808(cpu); if cpu.returned_to != 0x45ada5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45adaa => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045adac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45adac); }
 let mut pc: u32 = 0x45adac;
 loop { match pc {
 0x45adac => {
  cpu.push(0x45adb1); f_0045a6a0(cpu); if cpu.returned_to != 0x45adb1 { return; }
  if cpu.cf { pc = 0x45adce; continue; }
  pc = 0x45adb3; continue;
 }
 0x45adb3 => {
  { let v: u8 = 0x28u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45adce => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045add0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45add0); }
 let mut pc: u32 = 0x45add0;
 loop { match pc {
 0x45add0 => {
  cpu.push(0x45add5); f_0045a6a0(cpu); if cpu.returned_to != 0x45add5 { return; }
  if cpu.cf { pc = 0x45ae1b; continue; }
  pc = 0x45add7; continue;
 }
 0x45add7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x45ae1a; continue; }
  pc = 0x45adde; continue;
 }
 0x45adde => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x45ae1a; continue; }
  pc = 0x45ade5; continue;
 }
 0x45ade5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5ba062u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x29u8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x14u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x45ae1a; continue;
 }
 0x45ae1a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ae1b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ae1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ae1d); }
 let mut pc: u32 = 0x45ae1d;
 loop { match pc {
 0x45ae1d => {
  cpu.push(0x45ae22); f_0045a6a0(cpu); if cpu.returned_to != 0x45ae22 { return; }
  if cpu.cf { pc = 0x45ae55; continue; }
  pc = 0x45ae24; continue;
 }
 0x45ae24 => {
  cpu.push(0x45ae29); f_0045973c(cpu); if cpu.returned_to != 0x45ae29 { return; }
  if cpu.cf { pc = 0x45ae54; continue; }
  pc = 0x45ae2b; continue;
 }
 0x45ae2b => {
  { let v: u8 = 0x2bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45ae54; continue;
 }
 0x45ae54 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ae55 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ae57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ae57); }
 let mut pc: u32 = 0x45ae57;
 loop { match pc {
 0x45ae57 => {
  cpu.push(0x45ae5c); f_0045a6a0(cpu); if cpu.returned_to != 0x45ae5c { return; }
  if cpu.cf { pc = 0x45ae99; continue; }
  pc = 0x45ae5e; continue;
 }
 0x45ae5e => {
  { let v: u8 = 0x2au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2cu8; cpu.w8(0x85u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x86u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x14u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x28u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45ae94); f_0041c808(cpu); if cpu.returned_to != 0x45ae94 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ae99 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ae9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ae9b); }
 let mut pc: u32 = 0x45ae9b;
 loop { match pc {
 0x45ae9b => {
  { cpu.pushad(); }
  { let v: u32 = 0x5ba040u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x45af8a; continue; }
  pc = 0x45aeb6; continue;
 }
 0x45aeb6 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x5ba0d6u32, v); }
  { let v: u32 = 0x5ba0c6u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bc03u32); cpu.eax = v; }
  { let v: u32 = 0x5ba0c6u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  pc = 0x45aee9; continue;
 }
 0x45aee9 => {
  { cpu.sub8(cpu.r8(0x5ba0d6u32), 0x0u8); }
  if cpu.zf { pc = 0x45af8a; continue; }
  pc = 0x45aef6; continue;
 }
 0x45aef6 => {
  if cpu.sf { pc = 0x45af8a; continue; }
  pc = 0x45aefc; continue;
 }
 0x45aefc => {
  cpu.push(0x45af01); f_0041c743(cpu); if cpu.returned_to != 0x45af01 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  pc = 0x45af08; continue;
 }
 0x45af08 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x45af14; continue; }
  pc = 0x45af0e; continue;
 }
 0x45af0e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  cpu.poll(); pc = 0x45af08; continue;
 }
 0x45af14 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5ba0d6u32)); cpu.w8(0x5ba0d6u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x45aee9; continue; }
  pc = 0x45af30; continue;
 }
 0x45af30 => {
  { let v: u16 = (cpu.eax as u8) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x45aee9; continue; }
  pc = 0x45af45; continue;
 }
 0x45af45 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x19u8); }
  if !cpu.zf { cpu.poll(); pc = 0x45aee9; continue; }
  pc = 0x45af5b; continue;
 }
 0x45af5b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x45aee9; continue; }
  pc = 0x45af61; continue;
 }
 0x45af61 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x7fu32.wrapping_add(cpu.esi)), 0x3u16); }
  if !cpu.zf { pc = 0x45af79; continue; }
  pc = 0x45af75; continue;
 }
 0x45af75 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x45af79; continue;
 }
 0x45af79 => {
  cpu.push(0x45af7e); f_00489637(cpu); if cpu.returned_to != 0x45af7e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfffeu16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45af8a; continue;
 }
 0x45af8a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045af8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45af8c); }
 let mut pc: u32 = 0x45af8c;
 loop { match pc {
 0x45af8c => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bc03u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x45afa1; continue;
 }
 0x45afa1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x45afbe; continue; }
  pc = 0x45afa7; continue;
 }
 0x45afa7 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x2cu8); }
  if !cpu.zf { pc = 0x45afbe; continue; }
  pc = 0x45afba; continue;
 }
 0x45afba => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  pc = 0x45afbe; continue;
 }
 0x45afbe => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x45afa1; continue; }
  pc = 0x45afc1; continue;
 }
 0x45afc1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045afc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45afc2); }
 let mut pc: u32 = 0x45afc2;
 loop { match pc {
 0x45afc2 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x45afdb; continue; }
  pc = 0x45afcb; continue;
 }
 0x45afcb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x45afdb; continue; }
  pc = 0x45afd6; continue;
 }
 0x45afd6 => {
  cpu.push(0x45afdb); f_0043ffdb(cpu); if cpu.returned_to != 0x45afdb { return; }
  pc = 0x45afdb; continue;
 }
 0x45afdb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045afdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45afdc); }
 let mut pc: u32 = 0x45afdc;
 loop { match pc {
 0x45afdc => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b011; continue; }
  pc = 0x45afe2; continue;
 }
 0x45afe2 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45affa; continue; }
  pc = 0x45aff2; continue;
 }
 0x45aff2 => {
  if cpu.cf { pc = 0x45b011; continue; }
  pc = 0x45aff4; continue;
 }
 0x45aff4 => {
  cpu.push(0x45aff9); f_0043ffdb(cpu); if cpu.returned_to != 0x45aff9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45affa => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b00b); f_004824fa(cpu); if cpu.returned_to != 0x45b00b { return; }
  cpu.push(0x45b010); f_0045b012(cpu); if cpu.returned_to != 0x45b010 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b011 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b012); }
 let mut pc: u32 = 0x45b012;
 loop { match pc {
 0x45b012 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45b021); f_004679cb(cpu); if cpu.returned_to != 0x45b021 { return; }
  if cpu.cf { pc = 0x45b023; continue; }
  pc = 0x45b023; continue;
 }
 0x45b023 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x45b07b; continue; }
  pc = 0x45b02c; continue;
 }
 0x45b02c => {
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x45b069; continue; }
  pc = 0x45b035; continue;
 }
 0x45b035 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x55u8); }
  if !cpu.zf { pc = 0x45b0ff; continue; }
  pc = 0x45b03e; continue;
 }
 0x45b03e => {
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x45b062); f_00452a56(cpu); if cpu.returned_to != 0x45b062 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b069; continue;
 }
 0x45b069 => {
  { let v: u32 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x45b0b3; continue;
 }
 0x45b07b => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x45b0a3; continue; }
  pc = 0x45b098; continue;
 }
 0x45b098 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x45b0a3; continue; }
  pc = 0x45b0a1; continue;
 }
 0x45b0a1 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45b0a3; continue;
 }
 0x45b0a3 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x45b0b3); f_0045bd38(cpu); if cpu.returned_to != 0x45b0b3 { return; }
  pc = 0x45b0b3; continue;
 }
 0x45b0b3 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 3u32); cpu.esi = v; }
  if !cpu.zf { pc = 0x45b0c8; continue; }
  pc = 0x45b0c7; continue;
 }
 0x45b0c7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x45b0c8; continue;
 }
 0x45b0c8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b9cacu32.wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5b9cb0u32.wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.ebx = v; }
  { let v: u8 = 0x1fu8; cpu.w8(0x5d5f25u32, v); }
  cpu.push(0x45b0e8); f_0048126b(cpu); if cpu.returned_to != 0x45b0e8 { return; }
  if !cpu.cf { pc = 0x45b0ed; continue; }
  pc = 0x45b0ea; continue;
 }
 0x45b0ea => {
  { cpu.popad(); }
  pc = 0x45b0ff; continue;
 }
 0x45b0ed => {
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v: u8 = cpu.dec8(cpu.r8(0x95u32.wrapping_add(cpu.ebp))); cpu.w8(0x95u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b0ff; continue;
 }
 0x45b0ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b100); }
 let mut pc: u32 = 0x45b100;
 loop { match pc {
 0x45b100 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45b119); f_00458a0c(cpu); if cpu.returned_to != 0x45b119 { return; }
  cpu.push(0x45b11e); f_004895be(cpu); if cpu.returned_to != 0x45b11e { return; }
  cpu.push(0x45b123); f_0048990c(cpu); if cpu.returned_to != 0x45b123 { return; }
  if !cpu.cf { pc = 0x45b13c; continue; }
  pc = 0x45b125; continue;
 }
 0x45b125 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x45b13d; continue; }
  pc = 0x45b131; continue;
 }
 0x45b131 => {
  cpu.push(0x45b136); f_0046b3d0(cpu); if cpu.returned_to != 0x45b136 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x12u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b13c; continue;
 }
 0x45b13c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b13d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x45b143); f_0043ffdb(cpu); if cpu.returned_to != 0x45b143 { return; }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b14e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b14e); }
 let mut pc: u32 = 0x45b14e;
 loop { match pc {
 0x45b14e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x75u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5ba1b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x45b15e); f_0045b46c(cpu); if cpu.returned_to != 0x45b15e { return; }
  if cpu.cf { pc = 0x45b1a8; continue; }
  pc = 0x45b160; continue;
 }
 0x45b160 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x45b1a9; continue; }
  pc = 0x45b167; continue;
 }
 0x45b167 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { f_0045b210(cpu); return; }
  pc = 0x45b172; continue;
 }
 0x45b172 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { f_0045b26c(cpu); return; }
  pc = 0x45b17d; continue;
 }
 0x45b17d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { f_0045b2c8(cpu); return; }
  pc = 0x45b188; continue;
 }
 0x45b188 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x21u8); }
  if cpu.zf { f_0045b3a4(cpu); return; }
  pc = 0x45b193; continue;
 }
 0x45b193 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x23u8); }
  if cpu.zf { f_0045b400(cpu); return; }
  pc = 0x45b19e; continue;
 }
 0x45b19e => {
  cpu.push(0x45b1a3); f_0045b1b4(cpu); if cpu.returned_to != 0x45b1a3 { return; }
  cpu.push(0x45b1a8); f_00489968(cpu); if cpu.returned_to != 0x45b1a8 { return; }
  pc = 0x45b1a8; continue;
 }
 0x45b1a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b1a9 => {
  cpu.push(0x45b1ae); f_0045b358(cpu); if cpu.returned_to != 0x45b1ae { return; }
  cpu.push(0x45b1b3); f_00489968(cpu); if cpu.returned_to != 0x45b1b3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b1b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b1b4); }
 let mut pc: u32 = 0x45b1b4;
 loop { match pc {
 0x45b1b4 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b20f); f_0045c73e(cpu); if cpu.returned_to != 0x45b20f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b210); }
 let mut pc: u32 = 0x45b210;
 loop { match pc {
 0x45b210 => {
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x19u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b26b); f_0045c73e(cpu); if cpu.returned_to != 0x45b26b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b26c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b26c); }
 let mut pc: u32 = 0x45b26c;
 loop { match pc {
 0x45b26c => {
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1eu8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b2c7); f_0045c73e(cpu); if cpu.returned_to != 0x45b2c7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b2c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b2c8); }
 let mut pc: u32 = 0x45b2c8;
 loop { match pc {
 0x45b2c8 => {
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5ba1b8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x45b2de; continue; }
  pc = 0x45b2dc; continue;
 }
 0x45b2dc => {
  { let v: u8 = 0x50u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45b2de; continue;
 }
 0x45b2de => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xau8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b357); f_0045c73e(cpu); if cpu.returned_to != 0x45b357 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b358); }
 let mut pc: u32 = 0x45b358;
 loop { match pc {
 0x45b358 => {
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x18u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b3a3); f_0045c73e(cpu); if cpu.returned_to != 0x45b3a3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b3a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b3a4); }
 let mut pc: u32 = 0x45b3a4;
 loop { match pc {
 0x45b3a4 => {
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x22u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b3ff); f_0045c73e(cpu); if cpu.returned_to != 0x45b3ff { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b400); }
 let mut pc: u32 = 0x45b400;
 loop { match pc {
 0x45b400 => {
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x24u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45b46b); f_0045c73e(cpu); if cpu.returned_to != 0x45b46b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b46c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b46c); }
 let mut pc: u32 = 0x45b46c;
 loop { match pc {
 0x45b46c => {
  cpu.push(0x45b471); f_004819ab(cpu); if cpu.returned_to != 0x45b471 { return; }
  if cpu.cf { pc = 0x45b4f8; continue; }
  pc = 0x45b477; continue;
 }
 0x45b477 => {
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x75u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2au32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0xfu32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b4f8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b501(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b501); }
 let mut pc: u32 = 0x45b501;
 loop { match pc {
 0x45b501 => {
  cpu.push(0x45b506); f_0045b46c(cpu); if cpu.returned_to != 0x45b506 { return; }
  if cpu.cf { pc = 0x45b5aa; continue; }
  pc = 0x45b50c; continue;
 }
 0x45b50c => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x45b59d; continue; }
  pc = 0x45b556; continue;
 }
 0x45b556 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x45b577; continue; }
  pc = 0x45b55d; continue;
 }
 0x45b55d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x45b58a; continue; }
  pc = 0x45b564; continue;
 }
 0x45b564 => {
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b577 => {
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b58a => {
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b59d => {
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b5aa; continue;
 }
 0x45b5aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b5ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b5ab); }
 let mut pc: u32 = 0x45b5ab;
 loop { match pc {
 0x45b5ab => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x45b5e6; continue; }
  pc = 0x45b5c5; continue;
 }
 0x45b5c5 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x45b5cb); f_004782f8(cpu); if cpu.returned_to != 0x45b5cb { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x45b63c; continue; }
  pc = 0x45b5ce; continue;
 }
 0x45b5ce => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x45b5d4); f_0047a472(cpu); if cpu.returned_to != 0x45b5d4 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x45b63c; continue; }
  pc = 0x45b5d7; continue;
 }
 0x45b5d7 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x45b5e6; continue; }
  pc = 0x45b5e0; continue;
 }
 0x45b5e0 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  pc = 0x45b5e6; continue;
 }
 0x45b5e6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45b602; continue; }
  pc = 0x45b5ed; continue;
 }
 0x45b5ed => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x45b602; continue; }
  pc = 0x45b5f4; continue;
 }
 0x45b5f4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45b602; continue; }
  pc = 0x45b5fb; continue;
 }
 0x45b5fb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45b608; continue; }
  pc = 0x45b602; continue;
 }
 0x45b602 => {
  cpu.push(0x45b607); f_004525bf(cpu); if cpu.returned_to != 0x45b607 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b608 => {
  cpu.push(0x45b60d); f_0047a294(cpu); if cpu.returned_to != 0x45b60d { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x45b631); f_0045955b(cpu); if cpu.returned_to != 0x45b631 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x45b637); f_004895c7(cpu); if cpu.returned_to != 0x45b637 { return; }
  cpu.push(0x45b63c); f_00489637(cpu); if cpu.returned_to != 0x45b63c { return; }
  pc = 0x45b63c; continue;
 }
 0x45b63c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b63d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b63d); }
 let mut pc: u32 = 0x45b63d;
 loop { match pc {
 0x45b63d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b68a; continue; }
  pc = 0x45b643; continue;
 }
 0x45b643 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45b684; continue; }
  pc = 0x45b653; continue;
 }
 0x45b653 => {
  if !cpu.zf { pc = 0x45b68a; continue; }
  pc = 0x45b655; continue;
 }
 0x45b655 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x45b68a; continue; }
  pc = 0x45b66e; continue;
 }
 0x45b66e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b684 => {
  cpu.push(0x45b689); f_0043ffdb(cpu); if cpu.returned_to != 0x45b689 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b68a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b68b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b68b); }
 let mut pc: u32 = 0x45b68b;
 loop { match pc {
 0x45b68b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b6a3; continue; }
  pc = 0x45b691; continue;
 }
 0x45b691 => {
  { let v: u8 = 0x4u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45b6a4; continue; }
  pc = 0x45b6a1; continue;
 }
 0x45b6a1 => {
  if !cpu.zf { pc = 0x45b6a3; continue; }
  pc = 0x45b6a3; continue;
 }
 0x45b6a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b6a4 => {
  { let v: u8 = 0x11u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b6b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b6b9); }
 let mut pc: u32 = 0x45b6b9;
 loop { match pc {
 0x45b6b9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b6da; continue; }
  pc = 0x45b6bf; continue;
 }
 0x45b6bf => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b6da; continue; }
  pc = 0x45b6cf; continue;
 }
 0x45b6cf => {
  { let v: u8 = cpu.dec8(cpu.r8(0x36u32.wrapping_add(cpu.ebp))); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x45b6da; continue; }
  pc = 0x45b6d5; continue;
 }
 0x45b6d5 => {
  cpu.push(0x45b6da); f_0043ffdb(cpu); if cpu.returned_to != 0x45b6da { return; }
  pc = 0x45b6da; continue;
 }
 0x45b6da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b70e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b70e); }
 let mut pc: u32 = 0x45b70e;
 loop { match pc {
 0x45b70e => {
  { let v: u8 = 0x0u8; cpu.w8(0x5820f8u32, v); }
  { cpu.sub8(cpu.r8(0x5d2f40u32), 0x0u8); }
  if !cpu.zf { pc = 0x45b729; continue; }
  pc = 0x45b71e; continue;
 }
 0x45b71e => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x45b8fa; continue; }
  pc = 0x45b729; continue;
 }
 0x45b729 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x45b73e; continue; }
  pc = 0x45b730; continue;
 }
 0x45b730 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45b73e; continue; }
  pc = 0x45b737; continue;
 }
 0x45b737 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45b77e; continue; }
  pc = 0x45b73e; continue;
 }
 0x45b73e => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45b8fa; continue; }
  pc = 0x45b749; continue;
 }
 0x45b749 => {
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x45b77e; continue; }
  pc = 0x45b753; continue;
 }
 0x45b753 => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x45b75a); f_004521d0(cpu); if cpu.returned_to != 0x45b75a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  if cpu.cf { pc = 0x45b76e; continue; }
  pc = 0x45b75f; continue;
 }
 0x45b75f => {
  { let v: u8 = cpu.r8(0x95u32.wrapping_add(cpu.esi)).wrapping_sub(0x1u8); cpu.w8(0x95u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x95u32.wrapping_add(cpu.ebp)).wrapping_add(0x1u8); cpu.w8(0x95u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b76e; continue;
 }
 0x45b76e => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x45b8fa; continue; }
  pc = 0x45b777; continue;
 }
 0x45b777 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5820f8u32, v); }
  pc = 0x45b77e; continue;
 }
 0x45b77e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b789; continue;
 }
 0x45b789 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x45b79e; continue; }
  pc = 0x45b790; continue;
 }
 0x45b790 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b79e; continue;
 }
 0x45b79e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b7a9; continue;
 }
 0x45b7a9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b7b4; continue;
 }
 0x45b7b4 => {
  { let v: u32 = cpu.r32(0x4d6b7cu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x45b7ca; continue; }
  pc = 0x45b7bd; continue;
 }
 0x45b7bd => {
  { let v: u32 = cpu.r32(0x4d6b80u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b7ca; continue;
 }
 0x45b7ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4ea063u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b7dc; continue;
 }
 0x45b7dc => {
  { cpu.sub8(cpu.r8(0x5820f8u32), 0x0u8); }
  if cpu.zf { pc = 0x45b7ee; continue; }
  pc = 0x45b7e5; continue;
 }
 0x45b7e5 => {
  { cpu.sub8((cpu.ebx as u8), 0x11u8); }
  if cpu.zf { pc = 0x45b8fa; continue; }
  pc = 0x45b7ee; continue;
 }
 0x45b7ee => {
  cpu.push(0x45b7f3); f_0042c9e5(cpu); if cpu.returned_to != 0x45b7f3 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b801; continue;
 }
 0x45b801 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b80c; continue;
 }
 0x45b80c => {
  { cpu.sub32(cpu.r32(0x4d6a60u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x45b81d; continue; }
  pc = 0x45b815; continue;
 }
 0x45b815 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4d6a34u32)); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b81d; continue;
 }
 0x45b81d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b824; continue;
 }
 0x45b824 => {
  { cpu.sub8(cpu.r8(0x5d2f40u32), 0x0u8); }
  if !cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b82d; continue;
 }
 0x45b82d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b834; continue;
 }
 0x45b834 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b83b; continue;
 }
 0x45b83b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b842; continue;
 }
 0x45b842 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b849; continue;
 }
 0x45b849 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b850; continue;
 }
 0x45b850 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b857; continue;
 }
 0x45b857 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b85e; continue;
 }
 0x45b85e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45b898; continue; }
  pc = 0x45b865; continue;
 }
 0x45b865 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x45b86c); f_00476604(cpu); if cpu.returned_to != 0x45b86c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x1000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45b8fa; continue; }
  pc = 0x45b898; continue;
 }
 0x45b898 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x45b8a9); f_0041c743(cpu); if cpu.returned_to != 0x45b8a9 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x45b8b7); f_0041c743(cpu); if cpu.returned_to != 0x45b8b7 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x45b8ea); f_00413337(cpu); if cpu.returned_to != 0x45b8ea { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b8fa => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b8fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b8fc); }
 let mut pc: u32 = 0x45b8fc;
 loop { match pc {
 0x45b8fc => {
  { let v: u32 = cpu.r32(0x4d6b7cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x45b90d; continue; }
  pc = 0x45b906; continue;
 }
 0x45b906 => {
  cpu.push(0x45b90b); f_0045b9b0(cpu); if cpu.returned_to != 0x45b90b { return; }
  if cpu.cf { pc = 0x45b933; continue; }
  pc = 0x45b90d; continue;
 }
 0x45b90d => {
  { let v: u32 = cpu.r32(0x4d6b80u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x45b91e; continue; }
  pc = 0x45b917; continue;
 }
 0x45b917 => {
  cpu.push(0x45b91c); f_0045b9b0(cpu); if cpu.returned_to != 0x45b91c { return; }
  if cpu.cf { pc = 0x45b933; continue; }
  pc = 0x45b91e; continue;
 }
 0x45b91e => {
  { cpu.sub8(cpu.r8(0x65u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x45b935; continue; }
  pc = 0x45b925; continue;
 }
 0x45b925 => {
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(cpu.r8(0x65u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x45b933; continue; }
  pc = 0x45b92f; continue;
 }
 0x45b92f => {
  if cpu.sf { pc = 0x45b933; continue; }
  pc = 0x45b931; continue;
 }
 0x45b931 => {
  pc = 0x45b95b; continue;
 }
 0x45b933 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b935 => {
  { cpu.sub8(cpu.r8(0x9eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x45b94a; continue; }
  pc = 0x45b93f; continue;
 }
 0x45b93f => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b94a; continue;
 }
 0x45b94a => {
  { let v: u8 = 0x8u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x45b95b; continue; }
  pc = 0x45b956; continue;
 }
 0x45b956 => {
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.ebp)).wrapping_add(0x8u8); cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45b95b; continue;
 }
 0x45b95b => {
  { cpu.sub32(cpu.r32(0x4d6b7cu32), 0x0u32); }
  if cpu.zf { pc = 0x45b96f; continue; }
  pc = 0x45b964; continue;
 }
 0x45b964 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  pc = 0x45b96f; continue;
 }
 0x45b96f => {
  { cpu.sub32(cpu.r32(0x4d6b80u32), 0x0u32); }
  if cpu.zf { pc = 0x45b983; continue; }
  pc = 0x45b978; continue;
 }
 0x45b978 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  pc = 0x45b983; continue;
 }
 0x45b983 => {
  cpu.push(0x45b988); f_0046a5a6(cpu); if cpu.returned_to != 0x45b988 { return; }
  if !cpu.cf { pc = 0x45b9a2; continue; }
  pc = 0x45b98a; continue;
 }
 0x45b98a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45b9a2 => {
  cpu.push(0x45b9a7); f_0045ba19(cpu); if cpu.returned_to != 0x45b9a7 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc69fu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045b9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45b9b0); }
 let mut pc: u32 = 0x45b9b0;
 loop { match pc {
 0x45b9b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45b9bb; continue;
 }
 0x45b9bb => {
  { cpu.sub32(cpu.ebx, 0x4dd54du32); }
  if cpu.cf { pc = 0x45ba02; continue; }
  pc = 0x45b9c3; continue;
 }
 0x45b9c3 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45b9cc; continue;
 }
 0x45b9cc => {
  { cpu.sub32(cpu.ebx, 0x4dd5cbu32); }
  if cpu.cf { pc = 0x45ba02; continue; }
  pc = 0x45b9d4; continue;
 }
 0x45b9d4 => {
  { let v: u8 = cpu.r8(0xffffff81u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45b9dd; continue;
 }
 0x45b9dd => {
  { cpu.sub32(cpu.ebx, 0x4dd5ccu32); }
  if cpu.cf { pc = 0x45ba02; continue; }
  pc = 0x45b9e5; continue;
 }
 0x45b9e5 => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45b9ee; continue;
 }
 0x45b9ee => {
  { cpu.sub32(cpu.ebx, 0x4dd5cdu32); }
  if cpu.cf { pc = 0x45ba02; continue; }
  pc = 0x45b9f6; continue;
 }
 0x45b9f6 => {
  { let v: u8 = cpu.r8(0xffffff7fu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45ba02; continue;
 }
 0x45ba02 => {
  { cpu.sub32(cpu.ebx, 0x4e154bu32); }
  if !cpu.cf { pc = 0x45ba13; continue; }
  pc = 0x45ba0a; continue;
 }
 0x45ba0a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x42u8); }
  if cpu.zf { pc = 0x45ba16; continue; }
  pc = 0x45ba13; continue;
 }
 0x45ba13 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba16 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ba19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45ba19); }
 let mut pc: u32 = 0x45ba19;
 loop { match pc {
 0x45ba19 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45ba47; continue; }
  pc = 0x45ba2e; continue;
 }
 0x45ba2e => {
  if (cpu.sf != cpu.of) { pc = 0x45ba65; continue; }
  pc = 0x45ba30; continue;
 }
 0x45ba30 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x45ba81; continue; }
  pc = 0x45ba37; continue;
 }
 0x45ba37 => {
  if (cpu.sf != cpu.of) { pc = 0x45ba40; continue; }
  pc = 0x45ba39; continue;
 }
 0x45ba39 => {
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba40 => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba47 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x45ba57; continue; }
  pc = 0x45ba4e; continue;
 }
 0x45ba4e => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45ba5e; continue; }
  pc = 0x45ba50; continue;
 }
 0x45ba50 => {
  { let v: u8 = 0x2u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba57 => {
  { let v: u8 = 0x1u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba5e => {
  { let v: u8 = 0x3u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba65 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x45ba75; continue; }
  pc = 0x45ba6c; continue;
 }
 0x45ba6c => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x45ba7c; continue; }
  pc = 0x45ba6e; continue;
 }
 0x45ba6e => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba75 => {
  { let v: u8 = 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45ba7c => {
  { let v: u8 = 0x5u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45ba81; continue;
 }
 0x45ba81 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045ba83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45ba83); } r_0045ba83(cpu, 0x45ba83); }
pub fn f_0045bac8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45bac8); } r_0045ba83(cpu, 0x45bac8); }
fn r_0045ba83(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45ba83 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6b7cu32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x45baa6; continue; }
  pc = 0x45ba91; continue;
 }
 0x45ba91 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x45baa6; continue; }
  pc = 0x45ba9a; continue;
 }
 0x45ba9a => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x61u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x45bac8; continue; }
  pc = 0x45baa2; continue;
 }
 0x45baa2 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x45bac8; continue; }
  pc = 0x45baa6; continue;
 }
 0x45baa6 => {
  { let v: u32 = cpu.r32(0x4d6b80u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x45bac5; continue; }
  pc = 0x45bab0; continue;
 }
 0x45bab0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x45bac5; continue; }
  pc = 0x45bab9; continue;
 }
 0x45bab9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x61u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x45bac7; continue; }
  pc = 0x45bac1; continue;
 }
 0x45bac1 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x45bac8; continue; }
  pc = 0x45bac5; continue;
 }
 0x45bac5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45bac7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45bac8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc69du32, v); }
  cpu.push(0x45badb); f_0046a42c(cpu); if cpu.returned_to != 0x45badb { return; }
  if cpu.cf { pc = 0x45bba1; continue; }
  pc = 0x45bae1; continue;
 }
 0x45bae1 => {
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x45baea); f_0045c7e2(cpu); if cpu.returned_to != 0x45baea { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x36u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x45bb0d); f_0045bb26(cpu); if cpu.returned_to != 0x45bb0d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x36u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45bba1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fa8u32, v); }
  { let v: u32 = cpu.r32(0x4d6be8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x45bbbd; continue; }
  pc = 0x45bbb6; continue;
 }
 0x45bbb6 => {
  { cpu.pushad(); }
  cpu.push(0x45bbbc); f_0046a42e(cpu); if cpu.returned_to != 0x45bbbc { return; }
  { cpu.popad(); }
  pc = 0x45bbbd; continue;
 }
 0x45bbbd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045bb26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45bb26); }
 let mut pc: u32 = 0x45bb26;
 loop { match pc {
 0x45bb26 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b7cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b80u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x45bb90; continue; }
  pc = 0x45bb68; continue;
 }
 0x45bb68 => {
  if cpu.sf { pc = 0x45bb7d; continue; }
  pc = 0x45bb6a; continue;
 }
 0x45bb6a => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { f_004892a9(cpu); return; }
  pc = 0x45bb72; continue;
 }
 0x45bb72 => {
  if !cpu.sf { pc = 0x489322; continue; }
  pc = 0x45bb78; continue;
 }
 0x45bb78 => {
  pc = 0x48934a; continue;
 }
 0x45bb7d => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { f_00489301(cpu); return; }
  pc = 0x45bb85; continue;
 }
 0x45bb85 => {
  if !cpu.sf { pc = 0x489372; continue; }
  pc = 0x45bb8b; continue;
 }
 0x45bb8b => {
  pc = 0x48939a; continue;
 }
 0x45bb90 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x45bb9f; continue; }
  pc = 0x45bb94; continue;
 }
 0x45bb94 => {
  if !cpu.sf { f_00489286(cpu); return; }
  pc = 0x45bb9a; continue;
 }
 0x45bb9a => {
  f_00489238(cpu); return;
 }
 0x45bb9f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489322 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x489332); f_004892a9(cpu); if cpu.returned_to != 0x489332 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x489338); f_00489286(cpu); if cpu.returned_to != 0x489338 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x489348; continue; }
  pc = 0x489346; continue;
 }
 0x489346 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489348 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48934a => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48935a); f_004892a9(cpu); if cpu.returned_to != 0x48935a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x489360); f_00489238(cpu); if cpu.returned_to != 0x489360 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x489370; continue; }
  pc = 0x48936e; continue;
 }
 0x48936e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489370 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489372 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x489382); f_00489301(cpu); if cpu.returned_to != 0x489382 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x489388); f_00489286(cpu); if cpu.returned_to != 0x489388 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x489398; continue; }
  pc = 0x489396; continue;
 }
 0x489396 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489398 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48939a => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4893aa); f_00489301(cpu); if cpu.returned_to != 0x4893aa { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x4893b0); f_00489238(cpu); if cpu.returned_to != 0x4893b0 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4893c0; continue; }
  pc = 0x4893be; continue;
 }
 0x4893be => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4893c0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045bbbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45bbbe); }
 let mut pc: u32 = 0x45bbbe;
 loop { match pc {
 0x45bbbe => {
  cpu.push(0x45bbc3); f_0041c743(cpu); if cpu.returned_to != 0x45bbc3 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045bbca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45bbca); }
 let mut pc: u32 = 0x45bbca;
 loop { match pc {
 0x45bbca => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x45bbf0; continue; }
  pc = 0x45bbd1; continue;
 }
 0x45bbd1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x45bbe2; continue; }
  pc = 0x45bbe0; continue;
 }
 0x45bbe0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45bbe2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x45bbea); f_0041c743(cpu); if cpu.returned_to != 0x45bbea { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45bbf0 => {
  cpu.push(0x45bbf5); f_0041c743(cpu); if cpu.returned_to != 0x45bbf5 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
