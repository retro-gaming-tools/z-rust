#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0042ccd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ccd8); }
 let mut pc: u32 = 0x42ccd8;
 loop { match pc {
 0x42ccd8 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x42cd28; continue; }
  pc = 0x42cce4; continue;
 }
 0x42cce4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c08u32); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x42cd0c; continue; }
  pc = 0x42ccfc; continue;
 }
 0x42ccfc => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x42cd0c; continue; }
  pc = 0x42cd03; continue;
 }
 0x42cd03 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c838u32, v); }
  pc = 0x42cd13; continue;
 }
 0x42cd0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c838u32, v); }
  pc = 0x42cd13; continue;
 }
 0x42cd13 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x42cd19); f_0042cd2e(cpu); if cpu.returned_to != 0x42cd19 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  pc = 0x42cd2d; continue;
 }
 0x42cd28 => {
  cpu.push(0x42cd2d); f_0042cd2e(cpu); if cpu.returned_to != 0x42cd2d { return; }
  pc = 0x42cd2d; continue;
 }
 0x42cd2d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cd2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cd2e); }
 let mut pc: u32 = 0x42cd2e;
 loop { match pc {
 0x42cd2e => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42cd38; continue; }
  pc = 0x42cd37; continue;
 }
 0x42cd37 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42cd38 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x42cd73; continue; }
  pc = 0x42cd41; continue;
 }
 0x42cd41 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x42cd73; continue; }
  pc = 0x42cd4a; continue;
 }
 0x42cd4a => {
  { let v: u32 = cpu.r32(0x4d6c10u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x42cd73; continue; }
  pc = 0x42cd54; continue;
 }
 0x42cd54 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x42cd73; continue; }
  pc = 0x42cd5a; continue;
 }
 0x42cd5a => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x52bc03u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42cd73; continue; }
  pc = 0x42cd6d; continue;
 }
 0x42cd6d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x42cd73); f_0041a8a2(cpu); if cpu.returned_to != 0x42cd73 { return; }
  pc = 0x42cd73; continue;
 }
 0x42cd73 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cdbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cdbd); }
 let mut pc: u32 = 0x42cdbd;
 loop { match pc {
 0x42cdbd => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42ce09; continue; }
  pc = 0x42cdc6; continue;
 }
 0x42cdc6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x59da88u32)); cpu.w32(0x59da88u32, v); }
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42ce09; continue; }
  pc = 0x42cdd6; continue;
 }
 0x42cdd6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c464u32, v); }
  { cpu.sub32(cpu.r32(0x59da88u32), 0x22u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x42cded; continue; }
  pc = 0x42cde6; continue;
 }
 0x42cde6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c464u32, v); }
  pc = 0x42cded; continue;
 }
 0x42cded => {
  cpu.push(0x42cdf2); f_004184ca(cpu); if cpu.returned_to != 0x42cdf2 { return; }
  { let v = cpu.r32(0x59ee15u32); cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x59ee15u32, v); }
  cpu.push(0x42ce03); f_0042ce0a(cpu); if cpu.returned_to != 0x42ce03 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x59ee15u32, v); }
  pc = 0x42ce09; continue;
 }
 0x42ce09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ce0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ce0a); }
 let mut pc: u32 = 0x42ce0a;
 loop { match pc {
 0x42ce0a => {
  { cpu.sub32(cpu.r32(0x59ee15u32), 0x0u32); }
  if cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce17; continue;
 }
 0x42ce17 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce24; continue;
 }
 0x42ce24 => {
  { let v: u32 = cpu.r32(0x59ee15u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce35; continue;
 }
 0x42ce35 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce40; continue;
 }
 0x42ce40 => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x42ce60; continue; }
  pc = 0x42ce50; continue;
 }
 0x42ce50 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce5e; continue;
 }
 0x42ce5e => {
  pc = 0x42ce60; continue;
 }
 0x42ce60 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x42cff6; continue; }
  pc = 0x42ce7a; continue;
 }
 0x42ce7a => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if !cpu.zf { pc = 0x42ceb2; continue; }
  pc = 0x42ce9e; continue;
 }
 0x42ce9e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x4d42u16); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42cea5; continue;
 }
 0x42cea5 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x54u8); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42ceab; continue;
 }
 0x42ceab => {
  { let v: u32 = cpu.r32(0x5ae0b0u32); cpu.eax = v; }
  pc = 0x42cee1; continue;
 }
 0x42ceb2 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if !cpu.zf { pc = 0x42cecf; continue; }
  pc = 0x42cebb; continue;
 }
 0x42cebb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x5456u16); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42cec2; continue;
 }
 0x42cec2 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x42u8); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42cec8; continue;
 }
 0x42cec8 => {
  { let v: u32 = cpu.r32(0x5ae0b0u32); cpu.eax = v; }
  pc = 0x42cee1; continue;
 }
 0x42cecf => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x5041u16); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42ced6; continue;
 }
 0x42ced6 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x43u8); }
  if !cpu.zf { pc = 0x42cee1; continue; }
  pc = 0x42cedc; continue;
 }
 0x42cedc => {
  { let v: u32 = cpu.r32(0x5ae0b0u32); cpu.eax = v; }
  pc = 0x42cee1; continue;
 }
 0x42cee1 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x42ceff; continue; }
  pc = 0x42cee9; continue;
 }
 0x42cee9 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.eax)), 0x7265u16); }
  if !cpu.zf { pc = 0x42ceff; continue; }
  pc = 0x42cef1; continue;
 }
 0x42cef1 => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  pc = 0x42ceff; continue;
 }
 0x42ceff => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xfffffff6u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff5u32; cpu.ebx = v; }
  { let v: u32 = 0xeu32; cpu.w32(0x59da84u32, v); }
  { let v: u32 = 0x2cfu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42cf3c; continue; }
  pc = 0x42cf21; continue;
 }
 0x42cf21 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x42cf3c; continue; }
  pc = 0x42cf28; continue;
 }
 0x42cf28 => {
  { let v: u32 = 0x2cfu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  { let v: u32 = 0x12u32; cpu.w32(0x59da84u32, v); }
  pc = 0x42cf3c; continue;
 }
 0x42cf3c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x43u32.wrapping_add(cpu.edi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x42cf68); f_0042d0e4(cpu); if cpu.returned_to != 0x42cf68 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x42cf7c); f_0041cdf8(cpu); if cpu.returned_to != 0x42cf7c { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x59da84u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58c446u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x58c448u32, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x6u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c464u32), 0x0u8); }
  if cpu.zf { pc = 0x42cfb1; continue; }
  pc = 0x42cfac; continue;
 }
 0x42cfac => {
  cpu.push(0x42cfb1); f_0041cc22(cpu); if cpu.returned_to != 0x42cfb1 { return; }
  pc = 0x42cfb1; continue;
 }
 0x42cfb1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2cfu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c464u32), 0x0u8); }
  if cpu.zf { pc = 0x42cfe8; continue; }
  pc = 0x42cfe2; continue;
 }
 0x42cfe2 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42cfe8); cpu.call(t); if cpu.returned_to != 0x42cfe8 { return; }
  pc = 0x42cfe8; continue;
 }
 0x42cfe8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.add16(cpu.r16(0x58c448u32), 0x3u16); cpu.w16(0x58c448u32, v); }
  cpu.push(0x42cff6); f_0042cff7(cpu); if cpu.returned_to != 0x42cff6 { return; }
  pc = 0x42cff6; continue;
 }
 0x42cff6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042cff7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42cff7); }
 let mut pc: u32 = 0x42cff7;
 loop { match pc {
 0x42cff7 => {
  { let v: u32 = 0xb8u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x42d0e3; continue; }
  pc = 0x42d00c; continue;
 }
 0x42d00c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5cc3f2u32, v); }
  { let v: u32 = 0x59f458u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.push(0x42d03f); f_00440914(cpu); if cpu.returned_to != 0x42d03f { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d67e4u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4d67e6u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x4d67e8u32) as u32; cpu.edx = v; }
  cpu.push(0x42d05e); f_004672ca(cpu); if cpu.returned_to != 0x42d05e { return; }
  { let v: u32 = cpu.r16(0x4d68f2u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68f2u32, v); }
  { let v: u32 = cpu.r8(0x58e95cu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x42d08c); f_004298e5(cpu); if cpu.returned_to != 0x42d08c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x58c446u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x58c448u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x42d0a8); cpu.call(t); if cpu.returned_to != 0x42d0a8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68f2u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x59f458u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  pc = 0x42d0e3; continue;
 }
 0x42d0e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042d0e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42d0e4); }
 let mut pc: u32 = 0x42d0e4;
 loop { match pc {
 0x42d0e4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42d105; continue; }
  pc = 0x42d0ef; continue;
 }
 0x42d0ef => {
  { let v: u8 = cpu.r8(0x5b9ff4u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x59ddabu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x59ddadu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42d105; continue;
 }
 0x42d105 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042d106(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42d106); }
 let mut pc: u32 = 0x42d106;
 loop { match pc {
 0x42d106 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42d194; continue; }
  pc = 0x42d113; continue;
 }
 0x42d113 => {
  { cpu.sub32(cpu.r32(0x59ee19u32), 0x0u32); }
  if cpu.zf { pc = 0x42d194; continue; }
  pc = 0x42d120; continue;
 }
 0x42d120 => {
  { let v: u32 = cpu.r32(0x59ee19u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x20u8); }
  if cpu.zf { pc = 0x42d194; continue; }
  pc = 0x42d130; continue;
 }
 0x42d130 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = 0xfffffff6u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff5u32; cpu.ebx = v; }
  { let v: u32 = 0x2cfu32; cpu.ecx = v; }
  { let v: u32 = 0x1cu32; cpu.w32(0x59da84u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x42d178); f_0041cdf8(cpu); if cpu.returned_to != 0x42d178 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x59da84u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x42d194); f_0041cc22(cpu); if cpu.returned_to != 0x42d194 { return; }
  pc = 0x42d194; continue;
 }
 0x42d194 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042d195(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42d195); }
 let mut pc: u32 = 0x42d195;
 loop { match pc {
 0x42d195 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c441u32, v); }
  { cpu.sub8(cpu.r8(0x5791c8u32), 0x0u8); }
  if cpu.zf { pc = 0x42d1ad; continue; }
  pc = 0x42d1a5; continue;
 }
 0x42d1a5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d22au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42d1ad => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6c88u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6c8cu32, v); }
  cpu.push(0x42d1c6); f_00418beb(cpu); if cpu.returned_to != 0x42d1c6 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d693eu32, v); }
  cpu.push(0x42d1d8); f_00418ba3(cpu); if cpu.returned_to != 0x42d1d8 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x59da9bu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x59da9fu32, v); }
  { let v: u16 = cpu.r16(0x4d693cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x42d204; continue; }
  pc = 0x42d1f2; continue;
 }
 0x42d1f2 => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x42d204; continue; }
  pc = 0x42d202; continue;
 }
 0x42d202 => {
  pc = 0x42d243; continue;
 }
 0x42d204 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x2u32); }
  if cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d214; continue;
 }
 0x42d214 => {
  cpu.push(0x42d219); f_00418237(cpu); if cpu.returned_to != 0x42d219 { return; }
  if cpu.cf { pc = 0x42de03; continue; }
  pc = 0x42d21f; continue;
 }
 0x42d21f => {
  { cpu.sub8(cpu.r8(0x58c552u32), 0x0u8); }
  if !cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d22c; continue;
 }
 0x42d22c => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c441u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x59da9bu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x59da9fu32, v); }
  pc = 0x42d308; continue;
 }
 0x42d243 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x42d25c; continue; }
  pc = 0x42d24f; continue;
 }
 0x42d24f => {
  { cpu.pushad(); }
  cpu.push(0x42d255); f_0044e68e(cpu); if cpu.returned_to != 0x42d255 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x42de03; continue; }
  pc = 0x42d25c; continue;
 }
 0x42d25c => {
  cpu.push(0x42d261); f_00434ac7(cpu); if cpu.returned_to != 0x42d261 { return; }
  { cpu.sub8(cpu.r8(0x58c634u32), 0x0u8); }
  if !cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d26e; continue;
 }
 0x42d26e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x0u32); }
  if cpu.zf { pc = 0x42d306; continue; }
  pc = 0x42d27d; continue;
 }
 0x42d27d => {
  { let v: u16 = cpu.r16(0x4d693cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0xe8u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x11u16; cpu.w16(0x5b3865u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x42d2db; continue; }
  pc = 0x42d2b7; continue;
 }
 0x42d2b7 => {
  { let v: u16 = 0xe0u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0x188u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x11u16; cpu.w16(0x5b3865u32, v); }
  pc = 0x42d2db; continue;
 }
 0x42d2db => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b385fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x42d306; continue; }
  pc = 0x42d2e4; continue;
 }
 0x42d2e4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b3861u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42d306; continue; }
  pc = 0x42d2ed; continue;
 }
 0x42d2ed => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3863u32)); }
  if (cpu.sf != cpu.of) { pc = 0x42d306; continue; }
  pc = 0x42d2f6; continue;
 }
 0x42d2f6 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3865u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42d306; continue; }
  pc = 0x42d2ff; continue;
 }
 0x42d2ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42de03; continue;
 }
 0x42d306 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42d308; continue;
 }
 0x42d308 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x42d321; continue; }
  pc = 0x42d314; continue;
 }
 0x42d314 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d321; continue;
 }
 0x42d321 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x42d406; continue; }
  pc = 0x42d331; continue;
 }
 0x42d331 => {
  { cpu.pushad(); }
  cpu.push(0x42d337); f_00403030(cpu); if cpu.returned_to != 0x42d337 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x42d406; continue; }
  pc = 0x42d340; continue;
 }
 0x42d340 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x42d358); f_00471211(cpu); if cpu.returned_to != 0x42d358 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  if !cpu.cf { pc = 0x42d404; continue; }
  pc = 0x42d368; continue;
 }
 0x42d368 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x42d3af; continue; }
  pc = 0x42d375; continue;
 }
 0x42d375 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x59ee11u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59ee15u32, v); }
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x42d3a1); f_0046d5b9(cpu); if cpu.returned_to != 0x42d3a1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x42d404; continue; }
  pc = 0x42d3af; continue;
 }
 0x42d3af => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42d404; continue; }
  pc = 0x42d3b6; continue;
 }
 0x42d3b6 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x42d3f6; continue; }
  pc = 0x42d3c5; continue;
 }
 0x42d3c5 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42d404; continue; }
  pc = 0x42d3ce; continue;
 }
 0x42d3ce => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42d404; continue; }
  pc = 0x42d3d2; continue;
 }
 0x42d3d2 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x42d3ef; continue; }
  pc = 0x42d3ed; continue;
 }
 0x42d3ed => {
  pc = 0x42d404; continue;
 }
 0x42d3ef => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x42dd59; continue;
 }
 0x42d3f6 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x42d3ef; continue; }
  pc = 0x42d3fd; continue;
 }
 0x42d3fd => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x42de03; continue;
 }
 0x42d404 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x42d406; continue;
 }
 0x42d406 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59ee19u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59da90u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x59da92u32, v); }
  cpu.push(0x42d432); f_004716d0(cpu); if cpu.returned_to != 0x42d432 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x42d44a; continue; }
  pc = 0x42d436; continue;
 }
 0x42d436 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1472u16); }
  if cpu.zf { pc = 0x42d44a; continue; }
  pc = 0x42d43d; continue;
 }
 0x42d43d => {
  { let v: u32 = cpu.esi; cpu.w32(0x59ee19u32, v); }
  { let v: u8 = 0x32u8; cpu.w8(0x59ee1du32, v); }
  pc = 0x42d44a; continue;
 }
 0x42d44a => {
  { let v: u32 = 0x0u32; cpu.w32(0x59ee11u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59ee15u32, v); }
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x42d475); f_0046d5b9(cpu); if cpu.returned_to != 0x42d475 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  if cpu.cf { pc = 0x42d55c; continue; }
  pc = 0x42d482; continue;
 }
 0x42d482 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x59ee11u32, v); }
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x42d55c; continue; }
  pc = 0x42d496; continue;
 }
 0x42d496 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x42d55c; continue; }
  pc = 0x42d4a1; continue;
 }
 0x42d4a1 => {
  cpu.push(0x42d4a6); f_0041a3a3(cpu); if cpu.returned_to != 0x42d4a6 { return; }
  if cpu.cf { pc = 0x42d55c; continue; }
  pc = 0x42d4ac; continue;
 }
 0x42d4ac => {
  { let v: u32 = cpu.ebp; cpu.w32(0x59ee11u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x59ee15u32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42d55c; continue; }
  pc = 0x42d4ca; continue;
 }
 0x42d4ca => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d4d8; continue;
 }
 0x42d4d8 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  cpu.push(0x42d4df); f_004184ca(cpu); if cpu.returned_to != 0x42d4df { return; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { pc = 0x42d54b; continue; }
  pc = 0x42d4e7; continue;
 }
 0x42d4e7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x42d4f0; continue; }
  pc = 0x42d4ee; continue;
 }
 0x42d4ee => {
  pc = 0x42d517; continue;
 }
 0x42d4f0 => {
  cpu.push(0x42d4f5); f_0045c359(cpu); if cpu.returned_to != 0x42d4f5 { return; }
  if cpu.cf { pc = 0x42ddd7; continue; }
  pc = 0x42d4fb; continue;
 }
 0x42d4fb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42d50b); f_004716d0(cpu); if cpu.returned_to != 0x42d50b { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x42de2f; continue; }
  pc = 0x42d512; continue;
 }
 0x42d512 => {
  pc = 0x42dd59; continue;
 }
 0x42d517 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x42ddd7; continue; }
  pc = 0x42d522; continue;
 }
 0x42d522 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x42ddd7; continue; }
  pc = 0x42d536; continue;
 }
 0x42d536 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x42de2f; continue; }
  pc = 0x42d546; continue;
 }
 0x42d546 => {
  pc = 0x42ddd7; continue;
 }
 0x42d54b => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x42ddd7; continue; }
  pc = 0x42d55c; continue;
 }
 0x42d55c => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42d56a; continue;
 }
 0x42d56a => {
  cpu.push(0x42d56f); f_004184ca(cpu); if cpu.returned_to != 0x42d56f { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x42d696; continue; }
  pc = 0x42d57a; continue;
 }
 0x42d57a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d585; continue;
 }
 0x42d585 => {
  cpu.push(0x42d58a); f_0046d525(cpu); if cpu.returned_to != 0x42d58a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x42d5af; continue; }
  pc = 0x42d58e; continue;
 }
 0x42d58e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x42d7c9; continue; }
  pc = 0x42d599; continue;
 }
 0x42d599 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d5a4; continue;
 }
 0x42d5a4 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d5af; continue;
 }
 0x42d5af => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x42d696; continue; }
  pc = 0x42d5c5; continue;
 }
 0x42d5c5 => {
  cpu.push(0x42d5ca); f_004302f6(cpu); if cpu.returned_to != 0x42d5ca { return; }
  if cpu.cf { pc = 0x42dec2; continue; }
  pc = 0x42d5d0; continue;
 }
 0x42d5d0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42d5e2); f_004713ce(cpu); if cpu.returned_to != 0x42d5e2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x42d604; continue; }
  pc = 0x42d5e6; continue;
 }
 0x42d5e6 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d5ff; continue;
 }
 0x42d5ff => {
  pc = 0x42e06c; continue;
 }
 0x42d604 => {
  cpu.push(0x42d609); f_0046e362(cpu); if cpu.returned_to != 0x42d609 { return; }
  if !cpu.cf { pc = 0x42d633; continue; }
  pc = 0x42d60b; continue;
 }
 0x42d60b => {
  { let v: u32 = cpu.esi; cpu.w32(0x576c00u32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42d621); f_0046e362(cpu); if cpu.returned_to != 0x42d621 { return; }
  { cpu.sub32(cpu.r32(0x576c00u32), cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d62e; continue;
 }
 0x42d62e => {
  pc = 0x42e06c; continue;
 }
 0x42d633 => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42d643); f_00470fc5(cpu); if cpu.returned_to != 0x42d643 { return; }
  if !cpu.cf { pc = 0x42d679; continue; }
  pc = 0x42d645; continue;
 }
 0x42d645 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d651; continue;
 }
 0x42d651 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x42d660; continue; }
  pc = 0x42d65b; continue;
 }
 0x42d65b => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x42d665; continue; }
  pc = 0x42d660; continue;
 }
 0x42d660 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x42d665; continue;
 }
 0x42d665 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d674; continue;
 }
 0x42d674 => {
  pc = 0x42e06c; continue;
 }
 0x42d679 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42d68b); f_0041a69f(cpu); if cpu.returned_to != 0x42d68b { return; }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d691; continue;
 }
 0x42d691 => {
  pc = 0x42def3; continue;
 }
 0x42d696 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x42d731; continue; }
  pc = 0x42d6a1; continue;
 }
 0x42d6a1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x42d731; continue; }
  pc = 0x42d6ac; continue;
 }
 0x42d6ac => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42d6bc); f_004712e3(cpu); if cpu.returned_to != 0x42d6bc { return; }
  if !cpu.cf { pc = 0x42d6e8; continue; }
  pc = 0x42d6be; continue;
 }
 0x42d6be => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42d6e8; continue; }
  pc = 0x42d6d3; continue;
 }
 0x42d6d3 => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d6e3; continue;
 }
 0x42d6e3 => {
  pc = 0x42e09d; continue;
 }
 0x42d6e8 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x4u8; cpu.w8(0x4ea600u32, v); }
  cpu.push(0x42d701); f_00453e97(cpu); if cpu.returned_to != 0x42d701 { return; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea600u32, v); }
  if cpu.cf { pc = 0x42e09d; continue; }
  pc = 0x42d714; continue;
 }
 0x42d714 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42d726); f_0041a69f(cpu); if cpu.returned_to != 0x42d726 { return; }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42d72c; continue;
 }
 0x42d72c => {
  pc = 0x42def3; continue;
 }
 0x42d731 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42d738); f_004184ca(cpu); if cpu.returned_to != 0x42d738 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x42d7af; continue; }
  pc = 0x42d745; continue;
 }
 0x42d745 => {
  cpu.push(0x42d74a); f_004481e0(cpu); if cpu.returned_to != 0x42d74a { return; }
  if !cpu.cf { pc = 0x42d7af; continue; }
  pc = 0x42d750; continue;
 }
 0x42d750 => {
  { let v: u32 = cpu.r32(0x59da8cu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4b847du32); }
  if cpu.zf { pc = 0x42d789; continue; }
  pc = 0x42d75f; continue;
 }
 0x42d75f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4b8496u32); }
  if cpu.zf { pc = 0x42d789; continue; }
  pc = 0x42d768; continue;
 }
 0x42d768 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x42d7af; continue; }
  pc = 0x42d76f; continue;
 }
 0x42d76f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x42d7af; continue; }
  pc = 0x42d776; continue;
 }
 0x42d776 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x42d7af; continue; }
  pc = 0x42d77d; continue;
 }
 0x42d77d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42d7af; continue; }
  pc = 0x42d784; continue;
 }
 0x42d784 => {
  pc = 0x42d9e1; continue;
 }
 0x42d789 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x42d7a2; continue; }
  pc = 0x42d790; continue;
 }
 0x42d790 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x42d7a2; continue; }
  pc = 0x42d797; continue;
 }
 0x42d797 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x42d9e1; continue; }
  pc = 0x42d7a2; continue;
 }
 0x42d7a2 => {
  cpu.push(0x42d7a7); f_0046d525(cpu); if cpu.returned_to != 0x42d7a7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x42d9e1; continue; }
  pc = 0x42d7af; continue;
 }
 0x42d7af => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c72fu32, v); }
  cpu.push(0x42d7bb); f_0041a7e3(cpu); if cpu.returned_to != 0x42d7bb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c72fu32, v); }
  if !cpu.cf { pc = 0x42d7c9; continue; }
  pc = 0x42d7c4; continue;
 }
 0x42d7c4 => {
  pc = 0x42def3; continue;
 }
 0x42d7c9 => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42d7d9); f_00470fc5(cpu); if cpu.returned_to != 0x42d7d9 { return; }
  if !cpu.cf { pc = 0x42d86f; continue; }
  pc = 0x42d7df; continue;
 }
 0x42d7df => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42d864; continue; }
  pc = 0x42d7eb; continue;
 }
 0x42d7eb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x42d86f; continue; }
  pc = 0x42d7f6; continue;
 }
 0x42d7f6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x42d805; continue; }
  pc = 0x42d800; continue;
 }
 0x42d800 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x42d80a; continue; }
  pc = 0x42d805; continue;
 }
 0x42d805 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x42d80a; continue;
 }
 0x42d80a => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.zf { pc = 0x42e06c; continue; }
  pc = 0x42d819; continue;
 }
 0x42d819 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d824; continue;
 }
 0x42d824 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x42d844; continue; }
  pc = 0x42d82b; continue;
 }
 0x42d82b => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x42d844; continue; }
  pc = 0x42d835; continue;
 }
 0x42d835 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x42d844; continue; }
  pc = 0x42d83f; continue;
 }
 0x42d83f => {
  pc = 0x42db2a; continue;
 }
 0x42d844 => {
  { cpu.pushad(); }
  cpu.push(0x42d84a); f_00403030(cpu); if cpu.returned_to != 0x42d84a { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x42d85f; continue; }
  pc = 0x42d84f; continue;
 }
 0x42d84f => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if !cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d85f; continue;
 }
 0x42d85f => {
  pc = 0x42de60; continue;
 }
 0x42d864 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d86f; continue;
 }
 0x42d86f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x42d87a); f_0042e0fa(cpu); if cpu.returned_to != 0x42d87a { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42d888; continue;
 }
 0x42d888 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x42d893); f_0046e362(cpu); if cpu.returned_to != 0x42d893 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x42db2a; continue; }
  pc = 0x42d89f; continue;
 }
 0x42d89f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x42d8e0; continue; }
  pc = 0x42d8a6; continue;
 }
 0x42d8a6 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x581a0cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x42d8df; continue; }
  pc = 0x42d8b5; continue;
 }
 0x42d8b5 => {
  cpu.push(0x42d8ba); f_0046e31e(cpu); if cpu.returned_to != 0x42d8ba { return; }
  if !cpu.cf { pc = 0x42d8df; continue; }
  pc = 0x42d8bc; continue;
 }
 0x42d8bc => {
  cpu.push(0x42d8c1); f_0046d834(cpu); if cpu.returned_to != 0x42d8c1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x42d8d3); f_0046c3d4(cpu); if cpu.returned_to != 0x42d8d3 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x42dd59; continue; }
  pc = 0x42d8da; continue;
 }
 0x42d8da => {
  pc = 0x42de60; continue;
 }
 0x42d8df => {
  { cpu.popad(); }
  pc = 0x42d8e0; continue;
 }
 0x42d8e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da90u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da92u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42d990; continue; }
  pc = 0x42d905; continue;
 }
 0x42d905 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x42d990; continue; }
  pc = 0x42d912; continue;
 }
 0x42d912 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x1u8); }
  if cpu.zf { pc = 0x42d96d; continue; }
  pc = 0x42d91b; continue;
 }
 0x42d91b => {
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x2u8); }
  if cpu.zf { pc = 0x42d94a; continue; }
  pc = 0x42d924; continue;
 }
 0x42d924 => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0xffu8); }
  if cpu.zf { pc = 0x42d938; continue; }
  pc = 0x42d92d; continue;
 }
 0x42d92d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42d9e1; continue;
 }
 0x42d938 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42d9ae; continue;
 }
 0x42d94a => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0xffu8); }
  if cpu.zf { pc = 0x42d95e; continue; }
  pc = 0x42d953; continue;
 }
 0x42d953 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42d9e1; continue;
 }
 0x42d95e => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42d9ae; continue;
 }
 0x42d96d => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0xffu8); }
  if cpu.zf { pc = 0x42d981; continue; }
  pc = 0x42d976; continue;
 }
 0x42d976 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42d9e1; continue;
 }
 0x42d981 => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42d9ae; continue;
 }
 0x42d990 => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0xffu8); }
  if cpu.zf { pc = 0x42d9a1; continue; }
  pc = 0x42d999; continue;
 }
 0x42d999 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42d9e1; continue;
 }
 0x42d9a1 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42d9ae; continue;
 }
 0x42d9ae => {
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x42d9bf); f_004714a5(cpu); if cpu.returned_to != 0x42d9bf { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x42db2a; continue; }
  pc = 0x42d9cb; continue;
 }
 0x42d9cb => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42d9db); f_00456878(cpu); if cpu.returned_to != 0x42d9db { return; }
  if cpu.cf { pc = 0x42db2a; continue; }
  pc = 0x42d9e1; continue;
 }
 0x42d9e1 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x42da6d; continue; }
  pc = 0x42d9f1; continue;
 }
 0x42d9f1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x42da03); f_004712e3(cpu); if cpu.returned_to != 0x42da03 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x42da6d; continue; }
  pc = 0x42da16; continue;
 }
 0x42da16 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x42da57; continue; }
  pc = 0x42da2d; continue;
 }
 0x42da2d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42da6b; continue; }
  pc = 0x42da36; continue;
 }
 0x42da36 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42da6b; continue; }
  pc = 0x42da3a; continue;
 }
 0x42da3a => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x42da6b; continue; }
  pc = 0x42da55; continue;
 }
 0x42da55 => {
  pc = 0x42da64; continue;
 }
 0x42da57 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x42da5e); f_00403030(cpu); if cpu.returned_to != 0x42da5e { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x42da6b; continue; }
  pc = 0x42da64; continue;
 }
 0x42da64 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x42db2a; continue;
 }
 0x42da6b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x42da6d; continue;
 }
 0x42da6d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x42daf7; continue; }
  pc = 0x42da78; continue;
 }
 0x42da78 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x42dacf; continue; }
  pc = 0x42da7f; continue;
 }
 0x42da7f => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x42daf7; continue; }
  pc = 0x42da8d; continue;
 }
 0x42da8d => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42da9b; continue;
 }
 0x42da9b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x42daa6); f_0046e31e(cpu); if cpu.returned_to != 0x42daa6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x42dacf; continue; }
  pc = 0x42daae; continue;
 }
 0x42daae => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x42dac0); f_004712e3(cpu); if cpu.returned_to != 0x42dac0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x42db2a; continue; }
  pc = 0x42dacf; continue;
 }
 0x42dacf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x42db2a; continue; }
  pc = 0x42dad6; continue;
 }
 0x42dad6 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dae1; continue;
 }
 0x42dae1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42daec; continue;
 }
 0x42daec => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42daf7; continue;
 }
 0x42daf7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x42de60; continue; }
  pc = 0x42db02; continue;
 }
 0x42db02 => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x42db1f); f_0046c3d4(cpu); if cpu.returned_to != 0x42db1f { return; }
  if cpu.cf { pc = 0x42dd59; continue; }
  pc = 0x42db25; continue;
 }
 0x42db25 => {
  pc = 0x42de60; continue;
 }
 0x42db2a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x42dd29; continue; }
  pc = 0x42db35; continue;
 }
 0x42db35 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x42db4c; continue; }
  pc = 0x42db3c; continue;
 }
 0x42db3c => {
  cpu.push(0x42db41); f_004302f6(cpu); if cpu.returned_to != 0x42db41 { return; }
  if cpu.cf { pc = 0x42dec2; continue; }
  pc = 0x42db47; continue;
 }
 0x42db47 => {
  pc = 0x42dc21; continue;
 }
 0x42db4c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x42db5b; continue; }
  pc = 0x42db56; continue;
 }
 0x42db56 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x42db60; continue; }
  pc = 0x42db5b; continue;
 }
 0x42db5b => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x42db60; continue;
 }
 0x42db60 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4d6938u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.r32(0x59ee11u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42db8e; continue;
 }
 0x42db8e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dba0; continue;
 }
 0x42dba0 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42dbc8; continue; }
  pc = 0x42dba9; continue;
 }
 0x42dba9 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dbc8; continue;
 }
 0x42dbc8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x42de60; continue; }
  pc = 0x42dbd3; continue;
 }
 0x42dbd3 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6936u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d6938u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if (cpu.cf || cpu.zf) { pc = 0x42dc21; continue; }
  pc = 0x42dc1c; continue;
 }
 0x42dc1c => {
  pc = 0x42dc73; continue;
 }
 0x42dc21 => {
  { let v: u32 = cpu.r32(0x59ee11u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dc2b; continue;
 }
 0x42dc2b => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dc35; continue;
 }
 0x42dc35 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dc3c; continue;
 }
 0x42dc3c => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dc4a; continue;
 }
 0x42dc4a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42dc6e; continue; }
  pc = 0x42dc53; continue;
 }
 0x42dc53 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x42dc73; continue; }
  pc = 0x42dc6e; continue;
 }
 0x42dc6e => {
  pc = 0x42de60; continue;
 }
 0x42dc73 => {
  cpu.push(0x42dc78); f_004184ca(cpu); if cpu.returned_to != 0x42dc78 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x42dd1e; continue; }
  pc = 0x42dc83; continue;
 }
 0x42dc83 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x42dd85; continue; }
  pc = 0x42dc8e; continue;
 }
 0x42dc8e => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x59dafdu32, v); }
  cpu.push(0x42dca5); f_004716d0(cpu); if cpu.returned_to != 0x42dca5 { return; }
  if !cpu.cf { pc = 0x42dd1e; continue; }
  pc = 0x42dcab; continue;
 }
 0x42dcab => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x50u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x42dce0); f_004714a5(cpu); if cpu.returned_to != 0x42dce0 { return; }
  if !cpu.cf { pc = 0x42dceb; continue; }
  pc = 0x42dce2; continue;
 }
 0x42dce2 => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d6934u32, v); }
  pc = 0x42dceb; continue;
 }
 0x42dceb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4ea1b2u32.wrapping_add(cpu.eax)), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42dec2; continue; }
  pc = 0x42dd0e; continue;
 }
 0x42dd0e => {
  if !cpu.zf { pc = 0x42dd1e; continue; }
  pc = 0x42dd10; continue;
 }
 0x42dd10 => {
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x0u8); }
  if !cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42dd19; continue;
 }
 0x42dd19 => {
  pc = 0x42de91; continue;
 }
 0x42dd1e => {
  cpu.push(0x42dd23); f_004302f6(cpu); if cpu.returned_to != 0x42dd23 { return; }
  if cpu.cf { pc = 0x42dec2; continue; }
  pc = 0x42dd29; continue;
 }
 0x42dd29 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42de03; continue; }
  pc = 0x42dd37; continue;
 }
 0x42dd37 => {
  cpu.push(0x42dd3c); f_004184ca(cpu); if cpu.returned_to != 0x42dd3c { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x42dd59; continue; }
  pc = 0x42dd43; continue;
 }
 0x42dd43 => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42dd53); f_0041a69f(cpu); if cpu.returned_to != 0x42dd53 { return; }
  if !cpu.cf { pc = 0x42def3; continue; }
  pc = 0x42dd59; continue;
 }
 0x42dd59 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0xaefu16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x4d67d8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42dd85 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42dd8c; continue;
 }
 0x42dd8c => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x42dda9); f_0046c3d4(cpu); if cpu.returned_to != 0x42dda9 { return; }
  if cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42ddab; continue;
 }
 0x42ddab => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0xaf7u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42ddd7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x7u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0xaf3u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42de03 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42de2f => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42de5f); f_004204d6(cpu); if cpu.returned_to != 0x42de5f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42de60 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42de90); f_004204d6(cpu); if cpu.returned_to != 0x42de90 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42de91 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff5u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42dec1); f_004204d6(cpu); if cpu.returned_to != 0x42dec1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42dec2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x2du16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff7u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42def2); f_004204d6(cpu); if cpu.returned_to != 0x42def2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42def3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x42df0e); f_00471538(cpu); if cpu.returned_to != 0x42df0e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x42df50; continue; }
  pc = 0x42df12; continue;
 }
 0x42df12 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42df22; continue;
 }
 0x42df22 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.ebx = v; }
  cpu.push(0x42df3d); f_00471211(cpu); if cpu.returned_to != 0x42df3d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  if !cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42df4b; continue;
 }
 0x42df4b => {
  cpu.poll(); pc = 0x42de03; continue;
 }
 0x42df50 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42e01d; continue; }
  pc = 0x42df5d; continue;
 }
 0x42df5d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42df8d; continue; }
  pc = 0x42df70; continue;
 }
 0x42df70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.edi = v; }
  cpu.push(0x42df85); f_004715d5(cpu); if cpu.returned_to != 0x42df85 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42df8d; continue;
 }
 0x42df8d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42dfbd; continue; }
  pc = 0x42dfa0; continue;
 }
 0x42dfa0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.edi = v; }
  cpu.push(0x42dfb5); f_004715d5(cpu); if cpu.returned_to != 0x42dfb5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42dfbd; continue;
 }
 0x42dfbd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42dfed; continue; }
  pc = 0x42dfd0; continue;
 }
 0x42dfd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.edi = v; }
  cpu.push(0x42dfe5); f_004715d5(cpu); if cpu.returned_to != 0x42dfe5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42dfed; continue;
 }
 0x42dfed => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42e01d; continue; }
  pc = 0x42e000; continue;
 }
 0x42e000 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x59da9bu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x59da9fu32) as u32; cpu.edi = v; }
  cpu.push(0x42e015); f_004715d5(cpu); if cpu.returned_to != 0x42e015 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42e01d; continue;
 }
 0x42e01d => {
  { let v: u32 = cpu.r32(0x59da9bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59da9fu32); cpu.ebx = v; }
  cpu.push(0x42e02d); f_0047111f(cpu); if cpu.returned_to != 0x42e02d { return; }
  if !cpu.cf { pc = 0x42e03b; continue; }
  pc = 0x42e02f; continue;
 }
 0x42e02f => {
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.ecx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x42dd59; continue; }
  pc = 0x42e03b; continue;
 }
 0x42e03b => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42e06b); f_004204d6(cpu); if cpu.returned_to != 0x42e06b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e06c => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x2bu16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42e09c); f_004204d6(cpu); if cpu.returned_to != 0x42e09c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e09d => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d67d4u32, v); }
  { let v: u16 = 0x2cu16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0xfffbu16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0xfff5u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x42e0cd); f_004204d6(cpu); if cpu.returned_to != 0x42e0cd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e0fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e0fa); }
 let mut pc: u32 = 0x42e0fa;
 loop { match pc {
 0x42e0fa => {
  cpu.push(0x42e0ff); f_00420989(cpu); if cpu.returned_to != 0x42e0ff { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59da62u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x581a0cu32, v); }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59d9f4u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x42e141; continue; }
  pc = 0x42e122; continue;
 }
 0x42e122 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x42e15a; continue; }
  pc = 0x42e129; continue;
 }
 0x42e129 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x42e13a; continue; }
  pc = 0x42e130; continue;
 }
 0x42e130 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x42e14c; continue; }
  pc = 0x42e13a; continue;
 }
 0x42e13a => {
  { let v: u8 = cpu.r8(0x4e9f14u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e141 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e147); f_0046d525(cpu); if cpu.returned_to != 0x42e147 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x42e153; continue; }
  pc = 0x42e14c; continue;
 }
 0x42e14c => {
  { let v: u8 = cpu.r8(0x4e9f57u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e153 => {
  { let v: u8 = cpu.r8(0x4e9fddu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e15a => {
  { let v: u8 = cpu.r8(0x4e9f9au32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e1c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e1c9); }
 let mut pc: u32 = 0x42e1c9;
 loop { match pc {
 0x42e1c9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dadau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1facu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fadu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1faeu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59daddu32, v); }
  cpu.push(0x42e1f1); f_0042e4ec(cpu); if cpu.returned_to != 0x42e1f1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e1f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e1f2); }
 let mut pc: u32 = 0x42e1f2;
 loop { match pc {
 0x42e1f2 => {
  { cpu.sub8(cpu.r8(0x4e1fadu32), 0x0u8); }
  if cpu.zf { pc = 0x42e1fd; continue; }
  pc = 0x42e1fb; continue;
 }
 0x42e1fb => {
  pc = 0x42e228; continue;
 }
 0x42e1fd => {
  { cpu.sub8(cpu.r8(0x59d9d9u32), 0x0u8); }
  if cpu.zf { pc = 0x42e288; continue; }
  pc = 0x42e20a; continue;
 }
 0x42e20a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dad5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e218); f_004010f0(cpu); if cpu.returned_to != 0x42e218 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d9u32, v); }
  pc = 0x42e288; continue;
 }
 0x42e228 => {
  { cpu.sub8(cpu.r8(0x59d9d9u32), 0x0u8); }
  if !cpu.zf { pc = 0x42e288; continue; }
  pc = 0x42e235; continue;
 }
 0x42e235 => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e288; continue; }
  pc = 0x42e23e; continue;
 }
 0x42e23e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x42e25e; continue; }
  pc = 0x42e257; continue;
 }
 0x42e257 => {
  { let v: u32 = 0x76u32; cpu.eax = v; }
  pc = 0x42e263; continue;
 }
 0x42e25e => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  pc = 0x42e263; continue;
 }
 0x42e263 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e26a); f_00401130(cpu); if cpu.returned_to != 0x42e26a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42e288; continue; }
  pc = 0x42e27c; continue;
 }
 0x42e27c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dad5u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d9d9u32, v); }
  pc = 0x42e288; continue;
 }
 0x42e288 => {
  { cpu.sub8(cpu.r8(0x59daddu32), 0x0u8); }
  if cpu.zf { pc = 0x42e293; continue; }
  pc = 0x42e291; continue;
 }
 0x42e291 => {
  pc = 0x42e2be; continue;
 }
 0x42e293 => {
  { cpu.sub8(cpu.r8(0x59dadfu32), 0x0u8); }
  if cpu.zf { pc = 0x42e32c; continue; }
  pc = 0x42e2a0; continue;
 }
 0x42e2a0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dadeu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e2ae); f_004010f0(cpu); if cpu.returned_to != 0x42e2ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadfu32, v); }
  pc = 0x42e32c; continue;
 }
 0x42e2be => {
  { cpu.sub8(cpu.r8(0x59dadfu32), 0x0u8); }
  if !cpu.zf { pc = 0x42e32c; continue; }
  pc = 0x42e2cb; continue;
 }
 0x42e2cb => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e32c; continue; }
  pc = 0x42e2d8; continue;
 }
 0x42e2d8 => {
  { cpu.sub8(cpu.r8(0x59daddu32), 0x1u8); }
  if cpu.zf { pc = 0x42e2fd; continue; }
  pc = 0x42e2e1; continue;
 }
 0x42e2e1 => {
  { let v: u32 = 0x2fu32; cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e2ed); f_00401130(cpu); if cpu.returned_to != 0x42e2ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  pc = 0x42e317; continue;
 }
 0x42e2fd => {
  { cpu.pushad(); }
  { let v: u32 = 0x2du32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e309); f_00401130(cpu); if cpu.returned_to != 0x42e309 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  pc = 0x42e317; continue;
 }
 0x42e317 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42e32c; continue; }
  pc = 0x42e31b; continue;
 }
 0x42e31b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dadeu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x59daddu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dadfu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42e32c; continue;
 }
 0x42e32c => {
  { cpu.sub8(cpu.r8(0x4e1facu32), 0x0u8); }
  if cpu.zf { pc = 0x42e337; continue; }
  pc = 0x42e335; continue;
 }
 0x42e335 => {
  pc = 0x42e369; continue;
 }
 0x42e337 => {
  { cpu.sub8(cpu.r8(0x59d9d8u32), 0x0u8); }
  if cpu.zf { pc = 0x42e414; continue; }
  pc = 0x42e344; continue;
 }
 0x42e344 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dad4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e352); f_004010f0(cpu); if cpu.returned_to != 0x42e352 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d8u32, v); }
  { let v: u8 = 0x2au8; cpu.w8(0x59dad4u32, v); }
  pc = 0x42e414; continue;
 }
 0x42e369 => {
  { cpu.sub8(cpu.r8(0x59d9d8u32), 0x0u8); }
  if cpu.zf { pc = 0x42e3a5; continue; }
  pc = 0x42e372; continue;
 }
 0x42e372 => {
  { let v: u8 = cpu.r8(0x4e1facu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x59d9d8u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x42e3e8; continue; }
  pc = 0x42e383; continue;
 }
 0x42e383 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dad4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e391); f_004010f0(cpu); if cpu.returned_to != 0x42e391 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d8u32, v); }
  { let v: u8 = 0x2au8; cpu.w8(0x59dad4u32, v); }
  pc = 0x42e3a5; continue;
 }
 0x42e3a5 => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e414; continue; }
  pc = 0x42e3b2; continue;
 }
 0x42e3b2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1facu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x59dbdfu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e3c7); f_00401130(cpu); if cpu.returned_to != 0x42e3c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42e414; continue; }
  pc = 0x42e3d9; continue;
 }
 0x42e3d9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dad4u32, v); }
  { let v: u8 = cpu.r8(0x4e1facu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59d9d8u32, v); }
  pc = 0x42e3e8; continue;
 }
 0x42e3e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x59dad4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x42e414; continue; }
  pc = 0x42e3f5; continue;
 }
 0x42e3f5 => {
  { let v: u16 = cpu.r16(0x59dad6u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42e408; continue; }
  pc = 0x42e403; continue;
 }
 0x42e403 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  pc = 0x42e408; continue;
 }
 0x42e408 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5cc64eu32, v); }
  cpu.push(0x42e414); f_00415d61(cpu); if cpu.returned_to != 0x42e414 { return; }
  pc = 0x42e414; continue;
 }
 0x42e414 => {
  { cpu.sub8(cpu.r8(0x59dadau32), 0x0u8); }
  if cpu.zf { pc = 0x42e41f; continue; }
  pc = 0x42e41d; continue;
 }
 0x42e41d => {
  pc = 0x42e44a; continue;
 }
 0x42e41f => {
  { cpu.sub8(cpu.r8(0x59dadbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e4e8; continue; }
  pc = 0x42e42c; continue;
 }
 0x42e42c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dadcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e43a); f_004010f0(cpu); if cpu.returned_to != 0x42e43a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadbu32, v); }
  pc = 0x42e4e8; continue;
 }
 0x42e44a => {
  { cpu.sub8(cpu.r8(0x59dadbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e476; continue; }
  pc = 0x42e453; continue;
 }
 0x42e453 => {
  { let v: u8 = cpu.r8(0x59dadau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x59dadbu32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x42e4e8; continue; }
  pc = 0x42e464; continue;
 }
 0x42e464 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dadcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e472); f_004010f0(cpu); if cpu.returned_to != 0x42e472 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x42e476; continue;
 }
 0x42e476 => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x42e4e8; continue; }
  pc = 0x42e483; continue;
 }
 0x42e483 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x59dadau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x59dc03u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42e498); f_00401130(cpu); if cpu.returned_to != 0x42e498 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42e4e8; continue; }
  pc = 0x42e4aa; continue;
 }
 0x42e4aa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dadcu32, v); }
  { let v: u8 = cpu.r8(0x59dadau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dadbu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x59dadcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { cpu.poll(); pc = 0x42e414; continue; }
  pc = 0x42e4ca; continue;
 }
 0x42e4ca => {
  { let v: u16 = cpu.r16(0x59dad8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42e4dc; continue; }
  pc = 0x42e4d8; continue;
 }
 0x42e4d8 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x42e4dc; continue;
 }
 0x42e4dc => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5cc64eu32, v); }
  cpu.push(0x42e4e8); f_00415d61(cpu); if cpu.returned_to != 0x42e4e8 { return; }
  pc = 0x42e4e8; continue;
 }
 0x42e4e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e4eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e4eb); }
 let mut pc: u32 = 0x42e4eb;
 loop { match pc {
 0x42e4eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e4ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e4ec); }
 let mut pc: u32 = 0x42e4ec;
 loop { match pc {
 0x42e4ec => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x42e4fe); f_00420989(cpu); if cpu.returned_to != 0x42e4fe { return; }
  { let v: u32 = 0x4000u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4d6ba8u32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4d6ba4u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x42e538; continue; }
  pc = 0x42e534; continue;
 }
 0x42e534 => {
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x42e53c; continue;
 }
 0x42e538 => {
  { let v: u8 = 0x1du8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x42e53c; continue;
 }
 0x42e53c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x42e555; continue; }
  pc = 0x42e542; continue;
 }
 0x42e542 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42e53c; continue; }
  pc = 0x42e54d; continue;
 }
 0x42e54d => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fadu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e555 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fadu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e55d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e55d); }
 let mut pc: u32 = 0x42e55d;
 loop { match pc {
 0x42e55d => {
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x42e5a0; continue; }
  pc = 0x42e566; continue;
 }
 0x42e566 => {
  { let v = cpu.r16(0x4d6752u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6754u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6752u32, v); }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6754u32, v); }
  cpu.push(0x42e591); f_0044ed94(cpu); if cpu.returned_to != 0x42e591 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6754u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6752u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e5a0 => {
  cpu.push(0x42e5a5); f_0044ed94(cpu); if cpu.returned_to != 0x42e5a5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e5a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e5a6); }
 let mut pc: u32 = 0x42e5a6;
 loop { match pc {
 0x42e5a6 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x42e65c; continue; }
  pc = 0x42e5b3; continue;
 }
 0x42e5b3 => {
  { cpu.sub8(cpu.r8(0x56d302u32), 0x0u8); }
  if cpu.zf { pc = 0x42e5c9; continue; }
  pc = 0x42e5bc; continue;
 }
 0x42e5bc => {
  { cpu.sub8(cpu.r8(0x4e1570u32), 0x0u8); }
  if cpu.zf { pc = 0x42e62e; continue; }
  pc = 0x42e5c9; continue;
 }
 0x42e5c9 => {
  { cpu.sub8(cpu.r8(0x56d303u32), 0x0u8); }
  if cpu.zf { pc = 0x42e5df; continue; }
  pc = 0x42e5d2; continue;
 }
 0x42e5d2 => {
  { cpu.sub8(cpu.r8(0x4e1571u32), 0x0u8); }
  if cpu.zf { pc = 0x42e63a; continue; }
  pc = 0x42e5df; continue;
 }
 0x42e5df => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x42e65c; continue; }
  pc = 0x42e5ec; continue;
 }
 0x42e5ec => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42e65c; continue; }
  pc = 0x42e5fa; continue;
 }
 0x42e5fa => {
  { cpu.sub8(cpu.r8(0x56d304u32), 0x0u8); }
  if cpu.zf { pc = 0x42e60c; continue; }
  pc = 0x42e603; continue;
 }
 0x42e603 => {
  { cpu.sub8(cpu.r8(0x4e1572u32), 0x0u8); }
  if cpu.zf { pc = 0x42e646; continue; }
  pc = 0x42e60c; continue;
 }
 0x42e60c => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42e65c; continue; }
  pc = 0x42e61a; continue;
 }
 0x42e61a => {
  { cpu.sub8(cpu.r8(0x56d305u32), 0x0u8); }
  if cpu.zf { pc = 0x42e65c; continue; }
  pc = 0x42e623; continue;
 }
 0x42e623 => {
  { cpu.sub8(cpu.r8(0x4e1573u32), 0x0u8); }
  if cpu.zf { pc = 0x42e652; continue; }
  pc = 0x42e62c; continue;
 }
 0x42e62c => {
  pc = 0x42e65c; continue;
 }
 0x42e62e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x42e638); f_00456b9e(cpu); if cpu.returned_to != 0x42e638 { return; }
  pc = 0x42e65c; continue;
 }
 0x42e63a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x42e644); f_00456ca1(cpu); if cpu.returned_to != 0x42e644 { return; }
  pc = 0x42e65c; continue;
 }
 0x42e646 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x42e650); f_00456da6(cpu); if cpu.returned_to != 0x42e650 { return; }
  pc = 0x42e65c; continue;
 }
 0x42e652 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x42e65c); f_00456ea4(cpu); if cpu.returned_to != 0x42e65c { return; }
  pc = 0x42e65c; continue;
 }
 0x42e65c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e65d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e65d); }
 let mut pc: u32 = 0x42e65d;
 loop { match pc {
 0x42e65d => {
  { let v: u32 = cpu.esi.wrapping_add(0x1efu32); cpu.esi = v; }
  { let v: u32 = 0x59ee1eu32; cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x59ee1eu32; cpu.esi = v; }
  { let v: u32 = 0x59ee5du32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ebp = v; }
  pc = 0x42e67e; continue;
 }
 0x42e67e => {
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x13u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x3u32); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.add32(cpu.esi, 0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42e67e; continue; }
  pc = 0x42e699; continue;
 }
 0x42e699 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042e69a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42e69a); }
 let mut pc: u32 = 0x42e69a;
 loop { match pc {
 0x42e69a => {
  { let v: u8 = cpu.r8(0x59d9ddu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x59d9dcu32), (cpu.eax as u8)); cpu.w8(0x59d9dcu32, v); }
  if !cpu.sf { pc = 0x42e6b5; continue; }
  pc = 0x42e6a7; continue;
 }
 0x42e6a7 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x59d9ddu32)); cpu.w8(0x59d9ddu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42e6b5 => {
  { cpu.sub8(cpu.r8(0x59d9dcu32), 0x14u8); }
  if cpu.cf { pc = 0x42e6c4; continue; }
  pc = 0x42e6be; continue;
 }
 0x42e6be => {
  { let v: u8 = cpu.neg8(cpu.r8(0x59d9ddu32)); cpu.w8(0x59d9ddu32, v); }
  pc = 0x42e6c4; continue;
 }
 0x42e6c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042eaf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42eaf0); }
 let mut pc: u32 = 0x42eaf0;
 loop { match pc {
 0x42eaf0 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x42eafa; continue;
 }
 0x42eafa => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x42eb1b; continue; }
  pc = 0x42eb01; continue;
 }
 0x42eb01 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x42eb16; continue; }
  pc = 0x42eb09; continue;
 }
 0x42eb09 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x42eb1b; continue; }
  pc = 0x42eb0f; continue;
 }
 0x42eb0f => {
  cpu.push(0x42eb14); f_0042eb3b(cpu); if cpu.returned_to != 0x42eb14 { return; }
  pc = 0x42eb1b; continue;
 }
 0x42eb16 => {
  cpu.push(0x42eb1b); f_0042eb94(cpu); if cpu.returned_to != 0x42eb1b { return; }
  pc = 0x42eb1b; continue;
 }
 0x42eb1b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42eafa; continue; }
  pc = 0x42eb21; continue;
 }
 0x42eb21 => {
  cpu.push(0x42eb26); f_00455069(cpu); if cpu.returned_to != 0x42eb26 { return; }
  cpu.push(0x42eb2b); f_00479ba4(cpu); if cpu.returned_to != 0x42eb2b { return; }
  cpu.push(0x42eb30); f_0047227b(cpu); if cpu.returned_to != 0x42eb30 { return; }
  cpu.push(0x42eb35); f_00471fab(cpu); if cpu.returned_to != 0x42eb35 { return; }
  cpu.push(0x42eb3a); f_004665ce(cpu); if cpu.returned_to != 0x42eb3a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042eb3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42eb3b); }
 let mut pc: u32 = 0x42eb3b;
 loop { match pc {
 0x42eb3b => {
  { cpu.pushad(); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x42eb46; continue;
 }
 0x42eb46 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42eb8c; continue; }
  pc = 0x42eb4e; continue;
 }
 0x42eb4e => {
  { let v: u8 = 0x0u8; cpu.w8(0x575f2cu32, v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x42eb60; continue; }
  pc = 0x42eb59; continue;
 }
 0x42eb59 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575f2cu32, v); }
  pc = 0x42eb60; continue;
 }
 0x42eb60 => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if !cpu.zf { pc = 0x42eb8c; continue; }
  pc = 0x42eb66; continue;
 }
 0x42eb66 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x42eb84; continue; }
  pc = 0x42eb6f; continue;
 }
 0x42eb6f => {
  { cpu.pushad(); }
  cpu.push(0x42eb75); f_004427d7(cpu); if cpu.returned_to != 0x42eb75 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f2cu32, v); }
  pc = 0x42eb84; continue;
 }
 0x42eb84 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  pc = 0x42eb8c; continue;
 }
 0x42eb8c => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42eb46; continue; }
  pc = 0x42eb92; continue;
 }
 0x42eb92 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042eb94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42eb94); }
 let mut pc: u32 = 0x42eb94;
 loop { match pc {
 0x42eb94 => {
  { cpu.pushad(); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x42eb9f; continue;
 }
 0x42eb9f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42ebca; continue; }
  pc = 0x42eba7; continue;
 }
 0x42eba7 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x42ebca; continue; }
  pc = 0x42ebab; continue;
 }
 0x42ebab => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if !cpu.zf { pc = 0x42ebca; continue; }
  pc = 0x42ebb1; continue;
 }
 0x42ebb1 => {
  { cpu.pushad(); }
  cpu.push(0x42ebb7); f_004427d7(cpu); if cpu.returned_to != 0x42ebb7 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x24du32.wrapping_add(cpu.ebp), v); }
  pc = 0x42ebca; continue;
 }
 0x42ebca => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42eb9f; continue; }
  pc = 0x42ebd0; continue;
 }
 0x42ebd0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ebd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ebd2); }
 let mut pc: u32 = 0x42ebd2;
 loop { match pc {
 0x42ebd2 => {
  { let v: u32 = 0x58c44au32; cpu.esi = v; }
  { let v: u32 = 0x59c5b0u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x42ebea; continue;
 }
 0x42ebea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42ebea; continue; }
  pc = 0x42ebf9; continue;
 }
 0x42ebf9 => {
  { let v: u32 = 0x59c5b1u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  pc = 0x42ec02; continue;
 }
 0x42ec02 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x42ec0a; continue; }
  pc = 0x42ec08; continue;
 }
 0x42ec08 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x42ec0a; continue;
 }
 0x42ec0a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42ec02; continue; }
  pc = 0x42ec10; continue;
 }
 0x42ec10 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ec8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ec8e); }
 let mut pc: u32 = 0x42ec8e;
 loop { match pc {
 0x42ec8e => {
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x42eda8; continue; }
  pc = 0x42ec9b; continue;
 }
 0x42ec9b => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d315u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56d302u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x56d2ffu32)); cpu.w16(0x56d2ffu32, v); }
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x1u16); }
  if cpu.zf { pc = 0x42ed6f; continue; }
  pc = 0x42ecbf; continue;
 }
 0x42ecbf => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if !cpu.zf { pc = 0x42ed1c; continue; }
  pc = 0x42ecc9; continue;
 }
 0x42ecc9 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x42ecce; continue;
 }
 0x42ecce => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x42ecd3; continue;
 }
 0x42ecd3 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x42ecdc; continue; }
  pc = 0x42ecd7; continue;
 }
 0x42ecd7 => {
  cpu.push(0x42ecdc); f_004517a3(cpu); if cpu.returned_to != 0x42ecdc { return; }
  pc = 0x42ecdc; continue;
 }
 0x42ecdc => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42ecd3; continue; }
  pc = 0x42ece2; continue;
 }
 0x42ece2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42ecce; continue; }
  pc = 0x42ece8; continue;
 }
 0x42ece8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x58c44au32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x58c44eu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x58c452u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x58c456u32, v); }
  { cpu.pushad(); }
  cpu.push(0x42ed16); f_00440e68(cpu); if cpu.returned_to != 0x42ed16 { return; }
  { cpu.popad(); }
  pc = 0x42ed79; continue;
 }
 0x42ed1c => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))), 0xffffffffu32); }
  if cpu.zf { pc = 0x42ed60; continue; }
  pc = 0x42ed2c; continue;
 }
 0x42ed2c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x42ed30; continue;
 }
 0x42ed30 => {
  { let v: u8 = cpu.r8(0x58c44au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x42ed51; continue; }
  pc = 0x42ed3c; continue;
 }
 0x42ed3c => {
  { let v: u8 = 0xffu8; cpu.w8(0x58c44au32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  cpu.push(0x42ed51); f_004517a3(cpu); if cpu.returned_to != 0x42ed51 { return; }
  pc = 0x42ed51; continue;
 }
 0x42ed51 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x42ed30; continue; }
  pc = 0x42ed57; continue;
 }
 0x42ed57 => {
  { cpu.pushad(); }
  cpu.push(0x42ed5d); f_00440e68(cpu); if cpu.returned_to != 0x42ed5d { return; }
  { cpu.popad(); }
  pc = 0x42ed79; continue;
 }
 0x42ed60 => {
  cpu.push(0x42ed65); f_0042ebd2(cpu); if cpu.returned_to != 0x42ed65 { return; }
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x42ed79; continue; }
  pc = 0x42ed6a; continue;
 }
 0x42ed6a => {
  cpu.poll(); pc = 0x42ecc9; continue;
 }
 0x42ed6f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4e1fccu32, v); }
  pc = 0x42ed79; continue;
 }
 0x42ed79 => {
  { cpu.popad(); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42ed84; continue;
 }
 0x42ed84 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x42ed9f; continue; }
  pc = 0x42ed8d; continue;
 }
 0x42ed8d => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x42ed9f; continue; }
  pc = 0x42ed94; continue;
 }
 0x42ed94 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x42ed9c); f_004586f4(cpu); if cpu.returned_to != 0x42ed9c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42ed9f; continue;
 }
 0x42ed9f => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42ed84; continue; }
  pc = 0x42eda8; continue;
 }
 0x42eda8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d315u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042edb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42edb0); }
 let mut pc: u32 = 0x42edb0;
 loop { match pc {
 0x42edb0 => {
  { cpu.sub8(cpu.r8(0x59d9dcu32), 0x0u8); }
  if cpu.sf { pc = 0x42ee19; continue; }
  pc = 0x42edb9; continue;
 }
 0x42edb9 => {
  { let v: u32 = 0x15u32; cpu.eax = v; }
  { let v = cpu.r8(0x59d9dcu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x59ee1eu32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x5e04fcu32; cpu.edi = v; }
  { let v: u8 = 0xa5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x42edd9; continue;
 }
 0x42edd9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x42ede7); f_00417e90(cpu); if cpu.returned_to != 0x42ede7 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42edd9; continue; }
  pc = 0x42ee19; continue;
 }
 0x42ee19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ee1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ee1a); }
 let mut pc: u32 = 0x42ee1a;
 loop { match pc {
 0x42ee1a => {
  { cpu.sub8(cpu.r8(0x4e156eu32), 0x0u8); }
  if cpu.zf { pc = 0x42ee28; continue; }
  pc = 0x42ee23; continue;
 }
 0x42ee23 => {
  cpu.push(0x42ee28); f_0042e69a(cpu); if cpu.returned_to != 0x42ee28 { return; }
  pc = 0x42ee28; continue;
 }
 0x42ee28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ee29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ee29); }
 let mut pc: u32 = 0x42ee29;
 loop { match pc {
 0x42ee29 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e156au32)); cpu.w8(0x4e156au32, v); }
  { let v: u8 = cpu.r8(0x4e156au32) & 0x7u8; cpu.w8(0x4e156au32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e72u32)); cpu.w32(0x576e72u32, v); }
  { let v: u32 = cpu.r32(0x576e72u32) & 0xfu32; cpu.w32(0x576e72u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e76u32)); cpu.w32(0x576e76u32, v); }
  { let v: u32 = cpu.r32(0x576e76u32) & 0x1fu32; cpu.w32(0x576e76u32, v); }
  { let v: u8 = !cpu.r8(0x4e1557u32); cpu.w8(0x4e1557u32, v); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x42ee6e; continue; }
  pc = 0x42ee5f; continue;
 }
 0x42ee5f => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d677eu32)); cpu.w16(0x4d677eu32, v); }
  { let v: u16 = cpu.r16(0x4d677eu32) & 0x3u16; cpu.w16(0x4d677eu32, v); }
  pc = 0x42ee6e; continue;
 }
 0x42ee6e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e15c5u32)); cpu.w8(0x4e15c5u32, v); }
  if !cpu.sf { pc = 0x42ee7d; continue; }
  pc = 0x42ee76; continue;
 }
 0x42ee76 => {
  { let v: u8 = 0x16u8; cpu.w8(0x4e15c5u32, v); }
  pc = 0x42ee7d; continue;
 }
 0x42ee7d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1569u32)); cpu.w8(0x4e1569u32, v); }
  { cpu.sub8(cpu.r8(0x4e1569u32), 0x6u8); }
  if !cpu.zf { pc = 0x42ee93; continue; }
  pc = 0x42ee8c; continue;
 }
 0x42ee8c => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1569u32, v); }
  pc = 0x42ee93; continue;
 }
 0x42ee93 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ee94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ee94); }
 let mut pc: u32 = 0x42ee94;
 loop { match pc {
 0x42ee94 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d346u32, v); }
  { let v: u32 = cpu.r32(0x4d69ecu32); cpu.ecx = v; }
  { let v: u32 = 0x505310u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  if !cpu.sf { pc = 0x42eeb7; continue; }
  pc = 0x42eeb5; continue;
 }
 0x42eeb5 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x42eeb7; continue;
 }
 0x42eeb7 => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6ba8u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x20u32); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6790u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42eed9; continue; }
  pc = 0x42eed2; continue;
 }
 0x42eed2 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x42eed9; continue;
 }
 0x42eed9 => {
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  pc = 0x42eee2; continue;
 }
 0x42eee2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x42ef15; continue; }
  pc = 0x42eeec; continue;
 }
 0x42eeec => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42ef10; continue; }
  pc = 0x42eef3; continue;
 }
 0x42eef3 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x42ef10; continue; }
  pc = 0x42eefa; continue;
 }
 0x42eefa => {
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42ef07; continue; }
  pc = 0x42ef01; continue;
 }
 0x42ef01 => {
  { let v: u8 = cpu.dec8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x42ef10; continue;
 }
 0x42ef07 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42ef0e); f_0042ef16(cpu); if cpu.returned_to != 0x42ef0e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42ef10; continue;
 }
 0x42ef10 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x10u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42eee2; continue; }
  pc = 0x42ef15; continue;
 }
 0x42ef15 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ef16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42ef16); }
 let mut pc: u32 = 0x42ef16;
 loop { match pc {
 0x42ef16 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42ef49; continue; }
  pc = 0x42ef1d; continue;
 }
 0x42ef1d => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x42f0e8; continue; }
  pc = 0x42ef2a; continue;
 }
 0x42ef2a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x42f0e8; continue; }
  pc = 0x42ef34; continue;
 }
 0x42ef34 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x42efb1; continue;
 }
 0x42ef49 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x42f0e8; continue; }
  pc = 0x42ef53; continue;
 }
 0x42ef53 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x6u32.wrapping_add(cpu.ebp))); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.cf { pc = 0x42ef89; continue; }
  pc = 0x42ef66; continue;
 }
 0x42ef66 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x42efb1; continue; }
  pc = 0x42ef7e; continue;
 }
 0x42ef7e => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x42f0e8; continue; }
  pc = 0x42ef89; continue;
 }
 0x42ef89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x42ef9e; continue; }
  pc = 0x42ef95; continue;
 }
 0x42ef95 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42ef9b); f_0041c6a6(cpu); if cpu.returned_to != 0x42ef9b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42ef9e; continue;
 }
 0x42ef9e => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x42f0e8; continue; }
  pc = 0x42efaf; continue;
 }
 0x42efaf => {
  cpu.poll(); pc = 0x42ef66; continue;
 }
 0x42efb1 => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d954cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd54cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x4dd54cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x42efd8); f_00420978(cpu); if cpu.returned_to != 0x42efd8 { return; }
  cpu.push(0x42efdd); f_004679cb(cpu); if cpu.returned_to != 0x42efdd { return; }
  if cpu.cf { pc = 0x42f063; continue; }
  pc = 0x42efe3; continue;
 }
 0x42efe3 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x42f060; continue; }
  pc = 0x42f007; continue;
 }
 0x42f007 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x42f060; continue; }
  pc = 0x42f018; continue;
 }
 0x42f018 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x42f060; continue; }
  pc = 0x42f01f; continue;
 }
 0x42f01f => {
  { cpu.sub32(cpu.r32(0x4d6bd8u32), 0x4u32); }
  if !cpu.zf { pc = 0x42f02f; continue; }
  pc = 0x42f028; continue;
 }
 0x42f028 => {
  cpu.push(0x42f02d); f_0042f0e9(cpu); if cpu.returned_to != 0x42f02d { return; }
  pc = 0x42f060; continue;
 }
 0x42f02f => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x76u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x42f055; continue; }
  pc = 0x42f045; continue;
 }
 0x42f045 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42f04c); f_00401130(cpu); if cpu.returned_to != 0x42f04c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x42f055; continue;
 }
 0x42f055 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x42f060); f_00415d61(cpu); if cpu.returned_to != 0x42f060 { return; }
  pc = 0x42f060; continue;
 }
 0x42f060 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42f066; continue;
 }
 0x42f063 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x42f066; continue;
 }
 0x42f066 => {
  { cpu.sub8(cpu.r8(0xbu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x42f08b; continue; }
  pc = 0x42f06d; continue;
 }
 0x42f06d => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x42f0a9; continue; }
  pc = 0x42f074; continue;
 }
 0x42f074 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d95ccu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd5ccu32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd5ccu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x4dd5ccu32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x4dd5ccu32.wrapping_add(cpu.edi), v); }
  pc = 0x42f0e8; continue;
 }
 0x42f08b => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x42f0e8; continue; }
  pc = 0x42f092; continue;
 }
 0x42f092 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d954du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd54du32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd54du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x4dd54du32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x4dd54du32.wrapping_add(cpu.edi), v); }
  pc = 0x42f0e8; continue;
 }
 0x42f0a9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d954du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd54du32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd54du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd54du32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x4dd54du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x13u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d95ccu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd5ccu32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd5ccu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd5ccu32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x4dd5ccu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d95cdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4dd5cdu32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x4dd5cdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x4dd5cdu32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x4dd5cdu32.wrapping_add(cpu.edi), v); }
  pc = 0x42f0e8; continue;
 }
 0x42f0e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f0e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f0e9); }
 let mut pc: u32 = 0x42f0e9;
 loop { match pc {
 0x42f0e9 => {
  { cpu.pushad(); }
  cpu.push(0x42f0ef); f_0041c6a6(cpu); if cpu.returned_to != 0x42f0ef { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42f10f; continue; }
  pc = 0x42f0f7; continue;
 }
 0x42f0f7 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x42f108; continue; }
  pc = 0x42f0fc; continue;
 }
 0x42f0fc => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x42f139; continue; }
  pc = 0x42f101; continue;
 }
 0x42f101 => {
  { let v: u32 = 0x69u32; cpu.eax = v; }
  pc = 0x42f114; continue;
 }
 0x42f108 => {
  { let v: u32 = 0x75u32; cpu.eax = v; }
  pc = 0x42f114; continue;
 }
 0x42f10f => {
  { let v: u32 = 0x76u32; cpu.eax = v; }
  pc = 0x42f114; continue;
 }
 0x42f114 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42f11b); f_00401130(cpu); if cpu.returned_to != 0x42f11b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59fb6cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x59fb6cu32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42f139; continue; }
  pc = 0x42f12d; continue;
 }
 0x42f12d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x59dad5u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d9d9u32, v); }
  pc = 0x42f139; continue;
 }
 0x42f139 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f13b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f13b); }
 let mut pc: u32 = 0x42f13b;
 loop { match pc {
 0x42f13b => {
  { let v: u32 = 0x4d6fc8u32; cpu.esi = v; }
  cpu.push(0x42f145); f_0042f177(cpu); if cpu.returned_to != 0x42f145 { return; }
  cpu.push(0x42f14a); f_0041750f(cpu); if cpu.returned_to != 0x42f14a { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x576c0bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f152(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f152); }
 let mut pc: u32 = 0x42f152;
 loop { match pc {
 0x42f152 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x42f176; continue; }
  pc = 0x42f15b; continue;
 }
 0x42f15b => {
  cpu.push(0x42f160); f_004173cf(cpu); if cpu.returned_to != 0x42f160 { return; }
  { let v: u32 = 0x4d7068u32; cpu.esi = v; }
  cpu.push(0x42f16a); f_0042f177(cpu); if cpu.returned_to != 0x42f16a { return; }
  cpu.push(0x42f16f); f_0041747a(cpu); if cpu.returned_to != 0x42f16f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x576c0bu32, v); }
  pc = 0x42f176; continue;
 }
 0x42f176 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f177); }
 let mut pc: u32 = 0x42f177;
 loop { match pc {
 0x42f177 => {
  { let v: u32 = 0x4d7068u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4d6fc8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x4d6f28u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  let t = cpu.r32(0x4d6f88u32); cpu.push(0x42f192); cpu.call(t); if cpu.returned_to != 0x42f192 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f25b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f25b); }
 let mut pc: u32 = 0x42f25b;
 loop { match pc {
 0x42f25b => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x56d314u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x576bd7u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f2c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f2c5); }
 let mut pc: u32 = 0x42f2c5;
 loop { match pc {
 0x42f2c5 => {
  { let v: u32 = 0x7d0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1fbcu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c48u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f2e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f2e0); }
 let mut pc: u32 = 0x42f2e0;
 loop { match pc {
 0x42f2e0 => {
  { cpu.sub8(cpu.r8(0x59d9e0u32), 0x0u8); }
  if cpu.zf { pc = 0x42f456; continue; }
  pc = 0x42f2ed; continue;
 }
 0x42f2ed => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42f2f7; continue;
 }
 0x42f2f7 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x42f44c; continue; }
  pc = 0x42f301; continue;
 }
 0x42f301 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x42f30a); f_004853e8(cpu); if cpu.returned_to != 0x42f30a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592ffbu32, v); }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x16u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592ffau32, v); }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x42f44a; continue; }
  pc = 0x42f354; continue;
 }
 0x42f354 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x42f44a; continue; }
  pc = 0x42f35e; continue;
 }
 0x42f35e => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x30u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42f44a; continue; }
  pc = 0x42f372; continue;
 }
 0x42f372 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x25u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x42f44a; continue; }
  pc = 0x42f386; continue;
 }
 0x42f386 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x42f398); f_0041b671(cpu); if cpu.returned_to != 0x42f398 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x42f44a; continue; }
  pc = 0x42f3ae; continue;
 }
 0x42f3ae => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x42f3b4); f_00478632(cpu); if cpu.returned_to != 0x42f3b4 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x42f44a; continue; }
  pc = 0x42f3bd; continue;
 }
 0x42f3bd => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x42f3db; continue; }
  pc = 0x42f3c4; continue;
 }
 0x42f3c4 => {
  { let v: u32 = 0x59d96fu32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x42f3d3); f_00447fe3(cpu); if cpu.returned_to != 0x42f3d3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  pc = 0x42f3f0; continue;
 }
 0x42f3db => {
  { let v: u32 = 0x59d973u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x42f3ea); f_00447fe3(cpu); if cpu.returned_to != 0x42f3ea { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  pc = 0x42f3f0; continue;
 }
 0x42f3f0 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x59d8e0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x42f409); f_00447fe3(cpu); if cpu.returned_to != 0x42f409 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x42f44a; continue; }
  pc = 0x42f418; continue;
 }
 0x42f418 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f44a; continue; }
  pc = 0x42f41d; continue;
 }
 0x42f41d => {
  { cpu.sub32(cpu.ecx, 0x5242ccu32); }
  if (cpu.cf || cpu.zf) { pc = 0x42f44a; continue; }
  pc = 0x42f425; continue;
 }
 0x42f425 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x5242ccu32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x1eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  cpu.push(0x42f44a); f_0041b671(cpu); if cpu.returned_to != 0x42f44a { return; }
  pc = 0x42f44a; continue;
 }
 0x42f44a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x42f44c; continue;
 }
 0x42f44c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f2f7; continue; }
  pc = 0x42f456; continue;
 }
 0x42f456 => {
  { let v: u8 = 0x0u8; cpu.w8(0x592ffau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f45e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f45e); }
 let mut pc: u32 = 0x42f45e;
 loop { match pc {
 0x42f45e => {
  { cpu.sub8(cpu.r8(0x4e1fc7u32), 0x0u8); }
  if cpu.zf { pc = 0x42f486; continue; }
  pc = 0x42f467; continue;
 }
 0x42f467 => {
  { let v: u32 = cpu.r32(0x4eaba8u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  cpu.push(0x42f486); f_0041b671(cpu); if cpu.returned_to != 0x42f486 { return; }
  pc = 0x42f486; continue;
 }
 0x42f486 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f488); }
 let mut pc: u32 = 0x42f488;
 loop { match pc {
 0x42f488 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f48b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f48b); }
 let mut pc: u32 = 0x42f48b;
 loop { match pc {
 0x42f48b => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if !cpu.zf { pc = 0x42f53b; continue; }
  pc = 0x42f49b; continue;
 }
 0x42f49b => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if cpu.zf { pc = 0x42f4d0; continue; }
  pc = 0x42f4a4; continue;
 }
 0x42f4a4 => {
  { cpu.sub32(cpu.r32(0x59d8dcu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x42f4cf; continue; }
  pc = 0x42f4ad; continue;
 }
 0x42f4ad => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59d8dcu32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59d9ecu32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59d9eeu32, v); }
  pc = 0x42f4cf; continue;
 }
 0x42f4cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f4d0 => {
  { cpu.sub32(cpu.r32(0x59d8dcu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x42f53b; continue; }
  pc = 0x42f4dd; continue;
 }
 0x42f4dd => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59d8dcu32)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59d8dcu32, v); }
  { cpu.sub32(cpu.eax, 0xfu32); }
  if cpu.cf { pc = 0x42f53b; continue; }
  pc = 0x42f4f7; continue;
 }
 0x42f4f7 => {
  { let v: u16 = cpu.r16(0x59d9ecu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x59d9eeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x42f53b; continue; }
  pc = 0x42f52c; continue;
 }
 0x42f52c => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x42f53b; continue; }
  pc = 0x42f536; continue;
 }
 0x42f536 => {
  cpu.push(0x42f53b); f_0041b24f(cpu); if cpu.returned_to != 0x42f53b { return; }
  pc = 0x42f53b; continue;
 }
 0x42f53b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f53c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f53c); }
 let mut pc: u32 = 0x42f53c;
 loop { match pc {
 0x42f53c => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f579; continue; }
  pc = 0x42f542; continue;
 }
 0x42f542 => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42f579; continue; }
  pc = 0x42f547; continue;
 }
 0x42f547 => {
  { cpu.sub32(cpu.eax, 0x4ba552u32); }
  if cpu.cf { pc = 0x42f579; continue; }
  pc = 0x42f54e; continue;
 }
 0x42f54e => {
  { cpu.sub32(cpu.eax, 0x4c3b53u32); }
  if !cpu.cf { pc = 0x42f579; continue; }
  pc = 0x42f555; continue;
 }
 0x42f555 => {
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42f55f; continue;
 }
 0x42f55f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x42f56e; continue; }
  pc = 0x42f565; continue;
 }
 0x42f565 => {
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if cpu.zf { pc = 0x42f56e; continue; }
  pc = 0x42f569; continue;
 }
 0x42f569 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if cpu.zf { pc = 0x42f579; continue; }
  pc = 0x42f56e; continue;
 }
 0x42f56e => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f55f; continue; }
  pc = 0x42f577; continue;
 }
 0x42f577 => {
  pc = 0x42f57a; continue;
 }
 0x42f579 => {
  {  }
  pc = 0x42f57a; continue;
 }
 0x42f57a => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f580); }
 let mut pc: u32 = 0x42f580;
 loop { match pc {
 0x42f580 => {
  { let v: u32 = 0x59c614u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x4b0u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x4ba552u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x42f59d; continue;
 }
 0x42f59d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f5ba; continue; }
  pc = 0x42f5a5; continue;
 }
 0x42f5a5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x42f5c1; continue; }
  pc = 0x42f5a9; continue;
 }
 0x42f5a9 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4ba552u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x59c614u32.wrapping_add(cpu.eax), v); }
  pc = 0x42f5c1; continue;
 }
 0x42f5ba => {
  { let v: u8 = 0x1u8; cpu.w8(0x59c614u32.wrapping_add(cpu.ebx), v); }
  pc = 0x42f5c1; continue;
 }
 0x42f5c1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f59d; continue; }
  pc = 0x42f5c8; continue;
 }
 0x42f5c8 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42f5d2; continue;
 }
 0x42f5d2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x42f5f5; continue; }
  pc = 0x42f5d9; continue;
 }
 0x42f5d9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x42f5f5; continue; }
  pc = 0x42f5e1; continue;
 }
 0x42f5e1 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f5f5; continue; }
  pc = 0x42f5e6; continue;
 }
 0x42f5e6 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4ba552u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x59c614u32.wrapping_add(cpu.eax), v); }
  pc = 0x42f5f5; continue;
 }
 0x42f5f5 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f5d2; continue; }
  pc = 0x42f5fe; continue;
 }
 0x42f5fe => {
  { let v: u32 = 0x4ba552u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  { let v: u32 = 0x59c614u32; cpu.ebx = v; }
  pc = 0x42f60d; continue;
 }
 0x42f60d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x42f619; continue; }
  pc = 0x42f612; continue;
 }
 0x42f612 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x42f619; continue;
 }
 0x42f619 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f60d; continue; }
  pc = 0x42f620; continue;
 }
 0x42f620 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f621(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f621); }
 let mut pc: u32 = 0x42f621;
 loop { match pc {
 0x42f621 => {
  { cpu.sub8(cpu.r8(0x59d9deu32), 0x0u8); }
  if cpu.zf { pc = 0x42f6c1; continue; }
  pc = 0x42f62e; continue;
 }
 0x42f62e => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x42f6c1; continue; }
  pc = 0x42f63b; continue;
 }
 0x42f63b => {
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x524b70u32); cpu.eax = v; }
  { let v = cpu.r32(0x4d67f8u32); cpu.mul32(v); }
  { let v: u32 = 0xddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x42f65f); f_0041b74b(cpu); if cpu.returned_to != 0x42f65f { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5d3a10u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x524b70u32)); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x42f6ab; continue; }
  pc = 0x42f682; continue;
 }
 0x42f682 => {
  { let v: u32 = cpu.r32(0x524b70u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x42f6a0); f_0041b74b(cpu); if cpu.returned_to != 0x42f6a0 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x42f6ab; continue;
 }
 0x42f6ab => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x42f6c1); f_00447fe3(cpu); if cpu.returned_to != 0x42f6c1 { return; }
  pc = 0x42f6c1; continue;
 }
 0x42f6c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f6c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f6c2); }
 let mut pc: u32 = 0x42f6c2;
 loop { match pc {
 0x42f6c2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x578d5fu32, v); }
  { let v: u32 = 0x578d7eu32; cpu.ebp = v; }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x578d0fu32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x42f6dd; continue;
 }
 0x42f6dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x42f774; continue; }
  pc = 0x42f6ea; continue;
 }
 0x42f6ea => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42f76a; continue; }
  pc = 0x42f6fb; continue;
 }
 0x42f6fb => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x42f76a; continue; }
  pc = 0x42f705; continue;
 }
 0x42f705 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x42f76a; continue; }
  pc = 0x42f70b; continue;
 }
 0x42f70b => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x42f71b); f_00420989(cpu); if cpu.returned_to != 0x42f71b { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x42f72e; continue; }
  pc = 0x42f728; continue;
 }
 0x42f728 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x42f72e; continue; }
  pc = 0x42f72c; continue;
 }
 0x42f72c => {
  pc = 0x42f76a; continue;
 }
 0x42f72e => {
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
  { let v: u8 = cpu.inc8(cpu.r8(0x578d5fu32)); cpu.w8(0x578d5fu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42f76a; continue;
 }
 0x42f76a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f6dd; continue; }
  pc = 0x42f774; continue;
 }
 0x42f774 => {
  { let v: u32 = 0x578d0fu32; cpu.w32(0x59daccu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x578d60u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dad0u32, v); }
  { cpu.sub8(cpu.r8(0x578d5fu32), 0x1u8); }
  if cpu.zf { pc = 0x42f7a7; continue; }
  pc = 0x42f794; continue;
 }
 0x42f794 => {
  { cpu.sub8(cpu.r8(0x578d5fu32), 0x0u8); }
  if !cpu.zf { pc = 0x42f7a8; continue; }
  pc = 0x42f79d; continue;
 }
 0x42f79d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59daccu32, v); }
  pc = 0x42f7a7; continue;
 }
 0x42f7a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f7a8 => {
  { let v: u8 = cpu.r8(0x578d5fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42f7ae; continue;
 }
 0x42f7ae => {
  { let v: u32 = 0x578d0fu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x578d5fu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42f7bb; continue;
 }
 0x42f7bb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42f7d0; continue; }
  pc = 0x42f7c3; continue;
 }
 0x42f7c3 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42f7d0; continue;
 }
 0x42f7d0 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x42f7bb; continue; }
  pc = 0x42f7d7; continue;
 }
 0x42f7d7 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42f7ae; continue; }
  pc = 0x42f7db; continue;
 }
 0x42f7db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f7dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f7dc); }
 let mut pc: u32 = 0x42f7dc;
 loop { match pc {
 0x42f7dc => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42f7e6; continue; }
  pc = 0x42f7e5; continue;
 }
 0x42f7e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f7e6 => {
  { cpu.sub8(cpu.r8(0x578d60u32), 0x0u8); }
  if cpu.zf { pc = 0x42f7f4; continue; }
  pc = 0x42f7ef; continue;
 }
 0x42f7ef => {
  cpu.push(0x42f7f4); f_0042f6c2(cpu); if cpu.returned_to != 0x42f7f4 { return; }
  pc = 0x42f7f4; continue;
 }
 0x42f7f4 => {
  { let v: u32 = cpu.r32(0x59daccu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f859; continue; }
  pc = 0x42f803; continue;
 }
 0x42f803 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x42f80d); f_00430178(cpu); if cpu.returned_to != 0x42f80d { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42f828; continue; }
  pc = 0x42f81b; continue;
 }
 0x42f81b => {
  if cpu.sf { pc = 0x42f828; continue; }
  pc = 0x42f81d; continue;
 }
 0x42f81d => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x42f828; continue; }
  pc = 0x42f824; continue;
 }
 0x42f824 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x42f828; continue;
 }
 0x42f828 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x42f833); f_0041a8a2(cpu); if cpu.returned_to != 0x42f833 { return; }
  { let v: u32 = cpu.r32(0x59daccu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dad0u32), cpu.edi); }
  if !cpu.zf { pc = 0x42f851; continue; }
  pc = 0x42f844; continue;
 }
 0x42f844 => {
  { let v: u32 = 0x578d0fu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x59daccu32, v); }
  pc = 0x42f857; continue;
 }
 0x42f851 => {
  { let v: u32 = cpu.edi; cpu.w32(0x59daccu32, v); }
  pc = 0x42f857; continue;
 }
 0x42f857 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f859 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f85b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f85b); }
 let mut pc: u32 = 0x42f85b;
 loop { match pc {
 0x42f85b => {
  { let v: u8 = 0x0u8; cpu.w8(0x578d65u32, v); }
  { let v: u32 = 0x578d7eu32; cpu.ebp = v; }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x578cbfu32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x42f876; continue;
 }
 0x42f876 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x42f8f3; continue; }
  pc = 0x42f883; continue;
 }
 0x42f883 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42f8ed; continue; }
  pc = 0x42f890; continue;
 }
 0x42f890 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x42f8ed; continue; }
  pc = 0x42f896; continue;
 }
 0x42f896 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x42f8ed; continue; }
  pc = 0x42f89c; continue;
 }
 0x42f89c => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x42f8ac); f_004712e3(cpu); if cpu.returned_to != 0x42f8ac { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x42f8b1; continue; }
  pc = 0x42f8af; continue;
 }
 0x42f8af => {
  pc = 0x42f8ed; continue;
 }
 0x42f8b1 => {
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
  { let v: u8 = cpu.inc8(cpu.r8(0x578d65u32)); cpu.w8(0x578d65u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42f8ed; continue;
 }
 0x42f8ed => {
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f876; continue; }
  pc = 0x42f8f3; continue;
 }
 0x42f8f3 => {
  { let v: u32 = 0x578cbfu32; cpu.w32(0x59dab8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d97au32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dac0u32, v); }
  { cpu.sub8(cpu.r8(0x578d65u32), 0x1u8); }
  if cpu.zf { pc = 0x42f926; continue; }
  pc = 0x42f913; continue;
 }
 0x42f913 => {
  { cpu.sub8(cpu.r8(0x578d65u32), 0x0u8); }
  if !cpu.zf { pc = 0x42f927; continue; }
  pc = 0x42f91c; continue;
 }
 0x42f91c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59dab8u32, v); }
  pc = 0x42f926; continue;
 }
 0x42f926 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f927 => {
  { let v: u8 = cpu.r8(0x578d65u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42f92d; continue;
 }
 0x42f92d => {
  { let v: u32 = 0x578cbfu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x578d65u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42f93a; continue;
 }
 0x42f93a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42f94f; continue; }
  pc = 0x42f942; continue;
 }
 0x42f942 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42f94f; continue;
 }
 0x42f94f => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x42f93a; continue; }
  pc = 0x42f956; continue;
 }
 0x42f956 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42f92d; continue; }
  pc = 0x42f95a; continue;
 }
 0x42f95a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f95b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f95b); }
 let mut pc: u32 = 0x42f95b;
 loop { match pc {
 0x42f95b => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42f965; continue; }
  pc = 0x42f964; continue;
 }
 0x42f964 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f965 => {
  { cpu.sub8(cpu.r8(0x59d97au32), 0x0u8); }
  if cpu.zf { pc = 0x42f973; continue; }
  pc = 0x42f96e; continue;
 }
 0x42f96e => {
  cpu.push(0x42f973); f_0042f85b(cpu); if cpu.returned_to != 0x42f973 { return; }
  pc = 0x42f973; continue;
 }
 0x42f973 => {
  { let v: u32 = cpu.r32(0x59dab8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x42f9d8; continue; }
  pc = 0x42f982; continue;
 }
 0x42f982 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x42f98c); f_00430178(cpu); if cpu.returned_to != 0x42f98c { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42f9a7; continue; }
  pc = 0x42f99a; continue;
 }
 0x42f99a => {
  if cpu.sf { pc = 0x42f9a7; continue; }
  pc = 0x42f99c; continue;
 }
 0x42f99c => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x42f9a7; continue; }
  pc = 0x42f9a3; continue;
 }
 0x42f9a3 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x42f9a7; continue;
 }
 0x42f9a7 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x42f9b2); f_0041a8a2(cpu); if cpu.returned_to != 0x42f9b2 { return; }
  { let v: u32 = cpu.r32(0x59dab8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dac0u32), cpu.edi); }
  if !cpu.zf { pc = 0x42f9d0; continue; }
  pc = 0x42f9c3; continue;
 }
 0x42f9c3 => {
  { let v: u32 = 0x578cbfu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x59dab8u32, v); }
  pc = 0x42f9d6; continue;
 }
 0x42f9d0 => {
  { let v: u32 = cpu.edi; cpu.w32(0x59dab8u32, v); }
  pc = 0x42f9d6; continue;
 }
 0x42f9d6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42f9d8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042f9da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42f9da); }
 let mut pc: u32 = 0x42f9da;
 loop { match pc {
 0x42f9da => {
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x576e9fu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42f9f0; continue;
 }
 0x42f9f0 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42fa67; continue; }
  pc = 0x42fa01; continue;
 }
 0x42fa01 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x42fa67; continue; }
  pc = 0x42fa0b; continue;
 }
 0x42fa0b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x42fa67; continue; }
  pc = 0x42fa11; continue;
 }
 0x42fa11 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x42fa2b; continue; }
  pc = 0x42fa17; continue;
 }
 0x42fa17 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x42fa2b; continue; }
  pc = 0x42fa1d; continue;
 }
 0x42fa1d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x42fa2b; continue; }
  pc = 0x42fa23; continue;
 }
 0x42fa23 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x42fa2b; continue; }
  pc = 0x42fa29; continue;
 }
 0x42fa29 => {
  pc = 0x42fa67; continue;
 }
 0x42fa2b => {
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42fa67; continue;
 }
 0x42fa67 => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42f9f0; continue; }
  pc = 0x42fa70; continue;
 }
 0x42fa70 => {
  { let v: u32 = 0x576e9fu32; cpu.w32(0x59daa4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d977u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59daa8u32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x1u8); }
  if cpu.zf { pc = 0x42faa3; continue; }
  pc = 0x42fa90; continue;
 }
 0x42fa90 => {
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if !cpu.zf { pc = 0x42faa4; continue; }
  pc = 0x42fa99; continue;
 }
 0x42fa99 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59daa4u32, v); }
  pc = 0x42faa3; continue;
 }
 0x42faa3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42faa4 => {
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42faaa; continue;
 }
 0x42faaa => {
  { let v: u32 = 0x576e9fu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42fab7; continue;
 }
 0x42fab7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42facc; continue; }
  pc = 0x42fabf; continue;
 }
 0x42fabf => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42facc; continue;
 }
 0x42facc => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x42fab7; continue; }
  pc = 0x42fad3; continue;
 }
 0x42fad3 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42faaa; continue; }
  pc = 0x42fad7; continue;
 }
 0x42fad7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fad8); }
 let mut pc: u32 = 0x42fad8;
 loop { match pc {
 0x42fad8 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42fae2; continue; }
  pc = 0x42fae1; continue;
 }
 0x42fae1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fae2 => {
  { cpu.pushad(); }
  cpu.push(0x42fae8); f_00403030(cpu); if cpu.returned_to != 0x42fae8 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42faee; continue; }
  pc = 0x42faed; continue;
 }
 0x42faed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42faee => {
  { cpu.sub8(cpu.r8(0x59d977u32), 0x0u8); }
  if cpu.zf { pc = 0x42fafc; continue; }
  pc = 0x42faf7; continue;
 }
 0x42faf7 => {
  cpu.push(0x42fafc); f_0042f9da(cpu); if cpu.returned_to != 0x42fafc { return; }
  pc = 0x42fafc; continue;
 }
 0x42fafc => {
  { let v: u32 = cpu.r32(0x59daa4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x42fb67; continue; }
  pc = 0x42fb0b; continue;
 }
 0x42fb0b => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x42fb15); f_00430178(cpu); if cpu.returned_to != 0x42fb15 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42fb30; continue; }
  pc = 0x42fb23; continue;
 }
 0x42fb23 => {
  if cpu.sf { pc = 0x42fb30; continue; }
  pc = 0x42fb25; continue;
 }
 0x42fb25 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x42fb30; continue; }
  pc = 0x42fb2c; continue;
 }
 0x42fb2c => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x42fb30; continue;
 }
 0x42fb30 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x42fb42); f_0041a8a2(cpu); if cpu.returned_to != 0x42fb42 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  { let v: u32 = cpu.r32(0x59daa4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59daa8u32), cpu.edi); }
  if !cpu.zf { pc = 0x42fb5f; continue; }
  pc = 0x42fb5a; continue;
 }
 0x42fb5a => {
  { let v: u32 = 0x576e9fu32; cpu.edi = v; }
  pc = 0x42fb5f; continue;
 }
 0x42fb5f => {
  { let v: u32 = cpu.edi; cpu.w32(0x59daa4u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fb67 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fb69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fb69); }
 let mut pc: u32 = 0x42fb69;
 loop { match pc {
 0x42fb69 => {
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x577627u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42fb7f; continue;
 }
 0x42fb7f => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42fbda; continue; }
  pc = 0x42fb8c; continue;
 }
 0x42fb8c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x42fbda; continue; }
  pc = 0x42fb92; continue;
 }
 0x42fb92 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x42fbda; continue; }
  pc = 0x42fb98; continue;
 }
 0x42fb98 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x42fbda; continue; }
  pc = 0x42fb9e; continue;
 }
 0x42fb9e => {
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42fbda; continue;
 }
 0x42fbda => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42fb7f; continue; }
  pc = 0x42fbe2; continue;
 }
 0x42fbe2 => {
  { let v: u32 = 0x577627u32; cpu.w32(0x59daacu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d979u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dab0u32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x1u8); }
  if cpu.zf { pc = 0x42fc15; continue; }
  pc = 0x42fc02; continue;
 }
 0x42fc02 => {
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if !cpu.zf { pc = 0x42fc16; continue; }
  pc = 0x42fc0b; continue;
 }
 0x42fc0b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59daacu32, v); }
  pc = 0x42fc15; continue;
 }
 0x42fc15 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fc16 => {
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42fc1c; continue;
 }
 0x42fc1c => {
  { let v: u32 = 0x577627u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42fc29; continue;
 }
 0x42fc29 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42fc3e; continue; }
  pc = 0x42fc31; continue;
 }
 0x42fc31 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42fc3e; continue;
 }
 0x42fc3e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x42fc29; continue; }
  pc = 0x42fc45; continue;
 }
 0x42fc45 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42fc1c; continue; }
  pc = 0x42fc49; continue;
 }
 0x42fc49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fc4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fc4a); }
 let mut pc: u32 = 0x42fc4a;
 loop { match pc {
 0x42fc4a => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42fc54; continue; }
  pc = 0x42fc53; continue;
 }
 0x42fc53 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fc54 => {
  { cpu.pushad(); }
  cpu.push(0x42fc5a); f_00403030(cpu); if cpu.returned_to != 0x42fc5a { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42fc60; continue; }
  pc = 0x42fc5f; continue;
 }
 0x42fc5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fc60 => {
  { cpu.sub8(cpu.r8(0x59d979u32), 0x0u8); }
  if cpu.zf { pc = 0x42fc6e; continue; }
  pc = 0x42fc69; continue;
 }
 0x42fc69 => {
  cpu.push(0x42fc6e); f_0042fb69(cpu); if cpu.returned_to != 0x42fc6e { return; }
  pc = 0x42fc6e; continue;
 }
 0x42fc6e => {
  { let v: u32 = cpu.r32(0x59daacu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x42fcd9; continue; }
  pc = 0x42fc7d; continue;
 }
 0x42fc7d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x42fc87); f_00430178(cpu); if cpu.returned_to != 0x42fc87 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42fca2; continue; }
  pc = 0x42fc95; continue;
 }
 0x42fc95 => {
  if cpu.sf { pc = 0x42fca2; continue; }
  pc = 0x42fc97; continue;
 }
 0x42fc97 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x42fca2; continue; }
  pc = 0x42fc9e; continue;
 }
 0x42fc9e => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x42fca2; continue;
 }
 0x42fca2 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x42fcb4); f_0041a8a2(cpu); if cpu.returned_to != 0x42fcb4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  { let v: u32 = cpu.r32(0x59daacu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dab0u32), cpu.edi); }
  if !cpu.zf { pc = 0x42fcd1; continue; }
  pc = 0x42fccc; continue;
 }
 0x42fccc => {
  { let v: u32 = 0x577627u32; cpu.edi = v; }
  pc = 0x42fcd1; continue;
 }
 0x42fcd1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x59daacu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fcd9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fcdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fcdb); }
 let mut pc: u32 = 0x42fcdb;
 loop { match pc {
 0x42fcdb => {
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0x577dafu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x42fcf1; continue;
 }
 0x42fcf1 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42fd4c; continue; }
  pc = 0x42fcfe; continue;
 }
 0x42fcfe => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x42fd4c; continue; }
  pc = 0x42fd04; continue;
 }
 0x42fd04 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x42fd4c; continue; }
  pc = 0x42fd0a; continue;
 }
 0x42fd0a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x42fd4c; continue; }
  pc = 0x42fd10; continue;
 }
 0x42fd10 => {
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42fd4c; continue;
 }
 0x42fd4c => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x42fcf1; continue; }
  pc = 0x42fd54; continue;
 }
 0x42fd54 => {
  { let v: u32 = 0x577dafu32; cpu.w32(0x59dab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d978u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dabcu32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x1u8); }
  if cpu.zf { pc = 0x42fd87; continue; }
  pc = 0x42fd74; continue;
 }
 0x42fd74 => {
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if !cpu.zf { pc = 0x42fd88; continue; }
  pc = 0x42fd7d; continue;
 }
 0x42fd7d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59dab4u32, v); }
  pc = 0x42fd87; continue;
 }
 0x42fd87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fd88 => {
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42fd8e; continue;
 }
 0x42fd8e => {
  { let v: u32 = 0x577dafu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42fd9b; continue;
 }
 0x42fd9b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42fdb0; continue; }
  pc = 0x42fda3; continue;
 }
 0x42fda3 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42fdb0; continue;
 }
 0x42fdb0 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x42fd9b; continue; }
  pc = 0x42fdb7; continue;
 }
 0x42fdb7 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42fd8e; continue; }
  pc = 0x42fdbb; continue;
 }
 0x42fdbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fdbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fdbc); }
 let mut pc: u32 = 0x42fdbc;
 loop { match pc {
 0x42fdbc => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42fdc6; continue; }
  pc = 0x42fdc5; continue;
 }
 0x42fdc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fdc6 => {
  { cpu.pushad(); }
  cpu.push(0x42fdcc); f_00403030(cpu); if cpu.returned_to != 0x42fdcc { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42fdd2; continue; }
  pc = 0x42fdd1; continue;
 }
 0x42fdd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fdd2 => {
  { cpu.sub8(cpu.r8(0x59d978u32), 0x0u8); }
  if cpu.zf { pc = 0x42fde0; continue; }
  pc = 0x42fddb; continue;
 }
 0x42fddb => {
  cpu.push(0x42fde0); f_0042fcdb(cpu); if cpu.returned_to != 0x42fde0 { return; }
  pc = 0x42fde0; continue;
 }
 0x42fde0 => {
  { let v: u32 = cpu.r32(0x59dab4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x42fe47; continue; }
  pc = 0x42fdeb; continue;
 }
 0x42fdeb => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x42fdf5); f_00430178(cpu); if cpu.returned_to != 0x42fdf5 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x42fe10; continue; }
  pc = 0x42fe03; continue;
 }
 0x42fe03 => {
  if cpu.sf { pc = 0x42fe10; continue; }
  pc = 0x42fe05; continue;
 }
 0x42fe05 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x42fe10; continue; }
  pc = 0x42fe0c; continue;
 }
 0x42fe0c => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x42fe10; continue;
 }
 0x42fe10 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x42fe22); f_0041a8a2(cpu); if cpu.returned_to != 0x42fe22 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  { let v: u32 = cpu.r32(0x59dab4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dabcu32), cpu.edi); }
  if !cpu.zf { pc = 0x42fe3f; continue; }
  pc = 0x42fe3a; continue;
 }
 0x42fe3a => {
  { let v: u32 = 0x577dafu32; cpu.edi = v; }
  pc = 0x42fe3f; continue;
 }
 0x42fe3f => {
  { let v: u32 = cpu.edi; cpu.w32(0x59dab4u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42fe47 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042fe49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x42fe49); }
 let mut pc: u32 = 0x42fe49;
 loop { match pc {
 0x42fe49 => {
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0x578cbfu32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x42fe5f; continue;
 }
 0x42fe5f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe6a; continue;
 }
 0x42fe6a => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe72; continue;
 }
 0x42fe72 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x42fe7f; continue; }
  pc = 0x42fe76; continue;
 }
 0x42fe76 => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe7f; continue;
 }
 0x42fe7f => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe87; continue;
 }
 0x42fe87 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe92; continue;
 }
 0x42fe92 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fe9a; continue;
 }
 0x42fe9a => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x42fedb; continue; }
  pc = 0x42fea0; continue;
 }
 0x42fea0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59da9bu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x59da9fu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x42fedb; continue;
 }
 0x42fedb => {
  { let v: u32 = cpu.esi.wrapping_add(0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x42fe5f; continue; }
  pc = 0x42fee5; continue;
 }
 0x42fee5 => {
  { let v: u32 = 0x578cbfu32; cpu.w32(0x59dab8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d97au32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dac0u32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x1u8); }
  if cpu.zf { pc = 0x42ff18; continue; }
  pc = 0x42ff05; continue;
 }
 0x42ff05 => {
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if !cpu.zf { pc = 0x42ff19; continue; }
  pc = 0x42ff0e; continue;
 }
 0x42ff0e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59dab8u32, v); }
  pc = 0x42ff18; continue;
 }
 0x42ff18 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42ff19 => {
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x42ff1f; continue;
 }
 0x42ff1f => {
  { let v: u32 = 0x578cbfu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x42ff2c; continue;
 }
 0x42ff2c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x42ff41; continue; }
  pc = 0x42ff34; continue;
 }
 0x42ff34 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x42ff41; continue;
 }
 0x42ff41 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x42ff4a; continue; }
  pc = 0x42ff48; continue;
 }
 0x42ff48 => {
  cpu.poll(); pc = 0x42ff2c; continue;
 }
 0x42ff4a => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x42ff1f; continue; }
  pc = 0x42ff4e; continue;
 }
 0x42ff4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0042ff4f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42ff4f); } r_0042ff4f(cpu, 0x42ff4f); }
pub fn f_0042ffff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42ffff); } r_0042ff4f(cpu, 0x42ffff); }
pub fn f_00430000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430000); } r_0042ff4f(cpu, 0x430000); }
fn r_0042ff4f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x42ff4f => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x42ff59; continue; }
  pc = 0x42ff58; continue;
 }
 0x42ff58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42ff59 => {
  { cpu.pushad(); }
  cpu.push(0x42ff5f); f_00403030(cpu); if cpu.returned_to != 0x42ff5f { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x42ff65; continue; }
  pc = 0x42ff64; continue;
 }
 0x42ff64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x42ff65 => {
  { cpu.sub8(cpu.r8(0x59d97au32), 0x0u8); }
  if cpu.zf { pc = 0x42ff73; continue; }
  pc = 0x42ff6e; continue;
 }
 0x42ff6e => {
  cpu.push(0x42ff73); f_0042fe49(cpu); if cpu.returned_to != 0x42ff73 { return; }
  pc = 0x42ff73; continue;
 }
 0x42ff73 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x42ff7d); f_00430178(cpu); if cpu.returned_to != 0x42ff7d { return; }
  { let v: u32 = cpu.r32(0x59dab8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x430009; continue; }
  pc = 0x42ff8c; continue;
 }
 0x42ff8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  cpu.push(0x42ffa9); f_00471ec2(cpu); if cpu.returned_to != 0x42ffa9 { return; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x42ffc5; continue; }
  pc = 0x42ffb5; continue;
 }
 0x42ffb5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c465u32, v); }
  { cpu.sub8(cpu.r8(0x58c838u32), 0x0u8); }
  if !cpu.zf { pc = 0x42ffe3; continue; }
  pc = 0x42ffc5; continue;
 }
 0x42ffc5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c465u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x42ffd2); f_00419d49(cpu); if cpu.returned_to != 0x42ffd2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.r16(0x4d675cu32).wrapping_sub(0x20u16); cpu.w16(0x4d675cu32, v); }
  pc = 0x42ffe3; continue;
 }
 0x42ffe3 => {
  cpu.push(0x42ffe8); f_0044cbe1(cpu); if cpu.returned_to != 0x42ffe8 { return; }
  cpu.push(0x42ffed); f_0041879e(cpu); if cpu.returned_to != 0x42ffed { return; }
  { let v: u32 = cpu.r32(0x59dab8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dac0u32), cpu.edi); }
  if !cpu.zf { pc = 0x430003; continue; }
  pc = 0x42fffe; continue;
 }
 0x42fffe => {
  { let v: u32 = 0x578cbfu32; cpu.edi = v; }
  pc = 0x430003; continue;
 }
 0x42ffff => {
  { let v: u32 = 0x8900578cu32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x59dab8u32); }
  pc = 0x430009; continue;
 }
 0x430000 => {
  { let v: u16 = 0x2bu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x430003; continue;
 }
 0x430003 => {
  { let v: u32 = cpu.edi; cpu.w32(0x59dab8u32, v); }
  pc = 0x430009; continue;
 }
 0x430009 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430001); } r_00430001(cpu, 0x430001); }
pub fn f_00430002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430002); } r_00430001(cpu, 0x430002); }
pub fn f_00430007(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430007); } r_00430001(cpu, 0x430007); }
fn r_00430001(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x430001 => {
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x430002; continue;
 }
 0x430002 => {
  { let v: u8 = cpu.r8(0x59dab83du32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x59dab83du32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  f_0043000a(cpu); return;
 }
 0x430007 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  f_0043000a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043000a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43000a); } r_0043000a(cpu, 0x43000a); }
pub fn f_00430010(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430010); } r_0043000a(cpu, 0x430010); }
pub fn f_00430013(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430013); } r_0043000a(cpu, 0x430013); }
pub fn f_0043001c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43001c); } r_0043000a(cpu, 0x43001c); }
pub fn f_0043001f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43001f); } r_0043000a(cpu, 0x43001f); }
pub fn f_00430020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430020); } r_0043000a(cpu, 0x430020); }
pub fn f_00430025(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430025); } r_0043000a(cpu, 0x430025); }
pub fn f_00430026(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430026); } r_0043000a(cpu, 0x430026); }
pub fn f_0043002d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43002d); } r_0043000a(cpu, 0x43002d); }
pub fn f_00430034(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430034); } r_0043000a(cpu, 0x430034); }
pub fn f_00430042(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430042); } r_0043000a(cpu, 0x430042); }
pub fn f_00430043(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430043); } r_0043000a(cpu, 0x430043); }
pub fn f_00430044(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430044); } r_0043000a(cpu, 0x430044); }
pub fn f_0043004c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43004c); } r_0043000a(cpu, 0x43004c); }
pub fn f_0043004d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43004d); } r_0043000a(cpu, 0x43004d); }
pub fn f_0043005b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43005b); } r_0043000a(cpu, 0x43005b); }
pub fn f_0043006c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43006c); } r_0043000a(cpu, 0x43006c); }
pub fn f_00430074(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430074); } r_0043000a(cpu, 0x430074); }
pub fn f_00430080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430080); } r_0043000a(cpu, 0x430080); }
pub fn f_00430082(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430082); } r_0043000a(cpu, 0x430082); }
pub fn f_00430083(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430083); } r_0043000a(cpu, 0x430083); }
pub fn f_00430089(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x430089); } r_0043000a(cpu, 0x430089); }
fn r_0043000a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43000a => {
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x576e88u32, v); }
  cpu.push(0x430020); f_004301c7(cpu); if cpu.returned_to != 0x430020 { return; }
  pc = 0x430020; continue;
 }
 0x430010 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x576e88u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x430020); f_004301c7(cpu); if cpu.returned_to != 0x430020 { return; }
  pc = 0x430020; continue;
 }
 0x430013 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x57u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43001c; continue;
 }
 0x43001c => {
  { cpu.cmps(4, 0); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x43001f; continue;
 }
 0x43001f => {
  { let v: u8 = cpu.r8(0x509290u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x509290u32.wrapping_add(cpu.esi), v); }
  pc = 0x430025; continue;
 }
 0x430020 => {
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  pc = 0x430025; continue;
 }
 0x430025 => {
  { let v: u32 = 0x578537u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x43002f; continue;
 }
 0x430026 => {
  cpu.unimplemented(0x430026, "Aaa");
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.edx); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x43002f; continue;
 }
 0x43002d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x43002f; continue;
 }
 0x43002f => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x430034; continue;
 }
 0x430034 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4300a3; continue; }
  pc = 0x430040; continue;
 }
 0x430040 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  pc = 0x430044; continue;
 }
 0x430042 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2))) ^ cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x430046; continue;
 }
 0x430043 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x74u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x430046; continue;
 }
 0x430044 => {
  if cpu.zf { pc = 0x4300a3; continue; }
  pc = 0x430046; continue;
 }
 0x430046 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x4300a3; continue; }
  pc = 0x43004c; continue;
 }
 0x43004c => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x430054); f_004301dd(cpu); if cpu.returned_to != 0x430054 { return; }
  if cpu.cf { pc = 0x4300a3; continue; }
  pc = 0x430056; continue;
 }
 0x43004d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x430054); f_004301dd(cpu); if cpu.returned_to != 0x430054 { return; }
  if cpu.cf { pc = 0x4300a3; continue; }
  pc = 0x430056; continue;
 }
 0x430056 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x59f349u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x43005b => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), 70u32); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x59da9bu32); cpu.eax = v; }
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x43006c => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xfffffff7u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x430074); cpu.call(0x6653e335); if cpu.returned_to != 0x430074 { return; }
  pc = 0x430074; continue;
 }
 0x430074 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
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
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x430080 => {
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let a = cpu.st(0); let b = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fcom(a, b); cpu.fpop(); }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x430082 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x430083; continue;
 }
 0x430083 => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xfffffff7u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x43008b); cpu.call(0x6653e34c); if cpu.returned_to != 0x43008b { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x430089 => {
  { let v: u8 = cpu.adc8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 1u32); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.st(0) * (cpu.r32(0x45f8937u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.set_st(0, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x576e88u32)); cpu.w32(0x576e88u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x4300a3; continue;
 }
 0x4300a3 => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43002f; continue; }
  pc = 0x4300ac; continue;
 }
 0x4300ac => {
  { let v: u32 = 0x578537u32; cpu.w32(0x59dac4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d97bu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x59dac8u32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x1u8); }
  if cpu.zf { pc = 0x4300df; continue; }
  pc = 0x4300cc; continue;
 }
 0x4300cc => {
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if !cpu.zf { pc = 0x4300e0; continue; }
  pc = 0x4300d5; continue;
 }
 0x4300d5 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x59dac4u32, v); }
  pc = 0x4300df; continue;
 }
 0x4300df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4300e0 => {
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4300e6; continue;
 }
 0x4300e6 => {
  { let v: u32 = 0x578537u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x59daa3u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4300f3; continue;
 }
 0x4300f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x430108; continue; }
  pc = 0x4300fb; continue;
 }
 0x4300fb => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x430108; continue;
 }
 0x430108 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x4300f3; continue; }
  pc = 0x43010f; continue;
 }
 0x43010f => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4300e6; continue; }
  pc = 0x430113; continue;
 }
 0x430113 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430114); }
 let mut pc: u32 = 0x430114;
 loop { match pc {
 0x430114 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x43012f; continue; }
  pc = 0x430120; continue;
 }
 0x430120 => {
  { let v: u32 = 0x59f349u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x430131; continue; }
  pc = 0x43012d; continue;
 }
 0x43012d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43012f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x430131; continue;
 }
 0x430131 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430133); }
 let mut pc: u32 = 0x430133;
 loop { match pc {
 0x430133 => {
  { cpu.pushad(); }
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x43013e; continue;
 }
 0x43013e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x430166; continue; }
  pc = 0x43014b; continue;
 }
 0x43014b => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x430166; continue; }
  pc = 0x430151; continue;
 }
 0x430151 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x430166; continue; }
  pc = 0x430157; continue;
 }
 0x430157 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c6e1u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x581b5au32, v); }
  pc = 0x430176; continue;
 }
 0x430166 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43013e; continue; }
  pc = 0x43016f; continue;
 }
 0x43016f => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c6e1u32, v); }
  pc = 0x430176; continue;
 }
 0x430176 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430178); }
 let mut pc: u32 = 0x430178;
 loop { match pc {
 0x430178 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x43017e; continue; }
  pc = 0x43017d; continue;
 }
 0x43017d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43017e => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x43018a; continue; }
  pc = 0x430183; continue;
 }
 0x430183 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d979u32, v); }
  pc = 0x43018a; continue;
 }
 0x43018a => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x430196; continue; }
  pc = 0x43018f; continue;
 }
 0x43018f => {
  { let v: u8 = 0x1u8; cpu.w8(0x578d60u32, v); }
  pc = 0x430196; continue;
 }
 0x430196 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x4301a2; continue; }
  pc = 0x43019b; continue;
 }
 0x43019b => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  pc = 0x4301a2; continue;
 }
 0x4301a2 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x4301ae; continue; }
  pc = 0x4301a7; continue;
 }
 0x4301a7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d977u32, v); }
  pc = 0x4301ae; continue;
 }
 0x4301ae => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if cpu.zf { pc = 0x4301ba; continue; }
  pc = 0x4301b3; continue;
 }
 0x4301b3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d978u32, v); }
  pc = 0x4301ba; continue;
 }
 0x4301ba => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.zf { pc = 0x4301c6; continue; }
  pc = 0x4301bf; continue;
 }
 0x4301bf => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97bu32, v); }
  pc = 0x4301c6; continue;
 }
 0x4301c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004301c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4301c7); }
 let mut pc: u32 = 0x4301c7;
 loop { match pc {
 0x4301c7 => {
  { let v: u32 = 0x59f349u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x4301d6; continue;
 }
 0x4301d6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4301d6; continue; }
  pc = 0x4301dc; continue;
 }
 0x4301dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004301dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4301dd); }
 let mut pc: u32 = 0x4301dd;
 loop { match pc {
 0x4301dd => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4301f5; continue; }
  pc = 0x4301e1; continue;
 }
 0x4301e1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x59f349u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x4301f5; continue; }
  pc = 0x4301f3; continue;
 }
 0x4301f3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4301f5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004301f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4301f7); }
 let mut pc: u32 = 0x4301f7;
 loop { match pc {
 0x4301f7 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x430201; continue; }
  pc = 0x430200; continue;
 }
 0x430200 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x430201 => {
  { cpu.sub8(cpu.r8(0x5791c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x430216; continue; }
  pc = 0x43020a; continue;
 }
 0x43020a => {
  { cpu.pushad(); }
  cpu.push(0x430210); f_00403030(cpu); if cpu.returned_to != 0x430210 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x430216; continue; }
  pc = 0x430215; continue;
 }
 0x430215 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x430216 => {
  { cpu.sub8(cpu.r8(0x59d97bu32), 0x0u8); }
  if !cpu.zf { pc = 0x43022a; continue; }
  pc = 0x43021f; continue;
 }
 0x43021f => {
  { let v: u32 = cpu.r32(0x59dac4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x43022f; continue; }
  pc = 0x43022a; continue;
 }
 0x43022a => {
  cpu.push(0x43022f); f_0043000a(cpu); if cpu.returned_to != 0x43022f { return; }
  pc = 0x43022f; continue;
 }
 0x43022f => {
  { let v: u32 = cpu.r32(0x59dac4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x4302c2; continue; }
  pc = 0x43023e; continue;
 }
 0x43023e => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.push(0x430248); f_00430178(cpu); if cpu.returned_to != 0x430248 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x59daa3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x430295; continue; }
  pc = 0x430261; continue;
 }
 0x430261 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x430295; continue; }
  pc = 0x430267; continue;
 }
 0x430267 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.eax)), 0x80u8); }
  if cpu.zf { pc = 0x430295; continue; }
  pc = 0x43026d; continue;
 }
 0x43026d => {
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x430284; continue; }
  pc = 0x430277; continue;
 }
 0x430277 => {
  if cpu.sf { pc = 0x4302c0; continue; }
  pc = 0x430279; continue;
 }
 0x430279 => {
  { cpu.sub32(cpu.eax, 0xac80u32); }
  if (cpu.sf == cpu.of) { pc = 0x430284; continue; }
  pc = 0x430280; continue;
 }
 0x430280 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x430284; continue;
 }
 0x430284 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x43028f); f_0041a8a2(cpu); if cpu.returned_to != 0x43028f { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daa3u32)); cpu.w8(0x59daa3u32, v); }
  pc = 0x430295; continue;
 }
 0x430295 => {
  { let v: u32 = cpu.r32(0x59dac4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x59dac8u32), cpu.edi); }
  if !cpu.zf { pc = 0x4302ab; continue; }
  pc = 0x4302a6; continue;
 }
 0x4302a6 => {
  { let v: u32 = 0x578537u32; cpu.edi = v; }
  pc = 0x4302ab; continue;
 }
 0x4302ab => {
  { let v: u32 = cpu.edi; cpu.w32(0x59dac4u32, v); }
  { cpu.sub8(cpu.r8(0x59daa3u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x43021f; continue; }
  pc = 0x4302be; continue;
 }
 0x4302be => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4302c0 => {
  cpu.poll(); pc = 0x430295; continue;
 }
 0x4302c2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004302c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4302c4); }
 let mut pc: u32 = 0x4302c4;
 loop { match pc {
 0x4302c4 => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xaau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59c60eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004302d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4302d4); }
 let mut pc: u32 = 0x4302d4;
 loop { match pc {
 0x4302d4 => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x59c60eu32), cpu.eax); }
  if !cpu.zf { pc = 0x4302f5; continue; }
  pc = 0x4302e1; continue;
 }
 0x4302e1 => {
  { let v: u32 = cpu.add32(cpu.eax, 0xaau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59c60eu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.push(0x4302f5); f_00430178(cpu); if cpu.returned_to != 0x4302f5 { return; }
  pc = 0x4302f5; continue;
 }
 0x4302f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004302f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4302f6); }
 let mut pc: u32 = 0x4302f6;
 loop { match pc {
 0x4302f6 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d693cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x59dafdu32, v); }
  cpu.push(0x430327); f_00472452(cpu); if cpu.returned_to != 0x430327 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430330); }
 let mut pc: u32 = 0x430330;
 loop { match pc {
 0x430330 => {
  { cpu.sub8(cpu.r8(0x5791c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x430346; continue; }
  pc = 0x430339; continue;
 }
 0x430339 => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if cpu.zf { pc = 0x430396; continue; }
  pc = 0x430346; continue;
 }
 0x430346 => {
  { cpu.sub8(cpu.r8(0x59db02u32), 0xffu8); }
  if cpu.zf { pc = 0x4303a0; continue; }
  pc = 0x430353; continue;
 }
 0x430353 => {
  { let v: u32 = cpu.r32(0x59db03u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x59db23u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x59db0bu32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x59db02u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x430396; continue; }
  pc = 0x430374; continue;
 }
 0x430374 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x59db03u32)); cpu.w32(0x59db03u32, v); }
  { let v: u32 = cpu.r32(0x59db03u32); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x59db23u32.wrapping_add(cpu.ecx.wrapping_mul(2))), 0x223du16); }
  if !cpu.zf { pc = 0x4303a0; continue; }
  pc = 0x43038c; continue;
 }
 0x43038c => {
  { let v: u32 = 0x1u32; cpu.w32(0x59dae5u32, v); }
  pc = 0x430396; continue;
 }
 0x430396 => {
  { let v: u32 = 0x0u32; cpu.w32(0x59db03u32, v); }
  pc = 0x4303a0; continue;
 }
 0x4303a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004303a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4303a1); }
 let mut pc: u32 = 0x4303a1;
 loop { match pc {
 0x4303a1 => {
  { cpu.sub32(cpu.r32(0x59dae5u32), 0x0u32); }
  if !cpu.zf { pc = 0x4303ab; continue; }
  pc = 0x4303aa; continue;
 }
 0x4303aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4303ab => {
  { let v: u32 = 0x59db95u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x59daf2u32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x59daeeu32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x59daf0u32, v); }
  pc = 0x4303c9; continue;
 }
 0x4303c9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x824u16); }
  if cpu.zf { pc = 0x430412; continue; }
  pc = 0x4303d5; continue;
 }
 0x4303d5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x59daf2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x59daf3u32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x59daf2u32)); cpu.w8(0x59daf2u32, v); }
  { let v: u8 = cpu.r8(0x59daf2u32) & 0x3u8; cpu.w8(0x59daf2u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x59daeeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x59daf0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x430407); f_0041cce3(cpu); if cpu.returned_to != 0x430407 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59daeeu32, v); }
  cpu.poll(); pc = 0x4303c9; continue;
 }
 0x430412 => {
  { let v: u32 = 0x42a237u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x59daeeu32).wrapping_add(0x20u16); cpu.w16(0x59daeeu32, v); }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x430424; continue;
 }
 0x430424 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x59daeeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x59daf0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x43043f); f_0041cce3(cpu); if cpu.returned_to != 0x43043f { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59daeeu32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x430424; continue; }
  pc = 0x430448; continue;
 }
 0x430448 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430449); }
 let mut pc: u32 = 0x430449;
 loop { match pc {
 0x430449 => {
  { cpu.sub8(cpu.r8(0x5791c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x43045f; continue; }
  pc = 0x430452; continue;
 }
 0x430452 => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if cpu.zf { pc = 0x4304af; continue; }
  pc = 0x43045f; continue;
 }
 0x43045f => {
  { cpu.sub8(cpu.r8(0x59db02u32), 0xffu8); }
  if cpu.zf { pc = 0x4304b9; continue; }
  pc = 0x43046c; continue;
 }
 0x43046c => {
  { let v: u32 = cpu.r32(0x59db07u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x59db41u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x59db1bu32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x59db02u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x4304af; continue; }
  pc = 0x43048d; continue;
 }
 0x43048d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x59db07u32)); cpu.w32(0x59db07u32, v); }
  { let v: u32 = cpu.r32(0x59db07u32); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x59db41u32.wrapping_add(cpu.ecx.wrapping_mul(2))), 0x223du16); }
  if !cpu.zf { pc = 0x4304b9; continue; }
  pc = 0x4304a5; continue;
 }
 0x4304a5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x59dae9u32, v); }
  pc = 0x4304af; continue;
 }
 0x4304af => {
  { let v: u32 = 0x0u32; cpu.w32(0x59db07u32, v); }
  pc = 0x4304b9; continue;
 }
 0x4304b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430540); }
 let mut pc: u32 = 0x430540;
 loop { match pc {
 0x430540 => {
  { let v: u16 = cpu.r16(0x56d2bcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x56d2bcu32)); cpu.w16(0x56d2bcu32, v); }
  { cpu.sub16(cpu.r16(0x56d2bcu32), 0x5u16); }
  if !cpu.zf { pc = 0x430561; continue; }
  pc = 0x430558; continue;
 }
 0x430558 => {
  { let v: u16 = 0x0u16; cpu.w16(0x56d2bcu32, v); }
  pc = 0x430561; continue;
 }
 0x430561 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004306e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4306e8); }
 let mut pc: u32 = 0x4306e8;
 loop { match pc {
 0x4306e8 => {
  { cpu.pushad(); }
  { let v: u32 = 0x57595fu32; cpu.esi = v; }
  { let v: u32 = 0x5759b8u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x57595bu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x430710; continue; }
  pc = 0x4306fe; continue;
 }
 0x4306fe => {
  { let v: u32 = cpu.r32(0x581c03u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x430707; continue;
 }
 0x430707 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x43070e; continue; }
  pc = 0x43070d; continue;
 }
 0x43070d => {
  { cpu.stos(1, false); }
  pc = 0x43070e; continue;
 }
 0x43070e => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x430707; continue; }
  pc = 0x430710; continue;
 }
 0x430710 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430715(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430715); }
 let mut pc: u32 = 0x430715;
 loop { match pc {
 0x430715 => {
  { cpu.pushad(); }
  { let v: u32 = 0x576113u32; cpu.edi = v; }
  { let v: u16 = 0x34u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbau16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x430739; continue; }
  pc = 0x43072c; continue;
 }
 0x43072c => {
  { let v: u16 = 0x68u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x1cu32); cpu.ebx = v; }
  pc = 0x430739; continue;
 }
 0x430739 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x576bd7u32) as u32; cpu.eax = v; }
  cpu.push(0x43074d); f_004306e8(cpu); if cpu.returned_to != 0x43074d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x5759b8u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc431u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x430786; continue; }
  pc = 0x430772; continue;
 }
 0x430772 => {
  { let v: u32 = 0x5cc43du32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x430786; continue; }
  pc = 0x430781; continue;
 }
 0x430781 => {
  { let v: u32 = 0x5cc446u32; cpu.ebp = v; }
  pc = 0x430786; continue;
 }
 0x430786 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x4307ae; continue; }
  pc = 0x43079e; continue;
 }
 0x43079e => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x4307ae; continue; }
  pc = 0x4307a6; continue;
 }
 0x4307a6 => {
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x4307ae; continue;
 }
 0x4307ae => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x430786; continue; }
  pc = 0x4307b1; continue;
 }
 0x4307b1 => {
  { let v: u8 = cpu.r8(0x59c5f6u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43080d; continue; }
  pc = 0x4307c0; continue;
 }
 0x4307c0 => {
  { let v: u32 = 0x59c60au32; cpu.eax = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x4307cd; continue; }
  pc = 0x4307c9; continue;
 }
 0x4307c9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x4307cd; continue;
 }
 0x4307cd => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x4307d5; continue; }
  pc = 0x4307d1; continue;
 }
 0x4307d1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x4307d5; continue;
 }
 0x4307d5 => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x4307dd; continue; }
  pc = 0x4307d9; continue;
 }
 0x4307d9 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x4307dd; continue;
 }
 0x4307dd => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x4307e5; continue; }
  pc = 0x4307e1; continue;
 }
 0x4307e1 => {
  { let v: u8 = 0x3u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x4307e5; continue;
 }
 0x4307e5 => {
  { let v: u32 = 0x59c60au32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59c606u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x4307fc; continue; }
  pc = 0x4307fb; continue;
 }
 0x4307fb => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x4307fc; continue;
 }
 0x4307fc => {
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if (cpu.sf != cpu.of) { pc = 0x430802; continue; }
  pc = 0x430800; continue;
 }
 0x430800 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x430802; continue;
 }
 0x430802 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x59c606u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43080f; continue;
 }
 0x43080d => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43080f; continue;
 }
 0x43080f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430814); }
 let mut pc: u32 = 0x430814;
 loop { match pc {
 0x430814 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x575959u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x57595bu32, v); }
  { let v: u32 = 0x57595fu32; cpu.edi = v; }
  { let v: u32 = 0x3du32; cpu.ecx = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x576113u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430846(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430846); }
 let mut pc: u32 = 0x430846;
 loop { match pc {
 0x430846 => {
  { cpu.pushad(); }
  { let v: u32 = 0x19u32; cpu.w32(0x57595bu32, v); }
  { let v: u32 = 0x57595fu32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x575959u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x57595bu32, v); }
  { let v: u32 = 0x57595fu32; cpu.edi = v; }
  { let v: u32 = 0x3du32; cpu.ecx = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x576113u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430893(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430893); }
 let mut pc: u32 = 0x430893;
 loop { match pc {
 0x430893 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.sf { pc = 0x4308a1; continue; }
  pc = 0x43089c; continue;
 }
 0x43089c => {
  f_0044139f(cpu); return;
 }
 0x4308a1 => {
  { cpu.sub32(cpu.r32(0x57595bu32), 0x0u32); }
  if cpu.zf { pc = 0x43095c; continue; }
  pc = 0x4308ae; continue;
 }
 0x4308ae => {
  { let v: u32 = 0x575a15u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = 0x57595fu32; cpu.esi = v; }
  { let v: u32 = 0x575a18u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x57595bu32); cpu.ecx = v; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x576bd7u32) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x581c03u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4308e3; continue;
 }
 0x4308e3 => {
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))), 0xffu8); }
  if !cpu.zf { pc = 0x4308ec; continue; }
  pc = 0x4308ea; continue;
 }
 0x4308ea => {
  { let v: u8 = 0x39u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4308ec; continue;
 }
 0x4308ec => {
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4308e3; continue; }
  pc = 0x4308ef; continue;
 }
 0x4308ef => {
  { let v: u8 = 0xfdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = 0x575a15u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5cc431u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x430922; continue; }
  pc = 0x43090e; continue;
 }
 0x43090e => {
  { let v: u32 = 0x5cc43du32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x430922; continue; }
  pc = 0x43091d; continue;
 }
 0x43091d => {
  { let v: u32 = 0x5cc446u32; cpu.ebp = v; }
  pc = 0x430922; continue;
 }
 0x430922 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1559u32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x43094a; continue; }
  pc = 0x43093a; continue;
 }
 0x43093a => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x43094a; continue; }
  pc = 0x430942; continue;
 }
 0x430942 => {
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x43094a; continue;
 }
 0x43094a => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x430922; continue; }
  pc = 0x43094d; continue;
 }
 0x43094d => {
  cpu.push(0x430952); f_00430846(cpu); if cpu.returned_to != 0x430952 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x43095c); f_004147f8(cpu); if cpu.returned_to != 0x43095c { return; }
  pc = 0x43095c; continue;
 }
 0x43095c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043095d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43095d); }
 let mut pc: u32 = 0x43095d;
 loop { match pc {
 0x43095d => {
  { let v: u32 = cpu.r32(0x4d6a9cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x59c5f2u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), (cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4309c5; continue; }
  pc = 0x43097a; continue;
 }
 0x43097a => {
  { let v: u32 = cpu.r8(0xfffffffeu32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x581c03u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x59c5b5u32; cpu.edi = v; }
  pc = 0x43098e; continue;
 }
 0x43098e => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43098e; continue; }
  pc = 0x430993; continue;
 }
 0x430993 => {
  { let v: u32 = cpu.r8(0x592192u32) as u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x59805cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4309b3; continue; }
  pc = 0x4309a3; continue;
 }
 0x4309a3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x4309b3; continue; }
  pc = 0x4309b1; continue;
 }
 0x4309b1 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x4309b3; continue;
 }
 0x4309b3 => {
  { let v: u32 = 0x59c5b5u32; cpu.esi = v; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4309c5); f_00446f7d(cpu); if cpu.returned_to != 0x4309c5 { return; }
  pc = 0x4309c5; continue;
 }
 0x4309c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004309c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4309c6); }
 let mut pc: u32 = 0x4309c6;
 loop { match pc {
 0x4309c6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x57577fu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004309e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4309e3); }
 let mut pc: u32 = 0x4309e3;
 loop { match pc {
 0x4309e3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x57577fu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430a00); }
 let mut pc: u32 = 0x430a00;
 loop { match pc {
 0x430a00 => {
  { let v: u32 = cpu.r32(0x5a0048u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x575860u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v = cpu.r32(0x575860u32); cpu.idiv32(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430a1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430a1e); }
 let mut pc: u32 = 0x430a1e;
 loop { match pc {
 0x430a1e => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x430a28; continue; }
  pc = 0x430a23; continue;
 }
 0x430a23 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x430a28; continue;
 }
 0x430a28 => {
  { let v: u32 = cpu.eax; cpu.w32(0x575860u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v = cpu.r32(0x575860u32); cpu.idiv32(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430a93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430a93); }
 let mut pc: u32 = 0x430a93;
 loop { match pc {
 0x430a93 => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  cpu.push(0x430a9a); f_00430a1e(cpu); if cpu.returned_to != 0x430a9a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x430aae); f_00430a00(cpu); if cpu.returned_to != 0x430aae { return; }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6bd8u32), cpu.eax); cpu.w32(0x4d6bd8u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430abb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430abb); }
 let mut pc: u32 = 0x430abb;
 loop { match pc {
 0x430abb => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x430ac5; continue; }
  pc = 0x430ac4; continue;
 }
 0x430ac4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x430ac5 => {
  cpu.push(0x430aca); f_00430a93(cpu); if cpu.returned_to != 0x430aca { return; }
  { let v: u32 = cpu.r32(0x575f24u32).wrapping_add(cpu.eax); cpu.w32(0x575f24u32, v); }
  { cpu.sub32(cpu.r32(0x575f24u32), 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x430ae5; continue; }
  pc = 0x430ad9; continue;
 }
 0x430ad9 => {
  { let v: u32 = 0x64u32; cpu.w32(0x575f24u32, v); }
  pc = 0x430af8; continue;
 }
 0x430ae5 => {
  { cpu.sub32(cpu.r32(0x575f24u32), 0x0u32); }
  if !cpu.sf { pc = 0x430af8; continue; }
  pc = 0x430aee; continue;
 }
 0x430aee => {
  { let v: u32 = 0x0u32; cpu.w32(0x575f24u32, v); }
  pc = 0x430af8; continue;
 }
 0x430af8 => {
  { let v: u32 = cpu.r32(0x575f24u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5adea0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ae038u32); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x430b0f; continue;
 }
 0x430b0f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x430b0f; continue; }
  pc = 0x430b18; continue;
 }
 0x430b18 => {
  { let v: u32 = cpu.r32(0x5ae034u32); cpu.esi = v; }
  cpu.push(0x430b23); f_00474f51(cpu); if cpu.returned_to != 0x430b23 { return; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ae034u32); cpu.esi = v; }
  cpu.push(0x430b3e); f_00474c5f(cpu); if cpu.returned_to != 0x430b3e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430b3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430b3f); }
 let mut pc: u32 = 0x430b3f;
 loop { match pc {
 0x430b3f => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  if !cpu.sf { pc = 0x430b48; continue; }
  pc = 0x430b43; continue;
 }
 0x430b43 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x430b48; continue;
 }
 0x430b48 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bdfu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x430b66; continue; }
  pc = 0x430b61; continue;
 }
 0x430b61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x430b66; continue;
 }
 0x430b66 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d2be3u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5a0048u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d2be3u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.r32(0x5d2bdfu32); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430ba7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430ba7); }
 let mut pc: u32 = 0x430ba7;
 loop { match pc {
 0x430ba7 => {
  { cpu.pushad(); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u32 = cpu.r32(0x5978d4u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x59fb84u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x59fb86u32, v); }
  pc = 0x430bc9; continue;
 }
 0x430bc9 => {
  { let v: u32 = 0x59fb88u32; cpu.edi = v; }
  { cpu.sub16((cpu.ecx as u16), 0x1cu16); }
  if !cpu.zf { pc = 0x430be6; continue; }
  pc = 0x430bd4; continue;
 }
 0x430bd4 => {
  { let v: u16 = 0x8cu16; cpu.w16(0x59fb84u32, v); }
  { let v: u16 = 0xffb0u16; cpu.w16(0x59fb86u32, v); }
  pc = 0x430be6; continue;
 }
 0x430be6 => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x430bfe; continue; }
  pc = 0x430bec; continue;
 }
 0x430bec => {
  { let v: u16 = 0x8cu16; cpu.w16(0x59fb84u32, v); }
  { let v: u16 = 0xff9cu16; cpu.w16(0x59fb86u32, v); }
  pc = 0x430bfe; continue;
 }
 0x430bfe => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x430c0d); f_00475109(cpu); if cpu.returned_to != 0x430c0d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x59fb88u32; cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5978d4u32)); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x44u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x59fb84u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x59fb86u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19b7u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x57925eu32, v); }
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430c5d); f_0040e620(cpu); if cpu.returned_to != 0x430c5d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x575780u32.wrapping_add(cpu.ecx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x59feabu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5adea0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5978d4u32)); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x44u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x38u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x59fb84u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x59fb86u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19b7u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430cb9); f_0040e620(cpu); if cpu.returned_to != 0x430cb9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x5a003cu32; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x575780u32.wrapping_add(cpu.ecx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x59fb8bu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x430cdb); f_00475109(cpu); if cpu.returned_to != 0x430cdb { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x5a003cu32; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5978d4u32)); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x44u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x65u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x59fb84u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x59fb86u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19b7u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430d1f); f_0040e620(cpu); if cpu.returned_to != 0x430d1f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x5a0042u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x575780u32.wrapping_add(cpu.ecx)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x59fd1bu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x430d44); f_00475109(cpu); if cpu.returned_to != 0x430d44 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x5a0045u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x575780u32.wrapping_add(cpu.ecx)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x59fd1du32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x430d65); f_00475109(cpu); if cpu.returned_to != 0x430d65 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x5a0042u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5978d4u32)); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x82u32; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x44u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x59fb84u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x59fb86u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19b7u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430da9); f_0040e620(cpu); if cpu.returned_to != 0x430da9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x14u32); }
  if cpu.zf { pc = 0x430dbd; continue; }
  pc = 0x430db4; continue;
 }
 0x430db4 => {
  { cpu.sub32(cpu.ecx, 0x23u32); }
  if !cpu.zf { cpu.poll(); pc = 0x430bc9; continue; }
  pc = 0x430dbd; continue;
 }
 0x430dbd => {
  { let v: u32 = 0x0u32; cpu.w32(0x57925eu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430dc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430dc9); }
 let mut pc: u32 = 0x430dc9;
 loop { match pc {
 0x430dc9 => {
  { cpu.pushad(); }
  cpu.push(0x430dcf); f_0040d2b0(cpu); if cpu.returned_to != 0x430dcf { return; }
  { cpu.popad(); }
  { let v: u32 = 0x59fb7bu32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430de2); f_0040d400(cpu); if cpu.returned_to != 0x430de2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x430df4; continue; }
  pc = 0x430dea; continue;
 }
 0x430dea => {
  cpu.push(0x430def); f_00430e7a(cpu); if cpu.returned_to != 0x430def { return; }
  cpu.push(0x430df4); f_00430e57(cpu); if cpu.returned_to != 0x430df4 { return; }
  pc = 0x430df4; continue;
 }
 0x430df4 => {
  { let v: u32 = 0x59fb7bu32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430e06); f_0040d370(cpu); if cpu.returned_to != 0x430e06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x430e39; continue; }
  pc = 0x430e18; continue;
 }
 0x430e18 => {
  cpu.push(0x430e1d); f_0041c997(cpu); if cpu.returned_to != 0x430e1d { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x59fb7bu32; cpu.edx = v; }
  { let v: u32 = 0x59fb8bu32; cpu.ebx = v; }
  { let v: u32 = 0x5a003bu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  cpu.push(0x430e38); f_0041c919(cpu); if cpu.returned_to != 0x430e38 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x430e39 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430e57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430e57); }
 let mut pc: u32 = 0x430e57;
 loop { match pc {
 0x430e57 => {
  { let v: u32 = 0x59fb7bu32; cpu.eax = v; }
  { let v: u32 = 0x59fb8bu32; cpu.ebx = v; }
  { let v: u32 = 0x5a003bu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x430e76); f_0040d500(cpu); if cpu.returned_to != 0x430e76 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430e7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430e7a); }
 let mut pc: u32 = 0x430e7a;
 loop { match pc {
 0x430e7a => {
  { let v: u32 = 0x59fb8bu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x59fd1bu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x59feabu32; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430eae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430eae); }
 let mut pc: u32 = 0x430eae;
 loop { match pc {
 0x430eae => {
  { cpu.sub8(cpu.r8(0x4d67dau32), 0x64u8); }
  if (cpu.sf == cpu.of) { pc = 0x430f19; continue; }
  pc = 0x430ebb; continue;
 }
 0x430ebb => {
  { let v: u32 = cpu.edx; cpu.w32(0x4d6be4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x430ec9); f_00430dc9(cpu); if cpu.returned_to != 0x430ec9 { return; }
  if cpu.cf { pc = 0x430f1a; continue; }
  pc = 0x430ecb; continue;
 }
 0x430ecb => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x59fb8bu32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6bd8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x430f14; continue; }
  pc = 0x430eea; continue;
 }
 0x430eea => {
  { let v: u32 = cpu.eax; cpu.w32(0x59fb8bu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x59fd1bu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x59fd1du32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x59feabu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x430f14); f_00430e57(cpu); if cpu.returned_to != 0x430f14 { return; }
  pc = 0x430f14; continue;
 }
 0x430f14 => {
  cpu.push(0x430f19); f_0041b99b(cpu); if cpu.returned_to != 0x430f19 { return; }
  pc = 0x430f19; continue;
 }
 0x430f19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x430f1a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00430f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x430f20); }
 let mut pc: u32 = 0x430f20;
 loop { match pc {
 0x430f20 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1922u32; cpu.ecx = v; }
  cpu.push(0x430f34); f_00438fb9(cpu); if cpu.returned_to != 0x430f34 { return; }
  { cpu.sub8(cpu.r8(0x575f20u32), 0x0u8); }
  if !cpu.zf { pc = 0x431146; continue; }
  pc = 0x430f41; continue;
 }
 0x430f41 => {
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x430f6b; continue; }
  pc = 0x430f4a; continue;
 }
 0x430f4a => {
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x430f6b; continue; }
  pc = 0x430f58; continue;
 }
 0x430f58 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575f21u32) & 0x1u8; cpu.w8(0x575f21u32, v); }
  { let v: u8 = cpu.r8(0x575f21u32) | (cpu.ecx as u8); cpu.w8(0x575f21u32, v); }
  pc = 0x430f6b; continue;
 }
 0x430f6b => {
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x575f21u32), 0xeu8); }
  if cpu.zf { pc = 0x430f8c; continue; }
  pc = 0x430f89; continue;
 }
 0x430f89 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x430f8c; continue;
 }
 0x430f8c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x430fa5); cpu.call(t); if cpu.returned_to != 0x430fa5 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x575f21u32), 0xeu8); }
  if cpu.zf { pc = 0x430fc5; continue; }
  pc = 0x430fbb; continue;
 }
 0x430fbb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x430fc5; continue;
 }
 0x430fc5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431009); f_0041cf66(cpu); if cpu.returned_to != 0x431009 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43102c); cpu.call(t); if cpu.returned_to != 0x43102c { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x431060; continue; }
  pc = 0x43103f; continue;
 }
 0x43103f => {
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x431060; continue; }
  pc = 0x43104d; continue;
 }
 0x43104d => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575f22u32) & 0x1u8; cpu.w8(0x575f22u32, v); }
  { let v: u8 = cpu.r8(0x575f22u32) | (cpu.ecx as u8); cpu.w8(0x575f22u32, v); }
  pc = 0x431060; continue;
 }
 0x431060 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x575f22u32), 0xeu8); }
  if cpu.zf { pc = 0x431083; continue; }
  pc = 0x431080; continue;
 }
 0x431080 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x431083; continue;
 }
 0x431083 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43109c); cpu.call(t); if cpu.returned_to != 0x43109c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x575f22u32), 0xeu8); }
  if cpu.zf { pc = 0x4310bc; continue; }
  pc = 0x4310b2; continue;
 }
 0x4310b2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4310bc; continue;
 }
 0x4310bc => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa3u32; cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431100); f_0041cf66(cpu); if cpu.returned_to != 0x431100 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431141; continue; }
  pc = 0x43110b; continue;
 }
 0x43110b => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43111e; continue; }
  pc = 0x431115; continue;
 }
 0x431115 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431141; continue; }
  pc = 0x43111e; continue;
 }
 0x43111e => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431140); cpu.call(t); if cpu.returned_to != 0x431140 { return; }
  { cpu.popad(); }
  pc = 0x431141; continue;
 }
 0x431141 => {
  pc = 0x43136c; continue;
 }
 0x431146 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x431179; continue; }
  pc = 0x431158; continue;
 }
 0x431158 => {
  { let v: u8 = cpu.r8(0x5a2370u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x431179; continue; }
  pc = 0x431166; continue;
 }
 0x431166 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2370u32) & 0x1u8; cpu.w8(0x5a2370u32, v); }
  { let v: u8 = cpu.r8(0x5a2370u32) | (cpu.ecx as u8); cpu.w8(0x5a2370u32, v); }
  pc = 0x431179; continue;
 }
 0x431179 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7au32); cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2370u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2370u32), 0xeu8); }
  if cpu.zf { pc = 0x43119a; continue; }
  pc = 0x431197; continue;
 }
 0x431197 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43119a; continue;
 }
 0x43119a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4311b3); cpu.call(t); if cpu.returned_to != 0x4311b3 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2370u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2370u32), 0xeu8); }
  if cpu.zf { pc = 0x4311d3; continue; }
  pc = 0x4311c9; continue;
 }
 0x4311c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4311d3; continue;
 }
 0x4311d3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431217); f_0041cf66(cpu); if cpu.returned_to != 0x431217 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431258; continue; }
  pc = 0x431222; continue;
 }
 0x431222 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431235; continue; }
  pc = 0x43122c; continue;
 }
 0x43122c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431258; continue; }
  pc = 0x431235; continue;
 }
 0x431235 => {
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431257); cpu.call(t); if cpu.returned_to != 0x431257 { return; }
  { cpu.popad(); }
  pc = 0x431258; continue;
 }
 0x431258 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43128b; continue; }
  pc = 0x43126a; continue;
 }
 0x43126a => {
  { let v: u8 = cpu.r8(0x5a2371u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43128b; continue; }
  pc = 0x431278; continue;
 }
 0x431278 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2371u32) & 0x1u8; cpu.w8(0x5a2371u32, v); }
  { let v: u8 = cpu.r8(0x5a2371u32) | (cpu.ecx as u8); cpu.w8(0x5a2371u32, v); }
  pc = 0x43128b; continue;
 }
 0x43128b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2371u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2371u32), 0xeu8); }
  if cpu.zf { pc = 0x4312ae; continue; }
  pc = 0x4312ab; continue;
 }
 0x4312ab => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4312ae; continue;
 }
 0x4312ae => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4312c7); cpu.call(t); if cpu.returned_to != 0x4312c7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2371u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2371u32), 0xeu8); }
  if cpu.zf { pc = 0x4312e7; continue; }
  pc = 0x4312dd; continue;
 }
 0x4312dd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4312e7; continue;
 }
 0x4312e7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa3u32; cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43132b); f_0041cf66(cpu); if cpu.returned_to != 0x43132b { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43136c; continue; }
  pc = 0x431336; continue;
 }
 0x431336 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431349; continue; }
  pc = 0x431340; continue;
 }
 0x431340 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43136c; continue; }
  pc = 0x431349; continue;
 }
 0x431349 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43136b); cpu.call(t); if cpu.returned_to != 0x43136b { return; }
  { cpu.popad(); }
  pc = 0x43136c; continue;
 }
 0x43136c => {
  { cpu.sub8(cpu.r8(0x575f1fu32), 0x0u8); }
  if !cpu.zf { pc = 0x43157e; continue; }
  pc = 0x431379; continue;
 }
 0x431379 => {
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4313a3; continue; }
  pc = 0x431382; continue;
 }
 0x431382 => {
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4313a3; continue; }
  pc = 0x431390; continue;
 }
 0x431390 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575f21u32) & 0x1u8; cpu.w8(0x575f21u32, v); }
  { let v: u8 = cpu.r8(0x575f21u32) | (cpu.ecx as u8); cpu.w8(0x575f21u32, v); }
  pc = 0x4313a3; continue;
 }
 0x4313a3 => {
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x575f21u32), 0xeu8); }
  if cpu.zf { pc = 0x4313c4; continue; }
  pc = 0x4313c1; continue;
 }
 0x4313c1 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4313c4; continue;
 }
 0x4313c4 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4313dd); cpu.call(t); if cpu.returned_to != 0x4313dd { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f21u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x575f21u32), 0xeu8); }
  if cpu.zf { pc = 0x4313fd; continue; }
  pc = 0x4313f3; continue;
 }
 0x4313f3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4313fd; continue;
 }
 0x4313fd => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x5fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431441); f_0041cf66(cpu); if cpu.returned_to != 0x431441 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431464); cpu.call(t); if cpu.returned_to != 0x431464 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x431498; continue; }
  pc = 0x431477; continue;
 }
 0x431477 => {
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x431498; continue; }
  pc = 0x431485; continue;
 }
 0x431485 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x575f22u32) & 0x1u8; cpu.w8(0x575f22u32, v); }
  { let v: u8 = cpu.r8(0x575f22u32) | (cpu.ecx as u8); cpu.w8(0x575f22u32, v); }
  pc = 0x431498; continue;
 }
 0x431498 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x575f22u32), 0xeu8); }
  if cpu.zf { pc = 0x4314bb; continue; }
  pc = 0x4314b8; continue;
 }
 0x4314b8 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4314bb; continue;
 }
 0x4314bb => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4314d4); cpu.call(t); if cpu.returned_to != 0x4314d4 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x575f22u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x575f22u32), 0xeu8); }
  if cpu.zf { pc = 0x4314f4; continue; }
  pc = 0x4314ea; continue;
 }
 0x4314ea => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4314f4; continue;
 }
 0x4314f4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa4u32; cpu.eax = v; }
  { let v: u32 = 0x5fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431538); f_0041cf66(cpu); if cpu.returned_to != 0x431538 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431579; continue; }
  pc = 0x431543; continue;
 }
 0x431543 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431556; continue; }
  pc = 0x43154d; continue;
 }
 0x43154d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431579; continue; }
  pc = 0x431556; continue;
 }
 0x431556 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431578); cpu.call(t); if cpu.returned_to != 0x431578 { return; }
  { cpu.popad(); }
  pc = 0x431579; continue;
 }
 0x431579 => {
  pc = 0x4317a4; continue;
 }
 0x43157e => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4315b1; continue; }
  pc = 0x431590; continue;
 }
 0x431590 => {
  { let v: u8 = cpu.r8(0x5a2373u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4315b1; continue; }
  pc = 0x43159e; continue;
 }
 0x43159e => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2373u32) & 0x1u8; cpu.w8(0x5a2373u32, v); }
  { let v: u8 = cpu.r8(0x5a2373u32) | (cpu.ecx as u8); cpu.w8(0x5a2373u32, v); }
  pc = 0x4315b1; continue;
 }
 0x4315b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7au32); cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2373u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2373u32), 0xeu8); }
  if cpu.zf { pc = 0x4315d2; continue; }
  pc = 0x4315cf; continue;
 }
 0x4315cf => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4315d2; continue;
 }
 0x4315d2 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4315eb); cpu.call(t); if cpu.returned_to != 0x4315eb { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2373u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2373u32), 0xeu8); }
  if cpu.zf { pc = 0x43160b; continue; }
  pc = 0x431601; continue;
 }
 0x431601 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x43160b; continue;
 }
 0x43160b => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x5fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43164f); f_0041cf66(cpu); if cpu.returned_to != 0x43164f { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431690; continue; }
  pc = 0x43165a; continue;
 }
 0x43165a => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43166d; continue; }
  pc = 0x431664; continue;
 }
 0x431664 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431690; continue; }
  pc = 0x43166d; continue;
 }
 0x43166d => {
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43168f); cpu.call(t); if cpu.returned_to != 0x43168f { return; }
  { cpu.popad(); }
  pc = 0x431690; continue;
 }
 0x431690 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4316c3; continue; }
  pc = 0x4316a2; continue;
 }
 0x4316a2 => {
  { let v: u8 = cpu.r8(0x5a2374u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4316c3; continue; }
  pc = 0x4316b0; continue;
 }
 0x4316b0 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2374u32) & 0x1u8; cpu.w8(0x5a2374u32, v); }
  { let v: u8 = cpu.r8(0x5a2374u32) | (cpu.ecx as u8); cpu.w8(0x5a2374u32, v); }
  pc = 0x4316c3; continue;
 }
 0x4316c3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2374u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2374u32), 0xeu8); }
  if cpu.zf { pc = 0x4316e6; continue; }
  pc = 0x4316e3; continue;
 }
 0x4316e3 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4316e6; continue;
 }
 0x4316e6 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4316ff); cpu.call(t); if cpu.returned_to != 0x4316ff { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2374u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2374u32), 0xeu8); }
  if cpu.zf { pc = 0x43171f; continue; }
  pc = 0x431715; continue;
 }
 0x431715 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x43171f; continue;
 }
 0x43171f => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa3u32; cpu.eax = v; }
  { let v: u32 = 0x5fu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431763); f_0041cf66(cpu); if cpu.returned_to != 0x431763 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4317a4; continue; }
  pc = 0x43176e; continue;
 }
 0x43176e => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431781; continue; }
  pc = 0x431778; continue;
 }
 0x431778 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4317a4; continue; }
  pc = 0x431781; continue;
 }
 0x431781 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4317a3); cpu.call(t); if cpu.returned_to != 0x4317a3 { return; }
  { cpu.popad(); }
  pc = 0x4317a4; continue;
 }
 0x4317a4 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237au32), 0x0u8); }
  if cpu.zf { pc = 0x4317c4; continue; }
  pc = 0x4317b9; continue;
 }
 0x4317b9 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237au32)); cpu.w8(0x5a237au32, v); }
  pc = 0x4317c4; continue;
 }
 0x4317c4 => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1911u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4317e7); cpu.call(t); if cpu.returned_to != 0x4317e7 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237bu32), 0x0u8); }
  if cpu.zf { pc = 0x431808; continue; }
  pc = 0x4317fd; continue;
 }
 0x4317fd => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237bu32)); cpu.w8(0x5a237bu32, v); }
  pc = 0x431808; continue;
 }
 0x431808 => {
  { let v: u32 = 0xaeu32; cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x190fu32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43182b); cpu.call(t); if cpu.returned_to != 0x43182b { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237cu32), 0x0u8); }
  if cpu.zf { pc = 0x43184c; continue; }
  pc = 0x431841; continue;
 }
 0x431841 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237cu32)); cpu.w8(0x5a237cu32, v); }
  pc = 0x43184c; continue;
 }
 0x43184c => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x6cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1911u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43186f); cpu.call(t); if cpu.returned_to != 0x43186f { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237du32), 0x0u8); }
  if cpu.zf { pc = 0x431890; continue; }
  pc = 0x431885; continue;
 }
 0x431885 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237du32)); cpu.w8(0x5a237du32, v); }
  pc = 0x431890; continue;
 }
 0x431890 => {
  { let v: u32 = 0xaeu32; cpu.eax = v; }
  { let v: u32 = 0x6cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x190fu32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4318b3); cpu.call(t); if cpu.returned_to != 0x4318b3 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4318e7; continue; }
  pc = 0x4318c6; continue;
 }
 0x4318c6 => {
  { let v: u8 = cpu.r8(0x58c6e4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4318e7; continue; }
  pc = 0x4318d4; continue;
 }
 0x4318d4 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6e4u32) & 0x1u8; cpu.w8(0x58c6e4u32, v); }
  { let v: u8 = cpu.r8(0x58c6e4u32) | (cpu.ecx as u8); cpu.w8(0x58c6e4u32, v); }
  pc = 0x4318e7; continue;
 }
 0x4318e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7au32); cpu.eax = v; }
  { let v: u32 = 0x79u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6e4u32), 0xeu8); }
  if cpu.zf { pc = 0x431908; continue; }
  pc = 0x431905; continue;
 }
 0x431905 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x431908; continue;
 }
 0x431908 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431921); cpu.call(t); if cpu.returned_to != 0x431921 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6e4u32), 0xeu8); }
  if cpu.zf { pc = 0x431941; continue; }
  pc = 0x431937; continue;
 }
 0x431937 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x431941; continue;
 }
 0x431941 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x7cu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431985); f_0041cf66(cpu); if cpu.returned_to != 0x431985 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4319c6; continue; }
  pc = 0x431990; continue;
 }
 0x431990 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4319a3; continue; }
  pc = 0x43199a; continue;
 }
 0x43199a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4319c6; continue; }
  pc = 0x4319a3; continue;
 }
 0x4319a3 => {
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0x79u32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4319c5); cpu.call(t); if cpu.returned_to != 0x4319c5 { return; }
  { cpu.popad(); }
  pc = 0x4319c6; continue;
 }
 0x4319c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4319f9; continue; }
  pc = 0x4319d8; continue;
 }
 0x4319d8 => {
  { let v: u8 = cpu.r8(0x58c6e5u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4319f9; continue; }
  pc = 0x4319e6; continue;
 }
 0x4319e6 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6e5u32) & 0x1u8; cpu.w8(0x58c6e5u32, v); }
  { let v: u8 = cpu.r8(0x58c6e5u32) | (cpu.ecx as u8); cpu.w8(0x58c6e5u32, v); }
  pc = 0x4319f9; continue;
 }
 0x4319f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x79u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e5u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6e5u32), 0xeu8); }
  if cpu.zf { pc = 0x431a1c; continue; }
  pc = 0x431a19; continue;
 }
 0x431a19 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x431a1c; continue;
 }
 0x431a1c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431a35); cpu.call(t); if cpu.returned_to != 0x431a35 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e5u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6e5u32), 0xeu8); }
  if cpu.zf { pc = 0x431a55; continue; }
  pc = 0x431a4b; continue;
 }
 0x431a4b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x431a55; continue;
 }
 0x431a55 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa4u32; cpu.eax = v; }
  { let v: u32 = 0x7cu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431a99); f_0041cf66(cpu); if cpu.returned_to != 0x431a99 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431ada; continue; }
  pc = 0x431aa4; continue;
 }
 0x431aa4 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431ab7; continue; }
  pc = 0x431aae; continue;
 }
 0x431aae => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431ada; continue; }
  pc = 0x431ab7; continue;
 }
 0x431ab7 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x79u32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431ad9); cpu.call(t); if cpu.returned_to != 0x431ad9 { return; }
  { cpu.popad(); }
  pc = 0x431ada; continue;
 }
 0x431ada => {
  { let v: u32 = 0x76u32; cpu.eax = v; }
  { let v: u32 = 0x4cu32; cpu.ebx = v; }
  { let v: u32 = 0x1917u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431afc); cpu.call(t); if cpu.returned_to != 0x431afc { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5a2376u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x190eu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431b30); cpu.call(t); if cpu.returned_to != 0x431b30 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x76u32; cpu.eax = v; }
  { let v: u32 = 0x6du32; cpu.ebx = v; }
  { let v: u32 = 0x1917u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431b53); cpu.call(t); if cpu.returned_to != 0x431b53 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5a2378u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x6eu32; cpu.ebx = v; }
  { let v: u32 = 0x190eu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431b87); cpu.call(t); if cpu.returned_to != 0x431b87 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = cpu.r32(0x5add50u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431bd4); f_0041cf66(cpu); if cpu.returned_to != 0x431bd4 { return; }
  { let v: u32 = cpu.r32(0x5add54u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431c0c); f_0041cf66(cpu); if cpu.returned_to != 0x431c0c { return; }
  { let v: u32 = cpu.r32(0x5add58u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x61u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431c44); f_0041cf66(cpu); if cpu.returned_to != 0x431c44 { return; }
  { let v: u32 = cpu.r32(0x5add54u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x6bu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431c7c); f_0041cf66(cpu); if cpu.returned_to != 0x431c7c { return; }
  { let v: u32 = cpu.r32(0x5ae0e8u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x79u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431cb4); f_0041cf66(cpu); if cpu.returned_to != 0x431cb4 { return; }
  { let v: u32 = cpu.r32(0x5ae0ecu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x83u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431cec); f_0041cf66(cpu); if cpu.returned_to != 0x431cec { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00431ced(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x431ced); } r_00431ced(cpu, 0x431ced); }
pub fn f_00432000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x432000); } r_00431ced(cpu, 0x432000); }
fn r_00431ced(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x431ced => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1921u32; cpu.ecx = v; }
  cpu.push(0x431d01); f_00438fb9(cpu); if cpu.returned_to != 0x431d01 { return; }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x192bu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431d23); cpu.call(t); if cpu.returned_to != 0x431d23 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x576e62u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xdu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x7fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x192cu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431d53); cpu.call(t); if cpu.returned_to != 0x431d53 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x431d87; continue; }
  pc = 0x431d66; continue;
 }
 0x431d66 => {
  { let v: u8 = cpu.r8(0x5a236eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x431d87; continue; }
  pc = 0x431d74; continue;
 }
 0x431d74 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236eu32) & 0x1u8; cpu.w8(0x5a236eu32, v); }
  { let v: u8 = cpu.r8(0x5a236eu32) | (cpu.ecx as u8); cpu.w8(0x5a236eu32, v); }
  pc = 0x431d87; continue;
 }
 0x431d87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236eu32), 0xeu8); }
  if cpu.zf { pc = 0x431daa; continue; }
  pc = 0x431da7; continue;
 }
 0x431da7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x431daa; continue;
 }
 0x431daa => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431dc3); cpu.call(t); if cpu.returned_to != 0x431dc3 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236eu32), 0xeu8); }
  if cpu.zf { pc = 0x431de3; continue; }
  pc = 0x431dd9; continue;
 }
 0x431dd9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x431de3; continue;
 }
 0x431de3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431e27); f_0041cf66(cpu); if cpu.returned_to != 0x431e27 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431e68; continue; }
  pc = 0x431e32; continue;
 }
 0x431e32 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431e45; continue; }
  pc = 0x431e3c; continue;
 }
 0x431e3c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431e68; continue; }
  pc = 0x431e45; continue;
 }
 0x431e45 => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431e67); cpu.call(t); if cpu.returned_to != 0x431e67 { return; }
  { cpu.popad(); }
  pc = 0x431e68; continue;
 }
 0x431e68 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x431e9b; continue; }
  pc = 0x431e7a; continue;
 }
 0x431e7a => {
  { let v: u8 = cpu.r8(0x5a236fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x431e9b; continue; }
  pc = 0x431e88; continue;
 }
 0x431e88 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236fu32) & 0x1u8; cpu.w8(0x5a236fu32, v); }
  { let v: u8 = cpu.r8(0x5a236fu32) | (cpu.ecx as u8); cpu.w8(0x5a236fu32, v); }
  pc = 0x431e9b; continue;
 }
 0x431e9b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236fu32), 0xeu8); }
  if cpu.zf { pc = 0x431ebe; continue; }
  pc = 0x431ebb; continue;
 }
 0x431ebb => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x431ebe; continue;
 }
 0x431ebe => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431ed7); cpu.call(t); if cpu.returned_to != 0x431ed7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236fu32), 0xeu8); }
  if cpu.zf { pc = 0x431ef7; continue; }
  pc = 0x431eed; continue;
 }
 0x431eed => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x431ef7; continue;
 }
 0x431ef7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa3u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x431f3b); f_0041cf66(cpu); if cpu.returned_to != 0x431f3b { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x431f7c; continue; }
  pc = 0x431f46; continue;
 }
 0x431f46 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x431f59; continue; }
  pc = 0x431f50; continue;
 }
 0x431f50 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x431f7c; continue; }
  pc = 0x431f59; continue;
 }
 0x431f59 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431f7b); cpu.call(t); if cpu.returned_to != 0x431f7b { return; }
  { cpu.popad(); }
  pc = 0x431f7c; continue;
 }
 0x431f7c => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x0u8); }
  if cpu.zf { pc = 0x431fc4; continue; }
  pc = 0x431fa5; continue;
 }
 0x431fa5 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x58c6e6u32)); cpu.w8(0x58c6e6u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5a2387u32, v); }
  pc = 0x431fc4; continue;
 }
 0x431fc4 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x192eu32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x431fe7); cpu.call(t); if cpu.returned_to != 0x431fe7 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae100u32); cpu.esi = v; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432021); f_0041cf66(cpu); if cpu.returned_to != 0x432021 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x1u8); }
  if !cpu.zf { pc = 0x432032; continue; }
  pc = 0x43202b; continue;
 }
 0x432000 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x5a2387u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432021); f_0041cf66(cpu); if cpu.returned_to != 0x432021 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x1u8); }
  if !cpu.zf { pc = 0x432032; continue; }
  pc = 0x43202b; continue;
 }
 0x43202b => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e6u32, v); }
  pc = 0x432032; continue;
 }
 0x432032 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x192bu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432068); cpu.call(t); if cpu.returned_to != 0x432068 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x576e64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xdu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x7fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x192cu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432098); cpu.call(t); if cpu.returned_to != 0x432098 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5add4cu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4320d1); f_0041cf66(cpu); if cpu.returned_to != 0x4320d1 { return; }
  { let v: u32 = cpu.r32(0x5add48u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x51u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432109); f_0041cf66(cpu); if cpu.returned_to != 0x432109 { return; }
  { let v: u32 = cpu.r32(0x5ae070u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x61u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432141); f_0041cf66(cpu); if cpu.returned_to != 0x432141 { return; }
  { let v: u32 = cpu.r32(0x5ae074u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x71u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432179); f_0041cf66(cpu); if cpu.returned_to != 0x432179 { return; }
  { let v: u32 = 0x70u32; cpu.eax = v; }
  { let v: u32 = 0x69u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x128au32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43219a); cpu.call(t); if cpu.returned_to != 0x43219a { return; }
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1281u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4321bb); cpu.call(t); if cpu.returned_to != 0x4321bb { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004321bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4321bc); } r_004321bc(cpu, 0x4321bc); }
pub fn f_00432f4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x432f4c); } r_004321bc(cpu, 0x432f4c); }
fn r_004321bc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4321bc => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1921u32; cpu.ecx = v; }
  cpu.push(0x4321d0); f_00438fb9(cpu); if cpu.returned_to != 0x4321d0 { return; }
  { let v: u32 = 0x11u32; cpu.w32(0x578d6fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43220d; continue; }
  pc = 0x4321ec; continue;
 }
 0x4321ec => {
  { let v: u8 = cpu.r8(0x5a2369u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43220d; continue; }
  pc = 0x4321fa; continue;
 }
 0x4321fa => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2369u32) & 0x1u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = cpu.r8(0x5a2369u32) | (cpu.ecx as u8); cpu.w8(0x5a2369u32, v); }
  pc = 0x43220d; continue;
 }
 0x43220d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432223); f_0041ce38(cpu); if cpu.returned_to != 0x432223 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6eu32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2369u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2369u32), 0xeu8); }
  if cpu.zf { pc = 0x432256; continue; }
  pc = 0x432253; continue;
 }
 0x432253 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432256; continue;
 }
 0x432256 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43226f); cpu.call(t); if cpu.returned_to != 0x43226f { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2369u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2369u32), 0xeu8); }
  if cpu.zf { pc = 0x43228f; continue; }
  pc = 0x432285; continue;
 }
 0x432285 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x43228f; continue;
 }
 0x43228f => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x72u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4322d9); f_0041cf66(cpu); if cpu.returned_to != 0x4322d9 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43231a; continue; }
  pc = 0x4322e4; continue;
 }
 0x4322e4 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4322f7; continue; }
  pc = 0x4322ee; continue;
 }
 0x4322ee => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43231a; continue; }
  pc = 0x4322f7; continue;
 }
 0x4322f7 => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432319); cpu.call(t); if cpu.returned_to != 0x432319 { return; }
  { cpu.popad(); }
  pc = 0x43231a; continue;
 }
 0x43231a => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43234d; continue; }
  pc = 0x43232c; continue;
 }
 0x43232c => {
  { let v: u8 = cpu.r8(0x5a236au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43234d; continue; }
  pc = 0x43233a; continue;
 }
 0x43233a => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236au32) & 0x1u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = cpu.r8(0x5a236au32) | (cpu.ecx as u8); cpu.w8(0x5a236au32, v); }
  pc = 0x43234d; continue;
 }
 0x43234d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432363); f_0041ce38(cpu); if cpu.returned_to != 0x432363 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236au32), 0xeu8); }
  if cpu.zf { pc = 0x432398; continue; }
  pc = 0x432395; continue;
 }
 0x432395 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432398; continue;
 }
 0x432398 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4323b1); cpu.call(t); if cpu.returned_to != 0x4323b1 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236au32), 0xeu8); }
  if cpu.zf { pc = 0x4323d1; continue; }
  pc = 0x4323c7; continue;
 }
 0x4323c7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4323d1; continue;
 }
 0x4323d1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43241b); f_0041cf66(cpu); if cpu.returned_to != 0x43241b { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43245c; continue; }
  pc = 0x432426; continue;
 }
 0x432426 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x432439; continue; }
  pc = 0x432430; continue;
 }
 0x432430 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43245c; continue; }
  pc = 0x432439; continue;
 }
 0x432439 => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43245b); cpu.call(t); if cpu.returned_to != 0x43245b { return; }
  { cpu.popad(); }
  pc = 0x43245c; continue;
 }
 0x43245c => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43248f; continue; }
  pc = 0x43246e; continue;
 }
 0x43246e => {
  { let v: u8 = cpu.r8(0x5a236bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43248f; continue; }
  pc = 0x43247c; continue;
 }
 0x43247c => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236bu32) & 0x1u8; cpu.w8(0x5a236bu32, v); }
  { let v: u8 = cpu.r8(0x5a236bu32) | (cpu.ecx as u8); cpu.w8(0x5a236bu32, v); }
  pc = 0x43248f; continue;
 }
 0x43248f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4324a5); f_0041ce38(cpu); if cpu.returned_to != 0x4324a5 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236bu32), 0xeu8); }
  if cpu.zf { pc = 0x4324da; continue; }
  pc = 0x4324d7; continue;
 }
 0x4324d7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4324da; continue;
 }
 0x4324da => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4324f3); cpu.call(t); if cpu.returned_to != 0x4324f3 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236bu32), 0xeu8); }
  if cpu.zf { pc = 0x432513; continue; }
  pc = 0x432509; continue;
 }
 0x432509 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432513; continue;
 }
 0x432513 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43255d); f_0041cf66(cpu); if cpu.returned_to != 0x43255d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43259e; continue; }
  pc = 0x432568; continue;
 }
 0x432568 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43257b; continue; }
  pc = 0x432572; continue;
 }
 0x432572 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43259e; continue; }
  pc = 0x43257b; continue;
 }
 0x43257b => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43259d); cpu.call(t); if cpu.returned_to != 0x43259d { return; }
  { cpu.popad(); }
  pc = 0x43259e; continue;
 }
 0x43259e => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4325d1; continue; }
  pc = 0x4325b0; continue;
 }
 0x4325b0 => {
  { let v: u8 = cpu.r8(0x58c6e7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4325d1; continue; }
  pc = 0x4325be; continue;
 }
 0x4325be => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6e7u32) & 0x1u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = cpu.r8(0x58c6e7u32) | (cpu.ecx as u8); cpu.w8(0x58c6e7u32, v); }
  pc = 0x4325d1; continue;
 }
 0x4325d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4325e7); f_0041ce38(cpu); if cpu.returned_to != 0x4325e7 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6eu32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6e7u32), 0xeu8); }
  if cpu.zf { pc = 0x43261a; continue; }
  pc = 0x432617; continue;
 }
 0x432617 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43261a; continue;
 }
 0x43261a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432633); cpu.call(t); if cpu.returned_to != 0x432633 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6e7u32), 0xeu8); }
  if cpu.zf { pc = 0x432653; continue; }
  pc = 0x432649; continue;
 }
 0x432649 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432653; continue;
 }
 0x432653 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x72u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43269d); f_0041cf66(cpu); if cpu.returned_to != 0x43269d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4326de; continue; }
  pc = 0x4326a8; continue;
 }
 0x4326a8 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4326bb; continue; }
  pc = 0x4326b2; continue;
 }
 0x4326b2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4326de; continue; }
  pc = 0x4326bb; continue;
 }
 0x4326bb => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4326dd); cpu.call(t); if cpu.returned_to != 0x4326dd { return; }
  { cpu.popad(); }
  pc = 0x4326de; continue;
 }
 0x4326de => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432711; continue; }
  pc = 0x4326f0; continue;
 }
 0x4326f0 => {
  { let v: u8 = cpu.r8(0x58c6e8u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432711; continue; }
  pc = 0x4326fe; continue;
 }
 0x4326fe => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6e8u32) & 0x1u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = cpu.r8(0x58c6e8u32) | (cpu.ecx as u8); cpu.w8(0x58c6e8u32, v); }
  pc = 0x432711; continue;
 }
 0x432711 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432727); f_0041ce38(cpu); if cpu.returned_to != 0x432727 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e8u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6e8u32), 0xeu8); }
  if cpu.zf { pc = 0x43275c; continue; }
  pc = 0x432759; continue;
 }
 0x432759 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43275c; continue;
 }
 0x43275c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432775); cpu.call(t); if cpu.returned_to != 0x432775 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e8u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6e8u32), 0xeu8); }
  if cpu.zf { pc = 0x432795; continue; }
  pc = 0x43278b; continue;
 }
 0x43278b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432795; continue;
 }
 0x432795 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4327df); f_0041cf66(cpu); if cpu.returned_to != 0x4327df { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x432820; continue; }
  pc = 0x4327ea; continue;
 }
 0x4327ea => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4327fd; continue; }
  pc = 0x4327f4; continue;
 }
 0x4327f4 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x432820; continue; }
  pc = 0x4327fd; continue;
 }
 0x4327fd => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43281f); cpu.call(t); if cpu.returned_to != 0x43281f { return; }
  { cpu.popad(); }
  pc = 0x432820; continue;
 }
 0x432820 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432853; continue; }
  pc = 0x432832; continue;
 }
 0x432832 => {
  { let v: u8 = cpu.r8(0x58c6e9u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432853; continue; }
  pc = 0x432840; continue;
 }
 0x432840 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6e9u32) & 0x1u8; cpu.w8(0x58c6e9u32, v); }
  { let v: u8 = cpu.r8(0x58c6e9u32) | (cpu.ecx as u8); cpu.w8(0x58c6e9u32, v); }
  pc = 0x432853; continue;
 }
 0x432853 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432869); f_0041ce38(cpu); if cpu.returned_to != 0x432869 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e9u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6e9u32), 0xeu8); }
  if cpu.zf { pc = 0x43289e; continue; }
  pc = 0x43289b; continue;
 }
 0x43289b => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43289e; continue;
 }
 0x43289e => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4328b7); cpu.call(t); if cpu.returned_to != 0x4328b7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6e9u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6e9u32), 0xeu8); }
  if cpu.zf { pc = 0x4328d7; continue; }
  pc = 0x4328cd; continue;
 }
 0x4328cd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4328d7; continue;
 }
 0x4328d7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432921); f_0041cf66(cpu); if cpu.returned_to != 0x432921 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x432962; continue; }
  pc = 0x43292c; continue;
 }
 0x43292c => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43293f; continue; }
  pc = 0x432936; continue;
 }
 0x432936 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x432962; continue; }
  pc = 0x43293f; continue;
 }
 0x43293f => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432961); cpu.call(t); if cpu.returned_to != 0x432961 { return; }
  { cpu.popad(); }
  pc = 0x432962; continue;
 }
 0x432962 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x432d38; continue; }
  pc = 0x43296f; continue;
 }
 0x43296f => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4329a2; continue; }
  pc = 0x432981; continue;
 }
 0x432981 => {
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4329a2; continue; }
  pc = 0x43298f; continue;
 }
 0x43298f => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6eau32) & 0x1u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = cpu.r8(0x58c6eau32) | (cpu.ecx as u8); cpu.w8(0x58c6eau32, v); }
  pc = 0x4329a2; continue;
 }
 0x4329a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4329b8); f_0041ce38(cpu); if cpu.returned_to != 0x4329b8 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6eu32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6eau32), 0xeu8); }
  if cpu.zf { pc = 0x4329eb; continue; }
  pc = 0x4329e8; continue;
 }
 0x4329e8 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4329eb; continue;
 }
 0x4329eb => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432a04); cpu.call(t); if cpu.returned_to != 0x432a04 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6eau32), 0xeu8); }
  if cpu.zf { pc = 0x432a24; continue; }
  pc = 0x432a1a; continue;
 }
 0x432a1a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432a24; continue;
 }
 0x432a24 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x72u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432a6e); f_0041cf66(cpu); if cpu.returned_to != 0x432a6e { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x432aaf; continue; }
  pc = 0x432a79; continue;
 }
 0x432a79 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x432a8c; continue; }
  pc = 0x432a83; continue;
 }
 0x432a83 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x432aaf; continue; }
  pc = 0x432a8c; continue;
 }
 0x432a8c => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432aae); cpu.call(t); if cpu.returned_to != 0x432aae { return; }
  { cpu.popad(); }
  pc = 0x432aaf; continue;
 }
 0x432aaf => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432ae2; continue; }
  pc = 0x432ac1; continue;
 }
 0x432ac1 => {
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432ae2; continue; }
  pc = 0x432acf; continue;
 }
 0x432acf => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6ebu32) & 0x1u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = cpu.r8(0x58c6ebu32) | (cpu.ecx as u8); cpu.w8(0x58c6ebu32, v); }
  pc = 0x432ae2; continue;
 }
 0x432ae2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432af8); f_0041ce38(cpu); if cpu.returned_to != 0x432af8 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6ebu32), 0xeu8); }
  if cpu.zf { pc = 0x432b2d; continue; }
  pc = 0x432b2a; continue;
 }
 0x432b2a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432b2d; continue;
 }
 0x432b2d => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432b46); cpu.call(t); if cpu.returned_to != 0x432b46 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6ebu32), 0xeu8); }
  if cpu.zf { pc = 0x432b66; continue; }
  pc = 0x432b5c; continue;
 }
 0x432b5c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432b66; continue;
 }
 0x432b66 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432bb0); f_0041cf66(cpu); if cpu.returned_to != 0x432bb0 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x432bf1; continue; }
  pc = 0x432bbb; continue;
 }
 0x432bbb => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x432bce; continue; }
  pc = 0x432bc5; continue;
 }
 0x432bc5 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x432bf1; continue; }
  pc = 0x432bce; continue;
 }
 0x432bce => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432bf0); cpu.call(t); if cpu.returned_to != 0x432bf0 { return; }
  { cpu.popad(); }
  pc = 0x432bf1; continue;
 }
 0x432bf1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432c24; continue; }
  pc = 0x432c03; continue;
 }
 0x432c03 => {
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432c24; continue; }
  pc = 0x432c11; continue;
 }
 0x432c11 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6ecu32) & 0x1u8; cpu.w8(0x58c6ecu32, v); }
  { let v: u8 = cpu.r8(0x58c6ecu32) | (cpu.ecx as u8); cpu.w8(0x58c6ecu32, v); }
  pc = 0x432c24; continue;
 }
 0x432c24 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x432c3a); f_0041ce38(cpu); if cpu.returned_to != 0x432c3a { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6ecu32), 0xeu8); }
  if cpu.zf { pc = 0x432c6f; continue; }
  pc = 0x432c6c; continue;
 }
 0x432c6c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432c6f; continue;
 }
 0x432c6f => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432c88); cpu.call(t); if cpu.returned_to != 0x432c88 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6ecu32), 0xeu8); }
  if cpu.zf { pc = 0x432ca8; continue; }
  pc = 0x432c9e; continue;
 }
 0x432c9e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432ca8; continue;
 }
 0x432ca8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432cf2); f_0041cf66(cpu); if cpu.returned_to != 0x432cf2 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x432d33; continue; }
  pc = 0x432cfd; continue;
 }
 0x432cfd => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x432d10; continue; }
  pc = 0x432d07; continue;
 }
 0x432d07 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x432d33; continue; }
  pc = 0x432d10; continue;
 }
 0x432d10 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432d32); cpu.call(t); if cpu.returned_to != 0x432d32 { return; }
  { cpu.popad(); }
  pc = 0x432d33; continue;
 }
 0x432d33 => {
  pc = 0x432ffc; continue;
 }
 0x432d38 => {
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432d62; continue; }
  pc = 0x432d41; continue;
 }
 0x432d41 => {
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432d62; continue; }
  pc = 0x432d4f; continue;
 }
 0x432d4f => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6eau32) & 0x1u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = cpu.r8(0x58c6eau32) | (cpu.ecx as u8); cpu.w8(0x58c6eau32, v); }
  pc = 0x432d62; continue;
 }
 0x432d62 => {
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6eau32), 0xeu8); }
  if cpu.zf { pc = 0x432d83; continue; }
  pc = 0x432d80; continue;
 }
 0x432d80 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432d83; continue;
 }
 0x432d83 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432d9c); cpu.call(t); if cpu.returned_to != 0x432d9c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6eau32), 0xeu8); }
  if cpu.zf { pc = 0x432dbc; continue; }
  pc = 0x432db2; continue;
 }
 0x432db2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432dbc; continue;
 }
 0x432dbc => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add2cu32); cpu.esi = v; }
  { let v: u32 = 0x74u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432e00); f_0041cf66(cpu); if cpu.returned_to != 0x432e00 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x6eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432e23); cpu.call(t); if cpu.returned_to != 0x432e23 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432e4e; continue; }
  pc = 0x432e2d; continue;
 }
 0x432e2d => {
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432e4e; continue; }
  pc = 0x432e3b; continue;
 }
 0x432e3b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6ebu32) & 0x1u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = cpu.r8(0x58c6ebu32) | (cpu.ecx as u8); cpu.w8(0x58c6ebu32, v); }
  pc = 0x432e4e; continue;
 }
 0x432e4e => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6ebu32), 0xeu8); }
  if cpu.zf { pc = 0x432e6f; continue; }
  pc = 0x432e6c; continue;
 }
 0x432e6c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432e6f; continue;
 }
 0x432e6f => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432e88); cpu.call(t); if cpu.returned_to != 0x432e88 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6ebu32), 0xeu8); }
  if cpu.zf { pc = 0x432ea8; continue; }
  pc = 0x432e9e; continue;
 }
 0x432e9e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432ea8; continue;
 }
 0x432ea8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add30u32); cpu.esi = v; }
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432eec); f_0041cf66(cpu); if cpu.returned_to != 0x432eec { return; }
  { cpu.popad(); }
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432f0f); cpu.call(t); if cpu.returned_to != 0x432f0f { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x432f3a; continue; }
  pc = 0x432f19; continue;
 }
 0x432f19 => {
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x432f3a; continue; }
  pc = 0x432f27; continue;
 }
 0x432f27 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6ecu32) & 0x1u8; cpu.w8(0x58c6ecu32, v); }
  { let v: u8 = cpu.r8(0x58c6ecu32) | (cpu.ecx as u8); cpu.w8(0x58c6ecu32, v); }
  pc = 0x432f3a; continue;
 }
 0x432f3a => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x432f4c; continue;
 }
 0x432f4c => {
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6ecu32), 0xeu8); }
  if cpu.zf { pc = 0x432f5b; continue; }
  pc = 0x432f58; continue;
 }
 0x432f58 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x432f5b; continue;
 }
 0x432f5b => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432f74); cpu.call(t); if cpu.returned_to != 0x432f74 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6ecu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6ecu32), 0xeu8); }
  if cpu.zf { pc = 0x432f94; continue; }
  pc = 0x432f8a; continue;
 }
 0x432f8a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x432f94; continue;
 }
 0x432f94 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add34u32); cpu.esi = v; }
  { let v: u32 = 0xa4u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x432fd8); f_0041cf66(cpu); if cpu.returned_to != 0x432fd8 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x432ffb); cpu.call(t); if cpu.returned_to != 0x432ffb { return; }
  { cpu.popad(); }
  pc = 0x432ffc; continue;
 }
 0x432ffc => {
  { let v: u32 = 0x80u32; cpu.w32(0x578d6fu32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x0u8); }
  if cpu.zf { pc = 0x43304e; continue; }
  pc = 0x43302f; continue;
 }
 0x43302f => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x58c6e6u32)); cpu.w8(0x58c6e6u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5a2387u32, v); }
  pc = 0x43304e; continue;
 }
 0x43304e => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x192eu32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433071); cpu.call(t); if cpu.returned_to != 0x433071 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae100u32); cpu.esi = v; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4330ab); f_0041cf66(cpu); if cpu.returned_to != 0x4330ab { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = cpu.r32(0x5add40u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4330f8); f_0041cf66(cpu); if cpu.returned_to != 0x4330f8 { return; }
  { let v: u32 = cpu.r32(0x5ae114u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x51u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433130); f_0041cf66(cpu); if cpu.returned_to != 0x433130 { return; }
  { let v: u32 = cpu.r32(0x5ae0f0u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x61u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433168); f_0041cf66(cpu); if cpu.returned_to != 0x433168 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433169(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433169); }
 let mut pc: u32 = 0x433169;
 loop { match pc {
 0x433169 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1921u32; cpu.ecx = v; }
  cpu.push(0x43317d); f_00438fb9(cpu); if cpu.returned_to != 0x43317d { return; }
  { let v: u32 = 0x11u32; cpu.w32(0x578d6fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4331ba; continue; }
  pc = 0x433199; continue;
 }
 0x433199 => {
  { let v: u8 = cpu.r8(0x58c6edu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4331ba; continue; }
  pc = 0x4331a7; continue;
 }
 0x4331a7 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6edu32) & 0x1u8; cpu.w8(0x58c6edu32, v); }
  { let v: u8 = cpu.r8(0x58c6edu32) | (cpu.ecx as u8); cpu.w8(0x58c6edu32, v); }
  pc = 0x4331ba; continue;
 }
 0x4331ba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae104u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4331d0); f_0041ce38(cpu); if cpu.returned_to != 0x4331d0 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6edu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6edu32), 0xeu8); }
  if cpu.zf { pc = 0x433205; continue; }
  pc = 0x433202; continue;
 }
 0x433202 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x433205; continue;
 }
 0x433205 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43321e); cpu.call(t); if cpu.returned_to != 0x43321e { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6edu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6edu32), 0xeu8); }
  if cpu.zf { pc = 0x43323e; continue; }
  pc = 0x433234; continue;
 }
 0x433234 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x43323e; continue;
 }
 0x43323e => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae104u32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433288); f_0041cf66(cpu); if cpu.returned_to != 0x433288 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4332c9; continue; }
  pc = 0x433293; continue;
 }
 0x433293 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4332a6; continue; }
  pc = 0x43329d; continue;
 }
 0x43329d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4332c9; continue; }
  pc = 0x4332a6; continue;
 }
 0x4332a6 => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4332c8); cpu.call(t); if cpu.returned_to != 0x4332c8 { return; }
  { cpu.popad(); }
  pc = 0x4332c9; continue;
 }
 0x4332c9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4332fc; continue; }
  pc = 0x4332db; continue;
 }
 0x4332db => {
  { let v: u8 = cpu.r8(0x58c6eeu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4332fc; continue; }
  pc = 0x4332e9; continue;
 }
 0x4332e9 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6eeu32) & 0x1u8; cpu.w8(0x58c6eeu32, v); }
  { let v: u8 = cpu.r8(0x58c6eeu32) | (cpu.ecx as u8); cpu.w8(0x58c6eeu32, v); }
  pc = 0x4332fc; continue;
 }
 0x4332fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae108u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x433312); f_0041ce38(cpu); if cpu.returned_to != 0x433312 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eeu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6eeu32), 0xeu8); }
  if cpu.zf { pc = 0x433347; continue; }
  pc = 0x433344; continue;
 }
 0x433344 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x433347; continue;
 }
 0x433347 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433360); cpu.call(t); if cpu.returned_to != 0x433360 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6eeu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6eeu32), 0xeu8); }
  if cpu.zf { pc = 0x433380; continue; }
  pc = 0x433376; continue;
 }
 0x433376 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x433380; continue;
 }
 0x433380 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae108u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4333ca); f_0041cf66(cpu); if cpu.returned_to != 0x4333ca { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43340b; continue; }
  pc = 0x4333d5; continue;
 }
 0x4333d5 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4333e8; continue; }
  pc = 0x4333df; continue;
 }
 0x4333df => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43340b; continue; }
  pc = 0x4333e8; continue;
 }
 0x4333e8 => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43340a); cpu.call(t); if cpu.returned_to != 0x43340a { return; }
  { cpu.popad(); }
  pc = 0x43340b; continue;
 }
 0x43340b => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43343e; continue; }
  pc = 0x43341d; continue;
 }
 0x43341d => {
  { let v: u8 = cpu.r8(0x5a236cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43343e; continue; }
  pc = 0x43342b; continue;
 }
 0x43342b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236cu32) & 0x1u8; cpu.w8(0x5a236cu32, v); }
  { let v: u8 = cpu.r8(0x5a236cu32) | (cpu.ecx as u8); cpu.w8(0x5a236cu32, v); }
  pc = 0x43343e; continue;
 }
 0x43343e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x433454); f_0041ce38(cpu); if cpu.returned_to != 0x433454 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236cu32), 0xeu8); }
  if cpu.zf { pc = 0x433489; continue; }
  pc = 0x433486; continue;
 }
 0x433486 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x433489; continue;
 }
 0x433489 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4334a2); cpu.call(t); if cpu.returned_to != 0x4334a2 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236cu32), 0xeu8); }
  if cpu.zf { pc = 0x4334c2; continue; }
  pc = 0x4334b8; continue;
 }
 0x4334b8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4334c2; continue;
 }
 0x4334c2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add38u32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43350c); f_0041cf66(cpu); if cpu.returned_to != 0x43350c { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43354d; continue; }
  pc = 0x433517; continue;
 }
 0x433517 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43352a; continue; }
  pc = 0x433521; continue;
 }
 0x433521 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43354d; continue; }
  pc = 0x43352a; continue;
 }
 0x43352a => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43354c); cpu.call(t); if cpu.returned_to != 0x43354c { return; }
  { cpu.popad(); }
  pc = 0x43354d; continue;
 }
 0x43354d => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x433580; continue; }
  pc = 0x43355f; continue;
 }
 0x43355f => {
  { let v: u8 = cpu.r8(0x5a236du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x433580; continue; }
  pc = 0x43356d; continue;
 }
 0x43356d => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a236du32) & 0x1u8; cpu.w8(0x5a236du32, v); }
  { let v: u8 = cpu.r8(0x5a236du32) | (cpu.ecx as u8); cpu.w8(0x5a236du32, v); }
  pc = 0x433580; continue;
 }
 0x433580 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x433596); f_0041ce38(cpu); if cpu.returned_to != 0x433596 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a236du32), 0xeu8); }
  if cpu.zf { pc = 0x4335cb; continue; }
  pc = 0x4335c8; continue;
 }
 0x4335c8 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4335cb; continue;
 }
 0x4335cb => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4335e4); cpu.call(t); if cpu.returned_to != 0x4335e4 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a236du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a236du32), 0xeu8); }
  if cpu.zf { pc = 0x433604; continue; }
  pc = 0x4335fa; continue;
 }
 0x4335fa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x433604; continue;
 }
 0x433604 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add3cu32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43364e); f_0041cf66(cpu); if cpu.returned_to != 0x43364e { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43368f; continue; }
  pc = 0x433659; continue;
 }
 0x433659 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43366c; continue; }
  pc = 0x433663; continue;
 }
 0x433663 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43368f; continue; }
  pc = 0x43366c; continue;
 }
 0x43366c => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43368e); cpu.call(t); if cpu.returned_to != 0x43368e { return; }
  { cpu.popad(); }
  pc = 0x43368f; continue;
 }
 0x43368f => {
  { cpu.sub8(cpu.r8(0x58c6edu32), 0x0u8); }
  if !cpu.zf { pc = 0x433925; continue; }
  pc = 0x43369c; continue;
 }
 0x43369c => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4336cf; continue; }
  pc = 0x4336ae; continue;
 }
 0x4336ae => {
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4336cf; continue; }
  pc = 0x4336bc; continue;
 }
 0x4336bc => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6efu32) & 0x1u8; cpu.w8(0x58c6efu32, v); }
  { let v: u8 = cpu.r8(0x58c6efu32) | (cpu.ecx as u8); cpu.w8(0x58c6efu32, v); }
  pc = 0x4336cf; continue;
 }
 0x4336cf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae10cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4336e5); f_0041ce38(cpu); if cpu.returned_to != 0x4336e5 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x86u32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6efu32), 0xeu8); }
  if cpu.zf { pc = 0x43371a; continue; }
  pc = 0x433717; continue;
 }
 0x433717 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43371a; continue;
 }
 0x43371a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433733); cpu.call(t); if cpu.returned_to != 0x433733 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6efu32), 0xeu8); }
  if cpu.zf { pc = 0x433753; continue; }
  pc = 0x433749; continue;
 }
 0x433749 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x433753; continue;
 }
 0x433753 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae10cu32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43379d); f_0041cf66(cpu); if cpu.returned_to != 0x43379d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4337de; continue; }
  pc = 0x4337a8; continue;
 }
 0x4337a8 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4337bb; continue; }
  pc = 0x4337b2; continue;
 }
 0x4337b2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4337de; continue; }
  pc = 0x4337bb; continue;
 }
 0x4337bb => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4337dd); cpu.call(t); if cpu.returned_to != 0x4337dd { return; }
  { cpu.popad(); }
  pc = 0x4337de; continue;
 }
 0x4337de => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x433811; continue; }
  pc = 0x4337f0; continue;
 }
 0x4337f0 => {
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x433811; continue; }
  pc = 0x4337fe; continue;
 }
 0x4337fe => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6f0u32) & 0x1u8; cpu.w8(0x58c6f0u32, v); }
  { let v: u8 = cpu.r8(0x58c6f0u32) | (cpu.ecx as u8); cpu.w8(0x58c6f0u32, v); }
  pc = 0x433811; continue;
 }
 0x433811 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae110u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x433827); f_0041ce38(cpu); if cpu.returned_to != 0x433827 { return; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9eu32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6f0u32), 0xeu8); }
  if cpu.zf { pc = 0x43385c; continue; }
  pc = 0x433859; continue;
 }
 0x433859 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43385c; continue;
 }
 0x43385c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433875); cpu.call(t); if cpu.returned_to != 0x433875 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6f0u32), 0xeu8); }
  if cpu.zf { pc = 0x433895; continue; }
  pc = 0x43388b; continue;
 }
 0x43388b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x433895; continue;
 }
 0x433895 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae110u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4338df); f_0041cf66(cpu); if cpu.returned_to != 0x4338df { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x433920; continue; }
  pc = 0x4338ea; continue;
 }
 0x4338ea => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4338fd; continue; }
  pc = 0x4338f4; continue;
 }
 0x4338f4 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x433920; continue; }
  pc = 0x4338fd; continue;
 }
 0x4338fd => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43391f); cpu.call(t); if cpu.returned_to != 0x43391f { return; }
  { cpu.popad(); }
  pc = 0x433920; continue;
 }
 0x433920 => {
  pc = 0x433afd; continue;
 }
 0x433925 => {
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43394f; continue; }
  pc = 0x43392e; continue;
 }
 0x43392e => {
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43394f; continue; }
  pc = 0x43393c; continue;
 }
 0x43393c => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6efu32) & 0x1u8; cpu.w8(0x58c6efu32, v); }
  { let v: u8 = cpu.r8(0x58c6efu32) | (cpu.ecx as u8); cpu.w8(0x58c6efu32, v); }
  pc = 0x43394f; continue;
 }
 0x43394f => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6efu32), 0xeu8); }
  if cpu.zf { pc = 0x433970; continue; }
  pc = 0x43396d; continue;
 }
 0x43396d => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x433970; continue;
 }
 0x433970 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433989); cpu.call(t); if cpu.returned_to != 0x433989 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6efu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6efu32), 0xeu8); }
  if cpu.zf { pc = 0x4339a9; continue; }
  pc = 0x43399f; continue;
 }
 0x43399f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4339a9; continue;
 }
 0x4339a9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae10cu32); cpu.esi = v; }
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4339ed); f_0041cf66(cpu); if cpu.returned_to != 0x4339ed { return; }
  { cpu.popad(); }
  { let v: u32 = 0x86u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433a10); cpu.call(t); if cpu.returned_to != 0x433a10 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x433a3b; continue; }
  pc = 0x433a1a; continue;
 }
 0x433a1a => {
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x433a3b; continue; }
  pc = 0x433a28; continue;
 }
 0x433a28 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c6f0u32) & 0x1u8; cpu.w8(0x58c6f0u32, v); }
  { let v: u8 = cpu.r8(0x58c6f0u32) | (cpu.ecx as u8); cpu.w8(0x58c6f0u32, v); }
  pc = 0x433a3b; continue;
 }
 0x433a3b => {
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x58c6f0u32), 0xeu8); }
  if cpu.zf { pc = 0x433a5c; continue; }
  pc = 0x433a59; continue;
 }
 0x433a59 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x433a5c; continue;
 }
 0x433a5c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x190au32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433a75); cpu.call(t); if cpu.returned_to != 0x433a75 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x58c6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x58c6f0u32), 0xeu8); }
  if cpu.zf { pc = 0x433a95; continue; }
  pc = 0x433a8b; continue;
 }
 0x433a8b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x433a95; continue;
 }
 0x433a95 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ae110u32); cpu.esi = v; }
  { let v: u32 = 0xa2u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433ad9); f_0041cf66(cpu); if cpu.returned_to != 0x433ad9 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x9eu32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1929u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x433afc); cpu.call(t); if cpu.returned_to != 0x433afc { return; }
  { cpu.popad(); }
  pc = 0x433afd; continue;
 }
 0x433afd => {
  { let v: u32 = 0x80u32; cpu.w32(0x578d6fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a2387u32, v); }
  { let v: u32 = cpu.r32(0x5ae118u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433b53); f_0041cf66(cpu); if cpu.returned_to != 0x433b53 { return; }
  { let v: u32 = cpu.r32(0x5add44u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x51u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433b8b); f_0041cf66(cpu); if cpu.returned_to != 0x433b8b { return; }
  { let v: u32 = cpu.r32(0x5ae11cu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x61u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x433bc3); f_0041cf66(cpu); if cpu.returned_to != 0x433bc3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433bc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433bc4); }
 let mut pc: u32 = 0x433bc4;
 loop { match pc {
 0x433bc4 => {
  { let v: u32 = 0x5a0458u32; cpu.esi = v; }
  cpu.push(0x433bce); f_0041b447(cpu); if cpu.returned_to != 0x433bce { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433bcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433bcf); }
 let mut pc: u32 = 0x433bcf;
 loop { match pc {
 0x433bcf => {
  { let v: u32 = 0x5a0590u32; cpu.esi = v; }
  cpu.push(0x433bd9); f_0041b447(cpu); if cpu.returned_to != 0x433bd9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433bda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433bda); }
 let mut pc: u32 = 0x433bda;
 loop { match pc {
 0x433bda => {
  { let v: u32 = 0x5a06e4u32; cpu.esi = v; }
  cpu.push(0x433be4); f_0041b447(cpu); if cpu.returned_to != 0x433be4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433be5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433be5); }
 let mut pc: u32 = 0x433be5;
 loop { match pc {
 0x433be5 => {
  cpu.push(0x433bea); f_0041b41d(cpu); if cpu.returned_to != 0x433bea { return; }
  { let v: u32 = 0x5a0458u32; cpu.ebp = v; }
  cpu.push(0x433bf4); f_0041b3c6(cpu); if cpu.returned_to != 0x433bf4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433bf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433bf5); }
 let mut pc: u32 = 0x433bf5;
 loop { match pc {
 0x433bf5 => {
  cpu.push(0x433bfa); f_0041b41d(cpu); if cpu.returned_to != 0x433bfa { return; }
  { let v: u32 = 0x5a0590u32; cpu.ebp = v; }
  cpu.push(0x433c04); f_0041b3c6(cpu); if cpu.returned_to != 0x433c04 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433c05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433c05); }
 let mut pc: u32 = 0x433c05;
 loop { match pc {
 0x433c05 => {
  cpu.push(0x433c0a); f_0041b41d(cpu); if cpu.returned_to != 0x433c0a { return; }
  { let v: u32 = 0x5a06e4u32; cpu.ebp = v; }
  cpu.push(0x433c14); f_0041b3c6(cpu); if cpu.returned_to != 0x433c14 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433c15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433c15); }
 let mut pc: u32 = 0x433c15;
 loop { match pc {
 0x433c15 => {
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e64u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236fu32, v); }
  { cpu.sub8(cpu.r8(0x5a23c8u32), 0x0u8); }
  if cpu.zf { pc = 0x433c40; continue; }
  pc = 0x433c37; continue;
 }
 0x433c37 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a236eu32, v); }
  pc = 0x433c47; continue;
 }
 0x433c40 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a236fu32, v); }
  pc = 0x433c47; continue;
 }
 0x433c47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1fbcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e62u32, v); }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x576e6eu32, v); }
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if cpu.zf { pc = 0x433c70; continue; }
  pc = 0x433c67; continue;
 }
 0x433c67 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x433c80; continue; }
  pc = 0x433c70; continue;
 }
 0x433c70 => {
  { let v: u8 = 0x0u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575f30u32, v); }
  pc = 0x433c8e; continue;
 }
 0x433c80 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f30u32, v); }
  pc = 0x433c8e; continue;
 }
 0x433c8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433c8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433c8f); }
 let mut pc: u32 = 0x433c8f;
 loop { match pc {
 0x433c8f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236bu32, v); }
  { cpu.sub8(cpu.r8(0x5a2361u32), 0x0u8); }
  if cpu.zf { pc = 0x433cd2; continue; }
  pc = 0x433cad; continue;
 }
 0x433cad => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a236au32, v); }
  { cpu.sub8(cpu.r8(0x5a2361u32), 0x1u8); }
  if cpu.zf { pc = 0x433cd2; continue; }
  pc = 0x433cc4; continue;
 }
 0x433cc4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a236bu32, v); }
  pc = 0x433cd2; continue;
 }
 0x433cd2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e9u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x433d20; continue; }
  pc = 0x433cf0; continue;
 }
 0x433cf0 => {
  { cpu.sub8(cpu.r8(0x58c887u32), 0x0u8); }
  if cpu.zf { pc = 0x433d4e; continue; }
  pc = 0x433cf9; continue;
 }
 0x433cf9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e8u32, v); }
  { cpu.sub8(cpu.r8(0x58c887u32), 0x1u8); }
  if cpu.zf { pc = 0x433d4e; continue; }
  pc = 0x433d10; continue;
 }
 0x433d10 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e9u32, v); }
  pc = 0x433d4e; continue;
 }
 0x433d20 => {
  { cpu.sub8(cpu.r8(0x58c888u32), 0x0u8); }
  if cpu.zf { pc = 0x433d4e; continue; }
  pc = 0x433d29; continue;
 }
 0x433d29 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e8u32, v); }
  { cpu.sub8(cpu.r8(0x58c888u32), 0x1u8); }
  if cpu.zf { pc = 0x433d4e; continue; }
  pc = 0x433d40; continue;
 }
 0x433d40 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e9u32, v); }
  pc = 0x433d4e; continue;
 }
 0x433d4e => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ecu32, v); }
  { cpu.sub8(cpu.r8(0x58c884u32), 0x0u8); }
  if cpu.zf { pc = 0x433d91; continue; }
  pc = 0x433d6c; continue;
 }
 0x433d6c => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6ebu32, v); }
  { cpu.sub8(cpu.r8(0x58c884u32), 0x1u8); }
  if cpu.zf { pc = 0x433d91; continue; }
  pc = 0x433d83; continue;
 }
 0x433d83 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6ecu32, v); }
  pc = 0x433d91; continue;
 }
 0x433d91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433d92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433d92); }
 let mut pc: u32 = 0x433d92;
 loop { match pc {
 0x433d92 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6edu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6eeu32, v); }
  { cpu.and32(cpu.r32(0x58c87au32), 0x200u32); }
  if cpu.zf { pc = 0x433db5; continue; }
  pc = 0x433dac; continue;
 }
 0x433dac => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6eeu32, v); }
  pc = 0x433dbc; continue;
 }
 0x433db5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6edu32, v); }
  pc = 0x433dbc; continue;
 }
 0x433dbc => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a236cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236du32, v); }
  { cpu.sub8(cpu.r8(0x58c86eu32), 0x0u8); }
  if cpu.zf { pc = 0x433ddc; continue; }
  pc = 0x433dd3; continue;
 }
 0x433dd3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a236cu32, v); }
  pc = 0x433de3; continue;
 }
 0x433ddc => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a236du32, v); }
  pc = 0x433de3; continue;
 }
 0x433de3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6efu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f0u32, v); }
  { cpu.sub32(cpu.r32(0x58c83eu32), 0x0u32); }
  if cpu.zf { pc = 0x433e03; continue; }
  pc = 0x433dfa; continue;
 }
 0x433dfa => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6efu32, v); }
  pc = 0x433e0a; continue;
 }
 0x433e03 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f0u32, v); }
  pc = 0x433e0a; continue;
 }
 0x433e0a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433e0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433e0b); }
 let mut pc: u32 = 0x433e0b;
 loop { match pc {
 0x433e0b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a23c8u32, v); }
  { cpu.sub8(cpu.r8(0x5a236eu32), 0x0u8); }
  if cpu.zf { pc = 0x433e22; continue; }
  pc = 0x433e1b; continue;
 }
 0x433e1b => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a23c8u32, v); }
  pc = 0x433e22; continue;
 }
 0x433e22 => {
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if cpu.zf { pc = 0x433eb8; continue; }
  pc = 0x433e2f; continue;
 }
 0x433e2f => {
  { cpu.sub8(cpu.r8(0x575f2fu32), 0x0u8); }
  if cpu.zf { pc = 0x433e79; continue; }
  pc = 0x433e38; continue;
 }
 0x433e38 => {
  { cpu.sub8(cpu.r8(0x576c0bu32), 0x0u8); }
  if cpu.zf { pc = 0x433e72; continue; }
  pc = 0x433e41; continue;
 }
 0x433e41 => {
  { cpu.pushad(); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = 0xecu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x433e68); f_00413337(cpu); if cpu.returned_to != 0x433e68 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  pc = 0x433eb8; continue;
 }
 0x433e72 => {
  cpu.push(0x433e77); f_0041770d(cpu); if cpu.returned_to != 0x433e77 { return; }
  pc = 0x433eb8; continue;
 }
 0x433e79 => {
  { cpu.sub8(cpu.r8(0x576c0bu32), 0x0u8); }
  if !cpu.zf { pc = 0x433eb3; continue; }
  pc = 0x433e82; continue;
 }
 0x433e82 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = 0xecu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x433ea9); f_00413337(cpu); if cpu.returned_to != 0x433ea9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { cpu.popad(); }
  pc = 0x433eb8; continue;
 }
 0x433eb3 => {
  cpu.push(0x433eb8); f_004175d9(cpu); if cpu.returned_to != 0x433eb8 { return; }
  pc = 0x433eb8; continue;
 }
 0x433eb8 => {
  { let v: u16 = cpu.r16(0x576e62u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fbcu32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x433ef0; continue; }
  pc = 0x433ec6; continue;
 }
 0x433ec6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68b0u32, v); }
  { let v: u8 = 0xedu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x433ee9); f_00413337(cpu); if cpu.returned_to != 0x433ee9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  pc = 0x433ef0; continue;
 }
 0x433ef0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433ef1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433ef1); }
 let mut pc: u32 = 0x433ef1;
 loop { match pc {
 0x433ef1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2361u32, v); }
  { cpu.sub8(cpu.r8(0x5a2369u32), 0x0u8); }
  if cpu.zf { pc = 0x433f03; continue; }
  pc = 0x433f01; continue;
 }
 0x433f01 => {
  pc = 0x433f25; continue;
 }
 0x433f03 => {
  { cpu.sub8(cpu.r8(0x5a236au32), 0x0u8); }
  if cpu.zf { pc = 0x433f15; continue; }
  pc = 0x433f0c; continue;
 }
 0x433f0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2361u32, v); }
  pc = 0x433f25; continue;
 }
 0x433f15 => {
  { cpu.sub8(cpu.r8(0x5a236bu32), 0x0u8); }
  if cpu.zf { pc = 0x433f25; continue; }
  pc = 0x433f1e; continue;
 }
 0x433f1e => {
  { let v: u8 = 0x2u8; cpu.w8(0x5a2361u32, v); }
  pc = 0x433f25; continue;
 }
 0x433f25 => {
  { cpu.sub8(cpu.r8(0x58c6e7u32), 0x0u8); }
  if !cpu.zf { pc = 0x433f40; continue; }
  pc = 0x433f2e; continue;
 }
 0x433f2e => {
  { cpu.sub8(cpu.r8(0x58c6e8u32), 0x0u8); }
  if !cpu.zf { pc = 0x433f49; continue; }
  pc = 0x433f37; continue;
 }
 0x433f37 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x433f3e); f_00413fda(cpu); if cpu.returned_to != 0x433f3e { return; }
  pc = 0x433f50; continue;
 }
 0x433f40 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x433f47); f_00413fda(cpu); if cpu.returned_to != 0x433f47 { return; }
  pc = 0x433f50; continue;
 }
 0x433f49 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x433f50); f_00413fda(cpu); if cpu.returned_to != 0x433f50 { return; }
  pc = 0x433f50; continue;
 }
 0x433f50 => {
  { cpu.sub8(cpu.r8(0x58c6eau32), 0x1u8); }
  if cpu.zf { pc = 0x433fa6; continue; }
  pc = 0x433f59; continue;
 }
 0x433f59 => {
  { cpu.sub8(cpu.r8(0x58c6ebu32), 0x1u8); }
  if cpu.zf { pc = 0x433f84; continue; }
  pc = 0x433f62; continue;
 }
 0x433f62 => {
  { let v: u8 = 0x2u8; cpu.w8(0x58c884u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x58c886u32, v); }
  { let v: u16 = 0x28u16; cpu.w16(0x58c83au32, v); }
  { let v: u16 = 0x50u16; cpu.w16(0x58c83cu32, v); }
  pc = 0x433fc6; continue;
 }
 0x433f84 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c884u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c886u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x58c83au32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x58c83cu32, v); }
  pc = 0x433fc6; continue;
 }
 0x433fa6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c884u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c886u32, v); }
  { let v: u16 = 0x3cu16; cpu.w16(0x58c83au32, v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x58c83cu32, v); }
  pc = 0x433fc6; continue;
 }
 0x433fc6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00433fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x433fc7); }
 let mut pc: u32 = 0x433fc7;
 loop { match pc {
 0x433fc7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c86eu32, v); }
  { cpu.sub8(cpu.r8(0x5a236cu32), 0x0u8); }
  if !cpu.zf { pc = 0x433fde; continue; }
  pc = 0x433fd7; continue;
 }
 0x433fd7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c86eu32, v); }
  pc = 0x433fde; continue;
 }
 0x433fde => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x200u32); }
  if cpu.zf { pc = 0x433ffa; continue; }
  pc = 0x433fea; continue;
 }
 0x433fea => {
  { cpu.sub8(cpu.r8(0x58c6eeu32), 0x0u8); }
  if !cpu.zf { pc = 0x434008; continue; }
  pc = 0x433ff3; continue;
 }
 0x433ff3 => {
  cpu.push(0x433ff8); f_0042c1ec(cpu); if cpu.returned_to != 0x433ff8 { return; }
  pc = 0x434008; continue;
 }
 0x433ffa => {
  { cpu.sub8(cpu.r8(0x58c6edu32), 0x0u8); }
  if !cpu.zf { pc = 0x434008; continue; }
  pc = 0x434003; continue;
 }
 0x434003 => {
  cpu.push(0x434008); f_0042c1ec(cpu); if cpu.returned_to != 0x434008 { return; }
  pc = 0x434008; continue;
 }
 0x434008 => {
  cpu.push(0x43400d); f_0043400e(cpu); if cpu.returned_to != 0x43400d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043400e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43400e); }
 let mut pc: u32 = 0x43400e;
 loop { match pc {
 0x43400e => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c83eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c842u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c846u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c84au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c84eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c852u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c856u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c85au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c85eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c862u32, v); }
  { cpu.sub8(cpu.r8(0x58c6efu32), 0x0u8); }
  if !cpu.zf { pc = 0x4340af; continue; }
  pc = 0x43407b; continue;
 }
 0x43407b => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c842u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c84au32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c852u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c85au32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c862u32, v); }
  pc = 0x4340e1; continue;
 }
 0x4340af => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c83eu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c846u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c84eu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c856u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c85eu32, v); }
  pc = 0x4340e1; continue;
 }
 0x4340e1 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x200u32); }
  if cpu.zf { pc = 0x434117; continue; }
  pc = 0x4340ed; continue;
 }
 0x4340ed => {
  { let v: u32 = cpu.r32(0x58c84eu32) ^ 0x1u32; cpu.w32(0x58c84eu32, v); }
  { let v: u32 = cpu.r32(0x58c856u32) ^ 0x1u32; cpu.w32(0x58c856u32, v); }
  { let v: u32 = cpu.r32(0x58c85eu32) ^ 0x1u32; cpu.w32(0x58c85eu32, v); }
  { let v: u32 = cpu.r32(0x58c852u32) ^ 0x1u32; cpu.w32(0x58c852u32, v); }
  { let v: u32 = cpu.r32(0x58c85au32) ^ 0x1u32; cpu.w32(0x58c85au32, v); }
  { let v: u32 = cpu.xor32(cpu.r32(0x58c862u32), 0x1u32); cpu.w32(0x58c862u32, v); }
  pc = 0x434117; continue;
 }
 0x434117 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434243(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434243); }
 let mut pc: u32 = 0x434243;
 loop { match pc {
 0x434243 => {
  { cpu.sub32(cpu.r32(0x58c7eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x43424d; continue; }
  pc = 0x43424c; continue;
 }
 0x43424c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43424d => {
  cpu.push(0x434252); f_00434354(cpu); if cpu.returned_to != 0x434252 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x43425b); cpu.call(t); if cpu.returned_to != 0x43425b { return; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434271); cpu.call(t); if cpu.returned_to != 0x434271 { return; }
  { cpu.sub32(cpu.r32(0x58c7e6u32), 0x0u32); }
  if cpu.zf { pc = 0x4342cc; continue; }
  pc = 0x43427a; continue;
 }
 0x43427a => {
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1bu32); cpu.ebx = v; }
  { let v: u32 = 0x1903u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x58c7f2u32), 0x0u32); }
  if cpu.zf { pc = 0x43429a; continue; }
  pc = 0x434299; continue;
 }
 0x434299 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x43429a; continue;
 }
 0x43429a => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4342a0); cpu.call(t); if cpu.returned_to != 0x4342a0 { return; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c816u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c7e6u32); cpu.esi = v; }
  cpu.push(0x4342cc); f_0041cf66(cpu); if cpu.returned_to != 0x4342cc { return; }
  pc = 0x4342cc; continue;
 }
 0x4342cc => {
  { cpu.sub32(cpu.r32(0x58c7eau32), 0x0u32); }
  if cpu.zf { pc = 0x434327; continue; }
  pc = 0x4342d5; continue;
 }
 0x4342d5 => {
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x42u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1bu32); cpu.ebx = v; }
  { let v: u32 = 0x1903u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x58c7f6u32), 0x0u32); }
  if cpu.zf { pc = 0x4342f5; continue; }
  pc = 0x4342f4; continue;
 }
 0x4342f4 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x4342f5; continue;
 }
 0x4342f5 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4342fb); cpu.call(t); if cpu.returned_to != 0x4342fb { return; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c81au32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c7eau32); cpu.esi = v; }
  cpu.push(0x434327); f_0041cf66(cpu); if cpu.returned_to != 0x434327 { return; }
  pc = 0x434327; continue;
 }
 0x434327 => {
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c81eu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c7e2u32); cpu.esi = v; }
  cpu.push(0x434353); f_0041cf66(cpu); if cpu.returned_to != 0x434353 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434354); }
 let mut pc: u32 = 0x434354;
 loop { match pc {
 0x434354 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c832u32, v); }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if !cpu.zf { pc = 0x4343ad; continue; }
  pc = 0x434367; continue;
 }
 0x434367 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c812u32, v); }
  { cpu.sub32(cpu.r32(0x58c7f2u32), 0x0u32); }
  if cpu.zf { pc = 0x434381; continue; }
  pc = 0x43437a; continue;
 }
 0x43437a => {
  cpu.push(0x43437f); f_00434416(cpu); if cpu.returned_to != 0x43437f { return; }
  if cpu.cf { pc = 0x4343e9; continue; }
  pc = 0x434381; continue;
 }
 0x434381 => {
  { cpu.sub32(cpu.r32(0x58c7f6u32), 0x0u32); }
  if cpu.zf { pc = 0x434391; continue; }
  pc = 0x43438a; continue;
 }
 0x43438a => {
  cpu.push(0x43438f); f_00434444(cpu); if cpu.returned_to != 0x43438f { return; }
  if cpu.cf { pc = 0x4343e9; continue; }
  pc = 0x434391; continue;
 }
 0x434391 => {
  { cpu.and8(cpu.r8(0x58c836u32), 0x2u8); }
  if !cpu.zf { pc = 0x434415; continue; }
  pc = 0x43439a; continue;
 }
 0x43439a => {
  cpu.push(0x43439f); f_00434472(cpu); if cpu.returned_to != 0x43439f { return; }
  if !cpu.cf { pc = 0x434415; continue; }
  pc = 0x4343a1; continue;
 }
 0x4343a1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c7f6u32, v); }
  pc = 0x4343e9; continue;
 }
 0x4343ad => {
  { cpu.and8(cpu.r8(0x58c836u32), 0x1u8); }
  if !cpu.zf { pc = 0x4343c6; continue; }
  pc = 0x4343b6; continue;
 }
 0x4343b6 => {
  { cpu.sub32(cpu.r32(0x58c812u32), 0x0u32); }
  if cpu.zf { pc = 0x4343c6; continue; }
  pc = 0x4343bf; continue;
 }
 0x4343bf => {
  cpu.push(0x4343c4); f_0043447e(cpu); if cpu.returned_to != 0x4343c4 { return; }
  pc = 0x434415; continue;
 }
 0x4343c6 => {
  cpu.push(0x4343cb); f_00434513(cpu); if cpu.returned_to != 0x4343cb { return; }
  if !cpu.cf { pc = 0x434415; continue; }
  pc = 0x4343cd; continue;
 }
 0x4343cd => {
  cpu.push(0x4343d2); f_00434551(cpu); if cpu.returned_to != 0x4343d2 { return; }
  { let v: u32 = cpu.r32(0x58c7f2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x58c7f6u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x434415; continue; }
  pc = 0x4343e2; continue;
 }
 0x4343e2 => {
  cpu.push(0x4343e7); f_00434592(cpu); if cpu.returned_to != 0x4343e7 { return; }
  pc = 0x434415; continue;
 }
 0x4343e9 => {
  { cpu.sub32(cpu.r32(0x58c7f2u32), 0x0u32); }
  if cpu.zf { pc = 0x4343fb; continue; }
  pc = 0x4343f2; continue;
 }
 0x4343f2 => {
  { let v: u32 = cpu.or32(cpu.r32(0x58c832u32), 0x1u32); cpu.w32(0x58c832u32, v); }
  pc = 0x43440b; continue;
 }
 0x4343fb => {
  { cpu.sub32(cpu.r32(0x58c7f6u32), 0x0u32); }
  if cpu.zf { pc = 0x434415; continue; }
  pc = 0x434404; continue;
 }
 0x434404 => {
  { let v: u32 = cpu.or32(cpu.r32(0x58c832u32), 0x2u32); cpu.w32(0x58c832u32, v); }
  pc = 0x43440b; continue;
 }
 0x43440b => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c7eeu32, v); }
  pc = 0x434415; continue;
 }
 0x434415 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434416(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434416); }
 let mut pc: u32 = 0x434416;
 loop { match pc {
 0x434416 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43441f); f_00418beb(cpu); if cpu.returned_to != 0x43441f { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1bu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xeu32); cpu.edx = v; }
  cpu.push(0x434443); f_004714a5(cpu); if cpu.returned_to != 0x434443 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434444); }
 let mut pc: u32 = 0x434444;
 loop { match pc {
 0x434444 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43444d); f_00418beb(cpu); if cpu.returned_to != 0x43444d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x42u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1bu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xeu32); cpu.edx = v; }
  cpu.push(0x434471); f_004714a5(cpu); if cpu.returned_to != 0x434471 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434472(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434472); }
 let mut pc: u32 = 0x434472;
 loop { match pc {
 0x434472 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x58c82au32)); cpu.w32(0x58c82au32, v); }
  if !cpu.zf { pc = 0x43447c; continue; }
  pc = 0x43447a; continue;
 }
 0x43447a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43447c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043447e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43447e); }
 let mut pc: u32 = 0x43447e;
 loop { match pc {
 0x43447e => {
  cpu.push(0x434483); f_004345c2(cpu); if cpu.returned_to != 0x434483 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43448c); f_00418beb(cpu); if cpu.returned_to != 0x43448c { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x58c80au32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x58c80eu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c7dau32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c7deu32, v); }
  { cpu.sub32(cpu.r32(0x58c7dau32), 0x0u32); }
  if !cpu.sf { pc = 0x4344b6; continue; }
  pc = 0x4344ac; continue;
 }
 0x4344ac => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c7dau32, v); }
  pc = 0x4344b6; continue;
 }
 0x4344b6 => {
  { cpu.sub32(cpu.r32(0x58c7deu32), 0x0u32); }
  if !cpu.sf { pc = 0x4344c9; continue; }
  pc = 0x4344bf; continue;
 }
 0x4344bf => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c7deu32, v); }
  pc = 0x4344c9; continue;
 }
 0x4344c9 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4344dc; continue; }
  pc = 0x4344d7; continue;
 }
 0x4344d7 => {
  { let v: u32 = 0x210u32; cpu.eax = v; }
  pc = 0x4344dc; continue;
 }
 0x4344dc => {
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x58c7dau32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4344ec; continue; }
  pc = 0x4344e7; continue;
 }
 0x4344e7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x58c7dau32, v); }
  pc = 0x4344ec; continue;
 }
 0x4344ec => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x434502; continue; }
  pc = 0x4344fa; continue;
 }
 0x4344fa => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  pc = 0x434502; continue;
 }
 0x434502 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x2eu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x58c7deu32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x434512; continue; }
  pc = 0x43450d; continue;
 }
 0x43450d => {
  { let v: u32 = cpu.eax; cpu.w32(0x58c7deu32, v); }
  pc = 0x434512; continue;
 }
 0x434512 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434513); }
 let mut pc: u32 = 0x434513;
 loop { match pc {
 0x434513 => {
  { cpu.sub32(cpu.r32(0x58c7eeu32), 0x0u32); }
  if cpu.zf { pc = 0x43454f; continue; }
  pc = 0x43451c; continue;
 }
 0x43451c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x434525); f_00418beb(cpu); if cpu.returned_to != 0x434525 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c7dau32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c7deu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x60u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x2eu32); cpu.edx = v; }
  cpu.push(0x43454b); f_004714a5(cpu); if cpu.returned_to != 0x43454b { return; }
  if !cpu.cf { pc = 0x43454f; continue; }
  pc = 0x43454d; continue;
 }
 0x43454d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43454f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434551(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434551); }
 let mut pc: u32 = 0x434551;
 loop { match pc {
 0x434551 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c7f2u32, v); }
  cpu.push(0x434560); f_00434416(cpu); if cpu.returned_to != 0x434560 { return; }
  if !cpu.cf { pc = 0x434571; continue; }
  pc = 0x434562; continue;
 }
 0x434562 => {
  cpu.push(0x434567); f_004345c2(cpu); if cpu.returned_to != 0x434567 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x58c7f2u32, v); }
  pc = 0x434571; continue;
 }
 0x434571 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c7f6u32, v); }
  cpu.push(0x434580); f_00434444(cpu); if cpu.returned_to != 0x434580 { return; }
  if !cpu.cf { pc = 0x434591; continue; }
  pc = 0x434582; continue;
 }
 0x434582 => {
  cpu.push(0x434587); f_004345c2(cpu); if cpu.returned_to != 0x434587 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x58c7f6u32, v); }
  pc = 0x434591; continue;
 }
 0x434591 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434592(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434592); }
 let mut pc: u32 = 0x434592;
 loop { match pc {
 0x434592 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43459b); f_00418beb(cpu); if cpu.returned_to != 0x43459b { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x58c7dau32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x58c7deu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c80au32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c80eu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c812u32, v); }
  cpu.push(0x4345c1); f_004345c2(cpu); if cpu.returned_to != 0x4345c1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004345c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4345c2); }
 let mut pc: u32 = 0x4345c2;
 loop { match pc {
 0x4345c2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c82eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c82au32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434612(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434612); }
 let mut pc: u32 = 0x434612;
 loop { match pc {
 0x434612 => {
  { cpu.sub32(cpu.r32(0x58c505u32), 0x0u32); }
  if cpu.zf { pc = 0x43461c; continue; }
  pc = 0x43461b; continue;
 }
 0x43461b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43461c => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c539u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c53du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c509u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c50du32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c545u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c541u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4f1u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c4f5u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x58c4f9u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x58c4fdu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x58c501u32, v); }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c4fdu32); cpu.esi = v; }
  cpu.push(0x43467d); f_0041ce38(cpu); if cpu.returned_to != 0x43467d { return; }
  { cpu.sub32(cpu.eax, 0x22u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x434701; continue; }
  pc = 0x434682; continue;
 }
 0x434682 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c52du32, v); }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c501u32); cpu.esi = v; }
  cpu.push(0x4346a1); f_0041ce38(cpu); if cpu.returned_to != 0x4346a1 { return; }
  { cpu.sub32(cpu.eax, 0x22u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x434701; continue; }
  pc = 0x4346a6; continue;
 }
 0x4346a6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c531u32, v); }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c4f9u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x4346c6); f_0041ce38(cpu); if cpu.returned_to != 0x4346c6 { return; }
  { cpu.sub32(cpu.eax, 0x6au32); }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x35u32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c535u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c509u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c50du32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c505u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c529u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434701 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c505u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58c529u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434717(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434717); }
 let mut pc: u32 = 0x434717;
 loop { match pc {
 0x434717 => {
  cpu.push(0x43471c); f_00413e2a(cpu); if cpu.returned_to != 0x43471c { return; }
  { cpu.sub32(cpu.r32(0x58c505u32), 0x0u32); }
  if !cpu.zf { pc = 0x434726; continue; }
  pc = 0x434725; continue;
 }
 0x434725 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434726 => {
  cpu.push(0x43472b); f_00434867(cpu); if cpu.returned_to != 0x43472b { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x434734); cpu.call(t); if cpu.returned_to != 0x434734 { return; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43474a); cpu.call(t); if cpu.returned_to != 0x43474a { return; }
  { cpu.sub32(cpu.r32(0x58c4fdu32), 0x0u32); }
  if cpu.zf { pc = 0x4347a5; continue; }
  pc = 0x434753; continue;
 }
 0x434753 => {
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x29u32); cpu.ebx = v; }
  { let v: u32 = 0x1903u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x58c509u32), 0x0u32); }
  if cpu.zf { pc = 0x434773; continue; }
  pc = 0x434772; continue;
 }
 0x434772 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x434773; continue;
 }
 0x434773 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434779); cpu.call(t); if cpu.returned_to != 0x434779 { return; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c52du32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c4fdu32); cpu.esi = v; }
  cpu.push(0x4347a5); f_0041cf66(cpu); if cpu.returned_to != 0x4347a5 { return; }
  pc = 0x4347a5; continue;
 }
 0x4347a5 => {
  { cpu.sub32(cpu.r32(0x58c501u32), 0x0u32); }
  if cpu.zf { pc = 0x434800; continue; }
  pc = 0x4347ae; continue;
 }
 0x4347ae => {
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x42u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x29u32); cpu.ebx = v; }
  { let v: u32 = 0x1903u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x58c50du32), 0x0u32); }
  if cpu.zf { pc = 0x4347ce; continue; }
  pc = 0x4347cd; continue;
 }
 0x4347cd => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x4347ce; continue;
 }
 0x4347ce => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4347d4); cpu.call(t); if cpu.returned_to != 0x4347d4 { return; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c531u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c501u32); cpu.esi = v; }
  cpu.push(0x434800); f_0041cf66(cpu); if cpu.returned_to != 0x434800 { return; }
  pc = 0x434800; continue;
 }
 0x434800 => {
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x58c535u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1au32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c4f9u32); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x434833); cpu.call(t); if cpu.returned_to != 0x434833 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x43483a); f_0041cf66(cpu); if cpu.returned_to != 0x43483a { return; }
  { cpu.sub32(cpu.r32(0x58c549u32), 0x0u32); }
  if cpu.zf { pc = 0x434866; continue; }
  pc = 0x434843; continue;
 }
 0x434843 => {
  { let v: u32 = 0x58c62du32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x434866; continue; }
  pc = 0x434852; continue;
 }
 0x434852 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x434858); f_004414ab(cpu); if cpu.returned_to != 0x434858 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c559u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x434866; continue;
 }
 0x434866 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434867); }
 let mut pc: u32 = 0x434867;
 loop { match pc {
 0x434867 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c549u32, v); }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if !cpu.zf { pc = 0x4348b7; continue; }
  pc = 0x43487a; continue;
 }
 0x43487a => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c529u32, v); }
  { cpu.sub32(cpu.r32(0x58c509u32), 0x0u32); }
  if cpu.zf { pc = 0x434894; continue; }
  pc = 0x43488d; continue;
 }
 0x43488d => {
  cpu.push(0x434892); f_00434917(cpu); if cpu.returned_to != 0x434892 { return; }
  if cpu.cf { pc = 0x4348ea; continue; }
  pc = 0x434894; continue;
 }
 0x434894 => {
  { cpu.sub32(cpu.r32(0x58c50du32), 0x0u32); }
  if cpu.zf { pc = 0x4348a4; continue; }
  pc = 0x43489d; continue;
 }
 0x43489d => {
  cpu.push(0x4348a2); f_00434945(cpu); if cpu.returned_to != 0x4348a2 { return; }
  if cpu.cf { pc = 0x4348ea; continue; }
  pc = 0x4348a4; continue;
 }
 0x4348a4 => {
  cpu.push(0x4348a9); f_00434973(cpu); if cpu.returned_to != 0x4348a9 { return; }
  if !cpu.cf { pc = 0x434916; continue; }
  pc = 0x4348ab; continue;
 }
 0x4348ab => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c50du32, v); }
  pc = 0x4348ea; continue;
 }
 0x4348b7 => {
  { cpu.sub32(cpu.r32(0x58c529u32), 0x0u32); }
  if cpu.zf { pc = 0x4348c7; continue; }
  pc = 0x4348c0; continue;
 }
 0x4348c0 => {
  cpu.push(0x4348c5); f_0043497f(cpu); if cpu.returned_to != 0x4348c5 { return; }
  pc = 0x434916; continue;
 }
 0x4348c7 => {
  cpu.push(0x4348cc); f_00434a14(cpu); if cpu.returned_to != 0x4348cc { return; }
  if !cpu.cf { pc = 0x434916; continue; }
  pc = 0x4348ce; continue;
 }
 0x4348ce => {
  cpu.push(0x4348d3); f_00434a49(cpu); if cpu.returned_to != 0x4348d3 { return; }
  { let v: u32 = cpu.r32(0x58c509u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x58c50du32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x434916; continue; }
  pc = 0x4348e3; continue;
 }
 0x4348e3 => {
  cpu.push(0x4348e8); f_00434a8a(cpu); if cpu.returned_to != 0x4348e8 { return; }
  pc = 0x434916; continue;
 }
 0x4348ea => {
  { cpu.sub32(cpu.r32(0x58c509u32), 0x0u32); }
  if cpu.zf { pc = 0x4348fc; continue; }
  pc = 0x4348f3; continue;
 }
 0x4348f3 => {
  { let v: u32 = cpu.or32(cpu.r32(0x58c549u32), 0x1u32); cpu.w32(0x58c549u32, v); }
  pc = 0x43490c; continue;
 }
 0x4348fc => {
  { cpu.sub32(cpu.r32(0x58c50du32), 0x0u32); }
  if cpu.zf { pc = 0x434916; continue; }
  pc = 0x434905; continue;
 }
 0x434905 => {
  { let v: u32 = cpu.or32(cpu.r32(0x58c549u32), 0x2u32); cpu.w32(0x58c549u32, v); }
  pc = 0x43490c; continue;
 }
 0x43490c => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c505u32, v); }
  pc = 0x434916; continue;
 }
 0x434916 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434917(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434917); }
 let mut pc: u32 = 0x434917;
 loop { match pc {
 0x434917 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x434920); f_00418beb(cpu); if cpu.returned_to != 0x434920 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x29u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xau32); cpu.edx = v; }
  cpu.push(0x434944); f_004714a5(cpu); if cpu.returned_to != 0x434944 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434945(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434945); }
 let mut pc: u32 = 0x434945;
 loop { match pc {
 0x434945 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43494e); f_00418beb(cpu); if cpu.returned_to != 0x43494e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x42u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x29u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xau32); cpu.edx = v; }
  cpu.push(0x434972); f_004714a5(cpu); if cpu.returned_to != 0x434972 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434973(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434973); }
 let mut pc: u32 = 0x434973;
 loop { match pc {
 0x434973 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x58c541u32)); cpu.w32(0x58c541u32, v); }
  if !cpu.zf { pc = 0x43497d; continue; }
  pc = 0x43497b; continue;
 }
 0x43497b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43497d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043497f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43497f); }
 let mut pc: u32 = 0x43497f;
 loop { match pc {
 0x43497f => {
  cpu.push(0x434984); f_00434aba(cpu); if cpu.returned_to != 0x434984 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x43498d); f_00418beb(cpu); if cpu.returned_to != 0x43498d { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x58c521u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x58c525u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4f1u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c4f5u32, v); }
  { cpu.sub32(cpu.r32(0x58c4f1u32), 0x0u32); }
  if !cpu.sf { pc = 0x4349b7; continue; }
  pc = 0x4349ad; continue;
 }
 0x4349ad => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c4f1u32, v); }
  pc = 0x4349b7; continue;
 }
 0x4349b7 => {
  { cpu.sub32(cpu.r32(0x58c4f5u32), 0x0u32); }
  if !cpu.sf { pc = 0x4349ca; continue; }
  pc = 0x4349c0; continue;
 }
 0x4349c0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c4f5u32, v); }
  pc = 0x4349ca; continue;
 }
 0x4349ca => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4349dd; continue; }
  pc = 0x4349d8; continue;
 }
 0x4349d8 => {
  { let v: u32 = 0x210u32; cpu.eax = v; }
  pc = 0x4349dd; continue;
 }
 0x4349dd => {
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x58c4f1u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4349ed; continue; }
  pc = 0x4349e8; continue;
 }
 0x4349e8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x58c4f1u32, v); }
  pc = 0x4349ed; continue;
 }
 0x4349ed => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x434a03; continue; }
  pc = 0x4349fb; continue;
 }
 0x4349fb => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  pc = 0x434a03; continue;
 }
 0x434a03 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x58c4f5u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x434a13; continue; }
  pc = 0x434a0e; continue;
 }
 0x434a0e => {
  { let v: u32 = cpu.eax; cpu.w32(0x58c4f5u32, v); }
  pc = 0x434a13; continue;
 }
 0x434a13 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434a14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434a14); }
 let mut pc: u32 = 0x434a14;
 loop { match pc {
 0x434a14 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x434a1d); f_00418beb(cpu); if cpu.returned_to != 0x434a1d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x60u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x3cu32); cpu.edx = v; }
  cpu.push(0x434a43); f_004714a5(cpu); if cpu.returned_to != 0x434a43 { return; }
  if !cpu.cf { pc = 0x434a47; continue; }
  pc = 0x434a45; continue;
 }
 0x434a45 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434a47 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434a49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434a49); }
 let mut pc: u32 = 0x434a49;
 loop { match pc {
 0x434a49 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c509u32, v); }
  cpu.push(0x434a58); f_00434917(cpu); if cpu.returned_to != 0x434a58 { return; }
  if !cpu.cf { pc = 0x434a69; continue; }
  pc = 0x434a5a; continue;
 }
 0x434a5a => {
  cpu.push(0x434a5f); f_00434aba(cpu); if cpu.returned_to != 0x434a5f { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x58c509u32, v); }
  pc = 0x434a69; continue;
 }
 0x434a69 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58c50du32, v); }
  cpu.push(0x434a78); f_00434945(cpu); if cpu.returned_to != 0x434a78 { return; }
  if !cpu.cf { pc = 0x434a89; continue; }
  pc = 0x434a7a; continue;
 }
 0x434a7a => {
  cpu.push(0x434a7f); f_00434aba(cpu); if cpu.returned_to != 0x434a7f { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x58c50du32, v); }
  pc = 0x434a89; continue;
 }
 0x434a89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434a8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434a8a); }
 let mut pc: u32 = 0x434a8a;
 loop { match pc {
 0x434a8a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x434a93); f_00418beb(cpu); if cpu.returned_to != 0x434a93 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x58c4f1u32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x58c4f5u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c521u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x58c525u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58c529u32, v); }
  cpu.push(0x434ab9); f_00434aba(cpu); if cpu.returned_to != 0x434ab9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434aba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434aba); }
 let mut pc: u32 = 0x434aba;
 loop { match pc {
 0x434aba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c545u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c541u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434ac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434ac7); }
 let mut pc: u32 = 0x434ac7;
 loop { match pc {
 0x434ac7 => {
  { cpu.pushad(); }
  cpu.push(0x434acd); f_00418beb(cpu); if cpu.returned_to != 0x434acd { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5928b1u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5928b3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c631u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c632u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c633u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c837u32, v); }
  cpu.push(0x434afb); f_00434c3c(cpu); if cpu.returned_to != 0x434afb { return; }
  if !cpu.cf { pc = 0x434b04; continue; }
  pc = 0x434afd; continue;
 }
 0x434afd => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c631u32, v); }
  pc = 0x434b04; continue;
 }
 0x434b04 => {
  cpu.push(0x434b09); f_00434b4c(cpu); if cpu.returned_to != 0x434b09 { return; }
  if !cpu.cf { pc = 0x434b12; continue; }
  pc = 0x434b0b; continue;
 }
 0x434b0b => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c632u32, v); }
  pc = 0x434b12; continue;
 }
 0x434b12 => {
  cpu.push(0x434b17); f_00434bf1(cpu); if cpu.returned_to != 0x434b17 { return; }
  if !cpu.cf { pc = 0x434b20; continue; }
  pc = 0x434b19; continue;
 }
 0x434b19 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c633u32, v); }
  pc = 0x434b20; continue;
 }
 0x434b20 => {
  cpu.push(0x434b25); f_00434513(cpu); if cpu.returned_to != 0x434b25 { return; }
  if !cpu.cf { pc = 0x434b2e; continue; }
  pc = 0x434b27; continue;
 }
 0x434b27 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c837u32, v); }
  pc = 0x434b2e; continue;
 }
 0x434b2e => {
  { let v: u8 = cpu.r8(0x58c631u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x58c632u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x58c633u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x58c837u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c634u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434b4c); }
 let mut pc: u32 = 0x434b4c;
 loop { match pc {
 0x434b4c => {
  { cpu.sub32(cpu.r32(0x58c505u32), 0x0u32); }
  if cpu.zf { pc = 0x434b8d; continue; }
  pc = 0x434b55; continue;
 }
 0x434b55 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x5928b1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5928b3u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58c4f1u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58c4f5u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x60u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x3cu32); cpu.edx = v; }
  cpu.push(0x434b89); f_004714a5(cpu); if cpu.returned_to != 0x434b89 { return; }
  if !cpu.cf { pc = 0x434b8d; continue; }
  pc = 0x434b8b; continue;
 }
 0x434b8b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434b8d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434b8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434b8f); }
 let mut pc: u32 = 0x434b8f;
 loop { match pc {
 0x434b8f => {
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x576e8cu32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x434bf0; continue; }
  pc = 0x434b9c; continue;
 }
 0x434b9c => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x434bc9; continue; }
  pc = 0x434ba0; continue;
 }
 0x434ba0 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x434bba); f_00417eb0(cpu); if cpu.returned_to != 0x434bba { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6a00u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6a04u32, v); }
  pc = 0x434bf0; continue;
 }
 0x434bc9 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x434be3); f_00417eb0(cpu); if cpu.returned_to != 0x434be3 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6a00u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6a04u32, v); }
  pc = 0x434bf0; continue;
 }
 0x434bf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434bf1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x434bf1); } r_00434bf1(cpu, 0x434bf1); }
pub fn f_00434c01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x434c01); } r_00434bf1(cpu, 0x434c01); }
fn r_00434bf1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x434bf1 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x434c3a; continue; }
  pc = 0x434bfa; continue;
 }
 0x434bfa => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x5928b1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5928b3u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x434c3a; continue; }
  pc = 0x434c26; continue;
 }
 0x434c01 => {
  { let v: u8 = 0x28u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x5928b3u32); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x434c3a; continue; }
  pc = 0x434c26; continue;
 }
 0x434c26 => {
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x434c3a; continue; }
  pc = 0x434c2a; continue;
 }
 0x434c2a => {
  { let v: u32 = cpu.eax.wrapping_add(0x70u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x76u32); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x434c3a; continue; }
  pc = 0x434c34; continue;
 }
 0x434c34 => {
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x434c3a; continue; }
  pc = 0x434c38; continue;
 }
 0x434c38 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434c3a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434c3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434c3c); }
 let mut pc: u32 = 0x434c3c;
 loop { match pc {
 0x434c3c => {
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x0u32); }
  if cpu.zf { pc = 0x434c7d; continue; }
  pc = 0x434c45; continue;
 }
 0x434c45 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x5928b1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5928b3u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x70u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x3cu32); cpu.edx = v; }
  cpu.push(0x434c7c); f_004714a5(cpu); if cpu.returned_to != 0x434c7c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434c7d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434d0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x434d0b); } r_00434d0b(cpu, 0x434d0b); }
pub fn f_00434e59(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x434e59); } r_00434d0b(cpu, 0x434e59); }
fn r_00434d0b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x434d0b => {
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x0u32); }
  if !cpu.zf { pc = 0x434d15; continue; }
  pc = 0x434d14; continue;
 }
 0x434d14 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x434d15 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  cpu.push(0x434d21); f_0043503b(cpu); if cpu.returned_to != 0x434d21 { return; }
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x1u32); }
  if cpu.zf { pc = 0x434f11; continue; }
  pc = 0x434d2e; continue;
 }
 0x434d2e => {
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x2u32); }
  if cpu.zf { pc = 0x434e24; continue; }
  pc = 0x434d3b; continue;
 }
 0x434d3b => {
  { let v: u32 = cpu.r32(0x5ae0d8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ae0dcu32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x65u32; cpu.edx = v; }
  cpu.push(0x434d60); f_00434ffc(cpu); if cpu.returned_to != 0x434d60 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x434dad; continue; }
  pc = 0x434d69; continue;
 }
 0x434d69 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x434d89; continue; }
  pc = 0x434d7e; continue;
 }
 0x434d7e => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x434d89; continue;
 }
 0x434d89 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434dac); cpu.call(t); if cpu.returned_to != 0x434dac { return; }
  { cpu.popad(); }
  pc = 0x434dad; continue;
 }
 0x434dad => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x434dfa; continue; }
  pc = 0x434db6; continue;
 }
 0x434db6 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434dd6; continue; }
  pc = 0x434dcb; continue;
 }
 0x434dcb => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x434dd6; continue;
 }
 0x434dd6 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434df9); cpu.call(t); if cpu.returned_to != 0x434df9 { return; }
  { cpu.popad(); }
  pc = 0x434dfa; continue;
 }
 0x434dfa => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x434fec; continue; }
  pc = 0x434e07; continue;
 }
 0x434e07 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434ffb; continue; }
  pc = 0x434e14; continue;
 }
 0x434e14 => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c4e9u32, v); }
  if !cpu.zf { pc = 0x434fec; continue; }
  pc = 0x434e24; continue;
 }
 0x434e24 => {
  { let v: u32 = cpu.r32(0x5ae078u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ae07cu32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x65u32; cpu.edx = v; }
  cpu.push(0x434e49); f_00434ffc(cpu); if cpu.returned_to != 0x434e49 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x434e96; continue; }
  pc = 0x434e52; continue;
 }
 0x434e52 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x434e72; continue; }
  pc = 0x434e67; continue;
 }
 0x434e59 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x434e72; continue; }
  pc = 0x434e67; continue;
 }
 0x434e67 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x434e72; continue;
 }
 0x434e72 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434e95); cpu.call(t); if cpu.returned_to != 0x434e95 { return; }
  { cpu.popad(); }
  pc = 0x434e96; continue;
 }
 0x434e96 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x434ee3; continue; }
  pc = 0x434e9f; continue;
 }
 0x434e9f => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434ebf; continue; }
  pc = 0x434eb4; continue;
 }
 0x434eb4 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x434ebf; continue;
 }
 0x434ebf => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434ee2); cpu.call(t); if cpu.returned_to != 0x434ee2 { return; }
  { cpu.popad(); }
  pc = 0x434ee3; continue;
 }
 0x434ee3 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x434fec; continue; }
  pc = 0x434ef0; continue;
 }
 0x434ef0 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434ffb; continue; }
  pc = 0x434efd; continue;
 }
 0x434efd => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c4edu32, v); }
  pc = 0x434fec; continue;
 }
 0x434f11 => {
  { let v: u32 = cpu.r32(0x5add60u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5add64u32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x65u32; cpu.edx = v; }
  cpu.push(0x434f36); f_00434ffc(cpu); if cpu.returned_to != 0x434f36 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x434f83; continue; }
  pc = 0x434f3f; continue;
 }
 0x434f3f => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x434f5f; continue; }
  pc = 0x434f54; continue;
 }
 0x434f54 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x434f5f; continue;
 }
 0x434f5f => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434f82); cpu.call(t); if cpu.returned_to != 0x434f82 { return; }
  { cpu.popad(); }
  pc = 0x434f83; continue;
 }
 0x434f83 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x434fd0; continue; }
  pc = 0x434f8c; continue;
 }
 0x434f8c => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434fac; continue; }
  pc = 0x434fa1; continue;
 }
 0x434fa1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x434fac; continue;
 }
 0x434fac => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x434fcf); cpu.call(t); if cpu.returned_to != 0x434fcf { return; }
  { cpu.popad(); }
  pc = 0x434fd0; continue;
 }
 0x434fd0 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x434fec; continue; }
  pc = 0x434fd9; continue;
 }
 0x434fd9 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x434ffb; continue; }
  pc = 0x434fe2; continue;
 }
 0x434fe2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c4e1u32, v); }
  pc = 0x434fec; continue;
 }
 0x434fec => {
  cpu.push(0x434ff1); f_0041b41d(cpu); if cpu.returned_to != 0x434ff1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x58c4e5u32, v); }
  pc = 0x434ffb; continue;
 }
 0x434ffb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00434ffc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x434ffc); }
 let mut pc: u32 = 0x434ffc;
 loop { match pc {
 0x434ffc => {
  { cpu.pushad(); }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x435018); f_0041cf66(cpu); if cpu.returned_to != 0x435018 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43503a); f_0041cf66(cpu); if cpu.returned_to != 0x43503a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043503b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43503b); } r_0043503b(cpu, 0x43503b); }
pub fn f_00435041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x435041); } r_0043503b(cpu, 0x435041); }
fn r_0043503b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43503b => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43505c); cpu.call(t); if cpu.returned_to != 0x43505c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x435041 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x191fu32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x191fu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43505c); cpu.call(t); if cpu.returned_to != 0x43505c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043505d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43505d); }
 let mut pc: u32 = 0x43505d;
 loop { match pc {
 0x43505d => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x435067; continue; }
  pc = 0x435066; continue;
 }
 0x435066 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x435067 => {
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x0u32); }
  if cpu.zf { pc = 0x435071; continue; }
  pc = 0x435070; continue;
 }
 0x435070 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x435071 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x435090; continue; }
  pc = 0x43507a; continue;
 }
 0x43507a => {
  { let v: u32 = 0xa0u32; cpu.w32(0x5a23b6u32, v); }
  { let v: u32 = 0x64u32; cpu.w32(0x5a23bau32, v); }
  pc = 0x4350a4; continue;
 }
 0x435090 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5a23b6u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5a23bau32, v); }
  pc = 0x4350a4; continue;
 }
 0x4350a4 => {
  cpu.push(0x4350a9); f_004350aa(cpu); if cpu.returned_to != 0x4350a9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004350aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4350aa); }
 let mut pc: u32 = 0x4350aa;
 loop { match pc {
 0x4350aa => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u32 = 0x5a0390u32; cpu.ebp = v; }
  cpu.push(0x4350f6); f_0041b3c6(cpu); if cpu.returned_to != 0x4350f6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004350f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4350f7); }
 let mut pc: u32 = 0x4350f7;
 loop { match pc {
 0x4350f7 => {
  { let v: u32 = 0x6u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x43511b); f_0041b41d(cpu); if cpu.returned_to != 0x43511b { return; }
  { let v: u32 = 0x5a0c00u32; cpu.ebp = v; }
  cpu.push(0x435125); f_0041b3c6(cpu); if cpu.returned_to != 0x435125 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00435126(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x435126); }
 let mut pc: u32 = 0x435126;
 loop { match pc {
 0x435126 => {
  { let v: u32 = 0x2u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  cpu.push(0x435143); f_0043d1db(cpu); if cpu.returned_to != 0x435143 { return; }
  if cpu.cf { pc = 0x435151; continue; }
  pc = 0x435145; continue;
 }
 0x435145 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  pc = 0x43515f; continue;
 }
 0x435151 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2322u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2323u32, v); }
  pc = 0x43515f; continue;
 }
 0x43515f => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x435169); f_00439c80(cpu); if cpu.returned_to != 0x435169 { return; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x435176; continue; }
  pc = 0x435172; continue;
 }
 0x435172 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43517c; continue;
 }
 0x435176 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43517c; continue; }
  pc = 0x43517a; continue;
 }
 0x43517a => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43517c; continue;
 }
 0x43517c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x435194); f_0041b41d(cpu); if cpu.returned_to != 0x435194 { return; }
  { let v: u32 = 0x5a0a1cu32; cpu.ebp = v; }
  cpu.push(0x43519e); f_0041b3c6(cpu); if cpu.returned_to != 0x43519e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043519f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43519f); }
 let mut pc: u32 = 0x43519f;
 loop { match pc {
 0x43519f => {
  { let v: u32 = 0x3u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575569u32, v); }
  cpu.push(0x4351d1); f_00439d32(cpu); if cpu.returned_to != 0x4351d1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2322u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2323u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4351f0); f_00439c80(cpu); if cpu.returned_to != 0x4351f0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x435203); f_0041b41d(cpu); if cpu.returned_to != 0x435203 { return; }
  { let v: u32 = 0x5a0b00u32; cpu.ebp = v; }
  cpu.push(0x43520d); f_0041b3c6(cpu); if cpu.returned_to != 0x43520d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043520e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43520e); } r_0043520e(cpu, 0x43520e); }
pub fn f_00435243(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x435243); } r_0043520e(cpu, 0x435243); }
fn r_0043520e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43520e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4d1u32, v); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4d5u32, v); }
  { let v: u8 = cpu.r8(0x5922f9u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4d9u32, v); }
  { let v: u8 = cpu.r8(0x5922fbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4ddu32, v); }
  { let v: u32 = 0x5u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e5u32, v); }
  { cpu.sub8(cpu.r8(0x58c88au32), 0x0u8); }
  if !cpu.zf { pc = 0x43527e; continue; }
  pc = 0x435275; continue;
 }
 0x435243 => {
  { let v: u32 = cpu.eax.wrapping_add(0x56d39cu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x5a2380u32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5a23b1u32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x56c9d3u32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x58c6e4u32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x58c6e5u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x58c88a3du32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x58c88a3du32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x43527e; continue; }
  pc = 0x435275; continue;
 }
 0x435275 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e4u32, v); }
  pc = 0x435285; continue;
 }
 0x43527e => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e5u32, v); }
  pc = 0x435285; continue;
 }
 0x435285 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2370u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2371u32, v); }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x4352a5; continue; }
  pc = 0x43529c; continue;
 }
 0x43529c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2370u32, v); }
  pc = 0x4352ac; continue;
 }
 0x4352a5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2371u32, v); }
  pc = 0x4352ac; continue;
 }
 0x4352ac => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2373u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2374u32, v); }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x4352cc; continue; }
  pc = 0x4352c3; continue;
 }
 0x4352c3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2373u32, v); }
  pc = 0x4352d3; continue;
 }
 0x4352cc => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2374u32, v); }
  pc = 0x4352d3; continue;
 }
 0x4352d3 => {
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2376u32, v); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2378u32, v); }
  cpu.push(0x4352f4); f_0041b41d(cpu); if cpu.returned_to != 0x4352f4 { return; }
  { let v: u32 = 0x5a0858u32; cpu.ebp = v; }
  cpu.push(0x4352fe); f_0041b3c6(cpu); if cpu.returned_to != 0x4352fe { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043530d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43530d); }
 let mut pc: u32 = 0x43530d;
 loop { match pc {
 0x43530d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x58c86eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4c5u32, v); }
  { let v: u8 = cpu.r8(0x5a2361u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4c9u32, v); }
  { let v: u8 = cpu.r8(0x5a23c8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4cdu32, v); }
  { let v: u32 = 0x4u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f1u32, v); }
  cpu.push(0x43535f); f_00433c15(cpu); if cpu.returned_to != 0x43535f { return; }
  cpu.push(0x435364); f_0041b41d(cpu); if cpu.returned_to != 0x435364 { return; }
  { let v: u32 = 0x5a0458u32; cpu.ebp = v; }
  cpu.push(0x43536e); f_0041b3c6(cpu); if cpu.returned_to != 0x43536e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043536f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43536f); }
 let mut pc: u32 = 0x43536f;
 loop { match pc {
 0x43536f => {
  cpu.push(0x435374); f_00437e82(cpu); if cpu.returned_to != 0x435374 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5922f9u32, v); }
  { cpu.sub8(cpu.r8(0x5a2370u32), 0x0u8); }
  if cpu.zf { pc = 0x43538b; continue; }
  pc = 0x435384; continue;
 }
 0x435384 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5922f9u32, v); }
  pc = 0x43538b; continue;
 }
 0x43538b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5922fbu32, v); }
  { cpu.sub8(cpu.r8(0x5a2373u32), 0x0u8); }
  if cpu.zf { pc = 0x4353a2; continue; }
  pc = 0x43539b; continue;
 }
 0x43539b => {
  { let v: u8 = 0x1u8; cpu.w8(0x5922fbu32, v); }
  pc = 0x4353a2; continue;
 }
 0x4353a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fdu32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922ffu32, v); }
  cpu.push(0x4353c5); f_00437eba(cpu); if cpu.returned_to != 0x4353c5 { return; }
  { cpu.sub8(cpu.r8(0x58c6e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x4353de; continue; }
  pc = 0x4353ce; continue;
 }
 0x4353ce => {
  { let v: u8 = 0xffu8; cpu.w8(0x58c88au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x58c88bu32, v); }
  pc = 0x4353ec; continue;
 }
 0x4353de => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c88au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c88bu32, v); }
  pc = 0x4353ec; continue;
 }
 0x4353ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004353ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4353ed); }
 let mut pc: u32 = 0x4353ed;
 loop { match pc {
 0x4353ed => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if !cpu.zf { pc = 0x4353f7; continue; }
  pc = 0x4353f6; continue;
 }
 0x4353f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4353f7 => {
  { cpu.sub32(cpu.r32(0x58c635u32), 0x0u32); }
  if !cpu.zf { pc = 0x4356bb; continue; }
  pc = 0x435404; continue;
 }
 0x435404 => {
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x435449; continue; }
  pc = 0x43540d; continue;
 }
 0x43540d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x4354da; continue;
 }
 0x435449 => {
  { cpu.sub8(cpu.r8(0x5a2364u32), 0x0u8); }
  if cpu.zf { pc = 0x435466; continue; }
  pc = 0x435452; continue;
 }
 0x435452 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2364u32)); cpu.w8(0x5a2364u32, v); }
  { cpu.sub8(cpu.r8(0x5a2364u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x435466; continue; }
  pc = 0x435461; continue;
 }
 0x435461 => {
  cpu.push(0x435466); f_00435126(cpu); if cpu.returned_to != 0x435466 { return; }
  pc = 0x435466; continue;
 }
 0x435466 => {
  { cpu.sub8(cpu.r8(0x5a2365u32), 0x0u8); }
  if cpu.zf { pc = 0x435483; continue; }
  pc = 0x43546f; continue;
 }
 0x43546f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2365u32)); cpu.w8(0x5a2365u32, v); }
  { cpu.sub8(cpu.r8(0x5a2365u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x435483; continue; }
  pc = 0x43547e; continue;
 }
 0x43547e => {
  cpu.push(0x435483); f_0043519f(cpu); if cpu.returned_to != 0x435483 { return; }
  pc = 0x435483; continue;
 }
 0x435483 => {
  { cpu.sub8(cpu.r8(0x5a2368u32), 0x0u8); }
  if cpu.zf { pc = 0x4354a0; continue; }
  pc = 0x43548c; continue;
 }
 0x43548c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2368u32)); cpu.w8(0x5a2368u32, v); }
  { cpu.sub8(cpu.r8(0x5a2368u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4354a0; continue; }
  pc = 0x43549b; continue;
 }
 0x43549b => {
  cpu.push(0x4354a0); f_0043520e(cpu); if cpu.returned_to != 0x4354a0 { return; }
  pc = 0x4354a0; continue;
 }
 0x4354a0 => {
  { cpu.sub8(cpu.r8(0x5a2367u32), 0x0u8); }
  if cpu.zf { pc = 0x4354bd; continue; }
  pc = 0x4354a9; continue;
 }
 0x4354a9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2367u32)); cpu.w8(0x5a2367u32, v); }
  { cpu.sub8(cpu.r8(0x5a2367u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4354bd; continue; }
  pc = 0x4354b8; continue;
 }
 0x4354b8 => {
  cpu.push(0x4354bd); f_0043530d(cpu); if cpu.returned_to != 0x4354bd { return; }
  pc = 0x4354bd; continue;
 }
 0x4354bd => {
  { cpu.sub8(cpu.r8(0x5a2366u32), 0x0u8); }
  if cpu.zf { pc = 0x4354da; continue; }
  pc = 0x4354c6; continue;
 }
 0x4354c6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2366u32)); cpu.w8(0x5a2366u32, v); }
  { cpu.sub8(cpu.r8(0x5a2366u32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4354da; continue; }
  pc = 0x4354d5; continue;
 }
 0x4354d5 => {
  cpu.push(0x4354da); f_004350f7(cpu); if cpu.returned_to != 0x4354da { return; }
  pc = 0x4354da; continue;
 }
 0x4354da => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x1u32); }
  if cpu.zf { pc = 0x4356ad; continue; }
  pc = 0x4354e7; continue;
 }
 0x4354e7 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x4u32); }
  if cpu.zf { pc = 0x43556f; continue; }
  pc = 0x4354f0; continue;
 }
 0x4354f0 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x5u32); }
  if cpu.zf { pc = 0x43564f; continue; }
  pc = 0x4354fd; continue;
 }
 0x4354fd => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x6u32); }
  if cpu.zf { pc = 0x43550b; continue; }
  pc = 0x435506; continue;
 }
 0x435506 => {
  pc = 0x4356d1; continue;
 }
 0x43550b => {
  { let v: u8 = cpu.r8(0x5a2362u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5a2363u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x43552a); f_004356d9(cpu); if cpu.returned_to != 0x43552a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2363u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2362u32, v); }
  cpu.push(0x43553b); f_00436004(cpu); if cpu.returned_to != 0x43553b { return; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x435560; continue; }
  pc = 0x435544; continue;
 }
 0x435544 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4356d1; continue; }
  pc = 0x435551; continue;
 }
 0x435551 => {
  { let v: u32 = 0x1u32; cpu.w32(0x58c4e1u32, v); }
  pc = 0x4356bb; continue;
 }
 0x435560 => {
  cpu.push(0x435565); f_0041b41d(cpu); if cpu.returned_to != 0x435565 { return; }
  cpu.push(0x43556a); f_004350aa(cpu); if cpu.returned_to != 0x43556a { return; }
  pc = 0x4356ad; continue;
 }
 0x43556f => {
  cpu.push(0x435574); f_00435f6c(cpu); if cpu.returned_to != 0x435574 { return; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x4355cd; continue; }
  pc = 0x43557d; continue;
 }
 0x43557d => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4356d1; continue; }
  pc = 0x43558a; continue;
 }
 0x43558a => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if cpu.zf { pc = 0x4355b9; continue; }
  pc = 0x435593; continue;
 }
 0x435593 => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x4355a8; continue; }
  pc = 0x43559c; continue;
 }
 0x43559c => {
  cpu.push(0x4355a1); f_00433ef1(cpu); if cpu.returned_to != 0x4355a1 { return; }
  cpu.push(0x4355a6); f_00433e0b(cpu); if cpu.returned_to != 0x4355a6 { return; }
  pc = 0x4355be; continue;
 }
 0x4355a8 => {
  cpu.push(0x4355ad); f_00433fc7(cpu); if cpu.returned_to != 0x4355ad { return; }
  cpu.push(0x4355b2); f_00433ef1(cpu); if cpu.returned_to != 0x4355b2 { return; }
  cpu.push(0x4355b7); f_00433e0b(cpu); if cpu.returned_to != 0x4355b7 { return; }
  pc = 0x4355be; continue;
 }
 0x4355b9 => {
  cpu.push(0x4355be); f_00433e0b(cpu); if cpu.returned_to != 0x4355be { return; }
  pc = 0x4355be; continue;
 }
 0x4355be => {
  pc = 0x4356bb; continue;
 }
 0x4355cd => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if cpu.zf { pc = 0x435619; continue; }
  pc = 0x4355d6; continue;
 }
 0x4355d6 => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x4355fc; continue; }
  pc = 0x4355df; continue;
 }
 0x4355df => {
  cpu.push(0x4355e4); f_0041b41d(cpu); if cpu.returned_to != 0x4355e4 { return; }
  cpu.push(0x4355e9); f_00433be5(cpu); if cpu.returned_to != 0x4355e9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x4356d1; continue;
 }
 0x4355fc => {
  cpu.push(0x435601); f_0041b41d(cpu); if cpu.returned_to != 0x435601 { return; }
  cpu.push(0x435606); f_00433bf5(cpu); if cpu.returned_to != 0x435606 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x4356d1; continue;
 }
 0x435619 => {
  { let v: u32 = cpu.r32(0x576e6eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x575f6eu32, v); }
  cpu.push(0x435628); f_0040c270(cpu); if cpu.returned_to != 0x435628 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x55b9b8u32; cpu.eax = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x43563d); f_0040c240(cpu); if cpu.returned_to != 0x43563d { return; }
  { cpu.popad(); }
  cpu.push(0x435643); f_00417e3e(cpu); if cpu.returned_to != 0x435643 { return; }
  cpu.push(0x435648); f_0041b41d(cpu); if cpu.returned_to != 0x435648 { return; }
  cpu.push(0x43564d); f_004350aa(cpu); if cpu.returned_to != 0x43564d { return; }
  pc = 0x4356ad; continue;
 }
 0x43564f => {
  cpu.push(0x435654); f_00435f61(cpu); if cpu.returned_to != 0x435654 { return; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x435674; continue; }
  pc = 0x43565d; continue;
 }
 0x43565d => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4356d1; continue; }
  pc = 0x435666; continue;
 }
 0x435666 => {
  cpu.push(0x43566b); f_0043536f(cpu); if cpu.returned_to != 0x43566b { return; }
  pc = 0x4356bb; continue;
 }
 0x435674 => {
  { let v: u32 = cpu.r32(0x58c4d1u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fdu32, v); }
  { let v: u32 = cpu.r32(0x58c4d5u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922ffu32, v); }
  { let v: u32 = cpu.r32(0x58c4d9u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922f9u32, v); }
  { let v: u32 = cpu.r32(0x58c4ddu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fbu32, v); }
  cpu.push(0x4356a1); f_00437eba(cpu); if cpu.returned_to != 0x4356a1 { return; }
  cpu.push(0x4356a6); f_0041b41d(cpu); if cpu.returned_to != 0x4356a6 { return; }
  cpu.push(0x4356ab); f_004350aa(cpu); if cpu.returned_to != 0x4356ab { return; }
  pc = 0x4356ad; continue;
 }
 0x4356ad => {
  cpu.push(0x4356b2); f_004356d9(cpu); if cpu.returned_to != 0x4356b2 { return; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4356d1; continue; }
  pc = 0x4356bb; continue;
 }
 0x4356bb => {
  cpu.push(0x4356c0); f_0041b41d(cpu); if cpu.returned_to != 0x4356c0 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x58c49du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  pc = 0x4356d1; continue;
 }
 0x4356d1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004356d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4356d9); } r_004356d9(cpu, 0x4356d9); }
pub fn f_00435a01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x435a01); } r_004356d9(cpu, 0x435a01); }
fn r_004356d9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4356d9 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1920u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23b1u32, v); }
  cpu.push(0x4356f4); f_00438fb9(cpu); if cpu.returned_to != 0x4356f4 { return; }
  { let v: u16 = cpu.r16(0x5a235du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a235fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x127bu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43571e); cpu.call(t); if cpu.returned_to != 0x43571e { return; }
  { let v: u16 = 0x2u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x435751; continue; }
  pc = 0x435730; continue;
 }
 0x435730 => {
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x435751; continue; }
  pc = 0x43573e; continue;
 }
 0x43573e => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2364u32) & 0x1u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = cpu.r8(0x5a2364u32) | (cpu.ecx as u8); cpu.w8(0x5a2364u32, v); }
  pc = 0x435751; continue;
 }
 0x435751 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add0cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x435767); f_0041ce38(cpu); if cpu.returned_to != 0x435767 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2364u32), 0xeu8); }
  if cpu.zf { pc = 0x43579a; continue; }
  pc = 0x435797; continue;
 }
 0x435797 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x43579a; continue;
 }
 0x43579a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4357b3); cpu.call(t); if cpu.returned_to != 0x4357b3 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2364u32), 0xeu8); }
  if cpu.zf { pc = 0x4357d3; continue; }
  pc = 0x4357c9; continue;
 }
 0x4357c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4357d3; continue;
 }
 0x4357d3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add0cu32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43581d); f_0041cf66(cpu); if cpu.returned_to != 0x43581d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43585e; continue; }
  pc = 0x435828; continue;
 }
 0x435828 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43583b; continue; }
  pc = 0x435832; continue;
 }
 0x435832 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43585e; continue; }
  pc = 0x43583b; continue;
 }
 0x43583b => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43585d); cpu.call(t); if cpu.returned_to != 0x43585d { return; }
  { cpu.popad(); }
  pc = 0x43585e; continue;
 }
 0x43585e => {
  { let v: u16 = 0x2u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x435891; continue; }
  pc = 0x435870; continue;
 }
 0x435870 => {
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x435891; continue; }
  pc = 0x43587e; continue;
 }
 0x43587e => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2365u32) & 0x1u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = cpu.r8(0x5a2365u32) | (cpu.ecx as u8); cpu.w8(0x5a2365u32, v); }
  pc = 0x435891; continue;
 }
 0x435891 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add10u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4358a7); f_0041ce38(cpu); if cpu.returned_to != 0x4358a7 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2365u32), 0xeu8); }
  if cpu.zf { pc = 0x4358da; continue; }
  pc = 0x4358d7; continue;
 }
 0x4358d7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4358da; continue;
 }
 0x4358da => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4358f3); cpu.call(t); if cpu.returned_to != 0x4358f3 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2365u32), 0xeu8); }
  if cpu.zf { pc = 0x435913; continue; }
  pc = 0x435909; continue;
 }
 0x435909 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x435913; continue;
 }
 0x435913 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add10u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43595d); f_0041cf66(cpu); if cpu.returned_to != 0x43595d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43599e; continue; }
  pc = 0x435968; continue;
 }
 0x435968 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x43597b; continue; }
  pc = 0x435972; continue;
 }
 0x435972 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43599e; continue; }
  pc = 0x43597b; continue;
 }
 0x43597b => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43599d); cpu.call(t); if cpu.returned_to != 0x43599d { return; }
  { cpu.popad(); }
  pc = 0x43599e; continue;
 }
 0x43599e => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4359d1; continue; }
  pc = 0x4359b0; continue;
 }
 0x4359b0 => {
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4359d1; continue; }
  pc = 0x4359be; continue;
 }
 0x4359be => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2366u32) & 0x1u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = cpu.r8(0x5a2366u32) | (cpu.ecx as u8); cpu.w8(0x5a2366u32, v); }
  pc = 0x4359d1; continue;
 }
 0x4359d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add14u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4359e7); f_0041ce38(cpu); if cpu.returned_to != 0x4359e7 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if cpu.zf { pc = 0x435a1a; continue; }
  pc = 0x435a17; continue;
 }
 0x435a01 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if cpu.zf { pc = 0x435a1a; continue; }
  pc = 0x435a17; continue;
 }
 0x435a17 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x435a1a; continue;
 }
 0x435a1a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435a33); cpu.call(t); if cpu.returned_to != 0x435a33 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if cpu.zf { pc = 0x435a53; continue; }
  pc = 0x435a49; continue;
 }
 0x435a49 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x435a53; continue;
 }
 0x435a53 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add14u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x435a9d); f_0041cf66(cpu); if cpu.returned_to != 0x435a9d { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x435ade; continue; }
  pc = 0x435aa8; continue;
 }
 0x435aa8 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x435abb; continue; }
  pc = 0x435ab2; continue;
 }
 0x435ab2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x435ade; continue; }
  pc = 0x435abb; continue;
 }
 0x435abb => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435add); cpu.call(t); if cpu.returned_to != 0x435add { return; }
  { cpu.popad(); }
  pc = 0x435ade; continue;
 }
 0x435ade => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x435b11; continue; }
  pc = 0x435af0; continue;
 }
 0x435af0 => {
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x435b11; continue; }
  pc = 0x435afe; continue;
 }
 0x435afe => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2367u32) & 0x1u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = cpu.r8(0x5a2367u32) | (cpu.ecx as u8); cpu.w8(0x5a2367u32, v); }
  pc = 0x435b11; continue;
 }
 0x435b11 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add18u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x435b27); f_0041ce38(cpu); if cpu.returned_to != 0x435b27 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x6du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2367u32), 0xeu8); }
  if cpu.zf { pc = 0x435b5a; continue; }
  pc = 0x435b57; continue;
 }
 0x435b57 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x435b5a; continue;
 }
 0x435b5a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435b73); cpu.call(t); if cpu.returned_to != 0x435b73 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2367u32), 0xeu8); }
  if cpu.zf { pc = 0x435b93; continue; }
  pc = 0x435b89; continue;
 }
 0x435b89 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x435b93; continue;
 }
 0x435b93 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add18u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x435bdd); f_0041cf66(cpu); if cpu.returned_to != 0x435bdd { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x435c1e; continue; }
  pc = 0x435be8; continue;
 }
 0x435be8 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x435bfb; continue; }
  pc = 0x435bf2; continue;
 }
 0x435bf2 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x435c1e; continue; }
  pc = 0x435bfb; continue;
 }
 0x435bfb => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x6du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435c1d); cpu.call(t); if cpu.returned_to != 0x435c1d { return; }
  { cpu.popad(); }
  pc = 0x435c1e; continue;
 }
 0x435c1e => {
  { let v: u8 = cpu.r8(0x575f1fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x575f20u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x435d76; continue; }
  pc = 0x435c31; continue;
 }
 0x435c31 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x435c64; continue; }
  pc = 0x435c43; continue;
 }
 0x435c43 => {
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x435c64; continue; }
  pc = 0x435c51; continue;
 }
 0x435c51 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2368u32) & 0x1u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = cpu.r8(0x5a2368u32) | (cpu.ecx as u8); cpu.w8(0x5a2368u32, v); }
  pc = 0x435c64; continue;
 }
 0x435c64 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x435c7a); f_0041ce38(cpu); if cpu.returned_to != 0x435c7a { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x435cad; continue; }
  pc = 0x435caa; continue;
 }
 0x435caa => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x435cad; continue;
 }
 0x435cad => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435cc6); cpu.call(t); if cpu.returned_to != 0x435cc6 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x435ce6; continue; }
  pc = 0x435cdc; continue;
 }
 0x435cdc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x435ce6; continue;
 }
 0x435ce6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x435d30); f_0041cf66(cpu); if cpu.returned_to != 0x435d30 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x435d71; continue; }
  pc = 0x435d3b; continue;
 }
 0x435d3b => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x435d4e; continue; }
  pc = 0x435d45; continue;
 }
 0x435d45 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x435d71; continue; }
  pc = 0x435d4e; continue;
 }
 0x435d4e => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435d70); cpu.call(t); if cpu.returned_to != 0x435d70 { return; }
  { cpu.popad(); }
  pc = 0x435d71; continue;
 }
 0x435d71 => {
  pc = 0x435eb6; continue;
 }
 0x435d76 => {
  { let v: u16 = 0x2u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x435da9; continue; }
  pc = 0x435d88; continue;
 }
 0x435d88 => {
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x435da9; continue; }
  pc = 0x435d96; continue;
 }
 0x435d96 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2368u32) & 0x1u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = cpu.r8(0x5a2368u32) | (cpu.ecx as u8); cpu.w8(0x5a2368u32, v); }
  pc = 0x435da9; continue;
 }
 0x435da9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x435dbf); f_0041ce38(cpu); if cpu.returned_to != 0x435dbf { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x435df2; continue; }
  pc = 0x435def; continue;
 }
 0x435def => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x435df2; continue;
 }
 0x435df2 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435e0b); cpu.call(t); if cpu.returned_to != 0x435e0b { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x435e2b; continue; }
  pc = 0x435e21; continue;
 }
 0x435e21 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x435e2b; continue;
 }
 0x435e2b => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x435e75); f_0041cf66(cpu); if cpu.returned_to != 0x435e75 { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x435eb6; continue; }
  pc = 0x435e80; continue;
 }
 0x435e80 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x435e93; continue; }
  pc = 0x435e8a; continue;
 }
 0x435e8a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x435eb6; continue; }
  pc = 0x435e93; continue;
 }
 0x435e93 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x435eb5); cpu.call(t); if cpu.returned_to != 0x435eb5 { return; }
  { cpu.popad(); }
  pc = 0x435eb6; continue;
 }
 0x435eb6 => {
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x435ec7; continue; }
  pc = 0x435ebf; continue;
 }
 0x435ebf => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  pc = 0x435ec7; continue;
 }
 0x435ec7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00435f61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x435f61); }
 let mut pc: u32 = 0x435f61;
 loop { match pc {
 0x435f61 => {
  cpu.push(0x435f66); f_00430f20(cpu); if cpu.returned_to != 0x435f66 { return; }
  cpu.push(0x435f6b); f_00437eba(cpu); if cpu.returned_to != 0x435f6b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00435f6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x435f6c); }
 let mut pc: u32 = 0x435f6c;
 loop { match pc {
 0x435f6c => {
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x0u8); }
  if cpu.zf { pc = 0x435fde; continue; }
  pc = 0x435f75; continue;
 }
 0x435f75 => {
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x2u8); }
  if !cpu.zf { pc = 0x435fde; continue; }
  pc = 0x435f7e; continue;
 }
 0x435f7e => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if !cpu.zf { pc = 0x435faf; continue; }
  pc = 0x435f87; continue;
 }
 0x435f87 => {
  cpu.push(0x435f8c); f_00433bf5(cpu); if cpu.returned_to != 0x435f8c { return; }
  cpu.push(0x435f91); f_00433c8f(cpu); if cpu.returned_to != 0x435f91 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x435fde; continue;
 }
 0x435faf => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x435fde; continue; }
  pc = 0x435fb8; continue;
 }
 0x435fb8 => {
  cpu.push(0x435fbd); f_00433c05(cpu); if cpu.returned_to != 0x435fbd { return; }
  cpu.push(0x435fc2); f_00433d92(cpu); if cpu.returned_to != 0x435fc2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e6u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x435fde; continue;
 }
 0x435fde => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x2u8); }
  if cpu.zf { pc = 0x435ffe; continue; }
  pc = 0x435fe7; continue;
 }
 0x435fe7 => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if cpu.zf { pc = 0x435ff7; continue; }
  pc = 0x435ff0; continue;
 }
 0x435ff0 => {
  cpu.push(0x435ff5); f_00431ced(cpu); if cpu.returned_to != 0x435ff5 { return; }
  pc = 0x436003; continue;
 }
 0x435ff7 => {
  cpu.push(0x435ffc); f_004321bc(cpu); if cpu.returned_to != 0x435ffc { return; }
  pc = 0x436003; continue;
 }
 0x435ffe => {
  cpu.push(0x436003); f_00433169(cpu); if cpu.returned_to != 0x436003 { return; }
  pc = 0x436003; continue;
 }
 0x436003 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00436004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x436004); }
 let mut pc: u32 = 0x436004;
 loop { match pc {
 0x436004 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43602c); cpu.call(t); if cpu.returned_to != 0x43602c { return; }
  { let v: u32 = cpu.r32(0x5add60u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x436057); f_0041cf66(cpu); if cpu.returned_to != 0x436057 { return; }
  { let v: u32 = cpu.r32(0x5add64u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x436082); f_0041cf66(cpu); if cpu.returned_to != 0x436082 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4360cf; continue; }
  pc = 0x43608b; continue;
 }
 0x43608b => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4360ab; continue; }
  pc = 0x4360a0; continue;
 }
 0x4360a0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4360ab; continue;
 }
 0x4360ab => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4360ce); cpu.call(t); if cpu.returned_to != 0x4360ce { return; }
  { cpu.popad(); }
  pc = 0x4360cf; continue;
 }
 0x4360cf => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x43611c; continue; }
  pc = 0x4360d8; continue;
 }
 0x4360d8 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4360f8; continue; }
  pc = 0x4360ed; continue;
 }
 0x4360ed => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x4360f8; continue;
 }
 0x4360f8 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43611b); cpu.call(t); if cpu.returned_to != 0x43611b { return; }
  { cpu.popad(); }
  pc = 0x43611c; continue;
 }
 0x43611c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043611d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43611d); }
 let mut pc: u32 = 0x43611d;
 loop { match pc {
 0x43611d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d39cu32, v); }
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x436151); f_00402d70(cpu); if cpu.returned_to != 0x436151 { return; }
  cpu.push(0x436156); f_00405320(cpu); if cpu.returned_to != 0x436156 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x4e1e4bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e4bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x436174; continue; }
  pc = 0x43616d; continue;
 }
 0x43616d => {
  cpu.push(0x436172); f_00417352(cpu); if cpu.returned_to != 0x436172 { return; }
  pc = 0x43618e; continue;
 }
 0x436174 => {
  { cpu.pushad(); }
  { let v: u32 = 0x54a7d0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x43618a); f_0040bca0(cpu); if cpu.returned_to != 0x43618a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x43618e; continue;
 }
 0x43618e => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x43619b); f_00402d60(cpu); if cpu.returned_to != 0x43619b { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59d9d9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59dadfu32, v); }
  { cpu.pushad(); }
  cpu.push(0x4361fc); f_004010e0(cpu); if cpu.returned_to != 0x4361fc { return; }
  { cpu.popad(); }
  cpu.push(0x436202); f_0043a626(cpu); if cpu.returned_to != 0x436202 { return; }
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x43620c); f_0041b447(cpu); if cpu.returned_to != 0x43620c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x43621a; continue;
 }
 0x43621a => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1920u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23b1u32, v); }
  cpu.push(0x436235); f_00438fb9(cpu); if cpu.returned_to != 0x436235 { return; }
  { let v: u16 = cpu.r16(0x5a235du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a235fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x127bu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43625f); cpu.call(t); if cpu.returned_to != 0x43625f { return; }
  cpu.push(0x436264); f_00417df6(cpu); if cpu.returned_to != 0x436264 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b1u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43629e; continue; }
  pc = 0x43627d; continue;
 }
 0x43627d => {
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43629e; continue; }
  pc = 0x43628b; continue;
 }
 0x43628b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2364u32) & 0x1u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = cpu.r8(0x5a2364u32) | (cpu.ecx as u8); cpu.w8(0x5a2364u32, v); }
  pc = 0x43629e; continue;
 }
 0x43629e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add0cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4362b4); f_0041ce38(cpu); if cpu.returned_to != 0x4362b4 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2364u32), 0xeu8); }
  if cpu.zf { pc = 0x4362e7; continue; }
  pc = 0x4362e4; continue;
 }
 0x4362e4 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4362e7; continue;
 }
 0x4362e7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436300); cpu.call(t); if cpu.returned_to != 0x436300 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2364u32), 0xeu8); }
  if cpu.zf { pc = 0x436320; continue; }
  pc = 0x436316; continue;
 }
 0x436316 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x436320; continue;
 }
 0x436320 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add0cu32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43636a); f_0041cf66(cpu); if cpu.returned_to != 0x43636a { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4363ab; continue; }
  pc = 0x436375; continue;
 }
 0x436375 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x436388; continue; }
  pc = 0x43637f; continue;
 }
 0x43637f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4363ab; continue; }
  pc = 0x436388; continue;
 }
 0x436388 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4363aa); cpu.call(t); if cpu.returned_to != 0x4363aa { return; }
  { cpu.popad(); }
  pc = 0x4363ab; continue;
 }
 0x4363ab => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x4363de; continue; }
  pc = 0x4363bd; continue;
 }
 0x4363bd => {
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x4363de; continue; }
  pc = 0x4363cb; continue;
 }
 0x4363cb => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2365u32) & 0x1u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = cpu.r8(0x5a2365u32) | (cpu.ecx as u8); cpu.w8(0x5a2365u32, v); }
  pc = 0x4363de; continue;
 }
 0x4363de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add10u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4363f4); f_0041ce38(cpu); if cpu.returned_to != 0x4363f4 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2365u32), 0xeu8); }
  if cpu.zf { pc = 0x436427; continue; }
  pc = 0x436424; continue;
 }
 0x436424 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x436427; continue;
 }
 0x436427 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436440); cpu.call(t); if cpu.returned_to != 0x436440 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2365u32), 0xeu8); }
  if cpu.zf { pc = 0x436460; continue; }
  pc = 0x436456; continue;
 }
 0x436456 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x436460; continue;
 }
 0x436460 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add10u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4364aa); f_0041cf66(cpu); if cpu.returned_to != 0x4364aa { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4364eb; continue; }
  pc = 0x4364b5; continue;
 }
 0x4364b5 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x4364c8; continue; }
  pc = 0x4364bf; continue;
 }
 0x4364bf => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4364eb; continue; }
  pc = 0x4364c8; continue;
 }
 0x4364c8 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4364ea); cpu.call(t); if cpu.returned_to != 0x4364ea { return; }
  { cpu.popad(); }
  pc = 0x4364eb; continue;
 }
 0x4364eb => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43651e; continue; }
  pc = 0x4364fd; continue;
 }
 0x4364fd => {
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43651e; continue; }
  pc = 0x43650b; continue;
 }
 0x43650b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2366u32) & 0x1u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = cpu.r8(0x5a2366u32) | (cpu.ecx as u8); cpu.w8(0x5a2366u32, v); }
  pc = 0x43651e; continue;
 }
 0x43651e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add14u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x436534); f_0041ce38(cpu); if cpu.returned_to != 0x436534 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if cpu.zf { pc = 0x436567; continue; }
  pc = 0x436564; continue;
 }
 0x436564 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x436567; continue;
 }
 0x436567 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436580); cpu.call(t); if cpu.returned_to != 0x436580 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if cpu.zf { pc = 0x4365a0; continue; }
  pc = 0x436596; continue;
 }
 0x436596 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4365a0; continue;
 }
 0x4365a0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add14u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4365ea); f_0041cf66(cpu); if cpu.returned_to != 0x4365ea { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43662b; continue; }
  pc = 0x4365f5; continue;
 }
 0x4365f5 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x436608; continue; }
  pc = 0x4365ff; continue;
 }
 0x4365ff => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43662b; continue; }
  pc = 0x436608; continue;
 }
 0x436608 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43662a); cpu.call(t); if cpu.returned_to != 0x43662a { return; }
  { cpu.popad(); }
  pc = 0x43662b; continue;
 }
 0x43662b => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43665e; continue; }
  pc = 0x43663d; continue;
 }
 0x43663d => {
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43665e; continue; }
  pc = 0x43664b; continue;
 }
 0x43664b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2367u32) & 0x1u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = cpu.r8(0x5a2367u32) | (cpu.ecx as u8); cpu.w8(0x5a2367u32, v); }
  pc = 0x43665e; continue;
 }
 0x43665e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add18u32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x436674); f_0041ce38(cpu); if cpu.returned_to != 0x436674 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x6du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2367u32), 0xeu8); }
  if cpu.zf { pc = 0x4366a7; continue; }
  pc = 0x4366a4; continue;
 }
 0x4366a4 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4366a7; continue;
 }
 0x4366a7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4366c0); cpu.call(t); if cpu.returned_to != 0x4366c0 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2367u32), 0xeu8); }
  if cpu.zf { pc = 0x4366e0; continue; }
  pc = 0x4366d6; continue;
 }
 0x4366d6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4366e0; continue;
 }
 0x4366e0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add18u32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43672a); f_0041cf66(cpu); if cpu.returned_to != 0x43672a { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x43676b; continue; }
  pc = 0x436735; continue;
 }
 0x436735 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x436748; continue; }
  pc = 0x43673f; continue;
 }
 0x43673f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x43676b; continue; }
  pc = 0x436748; continue;
 }
 0x436748 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x6du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43676a); cpu.call(t); if cpu.returned_to != 0x43676a { return; }
  { cpu.popad(); }
  pc = 0x43676b; continue;
 }
 0x43676b => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x43679e; continue; }
  pc = 0x43677d; continue;
 }
 0x43677d => {
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x6u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x43679e; continue; }
  pc = 0x43678b; continue;
 }
 0x43678b => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2368u32) & 0x1u8; cpu.w8(0x5a2368u32, v); }
  { let v: u8 = cpu.r8(0x5a2368u32) | (cpu.ecx as u8); cpu.w8(0x5a2368u32, v); }
  pc = 0x43679e; continue;
 }
 0x43679e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x4367b4); f_0041ce38(cpu); if cpu.returned_to != 0x4367b4 { return; }
  { let v: u32 = 0x2du32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x4367e7; continue; }
  pc = 0x4367e4; continue;
 }
 0x4367e4 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  pc = 0x4367e7; continue;
 }
 0x4367e7 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1914u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436800); cpu.call(t); if cpu.returned_to != 0x436800 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5a2368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if cpu.zf { pc = 0x436820; continue; }
  pc = 0x436816; continue;
 }
 0x436816 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x436820; continue;
 }
 0x436820 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5a2383u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5a2387u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5add1cu32); cpu.esi = v; }
  { let v: u32 = 0x56u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x43686a); f_0041cf66(cpu); if cpu.returned_to != 0x43686a { return; }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if cpu.zf { pc = 0x4368ab; continue; }
  pc = 0x436875; continue;
 }
 0x436875 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2u16); }
  if cpu.zf { pc = 0x436888; continue; }
  pc = 0x43687f; continue;
 }
 0x43687f => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4368ab; continue; }
  pc = 0x436888; continue;
 }
 0x436888 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = 0x1926u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4368aa); cpu.call(t); if cpu.returned_to != 0x4368aa { return; }
  { cpu.popad(); }
  pc = 0x4368ab; continue;
 }
 0x4368ab => {
  pc = 0x4369f0; continue;
 }
 0x4369f0 => {
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x436a01; continue; }
  pc = 0x4369f9; continue;
 }
 0x4369f9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  pc = 0x436a7d; continue;
 }
 0x436a01 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x436a0d); f_0041c122(cpu); if cpu.returned_to != 0x436a0d { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x436a21; continue; }
  pc = 0x436a11; continue;
 }
 0x436a11 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x436a21; continue; }
  pc = 0x436a1a; continue;
 }
 0x436a1a => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x436a21; continue;
 }
 0x436a21 => {
  { let v: u8 = cpu.r8(0x5a2364u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x5a2365u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x5a2366u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x5a2367u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x5a2368u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x0u8); }
  if !cpu.zf { pc = 0x436a6c; continue; }
  pc = 0x436a44; continue;
 }
 0x436a44 => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if !cpu.zf { pc = 0x436a6c; continue; }
  pc = 0x436a48; continue;
 }
 0x436a48 => {
  { cpu.pushad(); }
  cpu.push(0x436a4e); f_00403050(cpu); if cpu.returned_to != 0x436a4e { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x436a6c; continue; }
  pc = 0x436a53; continue;
 }
 0x436a53 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39bu32, v); }
  cpu.push(0x436a5f); f_00436e6c(cpu); if cpu.returned_to != 0x436a5f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d39bu32, v); }
  if cpu.cf { pc = 0x436c00; continue; }
  pc = 0x436a6c; continue;
 }
 0x436a6c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x436a77); f_0041b447(cpu); if cpu.returned_to != 0x436a77 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x436a7d); f_0041b488(cpu); if cpu.returned_to != 0x436a7d { return; }
  pc = 0x436a7d; continue;
 }
 0x436a7d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x436a85); cpu.call(t); if cpu.returned_to != 0x436a85 { return; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x436bc4; continue; }
  pc = 0x436a92; continue;
 }
 0x436a92 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x436aa9; continue; }
  pc = 0x436a9b; continue;
 }
 0x436a9b => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { pc = 0x436aa9; continue; }
  pc = 0x436aa4; continue;
 }
 0x436aa4 => {
  pc = 0x436bc4; continue;
 }
 0x436aa9 => {
  { cpu.sub8(cpu.r8(0x5a2367u32), 0x0u8); }
  if cpu.zf { pc = 0x436ad8; continue; }
  pc = 0x436ab2; continue;
 }
 0x436ab2 => {
  { cpu.and8(cpu.r8(0x5a2367u32), 0xeu8); }
  if !cpu.zf { pc = 0x436ad8; continue; }
  pc = 0x436abb; continue;
 }
 0x436abb => {
  cpu.push(0x436ac0); f_00437867(cpu); if cpu.returned_to != 0x436ac0 { return; }
  if cpu.cf { pc = 0x436b9c; continue; }
  pc = 0x436ac6; continue;
 }
 0x436ac6 => {
  { cpu.sub8(cpu.r8(0x56d39au32), 0x0u8); }
  if !cpu.zf { pc = 0x436c00; continue; }
  pc = 0x436ad3; continue;
 }
 0x436ad3 => {
  pc = 0x436bc4; continue;
 }
 0x436ad8 => {
  { cpu.sub8(cpu.r8(0x5a2368u32), 0x0u8); }
  if cpu.zf { pc = 0x436b07; continue; }
  pc = 0x436ae1; continue;
 }
 0x436ae1 => {
  { cpu.and8(cpu.r8(0x5a2368u32), 0xeu8); }
  if !cpu.zf { pc = 0x436b07; continue; }
  pc = 0x436aea; continue;
 }
 0x436aea => {
  cpu.push(0x436aef); f_00437bbe(cpu); if cpu.returned_to != 0x436aef { return; }
  if cpu.cf { pc = 0x436b9c; continue; }
  pc = 0x436af5; continue;
 }
 0x436af5 => {
  { cpu.sub8(cpu.r8(0x56d39au32), 0x0u8); }
  if !cpu.zf { pc = 0x436c00; continue; }
  pc = 0x436b02; continue;
 }
 0x436b02 => {
  pc = 0x436bc4; continue;
 }
 0x436b07 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x436b63; continue; }
  pc = 0x436b10; continue;
 }
 0x436b10 => {
  { cpu.sub8(cpu.r8(0x5a2364u32), 0x0u8); }
  if cpu.zf { pc = 0x436b3b; continue; }
  pc = 0x436b19; continue;
 }
 0x436b19 => {
  { cpu.and8(cpu.r8(0x5a2364u32), 0xeu8); }
  if !cpu.zf { pc = 0x436b3b; continue; }
  pc = 0x436b22; continue;
 }
 0x436b22 => {
  cpu.push(0x436b27); f_00437f04(cpu); if cpu.returned_to != 0x436b27 { return; }
  if cpu.cf { pc = 0x436b9c; continue; }
  pc = 0x436b29; continue;
 }
 0x436b29 => {
  { cpu.sub8(cpu.r8(0x56d39au32), 0x0u8); }
  if !cpu.zf { pc = 0x436c00; continue; }
  pc = 0x436b36; continue;
 }
 0x436b36 => {
  pc = 0x436bc4; continue;
 }
 0x436b3b => {
  { cpu.sub8(cpu.r8(0x5a2365u32), 0x0u8); }
  if cpu.zf { pc = 0x436b63; continue; }
  pc = 0x436b44; continue;
 }
 0x436b44 => {
  { cpu.and8(cpu.r8(0x5a2365u32), 0xeu8); }
  if !cpu.zf { pc = 0x436b63; continue; }
  pc = 0x436b4d; continue;
 }
 0x436b4d => {
  cpu.push(0x436b52); f_00438539(cpu); if cpu.returned_to != 0x436b52 { return; }
  if cpu.cf { pc = 0x436b9c; continue; }
  pc = 0x436b54; continue;
 }
 0x436b54 => {
  { cpu.sub8(cpu.r8(0x56d39au32), 0x0u8); }
  if !cpu.zf { pc = 0x436c00; continue; }
  pc = 0x436b61; continue;
 }
 0x436b61 => {
  pc = 0x436bc4; continue;
 }
 0x436b63 => {
  { cpu.sub8(cpu.r8(0x5a2366u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x43621a; continue; }
  pc = 0x436b70; continue;
 }
 0x436b70 => {
  { cpu.and8(cpu.r8(0x5a2366u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x43621a; continue; }
  pc = 0x436b7d; continue;
 }
 0x436b7d => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39bu32, v); }
  cpu.push(0x436b89); f_00436e6c(cpu); if cpu.returned_to != 0x436b89 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d39bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddaa0u32, v); }
  if cpu.cf { pc = 0x436c00; continue; }
  pc = 0x436b9c; continue;
 }
 0x436b9c => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  cpu.poll(); pc = 0x43621a; continue;
 }
 0x436bc4 => {
  cpu.push(0x436bc9); f_0041b2d5(cpu); if cpu.returned_to != 0x436bc9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x436bdd); f_00402d70(cpu); if cpu.returned_to != 0x436bdd { return; }
  cpu.push(0x436be2); f_00405320(cpu); if cpu.returned_to != 0x436be2 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e4bu32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de71cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x436c00 => {
  cpu.push(0x436c05); f_0041b2d5(cpu); if cpu.returned_to != 0x436c05 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x436c19); f_00402d70(cpu); if cpu.returned_to != 0x436c19 { return; }
  cpu.push(0x436c1e); f_00405320(cpu); if cpu.returned_to != 0x436c1e { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e4bu32, v); }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5de71cu32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00436c32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x436c32); }
 let mut pc: u32 = 0x436c32;
 loop { match pc {
 0x436c32 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  cpu.push(0x436c40); f_0041b454(cpu); if cpu.returned_to != 0x436c40 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x56d39cu32, v); }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x436c63; continue; }
  pc = 0x436c5e; continue;
 }
 0x436c5e => {
  cpu.push(0x436c63); f_004384e2(cpu); if cpu.returned_to != 0x436c63 { return; }
  pc = 0x436c63; continue;
 }
 0x436c63 => {
  cpu.push(0x436c68); f_00436de5(cpu); if cpu.returned_to != 0x436c68 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x436c70); cpu.call(t); if cpu.returned_to != 0x436c70 { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x436c7e; continue; }
  pc = 0x436c79; continue;
 }
 0x436c79 => {
  cpu.push(0x436c7e); f_004384e2(cpu); if cpu.returned_to != 0x436c7e { return; }
  pc = 0x436c7e; continue;
 }
 0x436c7e => {
  cpu.push(0x436c83); f_00436de5(cpu); if cpu.returned_to != 0x436c83 { return; }
  pc = 0x436c83; continue;
 }
 0x436c83 => {
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x436cd0; continue; }
  pc = 0x436c8c; continue;
 }
 0x436c8c => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x436cac; continue; }
  pc = 0x436ca1; continue;
 }
 0x436ca1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x436cac; continue;
 }
 0x436cac => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436ccf); cpu.call(t); if cpu.returned_to != 0x436ccf { return; }
  { cpu.popad(); }
  pc = 0x436cd0; continue;
 }
 0x436cd0 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x436d1d; continue; }
  pc = 0x436cd9; continue;
 }
 0x436cd9 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x436cf9; continue; }
  pc = 0x436cee; continue;
 }
 0x436cee => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x436cf9; continue;
 }
 0x436cf9 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436d1c); cpu.call(t); if cpu.returned_to != 0x436d1c { return; }
  { cpu.popad(); }
  pc = 0x436d1d; continue;
 }
 0x436d1d => {
  cpu.push(0x436d22); f_00417df6(cpu); if cpu.returned_to != 0x436d22 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x436d2e); f_0041c122(cpu); if cpu.returned_to != 0x436d2e { return; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x436d42; continue; }
  pc = 0x436d37; continue;
 }
 0x436d37 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x436c83; continue;
 }
 0x436d42 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x436d56; continue; }
  pc = 0x436d46; continue;
 }
 0x436d46 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x436d56; continue; }
  pc = 0x436d4f; continue;
 }
 0x436d4f => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x436d56; continue;
 }
 0x436d56 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x436d6a; continue; }
  pc = 0x436d5a; continue;
 }
 0x436d5a => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x436d6a; continue; }
  pc = 0x436d63; continue;
 }
 0x436d63 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x436d6a; continue;
 }
 0x436d6a => {
  cpu.push(0x436d6f); f_0041b488(cpu); if cpu.returned_to != 0x436d6f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x436d77); cpu.call(t); if cpu.returned_to != 0x436d77 { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x436d8b; continue; }
  pc = 0x436d80; continue;
 }
 0x436d80 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x436d8b; continue; }
  pc = 0x436d89; continue;
 }
 0x436d89 => {
  pc = 0x436da7; continue;
 }
 0x436d8b => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x436c83; continue; }
  pc = 0x436d98; continue;
 }
 0x436d98 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x436c83; continue; }
  pc = 0x436da5; continue;
 }
 0x436da5 => {
  pc = 0x436dbf; continue;
 }
 0x436da7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  cpu.push(0x436dbd); f_0041b46d(cpu); if cpu.returned_to != 0x436dbd { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x436dbf => {
  cpu.push(0x436dc4); f_0041b46d(cpu); if cpu.returned_to != 0x436dc4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d2fau32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00436de5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x436de5); }
 let mut pc: u32 = 0x436de5;
 loop { match pc {
 0x436de5 => {
  cpu.push(0x436dea); f_0043a626(cpu); if cpu.returned_to != 0x436dea { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436e0b); cpu.call(t); if cpu.returned_to != 0x436e0b { return; }
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x436e15); f_0041b447(cpu); if cpu.returned_to != 0x436e15 { return; }
  { let v: u32 = cpu.r32(0x5ae120u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x436e40); f_0041cf66(cpu); if cpu.returned_to != 0x436e40 { return; }
  { let v: u32 = cpu.r32(0x5ae124u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x436e6b); f_0041cf66(cpu); if cpu.returned_to != 0x436e6b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00436e6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x436e6c); }
 let mut pc: u32 = 0x436e6c;
 loop { match pc {
 0x436e6c => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x56d39cu32, v); }
  cpu.push(0x436e86); f_0041b454(cpu); if cpu.returned_to != 0x436e86 { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x436e94; continue; }
  pc = 0x436e8f; continue;
 }
 0x436e8f => {
  cpu.push(0x436e94); f_004384e2(cpu); if cpu.returned_to != 0x436e94 { return; }
  pc = 0x436e94; continue;
 }
 0x436e94 => {
  cpu.push(0x436e99); f_00436ff9(cpu); if cpu.returned_to != 0x436e99 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x436ea1); cpu.call(t); if cpu.returned_to != 0x436ea1 { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x436eaf; continue; }
  pc = 0x436eaa; continue;
 }
 0x436eaa => {
  cpu.push(0x436eaf); f_004384e2(cpu); if cpu.returned_to != 0x436eaf { return; }
  pc = 0x436eaf; continue;
 }
 0x436eaf => {
  cpu.push(0x436eb4); f_00436ff9(cpu); if cpu.returned_to != 0x436eb4 { return; }
  pc = 0x436eb4; continue;
 }
 0x436eb4 => {
  cpu.push(0x436eb9); f_00436ff9(cpu); if cpu.returned_to != 0x436eb9 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x436f06; continue; }
  pc = 0x436ec2; continue;
 }
 0x436ec2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x436ee2; continue; }
  pc = 0x436ed7; continue;
 }
 0x436ed7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x436ee2; continue;
 }
 0x436ee2 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436f05); cpu.call(t); if cpu.returned_to != 0x436f05 { return; }
  { cpu.popad(); }
  pc = 0x436f06; continue;
 }
 0x436f06 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x436f53; continue; }
  pc = 0x436f0f; continue;
 }
 0x436f0f => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x436f2f; continue; }
  pc = 0x436f24; continue;
 }
 0x436f24 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x436f2f; continue;
 }
 0x436f2f => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x436f52); cpu.call(t); if cpu.returned_to != 0x436f52 { return; }
  { cpu.popad(); }
  pc = 0x436f53; continue;
 }
 0x436f53 => {
  cpu.push(0x436f58); f_00417df6(cpu); if cpu.returned_to != 0x436f58 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x436f64); f_0041c122(cpu); if cpu.returned_to != 0x436f64 { return; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x436f78; continue; }
  pc = 0x436f6d; continue;
 }
 0x436f6d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x436eb4; continue;
 }
 0x436f78 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x436f8c; continue; }
  pc = 0x436f7c; continue;
 }
 0x436f7c => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x436f8c; continue; }
  pc = 0x436f85; continue;
 }
 0x436f85 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x436f8c; continue;
 }
 0x436f8c => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x436fa0; continue; }
  pc = 0x436f90; continue;
 }
 0x436f90 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x436fa0; continue; }
  pc = 0x436f99; continue;
 }
 0x436f99 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x436fa0; continue;
 }
 0x436fa0 => {
  cpu.push(0x436fa5); f_0041b488(cpu); if cpu.returned_to != 0x436fa5 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x436fad); cpu.call(t); if cpu.returned_to != 0x436fad { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x436fc1; continue; }
  pc = 0x436fb6; continue;
 }
 0x436fb6 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x436fc1; continue; }
  pc = 0x436fbf; continue;
 }
 0x436fbf => {
  pc = 0x436fdd; continue;
 }
 0x436fc1 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x436eb4; continue; }
  pc = 0x436fce; continue;
 }
 0x436fce => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x436eb4; continue; }
  pc = 0x436fdb; continue;
 }
 0x436fdb => {
  pc = 0x436fe4; continue;
 }
 0x436fdd => {
  cpu.push(0x436fe2); f_0041b46d(cpu); if cpu.returned_to != 0x436fe2 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x436fe4 => {
  cpu.push(0x436fe9); f_0041b46d(cpu); if cpu.returned_to != 0x436fe9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00436ff9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x436ff9); }
 let mut pc: u32 = 0x436ff9;
 loop { match pc {
 0x436ff9 => {
  cpu.push(0x436ffe); f_0043a626(cpu); if cpu.returned_to != 0x436ffe { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43701f); cpu.call(t); if cpu.returned_to != 0x43701f { return; }
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x437029); f_0041b447(cpu); if cpu.returned_to != 0x437029 { return; }
  { let v: u32 = cpu.r32(0x5add60u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437054); f_0041cf66(cpu); if cpu.returned_to != 0x437054 { return; }
  { let v: u32 = cpu.r32(0x5add64u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43707f); f_0041cf66(cpu); if cpu.returned_to != 0x43707f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437080); }
 let mut pc: u32 = 0x437080;
 loop { match pc {
 0x437080 => {
  { cpu.pushad(); }
  cpu.push(0x437086); f_0041b454(cpu); if cpu.returned_to != 0x437086 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x56d39cu32, v); }
  cpu.push(0x437099); f_00437237(cpu); if cpu.returned_to != 0x437099 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4370e6; continue; }
  pc = 0x4370a2; continue;
 }
 0x4370a2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4370c2; continue; }
  pc = 0x4370b7; continue;
 }
 0x4370b7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4370c2; continue;
 }
 0x4370c2 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4370e5); cpu.call(t); if cpu.returned_to != 0x4370e5 { return; }
  { cpu.popad(); }
  pc = 0x4370e6; continue;
 }
 0x4370e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4370ee); cpu.call(t); if cpu.returned_to != 0x4370ee { return; }
  cpu.push(0x4370f3); f_00437237(cpu); if cpu.returned_to != 0x4370f3 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x437140; continue; }
  pc = 0x4370fc; continue;
 }
 0x4370fc => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x43711c; continue; }
  pc = 0x437111; continue;
 }
 0x437111 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x43711c; continue;
 }
 0x43711c => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43713f); cpu.call(t); if cpu.returned_to != 0x43713f { return; }
  { cpu.popad(); }
  pc = 0x437140; continue;
 }
 0x437140 => {
  cpu.push(0x437145); f_00417df6(cpu); if cpu.returned_to != 0x437145 { return; }
  cpu.push(0x43714a); f_0041c122(cpu); if cpu.returned_to != 0x43714a { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x43715e; continue; }
  pc = 0x43714e; continue;
 }
 0x43714e => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x43715e; continue; }
  pc = 0x437157; continue;
 }
 0x437157 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x43715e; continue;
 }
 0x43715e => {
  cpu.push(0x437163); f_0041b488(cpu); if cpu.returned_to != 0x437163 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x43716b); cpu.call(t); if cpu.returned_to != 0x43716b { return; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x43722f; continue; }
  pc = 0x437178; continue;
 }
 0x437178 => {
  cpu.push(0x43717d); f_00437237(cpu); if cpu.returned_to != 0x43717d { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4371ca; continue; }
  pc = 0x437186; continue;
 }
 0x437186 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4371a6; continue; }
  pc = 0x43719b; continue;
 }
 0x43719b => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4371a6; continue;
 }
 0x4371a6 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4371c9); cpu.call(t); if cpu.returned_to != 0x4371c9 { return; }
  { cpu.popad(); }
  pc = 0x4371ca; continue;
 }
 0x4371ca => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4371d2); cpu.call(t); if cpu.returned_to != 0x4371d2 { return; }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x43721f; continue; }
  pc = 0x4371db; continue;
 }
 0x4371db => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4371fb; continue; }
  pc = 0x4371f0; continue;
 }
 0x4371f0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4371fb; continue;
 }
 0x4371fb => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43721e); cpu.call(t); if cpu.returned_to != 0x43721e { return; }
  { cpu.popad(); }
  pc = 0x43721f; continue;
 }
 0x43721f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x437227); cpu.call(t); if cpu.returned_to != 0x437227 { return; }
  cpu.push(0x43722c); f_0041b46d(cpu); if cpu.returned_to != 0x43722c { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43722f => {
  cpu.push(0x437234); f_0041b46d(cpu); if cpu.returned_to != 0x437234 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437237); }
 let mut pc: u32 = 0x437237;
 loop { match pc {
 0x437237 => {
  cpu.push(0x43723c); f_0043a626(cpu); if cpu.returned_to != 0x43723c { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43725d); cpu.call(t); if cpu.returned_to != 0x43725d { return; }
  { let v: u32 = 0x5a0c74u32; cpu.esi = v; }
  cpu.push(0x437267); f_0041b447(cpu); if cpu.returned_to != 0x437267 { return; }
  { let v: u32 = cpu.r32(0x5ae094u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437292); f_0041cf66(cpu); if cpu.returned_to != 0x437292 { return; }
  { let v: u32 = cpu.r32(0x5ae098u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x4372bd); f_0041cf66(cpu); if cpu.returned_to != 0x4372bd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004372be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4372be); }
 let mut pc: u32 = 0x4372be;
 loop { match pc {
 0x4372be => {
  cpu.push(0x4372c3); f_0043a626(cpu); if cpu.returned_to != 0x4372c3 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4372e4); cpu.call(t); if cpu.returned_to != 0x4372e4 { return; }
  { let v: u32 = cpu.r32(0x5ae0a0u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43730f); f_0041cf66(cpu); if cpu.returned_to != 0x43730f { return; }
  { let v: u32 = cpu.r32(0x5ae0a4u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43733a); f_0041cf66(cpu); if cpu.returned_to != 0x43733a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043733b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43733b); }
 let mut pc: u32 = 0x43733b;
 loop { match pc {
 0x43733b => {
  cpu.push(0x437340); f_0041b454(cpu); if cpu.returned_to != 0x437340 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x56d39cu32, v); }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x437363; continue; }
  pc = 0x43735e; continue;
 }
 0x43735e => {
  cpu.push(0x437363); f_004384e2(cpu); if cpu.returned_to != 0x437363 { return; }
  pc = 0x437363; continue;
 }
 0x437363 => {
  cpu.push(0x437368); f_004374c3(cpu); if cpu.returned_to != 0x437368 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x437370); cpu.call(t); if cpu.returned_to != 0x437370 { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x43737e; continue; }
  pc = 0x437379; continue;
 }
 0x437379 => {
  cpu.push(0x43737e); f_004384e2(cpu); if cpu.returned_to != 0x43737e { return; }
  pc = 0x43737e; continue;
 }
 0x43737e => {
  cpu.push(0x437383); f_004374c3(cpu); if cpu.returned_to != 0x437383 { return; }
  pc = 0x437383; continue;
 }
 0x437383 => {
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4373d0; continue; }
  pc = 0x43738c; continue;
 }
 0x43738c => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4373ac; continue; }
  pc = 0x4373a1; continue;
 }
 0x4373a1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4373ac; continue;
 }
 0x4373ac => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4373cf); cpu.call(t); if cpu.returned_to != 0x4373cf { return; }
  { cpu.popad(); }
  pc = 0x4373d0; continue;
 }
 0x4373d0 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x43741d; continue; }
  pc = 0x4373d9; continue;
 }
 0x4373d9 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4373f9; continue; }
  pc = 0x4373ee; continue;
 }
 0x4373ee => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x4373f9; continue;
 }
 0x4373f9 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x43741c); cpu.call(t); if cpu.returned_to != 0x43741c { return; }
  { cpu.popad(); }
  pc = 0x43741d; continue;
 }
 0x43741d => {
  cpu.push(0x437422); f_00417df6(cpu); if cpu.returned_to != 0x437422 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x43742e); f_0041c122(cpu); if cpu.returned_to != 0x43742e { return; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x437442; continue; }
  pc = 0x437437; continue;
 }
 0x437437 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x437383; continue;
 }
 0x437442 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x437456; continue; }
  pc = 0x437446; continue;
 }
 0x437446 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x437456; continue; }
  pc = 0x43744f; continue;
 }
 0x43744f => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x437456; continue;
 }
 0x437456 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x43746a; continue; }
  pc = 0x43745a; continue;
 }
 0x43745a => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x43746a; continue; }
  pc = 0x437463; continue;
 }
 0x437463 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x43746a; continue;
 }
 0x43746a => {
  cpu.push(0x43746f); f_0041b488(cpu); if cpu.returned_to != 0x43746f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x437477); cpu.call(t); if cpu.returned_to != 0x437477 { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x43748b; continue; }
  pc = 0x437480; continue;
 }
 0x437480 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x43748b; continue; }
  pc = 0x437489; continue;
 }
 0x437489 => {
  pc = 0x4374a7; continue;
 }
 0x43748b => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x437383; continue; }
  pc = 0x437498; continue;
 }
 0x437498 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x437383; continue; }
  pc = 0x4374a5; continue;
 }
 0x4374a5 => {
  pc = 0x4374ae; continue;
 }
 0x4374a7 => {
  cpu.push(0x4374ac); f_0041b46d(cpu); if cpu.returned_to != 0x4374ac { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4374ae => {
  cpu.push(0x4374b3); f_0041b46d(cpu); if cpu.returned_to != 0x4374b3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004374c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4374c3); }
 let mut pc: u32 = 0x4374c3;
 loop { match pc {
 0x4374c3 => {
  cpu.push(0x4374c8); f_0043a626(cpu); if cpu.returned_to != 0x4374c8 { return; }
  { cpu.sub8(cpu.r8(0x58882du32), 0x0u8); }
  if cpu.zf { pc = 0x4374d6; continue; }
  pc = 0x4374d1; continue;
 }
 0x4374d1 => {
  pc = 0x4375b9; continue;
 }
 0x4374d6 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4374f7); cpu.call(t); if cpu.returned_to != 0x4374f7 { return; }
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x437501); f_0041b447(cpu); if cpu.returned_to != 0x437501 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x437562; continue; }
  pc = 0x43750a; continue;
 }
 0x43750a => {
  { let v: u32 = cpu.r32(0x5ae068u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437535); f_0041cf66(cpu); if cpu.returned_to != 0x437535 { return; }
  { let v: u32 = cpu.r32(0x5ae06cu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437560); f_0041cf66(cpu); if cpu.returned_to != 0x437560 { return; }
  pc = 0x4375b8; continue;
 }
 0x437562 => {
  { let v: u32 = cpu.r32(0x5ae078u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43758d); f_0041cf66(cpu); if cpu.returned_to != 0x43758d { return; }
  { let v: u32 = cpu.r32(0x5ae07cu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x4375b8); f_0041cf66(cpu); if cpu.returned_to != 0x4375b8 { return; }
  pc = 0x4375b8; continue;
 }
 0x4375b8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4375b9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4375c3; continue; }
  pc = 0x4375c2; continue;
 }
 0x4375c2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4375c3 => {
  cpu.push(0x4375c8); f_0043a626(cpu); if cpu.returned_to != 0x4375c8 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4375e9); cpu.call(t); if cpu.returned_to != 0x4375e9 { return; }
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x4375f3); f_0041b447(cpu); if cpu.returned_to != 0x4375f3 { return; }
  { let v: u32 = cpu.r32(0x5ae0d8u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43761e); f_0041cf66(cpu); if cpu.returned_to != 0x43761e { return; }
  { let v: u32 = cpu.r32(0x5ae0dcu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437649); f_0041cf66(cpu); if cpu.returned_to != 0x437649 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043764a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43764a); }
 let mut pc: u32 = 0x43764a;
 loop { match pc {
 0x43764a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x56d39cu32, v); }
  cpu.push(0x437664); f_0041b454(cpu); if cpu.returned_to != 0x437664 { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x437672; continue; }
  pc = 0x43766d; continue;
 }
 0x43766d => {
  cpu.push(0x437672); f_004384e2(cpu); if cpu.returned_to != 0x437672 { return; }
  pc = 0x437672; continue;
 }
 0x437672 => {
  cpu.push(0x437677); f_004377e0(cpu); if cpu.returned_to != 0x437677 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x43767f); cpu.call(t); if cpu.returned_to != 0x43767f { return; }
  { cpu.sub8(cpu.r8(0x56d39bu32), 0x0u8); }
  if !cpu.zf { pc = 0x43768d; continue; }
  pc = 0x437688; continue;
 }
 0x437688 => {
  cpu.push(0x43768d); f_004384e2(cpu); if cpu.returned_to != 0x43768d { return; }
  pc = 0x43768d; continue;
 }
 0x43768d => {
  cpu.push(0x437692); f_004377e0(cpu); if cpu.returned_to != 0x437692 { return; }
  pc = 0x437692; continue;
 }
 0x437692 => {
  { let v: u8 = 0x1u8; cpu.w8(0x576bd6u32, v); }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4376e6; continue; }
  pc = 0x4376a2; continue;
 }
 0x4376a2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4376c2; continue; }
  pc = 0x4376b7; continue;
 }
 0x4376b7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4376c2; continue;
 }
 0x4376c2 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4376e5); cpu.call(t); if cpu.returned_to != 0x4376e5 { return; }
  { cpu.popad(); }
  pc = 0x4376e6; continue;
 }
 0x4376e6 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x437733; continue; }
  pc = 0x4376ef; continue;
 }
 0x4376ef => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x43770f; continue; }
  pc = 0x437704; continue;
 }
 0x437704 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x43770f; continue;
 }
 0x43770f => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x437732); cpu.call(t); if cpu.returned_to != 0x437732 { return; }
  { cpu.popad(); }
  pc = 0x437733; continue;
 }
 0x437733 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576bd6u32, v); }
  cpu.push(0x43773f); f_00417df6(cpu); if cpu.returned_to != 0x43773f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x43774b); f_0041c122(cpu); if cpu.returned_to != 0x43774b { return; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x43775f; continue; }
  pc = 0x437754; continue;
 }
 0x437754 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x437692; continue;
 }
 0x43775f => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x437773; continue; }
  pc = 0x437763; continue;
 }
 0x437763 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x437773; continue; }
  pc = 0x43776c; continue;
 }
 0x43776c => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x437773; continue;
 }
 0x437773 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x437787; continue; }
  pc = 0x437777; continue;
 }
 0x437777 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x437787; continue; }
  pc = 0x437780; continue;
 }
 0x437780 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x437787; continue;
 }
 0x437787 => {
  cpu.push(0x43778c); f_0041b488(cpu); if cpu.returned_to != 0x43778c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x437794); cpu.call(t); if cpu.returned_to != 0x437794 { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x4377a8; continue; }
  pc = 0x43779d; continue;
 }
 0x43779d => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x4377a8; continue; }
  pc = 0x4377a6; continue;
 }
 0x4377a6 => {
  pc = 0x4377c4; continue;
 }
 0x4377a8 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x437692; continue; }
  pc = 0x4377b5; continue;
 }
 0x4377b5 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x437692; continue; }
  pc = 0x4377c2; continue;
 }
 0x4377c2 => {
  pc = 0x4377cb; continue;
 }
 0x4377c4 => {
  cpu.push(0x4377c9); f_0041b46d(cpu); if cpu.returned_to != 0x4377c9 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4377cb => {
  cpu.push(0x4377d0); f_0041b46d(cpu); if cpu.returned_to != 0x4377d0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004377e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4377e0); }
 let mut pc: u32 = 0x4377e0;
 loop { match pc {
 0x4377e0 => {
  cpu.push(0x4377e5); f_0043a626(cpu); if cpu.returned_to != 0x4377e5 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x437806); cpu.call(t); if cpu.returned_to != 0x437806 { return; }
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x437810); f_0041b447(cpu); if cpu.returned_to != 0x437810 { return; }
  { let v: u32 = cpu.r32(0x5ae044u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43783b); f_0041cf66(cpu); if cpu.returned_to != 0x43783b { return; }
  { let v: u32 = cpu.r32(0x5ae048u32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x437866); f_0041cf66(cpu); if cpu.returned_to != 0x437866 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437867); }
 let mut pc: u32 = 0x437867;
 loop { match pc {
 0x437867 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x5a238eu32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x58c86eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5a2361u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5a23c8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d39cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  cpu.push(0x4378a5); f_00433bc4(cpu); if cpu.returned_to != 0x4378a5 { return; }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e64u32, v); }
  cpu.push(0x4378b5); f_00433c15(cpu); if cpu.returned_to != 0x4378b5 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1fbcu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e62u32, v); }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x576e6eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f30u32, v); }
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x4378f0; continue;
 }
 0x4378f0 => {
  cpu.push(0x4378f5); f_00417df6(cpu); if cpu.returned_to != 0x4378f5 { return; }
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if cpu.zf { pc = 0x43790e; continue; }
  pc = 0x4378fe; continue;
 }
 0x4378fe => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x2u8); }
  if cpu.zf { pc = 0x437915; continue; }
  pc = 0x437907; continue;
 }
 0x437907 => {
  cpu.push(0x43790c); f_00431ced(cpu); if cpu.returned_to != 0x43790c { return; }
  pc = 0x43791a; continue;
 }
 0x43790e => {
  cpu.push(0x437913); f_004321bc(cpu); if cpu.returned_to != 0x437913 { return; }
  pc = 0x43791a; continue;
 }
 0x437915 => {
  cpu.push(0x43791a); f_00433169(cpu); if cpu.returned_to != 0x43791a { return; }
  pc = 0x43791a; continue;
 }
 0x43791a => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x437926); f_0041c122(cpu); if cpu.returned_to != 0x437926 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x437aab; continue; }
  pc = 0x43792e; continue;
 }
 0x43792e => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x437aa3; continue; }
  pc = 0x437936; continue;
 }
 0x437936 => {
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x437947; continue; }
  pc = 0x43793f; continue;
 }
 0x43793f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  pc = 0x43797e; continue;
 }
 0x437947 => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if !cpu.zf { pc = 0x43797e; continue; }
  pc = 0x43794b; continue;
 }
 0x43794b => {
  { cpu.pushad(); }
  cpu.push(0x437951); f_00403050(cpu); if cpu.returned_to != 0x437951 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x43797e; continue; }
  pc = 0x437956; continue;
 }
 0x437956 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d39au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d39bu32, v); }
  cpu.push(0x437969); f_00436e6c(cpu); if cpu.returned_to != 0x437969 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d39bu32, v); }
  if !cpu.cf { pc = 0x43797e; continue; }
  pc = 0x437972; continue;
 }
 0x437972 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39au32, v); }
  pc = 0x437b24; continue;
 }
 0x43797e => {
  cpu.push(0x437983); f_0041b488(cpu); if cpu.returned_to != 0x437983 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x43798b); cpu.call(t); if cpu.returned_to != 0x43798b { return; }
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x0u8); }
  if cpu.zf { pc = 0x4379fd; continue; }
  pc = 0x437994; continue;
 }
 0x437994 => {
  { cpu.sub8(cpu.r8(0x58c6e6u32), 0x2u8); }
  if !cpu.zf { pc = 0x4379fd; continue; }
  pc = 0x43799d; continue;
 }
 0x43799d => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if !cpu.zf { pc = 0x4379ce; continue; }
  pc = 0x4379a6; continue;
 }
 0x4379a6 => {
  cpu.push(0x4379ab); f_00433bcf(cpu); if cpu.returned_to != 0x4379ab { return; }
  cpu.push(0x4379b0); f_00433c8f(cpu); if cpu.returned_to != 0x4379b0 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x4379fd; continue;
 }
 0x4379ce => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x4379fd; continue; }
  pc = 0x4379d7; continue;
 }
 0x4379d7 => {
  cpu.push(0x4379dc); f_00433bda(cpu); if cpu.returned_to != 0x4379dc { return; }
  cpu.push(0x4379e1); f_00433d92(cpu); if cpu.returned_to != 0x4379e1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e6u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x4379fd; continue;
 }
 0x4379fd => {
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x437aab; continue; }
  pc = 0x437a0a; continue;
 }
 0x437a0a => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x437a5a; continue; }
  pc = 0x437a13; continue;
 }
 0x437a13 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { pc = 0x437a5a; continue; }
  pc = 0x437a1c; continue;
 }
 0x437a1c => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if cpu.zf { pc = 0x437aab; continue; }
  pc = 0x437a29; continue;
 }
 0x437a29 => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x437a47; continue; }
  pc = 0x437a32; continue;
 }
 0x437a32 => {
  cpu.push(0x437a37); f_00433bc4(cpu); if cpu.returned_to != 0x437a37 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x437a5a; continue;
 }
 0x437a47 => {
  cpu.push(0x437a4c); f_00433bcf(cpu); if cpu.returned_to != 0x437a4c { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c6f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x437a5a; continue;
 }
 0x437a5a => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x4378f0; continue; }
  pc = 0x437a67; continue;
 }
 0x437a67 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x4378f0; continue; }
  pc = 0x437a74; continue;
 }
 0x437a74 => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x0u8); }
  if cpu.zf { pc = 0x437aa3; continue; }
  pc = 0x437a7d; continue;
 }
 0x437a7d => {
  { cpu.sub8(cpu.r8(0x58c6f1u32), 0x1u8); }
  if !cpu.zf { pc = 0x437a92; continue; }
  pc = 0x437a86; continue;
 }
 0x437a86 => {
  cpu.push(0x437a8b); f_00433ef1(cpu); if cpu.returned_to != 0x437a8b { return; }
  cpu.push(0x437a90); f_00433e0b(cpu); if cpu.returned_to != 0x437a90 { return; }
  pc = 0x437aa8; continue;
 }
 0x437a92 => {
  cpu.push(0x437a97); f_00433fc7(cpu); if cpu.returned_to != 0x437a97 { return; }
  cpu.push(0x437a9c); f_00433ef1(cpu); if cpu.returned_to != 0x437a9c { return; }
  cpu.push(0x437aa1); f_00433e0b(cpu); if cpu.returned_to != 0x437aa1 { return; }
  pc = 0x437aa8; continue;
 }
 0x437aa3 => {
  cpu.push(0x437aa8); f_00433e0b(cpu); if cpu.returned_to != 0x437aa8 { return; }
  pc = 0x437aa8; continue;
 }
 0x437aa8 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x437aab => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x5a238eu32; cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c86eu32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2361u32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23c8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f30u32, v); }
  { let v: u32 = cpu.r32(0x576e6eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x575f6eu32, v); }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437aea); f_0040c270(cpu); if cpu.returned_to != 0x437aea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x55b9b8u32; cpu.eax = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437b04); f_0040c240(cpu); if cpu.returned_to != 0x437b04 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x437b0d); f_00417e3e(cpu); if cpu.returned_to != 0x437b0d { return; }
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x437b17); f_0041b447(cpu); if cpu.returned_to != 0x437b17 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x437b23; continue; }
  pc = 0x437b22; continue;
 }
 0x437b22 => {
  { cpu.cf = true; }
  pc = 0x437b23; continue;
 }
 0x437b23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x437b24 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x5a238eu32; cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c86eu32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2361u32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23c8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x575f2fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f30u32, v); }
  { let v: u32 = cpu.r32(0x576e6eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x575f6eu32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437b63); f_0040c270(cpu); if cpu.returned_to != 0x437b63 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x55b9b8u32; cpu.eax = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437b7d); f_0040c240(cpu); if cpu.returned_to != 0x437b7d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x437b86); f_00417e3e(cpu); if cpu.returned_to != 0x437b86 { return; }
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x437b90); f_0041b447(cpu); if cpu.returned_to != 0x437b90 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437b93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437b93); }
 let mut pc: u32 = 0x437b93;
 loop { match pc {
 0x437b93 => {
  cpu.push(0x437b98); f_00417b6f(cpu); if cpu.returned_to != 0x437b98 { return; }
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  { let v: u32 = 0x55b9b8u32; cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  cpu.push(0x437bae); f_00417e23(cpu); if cpu.returned_to != 0x437bae { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  cpu.push(0x437bbd); f_00417d55(cpu); if cpu.returned_to != 0x437bbd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437bbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437bbe); }
 let mut pc: u32 = 0x437bbe;
 loop { match pc {
 0x437bbe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x5a238eu32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5922f9u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5922fbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d39cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e5u32, v); }
  { cpu.sub8(cpu.r8(0x58c88au32), 0x0u8); }
  if !cpu.zf { pc = 0x437c1d; continue; }
  pc = 0x437c14; continue;
 }
 0x437c14 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e4u32, v); }
  pc = 0x437c24; continue;
 }
 0x437c1d => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c6e5u32, v); }
  pc = 0x437c24; continue;
 }
 0x437c24 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2370u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2371u32, v); }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x437c44; continue; }
  pc = 0x437c3b; continue;
 }
 0x437c3b => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2370u32, v); }
  pc = 0x437c4b; continue;
 }
 0x437c44 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2371u32, v); }
  pc = 0x437c4b; continue;
 }
 0x437c4b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2373u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2374u32, v); }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x437c6b; continue; }
  pc = 0x437c62; continue;
 }
 0x437c62 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2373u32, v); }
  pc = 0x437c72; continue;
 }
 0x437c6b => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2374u32, v); }
  pc = 0x437c72; continue;
 }
 0x437c72 => {
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2376u32, v); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2378u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x5a0858u32; cpu.esi = v; }
  cpu.push(0x437c99); f_0041b447(cpu); if cpu.returned_to != 0x437c99 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x437ca7; continue;
 }
 0x437ca7 => {
  cpu.push(0x437cac); f_00417df6(cpu); if cpu.returned_to != 0x437cac { return; }
  cpu.push(0x437cb1); f_00430f20(cpu); if cpu.returned_to != 0x437cb1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x437cbd); f_0041c122(cpu); if cpu.returned_to != 0x437cbd { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x437df1; continue; }
  pc = 0x437cc5; continue;
 }
 0x437cc5 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x437d71; continue; }
  pc = 0x437ccd; continue;
 }
 0x437ccd => {
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x437cde; continue; }
  pc = 0x437cd6; continue;
 }
 0x437cd6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  pc = 0x437d15; continue;
 }
 0x437cde => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if !cpu.zf { pc = 0x437d15; continue; }
  pc = 0x437ce2; continue;
 }
 0x437ce2 => {
  { cpu.pushad(); }
  cpu.push(0x437ce8); f_00403050(cpu); if cpu.returned_to != 0x437ce8 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x437d15; continue; }
  pc = 0x437ced; continue;
 }
 0x437ced => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d39au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d39bu32, v); }
  cpu.push(0x437d00); f_00436e6c(cpu); if cpu.returned_to != 0x437d00 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d39bu32, v); }
  if !cpu.cf { pc = 0x437d15; continue; }
  pc = 0x437d09; continue;
 }
 0x437d09 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39au32, v); }
  pc = 0x437df1; continue;
 }
 0x437d15 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5a0858u32; cpu.esi = v; }
  cpu.push(0x437d20); f_0041b447(cpu); if cpu.returned_to != 0x437d20 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x437d26); f_0041b488(cpu); if cpu.returned_to != 0x437d26 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x437d2e); cpu.call(t); if cpu.returned_to != 0x437d2e { return; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x437df1; continue; }
  pc = 0x437d3b; continue;
 }
 0x437d3b => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x437d52; continue; }
  pc = 0x437d44; continue;
 }
 0x437d44 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { pc = 0x437d52; continue; }
  pc = 0x437d4d; continue;
 }
 0x437d4d => {
  pc = 0x437df1; continue;
 }
 0x437d52 => {
  cpu.push(0x437d57); f_00437eba(cpu); if cpu.returned_to != 0x437d57 { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x437ca7; continue; }
  pc = 0x437d64; continue;
 }
 0x437d64 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x437ca7; continue; }
  pc = 0x437d71; continue;
 }
 0x437d71 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5922f9u32, v); }
  { cpu.sub8(cpu.r8(0x5a2370u32), 0x0u8); }
  if cpu.zf { pc = 0x437d88; continue; }
  pc = 0x437d81; continue;
 }
 0x437d81 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5922f9u32, v); }
  pc = 0x437d88; continue;
 }
 0x437d88 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5922fbu32, v); }
  { cpu.sub8(cpu.r8(0x5a2373u32), 0x0u8); }
  if cpu.zf { pc = 0x437d9f; continue; }
  pc = 0x437d98; continue;
 }
 0x437d98 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5922fbu32, v); }
  pc = 0x437d9f; continue;
 }
 0x437d9f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fdu32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922ffu32, v); }
  cpu.push(0x437dc2); f_00404640(cpu); if cpu.returned_to != 0x437dc2 { return; }
  cpu.push(0x437dc7); f_00437eba(cpu); if cpu.returned_to != 0x437dc7 { return; }
  { cpu.sub8(cpu.r8(0x58c6e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x437de0; continue; }
  pc = 0x437dd0; continue;
 }
 0x437dd0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x58c88au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x58c88bu32, v); }
  pc = 0x437dee; continue;
 }
 0x437de0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c88au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c88bu32, v); }
  pc = 0x437dee; continue;
 }
 0x437dee => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x437df1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x5a238eu32; cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fdu32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922ffu32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922f9u32, v); }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5922fbu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2376u32, v); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2378u32, v); }
  cpu.push(0x437e37); f_00437eba(cpu); if cpu.returned_to != 0x437e37 { return; }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x437e52; continue; }
  pc = 0x437e40; continue;
 }
 0x437e40 => {
  { cpu.sub8(cpu.r8(0x5a2370u32), 0x1u8); }
  if cpu.zf { pc = 0x437e62; continue; }
  pc = 0x437e49; continue;
 }
 0x437e49 => {
  { cpu.pushad(); }
  cpu.push(0x437e4f); f_00403c70(cpu); if cpu.returned_to != 0x437e4f { return; }
  { cpu.popad(); }
  pc = 0x437e62; continue;
 }
 0x437e52 => {
  { cpu.sub8(cpu.r8(0x5a2371u32), 0x1u8); }
  if cpu.zf { pc = 0x437e62; continue; }
  pc = 0x437e5b; continue;
 }
 0x437e5b => {
  { cpu.pushad(); }
  cpu.push(0x437e61); f_00403c40(cpu); if cpu.returned_to != 0x437e61 { return; }
  { cpu.popad(); }
  pc = 0x437e62; continue;
 }
 0x437e62 => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x437e6c); f_0041b447(cpu); if cpu.returned_to != 0x437e6c { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  { cpu.sub8(cpu.r8(0x56d39au32), 0x0u8); }
  if !cpu.zf { pc = 0x437e81; continue; }
  pc = 0x437e77; continue;
 }
 0x437e77 => {
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x437e81; continue; }
  pc = 0x437e80; continue;
 }
 0x437e80 => {
  { cpu.cf = true; }
  pc = 0x437e81; continue;
 }
 0x437e81 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437e82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437e82); }
 let mut pc: u32 = 0x437e82;
 loop { match pc {
 0x437e82 => {
  { cpu.sub8(cpu.r8(0x5a2370u32), 0x0u8); }
  if !cpu.zf { pc = 0x437ea3; continue; }
  pc = 0x437e8b; continue;
 }
 0x437e8b => {
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x437eb9; continue; }
  pc = 0x437e94; continue;
 }
 0x437e94 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437e9d); f_00401080(cpu); if cpu.returned_to != 0x437e9d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x437eb9; continue;
 }
 0x437ea3 => {
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if !cpu.zf { pc = 0x437eb9; continue; }
  pc = 0x437eac; continue;
 }
 0x437eac => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437eb5); f_00401010(cpu); if cpu.returned_to != 0x437eb5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x437eb9; continue;
 }
 0x437eb9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437eba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x437eba); }
 let mut pc: u32 = 0x437eba;
 loop { match pc {
 0x437eba => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437ecd); f_00401150(cpu); if cpu.returned_to != 0x437ecd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5a2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x437ee2); f_00401090(cpu); if cpu.returned_to != 0x437ee2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5a23b2u32), 0x0u8); }
  if cpu.zf { pc = 0x437f03; continue; }
  pc = 0x437eef; continue;
 }
 0x437eef => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a23b2u32)); cpu.w8(0x5a23b2u32, v); }
  if !cpu.zf { pc = 0x437f03; continue; }
  pc = 0x437ef7; continue;
 }
 0x437ef7 => {
  cpu.push(0x437efc); f_00439d18(cpu); if cpu.returned_to != 0x437efc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b2u32, v); }
  pc = 0x437f03; continue;
 }
 0x437f03 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00437f04(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x437f04); } r_00437f04(cpu, 0x437f04); }
pub fn f_00438000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x438000); } r_00437f04(cpu, 0x438000); }
fn r_00437f04(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x437f04 => {
  { cpu.pushad(); }
  cpu.push(0x437f0a); f_0043a626(cpu); if cpu.returned_to != 0x437f0a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575569u32, v); }
  cpu.push(0x437f34); f_00439d32(cpu); if cpu.returned_to != 0x437f34 { return; }
  { let v: u32 = 0x5a0a1cu32; cpu.esi = v; }
  cpu.push(0x437f3e); f_0041b447(cpu); if cpu.returned_to != 0x437f3e { return; }
  cpu.push(0x437f43); f_0043d1db(cpu); if cpu.returned_to != 0x437f43 { return; }
  if cpu.cf { pc = 0x437f51; continue; }
  pc = 0x437f45; continue;
 }
 0x437f45 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  pc = 0x437f5f; continue;
 }
 0x437f51 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2322u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2323u32, v); }
  pc = 0x437f5f; continue;
 }
 0x437f5f => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x437f69); f_00439c80(cpu); if cpu.returned_to != 0x437f69 { return; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x437f76; continue; }
  pc = 0x437f72; continue;
 }
 0x437f72 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x437f7c; continue;
 }
 0x437f76 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x437f7c; continue; }
  pc = 0x437f7a; continue;
 }
 0x437f7a => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x437f7c; continue;
 }
 0x437f7c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x437f8f; continue;
 }
 0x437f8f => {
  cpu.push(0x437f94); f_0041c122(cpu); if cpu.returned_to != 0x437f94 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a238bu32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x5a238cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1923u32; cpu.ecx = v; }
  cpu.push(0x437fba); f_00438fb9(cpu); if cpu.returned_to != 0x437fba { return; }
  cpu.push(0x437fbf); f_00417df6(cpu); if cpu.returned_to != 0x437fbf { return; }
  cpu.push(0x437fc4); f_004387ba(cpu); if cpu.returned_to != 0x437fc4 { return; }
  { let v: u32 = 0x74u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1913u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x437fe6); cpu.call(t); if cpu.returned_to != 0x437fe6 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x593005u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x593007u32, v); }
  { let v: u32 = 0x5a210cu32; cpu.esi = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43801a); f_0041cf66(cpu); if cpu.returned_to != 0x43801a { return; }
  { let v: u8 = cpu.r8(0x5a238bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x438033; continue; }
  pc = 0x438028; continue;
 }
 0x438000 => {
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43801a); f_0041cf66(cpu); if cpu.returned_to != 0x43801a { return; }
  { let v: u8 = cpu.r8(0x5a238bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x438033; continue; }
  pc = 0x438028; continue;
 }
 0x438028 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x437f8f; continue;
 }
 0x438033 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x438222; continue; }
  pc = 0x43803b; continue;
 }
 0x43803b => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x4380e1; continue; }
  pc = 0x438043; continue;
 }
 0x438043 => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if !cpu.zf { pc = 0x43806c; continue; }
  pc = 0x438047; continue;
 }
 0x438047 => {
  { cpu.pushad(); }
  cpu.push(0x43804d); f_00403050(cpu); if cpu.returned_to != 0x43804d { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x43806c; continue; }
  pc = 0x438052; continue;
 }
 0x438052 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d39au32, v); }
  cpu.push(0x43805e); f_00436e6c(cpu); if cpu.returned_to != 0x43805e { return; }
  if !cpu.cf { pc = 0x43806c; continue; }
  pc = 0x438060; continue;
 }
 0x438060 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39au32, v); }
  pc = 0x438239; continue;
 }
 0x43806c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5a0a1cu32; cpu.esi = v; }
  cpu.push(0x438077); f_0041b447(cpu); if cpu.returned_to != 0x438077 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5a238cu32), 0x48u8); }
  if !cpu.zf { pc = 0x438088; continue; }
  pc = 0x438081; continue;
 }
 0x438081 => {
  cpu.push(0x438086); f_0043985d(cpu); if cpu.returned_to != 0x438086 { return; }
  pc = 0x438096; continue;
 }
 0x438088 => {
  { cpu.sub8(cpu.r8(0x5a238cu32), 0x50u8); }
  if !cpu.zf { pc = 0x438096; continue; }
  pc = 0x438091; continue;
 }
 0x438091 => {
  cpu.push(0x438096); f_004398c9(cpu); if cpu.returned_to != 0x438096 { return; }
  pc = 0x438096; continue;
 }
 0x438096 => {
  cpu.push(0x43809b); f_0041b488(cpu); if cpu.returned_to != 0x43809b { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4380a3); cpu.call(t); if cpu.returned_to != 0x4380a3 { return; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x438222; continue; }
  pc = 0x4380b0; continue;
 }
 0x4380b0 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4380c7; continue; }
  pc = 0x4380b9; continue;
 }
 0x4380b9 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { pc = 0x4380c7; continue; }
  pc = 0x4380c2; continue;
 }
 0x4380c2 => {
  pc = 0x438222; continue;
 }
 0x4380c7 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x437f8f; continue; }
  pc = 0x4380d4; continue;
 }
 0x4380d4 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x437f8f; continue; }
  pc = 0x4380e1; continue;
 }
 0x4380e1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5978c7u32, v); }
  { cpu.sub8(cpu.r8(0x5978c6u32), 0x2u8); }
  if cpu.zf { pc = 0x43821f; continue; }
  pc = 0x4380ff; continue;
 }
 0x4380ff => {
  cpu.push(0x438104); f_00438246(cpu); if cpu.returned_to != 0x438104 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddaa0u32, v); }
  if !cpu.cf { cpu.poll(); pc = 0x437f8f; continue; }
  pc = 0x438114; continue;
 }
 0x438114 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43811d); f_00401080(cpu); if cpu.returned_to != 0x43811d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x43812b); f_00417dab(cpu); if cpu.returned_to != 0x43812b { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x43813d; continue; }
  pc = 0x438138; continue;
 }
 0x438138 => {
  cpu.push(0x43813d); f_004175d9(cpu); if cpu.returned_to != 0x43813d { return; }
  pc = 0x43813d; continue;
 }
 0x43813d => {
  { let v: u8 = 0x1u8; cpu.w8(0x5824e3u32, v); }
  cpu.push(0x438149); f_0047429a(cpu); if cpu.returned_to != 0x438149 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5824e3u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x438165); f_0046f4a7(cpu); if cpu.returned_to != 0x438165 { return; }
  { cpu.pushad(); }
  cpu.push(0x43816b); f_00401080(cpu); if cpu.returned_to != 0x43816b { return; }
  cpu.push(0x438170); f_004010e0(cpu); if cpu.returned_to != 0x438170 { return; }
  cpu.push(0x438175); f_00401080(cpu); if cpu.returned_to != 0x438175 { return; }
  cpu.push(0x43817a); f_00401140(cpu); if cpu.returned_to != 0x43817a { return; }
  { cpu.popad(); }
  cpu.push(0x438180); f_00415e2b(cpu); if cpu.returned_to != 0x438180 { return; }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x4381b7; continue; }
  pc = 0x438189; continue;
 }
 0x438189 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x438190); f_00415e53(cpu); if cpu.returned_to != 0x438190 { return; }
  cpu.push(0x438195); f_00415eee(cpu); if cpu.returned_to != 0x438195 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4ba3e0u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x4381b7; continue; }
  pc = 0x4381aa; continue;
 }
 0x4381aa => {
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4381b3); f_00401050(cpu); if cpu.returned_to != 0x4381b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x4381b7; continue;
 }
 0x4381b7 => {
  cpu.push(0x4381bc); f_00417e23(cpu); if cpu.returned_to != 0x4381bc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x4381c7; continue; }
  pc = 0x4381c2; continue;
 }
 0x4381c2 => {
  cpu.push(0x4381c7); f_0041770d(cpu); if cpu.returned_to != 0x4381c7 { return; }
  pc = 0x4381c7; continue;
 }
 0x4381c7 => {
  cpu.push(0x4381cc); f_00434b8f(cpu); if cpu.returned_to != 0x4381cc { return; }
  cpu.push(0x4381d1); f_0044d980(cpu); if cpu.returned_to != 0x4381d1 { return; }
  { cpu.pushad(); }
  cpu.push(0x4381d7); f_0041879e(cpu); if cpu.returned_to != 0x4381d7 { return; }
  cpu.push(0x4381dc); f_004188ae(cpu); if cpu.returned_to != 0x4381dc { return; }
  { cpu.popad(); }
  cpu.push(0x4381e2); f_0042c9ff(cpu); if cpu.returned_to != 0x4381e2 { return; }
  cpu.push(0x4381e7); f_0047b484(cpu); if cpu.returned_to != 0x4381e7 { return; }
  cpu.push(0x4381ec); f_0045dcfe(cpu); if cpu.returned_to != 0x4381ec { return; }
  cpu.push(0x4381f1); f_0046535a(cpu); if cpu.returned_to != 0x4381f1 { return; }
  cpu.push(0x4381f6); f_00465383(cpu); if cpu.returned_to != 0x4381f6 { return; }
  cpu.push(0x4381fb); f_00465532(cpu); if cpu.returned_to != 0x4381fb { return; }
  cpu.push(0x438200); f_004665ce(cpu); if cpu.returned_to != 0x438200 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x43820f); f_00417d55(cpu); if cpu.returned_to != 0x43820f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  cpu.push(0x43821a); f_0045cac0(cpu); if cpu.returned_to != 0x43821a { return; }
  cpu.push(0x43821f); f_00420d18(cpu); if cpu.returned_to != 0x43821f { return; }
  pc = 0x43821f; continue;
 }
 0x43821f => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x438222 => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x43822c); f_0041b447(cpu); if cpu.returned_to != 0x43822c { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x438238; continue; }
  pc = 0x438237; continue;
 }
 0x438237 => {
  { cpu.cf = true; }
  pc = 0x438238; continue;
 }
 0x438238 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x438239 => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x438243); f_0041b447(cpu); if cpu.returned_to != 0x438243 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438246(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438246); }
 let mut pc: u32 = 0x438246;
 loop { match pc {
 0x438246 => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x4383ae; continue; }
  pc = 0x438258; continue;
 }
 0x438258 => {
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x438262); f_0041b447(cpu); if cpu.returned_to != 0x438262 { return; }
  cpu.push(0x438267); f_004384e2(cpu); if cpu.returned_to != 0x438267 { return; }
  cpu.push(0x43826c); f_004383d4(cpu); if cpu.returned_to != 0x43826c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x438274); cpu.call(t); if cpu.returned_to != 0x438274 { return; }
  cpu.push(0x438279); f_004384e2(cpu); if cpu.returned_to != 0x438279 { return; }
  cpu.push(0x43827e); f_004383d4(cpu); if cpu.returned_to != 0x43827e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x43828c; continue;
 }
 0x43828c => {
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x4382d9; continue; }
  pc = 0x438295; continue;
 }
 0x438295 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x4382b5; continue; }
  pc = 0x4382aa; continue;
 }
 0x4382aa => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x4382b5; continue;
 }
 0x4382b5 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4382d8); cpu.call(t); if cpu.returned_to != 0x4382d8 { return; }
  { cpu.popad(); }
  pc = 0x4382d9; continue;
 }
 0x4382d9 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x438326; continue; }
  pc = 0x4382e2; continue;
 }
 0x4382e2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x438302; continue; }
  pc = 0x4382f7; continue;
 }
 0x4382f7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x438302; continue;
 }
 0x438302 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438325); cpu.call(t); if cpu.returned_to != 0x438325 { return; }
  { cpu.popad(); }
  pc = 0x438326; continue;
 }
 0x438326 => {
  cpu.push(0x43832b); f_00417df6(cpu); if cpu.returned_to != 0x43832b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x438337); f_0041c122(cpu); if cpu.returned_to != 0x438337 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x43834b; continue; }
  pc = 0x43833b; continue;
 }
 0x43833b => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x43834b; continue; }
  pc = 0x438344; continue;
 }
 0x438344 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x43834b; continue;
 }
 0x43834b => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x43835f; continue; }
  pc = 0x43834f; continue;
 }
 0x43834f => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x43835f; continue; }
  pc = 0x438358; continue;
 }
 0x438358 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x43835f; continue;
 }
 0x43835f => {
  cpu.push(0x438364); f_0041b488(cpu); if cpu.returned_to != 0x438364 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x43836e); cpu.call(t); if cpu.returned_to != 0x43836e { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x438382; continue; }
  pc = 0x438377; continue;
 }
 0x438377 => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x438382; continue; }
  pc = 0x438380; continue;
 }
 0x438380 => {
  pc = 0x43839e; continue;
 }
 0x438382 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x43828c; continue; }
  pc = 0x43838f; continue;
 }
 0x43838f => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x43828c; continue; }
  pc = 0x43839c; continue;
 }
 0x43839c => {
  pc = 0x4383ae; continue;
 }
 0x43839e => {
  { cpu.cf = true; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4383ae => {
  { let v: u32 = 0x5a0a1cu32; cpu.esi = v; }
  cpu.push(0x4383b8); f_0041b447(cpu); if cpu.returned_to != 0x4383b8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004383d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4383d4); }
 let mut pc: u32 = 0x4383d4;
 loop { match pc {
 0x4383d4 => {
  cpu.push(0x4383d9); f_0043a626(cpu); if cpu.returned_to != 0x4383d9 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4383fa); cpu.call(t); if cpu.returned_to != 0x4383fa { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add5cu32); cpu.esi = v; }
  { let v: u32 = 0x5a6671u32; cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  pc = 0x43840c; continue;
 }
 0x43840c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x438419; continue; }
  pc = 0x438412; continue;
 }
 0x438412 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x43840c; continue; }
  pc = 0x438419; continue;
 }
 0x438419 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if (cpu.sf != cpu.of) { pc = 0x438430; continue; }
  pc = 0x438427; continue;
 }
 0x438427 => {
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x30u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  pc = 0x438439; continue;
 }
 0x438430 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x31u32); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  pc = 0x438439; continue;
 }
 0x438439 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2318u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a668eu32; cpu.esi = v; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x5a6671u32; cpu.esi = v; }
  { let v: u32 = 0x58u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x43848f); f_0041cf66(cpu); if cpu.returned_to != 0x43848f { return; }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x5a6691u32; cpu.edi = v; }
  pc = 0x4384a2; continue;
 }
 0x4384a2 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x4384aa; continue; }
  pc = 0x4384a7; continue;
 }
 0x4384a7 => {
  { cpu.stos(1, false); }
  cpu.poll(); pc = 0x4384a2; continue;
 }
 0x4384aa => {
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5a668du32; cpu.esi = v; }
  { let v: u32 = 0x58u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x4384df); f_0041cf66(cpu); if cpu.returned_to != 0x4384df { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004384e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4384e2); }
 let mut pc: u32 = 0x4384e2;
 loop { match pc {
 0x4384e2 => {
  { let v: u32 = 0x74u32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { let v: u32 = 0x1913u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438504); cpu.call(t); if cpu.returned_to != 0x438504 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x593005u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x593007u32, v); }
  { let v: u32 = 0x5a210cu32; cpu.esi = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x438538); f_0041cf66(cpu); if cpu.returned_to != 0x438538 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438539(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438539); }
 let mut pc: u32 = 0x438539;
 loop { match pc {
 0x438539 => {
  { cpu.pushad(); }
  cpu.push(0x43853f); f_0043a626(cpu); if cpu.returned_to != 0x43853f { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x56d39cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2380u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575569u32, v); }
  cpu.push(0x438569); f_00439d32(cpu); if cpu.returned_to != 0x438569 { return; }
  { let v: u32 = 0x5a0b00u32; cpu.esi = v; }
  cpu.push(0x438573); f_0041b447(cpu); if cpu.returned_to != 0x438573 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2322u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2323u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x438592); f_00439c80(cpu); if cpu.returned_to != 0x438592 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x4385a0; continue;
 }
 0x4385a0 => {
  cpu.push(0x4385a5); f_0041c122(cpu); if cpu.returned_to != 0x4385a5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a238bu32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x5a238cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x29u32; cpu.ebx = v; }
  { let v: u32 = 0x1924u32; cpu.ecx = v; }
  cpu.push(0x4385cb); f_00438fb9(cpu); if cpu.returned_to != 0x4385cb { return; }
  cpu.push(0x4385d0); f_00417df6(cpu); if cpu.returned_to != 0x4385d0 { return; }
  cpu.push(0x4385d5); f_004387ba(cpu); if cpu.returned_to != 0x4385d5 { return; }
  cpu.push(0x4385da); f_004384e2(cpu); if cpu.returned_to != 0x4385da { return; }
  cpu.push(0x4385df); f_00438d6b(cpu); if cpu.returned_to != 0x4385df { return; }
  cpu.push(0x4385e4); f_00438e41(cpu); if cpu.returned_to != 0x4385e4 { return; }
  { let v: u8 = cpu.r8(0x5a238bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d39cu32), 0x0u8); }
  if cpu.zf { pc = 0x4385fa; continue; }
  pc = 0x4385f2; continue;
 }
 0x4385f2 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x56d39cu32)); cpu.w8(0x56d39cu32, v); }
  cpu.poll(); pc = 0x4385a0; continue;
 }
 0x4385fa => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x438796; continue; }
  pc = 0x438602; continue;
 }
 0x438602 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x4386a8; continue; }
  pc = 0x43860a; continue;
 }
 0x43860a => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if !cpu.zf { pc = 0x438633; continue; }
  pc = 0x43860e; continue;
 }
 0x43860e => {
  { cpu.pushad(); }
  cpu.push(0x438614); f_00403050(cpu); if cpu.returned_to != 0x438614 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x438633; continue; }
  pc = 0x438619; continue;
 }
 0x438619 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d39au32, v); }
  cpu.push(0x438625); f_00436e6c(cpu); if cpu.returned_to != 0x438625 { return; }
  if !cpu.cf { pc = 0x438633; continue; }
  pc = 0x438627; continue;
 }
 0x438627 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d39au32, v); }
  pc = 0x4387ad; continue;
 }
 0x438633 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5a0b00u32; cpu.esi = v; }
  cpu.push(0x43863e); f_0041b447(cpu); if cpu.returned_to != 0x43863e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5a238cu32), 0x48u8); }
  if !cpu.zf { pc = 0x43864f; continue; }
  pc = 0x438648; continue;
 }
 0x438648 => {
  cpu.push(0x43864d); f_00439716(cpu); if cpu.returned_to != 0x43864d { return; }
  pc = 0x43865d; continue;
 }
 0x43864f => {
  { cpu.sub8(cpu.r8(0x5a238cu32), 0x50u8); }
  if !cpu.zf { pc = 0x43865d; continue; }
  pc = 0x438658; continue;
 }
 0x438658 => {
  cpu.push(0x43865d); f_00439742(cpu); if cpu.returned_to != 0x43865d { return; }
  pc = 0x43865d; continue;
 }
 0x43865d => {
  cpu.push(0x438662); f_0041b488(cpu); if cpu.returned_to != 0x438662 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x43866a); cpu.call(t); if cpu.returned_to != 0x43866a { return; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x438796; continue; }
  pc = 0x438677; continue;
 }
 0x438677 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x43868e; continue; }
  pc = 0x438680; continue;
 }
 0x438680 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { pc = 0x43868e; continue; }
  pc = 0x438689; continue;
 }
 0x438689 => {
  pc = 0x438796; continue;
 }
 0x43868e => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x4385a0; continue; }
  pc = 0x43869b; continue;
 }
 0x43869b => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x4385a0; continue; }
  pc = 0x4386a8; continue;
 }
 0x4386a8 => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4386b2); f_00438a87(cpu); if cpu.returned_to != 0x4386b2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddaa0u32, v); }
  if !cpu.cf { pc = 0x4386ca; continue; }
  pc = 0x4386be; continue;
 }
 0x4386be => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.poll(); pc = 0x4385a0; continue;
 }
 0x4386ca => {
  { let v: u16 = 0x0u16; cpu.w16(0x5a2381u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a210cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5a23b5u32), 0x0u8); }
  if !cpu.zf { pc = 0x4386fd; continue; }
  pc = 0x4386e6; continue;
 }
 0x4386e6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x74706d45u32); }
  if !cpu.zf { pc = 0x4386fd; continue; }
  pc = 0x4386ee; continue;
 }
 0x4386ee => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x6c532079u32); }
  if !cpu.zf { pc = 0x4386fd; continue; }
  pc = 0x4386f7; continue;
 }
 0x4386f7 => {
  { let v: u32 = cpu.r32(0x5ade08u32); cpu.esi = v; }
  pc = 0x4386fd; continue;
 }
 0x4386fd => {
  { let v: u32 = cpu.esi; cpu.w32(0x576e6au32, v); }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x438719; continue; }
  pc = 0x43870c; continue;
 }
 0x43870c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x438715); f_00401080(cpu); if cpu.returned_to != 0x438715 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x438719; continue;
 }
 0x438719 => {
  { cpu.pushad(); }
  cpu.push(0x43871f); f_0041879e(cpu); if cpu.returned_to != 0x43871f { return; }
  cpu.push(0x438724); f_004188ae(cpu); if cpu.returned_to != 0x438724 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x43872c); f_0046e8d8(cpu); if cpu.returned_to != 0x43872c { return; }
  if !cpu.cf { pc = 0x438743; continue; }
  pc = 0x43872e; continue;
 }
 0x43872e => {
  { let v: u32 = cpu.r32(0x588841u32); cpu.edx = v; }
  cpu.push(0x438739); f_0042122d(cpu); if cpu.returned_to != 0x438739 { return; }
  cpu.push(0x43873e); f_0043764a(cpu); if cpu.returned_to != 0x43873e { return; }
  cpu.push(0x438743); f_00439d9d(cpu); if cpu.returned_to != 0x438743 { return; }
  pc = 0x438743; continue;
 }
 0x438743 => {
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x438793; continue; }
  pc = 0x43874c; continue;
 }
 0x43874c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x438755); f_00401010(cpu); if cpu.returned_to != 0x438755 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43876f); f_00401050(cpu); if cpu.returned_to != 0x43876f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4ba3e0u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x438793; continue; }
  pc = 0x438787; continue;
 }
 0x438787 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43878f); f_00401050(cpu); if cpu.returned_to != 0x43878f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x438793; continue;
 }
 0x438793 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x438796 => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x4387a0); f_0041b447(cpu); if cpu.returned_to != 0x4387a0 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  { cpu.sub8(cpu.r8(0x56c9d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x4387ac; continue; }
  pc = 0x4387ab; continue;
 }
 0x4387ab => {
  { cpu.cf = true; }
  pc = 0x4387ac; continue;
 }
 0x4387ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4387ad => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  cpu.push(0x4387b7); f_0041b447(cpu); if cpu.returned_to != 0x4387b7 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004387ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4387ba); }
 let mut pc: u32 = 0x4387ba;
 loop { match pc {
 0x4387ba => {
  { let v: u32 = cpu.r32(0x5ae03cu32); cpu.esi = v; }
  { let v: u32 = 0x4eu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a2383u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a2387u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x578d6fu32); cpu.edx = v; }
  cpu.push(0x4387f2); f_0041cf66(cpu); if cpu.returned_to != 0x4387f2 { return; }
  { let v: u32 = 0x4bu32; cpu.eax = v; }
  { let v: u32 = 0x4fu32; cpu.ebx = v; }
  { let v: u32 = 0x1918u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438814); cpu.call(t); if cpu.returned_to != 0x438814 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5a2323u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xabu32; cpu.eax = v; }
  { let v: u32 = 0x190du32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438842); cpu.call(t); if cpu.returned_to != 0x438842 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237eu32), 0x0u8); }
  if cpu.zf { pc = 0x438863; continue; }
  pc = 0x438858; continue;
 }
 0x438858 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237eu32)); cpu.w8(0x5a237eu32, v); }
  pc = 0x438863; continue;
 }
 0x438863 => {
  { let v: u32 = 0xa9u32; cpu.eax = v; }
  { let v: u32 = 0x52u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1919u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438886); cpu.call(t); if cpu.returned_to != 0x438886 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a237fu32), 0x0u8); }
  if cpu.zf { pc = 0x4388a7; continue; }
  pc = 0x43889c; continue;
 }
 0x43889c => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a237fu32)); cpu.w8(0x5a237fu32, v); }
  pc = 0x4388a7; continue;
 }
 0x4388a7 => {
  { let v: u32 = 0xa9u32; cpu.eax = v; }
  { let v: u32 = 0x7du32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x191bu32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4388ca); cpu.call(t); if cpu.returned_to != 0x4388ca { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x4u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x4bu32; cpu.eax = v; }
  { let v: u32 = 0x54u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x5a2323u32) as u32; cpu.ecx = v; }
  pc = 0x4388f2; continue;
 }
 0x4388f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5a23b0u32), 0x0u8); }
  if !cpu.zf { pc = 0x43891a; continue; }
  pc = 0x4388ff; continue;
 }
 0x4388ff => {
  { cpu.sub8(cpu.r8(0x5a2322u32), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x43891a; continue; }
  pc = 0x438907; continue;
 }
 0x438907 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x2u32); cpu.ebx = v; }
  { let v: u32 = 0x191du32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438919); cpu.call(t); if cpu.returned_to != 0x438919 { return; }
  { cpu.popad(); }
  pc = 0x43891a; continue;
 }
 0x43891a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if !cpu.zf { pc = 0x43894b; continue; }
  pc = 0x438929; continue;
 }
 0x438929 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  cpu.push(0x438934); f_0041cce3(cpu); if cpu.returned_to != 0x438934 { return; }
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  cpu.push(0x43893e); f_0041cce3(cpu); if cpu.returned_to != 0x43893e { return; }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  cpu.push(0x438948); f_0041cce3(cpu); if cpu.returned_to != 0x438948 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x438960; continue;
 }
 0x43894b => {
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x31u32); cpu.ecx = v; }
  cpu.push(0x438956); f_0041cce3(cpu); if cpu.returned_to != 0x438956 { return; }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  cpu.push(0x438960); f_0041cce3(cpu); if cpu.returned_to != 0x438960 { return; }
  pc = 0x438960; continue;
 }
 0x438960 => {
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5du32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0x4cu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x438988; continue; }
  pc = 0x43897f; continue;
 }
 0x43897f => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if !cpu.zf { pc = 0x43898d; continue; }
  pc = 0x438988; continue;
 }
 0x438988 => {
  { let v: u32 = 0x4eu32; cpu.ecx = v; }
  pc = 0x43898d; continue;
 }
 0x43898d => {
  { cpu.sub8(cpu.r8(0x5a2380u32), 0x0u8); }
  if cpu.zf { pc = 0x43899b; continue; }
  pc = 0x438996; continue;
 }
 0x438996 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x43899b; continue;
 }
 0x43899b => {
  cpu.push(0x4389a0); f_0041cce3(cpu); if cpu.returned_to != 0x4389a0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2380u32), 0x0u8); }
  if !cpu.zf { pc = 0x4389bb; continue; }
  pc = 0x4389aa; continue;
 }
 0x4389aa => {
  { let v: u8 = cpu.r8(0x5a2318u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x588845u32, v); }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x4389f1; continue; }
  pc = 0x4389bb; continue;
 }
 0x4389bb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2380u32), 0x0u8); }
  if cpu.zf { pc = 0x4389d0; continue; }
  pc = 0x4389cb; continue;
 }
 0x4389cb => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x4389d0; continue;
 }
 0x4389d0 => {
  cpu.push(0x4389d5); f_0041cce3(cpu); if cpu.returned_to != 0x4389d5 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2380u32), 0x0u8); }
  if cpu.zf { pc = 0x4389e9; continue; }
  pc = 0x4389e4; continue;
 }
 0x4389e4 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x4389e9; continue;
 }
 0x4389e9 => {
  cpu.push(0x4389ee); f_0041cce3(cpu); if cpu.returned_to != 0x4389ee { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x438a23; continue;
 }
 0x4389f1 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u8) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  cpu.push(0x438a17); f_0041cce3(cpu); if cpu.returned_to != 0x438a17 { return; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  cpu.push(0x438a22); f_0041cce3(cpu); if cpu.returned_to != 0x438a22 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x438a23; continue;
 }
 0x438a23 => {
  { cpu.sub8(cpu.r8(0x56c9a8u32), 0x0u8); }
  if cpu.zf { pc = 0x438a35; continue; }
  pc = 0x438a2c; continue;
 }
 0x438a2c => {
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x438a4b; continue;
 }
 0x438a35 => {
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x588845u32), 0xffu8); }
  if !cpu.zf { pc = 0x438a4b; continue; }
  pc = 0x438a45; continue;
 }
 0x438a45 => {
  { let v: u32 = cpu.r32(0x5ae0e0u32); cpu.esi = v; }
  pc = 0x438a4b; continue;
 }
 0x438a4b => {
  { let v: u32 = 0x6bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0x3au32; cpu.ecx = v; }
  cpu.push(0x438a60); f_0041cce3(cpu); if cpu.returned_to != 0x438a60 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x438a70); f_0041cf66(cpu); if cpu.returned_to != 0x438a70 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xdu32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4388f2; continue; }
  pc = 0x438a85; continue;
 }
 0x438a85 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438a87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438a87); }
 let mut pc: u32 = 0x438a87;
 loop { match pc {
 0x438a87 => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x438c02; continue; }
  pc = 0x438a99; continue;
 }
 0x438a99 => {
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2318u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x438c02; continue; }
  pc = 0x438ab0; continue;
 }
 0x438ab0 => {
  { let v: u32 = 0x5a0c00u32; cpu.esi = v; }
  cpu.push(0x438aba); f_0041b447(cpu); if cpu.returned_to != 0x438aba { return; }
  cpu.push(0x438abf); f_004384e2(cpu); if cpu.returned_to != 0x438abf { return; }
  cpu.push(0x438ac4); f_00438e92(cpu); if cpu.returned_to != 0x438ac4 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x438acc); cpu.call(t); if cpu.returned_to != 0x438acc { return; }
  cpu.push(0x438ad1); f_004384e2(cpu); if cpu.returned_to != 0x438ad1 { return; }
  cpu.push(0x438ad6); f_00438e92(cpu); if cpu.returned_to != 0x438ad6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x438ae4; continue;
 }
 0x438ae4 => {
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x438b31; continue; }
  pc = 0x438aed; continue;
 }
 0x438aed => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x438b0d; continue; }
  pc = 0x438b02; continue;
 }
 0x438b02 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x438b0d; continue;
 }
 0x438b0d => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438b30); cpu.call(t); if cpu.returned_to != 0x438b30 { return; }
  { cpu.popad(); }
  pc = 0x438b31; continue;
 }
 0x438b31 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x438b7e; continue; }
  pc = 0x438b3a; continue;
 }
 0x438b3a => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x438b5a; continue; }
  pc = 0x438b4f; continue;
 }
 0x438b4f => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x438b5a; continue;
 }
 0x438b5a => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438b7d); cpu.call(t); if cpu.returned_to != 0x438b7d { return; }
  { cpu.popad(); }
  pc = 0x438b7e; continue;
 }
 0x438b7e => {
  cpu.push(0x438b83); f_00417df6(cpu); if cpu.returned_to != 0x438b83 { return; }
  cpu.push(0x438b88); f_0041c122(cpu); if cpu.returned_to != 0x438b88 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x438b9c; continue; }
  pc = 0x438b8c; continue;
 }
 0x438b8c => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x438b9c; continue; }
  pc = 0x438b95; continue;
 }
 0x438b95 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x438b9c; continue;
 }
 0x438b9c => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x438bb0; continue; }
  pc = 0x438ba0; continue;
 }
 0x438ba0 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x438bb0; continue; }
  pc = 0x438ba9; continue;
 }
 0x438ba9 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x438bb0; continue;
 }
 0x438bb0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9eau32, v); }
  cpu.push(0x438bbc); f_0041b488(cpu); if cpu.returned_to != 0x438bbc { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x438bc6); cpu.call(t); if cpu.returned_to != 0x438bc6 { return; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x438bda; continue; }
  pc = 0x438bcf; continue;
 }
 0x438bcf => {
  { cpu.and8(cpu.r8(0x5a2363u32), 0xeu8); }
  if !cpu.zf { pc = 0x438bda; continue; }
  pc = 0x438bd8; continue;
 }
 0x438bd8 => {
  pc = 0x438c02; continue;
 }
 0x438bda => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x438ae4; continue; }
  pc = 0x438be7; continue;
 }
 0x438be7 => {
  { cpu.and8(cpu.r8(0x5a2362u32), 0xeu8); }
  if !cpu.zf { cpu.poll(); pc = 0x438ae4; continue; }
  pc = 0x438bf4; continue;
 }
 0x438bf4 => {
  pc = 0x438bf6; continue;
 }
 0x438bf6 => {
  { let v: u32 = 0x5a0b00u32; cpu.esi = v; }
  cpu.push(0x438c00); f_0041b447(cpu); if cpu.returned_to != 0x438c00 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x438c02 => {
  { let v: u32 = 0x5a0b00u32; cpu.esi = v; }
  cpu.push(0x438c0c); f_0041b447(cpu); if cpu.returned_to != 0x438c0c { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438d6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438d6b); }
 let mut pc: u32 = 0x438d6b;
 loop { match pc {
 0x438d6b => {
  { cpu.sub16(cpu.r16(0x5a2381u32), 0x0u16); }
  if cpu.zf { pc = 0x438e40; continue; }
  pc = 0x438d79; continue;
 }
 0x438d79 => {
  { let v: u8 = cpu.r8(0x5a238bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x438d90; continue; }
  pc = 0x438d82; continue;
 }
 0x438d82 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5a2381u32, v); }
  pc = 0x438e40; continue;
 }
 0x438d90 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x438db2; continue; }
  pc = 0x438d94; continue;
 }
 0x438d94 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x593009u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x438e40; continue; }
  pc = 0x438da6; continue;
 }
 0x438da6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a210bu32.wrapping_add(cpu.eax), v); }
  pc = 0x438e40; continue;
 }
 0x438db2 => {
  { let v: u16 = 0xb2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x438dc4; continue; }
  pc = 0x438dbf; continue;
 }
 0x438dbf => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xa0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x438dc4; continue;
 }
 0x438dc4 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x593005u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x7u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x438e40; continue; }
  pc = 0x438dd1; continue;
 }
 0x438dd1 => {
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x438ddb; continue; }
  pc = 0x438dd5; continue;
 }
 0x438dd5 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x438ddb; continue; }
  pc = 0x438dd9; continue;
 }
 0x438dd9 => {
  pc = 0x438dfb; continue;
 }
 0x438ddb => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x438def; continue; }
  pc = 0x438ddf; continue;
 }
 0x438ddf => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x438def; continue; }
  pc = 0x438de3; continue;
 }
 0x438de3 => {
  { cpu.sub8((cpu.eax as u8), 0x2cu8); }
  if cpu.zf { pc = 0x438def; continue; }
  pc = 0x438de7; continue;
 }
 0x438de7 => {
  { cpu.sub8((cpu.eax as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x438e40; continue; }
  pc = 0x438deb; continue;
 }
 0x438deb => {
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x438e40; continue; }
  pc = 0x438def; continue;
 }
 0x438def => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x593009u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x438e13; continue;
 }
 0x438dfb => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x593009u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x438e0b); f_00403030(cpu); if cpu.returned_to != 0x438e0b { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x438e13; continue; }
  pc = 0x438e10; continue;
 }
 0x438e10 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x438e13; continue;
 }
 0x438e13 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a210cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a210du32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23b5u32, v); }
  { cpu.sub8(cpu.r8(0x5a238bu32), 0x71u8); }
  if cpu.zf { pc = 0x438e39; continue; }
  pc = 0x438e30; continue;
 }
 0x438e30 => {
  { cpu.sub8(cpu.r8(0x5a238bu32), 0x51u8); }
  if cpu.zf { pc = 0x438e40; continue; }
  pc = 0x438e39; continue;
 }
 0x438e39 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a238bu32, v); }
  pc = 0x438e40; continue;
 }
 0x438e40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438e41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438e41); }
 let mut pc: u32 = 0x438e41;
 loop { match pc {
 0x438e41 => {
  { cpu.sub16(cpu.r16(0x5a2381u32), 0x0u16); }
  if cpu.zf { pc = 0x438e91; continue; }
  pc = 0x438e4b; continue;
 }
 0x438e4b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a238du32)); cpu.w8(0x5a238du32, v); }
  { let v: u8 = cpu.r8(0x5a238du32) & 0xfu8; cpu.w8(0x5a238du32, v); }
  { cpu.sub8(cpu.r8(0x5a238du32), 0x7u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x438e91; continue; }
  pc = 0x438e61; continue;
 }
 0x438e61 => {
  { let v: u16 = cpu.r16(0x593005u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x593007u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x191eu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x593009u32), 0x0u16); }
  if !cpu.zf { pc = 0x438e8a; continue; }
  pc = 0x438e7e; continue;
 }
 0x438e7e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  pc = 0x438e8a; continue;
 }
 0x438e8a => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438e90); cpu.call(t); if cpu.returned_to != 0x438e90 { return; }
  { cpu.popad(); }
  pc = 0x438e91; continue;
 }
 0x438e91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438e92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438e92); }
 let mut pc: u32 = 0x438e92;
 loop { match pc {
 0x438e92 => {
  cpu.push(0x438e97); f_0043a626(cpu); if cpu.returned_to != 0x438e97 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x191fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438eb8); cpu.call(t); if cpu.returned_to != 0x438eb8 { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5add68u32); cpu.esi = v; }
  { let v: u32 = 0x5a6739u32; cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  pc = 0x438eca; continue;
 }
 0x438eca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x438ed7; continue; }
  pc = 0x438ed0; continue;
 }
 0x438ed0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x438eca; continue; }
  pc = 0x438ed7; continue;
 }
 0x438ed7 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if (cpu.sf != cpu.of) { pc = 0x438eee; continue; }
  pc = 0x438ee5; continue;
 }
 0x438ee5 => {
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x30u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  pc = 0x438ef7; continue;
 }
 0x438eee => {
  { let v: u32 = cpu.ecx.wrapping_add(0x31u32); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  pc = 0x438ef7; continue;
 }
 0x438ef7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2318u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5a6755u32; cpu.esi = v; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5add68u32); cpu.esi = v; }
  { let v: u32 = 0x5a6739u32; cpu.esi = v; }
  { let v: u32 = 0x58u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x438f53); f_0041cf66(cpu); if cpu.returned_to != 0x438f53 { return; }
  { let v: u32 = cpu.r8(0x5a2322u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x5a6758u32; cpu.edi = v; }
  pc = 0x438f66; continue;
 }
 0x438f66 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x438f6e; continue; }
  pc = 0x438f6b; continue;
 }
 0x438f6b => {
  { cpu.stos(1, false); }
  cpu.poll(); pc = 0x438f66; continue;
 }
 0x438f6e => {
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5a6754u32; cpu.esi = v; }
  { let v: u32 = 0x58u32; cpu.eax = v; }
  { let v: u32 = 0x65u32; cpu.ebx = v; }
  { let v: u32 = 0x1319u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  cpu.push(0x438fa3); f_0041cf66(cpu); if cpu.returned_to != 0x438fa3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00438fb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x438fb9); }
 let mut pc: u32 = 0x438fb9;
 loop { match pc {
 0x438fb9 => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a235du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5a235fu32, v); }
  { let v: u32 = 0x1905u32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x438fe0); cpu.call(t); if cpu.returned_to != 0x438fe0 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x5a235du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5a235fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x1cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x439008); cpu.call(t); if cpu.returned_to != 0x439008 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x576bd6u32), 0x0u8); }
  if !cpu.zf { pc = 0x439056; continue; }
  pc = 0x439012; continue;
 }
 0x439012 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if cpu.zf { pc = 0x439032; continue; }
  pc = 0x439027; continue;
 }
 0x439027 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2362u32)); cpu.w8(0x5a2362u32, v); }
  pc = 0x439032; continue;
 }
 0x439032 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x8eu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1908u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x439055); cpu.call(t); if cpu.returned_to != 0x439055 { return; }
  { cpu.popad(); }
  pc = 0x439056; continue;
 }
 0x439056 => {
  { cpu.sub8(cpu.r8(0x5a23b1u32), 0x0u8); }
  if !cpu.zf { pc = 0x4390a3; continue; }
  pc = 0x43905f; continue;
 }
 0x43905f => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if cpu.zf { pc = 0x43907f; continue; }
  pc = 0x439074; continue;
 }
 0x439074 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2363u32)); cpu.w8(0x5a2363u32, v); }
  pc = 0x43907f; continue;
 }
 0x43907f => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  { let v: u32 = 0x8eu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1906u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5a23bau32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4390a2); cpu.call(t); if cpu.returned_to != 0x4390a2 { return; }
  { cpu.popad(); }
  pc = 0x4390a3; continue;
 }
 0x4390a3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439137); }
 let mut pc: u32 = 0x439137;
 loop { match pc {
 0x439137 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x0u8); }
  if !cpu.zf { pc = 0x43914e; continue; }
  pc = 0x439140; continue;
 }
 0x439140 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x43914e; continue;
 }
 0x43914e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043914f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43914f); }
 let mut pc: u32 = 0x43914f;
 loop { match pc {
 0x43914f => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x0u8); }
  if !cpu.zf { pc = 0x439166; continue; }
  pc = 0x439158; continue;
 }
 0x439158 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x439166; continue;
 }
 0x439166 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439167(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439167); }
 let mut pc: u32 = 0x439167;
 loop { match pc {
 0x439167 => {
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x439183; continue; }
  pc = 0x439170; continue;
 }
 0x439170 => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x439193; continue; }
  pc = 0x439183; continue;
 }
 0x439183 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x439193; continue; }
  pc = 0x43918c; continue;
 }
 0x43918c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2363u32, v); }
  pc = 0x439193; continue;
 }
 0x439193 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043919b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43919b); }
 let mut pc: u32 = 0x43919b;
 loop { match pc {
 0x43919b => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4391ab; continue; }
  pc = 0x4391a4; continue;
 }
 0x4391a4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5a2362u32, v); }
  pc = 0x4391ab; continue;
 }
 0x4391ab => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004391b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4391b3); }
 let mut pc: u32 = 0x4391b3;
 loop { match pc {
 0x4391b3 => {
  { cpu.sub8(cpu.r8(0x5a2363u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4391cf; continue; }
  pc = 0x4391bc; continue;
 }
 0x4391bc => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x4391cf); f_00422120(cpu); if cpu.returned_to != 0x4391cf { return; }
  pc = 0x4391cf; continue;
 }
 0x4391cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004391d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4391d0); }
 let mut pc: u32 = 0x4391d0;
 loop { match pc {
 0x4391d0 => {
  { cpu.sub8(cpu.r8(0x5a2362u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4391ec; continue; }
  pc = 0x4391d9; continue;
 }
 0x4391d9 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5a2362u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2363u32, v); }
  cpu.push(0x4391ec); f_00421fd6(cpu); if cpu.returned_to != 0x4391ec { return; }
  pc = 0x4391ec; continue;
 }
 0x4391ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004391ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4391ed); }
 let mut pc: u32 = 0x4391ed;
 loop { match pc {
 0x4391ed => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x79u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0xdu32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x439210; continue; }
  pc = 0x43920c; continue;
 }
 0x43920c => {
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x439210; continue;
 }
 0x439210 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e64u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x575f6eu32, v); }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439229); f_0040c270(cpu); if cpu.returned_to != 0x439229 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x55b9b8u32; cpu.eax = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439245); f_0040c240(cpu); if cpu.returned_to != 0x439245 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x43924e); f_00417e3e(cpu); if cpu.returned_to != 0x43924e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043924f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43924f); }
 let mut pc: u32 = 0x43924f;
 loop { match pc {
 0x43924f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x439260; continue; }
  pc = 0x43925f; continue;
 }
 0x43925f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439260 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x79u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0xdu32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x439283; continue; }
  pc = 0x43927f; continue;
 }
 0x43927f => {
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x439283; continue;
 }
 0x439283 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x576e62u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439291(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439291); }
 let mut pc: u32 = 0x439291;
 loop { match pc {
 0x439291 => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392a0); }
 let mut pc: u32 = 0x4392a0;
 loop { match pc {
 0x4392a0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e4u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392af); }
 let mut pc: u32 = 0x4392af;
 loop { match pc {
 0x4392af => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392b7); }
 let mut pc: u32 = 0x4392b7;
 loop { match pc {
 0x4392b7 => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392cd); }
 let mut pc: u32 = 0x4392cd;
 loop { match pc {
 0x4392cd => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392e3); }
 let mut pc: u32 = 0x4392e3;
 loop { match pc {
 0x4392e3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6e8u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6e9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004392f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4392f9); }
 let mut pc: u32 = 0x4392f9;
 loop { match pc {
 0x4392f9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x439303; continue; }
  pc = 0x439302; continue;
 }
 0x439302 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439303 => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439319); }
 let mut pc: u32 = 0x439319;
 loop { match pc {
 0x439319 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x439323; continue; }
  pc = 0x439322; continue;
 }
 0x439322 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439323 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439339(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439339); }
 let mut pc: u32 = 0x439339;
 loop { match pc {
 0x439339 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x439343; continue; }
  pc = 0x439342; continue;
 }
 0x439342 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439343 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6eau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6ebu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439359(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439359); }
 let mut pc: u32 = 0x439359;
 loop { match pc {
 0x439359 => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6edu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6eeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439368); }
 let mut pc: u32 = 0x439368;
 loop { match pc {
 0x439368 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6edu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6eeu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439377(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439377); }
 let mut pc: u32 = 0x439377;
 loop { match pc {
 0x439377 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a236cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439386(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439386); }
 let mut pc: u32 = 0x439386;
 loop { match pc {
 0x439386 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a236cu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a236du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439395(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439395); }
 let mut pc: u32 = 0x439395;
 loop { match pc {
 0x439395 => {
  { cpu.sub8(cpu.r8(0x58c6eeu32), 0x0u8); }
  if !cpu.zf { pc = 0x43939f; continue; }
  pc = 0x43939e; continue;
 }
 0x43939e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43939f => {
  { let v: u8 = 0x5u8; cpu.w8(0x58c6efu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004393ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4393ae); }
 let mut pc: u32 = 0x4393ae;
 loop { match pc {
 0x4393ae => {
  { cpu.sub8(cpu.r8(0x58c6eeu32), 0x0u8); }
  if !cpu.zf { pc = 0x4393b8; continue; }
  pc = 0x4393b7; continue;
 }
 0x4393b7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4393b8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c6efu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x58c6f0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004393c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4393c7); }
 let mut pc: u32 = 0x4393c7;
 loop { match pc {
 0x4393c7 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x4393d1; continue; }
  pc = 0x4393d0; continue;
 }
 0x4393d0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4393d1 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4393fd; continue; }
  pc = 0x4393da; continue;
 }
 0x4393da => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  pc = 0x4393fd; continue;
 }
 0x4393fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004393fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4393fe); }
 let mut pc: u32 = 0x4393fe;
 loop { match pc {
 0x4393fe => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x439408; continue; }
  pc = 0x439407; continue;
 }
 0x439407 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439408 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x439434; continue; }
  pc = 0x439411; continue;
 }
 0x439411 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  pc = 0x439434; continue;
 }
 0x439434 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439435(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439435); }
 let mut pc: u32 = 0x439435;
 loop { match pc {
 0x439435 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439459(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439459); }
 let mut pc: u32 = 0x439459;
 loop { match pc {
 0x439459 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2368u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043947d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43947d); }
 let mut pc: u32 = 0x43947d;
 loop { match pc {
 0x43947d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2367u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2368u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004394a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4394a1); }
 let mut pc: u32 = 0x4394a1;
 loop { match pc {
 0x4394a1 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004394b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4394b7); }
 let mut pc: u32 = 0x4394b7;
 loop { match pc {
 0x4394b7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004394cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4394cd); }
 let mut pc: u32 = 0x4394cd;
 loop { match pc {
 0x4394cd => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2369u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236au32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a236bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004394e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4394e3); }
 let mut pc: u32 = 0x4394e3;
 loop { match pc {
 0x4394e3 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a236eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a236fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004394f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4394f2); }
 let mut pc: u32 = 0x4394f2;
 loop { match pc {
 0x4394f2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a236eu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a236fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439501(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439501); }
 let mut pc: u32 = 0x439501;
 loop { match pc {
 0x439501 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2370u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2371u32, v); }
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x5ddb94u32), 0x0u32); }
  if cpu.zf { pc = 0x439520; continue; }
  pc = 0x439519; continue;
 }
 0x439519 => {
  cpu.push(0x43951e); f_00403c70(cpu); if cpu.returned_to != 0x43951e { return; }
  pc = 0x43952b; continue;
 }
 0x439520 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439528); f_00401010(cpu); if cpu.returned_to != 0x439528 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x43952b; continue;
 }
 0x43952b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043952d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43952d); }
 let mut pc: u32 = 0x43952d;
 loop { match pc {
 0x43952d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2370u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2371u32, v); }
  { cpu.pushad(); }
  cpu.push(0x439541); f_00403c40(cpu); if cpu.returned_to != 0x439541 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439543(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439543); }
 let mut pc: u32 = 0x439543;
 loop { match pc {
 0x439543 => {
  { let v: u8 = 0x5u8; cpu.w8(0x5a2373u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a2374u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439552(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439552); }
 let mut pc: u32 = 0x439552;
 loop { match pc {
 0x439552 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2373u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x5a2374u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004395b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4395b5); }
 let mut pc: u32 = 0x4395b5;
 loop { match pc {
 0x4395b5 => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237au32, v); }
  { cpu.sub16(cpu.r16(0x5a2376u32), 0x0u16); }
  if cpu.zf { pc = 0x4395cd; continue; }
  pc = 0x4395c6; continue;
 }
 0x4395c6 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x5a2376u32)); cpu.w16(0x5a2376u32, v); }
  pc = 0x4395cd; continue;
 }
 0x4395cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004395ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4395ce); }
 let mut pc: u32 = 0x4395ce;
 loop { match pc {
 0x4395ce => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237bu32, v); }
  { cpu.sub16(cpu.r16(0x5a2376u32), 0xfu16); }
  if (cpu.sf == cpu.of) { pc = 0x4395e6; continue; }
  pc = 0x4395df; continue;
 }
 0x4395df => {
  { let v: u16 = cpu.inc16(cpu.r16(0x5a2376u32)); cpu.w16(0x5a2376u32, v); }
  pc = 0x4395e6; continue;
 }
 0x4395e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004395e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4395e7); }
 let mut pc: u32 = 0x4395e7;
 loop { match pc {
 0x4395e7 => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237cu32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x5a2378u32), 0x0u16); }
  if cpu.zf { pc = 0x43962e; continue; }
  pc = 0x4395fe; continue;
 }
 0x4395fe => {
  { let v: u16 = cpu.dec16(cpu.r16(0x5a2378u32)); cpu.w16(0x5a2378u32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a23b3u32)); }
  if cpu.zf { pc = 0x43962e; continue; }
  pc = 0x439613; continue;
 }
 0x439613 => {
  { cpu.sub8(cpu.r8(0x5a23b2u32), 0x0u8); }
  if !cpu.zf { pc = 0x43962e; continue; }
  pc = 0x43961c; continue;
 }
 0x43961c => {
  { let v: u8 = 0x8u8; cpu.w8(0x5a23b2u32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23b3u32, v); }
  pc = 0x43962e; continue;
 }
 0x43962e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043962f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43962f); }
 let mut pc: u32 = 0x43962f;
 loop { match pc {
 0x43962f => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237du32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x5a2378u32), 0xfu16); }
  if (cpu.sf == cpu.of) { pc = 0x439676; continue; }
  pc = 0x439646; continue;
 }
 0x439646 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x5a2378u32)); cpu.w16(0x5a2378u32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a23b3u32)); }
  if cpu.zf { pc = 0x439676; continue; }
  pc = 0x43965b; continue;
 }
 0x43965b => {
  { cpu.sub8(cpu.r8(0x5a23b2u32), 0x0u8); }
  if !cpu.zf { pc = 0x439676; continue; }
  pc = 0x439664; continue;
 }
 0x439664 => {
  { let v: u8 = 0x8u8; cpu.w8(0x5a23b2u32, v); }
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23b3u32, v); }
  pc = 0x439676; continue;
 }
 0x439676 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439677(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439677); }
 let mut pc: u32 = 0x439677;
 loop { match pc {
 0x439677 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x77u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2376u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043969e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43969e); }
 let mut pc: u32 = 0x43969e;
 loop { match pc {
 0x43969e => {
  { let v: u16 = cpu.r16(0x5a2378u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a23b3u32, v); }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x77u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23b6u32)); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5a2378u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a23b3u32)); }
  if cpu.zf { pc = 0x4396e7; continue; }
  pc = 0x4396d7; continue;
 }
 0x4396d7 => {
  { cpu.sub8(cpu.r8(0x5a23b2u32), 0x0u8); }
  if !cpu.zf { pc = 0x4396e7; continue; }
  pc = 0x4396e0; continue;
 }
 0x4396e0 => {
  { let v: u8 = 0x8u8; cpu.w8(0x5a23b2u32, v); }
  pc = 0x4396e7; continue;
 }
 0x4396e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004396e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4396e8); }
 let mut pc: u32 = 0x4396e8;
 loop { match pc {
 0x4396e8 => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237eu32, v); }
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x0u8); }
  if cpu.zf { pc = 0x4396fe; continue; }
  pc = 0x4396f8; continue;
 }
 0x4396f8 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x4396fe; continue;
 }
 0x4396fe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004396ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4396ff); }
 let mut pc: u32 = 0x4396ff;
 loop { match pc {
 0x4396ff => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237fu32, v); }
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x6u8); }
  if cpu.zf { pc = 0x439715; continue; }
  pc = 0x43970f; continue;
 }
 0x43970f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x439715; continue;
 }
 0x439715 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439716(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439716); }
 let mut pc: u32 = 0x439716;
 loop { match pc {
 0x439716 => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x0u8); }
  if cpu.zf { pc = 0x439741; continue; }
  pc = 0x43971f; continue;
 }
 0x43971f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2322u32)); cpu.w8(0x5a2322u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a2323u32)); }
  if (cpu.sf == cpu.of) { pc = 0x439741; continue; }
  pc = 0x439732; continue;
 }
 0x439732 => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x0u8); }
  if cpu.zf { pc = 0x439741; continue; }
  pc = 0x43973b; continue;
 }
 0x43973b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x439741; continue;
 }
 0x439741 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439742(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439742); }
 let mut pc: u32 = 0x439742;
 loop { match pc {
 0x439742 => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x9u8); }
  if cpu.zf { pc = 0x43976d; continue; }
  pc = 0x43974b; continue;
 }
 0x43974b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2322u32)); cpu.w8(0x5a2322u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a2323u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43976d; continue; }
  pc = 0x43975e; continue;
 }
 0x43975e => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x43976d; continue; }
  pc = 0x439767; continue;
 }
 0x439767 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x43976d; continue;
 }
 0x43976d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004397a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4397a0); }
 let mut pc: u32 = 0x4397a0;
 loop { match pc {
 0x4397a0 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x52u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5a2323u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x576e9eu32), 0x0u8); }
  if cpu.zf { pc = 0x4397d4; continue; }
  pc = 0x4397cb; continue;
 }
 0x4397cb => {
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x4397db; continue;
 }
 0x4397d4 => {
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x4397db; continue;
 }
 0x4397db => {
  { let v: u32 = 0x5a210cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u16 = 0x1u16; cpu.w16(0x5a2381u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5a23b4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004397f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4397f8); }
 let mut pc: u32 = 0x4397f8;
 loop { match pc {
 0x4397f8 => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237eu32, v); }
  cpu.push(0x439804); f_00422044(cpu); if cpu.returned_to != 0x439804 { return; }
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x43981d; continue; }
  pc = 0x43980d; continue;
 }
 0x43980d => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43981c; continue; }
  pc = 0x439816; continue;
 }
 0x439816 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x43981c; continue;
 }
 0x43981c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43981d => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43982c; continue; }
  pc = 0x439826; continue;
 }
 0x439826 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x43982c; continue;
 }
 0x43982c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043982d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43982d); }
 let mut pc: u32 = 0x43982d;
 loop { match pc {
 0x43982d => {
  { let v: u8 = 0xau8; cpu.w8(0x5a237fu32, v); }
  cpu.push(0x439839); f_00422044(cpu); if cpu.returned_to != 0x439839 { return; }
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x439852; continue; }
  pc = 0x439842; continue;
 }
 0x439842 => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x6u8); }
  if cpu.zf { pc = 0x439851; continue; }
  pc = 0x43984b; continue;
 }
 0x43984b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x439851; continue;
 }
 0x439851 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439852 => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x439851; continue; }
  pc = 0x43985b; continue;
 }
 0x43985b => {
  cpu.poll(); pc = 0x43984b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043985d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43985d); }
 let mut pc: u32 = 0x43985d;
 loop { match pc {
 0x43985d => {
  { let v: u8 = cpu.r8(0x5a2323u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5a23b0u32), 0x0u8); }
  if !cpu.zf { pc = 0x4398b9; continue; }
  pc = 0x439873; continue;
 }
 0x439873 => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x0u8); }
  if cpu.zf { pc = 0x4398b9; continue; }
  pc = 0x43987c; continue;
 }
 0x43987c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2322u32)); cpu.w8(0x5a2322u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x43989d; continue; }
  pc = 0x439892; continue;
 }
 0x439892 => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x0u8); }
  if cpu.zf { pc = 0x4398bc; continue; }
  pc = 0x43989b; continue;
 }
 0x43989b => {
  cpu.poll(); pc = 0x43987c; continue;
 }
 0x43989d => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a2323u32)); }
  if (cpu.sf == cpu.of) { pc = 0x4398b9; continue; }
  pc = 0x4398aa; continue;
 }
 0x4398aa => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x0u8); }
  if cpu.zf { pc = 0x4398b9; continue; }
  pc = 0x4398b3; continue;
 }
 0x4398b3 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x4398b9; continue;
 }
 0x4398b9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4398bc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004398c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4398c9); }
 let mut pc: u32 = 0x4398c9;
 loop { match pc {
 0x4398c9 => {
  { let v: u8 = cpu.r8(0x5a2323u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5a23b0u32), 0x0u8); }
  if !cpu.zf { pc = 0x439925; continue; }
  pc = 0x4398df; continue;
 }
 0x4398df => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x9u8); }
  if cpu.zf { pc = 0x439925; continue; }
  pc = 0x4398e8; continue;
 }
 0x4398e8 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2322u32)); cpu.w8(0x5a2322u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x439909; continue; }
  pc = 0x4398fe; continue;
 }
 0x4398fe => {
  { cpu.sub8(cpu.r8(0x5a2322u32), 0x9u8); }
  if cpu.zf { pc = 0x439928; continue; }
  pc = 0x439907; continue;
 }
 0x439907 => {
  cpu.poll(); pc = 0x4398e8; continue;
 }
 0x439909 => {
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5a2323u32)); }
  if (cpu.sf != cpu.of) { pc = 0x439925; continue; }
  pc = 0x439916; continue;
 }
 0x439916 => {
  { cpu.sub8(cpu.r8(0x5a2323u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x439925; continue; }
  pc = 0x43991f; continue;
 }
 0x43991f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5a2323u32)); cpu.w8(0x5a2323u32, v); }
  pc = 0x439925; continue;
 }
 0x439925 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439928 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a2322u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439af8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439af8); }
 let mut pc: u32 = 0x439af8;
 loop { match pc {
 0x439af8 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x5bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x439b12; continue; }
  pc = 0x439b10; continue;
 }
 0x439b10 => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x439b12; continue;
 }
 0x439b12 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439b2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439b2d); }
 let mut pc: u32 = 0x439b2d;
 loop { match pc {
 0x439b2d => {
  { cpu.sub8(cpu.r8(0x576e9eu32), 0x0u8); }
  if cpu.zf { pc = 0x439b6a; continue; }
  pc = 0x439b36; continue;
 }
 0x439b36 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x52u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5a2323u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x439b9e; continue; }
  pc = 0x439b57; continue;
 }
 0x439b57 => {
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x439b9e; continue; }
  pc = 0x439b60; continue;
 }
 0x439b60 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  cpu.push(0x439b6a); f_00439ce7(cpu); if cpu.returned_to != 0x439b6a { return; }
  pc = 0x439b6a; continue;
 }
 0x439b6a => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x52u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5a2323u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x439b94; continue; }
  pc = 0x439b8b; continue;
 }
 0x439b8b => {
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x439b9e; continue; }
  pc = 0x439b94; continue;
 }
 0x439b94 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  cpu.push(0x439b9e); f_00439c80(cpu); if cpu.returned_to != 0x439b9e { return; }
  pc = 0x439b9e; continue;
 }
 0x439b9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439b9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439b9f); }
 let mut pc: u32 = 0x439b9f;
 loop { match pc {
 0x439b9f => {
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x439be2; continue; }
  pc = 0x439ba8; continue;
 }
 0x439ba8 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5a2323u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x439bd2; continue; }
  pc = 0x439bc9; continue;
 }
 0x439bc9 => {
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x439be1; continue; }
  pc = 0x439bd2; continue;
 }
 0x439bd2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  cpu.push(0x439bdc); f_004220b2(cpu); if cpu.returned_to != 0x439bdc { return; }
  cpu.push(0x439be1); f_00439c80(cpu); if cpu.returned_to != 0x439be1 { return; }
  pc = 0x439be1; continue;
 }
 0x439be1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439be2 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5a2323u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x57577fu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5756dcu32.wrapping_add(cpu.ebx)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x439c1f; continue; }
  pc = 0x439c10; continue;
 }
 0x439c10 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  cpu.push(0x439c1a); f_00439c80(cpu); if cpu.returned_to != 0x439c1a { return; }
  cpu.push(0x439c1f); f_004220b2(cpu); if cpu.returned_to != 0x439c1f { return; }
  pc = 0x439c1f; continue;
 }
 0x439c1f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439c20); }
 let mut pc: u32 = 0x439c20;
 loop { match pc {
 0x439c20 => {
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x439c5e; continue; }
  pc = 0x439c29; continue;
 }
 0x439c29 => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x75u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x439c43; continue; }
  pc = 0x439c41; continue;
 }
 0x439c41 => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x439c43; continue;
 }
 0x439c43 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439c5e => {
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5a23bau32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x75u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575623u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2323u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439c80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x439c80); } r_00439c80(cpu, 0x439c80); }
pub fn f_00439ca9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x439ca9); } r_00439c80(cpu, 0x439ca9); }
pub fn f_00439caa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x439caa); } r_00439c80(cpu, 0x439caa); }
pub fn f_00439cac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x439cac); } r_00439c80(cpu, 0x439cac); }
fn r_00439c80(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x439c80 => {
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x439cc3; continue; }
  pc = 0x439c89; continue;
 }
 0x439c89 => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56c9a8u32), 0x0u8); }
  if cpu.zf { pc = 0x439ca0; continue; }
  pc = 0x439c97; continue;
 }
 0x439c97 => {
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x439cb6; continue;
 }
 0x439ca0 => {
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5a2318u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x439cb6; continue; }
  pc = 0x439cb0; continue;
 }
 0x439ca9 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x439cac; continue;
 }
 0x439caa => {
  { let v: u32 = cpu.and32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x439cb0; continue;
 }
 0x439cac => {
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x439cb6; continue; }
  pc = 0x439cb0; continue;
 }
 0x439cb0 => {
  { let v: u32 = cpu.r32(0x5ae0e0u32); cpu.esi = v; }
  pc = 0x439cb6; continue;
 }
 0x439cb6 => {
  { let v: u32 = 0x5a210cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x439cc3 => {
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x57577fu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x5a210cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439ce7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439ce7); }
 let mut pc: u32 = 0x439ce7;
 loop { match pc {
 0x439ce7 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x5a210cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439d00); }
 let mut pc: u32 = 0x439d00;
 loop { match pc {
 0x439d00 => {
  { let v: u16 = 0x1u16; cpu.w16(0x5a2381u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d2fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a23b5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439d18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439d18); }
 let mut pc: u32 = 0x439d18;
 loop { match pc {
 0x439d18 => {
  { cpu.sub8(cpu.r8(0x5a2373u32), 0x0u8); }
  if cpu.zf { pc = 0x439d31; continue; }
  pc = 0x439d21; continue;
 }
 0x439d21 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439d2d); f_00401130(cpu); if cpu.returned_to != 0x439d2d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x439d31; continue;
 }
 0x439d31 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439d32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439d32); }
 let mut pc: u32 = 0x439d32;
 loop { match pc {
 0x439d32 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x439d37; continue;
 }
 0x439d37 => {
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439d44); f_004701ee(cpu); if cpu.returned_to != 0x439d44 { return; }
  { let v: u8 = cpu.r8(0x5ceb67u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a230eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5ceb68u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a2318u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { cpu.poll(); pc = 0x439d37; continue; }
  pc = 0x439d63; continue;
 }
 0x439d63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439d64); }
 let mut pc: u32 = 0x439d64;
 loop { match pc {
 0x439d64 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x439d69; continue;
 }
 0x439d69 => {
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5ceb68u32, v); }
  cpu.push(0x439d7d); f_004702c4(cpu); if cpu.returned_to != 0x439d7d { return; }
  { let v: u8 = cpu.r8(0x5ceb67u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a230eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5ceb68u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5a2318u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { cpu.poll(); pc = 0x439d69; continue; }
  pc = 0x439d9c; continue;
 }
 0x439d9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439d9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439d9d); }
 let mut pc: u32 = 0x439d9d;
 loop { match pc {
 0x439d9d => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x439da2; continue;
 }
 0x439da2 => {
  { let v: u32 = cpu.r32(0x5ade50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x439daf); f_004701ee(cpu); if cpu.returned_to != 0x439daf { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x439dbf; continue; }
  pc = 0x439db2; continue;
 }
 0x439db2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae0e0u32); cpu.esi = v; }
  cpu.push(0x439dbe); f_00470132(cpu); if cpu.returned_to != 0x439dbe { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x439dbf; continue;
 }
 0x439dbf => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { cpu.poll(); pc = 0x439da2; continue; }
  pc = 0x439dc5; continue;
 }
 0x439dc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439dc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439dc7); }
 let mut pc: u32 = 0x439dc7;
 loop { match pc {
 0x439dc7 => {
  cpu.push(0x439dcc); f_0043a626(cpu); if cpu.returned_to != 0x439dcc { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00439dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x439dfc); }
 let mut pc: u32 = 0x439dfc;
 loop { match pc {
 0x439dfc => {
  cpu.push(0x439e01); f_0043a626(cpu); if cpu.returned_to != 0x439e01 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043a626(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43a626); }
 let mut pc: u32 = 0x43a626;
 loop { match pc {
 0x43a626 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x43a637; continue; }
  pc = 0x43a630; continue;
 }
 0x43a630 => {
  cpu.push(0x43a635); f_0043a63e(cpu); if cpu.returned_to != 0x43a635 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43a637 => {
  cpu.push(0x43a63c); f_0043a674(cpu); if cpu.returned_to != 0x43a63c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043a63e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43a63e); }
 let mut pc: u32 = 0x43a63e;
 loop { match pc {
 0x43a63e => {
  { let v: u32 = 0x5a0cccu32; cpu.esi = v; }
  { let v: u32 = 0x5a02acu32; cpu.edi = v; }
  pc = 0x43a648; continue;
 }
 0x43a648 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x43a65e; continue; }
  pc = 0x43a650; continue;
 }
 0x43a650 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x43a66b; continue;
 }
 0x43a65e => {
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  pc = 0x43a66b; continue;
 }
 0x43a66b => {
  { cpu.sub32(cpu.edi, 0x5a0cccu32); }
  if !cpu.zf { cpu.poll(); pc = 0x43a648; continue; }
  pc = 0x43a673; continue;
 }
 0x43a673 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043a674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43a674); }
 let mut pc: u32 = 0x43a674;
 loop { match pc {
 0x43a674 => {
  { let v: u32 = 0x5a16ecu32; cpu.esi = v; }
  { let v: u32 = 0x5a02acu32; cpu.edi = v; }
  pc = 0x43a67e; continue;
 }
 0x43a67e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x43a694; continue; }
  pc = 0x43a686; continue;
 }
 0x43a686 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x43a6a1; continue;
 }
 0x43a694 => {
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(2, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  pc = 0x43a6a1; continue;
 }
 0x43a6a1 => {
  { cpu.sub32(cpu.edi, 0x5a0cccu32); }
  if !cpu.zf { cpu.poll(); pc = 0x43a67e; continue; }
  pc = 0x43a6a9; continue;
 }
 0x43a6a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043d1db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43d1db); }
 let mut pc: u32 = 0x43d1db;
 loop { match pc {
 0x43d1db => {
  { cpu.sub8(cpu.r8(0x575569u32), 0x0u8); }
  if !cpu.zf { pc = 0x43d222; continue; }
  pc = 0x43d1e4; continue;
 }
 0x43d1e4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x43d1e6; continue;
 }
 0x43d1e6 => {
  { cpu.sub8(cpu.r8(0x5a230eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x43d1f7; continue; }
  pc = 0x43d1ef; continue;
 }
 0x43d1ef => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { cpu.poll(); pc = 0x43d1e6; continue; }
  pc = 0x43d1f5; continue;
 }
 0x43d1f5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43d1f7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x43d209; continue; }
  pc = 0x43d200; continue;
 }
 0x43d200 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a2323u32, v); }
  pc = 0x43d21b; continue;
 }
 0x43d209 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (cpu.sf != cpu.of) { pc = 0x43d216; continue; }
  pc = 0x43d20d; continue;
 }
 0x43d20d => {
  { let v: u8 = 0x6u8; cpu.w8(0x5a2323u32, v); }
  pc = 0x43d21b; continue;
 }
 0x43d216 => {
  { let v: u8 = cpu.r8(0x5a2323u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x43d21b; continue;
 }
 0x43d21b => {
  { let v: u8 = cpu.r8(0x5a2323u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43d222 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043d95c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43d95c); }
 let mut pc: u32 = 0x43d95c;
 loop { match pc {
 0x43d95c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x43d963); f_00439d64(cpu); if cpu.returned_to != 0x43d963 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a2322u32, v); }
  { cpu.sub8(cpu.r8(0x5a2318u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x43da75; continue; }
  pc = 0x43d977; continue;
 }
 0x43d977 => {
  { cpu.sub8(cpu.r8(0x5a2318u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x43da75; continue; }
  pc = 0x43d984; continue;
 }
 0x43d984 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ade78u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x43d996); f_0046f4a7(cpu); if cpu.returned_to != 0x43d996 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  cpu.push(0x43d99c); f_00434b8f(cpu); if cpu.returned_to != 0x43d99c { return; }
  cpu.push(0x43d9a1); f_0044d980(cpu); if cpu.returned_to != 0x43d9a1 { return; }
  { cpu.pushad(); }
  cpu.push(0x43d9a7); f_0041879e(cpu); if cpu.returned_to != 0x43d9a7 { return; }
  cpu.push(0x43d9ac); f_004188ae(cpu); if cpu.returned_to != 0x43d9ac { return; }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x43da75; continue; }
  pc = 0x43d9b5; continue;
 }
 0x43d9b5 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4b94d0u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x56c9dbu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x54acd1u32, v); }
  { let v: u8 = cpu.r8(0x5a2322u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x43d9d2); f_00450e9d(cpu); if cpu.returned_to != 0x43d9d2 { return; }
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x0u8); }
  if cpu.zf { pc = 0x43da75; continue; }
  pc = 0x43d9df; continue;
 }
 0x43d9df => {
  cpu.push(0x43d9e4); f_0042c9ff(cpu); if cpu.returned_to != 0x43d9e4 { return; }
  cpu.push(0x43d9e9); f_0047b484(cpu); if cpu.returned_to != 0x43d9e9 { return; }
  cpu.push(0x43d9ee); f_0045dcfe(cpu); if cpu.returned_to != 0x43d9ee { return; }
  cpu.push(0x43d9f3); f_0046535a(cpu); if cpu.returned_to != 0x43d9f3 { return; }
  cpu.push(0x43d9f8); f_00465383(cpu); if cpu.returned_to != 0x43d9f8 { return; }
  cpu.push(0x43d9fd); f_00465532(cpu); if cpu.returned_to != 0x43d9fd { return; }
  cpu.push(0x43da02); f_004665ce(cpu); if cpu.returned_to != 0x43da02 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x43da11); f_00417d55(cpu); if cpu.returned_to != 0x43da11 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  cpu.push(0x43da1c); f_0045cac0(cpu); if cpu.returned_to != 0x43da1c { return; }
  cpu.push(0x43da21); f_00420d18(cpu); if cpu.returned_to != 0x43da21 { return; }
  { cpu.pushad(); }
  cpu.push(0x43da27); f_00401080(cpu); if cpu.returned_to != 0x43da27 { return; }
  cpu.push(0x43da2c); f_004010e0(cpu); if cpu.returned_to != 0x43da2c { return; }
  cpu.push(0x43da31); f_00401080(cpu); if cpu.returned_to != 0x43da31 { return; }
  cpu.push(0x43da36); f_00401140(cpu); if cpu.returned_to != 0x43da36 { return; }
  { cpu.popad(); }
  cpu.push(0x43da3c); f_00415e2b(cpu); if cpu.returned_to != 0x43da3c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x43da43); f_00415e53(cpu); if cpu.returned_to != 0x43da43 { return; }
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x43da73; continue; }
  pc = 0x43da4c; continue;
 }
 0x43da4c => {
  cpu.push(0x43da51); f_00415eee(cpu); if cpu.returned_to != 0x43da51 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4ba3e0u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x43da73; continue; }
  pc = 0x43da66; continue;
 }
 0x43da66 => {
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43da6f); f_00401050(cpu); if cpu.returned_to != 0x43da6f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x43da73; continue;
 }
 0x43da73 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43da75 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043da77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43da77); }
 let mut pc: u32 = 0x43da77;
 loop { match pc {
 0x43da77 => {
  { cpu.sub8(cpu.r8(0x56c9eau32), 0x0u8); }
  if !cpu.zf { pc = 0x43da87; continue; }
  pc = 0x43da80; continue;
 }
 0x43da80 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9d3u32, v); }
  pc = 0x43da87; continue;
 }
 0x43da87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043da88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43da88); }
 let mut pc: u32 = 0x43da88;
 loop { match pc {
 0x43da88 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x43da92; continue; }
  pc = 0x43da91; continue;
 }
 0x43da91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43da92 => {
  { cpu.sub32(cpu.r32(0x58c4e5u32), 0x0u32); }
  if cpu.zf { pc = 0x43da9c; continue; }
  pc = 0x43da9b; continue;
 }
 0x43da9b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43da9c => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56c9eau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043daab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43daab); }
 let mut pc: u32 = 0x43daab;
 loop { match pc {
 0x43daab => {
  { cpu.sub8(cpu.r8(0x58c6f4u32), 0x0u8); }
  if !cpu.zf { pc = 0x43dabb; continue; }
  pc = 0x43dab4; continue;
 }
 0x43dab4 => {
  { cpu.pushad(); }
  cpu.push(0x43daba); f_00401c10(cpu); if cpu.returned_to != 0x43daba { return; }
  { cpu.popad(); }
  pc = 0x43dabb; continue;
 }
 0x43dabb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dabc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dabc); }
 let mut pc: u32 = 0x43dabc;
 loop { match pc {
 0x43dabc => {
  { let v: u32 = 0x5a02acu32; cpu.esi = v; }
  { let v: u32 = 0x5a0cccu32; cpu.edi = v; }
  { let v: u32 = 0x5a16ecu32; cpu.ebp = v; }
  pc = 0x43dacb; continue;
 }
 0x43dacb => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x43daec; continue; }
  pc = 0x43dad3; continue;
 }
 0x43dad3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  pc = 0x43db59; continue;
 }
 0x43daec => {
  { cpu.lods(2, false); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x64u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x64u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.lods(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.lods(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.lods(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.lods(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.lods(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  pc = 0x43db59; continue;
 }
 0x43db59 => {
  { cpu.sub32(cpu.esi, 0x5a0cccu32); }
  if !cpu.zf { cpu.poll(); pc = 0x43dacb; continue; }
  pc = 0x43db65; continue;
 }
 0x43db65 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043db68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43db68); }
 let mut pc: u32 = 0x43db68;
 loop { match pc {
 0x43db68 => {
  { cpu.sub8(cpu.r8(0x5a23c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x43db72; continue; }
  pc = 0x43db71; continue;
 }
 0x43db71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x43db72 => {
  { cpu.pushad(); }
  cpu.push(0x43db78); f_004030c0(cpu); if cpu.returned_to != 0x43db78 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x56c721u32, v); }
  cpu.push(0x43db83); f_0043dc47(cpu); if cpu.returned_to != 0x43db83 { return; }
  cpu.push(0x43db88); f_0043dc5b(cpu); if cpu.returned_to != 0x43db88 { return; }
  cpu.push(0x43db8d); f_0043dc82(cpu); if cpu.returned_to != 0x43db8d { return; }
  cpu.push(0x43db92); f_0043dd0a(cpu); if cpu.returned_to != 0x43db92 { return; }
  { cpu.pushad(); }
  cpu.push(0x43db98); f_004030c0(cpu); if cpu.returned_to != 0x43db98 { return; }
  { let v: u32 = cpu.r32(0x4d6bfcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56d18du32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43dbaa); f_004030c0(cpu); if cpu.returned_to != 0x43dbaa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x5622u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.edx = v; }
  { let v: u16 = 0x2bu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x41c00eu32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43dbc8); f_00402fb0(cpu); if cpu.returned_to != 0x43dbc8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5a23fcu32; cpu.eax = v; }
  { let v: u32 = 0x4650u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x579945u32)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x43dbf0; continue; }
  pc = 0x43dbeb; continue;
 }
 0x43dbeb => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x43dbf0; continue;
 }
 0x43dbf0 => {
  { let v: u32 = cpu.r32(0x575f6eu32); cpu.edx = v; }
  cpu.push(0x43dbfb); f_0041b8fe(cpu); if cpu.returned_to != 0x43dbfb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x43dc02); f_00417e23(cpu); if cpu.returned_to != 0x43dc02 { return; }
  { cpu.pushad(); }
  cpu.push(0x43dc08); f_004010e0(cpu); if cpu.returned_to != 0x43dc08 { return; }
  cpu.push(0x43dc0d); f_004030c0(cpu); if cpu.returned_to != 0x43dc0d { return; }
  { let v: u32 = cpu.r32(0x4d6bfcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56d18du32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43dc1f); f_004030c0(cpu); if cpu.returned_to != 0x43dc1f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x2b11u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.edx = v; }
  { let v: u16 = 0x2bu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x41c00eu32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x43dc3d); f_00402fb0(cpu); if cpu.returned_to != 0x43dc3d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x43dc46); f_00415f1b(cpu); if cpu.returned_to != 0x43dc46 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dc47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dc47); }
 let mut pc: u32 = 0x43dc47;
 loop { match pc {
 0x43dc47 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5a23fcu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5a23c0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5a23c0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dc5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dc5b); }
 let mut pc: u32 = 0x43dc5b;
 loop { match pc {
 0x43dc5b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5a23c0u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x56d314u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x576be8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x5fu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x5a23c0u32), 0x2u32); cpu.w32(0x5a23c0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dc82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dc82); }
 let mut pc: u32 = 0x43dc82;
 loop { match pc {
 0x43dc82 => {
  { let v: u8 = 0x3u8; cpu.w8(0x579183u32, v); }
  { cpu.pushad(); }
  pc = 0x43dc8a; continue;
 }
 0x43dc8a => {
  { let v: u32 = cpu.r32(0x5a243cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x43dca8; continue; }
  pc = 0x43dc9c; continue;
 }
 0x43dc9c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x579183u32)); cpu.w8(0x579183u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x43dc8a; continue; }
  pc = 0x43dca8; continue;
 }
 0x43dca8 => {
  { let v: u32 = cpu.r32(0x5a23c0u32); cpu.edi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x43dcdb; continue; }
  pc = 0x43dcbc; continue;
 }
 0x43dcbc => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x43dce8; continue; }
  pc = 0x43dcc5; continue;
 }
 0x43dcc5 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x43dcf0; continue; }
  pc = 0x43dcce; continue;
 }
 0x43dcce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x579945u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x43dcf6; continue;
 }
 0x43dcdb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x579945u32, v); }
  pc = 0x43dcf6; continue;
 }
 0x43dce8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x43dcf6; continue;
 }
 0x43dcf0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x43dcf6; continue;
 }
 0x43dcf6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x43dd02; continue; }
  pc = 0x43dcfc; continue;
 }
 0x43dcfc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43dcf6; continue; }
  pc = 0x43dd02; continue;
 }
 0x43dd02 => {
  { let v: u32 = cpu.edi; cpu.w32(0x5a23c0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dd0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dd0a); }
 let mut pc: u32 = 0x43dd0a;
 loop { match pc {
 0x43dd0a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5a23c0u32); cpu.edi = v; }
  { let v: u32 = 0x5a23f7u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x43dd1b; continue;
 }
 0x43dd1b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x43dd1b; continue; }
  pc = 0x43dd23; continue;
 }
 0x43dd23 => {
  { let v: u32 = cpu.edi; cpu.w32(0x5a23c0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dd2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dd2c); }
 let mut pc: u32 = 0x43dd2c;
 loop { match pc {
 0x43dd2c => {
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x43dd32); cpu.call(t); if cpu.returned_to != 0x43dd32 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x43dd51; continue;
 }
 0x43dd51 => {
  { cpu.pushad(); }
  cpu.push(0x43dd57); f_0043ddfc(cpu); if cpu.returned_to != 0x43dd57 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x43dd6c; continue; }
  pc = 0x43dd5c; continue;
 }
 0x43dd5c => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43dd7f; continue;
 }
 0x43dd6c => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x43dd7f; continue;
 }
 0x43dd7f => {
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43dd51; continue; }
  pc = 0x43dd86; continue;
 }
 0x43dd86 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dd87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dd87); }
 let mut pc: u32 = 0x43dd87;
 loop { match pc {
 0x43dd87 => {
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x43dd8d); cpu.call(t); if cpu.returned_to != 0x43dd8d { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x4e15c5u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43ddb2; continue;
 }
 0x43ddb2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x43ddc4; continue; }
  pc = 0x43ddb7; continue;
 }
 0x43ddb7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x43ddbf); f_0043ddfc(cpu); if cpu.returned_to != 0x43ddbf { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x43ddc4; continue;
 }
 0x43ddc4 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x43ddd8; continue; }
  pc = 0x43ddc8; continue;
 }
 0x43ddc8 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43ddeb; continue;
 }
 0x43ddd8 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x43ddeb; continue;
 }
 0x43ddeb => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x43ddf2; continue; }
  pc = 0x43ddf0; continue;
 }
 0x43ddf0 => {
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x43ddf2; continue;
 }
 0x43ddf2 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x43ddb2; continue; }
  pc = 0x43ddfb; continue;
 }
 0x43ddfb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043ddfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43ddfc); }
 let mut pc: u32 = 0x43ddfc;
 loop { match pc {
 0x43ddfc => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de45); cpu.call(t); if cpu.returned_to != 0x43de45 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de55); cpu.call(t); if cpu.returned_to != 0x43de55 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de65); cpu.call(t); if cpu.returned_to != 0x43de65 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de75); cpu.call(t); if cpu.returned_to != 0x43de75 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = (cpu.edi as u16); let b = (cpu.esi as u16); { let v: u16 = b; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de88); cpu.call(t); if cpu.returned_to != 0x43de88 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43de98); cpu.call(t); if cpu.returned_to != 0x43de98 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43dea8); cpu.call(t); if cpu.returned_to != 0x43dea8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6fb0u32); cpu.push(0x43deb8); cpu.call(t); if cpu.returned_to != 0x43deb8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dec9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dec9); }
 let mut pc: u32 = 0x43dec9;
 loop { match pc {
 0x43dec9 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x43def7; continue; }
  pc = 0x43ded2; continue;
 }
 0x43ded2 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if cpu.sf { pc = 0x43def7; continue; }
  pc = 0x43dedb; continue;
 }
 0x43dedb => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43def7; continue; }
  pc = 0x43dee4; continue;
 }
 0x43dee4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43def7; continue; }
  pc = 0x43deed; continue;
 }
 0x43deed => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x4d6f64u32); cpu.push(0x43def5); cpu.call(t); if cpu.returned_to != 0x43def5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x43def7; continue;
 }
 0x43def7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043def8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43def8); }
 let mut pc: u32 = 0x43def8;
 loop { match pc {
 0x43def8 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x43df24; continue; }
  pc = 0x43df01; continue;
 }
 0x43df01 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if cpu.sf { pc = 0x43df24; continue; }
  pc = 0x43df0a; continue;
 }
 0x43df0a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43df24; continue; }
  pc = 0x43df13; continue;
 }
 0x43df13 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43df24; continue; }
  pc = 0x43df1c; continue;
 }
 0x43df1c => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f64u32); cpu.push(0x43df23); cpu.call(t); if cpu.returned_to != 0x43df23 { return; }
  { cpu.popad(); }
  pc = 0x43df24; continue;
 }
 0x43df24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043df25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43df25); }
 let mut pc: u32 = 0x43df25;
 loop { match pc {
 0x43df25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043df26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43df26); }
 let mut pc: u32 = 0x43df26;
 loop { match pc {
 0x43df26 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df3a; continue;
 }
 0x43df3a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df43; continue;
 }
 0x43df43 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df4c; continue;
 }
 0x43df4c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df55; continue;
 }
 0x43df55 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x43df64; continue; }
  pc = 0x43df5e; continue;
 }
 0x43df5e => {
  { let v: u16 = cpu.r16(0x4d6b9cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43df64; continue;
 }
 0x43df64 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43df74; continue; }
  pc = 0x43df6d; continue;
 }
 0x43df6d => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x43df74; continue;
 }
 0x43df74 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x43df84; continue; }
  pc = 0x43df7d; continue;
 }
 0x43df7d => {
  { let v: u16 = cpu.r16(0x4d6ba0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x43df84; continue;
 }
 0x43df84 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43df94; continue; }
  pc = 0x43df8d; continue;
 }
 0x43df8d => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x43df94; continue;
 }
 0x43df94 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df99; continue;
 }
 0x43df99 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x43dfa4; continue; }
  pc = 0x43df9e; continue;
 }
 0x43df9e => {
  let t = cpu.r32(0x4d6f54u32); cpu.push(0x43dfa4); cpu.call(t); if cpu.returned_to != 0x43dfa4 { return; }
  pc = 0x43dfa4; continue;
 }
 0x43dfa4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dfa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dfa9); }
 let mut pc: u32 = 0x43dfa9;
 loop { match pc {
 0x43dfa9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dfaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dfaa); }
 let mut pc: u32 = 0x43dfaa;
 loop { match pc {
 0x43dfaa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dfab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43dfab); }
 let mut pc: u32 = 0x43dfab;
 loop { match pc {
 0x43dfab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043dfac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43dfac); } r_0043dfac(cpu, 0x43dfac); }
pub fn f_0043e000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x43e000); } r_0043dfac(cpu, 0x43e000); }
fn r_0043dfac(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x43dfac => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d67b8u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43dfc4; continue; }
  pc = 0x43dfbf; continue;
 }
 0x43dfbf => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x43dfc4; continue;
 }
 0x43dfc4 => {
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub((cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ebx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e003; continue; }
  pc = 0x43dfd9; continue;
 }
 0x43dfd9 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6b9cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e0ad; continue; }
  pc = 0x43dfe6; continue;
 }
 0x43dfe6 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6b9cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.edi as u16); cpu.imul16(v); }
  { let v = (cpu.esi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6b9cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x43e003; continue;
 }
 0x43e000 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x66u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e03c; continue; }
  pc = 0x43e00c; continue;
 }
 0x43e003 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e03c; continue; }
  pc = 0x43e00c; continue;
 }
 0x43e00c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e0ad; continue; }
  pc = 0x43e019; continue;
 }
 0x43e019 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.edi as u16); cpu.imul16(v); }
  { let v = (cpu.esi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x43e03c; continue;
 }
 0x43e03c => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x43e046; continue; }
  pc = 0x43e041; continue;
 }
 0x43e041 => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x43e046; continue;
 }
 0x43e046 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e075; continue; }
  pc = 0x43e04f; continue;
 }
 0x43e04f => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e0ad; continue; }
  pc = 0x43e058; continue;
 }
 0x43e058 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6ba0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.imul16(v); }
  { let v = (cpu.edi as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6ba0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x43e075; continue;
 }
 0x43e075 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x43e0a7; continue; }
  pc = 0x43e07e; continue;
 }
 0x43e07e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x43e0ad; continue; }
  pc = 0x43e087; continue;
 }
 0x43e087 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.imul16(v); }
  { let v = (cpu.edi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x43e0a7; continue;
 }
 0x43e0a7 => {
  let t = cpu.r32(0x4d6f90u32); cpu.push(0x43e0ad); cpu.call(t); if cpu.returned_to != 0x43e0ad { return; }
  pc = 0x43e0ad; continue;
 }
 0x43e0ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0043e136(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x43e136); }
 let mut pc: u32 = 0x43e136;
 loop { match pc {
 0x43e136 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
