#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004810cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4810cb); }
 let mut pc: u32 = 0x4810cb;
 loop { match pc {
 0x4810cb => {
  cpu.push(0x4810d0); f_004824a8(cpu); if cpu.returned_to != 0x4810d0 { return; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x4810dc; continue; }
  pc = 0x4810d5; continue;
 }
 0x4810d5 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x4810dc; continue; }
  pc = 0x4810da; continue;
 }
 0x4810da => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4810dc => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x4810da; continue; }
  pc = 0x4810e0; continue;
 }
 0x4810e0 => {
  cpu.push(0x4810e5); f_004813d3(cpu); if cpu.returned_to != 0x4810e5 { return; }
  if cpu.cf { pc = 0x481183; continue; }
  pc = 0x4810eb; continue;
 }
 0x4810eb => {
  cpu.push(0x4810f0); f_00481184(cpu); if cpu.returned_to != 0x4810f0 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  cpu.push(0x4810ff); f_004679cb(cpu); if cpu.returned_to != 0x4810ff { return; }
  if cpu.cf { pc = 0x481101; continue; }
  pc = 0x481101; continue;
 }
 0x481101 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48110b); f_00481441(cpu); if cpu.returned_to != 0x48110b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x4810da; continue; }
  pc = 0x481112; continue;
 }
 0x481112 => {
  { let v: u8 = 0x2u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x481138); f_0040d710(cpu); if cpu.returned_to != 0x481138 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x481183; continue;
 }
 0x481183 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481184(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481184); }
 let mut pc: u32 = 0x481184;
 loop { match pc {
 0x481184 => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x5ba17fu32; cpu.w32(0x4d6af0u32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x4811a5); f_0046d630(cpu); if cpu.returned_to != 0x4811a5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x4811f6; continue; }
  pc = 0x4811a8; continue;
 }
 0x4811a8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811af; continue;
 }
 0x4811af => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811b6; continue;
 }
 0x4811b6 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811bd; continue;
 }
 0x4811bd => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811c4; continue;
 }
 0x4811c4 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5du32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811cd; continue;
 }
 0x4811cd => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4811f6; continue; }
  pc = 0x4811e1; continue;
 }
 0x4811e1 => {
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4811ee; continue; }
  pc = 0x4811e7; continue;
 }
 0x4811e7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4811ed); f_004811f8(cpu); if cpu.returned_to != 0x4811ed { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4811ee; continue;
 }
 0x4811ee => {
  { let v: u8 = 0x44u8; cpu.w8(0x3fu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x4811f6; continue;
 }
 0x4811f6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004811f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4811f8); }
 let mut pc: u32 = 0x4811f8;
 loop { match pc {
 0x4811f8 => {
  { let v = cpu.r32(0x4d6a28u32); cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x48120b; continue;
 }
 0x48120b => {
  { let v: u32 = 0xb800u32; cpu.eax = v; }
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x481260; continue; }
  pc = 0x481215; continue;
 }
 0x481215 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  cpu.push(0x48122e); f_0041c743(cpu); if cpu.returned_to != 0x48122e { return; }
  { let v: u32 = cpu.eax & 0x70u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x38u32); cpu.eax = v; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.imul8(v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  cpu.push(0x481244); f_0041c743(cpu); if cpu.returned_to != 0x481244 { return; }
  { let v: u32 = cpu.eax & 0x70u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x38u32); cpu.eax = v; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.imul8(v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x481259); f_004895c7(cpu); if cpu.returned_to != 0x481259 { return; }
  cpu.push(0x48125e); f_00489637(cpu); if cpu.returned_to != 0x48125e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x481260; continue;
 }
 0x481260 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48120b; continue; }
  pc = 0x481262; continue;
 }
 0x481262 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a28u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048126b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48126b); }
 let mut pc: u32 = 0x48126b;
 loop { match pc {
 0x48126b => {
  cpu.push(0x481270); f_004824a8(cpu); if cpu.returned_to != 0x481270 { return; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x48127c; continue; }
  pc = 0x481275; continue;
 }
 0x481275 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x48127c; continue; }
  pc = 0x48127a; continue;
 }
 0x48127a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48127c => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x48127a; continue; }
  pc = 0x481280; continue;
 }
 0x481280 => {
  cpu.push(0x481285); f_004813d3(cpu); if cpu.returned_to != 0x481285 { return; }
  if cpu.cf { pc = 0x4813d2; continue; }
  pc = 0x48128b; continue;
 }
 0x48128b => {
  cpu.push(0x481290); f_00481184(cpu); if cpu.returned_to != 0x481290 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4812a1); f_004679cb(cpu); if cpu.returned_to != 0x4812a1 { return; }
  if cpu.cf { pc = 0x4812f0; continue; }
  pc = 0x4812a3; continue;
 }
 0x4812a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5d5f25u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4812f0; continue; }
  pc = 0x4812ae; continue;
 }
 0x4812ae => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x4812f0; continue; }
  pc = 0x4812b7; continue;
 }
 0x4812b7 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4812be); f_00401130(cpu); if cpu.returned_to != 0x4812be { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4812d2); f_00415d61(cpu); if cpu.returned_to != 0x4812d2 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5d5f26u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x4812f0; continue; }
  pc = 0x4812e0; continue;
 }
 0x4812e0 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4812f0; continue; }
  pc = 0x4812e4; continue;
 }
 0x4812e4 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4812ec); f_004010d0(cpu); if cpu.returned_to != 0x4812ec { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x4812f0; continue;
 }
 0x4812f0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4812fc); f_00481441(cpu); if cpu.returned_to != 0x4812fc { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x48127a; continue; }
  pc = 0x481307; continue;
 }
 0x481307 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48132e); f_0040d710(cpu); if cpu.returned_to != 0x48132e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
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
  if (cpu.sf != cpu.of) { pc = 0x481375; continue; }
  pc = 0x481371; continue;
 }
 0x481371 => {
  { let v: u16 = 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x481375; continue;
 }
 0x481375 => {
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48138f; continue; }
  pc = 0x48138a; continue;
 }
 0x48138a => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x48138f; continue;
 }
 0x48138f => {
  { let v: u32 = cpu.r32(0x5d5edcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
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
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x4813d2; continue;
 }
 0x4813d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004813d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4813d3); }
 let mut pc: u32 = 0x4813d3;
 loop { match pc {
 0x4813d3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004813fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4813fe); }
 let mut pc: u32 = 0x4813fe;
 loop { match pc {
 0x4813fe => {
  { let v: u16 = cpu.r16(0x4d693cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x481440; continue; }
  pc = 0x481420; continue;
 }
 0x481420 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x481428); f_0041c743(cpu); if cpu.returned_to != 0x481428 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x162u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x481440); cpu.call(t); if cpu.returned_to != 0x481440 { return; }
  pc = 0x481440; continue;
 }
 0x481440 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481441(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481441); }
 let mut pc: u32 = 0x481441;
 loop { match pc {
 0x481441 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d5f1fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x481497; continue; }
  pc = 0x481466; continue;
 }
 0x481466 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x481497; continue; }
  pc = 0x48146d; continue;
 }
 0x48146d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48147b; continue; }
  pc = 0x481476; continue;
 }
 0x481476 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x481480; continue; }
  pc = 0x48147b; continue;
 }
 0x48147b => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x481480; continue;
 }
 0x481480 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x481486; continue;
 }
 0x481486 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x481493); f_004801d0(cpu); if cpu.returned_to != 0x481493 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x4814ae; continue; }
  pc = 0x481496; continue;
 }
 0x481496 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481497 => {
  { let v: u8 = cpu.r8(0x4d69e6u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4814a8; continue; }
  pc = 0x4814a4; continue;
 }
 0x4814a4 => {
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4814a8; continue;
 }
 0x4814a8 => {
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x481486; continue;
 }
 0x4814ae => {
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
  { let v: u8 = cpu.r8(0x5d5f1fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48154e; continue; }
  pc = 0x4814fa; continue;
 }
 0x4814fa => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481509; continue; }
  pc = 0x481504; continue;
 }
 0x481504 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x481509; continue;
 }
 0x481509 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x481530; continue; }
  pc = 0x481512; continue;
 }
 0x481512 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481530 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48154e => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481560; continue; }
  pc = 0x48155b; continue;
 }
 0x48155b => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x481560; continue;
 }
 0x481560 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x481587; continue; }
  pc = 0x481569; continue;
 }
 0x481569 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481587 => {
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
pub fn f_004815a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4815a5); }
 let mut pc: u32 = 0x4815a5;
 loop { match pc {
 0x4815a5 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4815c6; continue; }
  pc = 0x4815c0; continue;
 }
 0x4815c0 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4815ca; continue; }
  pc = 0x4815c6; continue;
 }
 0x4815c6 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4815ca; continue;
 }
 0x4815ca => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x4815d4; continue; }
  pc = 0x4815d1; continue;
 }
 0x4815d1 => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4815d4; continue;
 }
 0x4815d4 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4815e4; continue;
 }
 0x4815e4 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4815f5; continue; }
  pc = 0x4815e9; continue;
 }
 0x4815e9 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  pc = 0x4815f5; continue;
 }
 0x4815f5 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4815e4; continue; }
  pc = 0x4815f9; continue;
 }
 0x4815f9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481611(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481611); }
 let mut pc: u32 = 0x481611;
 loop { match pc {
 0x481611 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x481632; continue; }
  pc = 0x48162c; continue;
 }
 0x48162c => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x481636; continue; }
  pc = 0x481632; continue;
 }
 0x481632 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x481636; continue;
 }
 0x481636 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x481640; continue; }
  pc = 0x48163d; continue;
 }
 0x48163d => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x481640; continue;
 }
 0x481640 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x481650; continue;
 }
 0x481650 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48165c; continue; }
  pc = 0x481655; continue;
 }
 0x481655 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48165c; continue;
 }
 0x48165c => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481650; continue; }
  pc = 0x481665; continue;
 }
 0x481665 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048167d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48167d); }
 let mut pc: u32 = 0x48167d;
 loop { match pc {
 0x48167d => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48169e; continue; }
  pc = 0x481698; continue;
 }
 0x481698 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4816a2; continue; }
  pc = 0x48169e; continue;
 }
 0x48169e => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4816a2; continue;
 }
 0x4816a2 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x4816ac; continue; }
  pc = 0x4816a9; continue;
 }
 0x4816a9 => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4816ac; continue;
 }
 0x4816ac => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4816bc; continue;
 }
 0x4816bc => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4816cd; continue; }
  pc = 0x4816c1; continue;
 }
 0x4816c1 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  pc = 0x4816cd; continue;
 }
 0x4816cd => {
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4816bc; continue; }
  pc = 0x4816d1; continue;
 }
 0x4816d1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004816e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4816e9); }
 let mut pc: u32 = 0x4816e9;
 loop { match pc {
 0x4816e9 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48170a; continue; }
  pc = 0x481704; continue;
 }
 0x481704 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48170e; continue; }
  pc = 0x48170a; continue;
 }
 0x48170a => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x48170e; continue;
 }
 0x48170e => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x481718; continue; }
  pc = 0x481715; continue;
 }
 0x481715 => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x481718; continue;
 }
 0x481718 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x481728; continue;
 }
 0x481728 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481734; continue; }
  pc = 0x48172d; continue;
 }
 0x48172d => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x481734; continue;
 }
 0x481734 => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481728; continue; }
  pc = 0x48173d; continue;
 }
 0x48173d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481755(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481755); }
 let mut pc: u32 = 0x481755;
 loop { match pc {
 0x481755 => {
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4817a5; continue; }
  pc = 0x481766; continue;
 }
 0x481766 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481771; continue; }
  pc = 0x48176e; continue;
 }
 0x48176e => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x481771; continue;
 }
 0x481771 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x48178f; continue; }
  pc = 0x481776; continue;
 }
 0x481776 => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481788; continue; }
  pc = 0x48177e; continue;
 }
 0x48177e => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4817ed; continue; }
  pc = 0x481786; continue;
 }
 0x481786 => {
  pc = 0x4817ea; continue;
 }
 0x481788 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4817f0; continue; }
  pc = 0x48178d; continue;
 }
 0x48178d => {
  pc = 0x4817ed; continue;
 }
 0x48178f => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48179e; continue; }
  pc = 0x481794; continue;
 }
 0x481794 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x4817e7; continue; }
  pc = 0x48179c; continue;
 }
 0x48179c => {
  pc = 0x4817ea; continue;
 }
 0x48179e => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x4817f3; continue; }
  pc = 0x4817a3; continue;
 }
 0x4817a3 => {
  pc = 0x4817f0; continue;
 }
 0x4817a5 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4817b3; continue; }
  pc = 0x4817b0; continue;
 }
 0x4817b0 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4817b3; continue;
 }
 0x4817b3 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x4817d1; continue; }
  pc = 0x4817b8; continue;
 }
 0x4817b8 => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4817ca; continue; }
  pc = 0x4817c0; continue;
 }
 0x4817c0 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4817f9; continue; }
  pc = 0x4817c8; continue;
 }
 0x4817c8 => {
  pc = 0x4817fc; continue;
 }
 0x4817ca => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4817f9; continue; }
  pc = 0x4817cf; continue;
 }
 0x4817cf => {
  pc = 0x4817f6; continue;
 }
 0x4817d1 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4817e0; continue; }
  pc = 0x4817d6; continue;
 }
 0x4817d6 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x4817e7; continue; }
  pc = 0x4817de; continue;
 }
 0x4817de => {
  pc = 0x4817fc; continue;
 }
 0x4817e0 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x4817f3; continue; }
  pc = 0x4817e5; continue;
 }
 0x4817e5 => {
  pc = 0x4817f6; continue;
 }
 0x4817e7 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817ea => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817ed => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817f0 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817f3 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817f6 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817f9 => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4817fc => {
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004817ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4817ff); }
 let mut pc: u32 = 0x4817ff;
 loop { match pc {
 0x4817ff => {
  { let v: u32 = 0x51e310u32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x481809; continue;
 }
 0x481809 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if !cpu.zf { pc = 0x481811; continue; }
  pc = 0x48180f; continue;
 }
 0x48180f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481811 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481809; continue; }
  pc = 0x481816; continue;
 }
 0x481816 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481818(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481818); }
 let mut pc: u32 = 0x481818;
 loop { match pc {
 0x481818 => {
  { let v: u32 = 0x51e310u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x481826; continue;
 }
 0x481826 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481826; continue; }
  pc = 0x481830; continue;
 }
 0x481830 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481831(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481831); }
 let mut pc: u32 = 0x481831;
 loop { match pc {
 0x481831 => {
  cpu.push(0x481836); f_004817ff(cpu); if cpu.returned_to != 0x481836 { return; }
  if cpu.cf { pc = 0x48187b; continue; }
  pc = 0x481838; continue;
 }
 0x481838 => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x481843); f_00420989(cpu); if cpu.returned_to != 0x481843 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x481869; continue; }
  pc = 0x48184f; continue;
 }
 0x48184f => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x481869; continue; }
  pc = 0x481853; continue;
 }
 0x481853 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x481869; continue; }
  pc = 0x481857; continue;
 }
 0x481857 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x481869; continue; }
  pc = 0x48185b; continue;
 }
 0x48185b => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x481869; continue; }
  pc = 0x48185f; continue;
 }
 0x48185f => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x48186d; continue; }
  pc = 0x481863; continue;
 }
 0x481863 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x48187c; continue; }
  pc = 0x481867; continue;
 }
 0x481867 => {
  pc = 0x48186d; continue;
 }
 0x481869 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x48186d; continue;
 }
 0x48186d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x48187b; continue;
 }
 0x48187b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48187c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048187f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48187f); }
 let mut pc: u32 = 0x48187f;
 loop { match pc {
 0x48187f => {
  { let v: u32 = 0x51e310u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x48188f; continue;
 }
 0x48188f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x4818a0; continue; }
  pc = 0x481897; continue;
 }
 0x481897 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48189e); f_004818ac(cpu); if cpu.returned_to != 0x48189e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4818a0; continue;
 }
 0x4818a0 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48188f; continue; }
  pc = 0x4818a5; continue;
 }
 0x4818a5 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004818ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4818ac); }
 let mut pc: u32 = 0x4818ac;
 loop { match pc {
 0x4818ac => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { pc = 0x481948; continue; }
  pc = 0x4818c2; continue;
 }
 0x4818c2 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x481948; continue; }
  pc = 0x4818cb; continue;
 }
 0x4818cb => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x481948; continue; }
  pc = 0x4818dd; continue;
 }
 0x4818dd => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x481948; continue; }
  pc = 0x4818e6; continue;
 }
 0x4818e6 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x481907; continue; }
  pc = 0x4818ef; continue;
 }
 0x4818ef => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x481907; continue; }
  pc = 0x4818f7; continue;
 }
 0x4818f7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x481910; continue; }
  pc = 0x481907; continue;
 }
 0x481907 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x481948; continue;
 }
 0x481910 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x481939; continue; }
  pc = 0x481930; continue;
 }
 0x481930 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5a2ae8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x481940; continue;
 }
 0x481939 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5a2afau32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x481940; continue;
 }
 0x481940 => {
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x481948; continue;
 }
 0x481948 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x48197b; continue; }
  pc = 0x481951; continue;
 }
 0x481951 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x481966; continue; }
  pc = 0x48195d; continue;
 }
 0x48195d => {
  { let v: u16 = cpu.r16(0x5a2af6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x48196d; continue;
 }
 0x481966 => {
  { let v: u16 = cpu.r16(0x5a2af4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x48196d; continue;
 }
 0x48196d => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x48197b; continue; }
  pc = 0x481974; continue;
 }
 0x481974 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48197b; continue;
 }
 0x48197b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004819ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4819ab); }
 let mut pc: u32 = 0x4819ab;
 loop { match pc {
 0x4819ab => {
  { let v: u32 = 0x51e3b0u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4819d9; continue; }
  pc = 0x4819be; continue;
 }
 0x4819be => {
  { cpu.sub8(cpu.r8(0x4e1e4au32), 0x0u8); }
  if !cpu.zf { pc = 0x4819d9; continue; }
  pc = 0x4819c7; continue;
 }
 0x4819c7 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x4819d9; continue; }
  pc = 0x4819d3; continue;
 }
 0x4819d3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x14a0u32); cpu.edi = v; }
  pc = 0x4819d9; continue;
 }
 0x4819d9 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4819dd; continue;
 }
 0x4819dd => {
  { cpu.scas(2, 0); }
  if cpu.zf { pc = 0x4819e8; continue; }
  pc = 0x4819e1; continue;
 }
 0x4819e1 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x2au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4819dd; continue; }
  pc = 0x4819e6; continue;
 }
 0x4819e6 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4819e8 => {
  { let v: u32 = cpu.sub32(cpu.edi, 0x2u32); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004819ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4819ed); }
 let mut pc: u32 = 0x4819ed;
 loop { match pc {
 0x4819ed => {
  { let v: u32 = 0x51e3b0u32; cpu.edi = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004819ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4819ff); }
 let mut pc: u32 = 0x4819ff;
 loop { match pc {
 0x4819ff => {
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x481a05); cpu.call(t); if cpu.returned_to != 0x481a05 { return; }
  { let v: u32 = 0x5261a0u32; cpu.ebp = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x481a0f; continue;
 }
 0x481a0f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x481a21; continue; }
  pc = 0x481a18; continue;
 }
 0x481a18 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x481a1f); f_00481a27(cpu); if cpu.returned_to != 0x481a1f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x481a21; continue;
 }
 0x481a21 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481a0f; continue; }
  pc = 0x481a26; continue;
 }
 0x481a26 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481a27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481a27); }
 let mut pc: u32 = 0x481a27;
 loop { match pc {
 0x481a27 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693eu32, v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x5d5eccu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x481a57); cpu.call(t); if cpu.returned_to != 0x481a57 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x481b5c; continue; }
  pc = 0x481a5e; continue;
 }
 0x481a5e => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x481a7b); f_00481d9b(cpu); if cpu.returned_to != 0x481a7b { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x5d5eccu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x481a8b); cpu.call(t); if cpu.returned_to != 0x481a8b { return; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x481aa7); f_00481d9b(cpu); if cpu.returned_to != 0x481aa7 { return; }
  { cpu.popad(); }
  pc = 0x481aa8; continue;
 }
 0x481aa8 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x481ac3); f_00481831(cpu); if cpu.returned_to != 0x481ac3 { return; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x60u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e8b32u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4ea5feu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x481b52; continue; }
  pc = 0x481b1e; continue;
 }
 0x481b1e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f78u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  cpu.push(0x481b52); f_0048419c(cpu); if cpu.returned_to != 0x481b52 { return; }
  pc = 0x481b52; continue;
 }
 0x481b52 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x481b54; continue;
 }
 0x481b54 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481b5c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x481b73); f_0047cc40(cpu); if cpu.returned_to != 0x481b73 { return; }
  if !cpu.cf { pc = 0x481b7c; continue; }
  pc = 0x481b75; continue;
 }
 0x481b75 => {
  cpu.push(0x481b7a); f_00481d88(cpu); if cpu.returned_to != 0x481b7a { return; }
  cpu.poll(); pc = 0x481b54; continue;
 }
 0x481b7c => {
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x481bc9; continue; }
  pc = 0x481bba; continue;
 }
 0x481bba => {
  if cpu.cf { cpu.poll(); pc = 0x481aa8; continue; }
  pc = 0x481bc0; continue;
 }
 0x481bc0 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x481aa8; continue; }
  pc = 0x481bc9; continue;
 }
 0x481bc9 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x481be5); f_00481d9b(cpu); if cpu.returned_to != 0x481be5 { return; }
  if cpu.cf { cpu.poll(); pc = 0x481b54; continue; }
  pc = 0x481beb; continue;
 }
 0x481beb => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x481bfd); f_00420989(cpu); if cpu.returned_to != 0x481bfd { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x481c14; continue; }
  pc = 0x481c12; continue;
 }
 0x481c12 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481c14 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x481c71; continue; }
  pc = 0x481c36; continue;
 }
 0x481c36 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x481c70; continue; }
  pc = 0x481c3f; continue;
 }
 0x481c3f => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x481c70; continue; }
  pc = 0x481c48; continue;
 }
 0x481c48 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x481c70; continue; }
  pc = 0x481c51; continue;
 }
 0x481c51 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x481c70; continue; }
  pc = 0x481c60; continue;
 }
 0x481c60 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x17u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d6fb4u32); cpu.push(0x481c70); cpu.call(t); if cpu.returned_to != 0x481c70 { return; }
  pc = 0x481c70; continue;
 }
 0x481c70 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481c71 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x481c70; continue; }
  pc = 0x481c82; continue;
 }
 0x481c82 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x481c70; continue; }
  pc = 0x481c8b; continue;
 }
 0x481c8b => {
  { let v: u16 = cpu.r16(0x4d6b9cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x481c70; continue; }
  pc = 0x481c9b; continue;
 }
 0x481c9b => {
  { let v: u16 = cpu.r16(0x4d6ba0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x481c70; continue; }
  pc = 0x481cab; continue;
 }
 0x481cab => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)) ^ 0x1u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x55fu32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  cpu.push(0x481cdc); f_004418e0(cpu); if cpu.returned_to != 0x481cdc { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481cdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481cdd); }
 let mut pc: u32 = 0x481cdd;
 loop { match pc {
 0x481cdd => {
  { cpu.pushad(); }
  { cpu.and32(cpu.ebx, 0x8u32); }
  if !cpu.zf { pc = 0x481d07; continue; }
  pc = 0x481ce6; continue;
 }
 0x481ce6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  pc = 0x481cf6; continue;
 }
 0x481cf6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e932fu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x481d2c; continue; }
  pc = 0x481d04; continue;
 }
 0x481d04 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481d07 => {
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e932fu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x481cf6; continue; }
  pc = 0x481d29; continue;
 }
 0x481d29 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481d2c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x481d73); f_0048277b(cpu); if cpu.returned_to != 0x481d73 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481d76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481d76); }
 let mut pc: u32 = 0x481d76;
 loop { match pc {
 0x481d76 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x50932au32.wrapping_add(cpu.eax))); cpu.w8(0x50932au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481d88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481d88); }
 let mut pc: u32 = 0x481d88;
 loop { match pc {
 0x481d88 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x50932au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481d9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481d9b); }
 let mut pc: u32 = 0x481d9b;
 loop { match pc {
 0x481d9b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x481dae); f_00481cdd(cpu); if cpu.returned_to != 0x481dae { return; }
  if !cpu.cf { pc = 0x481db7; continue; }
  pc = 0x481db0; continue;
 }
 0x481db0 => {
  cpu.push(0x481db5); f_00481d76(cpu); if cpu.returned_to != 0x481db5 { return; }
  pc = 0x481dfe; continue;
 }
 0x481db7 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x4d73f4u32; cpu.ecx = v; }
  pc = 0x481dcd; continue;
 }
 0x481dcd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x481de1; continue; }
  pc = 0x481ddd; continue;
 }
 0x481ddd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x481df1; continue;
 }
 0x481de1 => {
  cpu.push(0x481de6); f_00481e00(cpu); if cpu.returned_to != 0x481de6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x481df1; continue; }
  pc = 0x481dea; continue;
 }
 0x481dea => {
  cpu.push(0x481def); f_00481d88(cpu); if cpu.returned_to != 0x481def { return; }
  pc = 0x481dfe; continue;
 }
 0x481df1 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4d7406u32); }
  if cpu.cf { cpu.poll(); pc = 0x481dcd; continue; }
  pc = 0x481dfc; continue;
 }
 0x481dfc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481dfe => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481e00); }
 let mut pc: u32 = 0x481e00;
 loop { match pc {
 0x481e00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x55dcb8u32.wrapping_add(cpu.eax)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x481e76; continue; }
  pc = 0x481e0c; continue;
 }
 0x481e0c => {
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = 0x55bcb8u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x481e2e; continue;
 }
 0x481e2e => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x481e73; continue; }
  pc = 0x481e42; continue;
 }
 0x481e42 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x481e73; continue; }
  pc = 0x481e49; continue;
 }
 0x481e49 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x481e79; continue; }
  pc = 0x481e4f; continue;
 }
 0x481e4f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481e5b; continue; }
  pc = 0x481e58; continue;
 }
 0x481e58 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x481e5b; continue;
 }
 0x481e5b => {
  { cpu.sub16((cpu.edi as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x481e73; continue; }
  pc = 0x481e61; continue;
 }
 0x481e61 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebp as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481e6d; continue; }
  pc = 0x481e6a; continue;
 }
 0x481e6a => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x481e6d; continue;
 }
 0x481e6d => {
  { cpu.sub16((cpu.edi as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x481e9d; continue; }
  pc = 0x481e73; continue;
 }
 0x481e73 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x481e2e; continue; }
  pc = 0x481e76; continue;
 }
 0x481e76 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481e79 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481e85; continue; }
  pc = 0x481e82; continue;
 }
 0x481e82 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x481e85; continue;
 }
 0x481e85 => {
  { cpu.sub16((cpu.edi as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x481e73; continue; }
  pc = 0x481e8b; continue;
 }
 0x481e8b => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebp as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x481e97; continue; }
  pc = 0x481e94; continue;
 }
 0x481e94 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x481e97; continue;
 }
 0x481e97 => {
  { cpu.sub16((cpu.edi as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x481e73; continue; }
  pc = 0x481e9d; continue;
 }
 0x481e9d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d5f22u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd4u32, v); }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x481f05; continue; }
  pc = 0x481eb3; continue;
 }
 0x481eb3 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x481f73; continue; }
  pc = 0x481ebf; continue;
 }
 0x481ebf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if cpu.zf { pc = 0x481f73; continue; }
  pc = 0x481ec9; continue;
 }
 0x481ec9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x26u8); }
  if cpu.zf { pc = 0x481f73; continue; }
  pc = 0x481ed3; continue;
 }
 0x481ed3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x481f73; continue; }
  pc = 0x481edd; continue;
 }
 0x481edd => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x22u8); }
  if cpu.zf { pc = 0x481f05; continue; }
  pc = 0x481ee3; continue;
 }
 0x481ee3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1bu8); }
  if cpu.zf { pc = 0x481f05; continue; }
  pc = 0x481ee9; continue;
 }
 0x481ee9 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x481ef9; continue; }
  pc = 0x481ef4; continue;
 }
 0x481ef4 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x481efe; continue; }
  pc = 0x481ef9; continue;
 }
 0x481ef9 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x481efe; continue;
 }
 0x481efe => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.eax)), 0x3u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x481f73; continue; }
  pc = 0x481f05; continue;
 }
 0x481f05 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x481f1d); f_0041c743(cpu); if cpu.returned_to != 0x481f1d { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x1bu8); }
  if cpu.zf { pc = 0x481f2e; continue; }
  pc = 0x481f28; continue;
 }
 0x481f28 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x18u8); }
  if !cpu.zf { pc = 0x481f44; continue; }
  pc = 0x481f2e; continue;
 }
 0x481f2e => {
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x39u32.wrapping_add(cpu.ebx))); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.zf { pc = 0x481f44; continue; }
  pc = 0x481f40; continue;
 }
 0x481f40 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x481f73; continue;
 }
 0x481f44 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x481f73; continue; }
  pc = 0x481f4c; continue;
 }
 0x481f4c => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x481f57); f_0046a1b9(cpu); if cpu.returned_to != 0x481f57 { return; }
  if !cpu.sf { pc = 0x4820d7; continue; }
  pc = 0x481f5d; continue;
 }
 0x481f5d => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d5f20u32, v); }
  cpu.push(0x481f69); f_004694f0(cpu); if cpu.returned_to != 0x481f69 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5f20u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x481f73 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x481fa6; continue; }
  pc = 0x481f7d; continue;
 }
 0x481f7d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x481fa6; continue; }
  pc = 0x481f83; continue;
 }
 0x481f83 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x481fa6; continue; }
  pc = 0x481f89; continue;
 }
 0x481f89 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x481fa6; continue; }
  pc = 0x481f8f; continue;
 }
 0x481f8f => {
  { let v: u8 = cpu.sub8(((cpu.edx >> 8) as u8), cpu.r8(0x4d69e8u32)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x481fa6; continue; }
  pc = 0x481f97; continue;
 }
 0x481f97 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x481f9d); f_0041c743(cpu); if cpu.returned_to != 0x481f9d { return; }
  { cpu.and32(cpu.eax, 0x200u32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x481fa6; continue;
 }
 0x481fa6 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x481fb8); f_004679cb(cpu); if cpu.returned_to != 0x481fb8 { return; }
  if cpu.cf { pc = 0x481feb; continue; }
  pc = 0x481fba; continue;
 }
 0x481fba => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x481fe0; continue; }
  pc = 0x481fd0; continue;
 }
 0x481fd0 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x481fd7); f_00401130(cpu); if cpu.returned_to != 0x481fd7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x481fe0; continue;
 }
 0x481fe0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x481feb); f_00415d61(cpu); if cpu.returned_to != 0x481feb { return; }
  pc = 0x481feb; continue;
 }
 0x481feb => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.zf { pc = 0x48206e; continue; }
  pc = 0x482038; continue;
 }
 0x482038 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x48205b; continue; }
  pc = 0x482044; continue;
 }
 0x482044 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x482060; continue; }
  pc = 0x48204d; continue;
 }
 0x48204d => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x482065; continue; }
  pc = 0x482056; continue;
 }
 0x482056 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x482068; continue;
 }
 0x48205b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x482068; continue;
 }
 0x482060 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x482068; continue;
 }
 0x482065 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x482068; continue;
 }
 0x482068 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4820a2; continue;
 }
 0x48206e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x482091; continue; }
  pc = 0x48207a; continue;
 }
 0x48207a => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x482096; continue; }
  pc = 0x482083; continue;
 }
 0x482083 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x48209b; continue; }
  pc = 0x48208c; continue;
 }
 0x48208c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x48209e; continue;
 }
 0x482091 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x48209e; continue;
 }
 0x482096 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x48209e; continue;
 }
 0x48209b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x3u32); cpu.eax = v; }
  pc = 0x48209e; continue;
 }
 0x48209e => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4820a2; continue;
 }
 0x4820a2 => {
  cpu.push(0x4820a7); f_00484297(cpu); if cpu.returned_to != 0x4820a7 { return; }
  { cpu.popad(); }
  cpu.push(0x4820ad); f_0046a10d(cpu); if cpu.returned_to != 0x4820ad { return; }
  if !cpu.sf { pc = 0x4820d7; continue; }
  pc = 0x4820af; continue;
 }
 0x4820af => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4820d0; continue; }
  pc = 0x4820b6; continue;
 }
 0x4820b6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x4820d0; continue; }
  pc = 0x4820bd; continue;
 }
 0x4820bd => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x4820d0; continue; }
  pc = 0x4820c4; continue;
 }
 0x4820c4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x4820d0; continue; }
  pc = 0x4820cb; continue;
 }
 0x4820cb => {
  cpu.push(0x4820d0); f_00469347(cpu); if cpu.returned_to != 0x4820d0 { return; }
  pc = 0x4820d0; continue;
 }
 0x4820d0 => {
  { cpu.pushad(); }
  cpu.push(0x4820d6); f_004586f4(cpu); if cpu.returned_to != 0x4820d6 { return; }
  { cpu.popad(); }
  pc = 0x4820d7; continue;
 }
 0x4820d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004820da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4820da); }
 let mut pc: u32 = 0x4820da;
 loop { match pc {
 0x4820da => {
  cpu.push(0x4820df); f_004824a8(cpu); if cpu.returned_to != 0x4820df { return; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x4820e9; continue; }
  pc = 0x4820e3; continue;
 }
 0x4820e3 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x4820e9; continue; }
  pc = 0x4820e7; continue;
 }
 0x4820e7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4820e9 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x4820e7; continue; }
  pc = 0x4820ed; continue;
 }
 0x4820ed => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f2cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d5f30u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5d5f34u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d5f38u32, v); }
  cpu.push(0x482109); f_0048213c(cpu); if cpu.returned_to != 0x482109 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48211d); f_004821bf(cpu); if cpu.returned_to != 0x48211d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x4820e7; continue; }
  pc = 0x482124; continue;
 }
 0x482124 => {
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048213c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48213c); }
 let mut pc: u32 = 0x48213c;
 loop { match pc {
 0x48213c => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0x5ba17fu32; cpu.w32(0x4d6af0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x48215d); f_0046d630(cpu); if cpu.returned_to != 0x48215d { return; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x482187; continue; }
  pc = 0x482162; continue;
 }
 0x482162 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x482187; continue; }
  pc = 0x482172; continue;
 }
 0x482172 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15bdu32, v); }
  cpu.push(0x482187); f_004527fe(cpu); if cpu.returned_to != 0x482187 { return; }
  pc = 0x482187; continue;
 }
 0x482187 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482189(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482189); }
 let mut pc: u32 = 0x482189;
 loop { match pc {
 0x482189 => {
  { let v: u32 = 0x5261a0u32; cpu.ebp = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x482193; continue;
 }
 0x482193 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x4821a3; continue; }
  pc = 0x48219c; continue;
 }
 0x48219c => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x482193; continue; }
  pc = 0x4821a1; continue;
 }
 0x4821a1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4821a3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004821a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4821a5); }
 let mut pc: u32 = 0x4821a5;
 loop { match pc {
 0x4821a5 => {
  { let v: u32 = 0x5261a0u32; cpu.ebp = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x4821b4; continue;
 }
 0x4821b4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0x1cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4821b4; continue; }
  pc = 0x4821be; continue;
 }
 0x4821be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004821bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4821bf); }
 let mut pc: u32 = 0x4821bf;
 loop { match pc {
 0x4821bf => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4821f4); f_00482189(cpu); if cpu.returned_to != 0x4821f4 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x4821f8; continue; }
  pc = 0x4821f7; continue;
 }
 0x4821f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4821f8 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5d5ec8u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5d5f21u32), 0x0u8); }
  if cpu.zf { pc = 0x482284; continue; }
  pc = 0x482231; continue;
 }
 0x482231 => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d5f2cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d5f30u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d5f34u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d5f38u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.esi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x482260); f_0040d710(cpu); if cpu.returned_to != 0x482260 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  pc = 0x482284; continue;
 }
 0x482284 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d6934u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4822f1; continue; }
  pc = 0x48229d; continue;
 }
 0x48229d => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4822ac; continue; }
  pc = 0x4822a7; continue;
 }
 0x4822a7 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4822ac; continue;
 }
 0x4822ac => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x4822d3; continue; }
  pc = 0x4822b5; continue;
 }
 0x4822b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4822d3 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4822f1 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x482303; continue; }
  pc = 0x4822fe; continue;
 }
 0x4822fe => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x482303; continue;
 }
 0x482303 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x48232a; continue; }
  pc = 0x48230c; continue;
 }
 0x48230c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48232a => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482348(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482348); }
 let mut pc: u32 = 0x482348;
 loop { match pc {
 0x482348 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x482368; continue; }
  pc = 0x482365; continue;
 }
 0x482365 => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x482368; continue;
 }
 0x482368 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x482378; continue;
 }
 0x482378 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x482389; continue; }
  pc = 0x48237d; continue;
 }
 0x48237d => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  pc = 0x482389; continue;
 }
 0x482389 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x482378; continue; }
  pc = 0x48238d; continue;
 }
 0x48238d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004823a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4823a0); }
 let mut pc: u32 = 0x4823a0;
 loop { match pc {
 0x4823a0 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x4823c0; continue; }
  pc = 0x4823bd; continue;
 }
 0x4823bd => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4823c0; continue;
 }
 0x4823c0 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4823d0; continue;
 }
 0x4823d0 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4823dc; continue; }
  pc = 0x4823d5; continue;
 }
 0x4823d5 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4823dc; continue;
 }
 0x4823dc => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4823d0; continue; }
  pc = 0x4823e5; continue;
 }
 0x4823e5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004823f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4823f8); }
 let mut pc: u32 = 0x4823f8;
 loop { match pc {
 0x4823f8 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x482418; continue; }
  pc = 0x482415; continue;
 }
 0x482415 => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x482418; continue;
 }
 0x482418 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x482428; continue;
 }
 0x482428 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x482439; continue; }
  pc = 0x48242d; continue;
 }
 0x48242d => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  pc = 0x482439; continue;
 }
 0x482439 => {
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x482428; continue; }
  pc = 0x48243d; continue;
 }
 0x48243d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482450); }
 let mut pc: u32 = 0x482450;
 loop { match pc {
 0x482450 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x482470; continue; }
  pc = 0x48246d; continue;
 }
 0x48246d => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x482470; continue;
 }
 0x482470 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x482480; continue;
 }
 0x482480 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48248c; continue; }
  pc = 0x482485; continue;
 }
 0x482485 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48248c; continue;
 }
 0x48248c => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x482480; continue; }
  pc = 0x482495; continue;
 }
 0x482495 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004824a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4824a8); }
 let mut pc: u32 = 0x4824a8;
 loop { match pc {
 0x4824a8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4824cb; continue; }
  pc = 0x4824b7; continue;
 }
 0x4824b7 => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x1u8); }
  if cpu.zf { pc = 0x4824f8; continue; }
  pc = 0x4824c0; continue;
 }
 0x4824c0 => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x2u8); }
  if cpu.zf { pc = 0x4824ec; continue; }
  pc = 0x4824c9; continue;
 }
 0x4824c9 => {
  pc = 0x4824df; continue;
 }
 0x4824cb => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x1u8); }
  if cpu.zf { pc = 0x4824f8; continue; }
  pc = 0x4824d4; continue;
 }
 0x4824d4 => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x2u8); }
  if cpu.zf { pc = 0x4824ec; continue; }
  pc = 0x4824dd; continue;
 }
 0x4824dd => {
  pc = 0x4824df; continue;
 }
 0x4824df => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.eax); cpu.esi = v; }
  if cpu.zf { pc = 0x4824e5; continue; }
  pc = 0x4824e3; continue;
 }
 0x4824e3 => {
  if !cpu.sf { pc = 0x4824f8; continue; }
  pc = 0x4824e5; continue;
 }
 0x4824e5 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x4824f8; continue;
 }
 0x4824ec => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xfu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4824f8; continue; }
  pc = 0x4824f3; continue;
 }
 0x4824f3 => {
  { let v: u32 = 0xfu32; cpu.esi = v; }
  pc = 0x4824f8; continue;
 }
 0x4824f8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004824fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4824fa); }
 let mut pc: u32 = 0x4824fa;
 loop { match pc {
 0x4824fa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x78u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 2u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x482523; continue; }
  pc = 0x48250d; continue;
 }
 0x48250d => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x1u8); }
  if cpu.zf { pc = 0x482541; continue; }
  pc = 0x482516; continue;
 }
 0x482516 => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x2u8); }
  if cpu.zf { pc = 0x482539; continue; }
  pc = 0x48251f; continue;
 }
 0x48251f => {
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x482541; continue;
 }
 0x482523 => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x1u8); }
  if cpu.zf { pc = 0x482541; continue; }
  pc = 0x48252c; continue;
 }
 0x48252c => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x2u8); }
  if cpu.zf { pc = 0x482539; continue; }
  pc = 0x482535; continue;
 }
 0x482535 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x482541; continue;
 }
 0x482539 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48253f; continue; }
  pc = 0x48253d; continue;
 }
 0x48253d => {
  if !cpu.sf { pc = 0x482541; continue; }
  pc = 0x48253f; continue;
 }
 0x48253f => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x482541; continue;
 }
 0x482541 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004826aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4826aa); }
 let mut pc: u32 = 0x4826aa;
 loop { match pc {
 0x4826aa => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x5d5f84u32.wrapping_add(cpu.ebx)); cpu.push(0x4826b7); cpu.call(t); if cpu.returned_to != 0x4826b7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004826b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4826b8); }
 let mut pc: u32 = 0x4826b8;
 loop { match pc {
 0x4826b8 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  cpu.push(0x4826c2); f_0041c743(cpu); if cpu.returned_to != 0x4826c2 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4826d2); f_0041c743(cpu); if cpu.returned_to != 0x4826d2 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x12cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4826da; continue;
 }
 0x4826da => {
  cpu.push(0x4826df); f_0048479b(cpu); if cpu.returned_to != 0x4826df { return; }
  if cpu.cf { pc = 0x482772; continue; }
  pc = 0x4826e5; continue;
 }
 0x4826e5 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d67beu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482715); f_0041c743(cpu); if cpu.returned_to != 0x482715 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x5u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x5u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfcu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482760); f_0041c743(cpu); if cpu.returned_to != 0x482760 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4fu8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x482772; continue;
 }
 0x482772 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4826da; continue; }
  pc = 0x48277a; continue;
 }
 0x48277a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048277b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48277b); }
 let mut pc: u32 = 0x48277b;
 loop { match pc {
 0x48277b => {
  { let v: u32 = 0x48278au32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048278a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48278a); }
 let mut pc: u32 = 0x48278a;
 loop { match pc {
 0x48278a => {
  cpu.push(0x48278f); f_0041c743(cpu); if cpu.returned_to != 0x48278f { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48279a); f_0041c743(cpu); if cpu.returned_to != 0x48279a { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4827a7); f_0041c743(cpu); if cpu.returned_to != 0x4827a7 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4827c1); f_0041c743(cpu); if cpu.returned_to != 0x4827c1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x4d6960u32), 0x77au16); }
  if !cpu.zf { pc = 0x4827e6; continue; }
  pc = 0x4827de; continue;
 }
 0x4827de => {
  { let v: u32 = 0x482875u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4827e6; continue;
 }
 0x4827e6 => {
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004827f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4827f8); }
 let mut pc: u32 = 0x4827f8;
 loop { match pc {
 0x4827f8 => {
  { let v: u32 = 0x482807u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482807(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482807); }
 let mut pc: u32 = 0x482807;
 loop { match pc {
 0x482807 => {
  cpu.push(0x48280c); f_0041c743(cpu); if cpu.returned_to != 0x48280c { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482817); f_0041c743(cpu); if cpu.returned_to != 0x482817 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x482824); f_0041c743(cpu); if cpu.returned_to != 0x482824 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48283e); f_0041c743(cpu); if cpu.returned_to != 0x48283e { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x4d6960u32), 0x77au16); }
  if !cpu.zf { pc = 0x482863; continue; }
  pc = 0x48285b; continue;
 }
 0x48285b => {
  { let v: u32 = 0x482875u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x482863; continue;
 }
 0x482863 => {
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482875(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482875); }
 let mut pc: u32 = 0x482875;
 loop { match pc {
 0x482875 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x482892); f_00420989(cpu); if cpu.returned_to != 0x482892 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4829ab; continue; }
  pc = 0x4828a4; continue;
 }
 0x4828a4 => {
  { cpu.sub32(cpu.esi, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4829ab; continue; }
  pc = 0x4828b0; continue;
 }
 0x4828b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x4828c8; continue; }
  pc = 0x4828b8; continue;
 }
 0x4828b8 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x4828c8; continue; }
  pc = 0x4828bc; continue;
 }
 0x4828bc => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x4828c8; continue; }
  pc = 0x4828c0; continue;
 }
 0x4828c0 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x4828c8; continue; }
  pc = 0x4828c4; continue;
 }
 0x4828c4 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x482946; continue; }
  pc = 0x4828c8; continue;
 }
 0x4828c8 => {
  { cpu.pushad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x4828f6); f_004679cb(cpu); if cpu.returned_to != 0x4828f6 { return; }
  if cpu.cf { pc = 0x48291f; continue; }
  pc = 0x4828f8; continue;
 }
 0x4828f8 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48291e; continue; }
  pc = 0x48290e; continue;
 }
 0x48290e => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x482915); f_00401130(cpu); if cpu.returned_to != 0x482915 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48291e; continue;
 }
 0x48291e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48291f; continue;
 }
 0x48291f => {
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x482935); f_00467adc(cpu); if cpu.returned_to != 0x482935 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { cpu.popad(); }
  pc = 0x4829aa; continue;
 }
 0x482946 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x482964); f_00485e62(cpu); if cpu.returned_to != 0x482964 { return; }
  if cpu.cf { pc = 0x4829a2; continue; }
  pc = 0x482966; continue;
 }
 0x482966 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5a2b00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  pc = 0x4829a2; continue;
 }
 0x4829a2 => {
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4829aa; continue;
 }
 0x4829aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4829ab => {
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004829bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4829bb); }
 let mut pc: u32 = 0x4829bb;
 loop { match pc {
 0x4829bb => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x4829deu32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x4829d7); f_00483fc4(cpu); if cpu.returned_to != 0x4829d7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004829de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4829de); }
 let mut pc: u32 = 0x4829de;
 loop { match pc {
 0x4829de => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4829ec); f_0041c743(cpu); if cpu.returned_to != 0x4829ec { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x10u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482a03); f_0041c743(cpu); if cpu.returned_to != 0x482a03 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482a27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482a27); }
 let mut pc: u32 = 0x482a27;
 loop { match pc {
 0x482a27 => {
  { let v: u32 = 0x482a36u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482a36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482a36); }
 let mut pc: u32 = 0x482a36;
 loop { match pc {
 0x482a36 => {
  cpu.push(0x482a3b); f_004829de(cpu); if cpu.returned_to != 0x482a3b { return; }
  { let v: u8 = 0xf3u8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xbu32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) | 0x30u8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x482a7eu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3cu16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482a6b); f_0041c743(cpu); if cpu.returned_to != 0x482a6b { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482a7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x482a7e); } r_00482a7e(cpu, 0x482a7e); }
pub fn f_00482d4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x482d4c); } r_00482a7e(cpu, 0x482d4c); }
fn r_00482a7e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x482a7e => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x482a9b); f_00420989(cpu); if cpu.returned_to != 0x482a9b { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x482d98; continue; }
  pc = 0x482aad; continue;
 }
 0x482aad => {
  { cpu.sub32(cpu.esi, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x482d98; continue; }
  pc = 0x482ab9; continue;
 }
 0x482ab9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x482ad5; continue; }
  pc = 0x482ac1; continue;
 }
 0x482ac1 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x482ad5; continue; }
  pc = 0x482ac5; continue;
 }
 0x482ac5 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x482ad5; continue; }
  pc = 0x482ac9; continue;
 }
 0x482ac9 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x482ad5; continue; }
  pc = 0x482acd; continue;
 }
 0x482acd => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x482b5d; continue; }
  pc = 0x482ad5; continue;
 }
 0x482ad5 => {
  { cpu.pushad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x482b02); f_004679cb(cpu); if cpu.returned_to != 0x482b02 { return; }
  if cpu.cf { pc = 0x482b34; continue; }
  pc = 0x482b04; continue;
 }
 0x482b04 => {
  { cpu.sub8(cpu.r8(0x4eab2au32), 0x0u8); }
  if !cpu.zf { pc = 0x482b34; continue; }
  pc = 0x482b0d; continue;
 }
 0x482b0d => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x482b33; continue; }
  pc = 0x482b23; continue;
 }
 0x482b23 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x482b2a); f_00401130(cpu); if cpu.returned_to != 0x482b2a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x482b33; continue;
 }
 0x482b33 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x482b34; continue;
 }
 0x482b34 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  cpu.push(0x482b49); f_00467adc(cpu); if cpu.returned_to != 0x482b49 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  pc = 0x482d98; continue;
 }
 0x482b5d => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x167eu16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482b81; continue;
 }
 0x482b81 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x164au16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482b8e; continue;
 }
 0x482b8e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x621u16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482b97; continue;
 }
 0x482b97 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x1642u16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482ba0; continue;
 }
 0x482ba0 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x16cdu16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482ba9; continue;
 }
 0x482ba9 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x758u16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482bb2; continue;
 }
 0x482bb2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x14ecu16); }
  if cpu.zf { pc = 0x482bf3; continue; }
  pc = 0x482bbb; continue;
 }
 0x482bbb => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x201u16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482bc4; continue;
 }
 0x482bc4 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x14d8u16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482bcd; continue;
 }
 0x482bcd => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x740u16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482bd6; continue;
 }
 0x482bd6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x164bu16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482bdf; continue;
 }
 0x482bdf => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x1653u16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482be8; continue;
 }
 0x482be8 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x164fu16); }
  if cpu.zf { pc = 0x482c2c; continue; }
  pc = 0x482bf1; continue;
 }
 0x482bf1 => {
  pc = 0x482c63; continue;
 }
 0x482bf3 => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  cpu.push(0x482c01); f_004679cb(cpu); if cpu.returned_to != 0x482c01 { return; }
  if cpu.cf { pc = 0x482c63; continue; }
  pc = 0x482c03; continue;
 }
 0x482c03 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x482c29; continue; }
  pc = 0x482c19; continue;
 }
 0x482c19 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x482c20); f_00401130(cpu); if cpu.returned_to != 0x482c20 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x482c29; continue;
 }
 0x482c29 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x482c63; continue;
 }
 0x482c2c => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  cpu.push(0x482c3a); f_004679cb(cpu); if cpu.returned_to != 0x482c3a { return; }
  if cpu.cf { pc = 0x482c63; continue; }
  pc = 0x482c3c; continue;
 }
 0x482c3c => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x482c62; continue; }
  pc = 0x482c52; continue;
 }
 0x482c52 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x482c59); f_00401130(cpu); if cpu.returned_to != 0x482c59 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x482c62; continue;
 }
 0x482c62 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x482c63; continue;
 }
 0x482c63 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x482c6f); f_00467adc(cpu); if cpu.returned_to != 0x482c6f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  { let v: u16 = 0x1dau16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x15u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfbu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x22eu16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x482d2f; continue; }
  pc = 0x482d18; continue;
 }
 0x482d18 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x482d34; continue; }
  pc = 0x482d21; continue;
 }
 0x482d21 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x482d39; continue; }
  pc = 0x482d2a; continue;
 }
 0x482d2a => {
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  pc = 0x482d3c; continue;
 }
 0x482d2f => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x482d3c; continue;
 }
 0x482d34 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x482d3c; continue;
 }
 0x482d39 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x482d3c; continue;
 }
 0x482d3c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x482d43); f_004829bb(cpu); if cpu.returned_to != 0x482d43 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x482d83; continue; }
  pc = 0x482d6c; continue;
 }
 0x482d4c => {
  if !cpu.sf { pc = 0x482d6d; continue; }
  pc = 0x482d4e; continue;
 }
 0x482d4e => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4e1f7au32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffc7u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4d6960u32); cpu.eax = v; }
  { cpu.interrupts_off = false; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.rol8(cpu.r8(0x4e1fbc3du32.wrapping_add(cpu.eax)), 0u32); cpu.w8(0x4e1fbc3du32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffff80u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x482d6d; continue;
 }
 0x482d6c => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x482d88; continue; }
  pc = 0x482d75; continue;
 }
 0x482d6d => {
  { cpu.sub32(cpu.eax, 0x4e1fbcu32); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x4e1fbcu32); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0xffffff83u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 235u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax | 0xeb04c083u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3eb08c0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x3eb08c0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x482d8d; continue;
 }
 0x482d75 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x482d8d; continue; }
  pc = 0x482d7e; continue;
 }
 0x482d7e => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x482d90; continue;
 }
 0x482d83 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x482d90; continue;
 }
 0x482d88 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x482d90; continue;
 }
 0x482d8d => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x482d90; continue;
 }
 0x482d90 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x482d97); f_0048277b(cpu); if cpu.returned_to != 0x482d97 { return; }
  { cpu.popad(); }
  pc = 0x482d98; continue;
 }
 0x482d98 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482da9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482da9); }
 let mut pc: u32 = 0x482da9;
 loop { match pc {
 0x482da9 => {
  { let v: u32 = 0x482db8u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482db8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482db8); }
 let mut pc: u32 = 0x482db8;
 loop { match pc {
 0x482db8 => {
  cpu.push(0x482dbd); f_004829de(cpu); if cpu.returned_to != 0x482dbd { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x482de9u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482dd6); f_0041c743(cpu); if cpu.returned_to != 0x482dd6 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482de9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482de9); }
 let mut pc: u32 = 0x482de9;
 loop { match pc {
 0x482de9 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = cpu.r16(0x5a2b02u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  cpu.push(0x482e4b); f_0041c743(cpu); if cpu.returned_to != 0x482e4b { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x482e58); f_0048277b(cpu); if cpu.returned_to != 0x482e58 { return; }
  { let v: u32 = cpu.r16(0x5a2b04u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6960u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x482e87; continue; }
  pc = 0x482e70; continue;
 }
 0x482e70 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x482e8c; continue; }
  pc = 0x482e79; continue;
 }
 0x482e79 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x482e91; continue; }
  pc = 0x482e82; continue;
 }
 0x482e82 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x482e94; continue;
 }
 0x482e87 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x482e94; continue;
 }
 0x482e8c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x482e94; continue;
 }
 0x482e91 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x6u32); cpu.eax = v; }
  pc = 0x482e94; continue;
 }
 0x482e94 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x482e9b); f_0048277b(cpu); if cpu.returned_to != 0x482e9b { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482ea4); }
 let mut pc: u32 = 0x482ea4;
 loop { match pc {
 0x482ea4 => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x482ec7u32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x482ec0); f_00483fc4(cpu); if cpu.returned_to != 0x482ec0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482ec7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482ec7); }
 let mut pc: u32 = 0x482ec7;
 loop { match pc {
 0x482ec7 => {
  cpu.push(0x482ecc); f_0041c743(cpu); if cpu.returned_to != 0x482ecc { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482ed7); f_0041c743(cpu); if cpu.returned_to != 0x482ed7 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482efa); f_0041c743(cpu); if cpu.returned_to != 0x482efa { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x482f04; continue; }
  pc = 0x482f02; continue;
 }
 0x482f02 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x482f04; continue;
 }
 0x482f04 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x482de9u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482f28); f_0041c743(cpu); if cpu.returned_to != 0x482f28 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482f3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f3b); }
 let mut pc: u32 = 0x482f3b;
 loop { match pc {
 0x482f3b => {
  { let v: u32 = 0x482f4au32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482f4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f4a); }
 let mut pc: u32 = 0x482f4a;
 loop { match pc {
 0x482f4a => {
  cpu.push(0x482f4f); f_004829de(cpu); if cpu.returned_to != 0x482f4f { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4837c3u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482f68); f_0041c743(cpu); if cpu.returned_to != 0x482f68 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x482f78; continue; }
  pc = 0x482f73; continue;
 }
 0x482f73 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  pc = 0x482f78; continue;
 }
 0x482f78 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482f8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f8f); }
 let mut pc: u32 = 0x482f8f;
 loop { match pc {
 0x482f8f => {
  { let v: u32 = 0x482f9eu32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482f9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f9e); }
 let mut pc: u32 = 0x482f9e;
 loop { match pc {
 0x482f9e => {
  cpu.push(0x482fa3); f_004829de(cpu); if cpu.returned_to != 0x482fa3 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4835b1u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x482fbc); f_0041c743(cpu); if cpu.returned_to != 0x482fbc { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482fd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482fd9); }
 let mut pc: u32 = 0x482fd9;
 loop { match pc {
 0x482fd9 => {
  { let v: u32 = 0x482fe8u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482fe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482fe8); }
 let mut pc: u32 = 0x482fe8;
 loop { match pc {
 0x482fe8 => {
  cpu.push(0x482fed); f_004829de(cpu); if cpu.returned_to != 0x482fed { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4835b1u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483006); f_0041c743(cpu); if cpu.returned_to != 0x483006 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483023); }
 let mut pc: u32 = 0x483023;
 loop { match pc {
 0x483023 => {
  { cpu.pushad(); }
  pc = 0x483024; continue;
 }
 0x483024 => {
  cpu.push(0x483029); f_0048479b(cpu); if cpu.returned_to != 0x483029 { return; }
  if cpu.cf { pc = 0x4830e5; continue; }
  pc = 0x48302f; continue;
 }
 0x48302f => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48305d); f_0041c743(cpu); if cpu.returned_to != 0x48305d { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483074); f_0041c743(cpu); if cpu.returned_to != 0x483074 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4831bcu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x5a2b38u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4830bc); f_0041c743(cpu); if cpu.returned_to != 0x4830bc { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4830ca); f_0041c743(cpu); if cpu.returned_to != 0x4830ca { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4830d8); f_0041c743(cpu); if cpu.returned_to != 0x4830d8 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4830e5; continue;
 }
 0x4830e5 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x483024; continue; }
  pc = 0x4830ed; continue;
 }
 0x4830ed => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004831bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4831bc); }
 let mut pc: u32 = 0x4831bc;
 loop { match pc {
 0x4831bc => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x16u8; cpu.w8(0x4e1f78u32, v); }
  cpu.push(0x483213); f_0048408f(cpu); if cpu.returned_to != 0x483213 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048321c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48321c); }
 let mut pc: u32 = 0x48321c;
 loop { match pc {
 0x48321c => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x483259); f_0047a2d9(cpu); if cpu.returned_to != 0x483259 { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ecx as u8), 0x18u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4e1f78u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x48327d); f_004840ba(cpu); if cpu.returned_to != 0x48327d { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048327f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48327f); }
 let mut pc: u32 = 0x48327f;
 loop { match pc {
 0x48327f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x483286); f_0048479b(cpu); if cpu.returned_to != 0x483286 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4832ac); f_00420989(cpu); if cpu.returned_to != 0x4832ac { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x4832f1); f_0041c743(cpu); if cpu.returned_to != 0x4832f1 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x4832fe); f_0048277b(cpu); if cpu.returned_to != 0x4832fe { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483307(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483307); }
 let mut pc: u32 = 0x483307;
 loop { match pc {
 0x483307 => {
  { let v: u32 = 0x483316u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483316(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483316); }
 let mut pc: u32 = 0x483316;
 loop { match pc {
 0x483316 => {
  cpu.push(0x48331b); f_004829de(cpu); if cpu.returned_to != 0x48331b { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483351u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483334); f_0041c743(cpu); if cpu.returned_to != 0x483334 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483351(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483351); }
 let mut pc: u32 = 0x483351;
 loop { match pc {
 0x483351 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x483378); f_00469dbb(cpu); if cpu.returned_to != 0x483378 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48339e); f_00420989(cpu); if cpu.returned_to != 0x48339e { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48347c; continue; }
  pc = 0x4833b0; continue;
 }
 0x4833b0 => {
  { cpu.sub32(cpu.esi, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48347c; continue; }
  pc = 0x4833bc; continue;
 }
 0x4833bc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x4833d4; continue; }
  pc = 0x4833c4; continue;
 }
 0x4833c4 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x4833d4; continue; }
  pc = 0x4833c8; continue;
 }
 0x4833c8 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x4833d4; continue; }
  pc = 0x4833cc; continue;
 }
 0x4833cc => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x4833d4; continue; }
  pc = 0x4833d0; continue;
 }
 0x4833d0 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x483415; continue; }
  pc = 0x4833d4; continue;
 }
 0x4833d4 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x48340b); f_00467adc(cpu); if cpu.returned_to != 0x48340b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  pc = 0x48347b; continue;
 }
 0x483415 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x483466); f_0041c743(cpu); if cpu.returned_to != 0x483466 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x483473); f_0048277b(cpu); if cpu.returned_to != 0x483473 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48347b; continue;
 }
 0x48347b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48347c => {
  { cpu.popad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004835b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4835b1); }
 let mut pc: u32 = 0x4835b1;
 loop { match pc {
 0x4835b1 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x4835d8); f_00469dbb(cpu); if cpu.returned_to != 0x4835d8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4835fe); f_00420989(cpu); if cpu.returned_to != 0x4835fe { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4837b2; continue; }
  pc = 0x483610; continue;
 }
 0x483610 => {
  { cpu.sub32(cpu.esi, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4837b2; continue; }
  pc = 0x48361c; continue;
 }
 0x48361c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x483638; continue; }
  pc = 0x483624; continue;
 }
 0x483624 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x483638; continue; }
  pc = 0x483628; continue;
 }
 0x483628 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x483638; continue; }
  pc = 0x48362c; continue;
 }
 0x48362c => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x483638; continue; }
  pc = 0x483630; continue;
 }
 0x483630 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if !cpu.zf { pc = 0x4836b8; continue; }
  pc = 0x483638; continue;
 }
 0x483638 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x483668); f_004679cb(cpu); if cpu.returned_to != 0x483668 { return; }
  if cpu.cf { pc = 0x483691; continue; }
  pc = 0x48366a; continue;
 }
 0x48366a => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x483690; continue; }
  pc = 0x483680; continue;
 }
 0x483680 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483687); f_00401130(cpu); if cpu.returned_to != 0x483687 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x483690; continue;
 }
 0x483690 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x483691; continue;
 }
 0x483691 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x4836a4); f_00467adc(cpu); if cpu.returned_to != 0x4836a4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { cpu.popad(); }
  pc = 0x4837b1; continue;
 }
 0x4836b8 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x4836da); f_004679cb(cpu); if cpu.returned_to != 0x4836da { return; }
  if cpu.cf { pc = 0x483705; continue; }
  pc = 0x4836dc; continue;
 }
 0x4836dc => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x483704; continue; }
  pc = 0x4836f4; continue;
 }
 0x4836f4 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4836fb); f_00401130(cpu); if cpu.returned_to != 0x4836fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x483704; continue;
 }
 0x483704 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x483705; continue;
 }
 0x483705 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x483718); f_00467adc(cpu); if cpu.returned_to != 0x483718 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x483793; continue; }
  pc = 0x48377c; continue;
 }
 0x48377c => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x483798; continue; }
  pc = 0x483785; continue;
 }
 0x483785 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x48379d; continue; }
  pc = 0x48378e; continue;
 }
 0x48378e => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x4837a0; continue;
 }
 0x483793 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x4837a0; continue;
 }
 0x483798 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x4837a0; continue;
 }
 0x48379d => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x4837a0; continue;
 }
 0x4837a0 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x4837a9); f_0048277b(cpu); if cpu.returned_to != 0x4837a9 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4837b1; continue;
 }
 0x4837b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4837b2 => {
  { cpu.popad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004837c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4837c3); }
 let mut pc: u32 = 0x4837c3;
 loop { match pc {
 0x4837c3 => {
  cpu.push(0x4837c8); f_004679cb(cpu); if cpu.returned_to != 0x4837c8 { return; }
  if cpu.cf { pc = 0x483803; continue; }
  pc = 0x4837ca; continue;
 }
 0x4837ca => {
  { cpu.sub8(cpu.r8(0x4eab2au32), 0x0u8); }
  if !cpu.zf { pc = 0x483803; continue; }
  pc = 0x4837d3; continue;
 }
 0x4837d3 => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x4837fa; continue; }
  pc = 0x4837ea; continue;
 }
 0x4837ea => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4837f1); f_00401130(cpu); if cpu.returned_to != 0x4837f1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x4837fa; continue;
 }
 0x4837fa => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2au32, v); }
  pc = 0x483803; continue;
 }
 0x483803 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x32u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x48382a); f_00469dbb(cpu); if cpu.returned_to != 0x48382a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if cpu.zf { pc = 0x4838a0; continue; }
  pc = 0x483889; continue;
 }
 0x483889 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if cpu.zf { pc = 0x4838a5; continue; }
  pc = 0x483892; continue;
 }
 0x483892 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if cpu.zf { pc = 0x4838aa; continue; }
  pc = 0x48389b; continue;
 }
 0x48389b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x4838ad; continue;
 }
 0x4838a0 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x4838ad; continue;
 }
 0x4838a5 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x4838ad; continue;
 }
 0x4838aa => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x4838ad; continue;
 }
 0x4838ad => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x4838b5); f_0048277b(cpu); if cpu.returned_to != 0x4838b5 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004838be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4838be); }
 let mut pc: u32 = 0x4838be;
 loop { match pc {
 0x4838be => {
  { let v: u32 = 0x4838cdu32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004838cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4838cd); }
 let mut pc: u32 = 0x4838cd;
 loop { match pc {
 0x4838cd => {
  cpu.push(0x4838d2); f_004829de(cpu); if cpu.returned_to != 0x4838d2 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x3u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483922u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4838eb); f_0041c743(cpu); if cpu.returned_to != 0x4838eb { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48390c); f_0041c743(cpu); if cpu.returned_to != 0x48390c { return; }
  { let v: u8 = (cpu.eax as u8) & 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483919); f_0041c743(cpu); if cpu.returned_to != 0x483919 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483922(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483922); }
 let mut pc: u32 = 0x483922;
 loop { match pc {
 0x483922 => {
  cpu.push(0x483927); f_004679cb(cpu); if cpu.returned_to != 0x483927 { return; }
  if cpu.cf { pc = 0x483960; continue; }
  pc = 0x483929; continue;
 }
 0x483929 => {
  { cpu.sub8(cpu.r8(0x4eab2au32), 0x0u8); }
  if !cpu.zf { pc = 0x483960; continue; }
  pc = 0x483932; continue;
 }
 0x483932 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x483958; continue; }
  pc = 0x483948; continue;
 }
 0x483948 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48394f); f_00401130(cpu); if cpu.returned_to != 0x48394f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x483958; continue;
 }
 0x483958 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2au32, v); }
  pc = 0x483960; continue;
 }
 0x483960 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48397d); f_00420989(cpu); if cpu.returned_to != 0x48397d { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x483af0; continue; }
  pc = 0x48398f; continue;
 }
 0x48398f => {
  { cpu.sub32(cpu.esi, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x483af0; continue; }
  pc = 0x48399b; continue;
 }
 0x48399b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x4839b3; continue; }
  pc = 0x4839a3; continue;
 }
 0x4839a3 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x4839b3; continue; }
  pc = 0x4839a7; continue;
 }
 0x4839a7 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x4839b3; continue; }
  pc = 0x4839ab; continue;
 }
 0x4839ab => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x4839b3; continue; }
  pc = 0x4839af; continue;
 }
 0x4839af => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x4839f6; continue; }
  pc = 0x4839b3; continue;
 }
 0x4839b3 => {
  { cpu.pushad(); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x28u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x4839e9); f_00467adc(cpu); if cpu.returned_to != 0x4839e9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  pc = 0x483aef; continue;
 }
 0x4839f6 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x28u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  cpu.push(0x483a1d); f_00467adc(cpu); if cpu.returned_to != 0x483a1d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { cpu.popad(); }
  { let v: u16 = 0x1dau16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x15u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfbu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x22eu16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f8eu32, v); }
  cpu.push(0x483ab9); f_0041c743(cpu); if cpu.returned_to != 0x483ab9 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x483ac6); f_004829bb(cpu); if cpu.returned_to != 0x483ac6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0xfbu16; cpu.w16(0x4d6960u32, v); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x483aee); f_0048277b(cpu); if cpu.returned_to != 0x483aee { return; }
  { cpu.popad(); }
  pc = 0x483aef; continue;
 }
 0x483aef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x483af0 => {
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b00); }
 let mut pc: u32 = 0x483b00;
 loop { match pc {
 0x483b00 => {
  { let v: u32 = 0x483b0fu32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b0f); }
 let mut pc: u32 = 0x483b0f;
 loop { match pc {
 0x483b0f => {
  cpu.push(0x483b14); f_00482f4a(cpu); if cpu.returned_to != 0x483b14 { return; }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1cu16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b46); }
 let mut pc: u32 = 0x483b46;
 loop { match pc {
 0x483b46 => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x483b69u32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x483b62); f_00483fc4(cpu); if cpu.returned_to != 0x483b62 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b69); }
 let mut pc: u32 = 0x483b69;
 loop { match pc {
 0x483b69 => {
  cpu.push(0x483b6e); f_0041c743(cpu); if cpu.returned_to != 0x483b6e { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483b79); f_0041c743(cpu); if cpu.returned_to != 0x483b79 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483b9c); f_0041c743(cpu); if cpu.returned_to != 0x483b9c { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x483ba6; continue; }
  pc = 0x483ba4; continue;
 }
 0x483ba4 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x483ba6; continue;
 }
 0x483ba6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483bddu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483bca); f_0041c743(cpu); if cpu.returned_to != 0x483bca { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483bdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483bdd); }
 let mut pc: u32 = 0x483bdd;
 loop { match pc {
 0x483bdd => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x9u8; cpu.w8(0x4e1f78u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x483c34); f_0048408f(cpu); if cpu.returned_to != 0x483c34 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483c3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483c3d); }
 let mut pc: u32 = 0x483c3d;
 loop { match pc {
 0x483c3d => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f78u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  cpu.push(0x483c94); f_0048408f(cpu); if cpu.returned_to != 0x483c94 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483c9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483c9d); }
 let mut pc: u32 = 0x483c9d;
 loop { match pc {
 0x483c9d => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x483cc0u32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x483cb9); f_00483fc4(cpu); if cpu.returned_to != 0x483cb9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483cc0); }
 let mut pc: u32 = 0x483cc0;
 loop { match pc {
 0x483cc0 => {
  cpu.push(0x483cc5); f_0041c743(cpu); if cpu.returned_to != 0x483cc5 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483cd0); f_0041c743(cpu); if cpu.returned_to != 0x483cd0 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xau8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483cf1); f_0041c743(cpu); if cpu.returned_to != 0x483cf1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x483cf9; continue; }
  pc = 0x483cf7; continue;
 }
 0x483cf7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x483cf9; continue;
 }
 0x483cf9 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483c3du32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483d1d); f_0041c743(cpu); if cpu.returned_to != 0x483d1d { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483d30); }
 let mut pc: u32 = 0x483d30;
 loop { match pc {
 0x483d30 => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x483d53u32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x483d4c); f_00483fc4(cpu); if cpu.returned_to != 0x483d4c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483d53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483d53); }
 let mut pc: u32 = 0x483d53;
 loop { match pc {
 0x483d53 => {
  cpu.push(0x483d58); f_0041c743(cpu); if cpu.returned_to != 0x483d58 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483d63); f_0041c743(cpu); if cpu.returned_to != 0x483d63 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x14u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483d86); f_0041c743(cpu); if cpu.returned_to != 0x483d86 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x483d90; continue; }
  pc = 0x483d8e; continue;
 }
 0x483d8e => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x483d90; continue;
 }
 0x483d90 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483dc7u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483db4); f_0041c743(cpu); if cpu.returned_to != 0x483db4 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483dc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483dc7); }
 let mut pc: u32 = 0x483dc7;
 loop { match pc {
 0x483dc7 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x9u8; cpu.w8(0x4e1f78u32, v); }
  cpu.push(0x483e1e); f_0048408f(cpu); if cpu.returned_to != 0x483e1e { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483e27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483e27); }
 let mut pc: u32 = 0x483e27;
 loop { match pc {
 0x483e27 => {
  cpu.push(0x483e2c); f_0048479b(cpu); if cpu.returned_to != 0x483e2c { return; }
  if cpu.cf { pc = 0x483ed8; continue; }
  pc = 0x483e32; continue;
 }
 0x483e32 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xddcu16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xau8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x23u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x226u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483ec6); f_0041c743(cpu); if cpu.returned_to != 0x483ec6 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483ed8; continue;
 }
 0x483ed8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ed9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ed9); }
 let mut pc: u32 = 0x483ed9;
 loop { match pc {
 0x483ed9 => {
  { let v: u8 = cpu.r8(0x4e1f86u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u32 = 0x483efcu32; cpu.w32(0x4d6b40u32, v); }
  cpu.push(0x483ef5); f_00483fc4(cpu); if cpu.returned_to != 0x483ef5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f86u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483efc); }
 let mut pc: u32 = 0x483efc;
 loop { match pc {
 0x483efc => {
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483f06); f_0041c743(cpu); if cpu.returned_to != 0x483f06 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x1u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483f23); f_0041c743(cpu); if cpu.returned_to != 0x483f23 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x483f2d; continue; }
  pc = 0x483f2b; continue;
 }
 0x483f2b => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x483f2d; continue;
 }
 0x483f2d => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x483f64u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfau16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483f51); f_0041c743(cpu); if cpu.returned_to != 0x483f51 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483f64); }
 let mut pc: u32 = 0x483f64;
 loop { match pc {
 0x483f64 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d5f66u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f78u32, v); }
  cpu.push(0x483fbb); f_0048408f(cpu); if cpu.returned_to != 0x483fbb { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483fc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483fc4); }
 let mut pc: u32 = 0x483fc4;
 loop { match pc {
 0x483fc4 => {
  { cpu.sub8(cpu.r8(0x4e1f86u32), 0x0u8); }
  if !cpu.zf { pc = 0x48408e; continue; }
  pc = 0x483fd1; continue;
 }
 0x483fd1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  pc = 0x483fdd; continue;
 }
 0x483fdd => {
  cpu.push(0x483fe2); f_0048479b(cpu); if cpu.returned_to != 0x483fe2 { return; }
  if cpu.cf { pc = 0x48408e; continue; }
  pc = 0x483fe8; continue;
 }
 0x483fe8 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67beu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f79u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d6960u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f8eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f78u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5a463du32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5a4641u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5a4645u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x23u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x4d6b40u32); cpu.push(0x484087); cpu.call(t); if cpu.returned_to != 0x484087 { return; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x483fdd; continue; }
  pc = 0x48408e; continue;
 }
 0x48408e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048408f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48408f); }
 let mut pc: u32 = 0x48408f;
 loop { match pc {
 0x48408f => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if (cpu.sf == cpu.of) { f_004840ba(cpu); return; }
  pc = 0x48409d; continue;
 }
 0x48409d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x0u8); }
  if !cpu.zf { f_004840ba(cpu); return; }
  pc = 0x4840ab; continue;
 }
 0x4840ab => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x4840b5); f_0041c743(cpu); if cpu.returned_to != 0x4840b5 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  f_004840ba(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004840ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4840ba); }
 let mut pc: u32 = 0x4840ba;
 loop { match pc {
 0x4840ba => {
  { cpu.sub8(cpu.r8(0x4e1f86u32), 0x0u8); }
  if !cpu.zf { pc = 0x484193; continue; }
  pc = 0x4840c7; continue;
 }
 0x4840c7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x4840cb; continue;
 }
 0x4840cb => {
  cpu.push(0x4840d0); f_0048479b(cpu); if cpu.returned_to != 0x4840d0 { return; }
  if cpu.cf { pc = 0x484193; continue; }
  pc = 0x4840d6; continue;
 }
 0x4840d6 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4840f0); f_0041c743(cpu); if cpu.returned_to != 0x4840f0 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67beu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484111); f_0041c743(cpu); if cpu.returned_to != 0x484111 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x8u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484128); f_0041c743(cpu); if cpu.returned_to != 0x484128 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f79u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d6960u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484166); f_0041c743(cpu); if cpu.returned_to != 0x484166 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x2fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f78u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4840cb; continue; }
  pc = 0x484193; continue;
 }
 0x484193 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484194(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484194); }
 let mut pc: u32 = 0x484194;
 loop { match pc {
 0x484194 => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048419c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48419c); } r_0048419c(cpu, 0x48419c); }
pub fn f_004841e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4841e8); } r_0048419c(cpu, 0x4841e8); }
fn r_0048419c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48419c => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.sf { pc = 0x4841a6; continue; }
  pc = 0x4841a2; continue;
 }
 0x4841a2 => {
  if cpu.zf { pc = 0x4841a6; continue; }
  pc = 0x4841a4; continue;
 }
 0x4841a4 => {
  pc = 0x4841aa; continue;
 }
 0x4841a6 => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4841aa; continue;
 }
 0x4841aa => {
  { cpu.sub8(cpu.r8(0x4e1f86u32), 0x0u8); }
  if !cpu.zf { pc = 0x484296; continue; }
  pc = 0x4841b7; continue;
 }
 0x4841b7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x4841bb; continue;
 }
 0x4841bb => {
  cpu.push(0x4841c0); f_0048479b(cpu); if cpu.returned_to != 0x4841c0 { return; }
  if cpu.cf { pc = 0x484296; continue; }
  pc = 0x4841c6; continue;
 }
 0x4841c6 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4841e1); f_0041c743(cpu); if cpu.returned_to != 0x4841e1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67beu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48420a); f_0041c808(cpu); if cpu.returned_to != 0x48420a { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48421d); f_0041c808(cpu); if cpu.returned_to != 0x48421d { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484228); f_0041c743(cpu); if cpu.returned_to != 0x484228 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f79u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d6960u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x18u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x38u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48426e); f_0041c808(cpu); if cpu.returned_to != 0x48426e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f78u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4841bb; continue; }
  pc = 0x484296; continue;
 }
 0x4841e8 => {
  { let v: u32 = cpu.r32(0x4d67beu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d67c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48420a); f_0041c808(cpu); if cpu.returned_to != 0x48420a { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48421d); f_0041c808(cpu); if cpu.returned_to != 0x48421d { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484228); f_0041c743(cpu); if cpu.returned_to != 0x484228 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f79u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d6960u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x18u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x38u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x48426e); f_0041c808(cpu); if cpu.returned_to != 0x48426e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f78u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1f7bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4841bb; continue; }
  pc = 0x484296; continue;
 }
 0x484296 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484297(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484297); }
 let mut pc: u32 = 0x484297;
 loop { match pc {
 0x484297 => {
  { let v: u32 = 0x4842a6u32; cpu.w32(0x4d6b40u32, v); }
  f_00483fc4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004842a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4842a6); }
 let mut pc: u32 = 0x4842a6;
 loop { match pc {
 0x4842a6 => {
  cpu.push(0x4842ab); f_0041c743(cpu); if cpu.returned_to != 0x4842ab { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4842b6); f_0041c743(cpu); if cpu.returned_to != 0x4842b6 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4842c3); f_0041c743(cpu); if cpu.returned_to != 0x4842c3 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4842dd); f_0041c743(cpu); if cpu.returned_to != 0x4842dd { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x484194u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x40u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048430a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48430a); }
 let mut pc: u32 = 0x48430a;
 loop { match pc {
 0x48430a => {
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x484310); cpu.call(t); if cpu.returned_to != 0x484310 { return; }
  { let v: u16 = 0x1u16; cpu.w16(0x5d5f74u32, v); }
  { let v: u16 = 0x1d7u16; cpu.w16(0x5d5f76u32, v); }
  { let v: u16 = 0xff00u16; cpu.w16(0x5d5f78u32, v); }
  { let v: u16 = 0x352u16; cpu.w16(0x5d5f7cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d5f80u32, v); }
  { let v: u32 = 0x4efb50u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6c48u32); cpu.ecx = v; }
  pc = 0x484349; continue;
 }
 0x484349 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xfc18u16); }
  if !cpu.zf { pc = 0x484358; continue; }
  pc = 0x484352; continue;
 }
 0x484352 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x2cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x484349; continue; }
  pc = 0x484357; continue;
 }
 0x484357 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484358 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d5f80u32)); cpu.w32(0x5d5f80u32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x484758; continue; }
  pc = 0x48436a; continue;
 }
 0x48436a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d5f4cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d5f4eu32, v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d5f50u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x484391); f_00484909(cpu); if cpu.returned_to != 0x484391 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x484664; continue; }
  pc = 0x484398; continue;
 }
 0x484398 => {
  { let v: u16 = cpu.r16(0x5d5f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d5f66u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x484485; continue; }
  pc = 0x4843c8; continue;
 }
 0x4843c8 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x4843d5; continue;
 }
 0x4843d5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x4843e2; continue;
 }
 0x4843e2 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x48441b; continue; }
  pc = 0x4843e9; continue;
 }
 0x4843e9 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x484450; continue; }
  pc = 0x4843f0; continue;
 }
 0x4843f0 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x4843fd; continue;
 }
 0x4843fd => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x48440a; continue;
 }
 0x48440a => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f60u32); cpu.push(0x484415); cpu.call(t); if cpu.returned_to != 0x484415 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x484664; continue;
 }
 0x48441b => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x48442d; continue;
 }
 0x48442d => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x48443f; continue;
 }
 0x48443f => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f64u32); cpu.push(0x48444a); cpu.call(t); if cpu.returned_to != 0x48444a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x484664; continue;
 }
 0x484450 => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x484462; continue;
 }
 0x484462 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x484474; continue;
 }
 0x484474 => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6fc0u32); cpu.push(0x48447f); cpu.call(t); if cpu.returned_to != 0x48447f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x484664; continue;
 }
 0x484485 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0xfbu16); }
  if cpu.zf { pc = 0x48460b; continue; }
  pc = 0x484492; continue;
 }
 0x484492 => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x484514; continue; }
  pc = 0x484499; continue;
 }
 0x484499 => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48450f; continue; }
  pc = 0x4844a0; continue;
 }
 0x4844a0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x5d5f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d5f66u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x20u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x23u32.wrapping_add(cpu.ebp))); cpu.w8(0x23u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x22u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  pc = 0x48450f; continue;
 }
 0x48450f => {
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)) ^ 0x8u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484514; continue;
 }
 0x484514 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x484525; continue;
 }
 0x484525 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x484536; continue;
 }
 0x484536 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x46u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xc8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x48454c; continue;
 }
 0x48454c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x48455e; continue;
 }
 0x48455e => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x46u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.edi)); cpu.push(0x484578); cpu.call(t); if cpu.returned_to != 0x484578 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x4845e8; continue; }
  pc = 0x48458c; continue;
 }
 0x48458c => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x1u8); }
  if (cpu.cf || cpu.zf) { pc = 0x4845e8; continue; }
  pc = 0x484595; continue;
 }
 0x484595 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4ac410u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd0u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if (cpu.cf || cpu.zf) { pc = 0x4845d6; continue; }
  pc = 0x4845bb; continue;
 }
 0x4845bb => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x20u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6bd0u32, v); }
  pc = 0x4845d6; continue;
 }
 0x4845d6 => {
  let t = cpu.r32(0x4d6f44u32); cpu.push(0x4845dc); cpu.call(t); if cpu.returned_to != 0x4845dc { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd0u32, v); }
  pc = 0x4845ee; continue;
 }
 0x4845e8 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4845ee); cpu.call(t); if cpu.returned_to != 0x4845ee { return; }
  pc = 0x4845ee; continue;
 }
 0x4845ee => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x484664; continue; }
  pc = 0x4845f6; continue;
 }
 0x4845f6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x19u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x484604; continue;
 }
 0x484604 => {
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484664; continue;
 }
 0x48460b => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x484614; continue;
 }
 0x484614 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484664; continue; }
  pc = 0x48461d; continue;
 }
 0x48461d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x484626; continue;
 }
 0x484626 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x484664; continue; }
  pc = 0x48462f; continue;
 }
 0x48462f => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 5u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484647; continue; }
  pc = 0x484645; continue;
 }
 0x484645 => {
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x484647; continue;
 }
 0x484647 => {
  { let v: u16 = cpu.shl16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xfbu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x48465a); cpu.call(t); if cpu.returned_to != 0x48465a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484664; continue;
 }
 0x484664 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x5d5f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x484792; continue; }
  pc = 0x48468f; continue;
 }
 0x48468f => {
  { cpu.sub16(cpu.r16(0x4d678eu32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484792; continue; }
  pc = 0x48469c; continue;
 }
 0x48469c => {
  { let v: u16 = cpu.r16(0x5d5f66u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x484792; continue; }
  pc = 0x4846b1; continue;
 }
 0x4846b1 => {
  { cpu.sub16(cpu.r16(0x4d6790u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x484792; continue; }
  pc = 0x4846be; continue;
 }
 0x4846be => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.add16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4846e5; continue; }
  pc = 0x4846cb; continue;
 }
 0x4846cb => {
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x484758; continue; }
  pc = 0x4846d9; continue;
 }
 0x4846d9 => {
  { let v: u8 = cpu.neg8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x4846e5; continue; }
  pc = 0x4846e3; continue;
 }
 0x4846e3 => {
  pc = 0x484758; continue;
 }
 0x4846e5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0xeu32.wrapping_add(cpu.ebp))); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x484734; continue; }
  pc = 0x4846eb; continue;
 }
 0x4846eb => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x48470a; continue; }
  pc = 0x4846fe; continue;
 }
 0x4846fe => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48470a; continue; }
  pc = 0x484702; continue;
 }
 0x484702 => {
  if cpu.sf { pc = 0x484708; continue; }
  pc = 0x484704; continue;
 }
 0x484704 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48470a; continue;
 }
 0x484708 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48470a; continue;
 }
 0x48470a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x484725; continue; }
  pc = 0x484719; continue;
 }
 0x484719 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x484725; continue; }
  pc = 0x48471d; continue;
 }
 0x48471d => {
  if cpu.sf { pc = 0x484723; continue; }
  pc = 0x48471f; continue;
 }
 0x48471f => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x484725; continue;
 }
 0x484723 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x484725; continue;
 }
 0x484725 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x484734; continue; }
  pc = 0x484730; continue;
 }
 0x484730 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484734; continue;
 }
 0x484734 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x484752; continue; }
  pc = 0x48473a; continue;
 }
 0x48473a => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0xfu32.wrapping_add(cpu.ebp))); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfu32.wrapping_add(cpu.ebp)), 0x3fu8); }
  if !cpu.zf { pc = 0x484752; continue; }
  pc = 0x48474d; continue;
 }
 0x48474d => {
  { let v: u8 = 0x40u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484752; continue;
 }
 0x484752 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x484352; continue;
 }
 0x484758 => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x484777; continue; }
  pc = 0x48475f; continue;
 }
 0x48475f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x23u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x484777; continue;
 }
 0x484777 => {
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x48477b); cpu.call(t); if cpu.returned_to != 0x48477b { return; }
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x484752; continue; }
  pc = 0x484783; continue;
 }
 0x484783 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xfc18u16); }
  if !cpu.zf { cpu.poll(); pc = 0x48436a; continue; }
  pc = 0x484790; continue;
 }
 0x484790 => {
  cpu.poll(); pc = 0x484752; continue;
 }
 0x484792 => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x484752; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048479b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48479b); }
 let mut pc: u32 = 0x48479b;
 loop { match pc {
 0x48479b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4efb50u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x576e66u32); cpu.ecx = v; }
  pc = 0x4847a7; continue;
 }
 0x4847a7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xfc18u16); }
  if cpu.zf { pc = 0x4847b6; continue; }
  pc = 0x4847ae; continue;
 }
 0x4847ae => {
  { let v: u32 = cpu.add32(cpu.edi, 0x2cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4847a7; continue; }
  pc = 0x4847b3; continue;
 }
 0x4847b3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4847b6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004847b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4847b9); }
 let mut pc: u32 = 0x4847b9;
 loop { match pc {
 0x4847b9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4efb50u32; cpu.edi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x4847c4; continue;
 }
 0x4847c4 => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4847c4; continue; }
  pc = 0x4847ce; continue;
 }
 0x4847ce => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484909(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484909); }
 let mut pc: u32 = 0x484909;
 loop { match pc {
 0x484909 => {
  { let v: u32 = 0x5d5f4cu32; cpu.esi = v; }
  { let v: u32 = 0x5d5f64u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6adcu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5d5f7cu32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5d5f74u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f40u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5d5f76u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f44u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5d5f78u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f48u32, v); }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d5f44u32); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d5f44u32); cpu.eax = v; }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d5f48u32); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d5f44u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f48u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = 0x100u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5d5f40u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x4849cb; continue; }
  pc = 0x4849c6; continue;
 }
 0x4849c6 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4849cb; continue;
 }
 0x4849cb => {
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x5d5f44u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x64u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004849e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4849e8); }
 let mut pc: u32 = 0x4849e8;
 loop { match pc {
 0x4849e8 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d5fa8u32, v); }
  cpu.push(0x4849f9); f_0046ac64(cpu); if cpu.returned_to != 0x4849f9 { return; }
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484a08); f_00484a1f(cpu); if cpu.returned_to != 0x484a08 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5fa8u32, v); }
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484a1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484a1f); }
 let mut pc: u32 = 0x484a1f;
 loop { match pc {
 0x484a1f => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x484b25; continue; }
  pc = 0x484a2a; continue;
 }
 0x484a2a => {
  cpu.push(0x484a2f); f_004851dc(cpu); if cpu.returned_to != 0x484a2f { return; }
  cpu.push(0x484a34); f_004851f3(cpu); if cpu.returned_to != 0x484a34 { return; }
  cpu.push(0x484a39); f_0048520a(cpu); if cpu.returned_to != 0x484a39 { return; }
  cpu.push(0x484a3e); f_00485221(cpu); if cpu.returned_to != 0x484a3e { return; }
  { let v: u8 = cpu.r8(0x5d5fa9u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x5d5faau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x5d5fabu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x5d5facu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x484b25; continue; }
  pc = 0x484a5d; continue;
 }
 0x484a5d => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x484a73; continue; }
  pc = 0x484a6f; continue;
 }
 0x484a6f => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484a73; continue;
 }
 0x484a73 => {
  cpu.push(0x484a78); f_004824fa(cpu); if cpu.returned_to != 0x484a78 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x52bc02u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x576e9du32, v); }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x484a99; continue;
 }
 0x484a99 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if (cpu.sf == cpu.of) { pc = 0x484b24; continue; }
  pc = 0x484aa4; continue;
 }
 0x484aa4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))), 0xffu8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.zf { pc = 0x484abd; continue; }
  pc = 0x484ab9; continue;
 }
 0x484ab9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x484a99; continue;
 }
 0x484abd => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.mul8(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0xb0u32.wrapping_add(cpu.eax))); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xfu8); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x484adc; continue; }
  pc = 0x484adc; continue;
 }
 0x484adc => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x484aef; continue; }
  pc = 0x484ae6; continue;
 }
 0x484ae6 => {
  { cpu.sub8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xau8); }
  if (cpu.sf == cpu.of) { pc = 0x484b11; continue; }
  pc = 0x484aef; continue;
 }
 0x484aef => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x576e9du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x484afb); f_00484c51(cpu); if cpu.returned_to != 0x484afb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x576e9du32)); cpu.w8(0x576e9du32, v); }
  { cpu.sub8(cpu.r8(0x576e9du32), 0xcu8); }
  if cpu.zf { pc = 0x484b11; continue; }
  pc = 0x484b0d; continue;
 }
 0x484b0d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x484aef; continue; }
  pc = 0x484b11; continue;
 }
 0x484b11 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x576e9du32), 0xcu8); }
  if cpu.zf { pc = 0x484b24; continue; }
  pc = 0x484b1c; continue;
 }
 0x484b1c => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x484a99; continue; }
  pc = 0x484b24; continue;
 }
 0x484b24 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x484b25; continue;
 }
 0x484b25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484b26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484b26); }
 let mut pc: u32 = 0x484b26;
 loop { match pc {
 0x484b26 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x484c50; continue; }
  pc = 0x484b31; continue;
 }
 0x484b31 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5d6172u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d6176u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  cpu.push(0x484b56); f_00481755(cpu); if cpu.returned_to != 0x484b56 { return; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d627au32, v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x484b63; continue;
 }
 0x484b63 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x484b8a; continue; }
  pc = 0x484b67; continue;
 }
 0x484b67 => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x484b8a; continue; }
  pc = 0x484b70; continue;
 }
 0x484b70 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x484b8a; continue; }
  pc = 0x484b7a; continue;
 }
 0x484b7a => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8(cpu.r8(0x5d627au32)); cpu.w8(0x5d627au32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x484b63; continue; }
  pc = 0x484b8a; continue;
 }
 0x484b8a => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d627bu32, v); }
  { let v: u8 = cpu.r8(0x5d627au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x52bc02u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x484bad; continue;
 }
 0x484bad => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0xb0u32.wrapping_add(cpu.eax))); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xfu8); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x484bca; continue; }
  pc = 0x484bca; continue;
 }
 0x484bca => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x484bdd; continue; }
  pc = 0x484bd4; continue;
 }
 0x484bd4 => {
  { cpu.sub8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x484c20; continue; }
  pc = 0x484bdd; continue;
 }
 0x484bdd => {
  { let v = cpu.r32(0x5d6172u32); cpu.push(v); }
  { let v = cpu.r32(0x5d6176u32); cpu.push(v); }
  cpu.push(0x484bee); f_0041c743(cpu); if cpu.returned_to != 0x484bee { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d6172u32).wrapping_add(cpu.eax); cpu.w32(0x5d6172u32, v); }
  cpu.push(0x484bff); f_0041c743(cpu); if cpu.returned_to != 0x484bff { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d6176u32).wrapping_add(cpu.eax); cpu.w32(0x5d6176u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x484c13); f_00484c51(cpu); if cpu.returned_to != 0x484c13 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x5d6176u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x5d6172u32, v); }
  pc = 0x484c20; continue;
 }
 0x484c20 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x484bad; continue; }
  pc = 0x484c27; continue;
 }
 0x484c27 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d627bu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x484c3d; continue; }
  pc = 0x484c38; continue;
 }
 0x484c38 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x484c42; continue; }
  pc = 0x484c3d; continue;
 }
 0x484c3d => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x484c42; continue;
 }
 0x484c42 => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484c50); f_004824fa(cpu); if cpu.returned_to != 0x484c50 { return; }
  pc = 0x484c50; continue;
 }
 0x484c50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484c51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484c51); }
 let mut pc: u32 = 0x484c51;
 loop { match pc {
 0x484c51 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x484dde; continue; }
  pc = 0x484c5a; continue;
 }
 0x484c5a => {
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x484dde; continue; }
  pc = 0x484c63; continue;
 }
 0x484c63 => {
  { cpu.sub8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x484dde; continue; }
  pc = 0x484c6c; continue;
 }
 0x484c6c => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x484cb2; continue; }
  pc = 0x484c71; continue;
 }
 0x484c71 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x484cb2; continue; }
  pc = 0x484c76; continue;
 }
 0x484c76 => {
  { cpu.sub8((cpu.ecx as u8), 0x9u8); }
  if cpu.zf { pc = 0x484cb2; continue; }
  pc = 0x484c7b; continue;
 }
 0x484c7b => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x484e76; continue; }
  pc = 0x484c84; continue;
 }
 0x484c84 => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x484e76; continue; }
  pc = 0x484c8d; continue;
 }
 0x484c8d => {
  { cpu.sub8((cpu.ecx as u8), 0xau8); }
  if cpu.zf { pc = 0x484e76; continue; }
  pc = 0x484c96; continue;
 }
 0x484c96 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x484d4a; continue; }
  pc = 0x484c9f; continue;
 }
 0x484c9f => {
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if cpu.zf { pc = 0x484d4a; continue; }
  pc = 0x484ca8; continue;
 }
 0x484ca8 => {
  { cpu.sub8((cpu.ecx as u8), 0xbu8); }
  if cpu.zf { pc = 0x484d4a; continue; }
  pc = 0x484cb1; continue;
 }
 0x484cb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484cb2 => {
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if !cpu.zf { pc = 0x484cc8; continue; }
  pc = 0x484cbb; continue;
 }
 0x484cbb => {
  { cpu.sub8(cpu.r8(0x5d5fa9u32), 0xffu8); }
  if cpu.zf { pc = 0x484d49; continue; }
  pc = 0x484cc8; continue;
 }
 0x484cc8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5d6010u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d600eu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if cpu.zf { pc = 0x484d2e; continue; }
  pc = 0x484d12; continue;
 }
 0x484d12 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x484f0a; continue; }
  pc = 0x484d20; continue;
 }
 0x484d20 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x484f7d; continue; }
  pc = 0x484d2e; continue;
 }
 0x484d2e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x5d5fa9u32); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x484d46); f_00484fc7(cpu); if cpu.returned_to != 0x484d46 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x484d49; continue;
 }
 0x484d49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484d4a => {
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if !cpu.zf { pc = 0x484d5c; continue; }
  pc = 0x484d53; continue;
 }
 0x484d53 => {
  { cpu.sub8(cpu.r8(0x5d5faau32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x484d49; continue; }
  pc = 0x484d5c; continue;
 }
 0x484d5c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5d5fb0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d5faeu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if cpu.zf { pc = 0x484dc2; continue; }
  pc = 0x484da6; continue;
 }
 0x484da6 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x484f0a; continue; }
  pc = 0x484db4; continue;
 }
 0x484db4 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x484f7d; continue; }
  pc = 0x484dc2; continue;
 }
 0x484dc2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x5d5faau32); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x484dda); f_00484fc7(cpu); if cpu.returned_to != 0x484dda { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484dde => {
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if !cpu.zf { pc = 0x484df4; continue; }
  pc = 0x484de7; continue;
 }
 0x484de7 => {
  { cpu.sub8(cpu.r8(0x5d5fabu32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x484d49; continue; }
  pc = 0x484df4; continue;
 }
 0x484df4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5d6070u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d606eu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if cpu.zf { pc = 0x484e5a; continue; }
  pc = 0x484e3e; continue;
 }
 0x484e3e => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x484f0a; continue; }
  pc = 0x484e4c; continue;
 }
 0x484e4c => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x484f7d; continue; }
  pc = 0x484e5a; continue;
 }
 0x484e5a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x5d5fabu32); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x484e72); f_00484fc7(cpu); if cpu.returned_to != 0x484e72 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484e76 => {
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if !cpu.zf { pc = 0x484e8c; continue; }
  pc = 0x484e7f; continue;
 }
 0x484e7f => {
  { cpu.sub8(cpu.r8(0x5d5facu32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x484d49; continue; }
  pc = 0x484e8c; continue;
 }
 0x484e8c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5d60d0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d60ceu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d627bu32), 0x0u8); }
  if cpu.zf { pc = 0x484eee; continue; }
  pc = 0x484ed6; continue;
 }
 0x484ed6 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x484f0a; continue; }
  pc = 0x484ee0; continue;
 }
 0x484ee0 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x484f7d; continue; }
  pc = 0x484eee; continue;
 }
 0x484eee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x5d5facu32); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x484f06); f_00484fc7(cpu); if cpu.returned_to != 0x484f06 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x484f0a => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5d5fadu32, v); }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x484f32; continue; }
  pc = 0x484f2d; continue;
 }
 0x484f2d => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x484f37; continue; }
  pc = 0x484f32; continue;
 }
 0x484f32 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x484f37; continue;
 }
 0x484f37 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x5d6172u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d6176u32); cpu.edx = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x5d5f23u32, v); }
  cpu.push(0x484f76); f_00480e87(cpu); if cpu.returned_to != 0x484f76 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x484d49; continue;
 }
 0x484f7d => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5d5fadu32, v); }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x5d6172u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d6176u32); cpu.edx = v; }
  cpu.push(0x484fc0); f_004810cb(cpu); if cpu.returned_to != 0x484fc0 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x484d49; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484fc7); }
 let mut pc: u32 = 0x484fc7;
 loop { match pc {
 0x484fc7 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5d5fadu32, v); }
  cpu.push(0x484fd6); f_004679cb(cpu); if cpu.returned_to != 0x484fd6 { return; }
  if cpu.cf { pc = 0x485056; continue; }
  pc = 0x484fd8; continue;
 }
 0x484fd8 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x485056; continue; }
  pc = 0x484fe2; continue;
 }
 0x484fe2 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x485056; continue; }
  pc = 0x484fec; continue;
 }
 0x484fec => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x48501f; continue; }
  pc = 0x484ff6; continue;
 }
 0x484ff6 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48501c; continue; }
  pc = 0x48500c; continue;
 }
 0x48500c => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485013); f_00401130(cpu); if cpu.returned_to != 0x485013 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48501c; continue;
 }
 0x48501c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x485056; continue;
 }
 0x48501f => {
  { cpu.sub8(cpu.r8(0x4eab2bu32), 0x0u8); }
  if !cpu.zf { pc = 0x485056; continue; }
  pc = 0x485028; continue;
 }
 0x485028 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48504e; continue; }
  pc = 0x48503e; continue;
 }
 0x48503e => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485045); f_00401130(cpu); if cpu.returned_to != 0x485045 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48504e; continue;
 }
 0x48504e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2bu32, v); }
  pc = 0x485056; continue;
 }
 0x485056 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x485064); f_00452a56(cpu); if cpu.returned_to != 0x485064 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x42u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x44u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x485092; continue; }
  pc = 0x48508d; continue;
 }
 0x48508d => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x485097; continue; }
  pc = 0x485092; continue;
 }
 0x485092 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x485097; continue;
 }
 0x485097 => {
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x485105; continue; }
  pc = 0x4850b0; continue;
 }
 0x4850b0 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x485155; continue; }
  pc = 0x4850be; continue;
 }
 0x4850be => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4850d7; continue; }
  pc = 0x4850c8; continue;
 }
 0x4850c8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d5f21u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4850d7; continue;
 }
 0x4850d7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d5ec8u32, v); }
  cpu.push(0x4850e7); f_004820da(cpu); if cpu.returned_to != 0x4850e7 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5f21u32, v); }
  { let v: u32 = cpu.r32(0x5d5ec8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4850fb; continue; }
  pc = 0x4850f8; continue;
 }
 0x4850f8 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xbu32.wrapping_add(cpu.eax))); cpu.w8(0xbu32.wrapping_add(cpu.eax), v); }
  pc = 0x4850fb; continue;
 }
 0x4850fb => {
  { let v: u8 = cpu.r8(0x5d5fadu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x485105 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x485114; continue; }
  pc = 0x48510f; continue;
 }
 0x48510f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x485119; continue; }
  pc = 0x485114; continue;
 }
 0x485114 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x485119; continue;
 }
 0x485119 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1du8; cpu.w8(0x5d5f23u32, v); }
  cpu.push(0x485152); f_00480e87(cpu); if cpu.returned_to != 0x485152 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x4850fb; continue;
 }
 0x485155 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5d5fadu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6150u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d614eu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6934u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x4d6936u32), (cpu.ebx as u16)); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48519a); f_004810cb(cpu); if cpu.returned_to != 0x48519a { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x4850fb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851dc); }
 let mut pc: u32 = 0x4851dc;
 loop { match pc {
 0x4851dc => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d5fa9u32, v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4851ea); f_00485238(cpu); if cpu.returned_to != 0x4851ea { return; }
  if !cpu.cf { pc = 0x4851f2; continue; }
  pc = 0x4851ec; continue;
 }
 0x4851ec => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d5fa9u32, v); }
  pc = 0x4851f2; continue;
 }
 0x4851f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851f3); }
 let mut pc: u32 = 0x4851f3;
 loop { match pc {
 0x4851f3 => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d5faau32, v); }
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x485201); f_00485238(cpu); if cpu.returned_to != 0x485201 { return; }
  if !cpu.cf { pc = 0x485209; continue; }
  pc = 0x485203; continue;
 }
 0x485203 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d5faau32, v); }
  pc = 0x485209; continue;
 }
 0x485209 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048520a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48520a); }
 let mut pc: u32 = 0x48520a;
 loop { match pc {
 0x48520a => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d5fabu32, v); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x485218); f_00485238(cpu); if cpu.returned_to != 0x485218 { return; }
  if !cpu.cf { pc = 0x485220; continue; }
  pc = 0x48521a; continue;
 }
 0x48521a => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d5fabu32, v); }
  pc = 0x485220; continue;
 }
 0x485220 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485221); }
 let mut pc: u32 = 0x485221;
 loop { match pc {
 0x485221 => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d5facu32, v); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48522f); f_00485238(cpu); if cpu.returned_to != 0x48522f { return; }
  if !cpu.cf { pc = 0x485237; continue; }
  pc = 0x485231; continue;
 }
 0x485231 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d5facu32, v); }
  pc = 0x485237; continue;
 }
 0x485237 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485238); }
 let mut pc: u32 = 0x485238;
 loop { match pc {
 0x485238 => {
  cpu.push(0x48523d); f_0043fede(cpu); if cpu.returned_to != 0x48523d { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x4852a0; continue; }
  pc = 0x485242; continue;
 }
 0x485242 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x4d6934u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x4d6936u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x48526c); f_004539be(cpu); if cpu.returned_to != 0x48526c { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x48527f); f_0046c3d4(cpu); if cpu.returned_to != 0x48527f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x4852a0; continue; }
  pc = 0x485290; continue;
 }
 0x485290 => {
  cpu.push(0x485295); f_004782f8(cpu); if cpu.returned_to != 0x485295 { return; }
  if cpu.cf { pc = 0x4852a0; continue; }
  pc = 0x485297; continue;
 }
 0x485297 => {
  cpu.push(0x48529c); f_0047a472(cpu); if cpu.returned_to != 0x48529c { return; }
  if cpu.cf { pc = 0x4852a0; continue; }
  pc = 0x48529e; continue;
 }
 0x48529e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4852a0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004852a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4852a4); }
 let mut pc: u32 = 0x4852a4;
 loop { match pc {
 0x4852a4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x4852b6; continue;
 }
 0x4852b6 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4852cd; continue; }
  pc = 0x4852bd; continue;
 }
 0x4852bd => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x4852c5; continue; }
  pc = 0x4852c3; continue;
 }
 0x4852c3 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x4852c5; continue;
 }
 0x4852c5 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4852cd; continue; }
  pc = 0x4852cb; continue;
 }
 0x4852cb => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4852cd; continue;
 }
 0x4852cd => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4852b6; continue; }
  pc = 0x4852d3; continue;
 }
 0x4852d3 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4852e2; continue; }
  pc = 0x4852df; continue;
 }
 0x4852df => {
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  pc = 0x4852e2; continue;
 }
 0x4852e2 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004852ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4852ec); }
 let mut pc: u32 = 0x4852ec;
 loop { match pc {
 0x4852ec => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4852f6; continue; }
  pc = 0x4852f5; continue;
 }
 0x4852f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4852f6 => {
  cpu.push(0x4852fb); f_004852a4(cpu); if cpu.returned_to != 0x4852fb { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x524a4cu32) as u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v = cpu.r16(0x58c83au32); cpu.mul16(v); }
  { let v: u32 = 0x2710u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.r8(0x524a4cu32) as u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x58c886u32), 0x0u8); }
  if !cpu.zf { pc = 0x485333; continue; }
  pc = 0x48532a; continue;
 }
 0x48532a => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48533f; continue; }
  pc = 0x48532f; continue;
 }
 0x48532f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x48533f; continue;
 }
 0x485333 => {
  { cpu.sub8(cpu.r8(0x58c886u32), 0x2u8); }
  if !cpu.zf { pc = 0x48533f; continue; }
  pc = 0x48533c; continue;
 }
 0x48533c => {
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48533f; continue;
 }
 0x48533f => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.cf || cpu.zf) { pc = 0x485349; continue; }
  pc = 0x485344; continue;
 }
 0x485344 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x485349; continue;
 }
 0x485349 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x58c885u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x28u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x485371; continue; }
  pc = 0x485355; continue;
 }
 0x485355 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.cf || cpu.zf) { pc = 0x485365; continue; }
  pc = 0x485360; continue;
 }
 0x485360 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x485365; continue;
 }
 0x485365 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5d627cu32, v); }
  cpu.push(0x485370); f_004578df(cpu); if cpu.returned_to != 0x485370 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x485371 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5d627cu32, v); }
  cpu.push(0x48537c); f_004578e9(cpu); if cpu.returned_to != 0x48537c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048537d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48537d); }
 let mut pc: u32 = 0x48537d;
 loop { match pc {
 0x48537d => {
  { let v: u8 = 0x1u8; cpu.w8(0x581b4au32, v); }
  { cpu.sub8(cpu.r8(0x5d627cu32), 0x7fu8); }
  if cpu.zf { pc = 0x4853d4; continue; }
  pc = 0x48538d; continue;
 }
 0x48538d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let a = cpu.r16(0x4d6756u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x4d6756u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48539e); f_0041c743(cpu); if cpu.returned_to != 0x48539e { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x5d627cu32)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x4853b1); f_0041c808(cpu); if cpu.returned_to != 0x4853b1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6756u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 7u32); cpu.eax = v; }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4853ce; continue; }
  pc = 0x4853cc; continue;
 }
 0x4853cc => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x4853ce; continue;
 }
 0x4853ce => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x4853d4; continue;
 }
 0x4853d4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b4au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004853dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4853dc); }
 let mut pc: u32 = 0x4853dc;
 loop { match pc {
 0x4853dc => {
  { cpu.and8(cpu.r8(0x1du32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4853e7; continue; }
  pc = 0x4853e2; continue;
 }
 0x4853e2 => {
  cpu.push(0x4853e7); f_0048537d(cpu); if cpu.returned_to != 0x4853e7 { return; }
  pc = 0x4853e7; continue;
 }
 0x4853e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004853e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4853e8); }
 let mut pc: u32 = 0x4853e8;
 loop { match pc {
 0x4853e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x485449; continue; }
  pc = 0x4853fa; continue;
 }
 0x4853fa => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x48540d); f_00478632(cpu); if cpu.returned_to != 0x48540d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf4u32, v); }
  { cpu.popad(); }
  pc = 0x485413; continue;
 }
 0x485413 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x485432; continue; }
  pc = 0x485418; continue;
 }
 0x485418 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48542d); f_00485452(cpu); if cpu.returned_to != 0x48542d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x485432; continue;
 }
 0x485432 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x485413; continue; }
  pc = 0x485437; continue;
 }
 0x485437 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x485449; continue; }
  pc = 0x485441; continue;
 }
 0x485441 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x485449; continue; }
  pc = 0x485446; continue;
 }
 0x485446 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x485449; continue;
 }
 0x485449 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
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
pub fn f_00485452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485452); }
 let mut pc: u32 = 0x485452;
 loop { match pc {
 0x485452 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5d6280u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d7u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x48548d; continue; }
  pc = 0x48547d; continue;
 }
 0x48547d => {
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48548d; continue;
 }
 0x48548d => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4854a5; continue; }
  pc = 0x485495; continue;
 }
 0x485495 => {
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4854a5; continue;
 }
 0x4854a5 => {
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4854bd; continue; }
  pc = 0x4854ad; continue;
 }
 0x4854ad => {
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4854bd; continue;
 }
 0x4854bd => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4854d5; continue; }
  pc = 0x4854c5; continue;
 }
 0x4854c5 => {
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4854d5; continue;
 }
 0x4854d5 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4854ed; continue; }
  pc = 0x4854dd; continue;
 }
 0x4854dd => {
  { let v = cpu.r8(0x3au32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4854ed; continue;
 }
 0x4854ed => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485505; continue; }
  pc = 0x4854f5; continue;
 }
 0x4854f5 => {
  { let v = cpu.r8(0x3eu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485505; continue;
 }
 0x485505 => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x48551d; continue; }
  pc = 0x48550d; continue;
 }
 0x48550d => {
  { let v = cpu.r8(0x77u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48551d; continue;
 }
 0x48551d => {
  { let v: u32 = cpu.r32(0x5887d3u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48552e; continue; }
  pc = 0x485526; continue;
 }
 0x485526 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x5887d7u32); cpu.div32(v); }
  pc = 0x48552e; continue;
 }
 0x48552e => {
  { let v: u32 = cpu.eax; cpu.w32(0x5887dbu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5d62ccu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d7u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4856c0; continue; }
  pc = 0x485562; continue;
 }
 0x485562 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x4856c0; continue; }
  pc = 0x48556d; continue;
 }
 0x48556d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x4856c0; continue; }
  pc = 0x485578; continue;
 }
 0x485578 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x4856c0; continue; }
  pc = 0x485583; continue;
 }
 0x485583 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x485591; continue; }
  pc = 0x48558c; continue;
 }
 0x48558c => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x485596; continue; }
  pc = 0x485591; continue;
 }
 0x485591 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x485596; continue;
 }
 0x485596 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4855ad; continue; }
  pc = 0x48559e; continue;
 }
 0x48559e => {
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4855ad; continue;
 }
 0x4855ad => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4855c4; continue; }
  pc = 0x4855b5; continue;
 }
 0x4855b5 => {
  { let v = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4855c4; continue;
 }
 0x4855c4 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4855db; continue; }
  pc = 0x4855cc; continue;
 }
 0x4855cc => {
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4855db; continue;
 }
 0x4855db => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4855f2; continue; }
  pc = 0x4855e3; continue;
 }
 0x4855e3 => {
  { let v = cpu.r8(0x21u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4855f2; continue;
 }
 0x4855f2 => {
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485609; continue; }
  pc = 0x4855fa; continue;
 }
 0x4855fa => {
  { let v = cpu.r8(0x23u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485609; continue;
 }
 0x485609 => {
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485620; continue; }
  pc = 0x485611; continue;
 }
 0x485611 => {
  { let v = cpu.r8(0x24u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485620; continue;
 }
 0x485620 => {
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485637; continue; }
  pc = 0x485628; continue;
 }
 0x485628 => {
  { let v = cpu.r8(0x25u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485637; continue;
 }
 0x485637 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485651; continue; }
  pc = 0x48563c; continue;
 }
 0x48563c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485651; continue;
 }
 0x485651 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x48566d; continue; }
  pc = 0x485657; continue;
 }
 0x485657 => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0x26u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48566d; continue;
 }
 0x48566d => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x48568d; continue; }
  pc = 0x485673; continue;
 }
 0x485673 => {
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.edx); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48568d; continue;
 }
 0x48568d => {
  { let v: u32 = cpu.r32(0x5887d3u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4856a2; continue; }
  pc = 0x485696; continue;
 }
 0x485696 => {
  { let v: u32 = cpu.r8(0xau32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v = cpu.r32(0x5887d7u32); cpu.div32(v); }
  pc = 0x4856a2; continue;
 }
 0x4856a2 => {
  { let v: u32 = cpu.r32(0x5887dbu32).wrapping_add(cpu.eax); cpu.w32(0x5887dbu32, v); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x95u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.r32(0x5887dbu32), cpu.eax); cpu.w32(0x5887dbu32, v); }
  { let v: u32 = cpu.r32(0x5887dbu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4856c0 => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4856d8; continue; }
  pc = 0x4856c8; continue;
 }
 0x4856c8 => {
  { let v = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4856d8; continue;
 }
 0x4856d8 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4856f0; continue; }
  pc = 0x4856e0; continue;
 }
 0x4856e0 => {
  { let v = cpu.r8(0x75u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x4856f0; continue;
 }
 0x4856f0 => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485708; continue; }
  pc = 0x4856f8; continue;
 }
 0x4856f8 => {
  { let v = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485708; continue;
 }
 0x485708 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485714; continue; }
  pc = 0x48570e; continue;
 }
 0x48570e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485714; continue;
 }
 0x485714 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485720; continue; }
  pc = 0x48571a; continue;
 }
 0x48571a => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485720; continue;
 }
 0x485720 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x48572c; continue; }
  pc = 0x485726; continue;
 }
 0x485726 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48572c; continue;
 }
 0x48572c => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485738; continue; }
  pc = 0x485732; continue;
 }
 0x485732 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485738; continue;
 }
 0x485738 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485744; continue; }
  pc = 0x48573e; continue;
 }
 0x48573e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485744; continue;
 }
 0x485744 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x485750; continue; }
  pc = 0x48574a; continue;
 }
 0x48574a => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485750; continue;
 }
 0x485750 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x48576b; continue; }
  pc = 0x485755; continue;
 }
 0x485755 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x48576b; continue;
 }
 0x48576b => {
  cpu.poll(); pc = 0x48568d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485770); }
 let mut pc: u32 = 0x485770;
 loop { match pc {
 0x485770 => {
  cpu.push(0x485775); f_00485823(cpu); if cpu.returned_to != 0x485775 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d69f0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d69f4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d69f8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d69fcu32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4857a7; continue;
 }
 0x4857a7 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x485819; continue; }
  pc = 0x4857ae; continue;
 }
 0x4857ae => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x485819; continue; }
  pc = 0x4857b5; continue;
 }
 0x4857b5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x485819; continue; }
  pc = 0x4857bc; continue;
 }
 0x4857bc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x485819; continue; }
  pc = 0x4857c3; continue;
 }
 0x4857c3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x485819; continue; }
  pc = 0x4857ca; continue;
 }
 0x4857ca => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x4857dd); f_004853e8(cpu); if cpu.returned_to != 0x4857dd { return; }
  cpu.push(0x4857e2); f_0048537d(cpu); if cpu.returned_to != 0x4857e2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4857f2); f_0047a2d9(cpu); if cpu.returned_to != 0x4857f2 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x485819; continue; }
  pc = 0x4857f5; continue;
 }
 0x4857f5 => {
  cpu.push(0x4857fa); f_004853dc(cpu); if cpu.returned_to != 0x4857fa { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48580d; continue; }
  pc = 0x485808; continue;
 }
 0x485808 => {
  { let v: u32 = 0xffffu32; cpu.eax = v; }
  pc = 0x48580d; continue;
 }
 0x48580d => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))).wrapping_add((cpu.eax as u16)); cpu.w16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2)), v); }
  { let v: u32 = cpu.r32(0x4d69f0u32.wrapping_add(cpu.ebx.wrapping_mul(4))).wrapping_add(cpu.eax); cpu.w32(0x4d69f0u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  pc = 0x485819; continue;
 }
 0x485819 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4857a7; continue; }
  pc = 0x485822; continue;
 }
 0x485822 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485823(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485823); }
 let mut pc: u32 = 0x485823;
 loop { match pc {
 0x485823 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x485832; continue;
 }
 0x485832 => {
  { let v: u32 = cpu.eax; cpu.w32(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xeu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x485832; continue; }
  pc = 0x48583d; continue;
 }
 0x48583d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485841(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485841); }
 let mut pc: u32 = 0x485841;
 loop { match pc {
 0x485841 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x48584b; continue;
 }
 0x48584b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x485879; continue; }
  pc = 0x485852; continue;
 }
 0x485852 => {
  { cpu.and8(cpu.r8(0x5d3d3eu32), 0x1u8); }
  if !cpu.zf { pc = 0x485861; continue; }
  pc = 0x48585b; continue;
 }
 0x48585b => {
  { cpu.and8(cpu.r8(0x1du32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x485879; continue; }
  pc = 0x485861; continue;
 }
 0x485861 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x485868); f_00485880(cpu); if cpu.returned_to != 0x485868 { return; }
  cpu.push(0x48586d); f_004858cc(cpu); if cpu.returned_to != 0x48586d { return; }
  cpu.push(0x485872); f_004858dd(cpu); if cpu.returned_to != 0x485872 { return; }
  cpu.push(0x485877); f_0048588b(cpu); if cpu.returned_to != 0x485877 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x485879; continue;
 }
 0x485879 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48584b; continue; }
  pc = 0x48587f; continue;
 }
 0x48587f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485880); }
 let mut pc: u32 = 0x485880;
 loop { match pc {
 0x485880 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x19u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048588b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48588b); }
 let mut pc: u32 = 0x48588b;
 loop { match pc {
 0x48588b => {
  { let v: u32 = 0x522514u32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x485895; continue;
 }
 0x485895 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x4858c5; continue; }
  pc = 0x48589b; continue;
 }
 0x48589b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4858b3; continue; }
  pc = 0x4858a3; continue;
 }
 0x4858a3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4858b0); f_0047a2d9(cpu); if cpu.returned_to != 0x4858b0 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4858b3; continue;
 }
 0x4858b3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1eu32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x4858c5; continue; }
  pc = 0x4858ba; continue;
 }
 0x4858ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  cpu.push(0x4858c2); f_0048537d(cpu); if cpu.returned_to != 0x4858c2 { return; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x19u32.wrapping_add(cpu.esi), v); }
  pc = 0x4858c5; continue;
 }
 0x4858c5 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x485895; continue; }
  pc = 0x4858cb; continue;
 }
 0x4858cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004858cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4858cc); }
 let mut pc: u32 = 0x4858cc;
 loop { match pc {
 0x4858cc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524b6bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x19u32.wrapping_add(cpu.esi)), cpu.eax); cpu.w32(0x19u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004858dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4858dd); }
 let mut pc: u32 = 0x4858dd;
 loop { match pc {
 0x4858dd => {
  { let v: u32 = 0x559754u32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x4858e7; continue;
 }
 0x4858e7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x48599d; continue; }
  pc = 0x4858f2; continue;
 }
 0x4858f2 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8419u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x4858fb; continue;
 }
 0x4858fb => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b86a3u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485904; continue;
 }
 0x485904 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b85a9u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x48590d; continue;
 }
 0x48590d => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b85c2u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485916; continue;
 }
 0x485916 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b85dbu32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x48591f; continue;
 }
 0x48591f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b85f4u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485928; continue;
 }
 0x485928 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b86d5u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485931; continue;
 }
 0x485931 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8707u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x48593a; continue;
 }
 0x48593a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8739u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485943; continue;
 }
 0x485943 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8464u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x48594c; continue;
 }
 0x48594c => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8671u32); }
  if cpu.zf { pc = 0x48595e; continue; }
  pc = 0x485955; continue;
 }
 0x485955 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4b8626u32); }
  if !cpu.zf { pc = 0x48599d; continue; }
  pc = 0x48595e; continue;
 }
 0x48595e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x485978; continue; }
  pc = 0x485966; continue;
 }
 0x485966 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x485973); f_0047a2d9(cpu); if cpu.returned_to != 0x485973 { return; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x48599d; continue; }
  pc = 0x485978; continue;
 }
 0x485978 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if !cpu.zf { pc = 0x48599d; continue; }
  pc = 0x48597f; continue;
 }
 0x48597f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  cpu.push(0x485990); f_004859a8(cpu); if cpu.returned_to != 0x485990 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x48599a); f_0048537d(cpu); if cpu.returned_to != 0x48599a { return; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x19u32.wrapping_add(cpu.esi), v); }
  pc = 0x48599d; continue;
 }
 0x48599d => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4858e7; continue; }
  pc = 0x4859a7; continue;
 }
 0x4859a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004859a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4859a8); }
 let mut pc: u32 = 0x4859a8;
 loop { match pc {
 0x4859a8 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x4859b3; continue; }
  pc = 0x4859ac; continue;
 }
 0x4859ac => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859b3 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x4859be; continue; }
  pc = 0x4859b7; continue;
 }
 0x4859b7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859be => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if !cpu.zf { pc = 0x4859c9; continue; }
  pc = 0x4859c2; continue;
 }
 0x4859c2 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859c9 => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if !cpu.zf { pc = 0x4859d4; continue; }
  pc = 0x4859cd; continue;
 }
 0x4859cd => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859d4 => {
  { cpu.sub8((cpu.eax as u8), 0x1du8); }
  if !cpu.zf { pc = 0x4859df; continue; }
  pc = 0x4859d8; continue;
 }
 0x4859d8 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859df => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if !cpu.zf { pc = 0x4859ea; continue; }
  pc = 0x4859e3; continue;
 }
 0x4859e3 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859ea => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if !cpu.zf { pc = 0x4859f5; continue; }
  pc = 0x4859ee; continue;
 }
 0x4859ee => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x4859f5 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if !cpu.zf { pc = 0x485a00; continue; }
  pc = 0x4859f9; continue;
 }
 0x4859f9 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x485a00 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x485a0b; continue; }
  pc = 0x485a04; continue;
 }
 0x485a04 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x485a0b => {
  { cpu.sub8((cpu.eax as u8), 0x21u8); }
  if !cpu.zf { pc = 0x485a16; continue; }
  pc = 0x485a0f; continue;
 }
 0x485a0f => {
  { let v: u32 = 0xbu32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x485a16 => {
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if !cpu.zf { pc = 0x485b62; continue; }
  pc = 0x485a1e; continue;
 }
 0x485a1e => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  pc = 0x485a23; continue;
 }
 0x485a23 => {
  { let v: u32 = cpu.r32(0x5d62ccu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d7u32, v); }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485a56; continue; }
  pc = 0x485a47; continue;
 }
 0x485a47 => {
  { let v = cpu.r8(0x10u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485a56; continue;
 }
 0x485a56 => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485a6e; continue; }
  pc = 0x485a5f; continue;
 }
 0x485a5f => {
  { let v = cpu.r8(0x11u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485a6e; continue;
 }
 0x485a6e => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485a86; continue; }
  pc = 0x485a77; continue;
 }
 0x485a77 => {
  { let v = cpu.r8(0x18u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485a86; continue;
 }
 0x485a86 => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485a9e; continue; }
  pc = 0x485a8f; continue;
 }
 0x485a8f => {
  { let v = cpu.r8(0x19u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485a9e; continue;
 }
 0x485a9e => {
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485ab6; continue; }
  pc = 0x485aa7; continue;
 }
 0x485aa7 => {
  { let v = cpu.r8(0x1bu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485ab6; continue;
 }
 0x485ab6 => {
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485ace; continue; }
  pc = 0x485abf; continue;
 }
 0x485abf => {
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485ace; continue;
 }
 0x485ace => {
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485ae6; continue; }
  pc = 0x485ad7; continue;
 }
 0x485ad7 => {
  { let v = cpu.r8(0x1du32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485ae6; continue;
 }
 0x485ae6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485b04; continue; }
  pc = 0x485aed; continue;
 }
 0x485aed => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0xeu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485b04; continue;
 }
 0x485b04 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485b22; continue; }
  pc = 0x485b0b; continue;
 }
 0x485b0b => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0x1eu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485b22; continue;
 }
 0x485b22 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485b4b; continue; }
  pc = 0x485b29; continue;
 }
 0x485b29 => {
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0xfu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = (cpu.edx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.edx); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485b4b; continue;
 }
 0x485b4b => {
  { let v: u32 = cpu.r32(0x5887d3u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x485b62; continue; }
  pc = 0x485b54; continue;
 }
 0x485b54 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v = cpu.r32(0x5887d7u32); cpu.div32(v); }
  pc = 0x485b62; continue;
 }
 0x485b62 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485b77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485b77); }
 let mut pc: u32 = 0x485b77;
 loop { match pc {
 0x485b77 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x485e0f; continue; }
  pc = 0x485b8a; continue;
 }
 0x485b8a => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d658cu32, v); }
  pc = 0x485b9c; continue;
 }
 0x485b9c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x485dfe; continue; }
  pc = 0x485ba9; continue;
 }
 0x485ba9 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edi = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d6280u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d7u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c04; continue; }
  pc = 0x485bf5; continue;
 }
 0x485bf5 => {
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c04; continue;
 }
 0x485c04 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c1c; continue; }
  pc = 0x485c0d; continue;
 }
 0x485c0d => {
  { let v = cpu.r8(0x1du32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c1c; continue;
 }
 0x485c1c => {
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c34; continue; }
  pc = 0x485c25; continue;
 }
 0x485c25 => {
  { let v = cpu.r8(0x1eu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c34; continue;
 }
 0x485c34 => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c4c; continue; }
  pc = 0x485c3d; continue;
 }
 0x485c3d => {
  { let v = cpu.r8(0x1fu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c4c; continue;
 }
 0x485c4c => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c64; continue; }
  pc = 0x485c55; continue;
 }
 0x485c55 => {
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c64; continue;
 }
 0x485c64 => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c7c; continue; }
  pc = 0x485c6d; continue;
 }
 0x485c6d => {
  { let v = cpu.r8(0x21u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c7c; continue;
 }
 0x485c7c => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485c94; continue; }
  pc = 0x485c85; continue;
 }
 0x485c85 => {
  { let v = cpu.r8(0x24u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485c94; continue;
 }
 0x485c94 => {
  { let v: u32 = cpu.r32(0x5887d3u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x485ca5; continue; }
  pc = 0x485c9d; continue;
 }
 0x485c9d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x5887d7u32); cpu.div32(v); }
  pc = 0x485ca5; continue;
 }
 0x485ca5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5887dbu32, v); }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d62ccu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5887d7u32, v); }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485ce1; continue; }
  pc = 0x485cd2; continue;
 }
 0x485cd2 => {
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485ce1; continue;
 }
 0x485ce1 => {
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485cf9; continue; }
  pc = 0x485cea; continue;
 }
 0x485cea => {
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485cf9; continue;
 }
 0x485cf9 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485d11; continue; }
  pc = 0x485d02; continue;
 }
 0x485d02 => {
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d11; continue;
 }
 0x485d11 => {
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485d29; continue; }
  pc = 0x485d1a; continue;
 }
 0x485d1a => {
  { let v = cpu.r8(0x23u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d29; continue;
 }
 0x485d29 => {
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485d41; continue; }
  pc = 0x485d32; continue;
 }
 0x485d32 => {
  { let v = cpu.r8(0x25u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d41; continue;
 }
 0x485d41 => {
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485d59; continue; }
  pc = 0x485d4a; continue;
 }
 0x485d4a => {
  { let v = cpu.r8(0x26u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d59; continue;
 }
 0x485d59 => {
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x485d71; continue; }
  pc = 0x485d62; continue;
 }
 0x485d62 => {
  { let v = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d71; continue;
 }
 0x485d71 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485d93; continue; }
  pc = 0x485d78; continue;
 }
 0x485d78 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.edx); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485d93; continue;
 }
 0x485d93 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485db1; continue; }
  pc = 0x485d9a; continue;
 }
 0x485d9a => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.r8(0x24u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.eax); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485db1; continue;
 }
 0x485db1 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x485dda; continue; }
  pc = 0x485db8; continue;
 }
 0x485db8 => {
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = (cpu.edx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5887d3u32).wrapping_add(cpu.edx); cpu.w32(0x5887d3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5887d7u32)); cpu.w32(0x5887d7u32, v); }
  pc = 0x485dda; continue;
 }
 0x485dda => {
  { let v: u32 = cpu.r32(0x5887d3u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x485deb; continue; }
  pc = 0x485de3; continue;
 }
 0x485de3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x5887d7u32); cpu.div32(v); }
  pc = 0x485deb; continue;
 }
 0x485deb => {
  { let v: u32 = cpu.r8(0xau32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5887dbu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d658cu32).wrapping_add(cpu.eax); cpu.w32(0x5d658cu32, v); }
  pc = 0x485dfe; continue;
 }
 0x485dfe => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x485b9c; continue; }
  pc = 0x485e0a; continue;
 }
 0x485e0a => {
  { let v: u32 = cpu.r32(0x5d658cu32); cpu.eax = v; }
  pc = 0x485e0f; continue;
 }
 0x485e0f => {
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
pub fn f_00485e18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485e18); }
 let mut pc: u32 = 0x485e18;
 loop { match pc {
 0x485e18 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = 0xbe5u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  cpu.push(0x485e49); f_00486419(cpu); if cpu.returned_to != 0x485e49 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485e4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485e4a); }
 let mut pc: u32 = 0x485e4a;
 loop { match pc {
 0x485e4a => {
  { let v: u32 = 0x527780u32; cpu.edi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x485e59; continue;
 }
 0x485e59 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x30u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x485e59; continue; }
  pc = 0x485e61; continue;
 }
 0x485e61 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485e62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485e62); }
 let mut pc: u32 = 0x485e62;
 loop { match pc {
 0x485e62 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x527780u32; cpu.edi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x485e73; continue;
 }
 0x485e73 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x485e81; continue; }
  pc = 0x485e78; continue;
 }
 0x485e78 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x30u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x485e73; continue; }
  pc = 0x485e7d; continue;
 }
 0x485e7d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x485e81 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485e85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485e85); }
 let mut pc: u32 = 0x485e85;
 loop { match pc {
 0x485e85 => {
  { let v: u32 = 0x527780u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x485e95; continue;
 }
 0x485e95 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x485ea5; continue; }
  pc = 0x485e9c; continue;
 }
 0x485e9c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x485ea3); f_00485eb1(cpu); if cpu.returned_to != 0x485ea3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x485ea5; continue;
 }
 0x485ea5 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x485e95; continue; }
  pc = 0x485eaa; continue;
 }
 0x485eaa => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485eb1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x485eb1); } r_00485eb1(cpu, 0x485eb1); }
pub fn f_00486000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x486000); } r_00485eb1(cpu, 0x486000); }
fn r_00485eb1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x485eb1 => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x485ec7; continue; }
  pc = 0x485eb9; continue;
 }
 0x485eb9 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x1au32.wrapping_add(cpu.esi))); cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x486094; continue; }
  pc = 0x485ec7; continue;
 }
 0x485ec7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x485ede; continue; }
  pc = 0x485ed4; continue;
 }
 0x485ed4 => {
  { cpu.sub8(cpu.r8(0x23u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x485ede; continue; }
  pc = 0x485eda; continue;
 }
 0x485eda => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x485ede; continue;
 }
 0x485ede => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x40u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x575856u32, v); }
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x40u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x575858u32, v); }
  { let v: u32 = cpu.r16(0x14u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x575856u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x485f9c; continue; }
  pc = 0x485f23; continue;
 }
 0x485f23 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x575858u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x485f9c; continue; }
  pc = 0x485f2c; continue;
 }
 0x485f2c => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6b9cu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x485f9c; continue; }
  pc = 0x485f3c; continue;
 }
 0x485f3c => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6ba0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x485f9c; continue; }
  pc = 0x485f4c; continue;
 }
 0x485f4c => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x485f6d; continue; }
  pc = 0x485f55; continue;
 }
 0x485f55 => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x485f6d; continue; }
  pc = 0x485f5d; continue;
 }
 0x485f5d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x485f76; continue; }
  pc = 0x485f6d; continue;
 }
 0x485f6d => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x485f9c; continue;
 }
 0x485f76 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0xcu8); }
  if !cpu.zf { pc = 0x486095; continue; }
  pc = 0x485f8b; continue;
 }
 0x485f8b => {
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x485f9c; continue;
 }
 0x485f9c => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x485fbc; continue; }
  pc = 0x485fa2; continue;
 }
 0x485fa2 => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x486094; continue; }
  pc = 0x485fae; continue;
 }
 0x485fae => {
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x485fbc; continue; }
  pc = 0x485fb4; continue;
 }
 0x485fb4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x486094; continue;
 }
 0x485fbc => {
  { let v: u8 = cpu.dec8(cpu.r8(0x20u32.wrapping_add(cpu.esi))); cpu.w8(0x20u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x486094; continue; }
  pc = 0x485fc5; continue;
 }
 0x485fc5 => {
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x485fe4; continue; }
  pc = 0x485fd1; continue;
 }
 0x485fd1 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_sub(0x2u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x23u32.wrapping_add(cpu.esi))); cpu.w8(0x23u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x23u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x485fe4; continue; }
  pc = 0x485fe2; continue;
 }
 0x485fe2 => {
  pc = 0x48604a; continue;
 }
 0x485fe4 => {
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x486008; continue; }
  pc = 0x485fea; continue;
 }
 0x485fea => {
  { let v: u8 = cpu.inc8(cpu.r8(0x18u32.wrapping_add(cpu.esi))); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if cpu.zf { pc = 0x486008; continue; }
  pc = 0x485ff4; continue;
 }
 0x485ff4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x18u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x486008; continue; }
  pc = 0x485ff9; continue;
 }
 0x485ff9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x486000; continue;
 }
 0x486000 => {
  { let v: u16 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(0x486008); cpu.call(t); if cpu.returned_to != 0x486008 { return; }
  pc = 0x486008; continue;
 }
 0x486008 => {
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x486013; continue; }
  pc = 0x48600e; continue;
 }
 0x48600e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.esi))); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  pc = 0x486016; continue;
 }
 0x486013 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x1fu32.wrapping_add(cpu.esi))); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  pc = 0x486016; continue;
 }
 0x486016 => {
  { let v: u16 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x14u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x16u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x486094; continue; }
  pc = 0x486025; continue;
 }
 0x486025 => {
  { cpu.sub8(cpu.r8(0x21u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x486036; continue; }
  pc = 0x48602b; continue;
 }
 0x48602b => {
  { cpu.sub8(cpu.r8(0x21u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x48604a; continue; }
  pc = 0x486031; continue;
 }
 0x486031 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x21u32.wrapping_add(cpu.esi))); cpu.w8(0x21u32.wrapping_add(cpu.esi), v); }
  if cpu.sf { pc = 0x48604a; continue; }
  pc = 0x486036; continue;
 }
 0x486036 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x486094; continue; }
  pc = 0x486040; continue;
 }
 0x486040 => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  pc = 0x486094; continue;
 }
 0x48604a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x29u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if !cpu.zf { pc = 0x486094; continue; }
  pc = 0x486058; continue;
 }
 0x486058 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6592u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6594u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d6590u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x27u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x18u32.wrapping_add(cpu.esi), v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(0x486092); cpu.call(t); if cpu.returned_to != 0x486092 { return; }
  pc = 0x486094; continue;
 }
 0x486094 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486095 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4d8236u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x21u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x4860e6; continue; }
  pc = 0x4860da; continue;
 }
 0x4860da => {
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4860e6; continue;
 }
 0x4860e6 => {
  { cpu.and8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x4860f5; continue; }
  pc = 0x4860ec; continue;
 }
 0x4860ec => {
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4860f5; continue;
 }
 0x4860f5 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  cpu.poll(); pc = 0x485f9c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486101(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x486101); } r_00486101(cpu, 0x486101); }
pub fn f_0048623e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48623e); } r_00486101(cpu, 0x48623e); }
fn r_00486101(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x486101 => {
  { cpu.sub16(cpu.r16(0x4d69d6u32), 0x0u16); }
  if cpu.zf { pc = 0x486273; continue; }
  pc = 0x48610f; continue;
 }
 0x48610f => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x486119; continue;
 }
 0x486119 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x486130; continue; }
  pc = 0x486121; continue;
 }
 0x486121 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x48612e); f_0048623e(cpu); if cpu.returned_to != 0x48612e { return; }
  if cpu.cf { pc = 0x486136; continue; }
  pc = 0x486130; continue;
 }
 0x486130 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x486119; continue; }
  pc = 0x486135; continue;
 }
 0x486135 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486136 => {
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6936u32, v); }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x48617a; continue; }
  pc = 0x486161; continue;
 }
 0x486161 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x48616b); f_0041c743(cpu); if cpu.returned_to != 0x48616b { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48617a; continue;
 }
 0x48617a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5d65d7u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4861b1); f_00486419(cpu); if cpu.returned_to != 0x4861b1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0xa5du16); }
  if !cpu.zf { pc = 0x4861f5; continue; }
  pc = 0x4861be; continue;
 }
 0x4861be => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xaaeu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  cpu.push(0x4861ee); f_00486419(cpu); if cpu.returned_to != 0x4861ee { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x486130; continue;
 }
 0x4861f5 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0xa65u16); }
  if !cpu.zf { pc = 0x486237; continue; }
  pc = 0x486200; continue;
 }
 0x486200 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xaafu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  cpu.push(0x486230); f_00486419(cpu); if cpu.returned_to != 0x486230 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x486130; continue;
 }
 0x486237 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x486130; continue;
 }
 0x48623e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5d6597u32; cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x486249; continue;
 }
 0x486249 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x486257; continue; }
  pc = 0x48624e; continue;
 }
 0x48624e => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x486249; continue; }
  pc = 0x486253; continue;
 }
 0x486253 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486257 => {
  { let v: u16 = 0x10u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x486273; continue;
 }
 0x486273 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004862bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4862bf); }
 let mut pc: u32 = 0x4862bf;
 loop { match pc {
 0x4862bf => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4862d9); f_0041c6a6(cpu); if cpu.returned_to != 0x4862d9 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xb58u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d693au32, v); }
  { let v: u8 = 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xecu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x486378u32; cpu.w32(0x4d6b64u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d6596u32, v); }
  cpu.push(0x486321); f_00486419(cpu); if cpu.returned_to != 0x486321 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486378(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486378); }
 let mut pc: u32 = 0x486378;
 loop { match pc {
 0x486378 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x26u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xecu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5d6590u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xb58u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0xb71u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d693au32, v); }
  { let v: u8 = 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4863d7u32; cpu.w32(0x4d6b64u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5d6596u32, v); }
  cpu.push(0x4863d5); f_00486419(cpu); if cpu.returned_to != 0x4863d5 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004863d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4863d7); }
 let mut pc: u32 = 0x4863d7;
 loop { match pc {
 0x4863d7 => {
  { cpu.pushad(); }
  { let v: u8 = 0xe2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0xb76u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6938u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), 0x10u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = ((cpu.edx >> 8) as u8) as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.w8(0x5d6596u32, v); }
  cpu.push(0x486417); f_00486419(cpu); if cpu.returned_to != 0x486417 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486419(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486419); }
 let mut pc: u32 = 0x486419;
 loop { match pc {
 0x486419 => {
  { cpu.pushad(); }
  cpu.push(0x48641f); f_00485e62(cpu); if cpu.returned_to != 0x48641f { return; }
  if cpu.cf { pc = 0x48650f; continue; }
  pc = 0x486425; continue;
 }
 0x486425 => {
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x5d6596u32), 0x0u8); }
  if cpu.zf { pc = 0x486490; continue; }
  pc = 0x48645e; continue;
 }
 0x48645e => {
  { cpu.sub8(cpu.r8(0x5d6596u32), 0x1u8); }
  if cpu.zf { pc = 0x48647b; continue; }
  pc = 0x486467; continue;
 }
 0x486467 => {
  { let v: u16 = cpu.r16(0x5d6592u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x5d6594u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x48647b; continue;
 }
 0x48647b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x486487); f_00486511(cpu); if cpu.returned_to != 0x486487 { return; }
  if !cpu.cf { pc = 0x486490; continue; }
  pc = 0x486489; continue;
 }
 0x486489 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x48650f; continue;
 }
 0x486490 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d6596u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x4864b6; continue; }
  pc = 0x4864a7; continue;
 }
 0x4864a7 => {
  { let v: u8 = 0x4u8; cpu.w8(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x4864b6; continue;
 }
 0x4864b6 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x4864d4; continue; }
  pc = 0x4864cd; continue;
 }
 0x4864cd => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x4864d4; continue;
 }
 0x4864d4 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x4864f1; continue; }
  pc = 0x4864ea; continue;
 }
 0x4864ea => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)) | 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  pc = 0x4864f1; continue;
 }
 0x4864f1 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x486508; continue; }
  pc = 0x4864f6; continue;
 }
 0x4864f6 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x27u32.wrapping_add(cpu.edi)), 0x20u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x486508; continue;
 }
 0x486508 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48650f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486511(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486511); }
 let mut pc: u32 = 0x486511;
 loop { match pc {
 0x486511 => {
  { cpu.pushad(); }
  { let v: u32 = 0x527780u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x48651c; continue;
 }
 0x48651c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x486529; continue; }
  pc = 0x486521; continue;
 }
 0x486521 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48651c; continue; }
  pc = 0x486526; continue;
 }
 0x486526 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486529 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x486521; continue; }
  pc = 0x48652d; continue;
 }
 0x48652d => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { cpu.poll(); pc = 0x486521; continue; }
  pc = 0x486533; continue;
 }
 0x486533 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esi)), (cpu.ebx as u16)); }
  if !cpu.zf { cpu.poll(); pc = 0x486521; continue; }
  pc = 0x486539; continue;
 }
 0x486539 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486542(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486542); }
 let mut pc: u32 = 0x486542;
 loop { match pc {
 0x486542 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x48654a; continue; }
  pc = 0x486549; continue;
 }
 0x486549 => {
  {  }
  pc = 0x48654a; continue;
 }
 0x48654a => {
  { cpu.sub16((cpu.ecx as u16), 0x1c89u16); }
  if (cpu.cf || cpu.zf) { pc = 0x486561; continue; }
  pc = 0x486551; continue;
 }
 0x486551 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cd0a8u32, v); }
  cpu.push(0x48655f); f_0046c9d8(cpu); if cpu.returned_to != 0x48655f { return; }
  pc = 0x486585; continue;
 }
 0x486561 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x486588; continue; }
  pc = 0x48656f; continue;
 }
 0x48656f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  pc = 0x486585; continue;
 }
 0x486585 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486588 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486c8e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x486c8e); } r_00486c8e(cpu, 0x486c8e); }
pub fn f_00486cb6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x486cb6); } r_00486c8e(cpu, 0x486cb6); }
fn r_00486c8e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x486c8e => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4d75b4u32); }
  if cpu.zf { pc = 0x486d5f; continue; }
  pc = 0x486ca3; continue;
 }
 0x486ca3 => {
  { let v: u32 = 0x4d75acu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x486d5f; continue; }
  pc = 0x486cb0; continue;
 }
 0x486cb0 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x486cb6; continue;
 }
 0x486cb6 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x486d5f; continue; }
  pc = 0x486cbe; continue;
 }
 0x486cbe => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x486cc8; continue;
 }
 0x486cc8 => {
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x486d11; continue; }
  pc = 0x486ccc; continue;
 }
 0x486ccc => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x486ccc; continue; }
  pc = 0x486cd4; continue;
 }
 0x486cd4 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x486cd4; continue; }
  pc = 0x486cdc; continue;
 }
 0x486cdc => {
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6a68u32, v); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6a6cu32, v); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_sub(0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x6u32); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x4d6a68u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x6u32); cpu.edi = v; }
  cpu.poll(); pc = 0x486cc8; continue;
 }
 0x486d11 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_sub(0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x6u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0xfffffffau32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x4d6a68u32; cpu.esi = v; }
  { let a = cpu.ebp; let b = cpu.edi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x6u32); cpu.edi = v; }
  { let a = cpu.ebp; let b = cpu.edi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  cpu.push(0x486d4e); f_00486cb6(cpu); if cpu.returned_to != 0x486d4e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  cpu.push(0x486d5c); f_00486cb6(cpu); if cpu.returned_to != 0x486d5c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x486d5f; continue;
 }
 0x486d5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486d60); }
 let mut pc: u32 = 0x486d60;
 loop { match pc {
 0x486d60 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4ac410u32, v); }
  cpu.push(0x486d6e); f_00486c8e(cpu); if cpu.returned_to != 0x486d6e { return; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4d75b4u32); cpu.ecx = v; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x486df3; continue; }
  pc = 0x486d80; continue;
 }
 0x486d80 => {
  { let v: u32 = 0x4d75b4u32; cpu.esi = v; }
  pc = 0x486d85; continue;
 }
 0x486d85 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f70u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x486e0f; continue; }
  pc = 0x486d98; continue;
 }
 0x486d98 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x486df4; continue; }
  pc = 0x486da6; continue;
 }
 0x486da6 => {
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x56c6feu32), 0x1u8); }
  if !cpu.zf { pc = 0x486db7; continue; }
  pc = 0x486db2; continue;
 }
 0x486db2 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  pc = 0x486db7; continue;
 }
 0x486db7 => {
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x486dbe); cpu.call(t); if cpu.returned_to != 0x486dbe { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x57552au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x57552cu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x57552eu32, v); }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x486deb); cpu.call(t); if cpu.returned_to != 0x486deb { return; }
  pc = 0x486deb; continue;
 }
 0x486deb => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x486d85; continue; }
  pc = 0x486df3; continue;
 }
 0x486df3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x486df4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x486e0d); cpu.call(t); if cpu.returned_to != 0x486e0d { return; }
  cpu.poll(); pc = 0x486deb; continue;
 }
 0x486e0f => {
  { let v = cpu.r32(0x4d6ba0u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a3au32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4d8236u32); cpu.esi = v; }
  pc = 0x486e33; continue;
 }
 0x486e33 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4ac410u32, v); }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x486e56; continue; }
  pc = 0x486e43; continue;
 }
 0x486e43 => {
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bd0u32, v); }
  pc = 0x486e60; continue;
 }
 0x486e56 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd0u32, v); }
  pc = 0x486e60; continue;
 }
 0x486e60 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x486f64; continue; }
  pc = 0x486e6e; continue;
 }
 0x486e6e => {
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x56c6feu32), 0x1u8); }
  if !cpu.zf { pc = 0x486e7f; continue; }
  pc = 0x486e7a; continue;
 }
 0x486e7a => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  pc = 0x486e7f; continue;
 }
 0x486e7f => {
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x486e86); cpu.call(t); if cpu.returned_to != 0x486e86 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x486eb5; continue; }
  pc = 0x486e97; continue;
 }
 0x486e97 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba0u32)); }
  if !cpu.sf { pc = 0x486eae; continue; }
  pc = 0x486eac; continue;
 }
 0x486eac => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x486eae; continue;
 }
 0x486eae => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba0u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x486eb5; continue;
 }
 0x486eb5 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x486edf; continue; }
  pc = 0x486ebb; continue;
 }
 0x486ebb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x486ed8; continue; }
  pc = 0x486ed0; continue;
 }
 0x486ed0 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x486ed8; continue;
 }
 0x486ed8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x486edf; continue;
 }
 0x486edf => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x486f06; continue; }
  pc = 0x486ee5; continue;
 }
 0x486ee5 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x486f06; continue; }
  pc = 0x486ef6; continue;
 }
 0x486ef6 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x486f40; continue; }
  pc = 0x486eff; continue;
 }
 0x486eff => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6ba8u32, v); }
  pc = 0x486f06; continue;
 }
 0x486f06 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x486f1a; continue; }
  pc = 0x486f0c; continue;
 }
 0x486f0c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5d6a3au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x486f1a; continue;
 }
 0x486f1a => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x486f28; continue; }
  pc = 0x486f20; continue;
 }
 0x486f20 => {
  { let v: u16 = cpu.r8(0xeu32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x486f28; continue;
 }
 0x486f28 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x486f38; continue; }
  pc = 0x486f2e; continue;
 }
 0x486f2e => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f44u32); cpu.push(0x486f35); cpu.call(t); if cpu.returned_to != 0x486f35 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x486f40; continue;
 }
 0x486f38 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x486f3f); cpu.call(t); if cpu.returned_to != 0x486f3f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x486f40; continue;
 }
 0x486f40 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x486f4e; continue; }
  pc = 0x486f46; continue;
 }
 0x486f46 => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  cpu.poll(); pc = 0x486e33; continue;
 }
 0x486f4e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.poll(); pc = 0x486deb; continue;
 }
 0x486f64 => {
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x486f71; continue; }
  pc = 0x486f6f; continue;
 }
 0x486f6f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x486f71; continue;
 }
 0x486f71 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x486f97; continue; }
  pc = 0x486f8f; continue;
 }
 0x486f8f => {
  { let v: u16 = cpu.r8(0xeu32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x486f97; continue;
 }
 0x486f97 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f44u32); cpu.push(0x486f9e); cpu.call(t); if cpu.returned_to != 0x486f9e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x486f40; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486fa1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486fa1); }
 let mut pc: u32 = 0x486fa1;
 loop { match pc {
 0x486fa1 => {
  { cpu.sub8(cpu.r8(0x5b385eu32), 0x0u8); }
  if !cpu.zf { pc = 0x487014; continue; }
  pc = 0x486faa; continue;
 }
 0x486faa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x486fc4); f_00420989(cpu); if cpu.returned_to != 0x486fc4 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4ea16fu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x93u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x486ff1; continue; }
  pc = 0x486fdf; continue;
 }
 0x486fdf => {
  if (cpu.sf != cpu.of) { pc = 0x486fea; continue; }
  pc = 0x486fe1; continue;
 }
 0x486fe1 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.w8(0x93u32.wrapping_add(cpu.ebp), v); }
  pc = 0x486ff1; continue;
 }
 0x486fea => {
  { let v: u8 = cpu.dec8(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.w8(0x93u32.wrapping_add(cpu.ebp), v); }
  pc = 0x486ff1; continue;
 }
 0x486ff1 => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.cf { pc = 0x487016; continue; }
  pc = 0x486ffb; continue;
 }
 0x486ffb => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x487014; continue; }
  pc = 0x487005; continue;
 }
 0x487005 => {
  cpu.push(0x48700a); f_0048748e(cpu); if cpu.returned_to != 0x48700a { return; }
  if !cpu.cf { pc = 0x48700d; continue; }
  pc = 0x48700c; continue;
 }
 0x48700c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48700d => {
  cpu.push(0x487012); f_0048701d(cpu); if cpu.returned_to != 0x487012 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487014 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487016 => {
  cpu.push(0x48701b); f_00487442(cpu); if cpu.returned_to != 0x48701b { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048701d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48701d); }
 let mut pc: u32 = 0x48701d;
 loop { match pc {
 0x48701d => {
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487033; continue; }
  pc = 0x48702c; continue;
 }
 0x48702c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x487037; continue; }
  pc = 0x487033; continue;
 }
 0x487033 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x487037; continue;
 }
 0x487037 => {
  cpu.push(0x48703c); f_0041c808(cpu); if cpu.returned_to != 0x48703c { return; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if !cpu.zf { pc = 0x487047; continue; }
  pc = 0x487042; continue;
 }
 0x487042 => {
  cpu.push(0x487047); f_0045aaf5(cpu); if cpu.returned_to != 0x487047 { return; }
  pc = 0x487047; continue;
 }
 0x487047 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if !cpu.zf { pc = 0x487052; continue; }
  pc = 0x48704d; continue;
 }
 0x48704d => {
  cpu.push(0x487052); f_0045aa00(cpu); if cpu.returned_to != 0x487052 { return; }
  pc = 0x487052; continue;
 }
 0x487052 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x487141; continue; }
  pc = 0x487068; continue;
 }
 0x487068 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x487141; continue; }
  pc = 0x487079; continue;
 }
 0x487079 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x487141; continue; }
  pc = 0x487093; continue;
 }
 0x487093 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x487141; continue; }
  pc = 0x4870a4; continue;
 }
 0x4870a4 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4d8236u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x44u8); }
  if cpu.zf { pc = 0x487142; continue; }
  pc = 0x4870d2; continue;
 }
 0x4870d2 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x43u8); }
  if cpu.zf { pc = 0x4872a1; continue; }
  pc = 0x4870dd; continue;
 }
 0x4870dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x487128; continue; }
  pc = 0x487123; continue;
 }
 0x487123 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487128; continue;
 }
 0x487128 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x487141; continue;
 }
 0x487141 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487142 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d6a39u32, v); }
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x4u8); }
  if cpu.zf { pc = 0x48715d; continue; }
  pc = 0x487152; continue;
 }
 0x487152 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x4871d7; continue; }
  pc = 0x48715b; continue;
 }
 0x48715b => {
  pc = 0x487166; continue;
 }
 0x48715d => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x4871d7; continue; }
  pc = 0x487166; continue;
 }
 0x487166 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a35u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d6a37u32, v); }
  { let v: u16 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x4871b9; continue; }
  pc = 0x4871b4; continue;
 }
 0x4871b4 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4871b9; continue;
 }
 0x4871b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u16 = cpu.r16(0x5d6a35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6a37u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4871d7; continue;
 }
 0x4871d7 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x4u8); }
  if cpu.zf { pc = 0x4871f4; continue; }
  pc = 0x4871e9; continue;
 }
 0x4871e9 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x48720e; continue; }
  pc = 0x4871f2; continue;
 }
 0x4871f2 => {
  pc = 0x4871fd; continue;
 }
 0x4871f4 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x48720e; continue; }
  pc = 0x4871fd; continue;
 }
 0x4871fd => {
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x487233; continue;
 }
 0x48720e => {
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x4u8); }
  if cpu.zf { pc = 0x487226; continue; }
  pc = 0x487217; continue;
 }
 0x487217 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x487233; continue; }
  pc = 0x487220; continue;
 }
 0x487220 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x487233; continue;
 }
 0x487226 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x7u16); }
  if (cpu.sf == cpu.of) { pc = 0x487233; continue; }
  pc = 0x48722f; continue;
 }
 0x48722f => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x487233; continue;
 }
 0x487233 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x44u32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x487273; continue; }
  pc = 0x48726b; continue;
 }
 0x48726b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x487275; continue;
 }
 0x487273 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x487275; continue;
 }
 0x487275 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4872a1 => {
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
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6a35u32, v); }
  { let v: u16 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d6a37u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x43u32; cpu.ebx = v; }
  { let v: u32 = 0x5d662cu32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x487315; continue; }
  pc = 0x487310; continue;
 }
 0x487310 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487315; continue;
 }
 0x487315 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x487336; continue; }
  pc = 0x48732d; continue;
 }
 0x48732d => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  pc = 0x487439; continue;
 }
 0x487336 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6a35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6a37u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x487356; continue;
 }
 0x487356 => {
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x4873b6; continue; }
  pc = 0x48735c; continue;
 }
 0x48735c => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4873b6; continue; }
  pc = 0x487366; continue;
 }
 0x487366 => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if (cpu.sf == cpu.of) { pc = 0x4873b6; continue; }
  pc = 0x487370; continue;
 }
 0x487370 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x81u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5d6a34u32), 0x1u8); }
  if !cpu.zf { pc = 0x487396; continue; }
  pc = 0x487390; continue;
 }
 0x487390 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x107fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48739a; continue;
 }
 0x487396 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18d0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48739a; continue;
 }
 0x48739a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x40u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4873b6; continue;
 }
 0x4873b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x178au16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x4873ce; continue; }
  pc = 0x4873c8; continue;
 }
 0x4873c8 => {
  { let v: u16 = 0x178bu16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x4873ce; continue;
 }
 0x4873ce => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x40u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x4873e7; continue; }
  pc = 0x4873e3; continue;
 }
 0x4873e3 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x4873e7; continue;
 }
 0x4873e7 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x487356; continue; }
  pc = 0x4873fc; continue;
 }
 0x4873fc => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ac0u32, v); }
  { cpu.and8(cpu.r8(0x7cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487439; continue; }
  pc = 0x487409; continue;
 }
 0x487409 => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x178cu16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6ac0u32).wrapping_add(0x10u32); cpu.w32(0x4d6ac0u32, v); }
  pc = 0x487439; continue;
 }
 0x487439 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ab4u32), 0x8u32); cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487442(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487442); }
 let mut pc: u32 = 0x487442;
 loop { match pc {
 0x487442 => {
  { cpu.pushad(); }
  cpu.push(0x487448); f_0041c743(cpu); if cpu.returned_to != 0x487448 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x48748c; continue; }
  pc = 0x48744d; continue;
 }
 0x48744d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x48748c); f_0047b6fe(cpu); if cpu.returned_to != 0x48748c { return; }
  pc = 0x48748c; continue;
 }
 0x48748c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048748e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48748e); }
 let mut pc: u32 = 0x48748e;
 loop { match pc {
 0x48748e => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7eau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x4874c5; continue; }
  pc = 0x4874ab; continue;
 }
 0x4874ab => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1b7u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x4874ba); f_00458cb3(cpu); if cpu.returned_to != 0x4874ba { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1b7u32, v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4874c5 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004874c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4874c8); } r_004874c8(cpu, 0x4874c8); }
pub fn f_004875e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4875e8); } r_004874c8(cpu, 0x4875e8); }
fn r_004874c8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4874c8 => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x4874fe); f_004714a5(cpu); if cpu.returned_to != 0x4874fe { return; }
  if cpu.cf { pc = 0x487592; continue; }
  pc = 0x487504; continue;
 }
 0x487504 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x38u32); cpu.ebx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x487525); f_004714a5(cpu); if cpu.returned_to != 0x487525 { return; }
  if cpu.cf { pc = 0x4875de; continue; }
  pc = 0x48752b; continue;
 }
 0x48752b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48758f; continue; }
  pc = 0x487534; continue;
 }
 0x487534 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48758f; continue; }
  pc = 0x48753e; continue;
 }
 0x48753e => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x38u32); cpu.ebx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x48755f); f_004714a5(cpu); if cpu.returned_to != 0x48755f { return; }
  if cpu.cf { pc = 0x487603; continue; }
  pc = 0x487565; continue;
 }
 0x487565 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48758f; continue; }
  pc = 0x48756f; continue;
 }
 0x48756f => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x48758d); f_004714a5(cpu); if cpu.returned_to != 0x48758d { return; }
  if cpu.cf { pc = 0x4875b8; continue; }
  pc = 0x48758f; continue;
 }
 0x48758f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487592 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x91u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4875b8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x91u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4875de => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4875e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x487603 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004880f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4880f4); }
 let mut pc: u32 = 0x4880f4;
 loop { match pc {
 0x4880f4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d6936u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x4d6936u32), 0x8u8); }
  if !cpu.zf { pc = 0x488112; continue; }
  pc = 0x48810c; continue;
 }
 0x48810c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488112 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488118(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488118); }
 let mut pc: u32 = 0x488118;
 loop { match pc {
 0x488118 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x488126; continue; }
  pc = 0x488121; continue;
 }
 0x488121 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48812b; continue; }
  pc = 0x488126; continue;
 }
 0x488126 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48812b; continue;
 }
 0x48812b => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x488185; continue; }
  pc = 0x488131; continue;
 }
 0x488131 => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x48813e; continue; }
  pc = 0x488137; continue;
 }
 0x488137 => {
  cpu.push(0x48813c); f_0048826c(cpu); if cpu.returned_to != 0x48813c { return; }
  pc = 0x48816e; continue;
 }
 0x48813e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x5d65ecu32.wrapping_add(cpu.esi)); cpu.push(0x488151); cpu.call(t); if cpu.returned_to != 0x488151 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x488165); f_004880f4(cpu); if cpu.returned_to != 0x488165 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  let t = cpu.r32(0x5d65f0u32.wrapping_add(cpu.esi)); cpu.push(0x48816e); cpu.call(t); if cpu.returned_to != 0x48816e { return; }
  pc = 0x48816e; continue;
 }
 0x48816e => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x488181); f_004880f4(cpu); if cpu.returned_to != 0x488181 { return; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488185 => {
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048818a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48818a); }
 let mut pc: u32 = 0x48818a;
 loop { match pc {
 0x48818a => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x4881b1; continue; }
  pc = 0x4881a1; continue;
 }
 0x4881a1 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5cbu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x488208; continue;
 }
 0x4881b1 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5c3u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d68ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68ceu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x4881dc; continue; }
  pc = 0x4881cb; continue;
 }
 0x4881cb => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x4881ee; continue; }
  pc = 0x4881d2; continue;
 }
 0x4881d2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x6u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4881fe; continue;
 }
 0x4881dc => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d68a4u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x5d68a6u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4881fe; continue;
 }
 0x4881ee => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d6904u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x5d6906u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4881fe; continue;
 }
 0x4881fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x488264; continue;
 }
 0x488208 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x488218; continue; }
  pc = 0x48820f; continue;
 }
 0x48820f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x48822a; continue; }
  pc = 0x488216; continue;
 }
 0x488216 => {
  pc = 0x48823c; continue;
 }
 0x488218 => {
  { let v: u16 = cpu.r16(0x5d68c4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d68c6u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48824c; continue;
 }
 0x48822a => {
  { let v: u16 = cpu.r16(0x5d68e4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d68e6u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48824c; continue;
 }
 0x48823c => {
  { let v: u16 = cpu.r16(0x5d6884u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6886u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48824c; continue;
 }
 0x48824c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x488264; continue;
 }
 0x488264 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048826c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48826c); }
 let mut pc: u32 = 0x48826c;
 loop { match pc {
 0x48826c => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x48829c; continue; }
  pc = 0x488284; continue;
 }
 0x488284 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x488292; continue; }
  pc = 0x48828b; continue;
 }
 0x48828b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x4882a6; continue; }
  pc = 0x488292; continue;
 }
 0x488292 => {
  { let v: u16 = cpu.r16(0x5d6854u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4882ae; continue;
 }
 0x48829c => {
  { let v: u16 = cpu.r16(0x5d6844u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4882ae; continue;
 }
 0x4882a6 => {
  { let v: u16 = cpu.r16(0x5d6834u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4882ae; continue;
 }
 0x4882ae => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ccu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d68ceu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x216u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004882df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4882df); }
 let mut pc: u32 = 0x4882df;
 loop { match pc {
 0x4882df => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004882e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4882e5); }
 let mut pc: u32 = 0x4882e5;
 loop { match pc {
 0x4882e5 => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048860e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48860e); }
 let mut pc: u32 = 0x48860e;
 loop { match pc {
 0x48860e => {
  { cpu.sub8(cpu.r8(0x9bu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x488619; continue; }
  pc = 0x488618; continue;
 }
 0x488618 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488619 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x9bu32.wrapping_add(cpu.ebp))); cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488621(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488621); }
 let mut pc: u32 = 0x488621;
 loop { match pc {
 0x488621 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4887d6; continue; }
  pc = 0x48862f; continue;
 }
 0x48862f => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x4887d4; continue; }
  pc = 0x48863a; continue;
 }
 0x48863a => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x576bf7u32, v); }
  cpu.push(0x488649); f_0045bf77(cpu); if cpu.returned_to != 0x488649 { return; }
  if !cpu.cf { pc = 0x4887d4; continue; }
  pc = 0x48864f; continue;
 }
 0x48864f => {
  { cpu.sub8(cpu.r8(0x5bu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4887d4; continue; }
  pc = 0x48865a; continue;
 }
 0x48865a => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x4887d4; continue; }
  pc = 0x488666; continue;
 }
 0x488666 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1448u16); }
  if !cpu.zf { pc = 0x4887d4; continue; }
  pc = 0x488681; continue;
 }
 0x488681 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6d32u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d6d34u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x900u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4887d4; continue; }
  pc = 0x4886c8; continue;
 }
 0x4886c8 => {
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4887d4; continue; }
  pc = 0x4886d2; continue;
 }
 0x4886d2 => {
  cpu.push(0x4886d7); f_00488c58(cpu); if cpu.returned_to != 0x4886d7 { return; }
  if !cpu.cf { pc = 0x488705; continue; }
  pc = 0x4886d9; continue;
 }
 0x4886d9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x4886e1); f_0045955b(cpu); if cpu.returned_to != 0x4886e1 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.push(0x4886f5); f_0046ab56(cpu); if cpu.returned_to != 0x4886f5 { return; }
  { cpu.popad(); }
  cpu.push(0x4886fb); f_0045c2d0(cpu); if cpu.returned_to != 0x4886fb { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4887d4; continue;
 }
 0x488705 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xafu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5d6d32u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d34u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x488737); f_00420989(cpu); if cpu.returned_to != 0x488737 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x48873f; continue;
 }
 0x48873f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x5b9b50u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.esi = v; }
  cpu.push(0x48874e); f_0045c57c(cpu); if cpu.returned_to != 0x48874e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x48875f; continue; }
  pc = 0x488753; continue;
 }
 0x488753 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48873f; continue; }
  pc = 0x48875c; continue;
 }
 0x48875c => {
  { cpu.popad(); }
  pc = 0x4887d4; continue;
 }
 0x48875f => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4b8946u32); }
  if cpu.zf { pc = 0x488777; continue; }
  pc = 0x488772; continue;
 }
 0x488772 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x488777; continue;
 }
 0x488777 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  cpu.push(0x488780); f_0048b31d(cpu); if cpu.returned_to != 0x488780 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x488789); f_0044c7f9(cpu); if cpu.returned_to != 0x488789 { return; }
  if cpu.cf { pc = 0x488790; continue; }
  pc = 0x48878b; continue;
 }
 0x48878b => {
  cpu.push(0x488790); f_0044c896(cpu); if cpu.returned_to != 0x488790 { return; }
  pc = 0x488790; continue;
 }
 0x488790 => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4b8946u32); }
  if cpu.zf { pc = 0x4887b2; continue; }
  pc = 0x48879a; continue;
 }
 0x48879a => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x4887b2; continue; }
  pc = 0x4887a0; continue;
 }
 0x4887a0 => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x4887ae; continue; }
  pc = 0x4887a4; continue;
 }
 0x4887a4 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x4887ae; continue; }
  pc = 0x4887a9; continue;
 }
 0x4887a9 => {
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x4887b2; continue; }
  pc = 0x4887ae; continue;
 }
 0x4887ae => {
  { let v: u8 = 0x1u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x4887b2; continue;
 }
 0x4887b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  cpu.push(0x4887c6); f_0046ab56(cpu); if cpu.returned_to != 0x4887c6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x4887d2); f_004887d8(cpu); if cpu.returned_to != 0x4887d2 { return; }
  pc = 0x4887d6; continue;
 }
 0x4887d4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4887d6 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004887d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4887d8); }
 let mut pc: u32 = 0x4887d8;
 loop { match pc {
 0x4887d8 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4d6be4u32, v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x5d6a3cu32.wrapping_add(cpu.esi.wrapping_mul(4))) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x5d6a3eu32.wrapping_add(cpu.esi.wrapping_mul(4))) as u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.esi)).wrapping_sub(0x8u16); cpu.w16(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x38u32.wrapping_add(cpu.esi)).wrapping_sub(0x8u16); cpu.w16(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x488823; continue;
 }
 0x488823 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x488889; continue; }
  pc = 0x488828; continue;
 }
 0x488828 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x576bf7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x488864; continue; }
  pc = 0x488847; continue;
 }
 0x488847 => {
  { cpu.sub8(cpu.r8(0x36u32.wrapping_add(cpu.ebp)), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x488864; continue; }
  pc = 0x48884d; continue;
 }
 0x48884d => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x488861); f_0047f6d0(cpu); if cpu.returned_to != 0x488861 { return; }
  { cpu.popad(); }
  pc = 0x488884; continue;
 }
 0x488864 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4d6be4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488877); f_00458a0c(cpu); if cpu.returned_to != 0x488877 { return; }
  cpu.push(0x48887c); f_0048990c(cpu); if cpu.returned_to != 0x48887c { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x18u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488884; continue;
 }
 0x488884 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x488889; continue;
 }
 0x488889 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x488823; continue; }
  pc = 0x48888e; continue;
 }
 0x48888e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048888f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48888f); }
 let mut pc: u32 = 0x48888f;
 loop { match pc {
 0x48888f => {
  { let v: u32 = cpu.ebx.wrapping_add(0x14u32); cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x48889b; continue;
 }
 0x48889b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4888ca; continue; }
  pc = 0x4888a0; continue;
 }
 0x4888a0 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { cpu.and8(cpu.r8(0x509317u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x4888bf; continue; }
  pc = 0x4888ad; continue;
 }
 0x4888ad => {
  { cpu.and8(cpu.r8(0x509317u32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x4888ca; continue; }
  pc = 0x4888b6; continue;
 }
 0x4888b6 => {
  { cpu.and8(cpu.r8(0x5092c1u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x4888ca; continue; }
  pc = 0x4888bf; continue;
 }
 0x4888bf => {
  { cpu.sub8(cpu.r8(0x5092ebu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4888ca; continue; }
  pc = 0x4888c8; continue;
 }
 0x4888c8 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4888ca; continue;
 }
 0x4888ca => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48889b; continue; }
  pc = 0x4888cd; continue;
 }
 0x4888cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488be6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488be6); }
 let mut pc: u32 = 0x488be6;
 loop { match pc {
 0x488be6 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x488c01; continue; }
  pc = 0x488bf1; continue;
 }
 0x488bf1 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x488bf7); f_004184ca(cpu); if cpu.returned_to != 0x488bf7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x488c01; continue; }
  pc = 0x488bfc; continue;
 }
 0x488bfc => {
  cpu.push(0x488c01); f_0041879e(cpu); if cpu.returned_to != 0x488c01 { return; }
  pc = 0x488c01; continue;
 }
 0x488c01 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d688au32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x3du8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488c15); f_00488c58(cpu); if cpu.returned_to != 0x488c15 { return; }
  if cpu.cf { pc = 0x488c51; continue; }
  pc = 0x488c17; continue;
 }
 0x488c17 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x21u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x488c51; continue; }
  pc = 0x488c32; continue;
 }
 0x488c32 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xafu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488c51 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c58); }
 let mut pc: u32 = 0x488c58;
 loop { match pc {
 0x488c58 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x488c6b; continue;
 }
 0x488c6b => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x488c7b; continue; }
  pc = 0x488c72; continue;
 }
 0x488c72 => {
  { cpu.sub8(cpu.r8(0xafu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x488c86; continue; }
  pc = 0x488c7b; continue;
 }
 0x488c7b => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x488c6b; continue; }
  pc = 0x488c83; continue;
 }
 0x488c83 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488c86 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x488c7b; continue; }
  pc = 0x488c8c; continue;
 }
 0x488c8c => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c8f); }
 let mut pc: u32 = 0x488c8f;
 loop { match pc {
 0x488c8f => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x55u8; cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488cb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488cb7); }
 let mut pc: u32 = 0x488cb7;
 loop { match pc {
 0x488cb7 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x488cbd); f_0045b14e(cpu); if cpu.returned_to != 0x488cbd { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x488d1f; continue; }
  pc = 0x488cc4; continue;
 }
 0x488cc4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x19u8); }
  if cpu.zf { pc = 0x488cd2; continue; }
  pc = 0x488cd0; continue;
 }
 0x488cd0 => {
  pc = 0x488cec; continue;
 }
 0x488cd2 => {
  { let v: u16 = cpu.r16(0x5d6b4cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x5d6b4eu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x488cec; continue;
 }
 0x488cec => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x488cff; continue; }
  pc = 0x488cfa; continue;
 }
 0x488cfa => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x488d04; continue; }
  pc = 0x488cff; continue;
 }
 0x488cff => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x488d04; continue;
 }
 0x488d04 => {
  { let v: u8 = 0x6u8; cpu.w8(0x15u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x581bffu32, v); }
  cpu.push(0x488d15); f_0047f7c2(cpu); if cpu.returned_to != 0x488d15 { return; }
  cpu.push(0x488d1a); f_0047fcdb(cpu); if cpu.returned_to != 0x488d1a { return; }
  cpu.push(0x488d1f); f_0047a0b2(cpu); if cpu.returned_to != 0x488d1f { return; }
  pc = 0x488d1f; continue;
 }
 0x488d1f => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6a28u32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x488d45); f_0041b794(cpu); if cpu.returned_to != 0x488d45 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488d4f); f_00489968(cpu); if cpu.returned_to != 0x488d4f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x488d55); f_00416b1d(cpu); if cpu.returned_to != 0x488d55 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488d8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488d8e); }
 let mut pc: u32 = 0x488d8e;
 loop { match pc {
 0x488d8e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x488dd6; continue; }
  pc = 0x488d95; continue;
 }
 0x488d95 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fc8u32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x488dd7; continue; }
  pc = 0x488da8; continue;
 }
 0x488da8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x488dd1; continue; }
  pc = 0x488dc8; continue;
 }
 0x488dc8 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xdfu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x488dd1; continue;
 }
 0x488dd1 => {
  cpu.push(0x488dd6); f_0044b9aa(cpu); if cpu.returned_to != 0x488dd6 { return; }
  pc = 0x488dd6; continue;
 }
 0x488dd6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488dd7 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x488e08; continue; }
  pc = 0x488df9; continue;
 }
 0x488df9 => {
  cpu.push(0x488dfe); f_004899f2(cpu); if cpu.returned_to != 0x488dfe { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x488e08; continue; }
  pc = 0x488e02; continue;
 }
 0x488e02 => {
  cpu.push(0x488e07); f_0048990c(cpu); if cpu.returned_to != 0x488e07 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488e08 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x488e1a; continue; }
  pc = 0x488e0f; continue;
 }
 0x488e0f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) & 0xdfu8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x488e1a; continue;
 }
 0x488e1a => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x488e54; continue; }
  pc = 0x488e2a; continue;
 }
 0x488e2a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x488e38; continue; }
  pc = 0x488e33; continue;
 }
 0x488e33 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x488e3d; continue; }
  pc = 0x488e38; continue;
 }
 0x488e38 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x488e3d; continue;
 }
 0x488e3d => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x488e4e; continue; }
  pc = 0x488e48; continue;
 }
 0x488e48 => {
  cpu.push(0x488e4d); f_0046a929(cpu); if cpu.returned_to != 0x488e4d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488e4e => {
  cpu.push(0x488e53); f_0044b9aa(cpu); if cpu.returned_to != 0x488e53 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488e54 => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489238(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489238); } r_00489238(cpu, 0x489238); }
pub fn f_00489286(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489286); } r_00489238(cpu, 0x489286); }
pub fn f_004892a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4892a9); } r_00489238(cpu, 0x4892a9); }
pub fn f_00489301(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489301); } r_00489238(cpu, 0x489301); }
fn r_00489238(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x489238 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { pc = 0x489284; continue; }
  pc = 0x489241; continue;
 }
 0x489241 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48925d; continue; }
  pc = 0x48924a; continue;
 }
 0x48924a => {
  { cpu.and8(cpu.r8(0x4d685au32), 0xfu8); }
  if cpu.zf { pc = 0x489284; continue; }
  pc = 0x489253; continue;
 }
 0x489253 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x489284; continue; }
  pc = 0x48925d; continue;
 }
 0x48925d => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x48926e; continue; }
  pc = 0x489261; continue;
 }
 0x489261 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x489278; continue;
 }
 0x48926e => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x489278; continue;
 }
 0x489278 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b80u32, v); }
  { cpu.cf = true; }
  pc = 0x489285; continue;
 }
 0x489284 => {
  { cpu.cf = false; }
  pc = 0x489285; continue;
 }
 0x489285 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489286 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x48928f; continue;
 }
 0x48928f => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x489298; continue;
 }
 0x489298 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x48925d; continue; }
  pc = 0x4892a7; continue;
 }
 0x4892a7 => {
  cpu.poll(); pc = 0x48924a; continue;
 }
 0x4892a9 => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x4892b2; continue;
 }
 0x4892b2 => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x4892bb; continue;
 }
 0x4892bb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { pc = 0x4892ce; continue; }
  pc = 0x4892c5; continue;
 }
 0x4892c5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4892e6; continue; }
  pc = 0x4892ce; continue;
 }
 0x4892ce => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x4892d7; continue;
 }
 0x4892d7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x4892e6; continue;
 }
 0x4892e6 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4892f0; continue; }
  pc = 0x4892ea; continue;
 }
 0x4892ea => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4892f0; continue;
 }
 0x4892f0 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b7cu32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x489285; continue;
 }
 0x489301 => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x489284; continue; }
  pc = 0x48930e; continue;
 }
 0x48930e => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4892ce; continue; }
  pc = 0x489317; continue;
 }
 0x489317 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x4892e6; continue; }
  pc = 0x489320; continue;
 }
 0x489320 => {
  cpu.poll(); pc = 0x4892ce; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004893c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4893c2); } r_004893c2(cpu, 0x4893c2); }
pub fn f_00489440(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489440); } r_004893c2(cpu, 0x489440); }
pub fn f_004894be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4894be); } r_004893c2(cpu, 0x4894be); }
pub fn f_0048953c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48953c); } r_004893c2(cpu, 0x48953c); }
fn r_004893c2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x488e5a => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fc8u32, v); }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5d6d2cu32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x488fab; continue; }
  pc = 0x488e78; continue;
 }
 0x488e78 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x488e8c; continue; }
  pc = 0x488e7e; continue;
 }
 0x488e7e => {
  cpu.push(0x488e83); f_00489881(cpu); if cpu.returned_to != 0x488e83 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488e8c; continue;
 }
 0x488e8c => {
  cpu.push(0x488e91); f_0045ba83(cpu); if cpu.returned_to != 0x488e91 { return; }
  if !cpu.cf { pc = 0x488e99; continue; }
  pc = 0x488e93; continue;
 }
 0x488e93 => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488e99 => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x488f3d; continue; }
  pc = 0x488ea5; continue;
 }
 0x488ea5 => {
  { cpu.sub8(cpu.r8(0x4e1fa8u32), 0x0u8); }
  if !cpu.zf { pc = 0x488f3d; continue; }
  pc = 0x488eb2; continue;
 }
 0x488eb2 => {
  cpu.push(0x488eb7); f_0045b70e(cpu); if cpu.returned_to != 0x488eb7 { return; }
  if !cpu.cf { pc = 0x488ed0; continue; }
  pc = 0x488eb9; continue;
 }
 0x488eb9 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488ed0 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x488ede; continue; }
  pc = 0x488ed7; continue;
 }
 0x488ed7 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488f3d; continue;
 }
 0x488ede => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x488f58; continue; }
  pc = 0x488ee5; continue;
 }
 0x488ee5 => {
  if cpu.sf { pc = 0x4890c3; continue; }
  pc = 0x488eeb; continue;
 }
 0x488eeb => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x488f58; continue; }
  pc = 0x488ef2; continue;
 }
 0x488ef2 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x488f58; continue; }
  pc = 0x488efc; continue;
 }
 0x488efc => {
  { cpu.sub8(cpu.r8(0x64u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x4890cf; continue; }
  pc = 0x488f07; continue;
 }
 0x488f07 => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x488f17; continue; }
  pc = 0x488f0c; continue;
 }
 0x488f0c => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  pc = 0x488f17; continue;
 }
 0x488f17 => {
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x488f27; continue; }
  pc = 0x488f1c; continue;
 }
 0x488f1c => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  pc = 0x488f27; continue;
 }
 0x488f27 => {
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x488f38; continue; }
  pc = 0x488f2e; continue;
 }
 0x488f2e => {
  cpu.push(0x488f33); f_00489968(cpu); if cpu.returned_to != 0x488f33 { return; }
  f_00488d8e(cpu); return;
 }
 0x488f38 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x64u32.wrapping_add(cpu.ebp))); cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488f3d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  f_00488d8e(cpu); return;
 }
 0x488f58 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x488f3d; continue; }
  pc = 0x488f63; continue;
 }
 0x488f63 => {
  cpu.push(0x488f68); f_004899b0(cpu); if cpu.returned_to != 0x488f68 { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b7cu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x488f7b; continue; }
  pc = 0x488f76; continue;
 }
 0x488f76 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  pc = 0x488f7b; continue;
 }
 0x488f7b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b80u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x488f8e; continue; }
  pc = 0x488f89; continue;
 }
 0x488f89 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  pc = 0x488f8e; continue;
 }
 0x488f8e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { cpu.poll(); pc = 0x488f3d; continue; }
  pc = 0x488fa5; continue;
 }
 0x488fa5 => {
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x488f3d; continue;
 }
 0x488fab => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x4890c8; continue; }
  pc = 0x488fb7; continue;
 }
 0x488fb7 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x488fc8; continue; }
  pc = 0x488fbe; continue;
 }
 0x488fbe => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x488f3d; continue;
 }
 0x488fc8 => {
  { cpu.sub8(cpu.r8(0x4e1fc1u32), 0x0u8); }
  if !cpu.zf { pc = 0x488fde; continue; }
  pc = 0x488fd1; continue;
 }
 0x488fd1 => {
  { cpu.sub8(cpu.r8(0x4e1fa8u32), 0x0u8); }
  if !cpu.zf { pc = 0x489069; continue; }
  pc = 0x488fde; continue;
 }
 0x488fde => {
  cpu.push(0x488fe3); f_0045b70e(cpu); if cpu.returned_to != 0x488fe3 { return; }
  if !cpu.cf { pc = 0x488ffc; continue; }
  pc = 0x488fe5; continue;
 }
 0x488fe5 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x488ffc => {
  cpu.push(0x489001); f_0045b8fc(cpu); if cpu.returned_to != 0x489001 { return; }
  if !cpu.cf { pc = 0x489068; continue; }
  pc = 0x489003; continue;
 }
 0x489003 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  cpu.push(0x48901e); f_004899b0(cpu); if cpu.returned_to != 0x48901e { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b7cu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x489031; continue; }
  pc = 0x48902c; continue;
 }
 0x48902c => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  pc = 0x489031; continue;
 }
 0x489031 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b80u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x489044; continue; }
  pc = 0x48903f; continue;
 }
 0x48903f => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  pc = 0x489044; continue;
 }
 0x489044 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x489068; continue; }
  pc = 0x489051; continue;
 }
 0x489051 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48905f; continue; }
  pc = 0x48905a; continue;
 }
 0x48905a => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x489064; continue; }
  pc = 0x48905f; continue;
 }
 0x48905f => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x489064; continue;
 }
 0x489064 => {
  { let v: u8 = cpu.or8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x2u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x489068; continue;
 }
 0x489068 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489069 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  { cpu.sub8(cpu.r8(0x5da9b0u32), 0x0u8); }
  if !cpu.zf { pc = 0x489089; continue; }
  pc = 0x489088; continue;
 }
 0x489088 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489089 => {
  { let v: u32 = cpu.r32(0x4d6be8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4890b8); f_00458a7f(cpu); if cpu.returned_to != 0x4890b8 { return; }
  cpu.push(0x4890bd); f_0048990c(cpu); if cpu.returned_to != 0x4890bd { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4890c3 => {
  cpu.poll(); pc = 0x488f58; continue;
 }
 0x4890c8 => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4890cf => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x488f58; continue; }
  pc = 0x4890e2; continue;
 }
 0x4890e2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf8u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489113; continue; }
  pc = 0x489110; continue;
 }
 0x489110 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x489113; continue;
 }
 0x489113 => {
  { cpu.sub16((cpu.eax as u16), 0x60u16); }
  if !cpu.cf { pc = 0x48921d; continue; }
  pc = 0x48911d; continue;
 }
 0x48911d => {
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48912a; continue; }
  pc = 0x489127; continue;
 }
 0x489127 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48912a; continue;
 }
 0x48912a => {
  { cpu.sub16((cpu.eax as u16), 0x60u16); }
  if !cpu.cf { pc = 0x48921d; continue; }
  pc = 0x489134; continue;
 }
 0x489134 => {
  { let v = cpu.r16(0x4d68e8u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d68eau32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d68eeu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d68ecu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6860u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x4d6a70u32); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x489165); f_00476911(cpu); if cpu.returned_to != 0x489165 { return; }
  pc = 0x489165; continue;
 }
 0x489165 => {
  { let v: u16 = cpu.r16(0x4d68e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d68eau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4891a3; continue; }
  pc = 0x4891a0; continue;
 }
 0x4891a0 => {
  { cpu.cf = true; }
  pc = 0x4891cb; continue;
 }
 0x4891a3 => {
  { cpu.sub16(cpu.r16(0x4d68ecu32), 0x0u16); }
  if cpu.zf { pc = 0x4891ca; continue; }
  pc = 0x4891ad; continue;
 }
 0x4891ad => {
  { let v: u8 = 0x10u8; cpu.w8(0x4d6860u32, v); }
  { let v: u16 = cpu.r16(0x4d68eeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6a70u32); cpu.push(0x4891c0); cpu.call(t); if cpu.returned_to != 0x4891c0 { return; }
  { let v: u16 = cpu.r16(0x4d68ecu32).wrapping_sub(0x10u16); cpu.w16(0x4d68ecu32, v); }
  cpu.poll(); pc = 0x489165; continue;
 }
 0x4891ca => {
  { cpu.cf = false; }
  pc = 0x4891cb; continue;
 }
 0x4891cb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a70u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6860u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d68ecu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d68eeu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d68eau32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d68e8u32, v); }
  if cpu.cf { cpu.poll(); pc = 0x488f58; continue; }
  pc = 0x4891fd; continue;
 }
 0x4891fd => {
  cpu.push(0x489202); f_00489968(cpu); if cpu.returned_to != 0x489202 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  f_00488d8e(cpu); return;
 }
 0x48921d => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x489229; continue; }
  pc = 0x489224; continue;
 }
 0x489224 => {
  cpu.poll(); pc = 0x488f58; continue;
 }
 0x489229 => {
  cpu.push(0x48922e); f_00489968(cpu); if cpu.returned_to != 0x48922e { return; }
  cpu.push(0x489233); f_0043fced(cpu); if cpu.returned_to != 0x489233 { return; }
  cpu.poll(); pc = 0x488f58; continue;
 }
 0x4893c2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48943f; continue; }
  pc = 0x4893e2; continue;
 }
 0x4893e2 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x4893e8; continue; }
  pc = 0x4893e6; continue;
 }
 0x4893e6 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x4893e8; continue;
 }
 0x4893e8 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x4893fc; continue;
 }
 0x4893fc => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4893fd; continue;
 }
 0x4893fd => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489410; continue; }
  pc = 0x489402; continue;
 }
 0x489402 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x489410; continue;
 }
 0x489410 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4893fd; continue; }
  pc = 0x489416; continue;
 }
 0x489416 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  let t = cpu.r32(0x4d6a7cu32); cpu.push(0x489428); cpu.call(t); if cpu.returned_to != 0x489428 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x488e5a; continue; }
  pc = 0x48942f; continue;
 }
 0x48942f => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x48943f; continue;
 }
 0x48943f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489440 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4894bd; continue; }
  pc = 0x489460; continue;
 }
 0x489460 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x489466; continue; }
  pc = 0x489464; continue;
 }
 0x489464 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x489466; continue;
 }
 0x489466 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x48947a; continue;
 }
 0x48947a => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x48947b; continue;
 }
 0x48947b => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489488; continue; }
  pc = 0x489480; continue;
 }
 0x489480 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x489488; continue;
 }
 0x489488 => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48947b; continue; }
  pc = 0x489494; continue;
 }
 0x489494 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  let t = cpu.r32(0x4d6a7cu32); cpu.push(0x4894a6); cpu.call(t); if cpu.returned_to != 0x4894a6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x488e5a; continue; }
  pc = 0x4894ad; continue;
 }
 0x4894ad => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x4894bd; continue;
 }
 0x4894bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4894be => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48953b; continue; }
  pc = 0x4894de; continue;
 }
 0x4894de => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x4894e4; continue; }
  pc = 0x4894e2; continue;
 }
 0x4894e2 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x4894e4; continue;
 }
 0x4894e4 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x4894f8; continue;
 }
 0x4894f8 => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4894f9; continue;
 }
 0x4894f9 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48950c; continue; }
  pc = 0x4894fe; continue;
 }
 0x4894fe => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x48950c; continue;
 }
 0x48950c => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4894f9; continue; }
  pc = 0x489512; continue;
 }
 0x489512 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  let t = cpu.r32(0x4d6a7cu32); cpu.push(0x489524); cpu.call(t); if cpu.returned_to != 0x489524 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x488e5a; continue; }
  pc = 0x48952b; continue;
 }
 0x48952b => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x48953b; continue;
 }
 0x48953b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48953c => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4895bd; continue; }
  pc = 0x489560; continue;
 }
 0x489560 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x489566; continue; }
  pc = 0x489564; continue;
 }
 0x489564 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x489566; continue;
 }
 0x489566 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x48957a; continue;
 }
 0x48957a => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x48957b; continue;
 }
 0x48957b => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489588; continue; }
  pc = 0x489580; continue;
 }
 0x489580 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x489588; continue;
 }
 0x489588 => {
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d685cu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48957b; continue; }
  pc = 0x489594; continue;
 }
 0x489594 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d685au32, v); }
  let t = cpu.r32(0x4d6a7cu32); cpu.push(0x4895a6); cpu.call(t); if cpu.returned_to != 0x4895a6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x488e5a; continue; }
  pc = 0x4895ad; continue;
 }
 0x4895ad => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_00488d8e(cpu); return; }
  pc = 0x4895bd; continue;
 }
 0x4895bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004895be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895be); }
 let mut pc: u32 = 0x4895be;
 loop { match pc {
 0x4895be => {
  { let v: u8 = 0x4u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004895c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895c7); }
 let mut pc: u32 = 0x4895c7;
 loop { match pc {
 0x4895c7 => {
  { let v: u8 = 0x2u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004895d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895d0); }
 let mut pc: u32 = 0x4895d0;
 loop { match pc {
 0x4895d0 => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4895e2; continue; }
  pc = 0x4895da; continue;
 }
 0x4895da => {
  { let v: u8 = 0x0u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4895e2; continue;
 }
 0x4895e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004895e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895e3); }
 let mut pc: u32 = 0x4895e3;
 loop { match pc {
 0x4895e3 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x489630; continue; }
  pc = 0x4895ea; continue;
 }
 0x4895ea => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x489621); f_00458a0c(cpu); if cpu.returned_to != 0x489621 { return; }
  cpu.push(0x489626); f_004895be(cpu); if cpu.returned_to != 0x489626 { return; }
  cpu.push(0x48962b); f_0048990c(cpu); if cpu.returned_to != 0x48962b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  pc = 0x489630; continue;
 }
 0x489630 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489637(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489637); }
 let mut pc: u32 = 0x489637;
 loop { match pc {
 0x489637 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x489647; continue; }
  pc = 0x48963e; continue;
 }
 0x48963e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x489647; continue; }
  pc = 0x489645; continue;
 }
 0x489645 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489647 => {
  { cpu.pushad(); }
  cpu.push(0x48964d); f_0046a9bc(cpu); if cpu.returned_to != 0x48964d { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x489695; continue; }
  pc = 0x48965a; continue;
 }
 0x48965a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x489695; continue; }
  pc = 0x489661; continue;
 }
 0x489661 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x489645; continue; }
  pc = 0x489668; continue;
 }
 0x489668 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x489645; continue; }
  pc = 0x48966f; continue;
 }
 0x48966f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x489645; continue; }
  pc = 0x489676; continue;
 }
 0x489676 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { cpu.poll(); pc = 0x489645; continue; }
  pc = 0x48967d; continue;
 }
 0x48967d => {
  { let v: u8 = 0x2u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xfcu8); }
  if cpu.zf { pc = 0x48968e; continue; }
  pc = 0x489689; continue;
 }
 0x489689 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48968e; continue;
 }
 0x48968e => {
  cpu.push(0x489693); f_0048985c(cpu); if cpu.returned_to != 0x489693 { return; }
  pc = 0x4896a6; continue;
 }
 0x489695 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4896a6; continue; }
  pc = 0x48969e; continue;
 }
 0x48969e => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x4896a6; continue; }
  pc = 0x4896a2; continue;
 }
 0x4896a2 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ecx)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4896a6; continue;
 }
 0x4896a6 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x4896df; continue; }
  pc = 0x4896ad; continue;
 }
 0x4896ad => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4896df; continue; }
  pc = 0x4896b4; continue;
 }
 0x4896b4 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4896c9); f_00459674(cpu); if cpu.returned_to != 0x4896c9 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x4896df; continue; }
  pc = 0x4896db; continue;
 }
 0x4896db => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  pc = 0x4896df; continue;
 }
 0x4896df => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489784; continue; }
  pc = 0x4896ea; continue;
 }
 0x4896ea => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4896fb; continue; }
  pc = 0x4896f6; continue;
 }
 0x4896f6 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4896fb; continue;
 }
 0x4896fb => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x489744; continue; }
  pc = 0x489704; continue;
 }
 0x489704 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489736; continue; }
  pc = 0x489725; continue;
 }
 0x489725 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489813; continue; }
  pc = 0x489731; continue;
 }
 0x489731 => {
  pc = 0x48980c; continue;
 }
 0x489736 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48981a; continue; }
  pc = 0x48973f; continue;
 }
 0x48973f => {
  pc = 0x489813; continue;
 }
 0x489744 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489776; continue; }
  pc = 0x489765; continue;
 }
 0x489765 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x489805; continue; }
  pc = 0x489771; continue;
 }
 0x489771 => {
  pc = 0x48980c; continue;
 }
 0x489776 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x489821; continue; }
  pc = 0x48977f; continue;
 }
 0x48977f => {
  pc = 0x48981a; continue;
 }
 0x489784 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489798; continue; }
  pc = 0x489793; continue;
 }
 0x489793 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x489798; continue;
 }
 0x489798 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x4897d3; continue; }
  pc = 0x4897a1; continue;
 }
 0x4897a1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4897cc; continue; }
  pc = 0x4897c2; continue;
 }
 0x4897c2 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48982f; continue; }
  pc = 0x4897ca; continue;
 }
 0x4897ca => {
  pc = 0x489836; continue;
 }
 0x4897cc => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48982f; continue; }
  pc = 0x4897d1; continue;
 }
 0x4897d1 => {
  pc = 0x489828; continue;
 }
 0x4897d3 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4897fe; continue; }
  pc = 0x4897f4; continue;
 }
 0x4897f4 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x489805; continue; }
  pc = 0x4897fc; continue;
 }
 0x4897fc => {
  pc = 0x489836; continue;
 }
 0x4897fe => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x489821; continue; }
  pc = 0x489803; continue;
 }
 0x489803 => {
  pc = 0x489828; continue;
 }
 0x489805 => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x48980c => {
  { let v: u8 = 0x1u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x489813 => {
  { let v: u8 = 0x2u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x48981a => {
  { let v: u8 = 0x3u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x489821 => {
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x489828 => {
  { let v: u8 = 0x5u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x48982f => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x489836 => {
  { let v: u8 = 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48983b; continue;
 }
 0x48983b => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x48984a; continue; }
  pc = 0x489843; continue;
 }
 0x489843 => {
  { let v: u16 = 0x1u16; cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48984a; continue;
 }
 0x48984a => {
  { let v: u8 = 0xffu8; cpu.w8(0x9eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048985c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48985c); }
 let mut pc: u32 = 0x48985c;
 loop { match pc {
 0x48985c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4eabacu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48987b; continue; }
  pc = 0x489873; continue;
 }
 0x489873 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48987b; continue; }
  pc = 0x489879; continue;
 }
 0x489879 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48987b => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489881(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489881); }
 let mut pc: u32 = 0x489881;
 loop { match pc {
 0x489881 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4eabacu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489897(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489897); }
 let mut pc: u32 = 0x489897;
 loop { match pc {
 0x489897 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x48990b; continue; }
  pc = 0x48989e; continue;
 }
 0x48989e => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x45bac8u32; cpu.w32(0x4d6a7cu32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4898d5; continue; }
  pc = 0x4898cb; continue;
 }
 0x4898cb => {
  { let v: u32 = 0x48bf23u32; cpu.w32(0x4d6a7cu32, v); }
  pc = 0x4898d5; continue;
 }
 0x4898d5 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x5b9c9cu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x4898e5); cpu.call(t); if cpu.returned_to != 0x4898e5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48990b); f_0046d574(cpu); if cpu.returned_to != 0x48990b { return; }
  pc = 0x48990b; continue;
 }
 0x48990b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048990c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48990c); }
 let mut pc: u32 = 0x48990c;
 loop { match pc {
 0x48990c => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x489966; continue; }
  pc = 0x489913; continue;
 }
 0x489913 => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x489931; continue; }
  pc = 0x489928; continue;
 }
 0x489928 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489931; continue;
 }
 0x489931 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x489966; continue; }
  pc = 0x489935; continue;
 }
 0x489935 => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x489953; continue; }
  pc = 0x489945; continue;
 }
 0x489945 => {
  cpu.push(0x48994a); f_004895d0(cpu); if cpu.returned_to != 0x48994a { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x489952); f_0045955b(cpu); if cpu.returned_to != 0x489952 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x489953; continue;
 }
 0x489953 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x489960); f_00489637(cpu); if cpu.returned_to != 0x489960 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489966 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489968); }
 let mut pc: u32 = 0x489968;
 loop { match pc {
 0x489968 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x489976; continue; }
  pc = 0x48996f; continue;
 }
 0x48996f => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x48999f; continue; }
  pc = 0x489976; continue;
 }
 0x489976 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x14u32); cpu.ebp = v; }
  cpu.push(0x48997e); f_004899c1(cpu); if cpu.returned_to != 0x48997e { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x14u32); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x48999e; continue; }
  pc = 0x48998b; continue;
 }
 0x48998b => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x489995; continue; }
  pc = 0x489990; continue;
 }
 0x489990 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48999a; continue; }
  pc = 0x489995; continue;
 }
 0x489995 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48999a; continue;
 }
 0x48999a => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  pc = 0x48999e; continue;
 }
 0x48999e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x48999f; continue;
 }
 0x48999f => {
  { let v: u32 = cpu.ebp.wrapping_add(0xcu32); cpu.ebp = v; }
  cpu.push(0x4899a7); f_004899c1(cpu); if cpu.returned_to != 0x4899a7 { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0xcu32); cpu.ebp = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004899b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4899b0); }
 let mut pc: u32 = 0x4899b0;
 loop { match pc {
 0x4899b0 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xcu32); cpu.ebp = v; }
  cpu.push(0x4899b8); f_004899c1(cpu); if cpu.returned_to != 0x4899b8 { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0xcu32); cpu.ebp = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004899c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4899c1); }
 let mut pc: u32 = 0x4899c1;
 loop { match pc {
 0x4899c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  pc = 0x4899c5; continue;
 }
 0x4899c5 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x4899e7; continue; }
  pc = 0x4899c9; continue;
 }
 0x4899c9 => {
  { cpu.sub32(cpu.ebx, 0x4ba552u32); }
  if cpu.cf { pc = 0x4899f0; continue; }
  pc = 0x4899d1; continue;
 }
 0x4899d1 => {
  { cpu.sub32(cpu.ebx, 0x4d6752u32); }
  if !cpu.cf { pc = 0x4899f0; continue; }
  pc = 0x4899d9; continue;
 }
 0x4899d9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.poll(); pc = 0x4899c5; continue;
 }
 0x4899e7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4899f0 => {
  cpu.poll(); pc = 0x4899e7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004899f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4899f2); }
 let mut pc: u32 = 0x4899f2;
 loop { match pc {
 0x4899f2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x489a06; continue; }
  pc = 0x4899f9; continue;
 }
 0x4899f9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x489a0e; continue; }
  pc = 0x489a01; continue;
 }
 0x489a01 => {
  pc = 0x489a92; continue;
 }
 0x489a06 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x489a64; continue; }
  pc = 0x489a0e; continue;
 }
 0x489a0e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x489a2b; continue; }
  pc = 0x489a1c; continue;
 }
 0x489a1c => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x489a2f; continue; }
  pc = 0x489a2b; continue;
 }
 0x489a2b => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489a2f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x489a81; continue; }
  pc = 0x489a40; continue;
 }
 0x489a40 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfdu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x489a64; continue; }
  pc = 0x489a4d; continue;
 }
 0x489a4d => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x489a64; continue; }
  pc = 0x489a51; continue;
 }
 0x489a51 => {
  cpu.push(0x489a56); f_0042f53c(cpu); if cpu.returned_to != 0x489a56 { return; }
  cpu.push(0x489a5b); f_00489af0(cpu); if cpu.returned_to != 0x489a5b { return; }
  cpu.push(0x489a60); f_00489abb(cpu); if cpu.returned_to != 0x489a60 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489a64 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x489a7d; continue; }
  pc = 0x489a73; continue;
 }
 0x489a73 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xf7u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x489a7d; continue;
 }
 0x489a7d => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489a81 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x489a92; continue; }
  pc = 0x489a85; continue;
 }
 0x489a85 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x489a92; continue; }
  pc = 0x489a8a; continue;
 }
 0x489a8a => {
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489a92 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x489a06; continue; }
  pc = 0x489aaa; continue;
 }
 0x489aaa => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x489a06; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489abb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489abb); }
 let mut pc: u32 = 0x489abb;
 loop { match pc {
 0x489abb => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x489aea; continue; }
  pc = 0x489ac3; continue;
 }
 0x489ac3 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x489ad1); f_004713ce(cpu); if cpu.returned_to != 0x489ad1 { return; }
  if !cpu.cf { pc = 0x489aea; continue; }
  pc = 0x489ad3; continue;
 }
 0x489ad3 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x40u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x40u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489aec; continue; }
  pc = 0x489ae3; continue;
 }
 0x489ae3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x489aea); f_00458a7f(cpu); if cpu.returned_to != 0x489aea { return; }
  pc = 0x489aea; continue;
 }
 0x489aea => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489aec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489af0); }
 let mut pc: u32 = 0x489af0;
 loop { match pc {
 0x489af0 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6bf4u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x489b07); f_004712e3(cpu); if cpu.returned_to != 0x489b07 { return; }
  if !cpu.cf { pc = 0x489b2c; continue; }
  pc = 0x489b09; continue;
 }
 0x489b09 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x489b2c; continue; }
  pc = 0x489b15; continue;
 }
 0x489b15 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x40u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x40u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489b2c; continue; }
  pc = 0x489b27; continue;
 }
 0x489b27 => {
  cpu.push(0x489b2c); f_00458a7f(cpu); if cpu.returned_to != 0x489b2c { return; }
  pc = 0x489b2c; continue;
 }
 0x489b2c => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489b34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489b34); }
 let mut pc: u32 = 0x489b34;
 loop { match pc {
 0x489b34 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x489b51; continue; }
  pc = 0x489b3d; continue;
 }
 0x489b3d => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x1u8); }
  if cpu.zf { pc = 0x489b6f; continue; }
  pc = 0x489b46; continue;
 }
 0x489b46 => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x2u8); }
  if cpu.zf { pc = 0x489b6d; continue; }
  pc = 0x489b4f; continue;
 }
 0x489b4f => {
  pc = 0x489b65; continue;
 }
 0x489b51 => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x1u8); }
  if cpu.zf { pc = 0x489b6f; continue; }
  pc = 0x489b5a; continue;
 }
 0x489b5a => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x2u8); }
  if cpu.zf { pc = 0x489b6d; continue; }
  pc = 0x489b63; continue;
 }
 0x489b63 => {
  pc = 0x489b65; continue;
 }
 0x489b65 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x489b6f; continue; }
  pc = 0x489b69; continue;
 }
 0x489b69 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x489b6f; continue;
 }
 0x489b6d => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x489b6f; continue;
 }
 0x489b6f => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489b7a; continue; }
  pc = 0x489b78; continue;
 }
 0x489b78 => {
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x489b7a; continue;
 }
 0x489b7a => {
  { cpu.sub8(cpu.r8(0x88u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x489bb4; continue; }
  pc = 0x489b83; continue;
 }
 0x489b83 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x88u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.sf { pc = 0x489b90; continue; }
  pc = 0x489b8e; continue;
 }
 0x489b8e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x489b90; continue;
 }
 0x489b90 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489b96; continue; }
  pc = 0x489b94; continue;
 }
 0x489b94 => {
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x489b96; continue;
 }
 0x489b96 => {
  { let v: u32 = cpu.r32(0x4eac74u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x489bb5; continue; }
  pc = 0x489ba2; continue;
 }
 0x489ba2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xb2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489bb4; continue;
 }
 0x489bb4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489bb5 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489bfd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489bfd); }
 let mut pc: u32 = 0x489bfd;
 loop { match pc {
 0x489bfd => {
  { cpu.sub16(cpu.r16(0x54u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if !cpu.zf { pc = 0x489c36; continue; }
  pc = 0x489c06; continue;
 }
 0x489c06 => {
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x489c19); f_00489c38(cpu); if cpu.returned_to != 0x489c19 { return; }
  if !cpu.cf { pc = 0x489c36; continue; }
  pc = 0x489c1b; continue;
 }
 0x489c1b => {
  cpu.push(0x489c20); f_00488d8e(cpu); if cpu.returned_to != 0x489c20 { return; }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  cpu.push(0x489c2a); f_0046ab56(cpu); if cpu.returned_to != 0x489c2a { return; }
  cpu.push(0x489c2f); f_00489968(cpu); if cpu.returned_to != 0x489c2f { return; }
  cpu.push(0x489c34); f_0043ffdb(cpu); if cpu.returned_to != 0x489c34 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489c36 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489c38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489c38); }
 let mut pc: u32 = 0x489c38;
 loop { match pc {
 0x489c38 => {
  { cpu.pushad(); }
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
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x489c75; continue; }
  pc = 0x489c72; continue;
 }
 0x489c72 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489c75 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489c78(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489c78); } r_00489c78(cpu, 0x489c78); }
pub fn f_00489ca5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489ca5); } r_00489c78(cpu, 0x489ca5); }
fn r_00489c78(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x489c78 => {
  cpu.push(0x489c7d); f_00489ecd(cpu); if cpu.returned_to != 0x489c7d { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.r16(0x4d6940u32) as u32; cpu.ecx = v; }
  pc = 0x489cde; continue;
 }
 0x489c9a => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d6b14u32); cpu.push(0x489ca1); cpu.call(t); if cpu.returned_to != 0x489ca1 { return; }
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x7u8); }
  if cpu.zf { pc = 0x489cd8; continue; }
  pc = 0x489cce; continue;
 }
 0x489ca5 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x4d6928u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x7u8); }
  if cpu.zf { pc = 0x489cd8; continue; }
  pc = 0x489cce; continue;
 }
 0x489cce => {
  { cpu.sub8((cpu.ebx as u8), 0x33u8); }
  if cpu.zf { pc = 0x489cd8; continue; }
  pc = 0x489cd3; continue;
 }
 0x489cd3 => {
  { cpu.sub8((cpu.ebx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x489cdd; continue; }
  pc = 0x489cd8; continue;
 }
 0x489cd8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x489c9a; continue; }
  pc = 0x489cdb; continue;
 }
 0x489cdb => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489cdd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x489cde; continue;
 }
 0x489cde => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489ce0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489ce0); } r_00489ce0(cpu, 0x489ce0); }
pub fn f_00489d13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489d13); } r_00489ce0(cpu, 0x489d13); }
pub fn f_00489d8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489d8b); } r_00489ce0(cpu, 0x489d8b); }
pub fn f_00489e00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489e00); } r_00489ce0(cpu, 0x489e00); }
pub fn f_00489e05(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x489e05); } r_00489ce0(cpu, 0x489e05); }
fn r_00489ce0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x489ce0 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d692au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6930u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692eu32, v); }
  pc = 0x489e39; continue;
 }
 0x489d13 => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x4d6926u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x489d3b); f_0048a1aa(cpu); if cpu.returned_to != 0x489d3b { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d692eu32, v); }
  cpu.push(0x489d67); f_00420989(cpu); if cpu.returned_to != 0x489d67 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x489d88; continue; }
  pc = 0x489d7c; continue;
 }
 0x489d7c => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6930u32, v); }
  pc = 0x489e39; continue;
 }
 0x489d88 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489d8b => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x5293a0u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x4d6926u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x489db3); f_0048a1aa(cpu); if cpu.returned_to != 0x489db3 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d692eu32, v); }
  cpu.push(0x489ddf); f_00420989(cpu); if cpu.returned_to != 0x489ddf { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x489dfd; continue; }
  pc = 0x489df4; continue;
 }
 0x489df4 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6930u32, v); }
  pc = 0x489e39; continue;
 }
 0x489dfd => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489e00 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x489e21; continue;
 }
 0x489e05 => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x489e21; continue;
 }
 0x489e21 => {
  { let v: u32 = 0x4d6926u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x489e2d); f_0048a1aa(cpu); if cpu.returned_to != 0x489e2d { return; }
  { let v: u32 = 0x4d692cu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x489e39); f_0048a1aa(cpu); if cpu.returned_to != 0x489e39 { return; }
  pc = 0x489e39; continue;
 }
 0x489e39 => {
  cpu.push(0x489e3e); f_00489ecd(cpu); if cpu.returned_to != 0x489e3e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.r16(0x4d6940u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x489e60; continue; }
  pc = 0x489e5d; continue;
 }
 0x489e5d => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489e60 => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d6b14u32); cpu.push(0x489e67); cpu.call(t); if cpu.returned_to != 0x489e67 { return; }
  { let v: u32 = cpu.r16(0x4d6926u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6928u32) as u32; cpu.ebx = v; }
  cpu.push(0x489e7a); f_00489ea2(cpu); if cpu.returned_to != 0x489e7a { return; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5d6d3cu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4d692au32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x489e9e; continue; }
  pc = 0x489e98; continue;
 }
 0x489e98 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x489e60; continue; }
  pc = 0x489e9b; continue;
 }
 0x489e9b => {
  { cpu.cf = false; }
  pc = 0x489ea0; continue;
 }
 0x489e9e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  pc = 0x489ea0; continue;
 }
 0x489ea0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489ea2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489ea2); }
 let mut pc: u32 = 0x489ea2;
 loop { match pc {
 0x489ea2 => {
  { cpu.sub8(cpu.r8(0x4e1fa5u32), 0x0u8); }
  if cpu.zf { pc = 0x489ecc; continue; }
  pc = 0x489eab; continue;
 }
 0x489eab => {
  { cpu.pushad(); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4f1u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x489ecb); cpu.call(t); if cpu.returned_to != 0x489ecb { return; }
  { cpu.popad(); }
  pc = 0x489ecc; continue;
 }
 0x489ecc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489ecd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489ecd); }
 let mut pc: u32 = 0x489ecd;
 loop { match pc {
 0x489ecd => {
  { cpu.sub16(cpu.r16(0x4d692au32), 0x0u16); }
  if cpu.zf { pc = 0x489ede; continue; }
  pc = 0x489ed7; continue;
 }
 0x489ed7 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d692au32)); cpu.w16(0x4d692au32, v); }
  pc = 0x489ede; continue;
 }
 0x489ede => {
  { cpu.sub16(cpu.r16(0x4d6930u32), 0x0u16); }
  if cpu.zf { pc = 0x489eef; continue; }
  pc = 0x489ee8; continue;
 }
 0x489ee8 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6930u32)); cpu.w16(0x4d6930u32, v); }
  pc = 0x489eef; continue;
 }
 0x489eef => {
  { let v: u16 = cpu.shr16(cpu.r16(0x4d6926u32), 4u32); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d6928u32), 4u32); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d692cu32), 4u32); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d692eu32), 4u32); cpu.w16(0x4d692eu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6942u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6944u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6946u32, v); }
  { let v: u16 = cpu.r16(0x4d692cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6930u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6926u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x489f70; continue; }
  pc = 0x489f47; continue;
 }
 0x489f47 => {
  { let a = cpu.r16(0x4d6926u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x4d6926u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let a = cpu.r16(0x4d6928u32); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.w16(0x4d6928u32, v); } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d692eu32, v); }
  { let a = cpu.r16(0x4d692au32); let b = (cpu.ecx as u16); { let v: u16 = b; cpu.w16(0x4d692au32, v); } { let v: u16 = a; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6930u32, v); }
  pc = 0x489f70; continue;
 }
 0x489f70 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6926u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6928u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489f8c; continue; }
  pc = 0x489f80; continue;
 }
 0x489f80 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6944u32, v); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x489f8c; continue;
 }
 0x489f8c => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d692au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x489fa1; continue; }
  pc = 0x489f95; continue;
 }
 0x489f95 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6946u32, v); }
  pc = 0x489fa1; continue;
 }
 0x489fa1 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x489fd9; continue; }
  pc = 0x489fa6; continue;
 }
 0x489fa6 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48a00d; continue; }
  pc = 0x489fab; continue;
 }
 0x489fab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694au32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6948u32, v); }
  { let v: u32 = 0x48a03cu32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x489fd9 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.ebx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48a00d; continue; }
  pc = 0x489fde; continue;
 }
 0x489fde => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d694au32, v); }
  { let v: u32 = 0x48a0b6u32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48a00d => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d694cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694au32, v); }
  { let v: u32 = 0x48a130u32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a03c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a03c); }
 let mut pc: u32 = 0x48a03c;
 loop { match pc {
 0x48a03c => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694au32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694cu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6936u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a07c; continue; }
  pc = 0x48a06e; continue;
 }
 0x48a06e => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48a07c; continue;
 }
 0x48a07c => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a093; continue; }
  pc = 0x48a085; continue;
 }
 0x48a085 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48a093; continue;
 }
 0x48a093 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d694au32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a0b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a0b6); }
 let mut pc: u32 = 0x48a0b6;
 loop { match pc {
 0x48a0b6 => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6948u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694cu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a0f6; continue; }
  pc = 0x48a0e8; continue;
 }
 0x48a0e8 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48a0f6; continue;
 }
 0x48a0f6 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a10d; continue; }
  pc = 0x48a0ff; continue;
 }
 0x48a0ff => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48a10d; continue;
 }
 0x48a10d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a130); }
 let mut pc: u32 = 0x48a130;
 loop { match pc {
 0x48a130 => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6948u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694au32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a170; continue; }
  pc = 0x48a162; continue;
 }
 0x48a162 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48a170; continue;
 }
 0x48a170 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48a187; continue; }
  pc = 0x48a179; continue;
 }
 0x48a179 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48a187; continue;
 }
 0x48a187 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1aa); }
 let mut pc: u32 = 0x48a1aa;
 loop { match pc {
 0x48a1aa => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48a1b5; continue; }
  pc = 0x48a1b3; continue;
 }
 0x48a1b3 => {
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x48a1b5; continue;
 }
 0x48a1b5 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1f0); }
 let mut pc: u32 = 0x48a1f0;
 loop { match pc {
 0x48a1f0 => {
  { let v: u8 = cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a1fd); f_0046ac64(cpu); if cpu.returned_to != 0x48a1fd { return; }
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a203); }
 let mut pc: u32 = 0x48a203;
 loop { match pc {
 0x48a203 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a20d); f_0046ac64(cpu); if cpu.returned_to != 0x48a20d { return; }
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a213); }
 let mut pc: u32 = 0x48a213;
 loop { match pc {
 0x48a213 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a21d); f_0046ac64(cpu); if cpu.returned_to != 0x48a21d { return; }
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a223(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a223); }
 let mut pc: u32 = 0x48a223;
 loop { match pc {
 0x48a223 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a22d); f_0046ac64(cpu); if cpu.returned_to != 0x48a22d { return; }
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a233(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48a233); } r_0048a233(cpu, 0x48a233); }
pub fn f_0048a2e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48a2e8); } r_0048a233(cpu, 0x48a2e8); }
fn r_0048a233(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48a233 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5d6d40u32, v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x48a2cc; continue; }
  pc = 0x48a248; continue;
 }
 0x48a248 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if cpu.zf { pc = 0x48a2cc; continue; }
  pc = 0x48a252; continue;
 }
 0x48a252 => {
  { let v: u16 = cpu.r16(0x8eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5d6d46u32, v); }
  { let v: u16 = (cpu.ecx as u16) & 0x3fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x92u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48a287; continue; }
  pc = 0x48a286; continue;
 }
 0x48a286 => {
  { cpu.cf = true; }
  pc = 0x48a287; continue;
 }
 0x48a287 => {
  cpu.push(0x48a28c); f_0048a80f(cpu); if cpu.returned_to != 0x48a28c { return; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.and8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x48a29e; continue; }
  pc = 0x48a298; continue;
 }
 0x48a298 => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  if cpu.cf { pc = 0x48a2cc; continue; }
  pc = 0x48a29c; continue;
 }
 0x48a29c => {
  pc = 0x48a2c5; continue;
 }
 0x48a29e => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x48a2cc; continue; }
  pc = 0x48a2a9; continue;
 }
 0x48a2a9 => {
  { let v: u8 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x48a2ba; continue; }
  pc = 0x48a2b8; continue;
 }
 0x48a2b8 => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48a2ba; continue;
 }
 0x48a2ba => {
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a2cc; continue;
 }
 0x48a2c5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x83u32.wrapping_add(cpu.ebp))); cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a2cc; continue;
 }
 0x48a2cc => {
  { cpu.popad(); }
  { let v: u16 = cpu.r8(0x92u32.wrapping_add(cpu.ebp)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x48a2e3); f_0041c6a6(cpu); if cpu.returned_to != 0x48a2e3 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if cpu.cf { pc = 0x48a2f5; continue; }
  pc = 0x48a2ed; continue;
 }
 0x48a2e8 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.cf { pc = 0x48a2f5; continue; }
  pc = 0x48a2ed; continue;
 }
 0x48a2ed => {
  { let v: u8 = cpu.inc8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.ebx >> 8) as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x48a2f5; continue;
 }
 0x48a2f5 => {
  { cpu.sub8(cpu.r8(0x4e156au32), 0x0u8); }
  if !cpu.zf { pc = 0x48a335; continue; }
  pc = 0x48a2fe; continue;
 }
 0x48a2fe => {
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if (cpu.sf != cpu.of) { pc = 0x48a307; continue; }
  pc = 0x48a303; continue;
 }
 0x48a303 => {
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x48a335; continue;
 }
 0x48a307 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x48a335; continue; }
  pc = 0x48a311; continue;
 }
 0x48a311 => {
  if cpu.zf { pc = 0x48a322; continue; }
  pc = 0x48a313; continue;
 }
 0x48a313 => {
  cpu.push(0x48a318); f_0041c6a6(cpu); if cpu.returned_to != 0x48a318 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x48a335; continue; }
  pc = 0x48a31e; continue;
 }
 0x48a31e => {
  { let v: u8 = 0x5u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x48a335; continue;
 }
 0x48a322 => {
  cpu.push(0x48a327); f_0041c6a6(cpu); if cpu.returned_to != 0x48a327 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x48a335; continue; }
  pc = 0x48a331; continue;
 }
 0x48a331 => {
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x48a335; continue;
 }
 0x48a335 => {
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x92u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a348); f_0046ac64(cpu); if cpu.returned_to != 0x48a348 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48a356; continue; }
  pc = 0x48a351; continue;
 }
 0x48a351 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48a35b; continue; }
  pc = 0x48a356; continue;
 }
 0x48a356 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48a35b; continue;
 }
 0x48a35b => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x48a387; continue; }
  pc = 0x48a375; continue;
 }
 0x48a375 => {
  cpu.push(0x48a37a); f_004899f2(cpu); if cpu.returned_to != 0x48a37a { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x48a5e0; continue; }
  pc = 0x48a382; continue;
 }
 0x48a382 => {
  pc = 0x48a7d0; continue;
 }
 0x48a387 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x48a3e2; continue; }
  pc = 0x48a391; continue;
 }
 0x48a391 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48a3e2; continue; }
  pc = 0x48a39b; continue;
 }
 0x48a39b => {
  { cpu.and8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x48a3c0; continue; }
  pc = 0x48a3a5; continue;
 }
 0x48a3a5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x96u32.wrapping_add(cpu.ebp))); cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if !cpu.zf { pc = 0x48a3e2; continue; }
  pc = 0x48a3b6; continue;
 }
 0x48a3b6 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a3e2; continue;
 }
 0x48a3c0 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x48a3da; continue; }
  pc = 0x48a3cf; continue;
 }
 0x48a3cf => {
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a3e2; continue;
 }
 0x48a3da => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a3e2; continue;
 }
 0x48a3e2 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x48a7d0; continue; }
  pc = 0x48a3f0; continue;
 }
 0x48a3f0 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48a7d0; continue; }
  pc = 0x48a3fe; continue;
 }
 0x48a3fe => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48a7d0; continue; }
  pc = 0x48a40b; continue;
 }
 0x48a40b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48a419; continue; }
  pc = 0x48a414; continue;
 }
 0x48a414 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48a41e; continue; }
  pc = 0x48a419; continue;
 }
 0x48a419 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48a41e; continue;
 }
 0x48a41e => {
  { let v: u16 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48a533; continue; }
  pc = 0x48a435; continue;
 }
 0x48a435 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x48a7d0; continue; }
  pc = 0x48a447; continue;
 }
 0x48a447 => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48a4a9; continue; }
  pc = 0x48a451; continue;
 }
 0x48a451 => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48a4c7; continue; }
  pc = 0x48a45b; continue;
 }
 0x48a45b => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x48a483; continue; }
  pc = 0x48a465; continue;
 }
 0x48a465 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x48a4ed; continue;
 }
 0x48a483 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x48a4ed; continue;
 }
 0x48a4a9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x48a4ed; continue;
 }
 0x48a4c7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x48a4ed; continue;
 }
 0x48a4ed => {
  cpu.push(0x48a4f2); f_00458a0c(cpu); if cpu.returned_to != 0x48a4f2 { return; }
  if cpu.cf { pc = 0x48a7b0; continue; }
  pc = 0x48a4f8; continue;
 }
 0x48a4f8 => {
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a505); f_0048990c(cpu); if cpu.returned_to != 0x48a505 { return; }
  if cpu.cf { pc = 0x48a7b0; continue; }
  pc = 0x48a50b; continue;
 }
 0x48a50b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48a519; continue; }
  pc = 0x48a514; continue;
 }
 0x48a514 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48a51e; continue; }
  pc = 0x48a519; continue;
 }
 0x48a519 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x48a51e; continue;
 }
 0x48a51e => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a7d0; continue;
 }
 0x48a533 => {
  { let v: u16 = cpu.r16(0x39u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x39u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x48a55e; continue; }
  pc = 0x48a53d; continue;
 }
 0x48a53d => {
  { let v: u8 = cpu.r8(0x84u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x48a556); f_0048a7d1(cpu); if cpu.returned_to != 0x48a556 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48a55d); f_0048a7d1(cpu); if cpu.returned_to != 0x48a55d { return; }
  { cpu.popad(); }
  pc = 0x48a55e; continue;
 }
 0x48a55e => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x39u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x48a7d0; continue; }
  pc = 0x48a56c; continue;
 }
 0x48a56c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4b8b48u32); cpu.eax = v; }
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
  cpu.push(0x48a5a7); f_00458a0c(cpu); if cpu.returned_to != 0x48a5a7 { return; }
  if cpu.cf { pc = 0x48a7b0; continue; }
  pc = 0x48a5ad; continue;
 }
 0x48a5ad => {
  cpu.push(0x48a5b2); f_0048990c(cpu); if cpu.returned_to != 0x48a5b2 { return; }
  if cpu.cf { pc = 0x48a7b0; continue; }
  pc = 0x48a5b8; continue;
 }
 0x48a5b8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48a5c6; continue; }
  pc = 0x48a5c1; continue;
 }
 0x48a5c1 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48a5cb; continue; }
  pc = 0x48a5c6; continue;
 }
 0x48a5c6 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x48a5cb; continue;
 }
 0x48a5cb => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a7d0; continue;
 }
 0x48a5e0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x92u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x48a643; continue; }
  pc = 0x48a5fe; continue;
 }
 0x48a5fe => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48a604); f_0048aadc(cpu); if cpu.returned_to != 0x48a604 { return; }
  if cpu.cf { pc = 0x48a7b0; continue; }
  pc = 0x48a60a; continue;
 }
 0x48a60a => {
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
  { let v: u32 = cpu.r32(0x5d6d40u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x48a7b0; continue;
 }
 0x48a643 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x48a700; continue; }
  pc = 0x48a650; continue;
 }
 0x48a650 => {
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
  cpu.push(0x48a675); f_0046e4b3(cpu); if cpu.returned_to != 0x48a675 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a681); f_0041c743(cpu); if cpu.returned_to != 0x48a681 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48a688; continue;
 }
 0x48a688 => {
  cpu.push(0x48a68d); f_0048aadc(cpu); if cpu.returned_to != 0x48a68d { return; }
  if cpu.cf { pc = 0x48a7ac; continue; }
  pc = 0x48a693; continue;
 }
 0x48a693 => {
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
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d6d40u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d48u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48a688; continue; }
  pc = 0x48a6fb; continue;
 }
 0x48a6fb => {
  pc = 0x48a7ac; continue;
 }
 0x48a700 => {
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
  cpu.push(0x48a72a); f_0046e4b3(cpu); if cpu.returned_to != 0x48a72a { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a736); f_0041c743(cpu); if cpu.returned_to != 0x48a736 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48a73d; continue;
 }
 0x48a73d => {
  cpu.push(0x48a742); f_0048aadc(cpu); if cpu.returned_to != 0x48a742 { return; }
  if cpu.cf { pc = 0x48a7ac; continue; }
  pc = 0x48a744; continue;
 }
 0x48a744 => {
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
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d6d40u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d50u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48a73d; continue; }
  pc = 0x48a7ac; continue;
 }
 0x48a7ac => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x48a7b0; continue;
 }
 0x48a7b0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x48a7d0); f_0041b794(cpu); if cpu.returned_to != 0x48a7d0 { return; }
  pc = 0x48a7d0; continue;
 }
 0x48a7d0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a7d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a7d1); }
 let mut pc: u32 = 0x48a7d1;
 loop { match pc {
 0x48a7d1 => {
  { cpu.pushad(); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x48a7d7; continue;
 }
 0x48a7d7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x48a80d; continue; }
  pc = 0x48a7de; continue;
 }
 0x48a7de => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); }
  if !cpu.zf { pc = 0x48a805; continue; }
  pc = 0x48a7e5; continue;
 }
 0x48a7e5 => {
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x48a805; continue; }
  pc = 0x48a7eb; continue;
 }
 0x48a7eb => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfffeu16); }
  if !cpu.zf { pc = 0x48a805; continue; }
  pc = 0x48a7f5; continue;
 }
 0x48a7f5 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x48a7fe; continue; }
  pc = 0x48a7fb; continue;
 }
 0x48a7fb => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48a7fe; continue;
 }
 0x48a7fe => {
  { let v: u8 = cpu.or8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x48a80d; continue;
 }
 0x48a805 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48a7d7; continue; }
  pc = 0x48a80d; continue;
 }
 0x48a80d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a80f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a80f); }
 let mut pc: u32 = 0x48a80f;
 loop { match pc {
 0x48a80f => {
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d6d44u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d6d45u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = 0x5d6d78u32; cpu.esi = v; }
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  if !cpu.cf { pc = 0x48a863; continue; }
  pc = 0x48a854; continue;
 }
 0x48a854 => {
  { let v: u32 = cpu.r8(0xabu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d6d94u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x48a863; continue;
 }
 0x48a863 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x48a958; continue; }
  pc = 0x48a86e; continue;
 }
 0x48a86e => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1775u16); }
  if !cpu.zf { pc = 0x48a8a1; continue; }
  pc = 0x48a893; continue;
 }
 0x48a893 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5d6d45u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48a8b4; continue;
 }
 0x48a8a1 => {
  { cpu.sub16((cpu.ecx as u16), 0x1777u16); }
  if !cpu.zf { pc = 0x48a8b4; continue; }
  pc = 0x48a8a8; continue;
 }
 0x48a8a8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5d6d44u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48a8b4; continue;
 }
 0x48a8b4 => {
  { cpu.and8(cpu.r8(0x5d6d46u32), 0x40u8); }
  if cpu.zf { pc = 0x48a8d5; continue; }
  pc = 0x48a8bd; continue;
 }
 0x48a8bd => {
  { cpu.sub16((cpu.ecx as u16), 0x177du16); }
  if !cpu.zf { pc = 0x48a8ca; continue; }
  pc = 0x48a8c4; continue;
 }
 0x48a8c4 => {
  { let v: u16 = 0x177fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x48a8d5; continue;
 }
 0x48a8ca => {
  { cpu.sub16((cpu.ecx as u16), 0x177fu16); }
  if !cpu.zf { pc = 0x48a8d5; continue; }
  pc = 0x48a8d1; continue;
 }
 0x48a8d1 => {
  { let v: u16 = 0x177du16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x48a8d5; continue;
 }
 0x48a8d5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x48a94f; continue; }
  pc = 0x48a8f7; continue;
 }
 0x48a8f7 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x48a94f; continue; }
  pc = 0x48a8fd; continue;
 }
 0x48a8fd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x48a94f; continue; }
  pc = 0x48a906; continue;
 }
 0x48a906 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x48a94f; continue; }
  pc = 0x48a90f; continue;
 }
 0x48a90f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x62u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.edx as u16), 0x3u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x48a920); f_00486542(cpu); if cpu.returned_to != 0x48a920 { return; }
  { cpu.popad(); }
  cpu.push(0x48a926); f_004679cb(cpu); if cpu.returned_to != 0x48a926 { return; }
  if cpu.cf { pc = 0x48a94f; continue; }
  pc = 0x48a928; continue;
 }
 0x48a928 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xcu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48a94e; continue; }
  pc = 0x48a93e; continue;
 }
 0x48a93e => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a945); f_00401130(cpu); if cpu.returned_to != 0x48a945 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48a94e; continue;
 }
 0x48a94e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48a94f; continue;
 }
 0x48a94f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x48a863; continue;
 }
 0x48a958 => {
  { cpu.sub16(cpu.r16(0x4d693au32), 0xffffu16); }
  if !cpu.zf { pc = 0x48a98e; continue; }
  pc = 0x48a962; continue;
 }
 0x48a962 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48a973); f_004679cb(cpu); if cpu.returned_to != 0x48a973 { return; }
  if cpu.cf { pc = 0x48a98a; continue; }
  pc = 0x48a975; continue;
 }
 0x48a975 => {
  { let v: u8 = 0x6u8; cpu.w8(0x4e1facu32, v); }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x59dad6u32, v); }
  pc = 0x48a98a; continue;
 }
 0x48a98a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48a98e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a990); }
 let mut pc: u32 = 0x48a990;
 loop { match pc {
 0x48a990 => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
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
  cpu.push(0x48aa07); f_004674e8(cpu); if cpu.returned_to != 0x48aa07 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  let t = cpu.r32(0x36u32.wrapping_add(cpu.esi)); cpu.push(0x48aa0b); cpu.call(t); if cpu.returned_to != 0x48aa0b { return; }
  { cpu.popad(); }
  cpu.push(0x48aa11); f_004435aa(cpu); if cpu.returned_to != 0x48aa11 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048aa13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aa13); }
 let mut pc: u32 = 0x48aa13;
 loop { match pc {
 0x48aa13 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48aa1d); f_0046ac64(cpu); if cpu.returned_to != 0x48aa1d { return; }
  { let v: u8 = 0x3cu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48aab4; continue; }
  pc = 0x48aa30; continue;
 }
 0x48aa30 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48aa45; continue; }
  pc = 0x48aa40; continue;
 }
 0x48aa40 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48aa4a; continue; }
  pc = 0x48aa45; continue;
 }
 0x48aa45 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x48aa4a; continue;
 }
 0x48aa4a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)) & 0x7u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48aa5c); f_0046c297(cpu); if cpu.returned_to != 0x48aa5c { return; }
  cpu.push(0x48aa61); f_0046c365(cpu); if cpu.returned_to != 0x48aa61 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5d6d5au32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d58u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  cpu.push(0x48aa8a); f_00481013(cpu); if cpu.returned_to != 0x48aa8a { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x48aa91); f_00481013(cpu); if cpu.returned_to != 0x48aa91 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x48aaad; continue; }
  pc = 0x48aaa3; continue;
 }
 0x48aaa3 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xfdu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48aab4; continue;
 }
 0x48aaad => {
  { let v: u8 = cpu.dec8(cpu.r8(0x8eu32.wrapping_add(cpu.ebp))); cpu.w8(0x8eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48aab4; continue;
 }
 0x48aab4 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x2084u32); }
  if !cpu.zf { pc = 0x48aac2; continue; }
  pc = 0x48aac0; continue;
 }
 0x48aac0 => {
  pc = 0x48aadb; continue;
 }
 0x48aac2 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x48aadb; continue; }
  pc = 0x48aacc; continue;
 }
 0x48aacc => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  cpu.push(0x48aad6); f_0046ab56(cpu); if cpu.returned_to != 0x48aad6 { return; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48aadb; continue;
 }
 0x48aadb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048aadc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aadc); }
 let mut pc: u32 = 0x48aadc;
 loop { match pc {
 0x48aadc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x56bfccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x48aae7; continue;
 }
 0x48aae7 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x48aaf3; continue; }
  pc = 0x48aaec; continue;
 }
 0x48aaec => {
  { let v: u32 = cpu.add32(cpu.esi, 0x18u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48aae7; continue; }
  pc = 0x48aaf1; continue;
 }
 0x48aaf1 => {
  pc = 0x48aaf6; continue;
 }
 0x48aaf3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48aaf6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048aafc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aafc); }
 let mut pc: u32 = 0x48aafc;
 loop { match pc {
 0x48aafc => {
  { cpu.pushad(); }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  { let v: u32 = 0x5d8968u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ab12); f_00421214(cpu); if cpu.returned_to != 0x48ab12 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ab30; continue; }
  pc = 0x48ab17; continue;
 }
 0x48ab17 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x48ab30; continue; }
  pc = 0x48ab1b; continue;
 }
 0x48ab1b => {
  { let v: u32 = 0x5d8968u32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d7954u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5d7958u32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48ab30 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab33); }
 let mut pc: u32 = 0x48ab33;
 loop { match pc {
 0x48ab33 => {
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x48ab3d; continue; }
  pc = 0x48ab37; continue;
 }
 0x48ab37 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48ab3d; continue; }
  pc = 0x48ab3b; continue;
 }
 0x48ab3b => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48ab3d; continue;
 }
 0x48ab3d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab3e); }
 let mut pc: u32 = 0x48ab3e;
 loop { match pc {
 0x48ab3e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffccu32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.w32(0x5d7964u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d795cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d7960u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48ab79); f_0040d370(cpu); if cpu.returned_to != 0x48ab79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d795cu32, v); }
  { cpu.popad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d7960u32, v); }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x48ae65; continue; }
  pc = 0x48ab99; continue;
 }
 0x48ab99 => {
  { let v: u32 = 0xfffffffeu32; cpu.w32(0x5d7960u32, v); }
  { let v: u32 = 0xfffffffcu32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48abba); f_00421241(cpu); if cpu.returned_to != 0x48abba { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48abc3; continue;
 }
 0x48abc3 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48abd6); f_00421214(cpu); if cpu.returned_to != 0x48abd6 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48abdf; continue;
 }
 0x48abdf => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x32504d4au32); }
  if !cpu.zf { pc = 0x48ae65; continue; }
  pc = 0x48abec; continue;
 }
 0x48abec => {
  { let v: u32 = 0xfffffff8u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48ac03); f_00421241(cpu); if cpu.returned_to != 0x48ac03 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ac0c; continue;
 }
 0x48ac0c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ac1f); f_00421214(cpu); if cpu.returned_to != 0x48ac1f { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ac28; continue;
 }
 0x48ac28 => {
  { let v: u32 = 0xfffffffdu32; cpu.w32(0x5d7960u32, v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xffffffe8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x8u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48ac4d); f_00421241(cpu); if cpu.returned_to != 0x48ac4d { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ac56; continue;
 }
 0x48ac56 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x48ac95; continue; }
  pc = 0x48ac5e; continue;
 }
 0x48ac5e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ac72); f_00421214(cpu); if cpu.returned_to != 0x48ac72 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48ac76; continue;
 }
 0x48ac76 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x48acf2; continue; }
  pc = 0x48ac7a; continue;
 }
 0x48ac7a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ac8e); f_00421214(cpu); if cpu.returned_to != 0x48ac8e { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x48ac76; continue;
 }
 0x48ac95 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48aca8); f_00421214(cpu); if cpu.returned_to != 0x48aca8 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x48acb6; continue;
 }
 0x48acb6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x48acbe); f_0048ab33(cpu); if cpu.returned_to != 0x48acbe { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x48acc8); f_0048ab33(cpu); if cpu.returned_to != 0x48acc8 { return; }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x48acd2; continue; }
  pc = 0x48accc; continue;
 }
 0x48accc => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x48acf2; continue; }
  pc = 0x48acd0; continue;
 }
 0x48acd0 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48acb6; continue; }
  pc = 0x48acd2; continue;
 }
 0x48acd2 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ace5); f_00421214(cpu); if cpu.returned_to != 0x48ace5 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x48ac95; continue; }
  pc = 0x48aced; continue;
 }
 0x48aced => {
  pc = 0x48ae65; continue;
 }
 0x48acf2 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ad05); f_00421214(cpu); if cpu.returned_to != 0x48ad05 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ad0e; continue;
 }
 0x48ad0e => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ad21); f_00421214(cpu); if cpu.returned_to != 0x48ad21 { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ad2a; continue;
 }
 0x48ad2a => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48ad39; continue; }
  pc = 0x48ad30; continue;
 }
 0x48ad30 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x48ae39; continue;
 }
 0x48ad39 => {
  { let v: u32 = cpu.r16(0xffffffd4u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffd6u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48ad4e); f_00421241(cpu); if cpu.returned_to != 0x48ad4e { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  cpu.push(0x48ad56); f_0048aafc(cpu); if cpu.returned_to != 0x48ad56 { return; }
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5d7958u32)); }
  if cpu.cf { pc = 0x48ad7d; continue; }
  pc = 0x48ad6c; continue;
 }
 0x48ad6c => {
  cpu.push(0x48ad71); f_0048aafc(cpu); if cpu.returned_to != 0x48ad71 { return; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ad77; continue;
 }
 0x48ad77 => {
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  pc = 0x48ad7d; continue;
 }
 0x48ad7d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x5d7968u32; cpu.ebp = v; }
  pc = 0x48adae; continue;
 }
 0x48ad8b => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x48adae; continue; }
  pc = 0x48ad8f; continue;
 }
 0x48ad8f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x5d7958u32)); }
  if cpu.cf { pc = 0x48ada8; continue; }
  pc = 0x48ad97; continue;
 }
 0x48ad97 => {
  cpu.push(0x48ad9c); f_0048aafc(cpu); if cpu.returned_to != 0x48ad9c { return; }
  if cpu.cf { pc = 0x48ae5b; continue; }
  pc = 0x48ada2; continue;
 }
 0x48ada2 => {
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  pc = 0x48ada8; continue;
 }
 0x48ada8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x48adae; continue;
 }
 0x48adae => {
  if !cpu.cf { pc = 0x48addb; continue; }
  pc = 0x48adb0; continue;
 }
 0x48adb0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x5d7958u32)); }
  if cpu.cf { pc = 0x48adc9; continue; }
  pc = 0x48adb8; continue;
 }
 0x48adb8 => {
  cpu.push(0x48adbd); f_0048aafc(cpu); if cpu.returned_to != 0x48adbd { return; }
  if cpu.cf { pc = 0x48ae5a; continue; }
  pc = 0x48adc3; continue;
 }
 0x48adc3 => {
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  pc = 0x48adc9; continue;
 }
 0x48adc9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x48ad8b; continue;
 }
 0x48addb => {
  { cpu.sub32(cpu.esi, cpu.r32(0x5d7958u32)); }
  if cpu.cf { pc = 0x48adf0; continue; }
  pc = 0x48ade3; continue;
 }
 0x48ade3 => {
  cpu.push(0x48ade8); f_0048aafc(cpu); if cpu.returned_to != 0x48ade8 { return; }
  if cpu.cf { pc = 0x48ae5a; continue; }
  pc = 0x48adea; continue;
 }
 0x48adea => {
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  pc = 0x48adf0; continue;
 }
 0x48adf0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5d7958u32)); }
  if cpu.cf { pc = 0x48ae08; continue; }
  pc = 0x48adfb; continue;
 }
 0x48adfb => {
  cpu.push(0x48ae00); f_0048aafc(cpu); if cpu.returned_to != 0x48ae00 { return; }
  if cpu.cf { pc = 0x48ae5a; continue; }
  pc = 0x48ae02; continue;
 }
 0x48ae02 => {
  { let v: u32 = cpu.r32(0x5d7954u32); cpu.esi = v; }
  pc = 0x48ae08; continue;
 }
 0x48ae08 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48ae39; continue; }
  pc = 0x48ae15; continue;
 }
 0x48ae15 => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x48ae1b; continue;
 }
 0x48ae1b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48ae1b; continue; }
  pc = 0x48ae34; continue;
 }
 0x48ae34 => {
  cpu.poll(); pc = 0x48ad8b; continue;
 }
 0x48ae39 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ae4b); f_004211ec(cpu); if cpu.returned_to != 0x48ae4b { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d7964u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48ae5a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x48ae5b; continue;
 }
 0x48ae5b => {
  { let v: u32 = 0xfffffffcu32; cpu.w32(0x5d7960u32, v); }
  pc = 0x48ae65; continue;
 }
 0x48ae65 => {
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x48ae7e; continue; }
  pc = 0x48ae70; continue;
 }
 0x48ae70 => {
  { let v: u32 = cpu.r32(0x5d795cu32); cpu.ebx = v; }
  cpu.push(0x48ae7b); f_004211ec(cpu); if cpu.returned_to != 0x48ae7b { return; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  pc = 0x48ae7e; continue;
 }
 0x48ae7e => {
  { let v: u32 = cpu.r32(0x5d7964u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d7960u32); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048afc8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48afc8); } r_0048afc8(cpu, 0x48afc8); }
pub fn f_0048afe2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48afe2); } r_0048afc8(cpu, 0x48afe2); }
pub fn f_0048b1e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b1e0); } r_0048afc8(cpu, 0x48b1e0); }
pub fn f_0048b297(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b297); } r_0048afc8(cpu, 0x48b297); }
pub fn f_0048b2cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b2cb); } r_0048afc8(cpu, 0x48b2cb); }
pub fn f_0048b31d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b31d); } r_0048afc8(cpu, 0x48b31d); }
fn r_0048afc8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48afb0 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48afc8 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x2710u16); }
  if !cpu.zf { pc = 0x48afda; continue; }
  pc = 0x48afd1; continue;
 }
 0x48afd1 => {
  { let v: u32 = 0x5b3858u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48afda => {
  { cpu.pushad(); }
  { let v: u32 = 0x48afe2u32; cpu.edx = v; }
  pc = 0x48aff9; continue;
 }
 0x48afe2 => {
  cpu.unimplemented(0x48afe2, "Arpl");
  cpu.unimplemented(0x48afe5, "Arpl");
  { cpu.popad(); }
  if cpu.of { pc = 0x48b04a; continue; }
  pc = 0x48afeb; continue;
 }
 0x48afeb => {
  if cpu.of { pc = 0x48b05c; continue; }
  pc = 0x48afed; continue;
 }
 0x48afed => {
  if !cpu.cf { pc = 0x48b04e; continue; }
  pc = 0x48afef; continue;
 }
 0x48afef => {
  { cpu.popad(); }
  cpu.unimplemented(0x48aff0, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x48b065; continue; }
  pc = 0x48aff5; continue;
 }
 0x48aff5 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48afb0; continue; }
  pc = 0x48b001; continue;
 }
 0x48aff9 => {
  { let v: u32 = 0x48afe2u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x48afd1; continue; }
  pc = 0x48b034; continue;
 }
 0x48b001 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3eu32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x3eu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4dd54cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x48afd1; continue; }
  pc = 0x48b034; continue;
 }
 0x48b034 => {
  { cpu.sub32(cpu.ebx, 0x4e154cu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x48afd1; continue; }
  pc = 0x48b03c; continue;
 }
 0x48b03c => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x48b05a; continue; }
  pc = 0x48b049; continue;
 }
 0x48b049 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  pc = 0x48b04e; continue;
 }
 0x48b04a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  pc = 0x48b04e; continue;
 }
 0x48b04e => {
  if cpu.zf { pc = 0x48b05a; continue; }
  pc = 0x48b050; continue;
 }
 0x48b050 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x48b05a; continue;
 }
 0x48b05a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b065; continue;
 }
 0x48b05c => {
  if (cpu.sf == cpu.of) { pc = 0x48b08e; continue; }
  pc = 0x48b05e; continue;
 }
 0x48b05e => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.edi); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48b092; continue; }
  pc = 0x48b06f; continue;
 }
 0x48b065 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48b092; continue; }
  pc = 0x48b06f; continue;
 }
 0x48b06f => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b07f); f_0046e362(cpu); if cpu.returned_to != 0x48b07f { return; }
  if cpu.cf { pc = 0x48b087; continue; }
  pc = 0x48b081; continue;
 }
 0x48b081 => {
  { cpu.popad(); }
  pc = 0x48b1a4; continue;
 }
 0x48b087 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b092; continue;
 }
 0x48b08e => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b092; continue;
 }
 0x48b092 => {
  { let v: u8 = 0x2fu8; cpu.w8(0x5da969u32, v); }
  { let v: u8 = 0x32u8; cpu.w8(0x5da968u32, v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x48b0b8; continue; }
  pc = 0x48b0aa; continue;
 }
 0x48b0aa => {
  { let v: u8 = 0x38u8; cpu.w8(0x5da969u32, v); }
  { let v: u8 = 0x3au8; cpu.w8(0x5da968u32, v); }
  pc = 0x48b0b8; continue;
 }
 0x48b0b8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5da969u32)); }
  if !cpu.zf { pc = 0x48b0d7; continue; }
  pc = 0x48b0c0; continue;
 }
 0x48b0c0 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x48b0d7; continue; }
  pc = 0x48b0ca; continue;
 }
 0x48b0ca => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b1a4; continue;
 }
 0x48b0d7 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x48b180; continue; }
  pc = 0x48b0e5; continue;
 }
 0x48b0e5 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b0f3; continue;
 }
 0x48b0f3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5da968u32)); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b0ff; continue;
 }
 0x48b0ff => {
  cpu.push(0x48b104); f_0048b354(cpu); if cpu.returned_to != 0x48b104 { return; }
  if !cpu.cf { pc = 0x48b11b; continue; }
  pc = 0x48b106; continue;
 }
 0x48b106 => {
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b1a4; continue;
 }
 0x48b11b => {
  cpu.push(0x48b120); f_0048b31d(cpu); if cpu.returned_to != 0x48b120 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xabu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x48b162; continue; }
  pc = 0x48b140; continue;
 }
 0x48b140 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48b157; continue; }
  pc = 0x48b14a; continue;
 }
 0x48b14a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x48b162; continue;
 }
 0x48b157 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x48b162; continue;
 }
 0x48b162 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48b1a4; continue;
 }
 0x48b180 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5da968u32)); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b188; continue;
 }
 0x48b188 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b192; continue;
 }
 0x48b192 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if !cpu.zf { pc = 0x48b1a4; continue; }
  pc = 0x48b19c; continue;
 }
 0x48b19c => {
  { let v: u8 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b1a4; continue;
 }
 0x48b1a4 => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b28e; continue; }
  pc = 0x48b1b0; continue;
 }
 0x48b1b0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48b1bc; continue; }
  pc = 0x48b1b7; continue;
 }
 0x48b1b7 => {
  cpu.push(0x48b1bc); f_0048b2cb(cpu); if cpu.returned_to != 0x48b1bc { return; }
  pc = 0x48b1bc; continue;
 }
 0x48b1bc => {
  { let v: u8 = cpu.r8(0x4e84eau32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x6du8); }
  if cpu.zf { pc = 0x48b1d0; continue; }
  pc = 0x48b1c6; continue;
 }
 0x48b1c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b1d0; continue;
 }
 0x48b1d0 => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if (cpu.sf == cpu.of) { pc = 0x48b28f; continue; }
  pc = 0x48b1d8; continue;
 }
 0x48b1d8 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48b1e0u32; cpu.edx = v; }
  pc = 0x48b1ea; continue;
 }
 0x48b1e0 => {
  cpu.unimplemented(0x48b1e0, "Arpl");
  cpu.unimplemented(0x48b1e3, "Arpl");
  if cpu.cf { pc = 0x48b24d; continue; }
  pc = 0x48b1e8; continue;
 }
 0x48b1e8 => {
  if cpu.zf { pc = 0x48b1ea; continue; }
  pc = 0x48b1ea; continue;
 }
 0x48b1ea => {
  { let v: u32 = 0x48b1e0u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48b224; continue; }
  pc = 0x48b1fc; continue;
 }
 0x48b1fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48b20a; continue; }
  pc = 0x48b205; continue;
 }
 0x48b205 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48b20f; continue; }
  pc = 0x48b20a; continue;
 }
 0x48b20a => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48b20f; continue;
 }
 0x48b20f => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x48b220; continue; }
  pc = 0x48b219; continue;
 }
 0x48b219 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48b24c; continue; }
  pc = 0x48b220; continue;
 }
 0x48b220 => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b24c; continue;
 }
 0x48b224 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48b23e; continue; }
  pc = 0x48b231; continue;
 }
 0x48b231 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x8u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x48b23e; continue;
 }
 0x48b23e => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x20u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48b24c; continue; }
  pc = 0x48b24a; continue;
 }
 0x48b24a => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b24c; continue;
 }
 0x48b24c => {
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let a = (cpu.ebx as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48b26f; continue; }
  pc = 0x48b261; continue;
 }
 0x48b24d => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let a = (cpu.ebx as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48b26f; continue; }
  pc = 0x48b261; continue;
 }
 0x48b261 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x48b26f; continue; }
  pc = 0x48b26d; continue;
 }
 0x48b26d => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b26f; continue;
 }
 0x48b26f => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x89u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x48b27a; continue; }
  pc = 0x48b278; continue;
 }
 0x48b278 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b27a; continue;
 }
 0x48b27a => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.cf { pc = 0x48b280; continue; }
  pc = 0x48b27e; continue;
 }
 0x48b27e => {
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b280; continue;
 }
 0x48b280 => {
  cpu.push(0x48b285); f_00489b34(cpu); if cpu.returned_to != 0x48b285 { return; }
  cpu.push(0x48b28a); f_00489bb9(cpu); if cpu.returned_to != 0x48b28a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b28e; continue;
 }
 0x48b28e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b28f => {
  { cpu.pushad(); }
  { let v: u32 = 0x48b297u32; cpu.edx = v; }
  pc = 0x48b2a1; continue;
 }
 0x48b297 => {
  cpu.unimplemented(0x48b297, "Arpl");
  cpu.unimplemented(0x48b29a, "Arpl");
  if (!cpu.cf && !cpu.zf) { pc = 0x48b308; continue; }
  pc = 0x48b29f; continue;
 }
 0x48b29f => {
  cpu.unimplemented(0x48b29f, "Outsb");
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x48u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x3cu32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x3cu32.wrapping_add(cpu.ecx), v); }
  if cpu.cf { pc = 0x48b323; continue; }
  pc = 0x48b2af; continue;
 }
 0x48b2a1 => {
  { let v: u32 = 0x48b297u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { cpu.sub8((cpu.eax as u8), 0x72u8); }
  if cpu.zf { pc = 0x48b2b9; continue; }
  pc = 0x48b2b0; continue;
 }
 0x48b2af => {
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2))), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  if cpu.zf { pc = 0x48b2b9; continue; }
  pc = 0x48b2b4; continue;
 }
 0x48b2b0 => {
  { cpu.sub8((cpu.eax as u8), 0x74u8); }
  if cpu.zf { pc = 0x48b2b9; continue; }
  pc = 0x48b2b4; continue;
 }
 0x48b2b4 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b2b9; continue;
 }
 0x48b2b9 => {
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x64u32); cpu.esi = v; }
  let t = cpu.r32(0x5da96cu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.push(0x48b2c6); cpu.call(t); if cpu.returned_to != 0x48b2c6 { return; }
  cpu.poll(); pc = 0x48b1d8; continue;
 }
 0x48b2cb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x48b315; continue; }
  pc = 0x48b2f1; continue;
 }
 0x48b2f1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x48b315; continue; }
  pc = 0x48b2fa; continue;
 }
 0x48b2fa => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x48b315; continue; }
  pc = 0x48b306; continue;
 }
 0x48b306 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x48b315; continue; }
  pc = 0x48b312; continue;
 }
 0x48b308 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(0x7f240000u32); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x48b315; continue; }
  pc = 0x48b312; continue;
 }
 0x48b312 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b315 => {
  cpu.push(0x48b31a); f_0048b984(cpu); if cpu.returned_to != 0x48b31a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b31d => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if !cpu.zf { pc = 0x48b328; continue; }
  pc = 0x48b327; continue;
 }
 0x48b323 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x48b328; continue; }
  pc = 0x48b327; continue;
 }
 0x48b327 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b328 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.zf { pc = 0x48b352; continue; }
  pc = 0x48b344; continue;
 }
 0x48b344 => {
  cpu.push(0x48b349); f_0041879e(cpu); if cpu.returned_to != 0x48b349 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d688au32, v); }
  pc = 0x48b352; continue;
 }
 0x48b352 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b354); }
 let mut pc: u32 = 0x48b354;
 loop { match pc {
 0x48b354 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48b374; continue; }
  pc = 0x48b363; continue;
 }
 0x48b363 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x48b384; continue; }
  pc = 0x48b369; continue;
 }
 0x48b369 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.edi)), 0x8u8); }
  if !cpu.zf { pc = 0x48b384; continue; }
  pc = 0x48b36f; continue;
 }
 0x48b36f => {
  pc = 0x48b3fd; continue;
 }
 0x48b374 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x48b384; continue; }
  pc = 0x48b37c; continue;
 }
 0x48b37c => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x48b3fd; continue; }
  pc = 0x48b382; continue;
 }
 0x48b382 => {
  pc = 0x48b400; continue;
 }
 0x48b384 => {
  { cpu.pushad(); }
  cpu.push(0x48b38a); f_00489968(cpu); if cpu.returned_to != 0x48b38a { return; }
  cpu.push(0x48b38f); f_0043ffdb(cpu); if cpu.returned_to != 0x48b38f { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x48b3a2); f_0046ab56(cpu); if cpu.returned_to != 0x48b3a2 { return; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48b3b2; continue; }
  pc = 0x48b3a9; continue;
 }
 0x48b3a9 => {
  cpu.push(0x48b3ae); f_00478632(cpu); if cpu.returned_to != 0x48b3ae { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b3b2; continue;
 }
 0x48b3b2 => {
  { cpu.popad(); }
  cpu.push(0x48b3b8); f_0048b46b(cpu); if cpu.returned_to != 0x48b3b8 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  cpu.push(0x48b3e0); f_00413337(cpu); if cpu.returned_to != 0x48b3e0 { return; }
  { cpu.pushad(); }
  pc = 0x48b3e1; continue;
 }
 0x48b3e1 => {
  { cpu.popad(); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b3fd => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b400 => {
  { cpu.pushad(); }
  cpu.push(0x48b406); f_00489968(cpu); if cpu.returned_to != 0x48b406 { return; }
  cpu.push(0x48b40b); f_0043ffdb(cpu); if cpu.returned_to != 0x48b40b { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x48b41e); f_0046ab56(cpu); if cpu.returned_to != 0x48b41e { return; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48b42e; continue; }
  pc = 0x48b425; continue;
 }
 0x48b425 => {
  cpu.push(0x48b42a); f_00478632(cpu); if cpu.returned_to != 0x48b42a { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b42e; continue;
 }
 0x48b42e => {
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  cpu.push(0x48b465); f_00413337(cpu); if cpu.returned_to != 0x48b465 { return; }
  { cpu.pushad(); }
  cpu.poll(); pc = 0x48b3e1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b46b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b46b); }
 let mut pc: u32 = 0x48b46b;
 loop { match pc {
 0x48b46b => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x48b4b3; continue; }
  pc = 0x48b477; continue;
 }
 0x48b477 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x48b4c9; continue; }
  pc = 0x48b47b; continue;
 }
 0x48b47b => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x48b495; continue; }
  pc = 0x48b47f; continue;
 }
 0x48b47f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48b4e5; continue;
 }
 0x48b495 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48b4e5; continue;
 }
 0x48b4b3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48b4e5; continue;
 }
 0x48b4c9 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48b4e5; continue;
 }
 0x48b4e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b4e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b4e6); }
 let mut pc: u32 = 0x48b4e6;
 loop { match pc {
 0x48b4e6 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b5af; continue; }
  pc = 0x48b4f7; continue;
 }
 0x48b4f7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48b518; continue; }
  pc = 0x48b50d; continue;
 }
 0x48b50d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x48b518; continue;
 }
 0x48b518 => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x48b522; continue;
 }
 0x48b522 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x48b5a5; continue; }
  pc = 0x48b529; continue;
 }
 0x48b529 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x48b5a5; continue; }
  pc = 0x48b52f; continue;
 }
 0x48b52f => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48b53a; continue; }
  pc = 0x48b537; continue;
 }
 0x48b537 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48b53a; continue;
 }
 0x48b53a => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x48b5a5; continue; }
  pc = 0x48b53f; continue;
 }
 0x48b53f => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48b54b; continue; }
  pc = 0x48b548; continue;
 }
 0x48b548 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48b54b; continue;
 }
 0x48b54b => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x48b5a5; continue; }
  pc = 0x48b550; continue;
 }
 0x48b550 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4b8496u32); }
  if cpu.zf { pc = 0x48b563; continue; }
  pc = 0x48b55b; continue;
 }
 0x48b55b => {
  { cpu.sub32(cpu.edi, 0x4b847du32); }
  if !cpu.zf { pc = 0x48b5a5; continue; }
  pc = 0x48b563; continue;
 }
 0x48b563 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48b578; continue; }
  pc = 0x48b56a; continue;
 }
 0x48b56a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x48b578; continue; }
  pc = 0x48b571; continue;
 }
 0x48b571 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x48b5af; continue; }
  pc = 0x48b578; continue;
 }
 0x48b578 => {
  cpu.push(0x48b57d); f_0046d525(cpu); if cpu.returned_to != 0x48b57d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48b5af; continue; }
  pc = 0x48b581; continue;
 }
 0x48b581 => {
  cpu.push(0x48b586); f_0048b5b2(cpu); if cpu.returned_to != 0x48b586 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b594); f_00420989(cpu); if cpu.returned_to != 0x48b594 { return; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b5a5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48b522; continue; }
  pc = 0x48b5af; continue;
 }
 0x48b5af => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b5b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b5b2); }
 let mut pc: u32 = 0x48b5b2;
 loop { match pc {
 0x48b5b2 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x4b8496u32); }
  if !cpu.zf { pc = 0x48b5d1; continue; }
  pc = 0x48b5cf; continue;
 }
 0x48b5cf => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48b5d1; continue;
 }
 0x48b5d1 => {
  { let v = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x48b5db; continue;
 }
 0x48b5db => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x48b5fc; continue; }
  pc = 0x48b5e0; continue;
 }
 0x48b5e0 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x509325u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x509325u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x509325u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0x509325u32.wrapping_add(cpu.eax), v); }
  if !cpu.sf { pc = 0x48b5fc; continue; }
  pc = 0x48b5f5; continue;
 }
 0x48b5f5 => {
  { let v: u8 = 0x7fu8; cpu.w8(0x509325u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b5fc; continue;
 }
 0x48b5fc => {
  { let v: u8 = cpu.xor8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48b5db; continue; }
  pc = 0x48b603; continue;
 }
 0x48b603 => {
  cpu.push(0x48b608); f_00416b62(cpu); if cpu.returned_to != 0x48b608 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b60a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b60a); }
 let mut pc: u32 = 0x48b60a;
 loop { match pc {
 0x48b60a => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b671; continue; }
  pc = 0x48b612; continue;
 }
 0x48b612 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x48b674; continue; }
  pc = 0x48b61d; continue;
 }
 0x48b61d => {
  { cpu.sub8((cpu.eax as u8), 0x6fu8); }
  if cpu.zf { pc = 0x48b6b8; continue; }
  pc = 0x48b625; continue;
 }
 0x48b625 => {
  { cpu.sub8((cpu.eax as u8), 0x70u8); }
  if cpu.zf { pc = 0x48b6f9; continue; }
  pc = 0x48b62d; continue;
 }
 0x48b62d => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0x0u8); }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b63a; continue;
 }
 0x48b63a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b643; continue;
 }
 0x48b643 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48b66c; continue; }
  pc = 0x48b64c; continue;
 }
 0x48b64c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b66c; continue;
 }
 0x48b66c => {
  pc = 0x48b72e; continue;
 }
 0x48b671 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b674 => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b681; continue;
 }
 0x48b681 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x0u8); }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b68a; continue;
 }
 0x48b68a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48b6b3; continue; }
  pc = 0x48b693; continue;
 }
 0x48b693 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b6b3; continue;
 }
 0x48b6b3 => {
  pc = 0x48b73f; continue;
 }
 0x48b6b8 => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b6c5; continue;
 }
 0x48b6c5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b6ce; continue;
 }
 0x48b6ce => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48b6f7; continue; }
  pc = 0x48b6d7; continue;
 }
 0x48b6d7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x48b782; continue; }
  pc = 0x48b6f7; continue;
 }
 0x48b6f7 => {
  pc = 0x48b750; continue;
 }
 0x48b6f9 => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0x0u8); }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b702; continue;
 }
 0x48b702 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b707; continue;
 }
 0x48b707 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48b72c; continue; }
  pc = 0x48b710; continue;
 }
 0x48b710 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x48b772; continue; }
  pc = 0x48b72c; continue;
 }
 0x48b72c => {
  pc = 0x48b761; continue;
 }
 0x48b72e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x48b73a); f_00456ca1(cpu); if cpu.returned_to != 0x48b73a { return; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 0x48b73f => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x48b74b); f_00456b9e(cpu); if cpu.returned_to != 0x48b74b { return; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 0x48b750 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x48b75c); f_00456da6(cpu); if cpu.returned_to != 0x48b75c { return; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 0x48b761 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x48b76d); f_00456ea4(cpu); if cpu.returned_to != 0x48b76d { return; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 0x48b772 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = 0x96u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b77c); f_0048b792(cpu); if cpu.returned_to != 0x48b77c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 0x48b782 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = 0xff6au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b78c); f_0048b792(cpu); if cpu.returned_to != 0x48b78c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = 0x48b671; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b792); }
 let mut pc: u32 = 0x48b792;
 loop { match pc {
 0x48b792 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48b7e9; continue; }
  pc = 0x48b799; continue;
 }
 0x48b799 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x48b7d3); f_0041b7b0(cpu); if cpu.returned_to != 0x48b7d3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { cpu.pushad(); }
  cpu.push(0x48b7e1); f_00413337(cpu); if cpu.returned_to != 0x48b7e1 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  pc = 0x48b7e9; continue;
 }
 0x48b7e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b7ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b7ea); } r_0048b7ea(cpu, 0x48b7ea); }
pub fn f_0048b800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b800); } r_0048b7ea(cpu, 0x48b800); }
fn r_0048b7ea(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48b7ea => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b814; continue; }
  pc = 0x48b7f2; continue;
 }
 0x48b7f2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48b814; continue; }
  pc = 0x48b7fb; continue;
 }
 0x48b7fb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x48b814; continue; }
  pc = 0x48b80f; continue;
 }
 0x48b800 => {
  { let v: u32 = cpu.r32(0x8c3803e1u32.wrapping_add(cpu.ebx)) ^ cpu.eax; cpu.w32(0x8c3803e1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.r32(0xffffffc4u32.wrapping_add(cpu.edx)), 0x58590058u32); cpu.w32(0xffffffc4u32.wrapping_add(cpu.edx), v); }
  if !cpu.zf { pc = 0x48b814; continue; }
  pc = 0x48b80f; continue;
 }
 0x48b80f => {
  cpu.push(0x48b814); f_0048b792(cpu); if cpu.returned_to != 0x48b814 { return; }
  pc = 0x48b814; continue;
 }
 0x48b814 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b815(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b815); }
 let mut pc: u32 = 0x48b815;
 loop { match pc {
 0x48b815 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x48b825); f_0048b7ea(cpu); if cpu.returned_to != 0x48b825 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b827(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b827); }
 let mut pc: u32 = 0x48b827;
 loop { match pc {
 0x48b827 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x48b837); f_0048b7ea(cpu); if cpu.returned_to != 0x48b837 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b839(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b839); } r_0048b839(cpu, 0x48b839); }
pub fn f_0048b83a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b83a); } r_0048b839(cpu, 0x48b83a); }
fn r_0048b839(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48b839 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x48b83a; continue;
 }
 0x48b83a => {
  { let v: u32 = 0xffffffe0u32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x48b849); f_0048b7ea(cpu); if cpu.returned_to != 0x48b849 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b84b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b84b); }
 let mut pc: u32 = 0x48b84b;
 loop { match pc {
 0x48b84b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x48b85b); f_0048b7ea(cpu); if cpu.returned_to != 0x48b85b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b85d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b85d); } r_0048b85d(cpu, 0x48b85d); }
pub fn f_0048b85e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b85e); } r_0048b85d(cpu, 0x48b85e); }
pub fn f_0048b860(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b860); } r_0048b85d(cpu, 0x48b860); }
pub fn f_0048b8c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b8c3); } r_0048b85d(cpu, 0x48b8c3); }
fn r_0048b85d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48b85d => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b8bf; continue; }
  pc = 0x48b865; continue;
 }
 0x48b85e => {
  { cpu.df = true; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x57u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(2))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x57u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(2)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48b874; continue; }
  pc = 0x48b86f; continue;
 }
 0x48b860 => {
  cpu.unimplemented(0x48b860, "Aas");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x57u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(2))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x57u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(2)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48b874; continue; }
  pc = 0x48b86f; continue;
 }
 0x48b865 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48b874; continue; }
  pc = 0x48b86f; continue;
 }
 0x48b86f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48b879; continue; }
  pc = 0x48b874; continue;
 }
 0x48b874 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x48b879; continue;
 }
 0x48b879 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x48b8bf; continue; }
  pc = 0x48b884; continue;
 }
 0x48b884 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48b8bf; continue; }
  pc = 0x48b88b; continue;
 }
 0x48b88b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b89b); f_0047111f(cpu); if cpu.returned_to != 0x48b89b { return; }
  if !cpu.cf { pc = 0x48b8c2; continue; }
  pc = 0x48b89d; continue;
 }
 0x48b89d => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48b8af; continue; }
  pc = 0x48b8ac; continue;
 }
 0x48b8ac => {
  { cpu.popad(); }
  pc = 0x48b8bf; continue;
 }
 0x48b8af => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.popad(); }
  cpu.push(0x48b8bf); f_0048bb38(cpu); if cpu.returned_to != 0x48b8bf { return; }
  pc = 0x48b8bf; continue;
 }
 0x48b8bf => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b8c2 => {
  { cpu.popad(); }
  pc = 0x48b8c3; continue;
 }
 0x48b8c3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b8c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b8c6); }
 let mut pc: u32 = 0x48b8c6;
 loop { match pc {
 0x48b8c6 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b8d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b8d5); } r_0048b8d5(cpu, 0x48b8d5); }
pub fn f_0048b8d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48b8d8); } r_0048b8d5(cpu, 0x48b8d8); }
fn r_0048b8d5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48b8d5 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xe8000000u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0xe8000000u32.wrapping_add(cpu.ecx), v); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b8d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b8ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b8ee); }
 let mut pc: u32 = 0x48b8ee;
 loop { match pc {
 0x48b8ee => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x48b914; continue; }
  pc = 0x48b8f5; continue;
 }
 0x48b8f5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  cpu.push(0x48b900); f_0046ab56(cpu); if cpu.returned_to != 0x48b900 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b913); f_00489637(cpu); if cpu.returned_to != 0x48b913 { return; }
  { cpu.popad(); }
  pc = 0x48b914; continue;
 }
 0x48b914 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b917(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b917); }
 let mut pc: u32 = 0x48b917;
 loop { match pc {
 0x48b917 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b91a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b91a); }
 let mut pc: u32 = 0x48b91a;
 loop { match pc {
 0x48b91a => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b91d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b91d); }
 let mut pc: u32 = 0x48b91d;
 loop { match pc {
 0x48b91d => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48b93e; continue; }
  pc = 0x48b925; continue;
 }
 0x48b925 => {
  { let v: u32 = 0x81u32; cpu.w32(0x4d6b64u32, v); }
  cpu.push(0x48b934); f_0048bc84(cpu); if cpu.returned_to != 0x48b934 { return; }
  if cpu.cf { pc = 0x48b939; continue; }
  pc = 0x48b936; continue;
 }
 0x48b936 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48b939 => {
  cpu.push(0x48b93e); f_0048b983(cpu); if cpu.returned_to != 0x48b93e { return; }
  pc = 0x48b93e; continue;
 }
 0x48b93e => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b941(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b941); }
 let mut pc: u32 = 0x48b941;
 loop { match pc {
 0x48b941 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8cu32.wrapping_add(cpu.ebp))); cpu.w16(0x8cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x8cu32.wrapping_add(cpu.ebp)), 0xc8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48b971; continue; }
  pc = 0x48b95a; continue;
 }
 0x48b95a => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1b7u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x48b969); f_00458cb3(cpu); if cpu.returned_to != 0x48b969 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1b7u32, v); }
  pc = 0x48b971; continue;
 }
 0x48b971 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b974); }
 let mut pc: u32 = 0x48b974;
 loop { match pc {
 0x48b974 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1b7u32, v); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b983(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b983); }
 let mut pc: u32 = 0x48b983;
 loop { match pc {
 0x48b983 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b984(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b984); }
 let mut pc: u32 = 0x48b984;
 loop { match pc {
 0x48b984 => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.zf { pc = 0x48ba7e; continue; }
  pc = 0x48b990; continue;
 }
 0x48b990 => {
  { let v: u8 = cpu.or8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48b99a); f_0048babe(cpu); if cpu.returned_to != 0x48b99a { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48b9a9); f_0047a2d9(cpu); if cpu.returned_to != 0x48b9a9 { return; }
  if cpu.cf { pc = 0x48ba7e; continue; }
  pc = 0x48b9af; continue;
 }
 0x48b9af => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x1d3u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48ba7e; continue; }
  pc = 0x48b9bb; continue;
 }
 0x48b9bb => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56c72eu32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x48ba7e; continue; }
  pc = 0x48b9d2; continue;
 }
 0x48b9d2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  cpu.push(0x48b9de); f_0044ea92(cpu); if cpu.returned_to != 0x48b9de { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x48ba10; continue; }
  pc = 0x48b9e9; continue;
 }
 0x48b9e9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b99f6u32, v); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c72eu32), 0x4u8); }
  if cpu.zf { pc = 0x48ba15; continue; }
  pc = 0x48b9fc; continue;
 }
 0x48b9fc => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x56c72eu32)); }
  if !cpu.zf { pc = 0x48ba15; continue; }
  pc = 0x48ba09; continue;
 }
 0x48ba09 => {
  cpu.push(0x48ba0e); f_00416a14(cpu); if cpu.returned_to != 0x48ba0e { return; }
  pc = 0x48ba15; continue;
 }
 0x48ba10 => {
  cpu.push(0x48ba15); f_004169d3(cpu); if cpu.returned_to != 0x48ba15 { return; }
  pc = 0x48ba15; continue;
 }
 0x48ba15 => {
  { let v: u16 = 0x1feu16; cpu.w16(0x16u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48ba22); f_00455069(cpu); if cpu.returned_to != 0x48ba22 { return; }
  cpu.push(0x48ba27); f_00479ba4(cpu); if cpu.returned_to != 0x48ba27 { return; }
  cpu.push(0x48ba2c); f_0047227b(cpu); if cpu.returned_to != 0x48ba2c { return; }
  cpu.push(0x48ba31); f_00471fab(cpu); if cpu.returned_to != 0x48ba31 { return; }
  cpu.push(0x48ba36); f_004665e2(cpu); if cpu.returned_to != 0x48ba36 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x48ba3c); f_00477de2(cpu); if cpu.returned_to != 0x48ba3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x48ba42); f_0045a72e(cpu); if cpu.returned_to != 0x48ba42 { return; }
  { cpu.sub8(cpu.r8(0x5bu32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x48ba7e; continue; }
  pc = 0x48ba49; continue;
 }
 0x48ba49 => {
  { cpu.sub16(cpu.r16(0x36u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x48ba7e; continue; }
  pc = 0x48ba52; continue;
 }
 0x48ba52 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x559754u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x48ba66); f_00488c8f(cpu); if cpu.returned_to != 0x48ba66 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x48ba7e; continue; }
  pc = 0x48ba79; continue;
 }
 0x48ba79 => {
  cpu.push(0x48ba7e); f_00489968(cpu); if cpu.returned_to != 0x48ba7e { return; }
  pc = 0x48ba7e; continue;
 }
 0x48ba7e => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048babe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48babe); } r_0048babe(cpu, 0x48babe); }
pub fn f_0048bb00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48bb00); } r_0048babe(cpu, 0x48bb00); }
fn r_0048babe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48babe => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48bace); f_0047a2d9(cpu); if cpu.returned_to != 0x48bace { return; }
  if cpu.cf { pc = 0x48bb36; continue; }
  pc = 0x48bad0; continue;
 }
 0x48bad0 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x48bae3; continue;
 }
 0x48bae3 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x48bb2d; continue; }
  pc = 0x48baea; continue;
 }
 0x48baea => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x48bb2d; continue; }
  pc = 0x48baf0; continue;
 }
 0x48baf0 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x48bb2d; continue; }
  pc = 0x48baf9; continue;
 }
 0x48baf9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48bb07); f_0047a2d9(cpu); if cpu.returned_to != 0x48bb07 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x48bb2d; continue; }
  pc = 0x48bb0c; continue;
 }
 0x48bb00 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48bb07); f_0047a2d9(cpu); if cpu.returned_to != 0x48bb07 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x48bb2d; continue; }
  pc = 0x48bb0c; continue;
 }
 0x48bb0c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.edi)) & 0xfbu8; cpu.w8(0x87u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48bb27; continue; }
  pc = 0x48bb20; continue;
 }
 0x48bb20 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x48bb27); f_00455368(cpu); if cpu.returned_to != 0x48bb27 { return; }
  pc = 0x48bb27; continue;
 }
 0x48bb27 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x48bb2d; continue;
 }
 0x48bb2d => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48bae3; continue; }
  pc = 0x48bb36; continue;
 }
 0x48bb36 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bb38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bb38); }
 let mut pc: u32 = 0x48bb38;
 loop { match pc {
 0x48bb38 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x81u32; cpu.w32(0x4d6b64u32, v); }
  cpu.push(0x48bb49); f_0048bc84(cpu); if cpu.returned_to != 0x48bb49 { return; }
  if cpu.cf { pc = 0x48bb4e; continue; }
  pc = 0x48bb4b; continue;
 }
 0x48bb4b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48bb4e => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x48bbe9; continue; }
  pc = 0x48bb65; continue;
 }
 0x48bb65 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x48bb7d); f_0046ab56(cpu); if cpu.returned_to != 0x48bb7d { return; }
  cpu.push(0x48bb82); f_00489968(cpu); if cpu.returned_to != 0x48bb82 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xf4u8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x48bbd0); f_0041b7b0(cpu); if cpu.returned_to != 0x48bbd0 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x48bbd6); f_00413337(cpu); if cpu.returned_to != 0x48bbd6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48bbe9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x48bc01; continue; }
  pc = 0x48bbf2; continue;
 }
 0x48bbf2 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x48bc06; continue; }
  pc = 0x48bc01; continue;
 }
 0x48bc01 => {
  cpu.push(0x48bc06); f_00416a48(cpu); if cpu.returned_to != 0x48bc06 { return; }
  pc = 0x48bc06; continue;
 }
 0x48bc06 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x509290u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x48bc33; continue; }
  pc = 0x48bc2e; continue;
 }
 0x48bc2e => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48bc38; continue; }
  pc = 0x48bc33; continue;
 }
 0x48bc33 => {
  { let v: u32 = 0x5b355cu32; cpu.ebp = v; }
  pc = 0x48bc38; continue;
 }
 0x48bc38 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.eax = v; }
  { let v: u32 = 0x14u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x48bc6e; continue; }
  pc = 0x48bc69; continue;
 }
 0x48bc69 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x48bc6e; continue;
 }
 0x48bc6e => {
  cpu.push(0x48bc73); f_004456ab(cpu); if cpu.returned_to != 0x48bc73 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x48bc79); f_0048b31d(cpu); if cpu.returned_to != 0x48bc79 { return; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bc84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bc84); }
 let mut pc: u32 = 0x48bc84;
 loop { match pc {
 0x48bc84 => {
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x48bc8e; continue;
 }
 0x48bc8e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48bc9c); f_00420989(cpu); if cpu.returned_to != 0x48bc9c { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6b64u32)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x48bcb9; continue; }
  pc = 0x48bcb2; continue;
 }
 0x48bcb2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48bc8e; continue; }
  pc = 0x48bcb7; continue;
 }
 0x48bcb7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48bcb9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bcbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bcbb); }
 let mut pc: u32 = 0x48bcbb;
 loop { match pc {
 0x48bcbb => {
  cpu.push(0x48bcc0); f_0048afc8(cpu); if cpu.returned_to != 0x48bcc0 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x48bcd5; continue; }
  pc = 0x48bcd3; continue;
 }
 0x48bcd3 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48bcd5; continue;
 }
 0x48bcd5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x61u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bcda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bcda); }
 let mut pc: u32 = 0x48bcda;
 loop { match pc {
 0x48bcda => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48bd03); f_00420989(cpu); if cpu.returned_to != 0x48bd03 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c5ecu32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x48be78; continue; }
  pc = 0x48bd20; continue;
 }
 0x48bd20 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c5ecu32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x48be78; continue; }
  pc = 0x48bd33; continue;
 }
 0x48bd33 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c5ecu32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x48be78; continue; }
  pc = 0x48bd49; continue;
 }
 0x48bd49 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c5ecu32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x48be78; continue; }
  pc = 0x48bd5f; continue;
 }
 0x48bd5f => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  pc = 0x48bd69; continue;
 }
 0x48bd69 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48bdc3; continue; }
  pc = 0x48bd70; continue;
 }
 0x48bd70 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x48bdc3; continue; }
  pc = 0x48bd77; continue;
 }
 0x48bd77 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4d6b64u32)); }
  if cpu.zf { pc = 0x48bdc3; continue; }
  pc = 0x48bd7f; continue;
 }
 0x48bd7f => {
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5b9ff4u32.wrapping_add(cpu.edx)) as u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.edx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48bdc3; continue; }
  pc = 0x48bd9e; continue;
 }
 0x48bd9e => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.edx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.esi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48bdc3; continue; }
  pc = 0x48bdab; continue;
 }
 0x48bdab => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x48bdc3; continue; }
  pc = 0x48bdb5; continue;
 }
 0x48bdb5 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x48be78; continue; }
  pc = 0x48bdc3; continue;
 }
 0x48bdc3 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48bd69; continue; }
  pc = 0x48bdcc; continue;
 }
 0x48bdcc => {
  cpu.push(0x48bdd1); f_0048bedb(cpu); if cpu.returned_to != 0x48bdd1 { return; }
  if cpu.cf { pc = 0x48be78; continue; }
  pc = 0x48bdd7; continue;
 }
 0x48bdd7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48bde4); f_00420989(cpu); if cpu.returned_to != 0x48bde4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48be12; continue; }
  pc = 0x48bdf9; continue;
 }
 0x48bdf9 => {
  { let v: u32 = cpu.r32(0x576c06u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x48be09; continue; }
  pc = 0x48be04; continue;
 }
 0x48be04 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x48be12; continue; }
  pc = 0x48be09; continue;
 }
 0x48be09 => {
  { let v: u8 = 0x1u8; cpu.w8(0x576c0au32, v); }
  pc = 0x48be2f; continue;
 }
 0x48be12 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x48be1c; continue; }
  pc = 0x48be17; continue;
 }
 0x48be17 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48be21; continue; }
  pc = 0x48be1c; continue;
 }
 0x48be1c => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x48be21; continue;
 }
 0x48be21 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x48be2f; continue; }
  pc = 0x48be27; continue;
 }
 0x48be27 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x48be4c; continue;
 }
 0x48be2f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x48be4c; continue;
 }
 0x48be4c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | 0x1cu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) | 0x1cu8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x80u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)) | 0x1cu8; cpu.w8(0x80u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x81u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x81u32.wrapping_add(cpu.esi)), 0x1cu8); cpu.w8(0x81u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48be78 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5da9b2u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  cpu.push(0x48be9c); f_004685fb(cpu); if cpu.returned_to != 0x48be9c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  cpu.push(0x48bead); f_004685fb(cpu); if cpu.returned_to != 0x48bead { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  cpu.push(0x48bebe); f_004685fb(cpu); if cpu.returned_to != 0x48bebe { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xcu32); cpu.ebx = v; }
  cpu.push(0x48becf); f_004685fb(cpu); if cpu.returned_to != 0x48becf { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5da9b2u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bedb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bedb); }
 let mut pc: u32 = 0x48bedb;
 loop { match pc {
 0x48bedb => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u32 = 0xd7u32; cpu.ecx = v; }
  pc = 0x48bee7; continue;
 }
 0x48bee7 => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48bf18; continue; }
  pc = 0x48bf03; continue;
 }
 0x48bf03 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48bf18; continue; }
  pc = 0x48bf07; continue;
 }
 0x48bf07 => {
  { let v: u32 = cpu.edx.wrapping_sub(0x20u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x20u32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x48bf18; continue; }
  pc = 0x48bf11; continue;
 }
 0x48bf11 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x48bf18; continue; }
  pc = 0x48bf15; continue;
 }
 0x48bf15 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48bf18 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xffu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48bee7; continue; }
  pc = 0x48bf20; continue;
 }
 0x48bf20 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bf23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48bf23); } r_0048bf23(cpu, 0x48bf23); }
pub fn f_0048c000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c000); } r_0048bf23(cpu, 0x48c000); }
fn r_0048bf23(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48bf23 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc69du32, v); }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x48bf3a); f_0045c7e2(cpu); if cpu.returned_to != 0x48bf3a { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if !cpu.cf { pc = 0x48bf9e; continue; }
  pc = 0x48bf4e; continue;
 }
 0x48bf4e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48bf95; continue; }
  pc = 0x48bf70; continue;
 }
 0x48bf70 => {
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48bf87); f_0047111f(cpu); if cpu.returned_to != 0x48bf87 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x48bf95; continue; }
  pc = 0x48bf8b; continue;
 }
 0x48bf8b => {
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x48bf9d; continue;
 }
 0x48bf95 => {
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x48bf9d; continue;
 }
 0x48bf9d => {
  { cpu.popad(); }
  pc = 0x48bf9e; continue;
 }
 0x48bf9e => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x36u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d685au32).wrapping_sub(0x7u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_sub(0x7u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_sub(0x8u16); cpu.w16(0x4d685cu32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48bfdd); f_0048c02f(cpu); if cpu.returned_to != 0x48bfdd { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d685au32).wrapping_add(0x7u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x7u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32).wrapping_add(0x8u16); cpu.w16(0x4d685cu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x36u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x48c02e; continue; }
  pc = 0x48c01f; continue;
 }
 0x48c000 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3547c6ffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.unimplemented(0x48c00d, "Aaa");
  let t = cpu.r32(0x5fu32.wrapping_add(cpu.ecx)); cpu.push(cpu.segments[1] as u32); cpu.push(0x48c011); cpu.call(t); if cpu.returned_to != 0x48c011 { return; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x48c02e; continue; }
  pc = 0x48c01f; continue;
 }
 0x48c01f => {
  { cpu.sub8(cpu.r8(0x5cc69du32), 0x0u8); }
  if !cpu.zf { pc = 0x48c02d; continue; }
  pc = 0x48c028; continue;
 }
 0x48c028 => {
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c02d; continue;
 }
 0x48c02d => {
  { cpu.cf = false; }
  pc = 0x48c02e; continue;
 }
 0x48c02e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c02f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c02f); } r_0048c02f(cpu, 0x48c02f); }
pub fn f_0048c0a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c0a2); } r_0048c02f(cpu, 0x48c0a2); }
pub fn f_0048c0f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c0f8); } r_0048c02f(cpu, 0x48c0f8); }
pub fn f_0048c125(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c125); } r_0048c02f(cpu, 0x48c125); }
pub fn f_0048c1aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48c1aa); } r_0048c02f(cpu, 0x48c1aa); }
fn r_0048c02f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x48c02f => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b7cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b80u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48c098; continue; }
  pc = 0x48c070; continue;
 }
 0x48c070 => {
  if cpu.sf { pc = 0x48c085; continue; }
  pc = 0x48c072; continue;
 }
 0x48c072 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x48c125; continue; }
  pc = 0x48c07a; continue;
 }
 0x48c07a => {
  if !cpu.sf { pc = 0x48c1e6; continue; }
  pc = 0x48c080; continue;
 }
 0x48c080 => {
  pc = 0x48c20e; continue;
 }
 0x48c085 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x48c1aa; continue; }
  pc = 0x48c08d; continue;
 }
 0x48c08d => {
  if !cpu.sf { pc = 0x48c236; continue; }
  pc = 0x48c093; continue;
 }
 0x48c093 => {
  pc = 0x48c25e; continue;
 }
 0x48c098 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x48c0a0; continue; }
  pc = 0x48c09c; continue;
 }
 0x48c09c => {
  if !cpu.sf { pc = 0x48c0f8; continue; }
  pc = 0x48c09e; continue;
 }
 0x48c09e => {
  pc = 0x48c0a2; continue;
 }
 0x48c0a0 => {
  { cpu.cf = false; }
  pc = 0x48c0a1; continue;
 }
 0x48c0a1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c0a2 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c0ab; continue;
 }
 0x48c0ab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48c0d1; continue; }
  pc = 0x48c0b4; continue;
 }
 0x48c0b4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48c0d1; continue; }
  pc = 0x48c0be; continue;
 }
 0x48c0be => {
  { cpu.and8(cpu.r8(0x4d685au32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c0c7; continue;
 }
 0x48c0c7 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c0d1; continue;
 }
 0x48c0d1 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x48c0e2; continue; }
  pc = 0x48c0d5; continue;
 }
 0x48c0d5 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c0ec; continue;
 }
 0x48c0e2 => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c0ec; continue;
 }
 0x48c0ec => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b80u32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x48c0a1; continue;
 }
 0x48c0f8 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c101; continue;
 }
 0x48c101 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c10a; continue;
 }
 0x48c10a => {
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x48c0d1; continue; }
  pc = 0x48c119; continue;
 }
 0x48c119 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x48c0d1; continue; }
  pc = 0x48c123; continue;
 }
 0x48c123 => {
  cpu.poll(); pc = 0x48c0be; continue;
 }
 0x48c125 => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c132; continue;
 }
 0x48c132 => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c13f; continue;
 }
 0x48c13f => {
  { let v: u32 = cpu.edi.wrapping_add(0x82u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48c18c; continue; }
  pc = 0x48c14e; continue;
 }
 0x48c14e => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { pc = 0x48c16c; continue; }
  pc = 0x48c157; continue;
 }
 0x48c157 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48c18c; continue; }
  pc = 0x48c166; continue;
 }
 0x48c166 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  pc = 0x48c16c; continue;
 }
 0x48c16c => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c179; continue;
 }
 0x48c179 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c18c; continue;
 }
 0x48c18c => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x48c196; continue; }
  pc = 0x48c190; continue;
 }
 0x48c190 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c196; continue;
 }
 0x48c196 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b7cu32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x48c0a1; continue;
 }
 0x48c1aa => {
  { cpu.and8(cpu.r8(0x4d685au32), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x48c0a0; continue; }
  pc = 0x48c1b7; continue;
 }
 0x48c1b7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x48c18c; continue; }
  pc = 0x48c1c6; continue;
 }
 0x48c1c6 => {
  { cpu.and8(cpu.r8(0x4d685cu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x48c16c; continue; }
  pc = 0x48c1cf; continue;
 }
 0x48c1cf => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x48c18c; continue; }
  pc = 0x48c1de; continue;
 }
 0x48c1de => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  cpu.poll(); pc = 0x48c16c; continue;
 }
 0x48c1e6 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48c1f6); f_0048c125(cpu); if cpu.returned_to != 0x48c1f6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48c1fc); f_0048c0f8(cpu); if cpu.returned_to != 0x48c1fc { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48c20c; continue; }
  pc = 0x48c20a; continue;
 }
 0x48c20a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c20c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c20e => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48c21e); f_0048c125(cpu); if cpu.returned_to != 0x48c21e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48c224); f_0048c0a2(cpu); if cpu.returned_to != 0x48c224 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48c234; continue; }
  pc = 0x48c232; continue;
 }
 0x48c232 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c234 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c236 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48c246); f_0048c1aa(cpu); if cpu.returned_to != 0x48c246 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48c24c); f_0048c0f8(cpu); if cpu.returned_to != 0x48c24c { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48c25c; continue; }
  pc = 0x48c25a; continue;
 }
 0x48c25a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c25c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c25e => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48c26e); f_0048c1aa(cpu); if cpu.returned_to != 0x48c26e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x48c274); f_0048c0a2(cpu); if cpu.returned_to != 0x48c274 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x48c284; continue; }
  pc = 0x48c282; continue;
 }
 0x48c282 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c284 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c288(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c288); }
 let mut pc: u32 = 0x48c288;
 loop { match pc {
 0x48c288 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x5da9b4u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x5da9b8u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48c2b9; continue; }
  pc = 0x48c2a8; continue;
 }
 0x48c2a8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x48c2b9; continue; }
  pc = 0x48c2af; continue;
 }
 0x48c2af => {
  { let v: u32 = 0xcu32; cpu.w32(0x5da9b4u32, v); }
  pc = 0x48c2b9; continue;
 }
 0x48c2b9 => {
  { let v: u32 = 0x8u32; cpu.w32(0x5da9bcu32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x5da9c0u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x48c2e3; continue; }
  pc = 0x48c2d3; continue;
 }
 0x48c2d3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x48c2e3; continue; }
  pc = 0x48c2d9; continue;
 }
 0x48c2d9 => {
  { let v: u32 = 0xcu32; cpu.w32(0x5da9bcu32, v); }
  pc = 0x48c2e3; continue;
 }
 0x48c2e3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x48c312; continue; }
  pc = 0x48c2f0; continue;
 }
 0x48c2f0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x48c312; continue; }
  pc = 0x48c2f7; continue;
 }
 0x48c2f7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x48c312; continue; }
  pc = 0x48c300; continue;
 }
 0x48c300 => {
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x48c30a; continue; }
  pc = 0x48c305; continue;
 }
 0x48c305 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48c30f; continue; }
  pc = 0x48c30a; continue;
 }
 0x48c30a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x48c30f; continue;
 }
 0x48c30f => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x48c312; continue;
 }
 0x48c312 => {
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5da9b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5da9bcu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5da9b8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5da9c0u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.cf { pc = 0x48c369; continue; }
  pc = 0x48c366; continue;
 }
 0x48c366 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c369 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c36c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c36c); }
 let mut pc: u32 = 0x48c36c;
 loop { match pc {
 0x48c36c => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c376); f_00472b00(cpu); if cpu.returned_to != 0x48c376 { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x48c382; continue; }
  pc = 0x48c37d; continue;
 }
 0x48c37d => {
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c382; continue;
 }
 0x48c382 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c383(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c383); }
 let mut pc: u32 = 0x48c383;
 loop { match pc {
 0x48c383 => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c38d); f_00472b00(cpu); if cpu.returned_to != 0x48c38d { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x48c3c3; continue; }
  pc = 0x48c394; continue;
 }
 0x48c394 => {
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x48c3a7; continue; }
  pc = 0x48c3a2; continue;
 }
 0x48c3a2 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48c3ac; continue; }
  pc = 0x48c3a7; continue;
 }
 0x48c3a7 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x48c3ac; continue;
 }
 0x48c3ac => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x48c3be; continue; }
  pc = 0x48c3b7; continue;
 }
 0x48c3b7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c3c3; continue;
 }
 0x48c3be => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c3c3; continue;
 }
 0x48c3c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c3c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c3c4); }
 let mut pc: u32 = 0x48c3c4;
 loop { match pc {
 0x48c3c4 => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c3ce); f_00472b00(cpu); if cpu.returned_to != 0x48c3ce { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x48c404; continue; }
  pc = 0x48c3d5; continue;
 }
 0x48c3d5 => {
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x48c3e8; continue; }
  pc = 0x48c3e3; continue;
 }
 0x48c3e3 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48c3ed; continue; }
  pc = 0x48c3e8; continue;
 }
 0x48c3e8 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x48c3ed; continue;
 }
 0x48c3ed => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x48c3ff; continue; }
  pc = 0x48c3f8; continue;
 }
 0x48c3f8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c404; continue;
 }
 0x48c3ff => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c404; continue;
 }
 0x48c404 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c408(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c408); }
 let mut pc: u32 = 0x48c408;
 loop { match pc {
 0x48c408 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x48c45b; continue; }
  pc = 0x48c412; continue;
 }
 0x48c412 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x48c45c; continue; }
  pc = 0x48c422; continue;
 }
 0x48c422 => {
  if !cpu.zf { pc = 0x48c45b; continue; }
  pc = 0x48c424; continue;
 }
 0x48c424 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x48c462; continue; }
  pc = 0x48c42b; continue;
 }
 0x48c42b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x48c439; continue; }
  pc = 0x48c432; continue;
 }
 0x48c432 => {
  cpu.push(0x48c437); f_0048c52d(cpu); if cpu.returned_to != 0x48c437 { return; }
  pc = 0x48c44c; continue;
 }
 0x48c439 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x48c447; continue; }
  pc = 0x48c440; continue;
 }
 0x48c440 => {
  cpu.push(0x48c445); f_0048c6ab(cpu); if cpu.returned_to != 0x48c445 { return; }
  pc = 0x48c44c; continue;
 }
 0x48c447 => {
  cpu.push(0x48c44c); f_0048c4b2(cpu); if cpu.returned_to != 0x48c44c { return; }
  pc = 0x48c44c; continue;
 }
 0x48c44c => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c45b); f_004824fa(cpu); if cpu.returned_to != 0x48c45b { return; }
  pc = 0x48c45b; continue;
 }
 0x48c45b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c45c => {
  cpu.push(0x48c461); f_0043ffdb(cpu); if cpu.returned_to != 0x48c461 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c462 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c467); }
 let mut pc: u32 = 0x48c467;
 loop { match pc {
 0x48c467 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x48c4b1; continue; }
  pc = 0x48c46d; continue;
 }
 0x48c46d => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x48c485; continue; }
  pc = 0x48c47d; continue;
 }
 0x48c47d => {
  if cpu.cf { pc = 0x48c4b1; continue; }
  pc = 0x48c47f; continue;
 }
 0x48c47f => {
  cpu.push(0x48c484); f_0043ffdb(cpu); if cpu.returned_to != 0x48c484 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c485 => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c4b1); f_0045b012(cpu); if cpu.returned_to != 0x48c4b1 { return; }
  pc = 0x48c4b1; continue;
 }
 0x48c4b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c4b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c4b2); }
 let mut pc: u32 = 0x48c4b2;
 loop { match pc {
 0x48c4b2 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x48c4c1); f_004679cb(cpu); if cpu.returned_to != 0x48c4c1 { return; }
  if cpu.cf { pc = 0x48c506; continue; }
  pc = 0x48c4c3; continue;
 }
 0x48c4c3 => {
  { cpu.sub8(cpu.r8(0x4eab2bu32), 0x0u8); }
  if !cpu.zf { pc = 0x48c506; continue; }
  pc = 0x48c4cc; continue;
 }
 0x48c4cc => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48c4f2; continue; }
  pc = 0x48c4e2; continue;
 }
 0x48c4e2 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48c4e9); f_00401130(cpu); if cpu.returned_to != 0x48c4e9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48c4f2; continue;
 }
 0x48c4f2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x48c505); f_00415d61(cpu); if cpu.returned_to != 0x48c505 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48c506; continue;
 }
 0x48c506 => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x48c51a); f_0045bd38(cpu); if cpu.returned_to != 0x48c51a { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  cpu.push(0x48c52b); f_004820da(cpu); if cpu.returned_to != 0x48c52b { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c52d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c52d); }
 let mut pc: u32 = 0x48c52d;
 loop { match pc {
 0x48c52d => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x48c53c); f_004679cb(cpu); if cpu.returned_to != 0x48c53c { return; }
  if cpu.cf { pc = 0x48c575; continue; }
  pc = 0x48c53e; continue;
 }
 0x48c53e => {
  { cpu.sub8(cpu.r8(0x4eab2bu32), 0x0u8); }
  if !cpu.zf { pc = 0x48c575; continue; }
  pc = 0x48c547; continue;
 }
 0x48c547 => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48c56d; continue; }
  pc = 0x48c55d; continue;
 }
 0x48c55d => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48c564); f_00401130(cpu); if cpu.returned_to != 0x48c564 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48c56d; continue;
 }
 0x48c56d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4eab2bu32, v); }
  pc = 0x48c575; continue;
 }
 0x48c575 => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x48c593); f_0045bd38(cpu); if cpu.returned_to != 0x48c593 { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  cpu.push(0x48c5a4); f_004585d0(cpu); if cpu.returned_to != 0x48c5a4 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c5a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c5a6); }
 let mut pc: u32 = 0x48c5a6;
 loop { match pc {
 0x48c5a6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x48c5f1; continue; }
  pc = 0x48c5ac; continue;
 }
 0x48c5ac => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48c5e0; continue; }
  pc = 0x48c5b7; continue;
 }
 0x48c5b7 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48c5e7; continue; }
  pc = 0x48c5b9; continue;
 }
 0x48c5b9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x48c5f2; continue; }
  pc = 0x48c5c5; continue;
 }
 0x48c5c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c5cf); f_0048c730(cpu); if cpu.returned_to != 0x48c5cf { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c5de); f_004824fa(cpu); if cpu.returned_to != 0x48c5de { return; }
  pc = 0x48c5f1; continue;
 }
 0x48c5e0 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c5f1; continue;
 }
 0x48c5e7 => {
  cpu.push(0x48c5ec); f_0044bc7b(cpu); if cpu.returned_to != 0x48c5ec { return; }
  cpu.push(0x48c5f1); f_0043ffdb(cpu); if cpu.returned_to != 0x48c5f1 { return; }
  pc = 0x48c5f1; continue;
 }
 0x48c5f1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c5f2 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c5f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c5f7); }
 let mut pc: u32 = 0x48c5f7;
 loop { match pc {
 0x48c5f7 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x48c649; continue; }
  pc = 0x48c5fd; continue;
 }
 0x48c5fd => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48c638; continue; }
  pc = 0x48c608; continue;
 }
 0x48c608 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48c63f; continue; }
  pc = 0x48c60a; continue;
 }
 0x48c60a => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x48c64a; continue; }
  pc = 0x48c616; continue;
 }
 0x48c616 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c627); f_0048c52d(cpu); if cpu.returned_to != 0x48c627 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c636); f_004824fa(cpu); if cpu.returned_to != 0x48c636 { return; }
  pc = 0x48c649; continue;
 }
 0x48c638 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c649; continue;
 }
 0x48c63f => {
  cpu.push(0x48c644); f_0044bc7b(cpu); if cpu.returned_to != 0x48c644 { return; }
  cpu.push(0x48c649); f_0043ffdb(cpu); if cpu.returned_to != 0x48c649 { return; }
  pc = 0x48c649; continue;
 }
 0x48c649 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c64a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c64f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c64f); }
 let mut pc: u32 = 0x48c64f;
 loop { match pc {
 0x48c64f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x48c6a5; continue; }
  pc = 0x48c659; continue;
 }
 0x48c659 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48c694; continue; }
  pc = 0x48c664; continue;
 }
 0x48c664 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48c69b; continue; }
  pc = 0x48c666; continue;
 }
 0x48c666 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x48c6a6; continue; }
  pc = 0x48c672; continue;
 }
 0x48c672 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c683); f_0048c6ab(cpu); if cpu.returned_to != 0x48c683 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c692); f_004824fa(cpu); if cpu.returned_to != 0x48c692 { return; }
  pc = 0x48c6a5; continue;
 }
 0x48c694 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c6a5; continue;
 }
 0x48c69b => {
  cpu.push(0x48c6a0); f_0044bc7b(cpu); if cpu.returned_to != 0x48c6a0 { return; }
  cpu.push(0x48c6a5); f_0043ffdb(cpu); if cpu.returned_to != 0x48c6a5 { return; }
  pc = 0x48c6a5; continue;
 }
 0x48c6a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48c6a6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c6ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c6ab); }
 let mut pc: u32 = 0x48c6ab;
 loop { match pc {
 0x48c6ab => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x48c6ba); f_004679cb(cpu); if cpu.returned_to != 0x48c6ba { return; }
  if cpu.cf { pc = 0x48c6e3; continue; }
  pc = 0x48c6bc; continue;
 }
 0x48c6bc => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x48c6e2; continue; }
  pc = 0x48c6d2; continue;
 }
 0x48c6d2 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48c6d9); f_00401130(cpu); if cpu.returned_to != 0x48c6d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x48c6e2; continue;
 }
 0x48c6e2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x48c6e3; continue;
 }
 0x48c6e3 => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x48c6f7); f_0045bd38(cpu); if cpu.returned_to != 0x48c6f7 { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d5ec8u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d5f21u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1c1u32, v); }
  cpu.push(0x48c720); f_004820da(cpu); if cpu.returned_to != 0x48c720 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1c1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5f21u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c730); }
 let mut pc: u32 = 0x48c730;
 loop { match pc {
 0x48c730 => {
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5da9c4u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5da9c6u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4e1f4eu32) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = 0x1du8; cpu.w8(0x5d5f23u32, v); }
  cpu.push(0x48c784); f_00480e87(cpu); if cpu.returned_to != 0x48c784 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48c794); f_004824fa(cpu); if cpu.returned_to != 0x48c794 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c798); }
 let mut pc: u32 = 0x48c798;
 loop { match pc {
 0x48c798 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5da9e8u32, v); }
  pc = 0x48c7a8; continue;
 }
 0x48c7a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x48c7de; continue; }
  pc = 0x48c7c6; continue;
 }
 0x48c7c6 => {
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x48c7de; continue; }
  pc = 0x48c7ce; continue;
 }
 0x48c7ce => {
  cpu.push(0x48c7d3); f_0048c7ea(cpu); if cpu.returned_to != 0x48c7d3 { return; }
  { let v: u32 = cpu.r32(0x5da9e8u32).wrapping_add(cpu.eax); cpu.w32(0x5da9e8u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x48c7a8; continue;
 }
 0x48c7de => {
  { let v: u32 = cpu.r32(0x5da9e8u32); cpu.eax = v; }
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
pub fn f_0048c7ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c7ea); }
 let mut pc: u32 = 0x48c7ea;
 loop { match pc {
 0x48c7ea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d692au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6930u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692eu32, v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x48c8ed; continue; }
  pc = 0x48c826; continue;
 }
 0x48c826 => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.sf { pc = 0x48c8ed; continue; }
  pc = 0x48c82f; continue;
 }
 0x48c82f => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.sf { pc = 0x48c8ed; continue; }
  pc = 0x48c838; continue;
 }
 0x48c838 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.sf { pc = 0x48c8ed; continue; }
  pc = 0x48c841; continue;
 }
 0x48c841 => {
  cpu.push(0x48c846); f_0048c8fa(cpu); if cpu.returned_to != 0x48c846 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.r16(0x4d6940u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x48c86f; continue; }
  pc = 0x48c865; continue;
 }
 0x48c865 => {
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
 0x48c86f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x4d6926u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6928u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x34u8); }
  if cpu.zf { pc = 0x48c896; continue; }
  pc = 0x48c891; continue;
 }
 0x48c891 => {
  { cpu.sub8((cpu.ecx as u8), 0x28u8); }
  if !cpu.zf { pc = 0x48c89c; continue; }
  pc = 0x48c896; continue;
 }
 0x48c896 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5da9edu32)); cpu.w8(0x5da9edu32, v); }
  pc = 0x48c89c; continue;
 }
 0x48c89c => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6c84u32)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x581c00u32), 0x0u8); }
  if !cpu.zf { pc = 0x48c8b3; continue; }
  pc = 0x48c8b3; continue;
 }
 0x48c8b3 => {
  { cpu.sub8((cpu.ecx as u8), 0xau8); }
  if cpu.cf { pc = 0x48c8d2; continue; }
  pc = 0x48c8b8; continue;
 }
 0x48c8b8 => {
  { cpu.sub8(cpu.r8(0x5da9ecu32), 0x0u8); }
  if cpu.zf { pc = 0x48c8c3; continue; }
  pc = 0x48c8c1; continue;
 }
 0x48c8c1 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x48c8c3; continue;
 }
 0x48c8c3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5da9ecu32, v); }
  { let v: u32 = cpu.r32(0x5da9e4u32).wrapping_add(cpu.ecx); cpu.w32(0x5da9e4u32, v); }
  pc = 0x48c8df; continue;
 }
 0x48c8d2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5da9ecu32, v); }
  { let v: u32 = cpu.r32(0x5da9e4u32).wrapping_add(cpu.ecx); cpu.w32(0x5da9e4u32, v); }
  pc = 0x48c8df; continue;
 }
 0x48c8df => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  let t = cpu.r32(0x4d6b14u32); cpu.push(0x48c8e6); cpu.call(t); if cpu.returned_to != 0x48c8e6 { return; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48c86f; continue; }
  pc = 0x48c8e9; continue;
 }
 0x48c8e9 => {
  { cpu.cf = false; }
  pc = 0x48c8ee; continue;
 }
 0x48c8ed => {
  { cpu.cf = true; }
  pc = 0x48c8ee; continue;
 }
 0x48c8ee => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5da9e4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c8fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c8fa); }
 let mut pc: u32 = 0x48c8fa;
 loop { match pc {
 0x48c8fa => {
  { let v: u32 = 0x0u32; cpu.w32(0x5da9e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5da9ecu32, v); }
  { cpu.sub16(cpu.r16(0x4d692au32), 0x0u16); }
  if cpu.zf { pc = 0x48c91c; continue; }
  pc = 0x48c915; continue;
 }
 0x48c915 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d692au32)); cpu.w16(0x4d692au32, v); }
  pc = 0x48c91c; continue;
 }
 0x48c91c => {
  { cpu.sub16(cpu.r16(0x4d6930u32), 0x0u16); }
  if cpu.zf { pc = 0x48c92d; continue; }
  pc = 0x48c926; continue;
 }
 0x48c926 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6930u32)); cpu.w16(0x4d6930u32, v); }
  pc = 0x48c92d; continue;
 }
 0x48c92d => {
  { let v: u16 = cpu.shr16(cpu.r16(0x4d6926u32), 4u32); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d6928u32), 4u32); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d692cu32), 4u32); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x4d692eu32), 4u32); cpu.w16(0x4d692eu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6942u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6944u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6946u32, v); }
  { let v: u16 = cpu.r16(0x4d692cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6930u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6926u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48c9ae; continue; }
  pc = 0x48c985; continue;
 }
 0x48c985 => {
  { let a = cpu.r16(0x4d6926u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x4d6926u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let a = cpu.r16(0x4d6928u32); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.w16(0x4d6928u32, v); } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d692eu32, v); }
  { let a = cpu.r16(0x4d692au32); let b = (cpu.ecx as u16); { let v: u16 = b; cpu.w16(0x4d692au32, v); } { let v: u16 = a; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6930u32, v); }
  pc = 0x48c9ae; continue;
 }
 0x48c9ae => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6926u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6928u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48c9ca; continue; }
  pc = 0x48c9be; continue;
 }
 0x48c9be => {
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6944u32, v); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48c9ca; continue;
 }
 0x48c9ca => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d692au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48c9df; continue; }
  pc = 0x48c9d3; continue;
 }
 0x48c9d3 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d6946u32, v); }
  pc = 0x48c9df; continue;
 }
 0x48c9df => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48ca17; continue; }
  pc = 0x48c9e4; continue;
 }
 0x48c9e4 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48ca4b; continue; }
  pc = 0x48c9e9; continue;
 }
 0x48c9e9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694au32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6948u32, v); }
  { let v: u32 = 0x48ca7au32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48ca17 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.ebx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x48ca4b; continue; }
  pc = 0x48ca1c; continue;
 }
 0x48ca1c => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d694au32, v); }
  { let v: u32 = 0x48caf4u32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48ca4b => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6940u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d694cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d694au32, v); }
  { let v: u32 = 0x48cb6eu32; cpu.w32(0x4d6b14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ca7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ca7a); }
 let mut pc: u32 = 0x48ca7a;
 loop { match pc {
 0x48ca7a => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694au32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694cu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6936u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48caba; continue; }
  pc = 0x48caac; continue;
 }
 0x48caac => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48caba; continue;
 }
 0x48caba => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48cad1; continue; }
  pc = 0x48cac3; continue;
 }
 0x48cac3 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48cad1; continue;
 }
 0x48cad1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d694au32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048caf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48caf4); }
 let mut pc: u32 = 0x48caf4;
 loop { match pc {
 0x48caf4 => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6948u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694cu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48cb34; continue; }
  pc = 0x48cb26; continue;
 }
 0x48cb26 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48cb34; continue;
 }
 0x48cb34 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48cb4b; continue; }
  pc = 0x48cb3d; continue;
 }
 0x48cb3d => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x48cb4b; continue;
 }
 0x48cb4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cb6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cb6e); }
 let mut pc: u32 = 0x48cb6e;
 loop { match pc {
 0x48cb6e => {
  { let v: u16 = cpu.r16(0x4d6926u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6928u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d692au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6948u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d694au32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6946u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48cbae; continue; }
  pc = 0x48cba0; continue;
 }
 0x48cba0 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6942u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x48cbae; continue;
 }
 0x48cbae => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48cbc5; continue; }
  pc = 0x48cbb7; continue;
 }
 0x48cbb7 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6944u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x48cbc5; continue;
 }
 0x48cbc5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d692au32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4d6948u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d694au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cbe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cbe8); }
 let mut pc: u32 = 0x48cbe8;
 loop { match pc {
 0x48cbe8 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x48cbf5; continue;
 }
 0x48cbf5 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x48cc08; continue; }
  pc = 0x48cbfc; continue;
 }
 0x48cbfc => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x48cc08; continue;
 }
 0x48cc08 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48cbf5; continue; }
  pc = 0x48cc0f; continue;
 }
 0x48cc0f => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4b94d4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4b94d8u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  if cpu.sf { pc = 0x48cc27; continue; }
  pc = 0x48cc23; continue;
 }
 0x48cc23 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x48cc32; continue; }
  pc = 0x48cc27; continue;
 }
 0x48cc27 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5d3a0cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48cc32 => {
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5da9f4u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5d3a0cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cc45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cc45); }
 let mut pc: u32 = 0x48cc45;
 loop { match pc {
 0x48cc45 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5d3a00u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d39e4u32, v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x48cc63; continue;
 }
 0x48cc63 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x48cc70; continue; }
  pc = 0x48cc6a; continue;
 }
 0x48cc6a => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x48cc78; continue; }
  pc = 0x48cc70; continue;
 }
 0x48cc70 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48cc63; continue; }
  pc = 0x48cc77; continue;
 }
 0x48cc77 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48cc78 => {
  cpu.push(0x48cc7d); f_0048ccf8(cpu); if cpu.returned_to != 0x48cc7d { return; }
  { let v: u32 = 0x5d3318u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5d385eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x48cc77; continue; }
  pc = 0x48cc8c; continue;
 }
 0x48cc8c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x48cc92; continue;
 }
 0x48cc92 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x48cc92; continue; }
  pc = 0x48cca7; continue;
 }
 0x48cca7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5d3a00u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d39e4u32, v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x48ccdd; continue; }
  pc = 0x48ccbf; continue;
 }
 0x48ccbf => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x524a4du32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x32u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d3a00u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48ccdd => {
  { let v: u32 = cpu.r8(0x524a4du32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x32u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d39e4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ccf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ccf8); }
 let mut pc: u32 = 0x48ccf8;
 loop { match pc {
 0x48ccf8 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x5d3318u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d385eu32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x48cd0f; continue;
 }
 0x48cd0f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x48cd27; continue; }
  pc = 0x48cd16; continue;
 }
 0x48cd16 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x48cd27; continue; }
  pc = 0x48cd1c; continue;
 }
 0x48cd1c => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5d385eu32)); cpu.w8(0x5d385eu32, v); }
  pc = 0x48cd27; continue;
 }
 0x48cd27 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x48cd0f; continue; }
  pc = 0x48cd2d; continue;
 }
 0x48cd2d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cd30); }
 let mut pc: u32 = 0x48cd30;
 loop { match pc {
 0x48cd30 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cd40); }
 let mut pc: u32 = 0x48cd40;
 loop { match pc {
 0x48cd40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x48cd47; continue;
 }
 0x48cd47 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48cd6e; continue; }
  pc = 0x48cd4d; continue;
 }
 0x48cd4d => {
  { let v = 0x4a1680u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x92u32; cpu.push(v); }
  { let v = 0x4a1678u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48cd65); f_00490280(cpu); if cpu.returned_to != 0x48cd65 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48cd6e; continue; }
  pc = 0x48cd6d; continue;
 }
 0x48cd6d => {
  cpu.unimplemented(0x48cd6d, "Int3");
  pc = 0x48cd6e; continue;
 }
 0x48cd6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x48cd47; continue; }
  pc = 0x48cd74; continue;
 }
 0x48cd74 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x83u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48cd9b; continue; }
  pc = 0x48cd89; continue;
 }
 0x48cd89 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48cdad; continue; }
  pc = 0x48cd8f; continue;
 }
 0x48cd8f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x48cdad; continue; }
  pc = 0x48cd95; continue;
 }
 0x48cd95 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x48cdad; continue; }
  pc = 0x48cd9b; continue;
 }
 0x48cd9b => {
  { let v: u32 = 0x16u32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x48ce5f; continue;
 }
 0x48cdad => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffffefu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x48cddd; continue; }
  pc = 0x48cdc2; continue;
 }
 0x48cdc2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48cdcb); f_0048d610(cpu); if cpu.returned_to != 0x48cdcb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48cddd; continue;
 }
 0x48cddd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48cde6); f_0048cfe0(cpu); if cpu.returned_to != 0x48cde6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48ce0a; continue; }
  pc = 0x48cdf9; continue;
 }
 0x48cdf9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x48ce3e; continue;
 }
 0x48ce0a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48ce3e; continue; }
  pc = 0x48ce17; continue;
 }
 0x48ce17 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48ce3e; continue; }
  pc = 0x48ce24; continue;
 }
 0x48ce24 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x400u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x48ce3e; continue; }
  pc = 0x48ce34; continue;
 }
 0x48ce34 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x200u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x48ce3e; continue;
 }
 0x48ce3e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48ce52); f_0048fff0(cpu); if cpu.returned_to != 0x48ce52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x48ce5f; continue;
 }
 0x48ce5f => {
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
pub fn f_0048ce70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ce70); }
 let mut pc: u32 = 0x48ce70;
 loop { match pc {
 0x48ce70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x48ce79; continue;
 }
 0x48ce79 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48ce9d; continue; }
  pc = 0x48ce7f; continue;
 }
 0x48ce7f => {
  { let v = 0x4a16ccu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x35u32; cpu.push(v); }
  { let v = 0x4a16c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48ce94); f_00490280(cpu); if cpu.returned_to != 0x48ce94 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48ce9d; continue; }
  pc = 0x48ce9c; continue;
 }
 0x48ce9c => {
  cpu.unimplemented(0x48ce9c, "Int3");
  pc = 0x48ce9d; continue;
 }
 0x48ce9d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x48ce79; continue; }
  pc = 0x48cea3; continue;
 }
 0x48cea3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x48cecb; continue; }
  pc = 0x48cead; continue;
 }
 0x48cead => {
  { let v = 0x4a16b0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x36u32; cpu.push(v); }
  { let v = 0x4a16c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48cec2); f_00490280(cpu); if cpu.returned_to != 0x48cec2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48cecb; continue; }
  pc = 0x48ceca; continue;
 }
 0x48ceca => {
  cpu.unimplemented(0x48ceca, "Int3");
  pc = 0x48cecb; continue;
 }
 0x48cecb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x48cea3; continue; }
  pc = 0x48ced1; continue;
 }
 0x48ced1 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48cef5; continue; }
  pc = 0x48ced7; continue;
 }
 0x48ced7 => {
  { let v = 0x4a16a0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x37u32; cpu.push(v); }
  { let v = 0x4a16c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48ceec); f_00490280(cpu); if cpu.returned_to != 0x48ceec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48cef5; continue; }
  pc = 0x48cef4; continue;
 }
 0x48cef4 => {
  cpu.unimplemented(0x48cef4, "Int3");
  pc = 0x48cef5; continue;
 }
 0x48cef5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x48ced1; continue; }
  pc = 0x48cefb; continue;
 }
 0x48cefb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48cf23; continue; }
  pc = 0x48cf05; continue;
 }
 0x48cf05 => {
  { let v = 0x4a168cu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x38u32; cpu.push(v); }
  { let v = 0x4a16c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48cf1a); f_00490280(cpu); if cpu.returned_to != 0x48cf1a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48cf23; continue; }
  pc = 0x48cf22; continue;
 }
 0x48cf22 => {
  cpu.unimplemented(0x48cf22, "Int3");
  pc = 0x48cf23; continue;
 }
 0x48cf23 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x48cefb; continue; }
  pc = 0x48cf29; continue;
 }
 0x48cf29 => {
  cpu.push(0x48cf2e); f_00490cb0(cpu); if cpu.returned_to != 0x48cf2e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48cf3b; continue; }
  pc = 0x48cf37; continue;
 }
 0x48cf37 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48cf59; continue;
 }
 0x48cf3b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x48cf50); f_00490920(cpu); if cpu.returned_to != 0x48cf50 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48cf59; continue;
 }
 0x48cf59 => {
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
pub fn f_0048cf60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cf60); }
 let mut pc: u32 = 0x48cf60;
 loop { match pc {
 0x48cf60 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48cf72); f_0048ce70(cpu); if cpu.returned_to != 0x48cf72 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cf80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cf80); }
 let mut pc: u32 = 0x48cf80;
 loop { match pc {
 0x48cf80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48cf95; continue; }
  pc = 0x48cf89; continue;
 }
 0x48cf89 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x48cf90); f_0048d0b0(cpu); if cpu.returned_to != 0x48cf90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48cfd1; continue;
 }
 0x48cf95 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48cf9e); f_0048cfe0(cpu); if cpu.returned_to != 0x48cf9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48cfaa; continue; }
  pc = 0x48cfa5; continue;
 }
 0x48cfa5 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x48cfd1; continue;
 }
 0x48cfaa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4000u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48cfcf; continue; }
  pc = 0x48cfba; continue;
 }
 0x48cfba => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48cfc6); f_00490da0(cpu); if cpu.returned_to != 0x48cfc6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48cfd1; continue;
 }
 0x48cfcf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48cfd1; continue;
 }
 0x48cfd1 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048cfe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48cfe0); }
 let mut pc: u32 = 0x48cfe0;
 loop { match pc {
 0x48cfe0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x48d07b; continue; }
  pc = 0x48d001; continue;
 }
 0x48d001 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x108u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d07b; continue; }
  pc = 0x48d011; continue;
 }
 0x48d011 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x48d07b; continue; }
  pc = 0x48d025; continue;
 }
 0x48d025 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d03c); f_00490e50(cpu); if cpu.returned_to != 0x48d03c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x48d065; continue; }
  pc = 0x48d044; continue;
 }
 0x48d044 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48d063; continue; }
  pc = 0x48d054; continue;
 }
 0x48d054 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xfffffffdu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x48d063; continue;
 }
 0x48d063 => {
  pc = 0x48d07b; continue;
 }
 0x48d065 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d07b; continue;
 }
 0x48d07b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d0a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d0a0); }
 let mut pc: u32 = 0x48d0a0;
 loop { match pc {
 0x48d0a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x48d0aa); f_0048d0b0(cpu); if cpu.returned_to != 0x48d0aa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d0b0); }
 let mut pc: u32 = 0x48d0b0;
 loop { match pc {
 0x48d0b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d0d6; continue;
 }
 0x48d0cd => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d0d6; continue;
 }
 0x48d0d6 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5f3080u32)); }
  if (cpu.sf == cpu.of) { pc = 0x48d17c; continue; }
  pc = 0x48d0e5; continue;
 }
 0x48d0e5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x48d177; continue; }
  pc = 0x48d0f7; continue;
 }
 0x48d0f7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x83u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d177; continue; }
  pc = 0x48d110; continue;
 }
 0x48d110 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x48d13a; continue; }
  pc = 0x48d116; continue;
 }
 0x48d116 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d127); f_0048cf80(cpu); if cpu.returned_to != 0x48d127 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x48d138; continue; }
  pc = 0x48d12f; continue;
 }
 0x48d12f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d138; continue;
 }
 0x48d138 => {
  pc = 0x48d177; continue;
 }
 0x48d13a => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d177; continue; }
  pc = 0x48d140; continue;
 }
 0x48d140 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48d177; continue; }
  pc = 0x48d156; continue;
 }
 0x48d156 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d168); f_0048cf80(cpu); if cpu.returned_to != 0x48d168 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x48d177; continue; }
  pc = 0x48d170; continue;
 }
 0x48d170 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d177; continue;
 }
 0x48d177 => {
  cpu.poll(); pc = 0x48d0cd; continue;
 }
 0x48d17c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x48d187; continue; }
  pc = 0x48d182; continue;
 }
 0x48d182 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48d18a; continue;
 }
 0x48d187 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48d18a; continue;
 }
 0x48d18a => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d190); }
 let mut pc: u32 = 0x48d190;
 loop { match pc {
 0x48d190 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x40u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48d1c5; continue; }
  pc = 0x48d1b3; continue;
 }
 0x48d1b3 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x48d266; continue;
 }
 0x48d1c5 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d1e9; continue; }
  pc = 0x48d1cb; continue;
 }
 0x48d1cb => {
  { let v = 0x4a1680u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x77u32; cpu.push(v); }
  { let v = 0x4a16dcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48d1e0); f_00490280(cpu); if cpu.returned_to != 0x48d1e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48d1e9; continue; }
  pc = 0x48d1e8; continue;
 }
 0x48d1e8 => {
  cpu.unimplemented(0x48d1e8, "Int3");
  pc = 0x48d1e9; continue;
 }
 0x48d1e9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x48d1c5; continue; }
  pc = 0x48d1ef; continue;
 }
 0x48d1ef => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x83u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48d259; continue; }
  pc = 0x48d1fe; continue;
 }
 0x48d1fe => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d207); f_0048cfe0(cpu); if cpu.returned_to != 0x48d207 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x48d216); f_004911f0(cpu); if cpu.returned_to != 0x48d216 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d225); f_00491100(cpu); if cpu.returned_to != 0x48d225 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x48d235; continue; }
  pc = 0x48d22c; continue;
 }
 0x48d22c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d259; continue;
 }
 0x48d235 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x48d259; continue; }
  pc = 0x48d23e; continue;
 }
 0x48d23e => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d24c); f_0048ecf0(cpu); if cpu.returned_to != 0x48d24c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  pc = 0x48d259; continue;
 }
 0x48d259 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48d266; continue;
 }
 0x48d266 => {
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
pub fn f_0048d270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d270); }
 let mut pc: u32 = 0x48d270;
 loop { match pc {
 0x48d270 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d299; continue; }
  pc = 0x48d292; continue;
 }
 0x48d292 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48d43b; continue;
 }
 0x48d299 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10cu32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d2b4; continue; }
  pc = 0x48d2a9; continue;
 }
 0x48d2a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d2bb; continue;
 }
 0x48d2b4 => {
  { let v: u32 = 0x1000u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d2bb; continue;
 }
 0x48d2bb => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48d438; continue; }
  pc = 0x48d2c5; continue;
 }
 0x48d2c5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10cu32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48d349; continue; }
  pc = 0x48d2d5; continue;
 }
 0x48d2d5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x48d349; continue; }
  pc = 0x48d2de; continue;
 }
 0x48d2de => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.ecx))); }
  if !cpu.cf { pc = 0x48d2f1; continue; }
  pc = 0x48d2e9; continue;
 }
 0x48d2e9 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d2fa; continue;
 }
 0x48d2f1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d2fa; continue;
 }
 0x48d2fa => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d313); f_004918d0(cpu); if cpu.returned_to != 0x48d313 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d433; continue;
 }
 0x48d349 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); }
  if cpu.cf { pc = 0x48d3ee; continue; }
  pc = 0x48d355; continue;
 }
 0x48d355 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48d36d; continue; }
  pc = 0x48d35b; continue;
 }
 0x48d35b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d373; continue;
 }
 0x48d36d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d373; continue;
 }
 0x48d373 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d38d); f_00491470(cpu); if cpu.returned_to != 0x48d38d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d3b8; continue; }
  pc = 0x48d399; continue;
 }
 0x48d399 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  pc = 0x48d43b; continue;
 }
 0x48d3b8 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x48d3da; continue; }
  pc = 0x48d3be; continue;
 }
 0x48d3be => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  pc = 0x48d43b; continue;
 }
 0x48d3da => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d433; continue;
 }
 0x48d3ee => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d3f7); f_00491290(cpu); if cpu.returned_to != 0x48d3f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x48d410; continue; }
  pc = 0x48d403; continue;
 }
 0x48d403 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  pc = 0x48d43b; continue;
 }
 0x48d410 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d433; continue;
 }
 0x48d433 => {
  cpu.poll(); pc = 0x48d2bb; continue;
 }
 0x48d438 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48d43b; continue;
 }
 0x48d43b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d440); }
 let mut pc: u32 = 0x48d440;
 loop { match pc {
 0x48d440 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5f3080u32), 0x0u32); }
  if !cpu.zf { pc = 0x48d459; continue; }
  pc = 0x48d44d; continue;
 }
 0x48d44d => {
  { let v: u32 = 0x200u32; cpu.w32(0x5f3080u32, v); }
  pc = 0x48d46c; continue;
 }
 0x48d459 => {
  { cpu.sub32(cpu.r32(0x5f3080u32), 0x14u32); }
  if (cpu.sf == cpu.of) { pc = 0x48d46c; continue; }
  pc = 0x48d462; continue;
 }
 0x48d462 => {
  { let v: u32 = 0x14u32; cpu.w32(0x5f3080u32, v); }
  pc = 0x48d46c; continue;
 }
 0x48d46c => {
  { let v = 0x84u32; cpu.push(v); }
  { let v = 0x4a16e8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5f3080u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d485); f_0048e720(cpu); if cpu.returned_to != 0x48d485 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5f2078u32, v); }
  { cpu.sub32(cpu.r32(0x5f2078u32), 0x0u32); }
  if !cpu.zf { pc = 0x48d4d5; continue; }
  pc = 0x48d496; continue;
 }
 0x48d496 => {
  { let v: u32 = 0x14u32; cpu.w32(0x5f3080u32, v); }
  { let v = 0x87u32; cpu.push(v); }
  { let v = 0x4a16e8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5f3080u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d4ba); f_0048e720(cpu); if cpu.returned_to != 0x48d4ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5f2078u32, v); }
  { cpu.sub32(cpu.r32(0x5f2078u32), 0x0u32); }
  if !cpu.zf { pc = 0x48d4d5; continue; }
  pc = 0x48d4cb; continue;
 }
 0x48d4cb => {
  { let v = 0x1au32; cpu.push(v); }
  cpu.push(0x48d4d2); f_0048ff90(cpu); if cpu.returned_to != 0x48d4d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48d4d5; continue;
 }
 0x48d4d5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d4e7; continue;
 }
 0x48d4de => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d4e7; continue;
 }
 0x48d4e7 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if (cpu.sf == cpu.of) { pc = 0x48d506; continue; }
  pc = 0x48d4ed; continue;
 }
 0x48d4ed => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5daa00u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f2078u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x48d4de; continue;
 }
 0x48d506 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d518; continue;
 }
 0x48d50f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d518; continue;
 }
 0x48d518 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (cpu.sf == cpu.of) { pc = 0x48d562; continue; }
  pc = 0x48d51e; continue;
 }
 0x48d51e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(8))), 0xffffffffu32); }
  if cpu.zf { pc = 0x48d550; continue; }
  pc = 0x48d537; continue;
 }
 0x48d537 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(8))), 0x0u32); }
  if !cpu.zf { pc = 0x48d560; continue; }
  pc = 0x48d550; continue;
 }
 0x48d550 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5daa10u32.wrapping_add(cpu.ecx), v); }
  pc = 0x48d560; continue;
 }
 0x48d560 => {
  cpu.poll(); pc = 0x48d50f; continue;
 }
 0x48d562 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d570); }
 let mut pc: u32 = 0x48d570;
 loop { match pc {
 0x48d570 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x48d578); f_0048d0a0(cpu); if cpu.returned_to != 0x48d578 { return; }
  { let v: u32 = cpu.r8(0x5de9fcu32) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48d588; continue; }
  pc = 0x48d583; continue;
 }
 0x48d583 => {
  cpu.push(0x48d588); f_00491f20(cpu); if cpu.returned_to != 0x48d588 { return; }
  pc = 0x48d588; continue;
 }
 0x48d588 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048d590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d590); }
 let mut pc: u32 = 0x48d590;
 loop { match pc {
 0x48d590 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d59f; continue;
 }
 0x48d59f => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d5c3; continue; }
  pc = 0x48d5a5; continue;
 }
 0x48d5a5 => {
  { let v = 0x4a16fcu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x36u32; cpu.push(v); }
  { let v = 0x4a16f0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48d5ba); f_00490280(cpu); if cpu.returned_to != 0x48d5ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48d5c3; continue; }
  pc = 0x48d5c2; continue;
 }
 0x48d5c2 => {
  cpu.unimplemented(0x48d5c2, "Int3");
  pc = 0x48d5c3; continue;
 }
 0x48d5c3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x48d59f; continue; }
  pc = 0x48d5c9; continue;
 }
 0x48d5c9 => {
  { let v = 0x5daa20u32; cpu.push(v); }
  cpu.push(0x48d5d3); f_00491fd0(cpu); if cpu.returned_to != 0x48d5d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5daa20u32; cpu.push(v); }
  cpu.push(0x48d5eb); f_004921f0(cpu); if cpu.returned_to != 0x48d5eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5daa20u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x48d5ff); f_00492130(cpu); if cpu.returned_to != 0x48d5ff { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_0048d610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48d610); }
 let mut pc: u32 = 0x48d610;
 loop { match pc {
 0x48d610 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x48d619; continue;
 }
 0x48d619 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d63d; continue; }
  pc = 0x48d61f; continue;
 }
 0x48d61f => {
  { let v = 0x4a1680u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x63u32; cpu.push(v); }
  { let v = 0x4a170cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x48d634); f_00490280(cpu); if cpu.returned_to != 0x48d634 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x48d63d; continue; }
  pc = 0x48d63c; continue;
 }
 0x48d63c => {
  cpu.unimplemented(0x48d63c, "Int3");
  pc = 0x48d63d; continue;
 }
 0x48d63d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x48d619; continue; }
  pc = 0x48d643; continue;
 }
 0x48d643 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x48d665; continue; }
  pc = 0x48d65b; continue;
 }
 0x48d65b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  pc = 0x48d665; continue;
 }
 0x48d665 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d672); f_0048fff0(cpu); if cpu.returned_to != 0x48d672 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x48d686; continue; }
  pc = 0x48d67e; continue;
 }
 0x48d67e => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x48d899; continue;
 }
 0x48d686 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x108u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x48d6a6; continue; }
  pc = 0x48d696; continue;
 }
 0x48d696 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x48d899; continue;
 }
 0x48d6a6 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48d719; continue; }
  pc = 0x48d6c1; continue;
 }
 0x48d6c1 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(8))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x80u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d717; continue; }
  pc = 0x48d6e3; continue;
 }
 0x48d6e3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d6f7; continue;
 }
 0x48d6ee => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d6f7; continue;
 }
 0x48d6f7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x48d717; continue; }
  pc = 0x48d701; continue;
 }
 0x48d701 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if !cpu.zf { pc = 0x48d715; continue; }
  pc = 0x48d70c; continue;
 }
 0x48d70c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d715; continue;
 }
 0x48d715 => {
  cpu.poll(); pc = 0x48d6ee; continue;
 }
 0x48d717 => {
  pc = 0x48d73b; continue;
 }
 0x48d719 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x48d73b; continue; }
  pc = 0x48d729; continue;
 }
 0x48d729 => {
  { let v: u32 = 0x16u32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x48d899; continue;
 }
 0x48d73b => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48d749; continue; }
  pc = 0x48d741; continue;
 }
 0x48d741 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48d899; continue;
 }
 0x48d749 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d893; continue; }
  pc = 0x48d75a; continue;
 }
 0x48d75a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x48d76f; continue; }
  pc = 0x48d763; continue;
 }
 0x48d763 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d893; continue;
 }
 0x48d76f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ecx))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(8))) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48d88a; continue; }
  pc = 0x48d7ab; continue;
 }
 0x48d7ab => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48d7b8); f_0048fff0(cpu); if cpu.returned_to != 0x48d7b8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x48d819; continue; }
  pc = 0x48d7c0; continue;
 }
 0x48d7c0 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d7e0; continue;
 }
 0x48d7d7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d7e0; continue;
 }
 0x48d7e0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x48d7fe; continue; }
  pc = 0x48d7e8; continue;
 }
 0x48d7e8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if !cpu.zf { pc = 0x48d7fc; continue; }
  pc = 0x48d7f3; continue;
 }
 0x48d7f3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d7fc; continue;
 }
 0x48d7fc => {
  cpu.poll(); pc = 0x48d7d7; continue;
 }
 0x48d7fe => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x2000u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x48d817; continue; }
  pc = 0x48d80e; continue;
 }
 0x48d80e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d817; continue;
 }
 0x48d817 => {
  pc = 0x48d88a; continue;
 }
 0x48d819 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x48d828); f_0048fff0(cpu); if cpu.returned_to != 0x48d828 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x200u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x48d859; continue; }
  pc = 0x48d834; continue;
 }
 0x48d834 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x48d859; continue; }
  pc = 0x48d841; continue;
 }
 0x48d841 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x400u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48d859; continue; }
  pc = 0x48d850; continue;
 }
 0x48d850 => {
  { let v: u32 = 0x200u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d862; continue;
 }
 0x48d859 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d862; continue;
 }
 0x48d862 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8))) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x48d88a; continue; }
  pc = 0x48d881; continue;
 }
 0x48d881 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d88a; continue;
 }
 0x48d88a => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48d893; continue;
 }
 0x48d893 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  pc = 0x48d899; continue;
 }
 0x48d899 => {
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
