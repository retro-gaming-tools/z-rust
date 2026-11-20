#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00447aed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447aed); }
 let mut pc: u32 = 0x447aed;
 loop { match pc {
 0x447aed => {
  { let v = cpu.r16(0x4d6758u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d688au32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x447bb4; continue; }
  pc = 0x447b08; continue;
 }
 0x447b08 => {
  { cpu.sub32(cpu.r32(0x4d6c10u32), 0x0u32); }
  if cpu.zf { pc = 0x447b33; continue; }
  pc = 0x447b11; continue;
 }
 0x447b11 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6758u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d688au32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x447b33; continue;
 }
 0x447b33 => {
  { cpu.pushad(); }
  cpu.push(0x447b39); f_004475e6(cpu); if cpu.returned_to != 0x447b39 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1f60u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (cpu.sf != cpu.of) { pc = 0x447b46; continue; }
  pc = 0x447b44; continue;
 }
 0x447b44 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x447b46; continue;
 }
 0x447b46 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4e6du32, v); }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x447bb4; continue; }
  pc = 0x447b56; continue;
 }
 0x447b56 => {
  { cpu.pushad(); }
  cpu.push(0x447b5c); f_004184ca(cpu); if cpu.returned_to != 0x447b5c { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = cpu.r8(0x4eab29u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5c4e71u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x447b7f; continue; }
  pc = 0x447b75; continue;
 }
 0x447b75 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5c4e71u32, v); }
  pc = 0x447b7f; continue;
 }
 0x447b7f => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x447ba9; continue; }
  pc = 0x447b86; continue;
 }
 0x447b86 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c08u32, v); }
  { cpu.popad(); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d688au32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6758u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447ba9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c08u32, v); }
  { cpu.popad(); }
  pc = 0x447bb4; continue;
 }
 0x447bb4 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d688au32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6758u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447bc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447bc3); }
 let mut pc: u32 = 0x447bc3;
 loop { match pc {
 0x447bc3 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x447c21; continue; }
  pc = 0x447bcc; continue;
 }
 0x447bcc => {
  { let v: u32 = cpu.r32(0x4d6c08u32); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x447c21; continue; }
  pc = 0x447bd6; continue;
 }
 0x447bd6 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x447c21; continue; }
  pc = 0x447beb; continue;
 }
 0x447beb => {
  cpu.push(0x447bf0); f_0045dccf(cpu); if cpu.returned_to != 0x447bf0 { return; }
  cpu.push(0x447bf5); f_00447f84(cpu); if cpu.returned_to != 0x447bf5 { return; }
  if cpu.cf { pc = 0x447c20; continue; }
  pc = 0x447bf7; continue;
 }
 0x447bf7 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x447c20; continue; }
  pc = 0x447bfe; continue;
 }
 0x447bfe => {
  { cpu.sub16(cpu.r16(0x56c6eeu32), 0x0u16); }
  if !cpu.zf { pc = 0x447c20; continue; }
  pc = 0x447c08; continue;
 }
 0x447c08 => {
  cpu.push(0x447c0d); f_0041c6a6(cpu); if cpu.returned_to != 0x447c0d { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x1fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x447c20; continue; }
  pc = 0x447c11; continue;
 }
 0x447c11 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x63u32; cpu.ebx = v; }
  cpu.push(0x447c20); f_00447c27(cpu); if cpu.returned_to != 0x447c20 { return; }
  pc = 0x447c20; continue;
 }
 0x447c20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447c21 => {
  cpu.push(0x447c26); f_00448036(cpu); if cpu.returned_to != 0x447c26 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447c27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447c27); }
 let mut pc: u32 = 0x447c27;
 loop { match pc {
 0x447c27 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x447c6c; continue; }
  pc = 0x447c31; continue;
 }
 0x447c31 => {
  { cpu.sub32(cpu.r32(0x5c4e34u32), 0x0u32); }
  if cpu.zf { pc = 0x447c6e; continue; }
  pc = 0x447c3a; continue;
 }
 0x447c3a => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x0u32); }
  if !cpu.zf { pc = 0x447c76; continue; }
  pc = 0x447c43; continue;
 }
 0x447c43 => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x447c6c; continue; }
  pc = 0x447c48; continue;
 }
 0x447c48 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c00u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c04u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x447c5f); f_00447aed(cpu); if cpu.returned_to != 0x447c5f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c58u32, v); }
  cpu.push(0x447c6c); f_0045d4b4(cpu); if cpu.returned_to != 0x447c6c { return; }
  pc = 0x447c6c; continue;
 }
 0x447c6c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447c6e => {
  { cpu.pushad(); }
  cpu.push(0x447c74); f_0045d4b4(cpu); if cpu.returned_to != 0x447c74 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447c76 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x447cb1; continue; }
  pc = 0x447c7b; continue;
 }
 0x447c7b => {
  { cpu.sub8(cpu.r8(0x5a48f1u32), 0x0u8); }
  if !cpu.zf { pc = 0x447cb1; continue; }
  pc = 0x447c84; continue;
 }
 0x447c84 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x4u32); }
  if !cpu.zf { pc = 0x447cb1; continue; }
  pc = 0x447c8d; continue;
 }
 0x447c8d => {
  { let v: u32 = cpu.r32(0x4d6c58u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x400u32); }
  if cpu.zf { pc = 0x447d3d; continue; }
  pc = 0x447ca5; continue;
 }
 0x447ca5 => {
  { cpu.sub32(cpu.edx, 0x500u32); }
  if cpu.zf { pc = 0x447d3d; continue; }
  pc = 0x447cb1; continue;
 }
 0x447cb1 => {
  { cpu.sub32(cpu.r32(0x4d6c00u32), 0x0u32); }
  if cpu.zf { pc = 0x447cc3; continue; }
  pc = 0x447cba; continue;
 }
 0x447cba => {
  { cpu.sub32(cpu.r32(0x4d6c00u32), 0x2u32); }
  if cpu.zf { pc = 0x447cc3; continue; }
  pc = 0x447cc3; continue;
 }
 0x447cc3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a48f1u32, v); }
  cpu.push(0x447cd1); f_00447aed(cpu); if cpu.returned_to != 0x447cd1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c58u32, v); }
  cpu.push(0x447ce0); f_0045d4b4(cpu); if cpu.returned_to != 0x447ce0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x447c6c; continue; }
  pc = 0x447ceb; continue;
 }
 0x447ceb => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x100u32); }
  if cpu.zf { pc = 0x447d3f; continue; }
  pc = 0x447cfb; continue;
 }
 0x447cfb => {
  { cpu.sub32(cpu.edx, 0x400u32); }
  if cpu.zf { pc = 0x447d3f; continue; }
  pc = 0x447d03; continue;
 }
 0x447d03 => {
  { cpu.sub32(cpu.edx, 0x500u32); }
  if cpu.zf { pc = 0x447d3f; continue; }
  pc = 0x447d0b; continue;
 }
 0x447d0b => {
  { cpu.sub32(cpu.edx, 0xa00u32); }
  if cpu.zf { pc = 0x447d3f; continue; }
  pc = 0x447d13; continue;
 }
 0x447d13 => {
  { cpu.sub32(cpu.edx, 0xb00u32); }
  if cpu.zf { pc = 0x447d3f; continue; }
  pc = 0x447d1b; continue;
 }
 0x447d1b => {
  pc = 0x447d3f; continue;
 }
 0x447d3d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447d3f => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c00u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c04u32, v); }
  cpu.poll(); pc = 0x447d3d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447d51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447d51); }
 let mut pc: u32 = 0x447d51;
 loop { match pc {
 0x447d51 => {
  { cpu.sub32(cpu.r32(0x4d6c00u32), 0x0u32); }
  if cpu.zf { pc = 0x447d97; continue; }
  pc = 0x447d5a; continue;
 }
 0x447d5a => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x3u32); }
  if !cpu.zf { pc = 0x447d96; continue; }
  pc = 0x447d63; continue;
 }
 0x447d63 => {
  { cpu.sub32(cpu.r32(0x4d6c00u32), 0x2u32); }
  if !cpu.zf { pc = 0x447d76; continue; }
  pc = 0x447d6c; continue;
 }
 0x447d6c => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  pc = 0x447d76; continue;
 }
 0x447d76 => {
  { let v: u32 = cpu.r32(0x4d6c00u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6c04u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c58u32, v); }
  cpu.push(0x447d8c); f_0045d4b4(cpu); if cpu.returned_to != 0x447d8c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c00u32, v); }
  pc = 0x447d96; continue;
 }
 0x447d96 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447d97 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447da0; continue;
 }
 0x447da0 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447da9; continue;
 }
 0x447da9 => {
  cpu.push(0x447dae); f_00447f84(cpu); if cpu.returned_to != 0x447dae { return; }
  if cpu.cf { pc = 0x447ea8; continue; }
  pc = 0x447db4; continue;
 }
 0x447db4 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x447ea8; continue; }
  pc = 0x447dbf; continue;
 }
 0x447dbf => {
  { cpu.sub16(cpu.r16(0x56c6eeu32), 0x0u16); }
  if !cpu.zf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447dc9; continue;
 }
 0x447dc9 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x447dec; continue; }
  pc = 0x447dd0; continue;
 }
 0x447dd0 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x447e02; continue; }
  pc = 0x447dda; continue;
 }
 0x447dda => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x447de0); f_004184ca(cpu); if cpu.returned_to != 0x447de0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { pc = 0x447dec; continue; }
  pc = 0x447de5; continue;
 }
 0x447de5 => {
  {  }
  if !cpu.zf { pc = 0x447ea8; continue; }
  pc = 0x447dec; continue;
 }
 0x447dec => {
  cpu.push(0x447df1); f_00447f84(cpu); if cpu.returned_to != 0x447df1 { return; }
  if cpu.cf { pc = 0x447ea8; continue; }
  pc = 0x447df7; continue;
 }
 0x447df7 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x447ea8; continue; }
  pc = 0x447e02; continue;
 }
 0x447e02 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x0u32); }
  if cpu.zf { pc = 0x447e59; continue; }
  pc = 0x447e0b; continue;
 }
 0x447e0b => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x2u32); }
  if cpu.zf { pc = 0x447e59; continue; }
  pc = 0x447e14; continue;
 }
 0x447e14 => {
  { cpu.sub32(cpu.r32(0x5c4e34u32), 0x0u32); }
  if cpu.zf { pc = 0x447e61; continue; }
  pc = 0x447e1d; continue;
 }
 0x447e1d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x447e61; continue; }
  pc = 0x447e24; continue;
 }
 0x447e24 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x447e61; continue; }
  pc = 0x447e2b; continue;
 }
 0x447e2b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x447e39; continue; }
  pc = 0x447e32; continue;
 }
 0x447e32 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x447e59; continue; }
  pc = 0x447e39; continue;
 }
 0x447e39 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x447e47; continue; }
  pc = 0x447e42; continue;
 }
 0x447e42 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x447e4c; continue; }
  pc = 0x447e47; continue;
 }
 0x447e47 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x447e4c; continue;
 }
 0x447e4c => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x447e59; continue; }
  pc = 0x447e52; continue;
 }
 0x447e52 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x447e61; continue; }
  pc = 0x447e59; continue;
 }
 0x447e59 => {
  { let v: u8 = 0x33u8; cpu.w8(0x5a48f2u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447e61 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a48f2u32)); cpu.w8(0x5a48f2u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447e6d; continue;
 }
 0x447e6d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a48f2u32, v); }
  cpu.push(0x447e79); f_0041c6a6(cpu); if cpu.returned_to != 0x447e79 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447e81; continue;
 }
 0x447e81 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5a48b0u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x447d96; continue; }
  pc = 0x447e9b; continue;
 }
 0x447e9b => {
  cpu.push(0x447ea0); f_00447c27(cpu); if cpu.returned_to != 0x447ea0 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5a48f1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447ea8 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x447eeb; continue; }
  pc = 0x447eb2; continue;
 }
 0x447eb2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c08u32, v); }
  cpu.push(0x447ec1); f_00448036(cpu); if cpu.returned_to != 0x447ec1 { return; }
  cpu.push(0x447ec6); f_0041c6a6(cpu); if cpu.returned_to != 0x447ec6 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = cpu.r8(0x5a48b0u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  cpu.push(0x447eea); f_00447c27(cpu); if cpu.returned_to != 0x447eea { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447eeb => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c08u32, v); }
  cpu.push(0x447efa); f_00448036(cpu); if cpu.returned_to != 0x447efa { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c00u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c04u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c58u32, v); }
  cpu.push(0x447f17); f_0045d4b4(cpu); if cpu.returned_to != 0x447f17 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447f22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447f22); }
 let mut pc: u32 = 0x447f22;
 loop { match pc {
 0x447f22 => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if cpu.zf { pc = 0x447f83; continue; }
  pc = 0x447f2b; continue;
 }
 0x447f2b => {
  cpu.push(0x447f30); f_0045d200(cpu); if cpu.returned_to != 0x447f30 { return; }
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x0u32); }
  if cpu.zf { pc = 0x447f7e; continue; }
  pc = 0x447f39; continue;
 }
 0x447f39 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x2u32); }
  if cpu.zf { pc = 0x447f7e; continue; }
  pc = 0x447f42; continue;
 }
 0x447f42 => {
  { cpu.sub32(cpu.r32(0x5c4e3cu32), 0x4u32); }
  if cpu.zf { pc = 0x447f7e; continue; }
  pc = 0x447f4b; continue;
 }
 0x447f4b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5a48f3u32)); cpu.w8(0x5a48f3u32, v); }
  if !cpu.sf { pc = 0x447f7e; continue; }
  pc = 0x447f53; continue;
 }
 0x447f53 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5a48f3u32, v); }
  cpu.push(0x447f5f); f_0041c6a6(cpu); if cpu.returned_to != 0x447f5f { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x447f7e; continue; }
  pc = 0x447f63; continue;
 }
 0x447f63 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5a48b0u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x447f7e; continue; }
  pc = 0x447f79; continue;
 }
 0x447f79 => {
  cpu.push(0x447f7e); f_0045d4b4(cpu); if cpu.returned_to != 0x447f7e { return; }
  pc = 0x447f7e; continue;
 }
 0x447f7e => {
  cpu.push(0x447f83); f_0045d1e7(cpu); if cpu.returned_to != 0x447f83 { return; }
  pc = 0x447f83; continue;
 }
 0x447f83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447f84); }
 let mut pc: u32 = 0x447f84;
 loop { match pc {
 0x447f84 => {
  { let v: u32 = cpu.r32(0x4d6c10u32); cpu.ebp = v; }
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x447fc7; continue; }
  pc = 0x447f8e; continue;
 }
 0x447f8e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x447fc7; continue; }
  pc = 0x447f95; continue;
 }
 0x447f95 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x447fc5; continue; }
  pc = 0x447f9c; continue;
 }
 0x447f9c => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x447fc7; continue; }
  pc = 0x447fb1; continue;
 }
 0x447fb1 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x447fc7; continue; }
  pc = 0x447fb7; continue;
 }
 0x447fb7 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x447fc5; continue;
 }
 0x447fc5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x447fc7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447fd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447fd3); }
 let mut pc: u32 = 0x447fd3;
 loop { match pc {
 0x447fd3 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.edi = v; }
  { let v: u32 = 0xd629u32; cpu.ecx = v; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00447fe3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x447fe3); }
 let mut pc: u32 = 0x447fe3;
 loop { match pc {
 0x447fe3 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  let t = cpu.r32(0x4d6fa8u32); cpu.push(0x447ff7); cpu.call(t); if cpu.returned_to != 0x447ff7 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4ac0ecu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d681cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x448016; continue;
 }
 0x448016 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44802f; continue; }
  pc = 0x44801d; continue;
 }
 0x44801d => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44802f; continue; }
  pc = 0x448021; continue;
 }
 0x448021 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x448031; continue; }
  pc = 0x448025; continue;
 }
 0x448025 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x44802f; continue; }
  pc = 0x448029; continue;
 }
 0x448029 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { cpu.poll(); pc = 0x448016; continue; }
  pc = 0x44802d; continue;
 }
 0x44802d => {
  cpu.poll(); pc = 0x448016; continue;
 }
 0x44802f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448031 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.poll(); pc = 0x448016; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448036(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448036); }
 let mut pc: u32 = 0x448036;
 loop { match pc {
 0x448036 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5c4e57u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5c4e56u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5c4e58u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5c4e59u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5c4e5au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448060); }
 let mut pc: u32 = 0x448060;
 loop { match pc {
 0x448060 => {
  { let v: u32 = 0x0u32; cpu.w32(0x56c967u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x56c963u32, v); }
  { let v: u32 = 0x559754u32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x44807a; continue;
 }
 0x44807a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x4480b7; continue; }
  pc = 0x448081; continue;
 }
 0x448081 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44809d; continue; }
  pc = 0x448091; continue;
 }
 0x448091 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x44809d; continue; }
  pc = 0x448095; continue;
 }
 0x448095 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x44809d; continue; }
  pc = 0x448099; continue;
 }
 0x448099 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x4480b7; continue; }
  pc = 0x44809d; continue;
 }
 0x44809d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4480a9); f_0047a2d9(cpu); if cpu.returned_to != 0x4480a9 { return; }
  { cpu.sub32(cpu.r32(0x56c963u32), cpu.esi); }
  if !cpu.zf { pc = 0x4480b7; continue; }
  pc = 0x4480b1; continue;
 }
 0x4480b1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x56c967u32)); cpu.w32(0x56c967u32, v); }
  pc = 0x4480b7; continue;
 }
 0x4480b7 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44807a; continue; }
  pc = 0x4480bd; continue;
 }
 0x4480bd => {
  { let v: u32 = cpu.r32(0x56c967u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004480c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4480c3); }
 let mut pc: u32 = 0x4480c3;
 loop { match pc {
 0x4480c3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = 0x559754u32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x4480e5; continue;
 }
 0x4480e5 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x448189; continue; }
  pc = 0x4480f0; continue;
 }
 0x4480f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448100; continue;
 }
 0x448100 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448104; continue;
 }
 0x448104 => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448108; continue;
 }
 0x448108 => {
  { cpu.sub8((cpu.eax as u8), 0x1du8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x44810c; continue;
 }
 0x44810c => {
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448110; continue;
 }
 0x448110 => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448114; continue;
 }
 0x448114 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448118; continue;
 }
 0x448118 => {
  { cpu.sub8((cpu.eax as u8), 0x21u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x44811c; continue;
 }
 0x44811c => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448120; continue;
 }
 0x448120 => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x448128; continue; }
  pc = 0x448124; continue;
 }
 0x448124 => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if !cpu.zf { pc = 0x448189; continue; }
  pc = 0x448128; continue;
 }
 0x448128 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6bd8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x448189; continue; }
  pc = 0x448133; continue;
 }
 0x448133 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6be0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x448189; continue; }
  pc = 0x44813b; continue;
 }
 0x44813b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6bdcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x448189; continue; }
  pc = 0x448147; continue;
 }
 0x448147 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6be4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x448189; continue; }
  pc = 0x44814f; continue;
 }
 0x44814f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x44815f); f_00420989(cpu); if cpu.returned_to != 0x44815f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x14u32); cpu.edx = v; }
  cpu.push(0x44816e); f_004689e9(cpu); if cpu.returned_to != 0x44816e { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x448174); f_004689e9(cpu); if cpu.returned_to != 0x448174 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7eu32); cpu.esi = v; }
  cpu.push(0x44817d); f_004689e9(cpu); if cpu.returned_to != 0x44817d { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x448183); f_004689e9(cpu); if cpu.returned_to != 0x448183 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x448189; continue;
 }
 0x448189 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4480e5; continue; }
  pc = 0x448193; continue;
 }
 0x448193 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448195(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448195); }
 let mut pc: u32 = 0x448195;
 loop { match pc {
 0x448195 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = 0x22eu16; cpu.w16(0x4d6960u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  cpu.push(0x4481de); f_004829bb(cpu); if cpu.returned_to != 0x4481de { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004481e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4481e0); }
 let mut pc: u32 = 0x4481e0;
 loop { match pc {
 0x4481e0 => {
  { cpu.pushad(); }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x4481ed; continue;
 }
 0x4481ed => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x44822f; continue; }
  pc = 0x4481f4; continue;
 }
 0x4481f4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x10u8); }
  if cpu.zf { pc = 0x44822f; continue; }
  pc = 0x4481fd; continue;
 }
 0x4481fd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x44822f; continue; }
  pc = 0x448202; continue;
 }
 0x448202 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x44822f; continue; }
  pc = 0x448208; continue;
 }
 0x448208 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44822f; continue; }
  pc = 0x44821d; continue;
 }
 0x44821d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44822f; continue; }
  pc = 0x448226; continue;
 }
 0x448226 => {
  { let v: u32 = cpu.esi; cpu.w32(0x59da8cu32, v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44822f => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4481ed; continue; }
  pc = 0x448235; continue;
 }
 0x448235 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448238); }
 let mut pc: u32 = 0x448238;
 loop { match pc {
 0x448238 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x559756u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x559754u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x448258; continue; }
  pc = 0x448256; continue;
 }
 0x448256 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448258 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044825a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44825a); }
 let mut pc: u32 = 0x44825a;
 loop { match pc {
 0x44825a => {
  { let v: u32 = 0x520cf0u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x448264; continue;
 }
 0x448264 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x44826c; continue; }
  pc = 0x44826a; continue;
 }
 0x44826a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44826c => {
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x448264; continue; }
  pc = 0x448271; continue;
 }
 0x448271 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448273(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448273); }
 let mut pc: u32 = 0x448273;
 loop { match pc {
 0x448273 => {
  { let v: u32 = 0x520cf0u32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x44827d; continue;
 }
 0x44827d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x44831e; continue; }
  pc = 0x448288; continue;
 }
 0x448288 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x448296); cpu.call(t); if cpu.returned_to != 0x448296 { return; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x5a2afcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4482b6; continue; }
  pc = 0x4482b1; continue;
 }
 0x4482b1 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4482b6; continue;
 }
 0x4482b6 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x44831c; continue; }
  pc = 0x4482c3; continue;
 }
 0x4482c3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44831c; continue; }
  pc = 0x4482cc; continue;
 }
 0x4482cc => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x44831c; continue; }
  pc = 0x4482d9; continue;
 }
 0x4482d9 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44831c; continue; }
  pc = 0x4482e2; continue;
 }
 0x4482e2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x4482f5); cpu.call(t); if cpu.returned_to != 0x4482f5 { return; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x13eu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x448309; continue; }
  pc = 0x448304; continue;
 }
 0x448304 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x448309; continue;
 }
 0x448309 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x44831c); cpu.call(t); if cpu.returned_to != 0x44831c { return; }
  pc = 0x44831c; continue;
 }
 0x44831c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44831e; continue;
 }
 0x44831e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44827d; continue; }
  pc = 0x448328; continue;
 }
 0x448328 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448329(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448329); }
 let mut pc: u32 = 0x448329;
 loop { match pc {
 0x448329 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5adcd6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5adcd5u32, v); }
  { cpu.sub16((cpu.ecx as u16), 0x1fu16); }
  if cpu.zf { pc = 0x44834c; continue; }
  pc = 0x44833d; continue;
 }
 0x44833d => {
  { cpu.sub16((cpu.ecx as u16), 0x3au16); }
  if !cpu.zf { pc = 0x448353; continue; }
  pc = 0x448343; continue;
 }
 0x448343 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5adcd5u32, v); }
  pc = 0x448353; continue;
 }
 0x44834c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5adcd5u32, v); }
  pc = 0x448353; continue;
 }
 0x448353 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x44835c); f_00448783(cpu); if cpu.returned_to != 0x44835c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x448435; continue; }
  pc = 0x448366; continue;
 }
 0x448366 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x448383); f_00448500(cpu); if cpu.returned_to != 0x448383 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x448389); f_00448436(cpu); if cpu.returned_to != 0x448389 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x448397; continue; }
  pc = 0x448393; continue;
 }
 0x448393 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x448397; continue;
 }
 0x448397 => {
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4483e4; continue; }
  pc = 0x44839e; continue;
 }
 0x44839e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4483b0; continue; }
  pc = 0x4483ac; continue;
 }
 0x4483ac => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4483b0; continue;
 }
 0x4483b0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x4483c2); f_00420989(cpu); if cpu.returned_to != 0x4483c2 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x448409; continue;
 }
 0x4483e4 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x448435; continue; }
  pc = 0x4483ee; continue;
 }
 0x4483ee => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4483fa); f_00420989(cpu); if cpu.returned_to != 0x4483fa { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x448409; continue;
 }
 0x448409 => {
  { cpu.sub16(cpu.r16(0x5adcd6u32), 0x47u16); }
  if (cpu.sf != cpu.of) { pc = 0x448421; continue; }
  pc = 0x448414; continue;
 }
 0x448414 => {
  { cpu.sub16(cpu.r16(0x5adcd6u32), 0x4du16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x448421; continue; }
  pc = 0x44841f; continue;
 }
 0x44841f => {
  pc = 0x448423; continue;
 }
 0x448421 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x448423; continue;
 }
 0x448423 => {
  { cpu.sub8(cpu.r8(0x5adbecu32), 0x1u8); }
  if !cpu.zf { pc = 0x448435; continue; }
  pc = 0x44842c; continue;
 }
 0x44842c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x179bu16); }
  if !cpu.zf { pc = 0x448435; continue; }
  pc = 0x448435; continue;
 }
 0x448435 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448436(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448436); }
 let mut pc: u32 = 0x448436;
 loop { match pc {
 0x448436 => {
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x448496; continue; }
  pc = 0x44843b; continue;
 }
 0x44843b => {
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.esi = v; }
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
 0x448496 => {
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
pub fn f_00448500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448500); }
 let mut pc: u32 = 0x448500;
 loop { match pc {
 0x448500 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x448512; continue; }
  pc = 0x448504; continue;
 }
 0x448504 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) & 0xfff0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) & 0xfff0u16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub(0xeu16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x448512; continue;
 }
 0x448512 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4b839cu32); cpu.eax = v; }
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
  cpu.push(0x448554); f_004486e8(cpu); if cpu.returned_to != 0x448554 { return; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5u32); }
  if cpu.zf { pc = 0x44861e; continue; }
  pc = 0x448563; continue;
 }
 0x448563 => {
  { cpu.sub32(cpu.esi, 0x15u32); }
  if cpu.zf { pc = 0x44861e; continue; }
  pc = 0x44856c; continue;
 }
 0x44856c => {
  { cpu.sub32(cpu.esi, 0x16u32); }
  if cpu.zf { pc = 0x44861e; continue; }
  pc = 0x448575; continue;
 }
 0x448575 => {
  { cpu.sub32(cpu.esi, 0x17u32); }
  if cpu.zf { pc = 0x44861e; continue; }
  pc = 0x44857e; continue;
 }
 0x44857e => {
  { cpu.sub32(cpu.esi, 0x8u32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x448587; continue;
 }
 0x448587 => {
  { cpu.sub32(cpu.esi, 0x1fu32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x448590; continue;
 }
 0x448590 => {
  { cpu.sub32(cpu.esi, 0x18u32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x448599; continue;
 }
 0x448599 => {
  { cpu.sub32(cpu.esi, 0x21u32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x4485a2; continue;
 }
 0x4485a2 => {
  { cpu.sub32(cpu.esi, 0x25u32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x4485ab; continue;
 }
 0x4485ab => {
  { cpu.sub32(cpu.esi, 0x23u32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x4485b4; continue;
 }
 0x4485b4 => {
  { cpu.sub32(cpu.esi, 0x1au32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x4485bd; continue;
 }
 0x4485bd => {
  { cpu.sub32(cpu.esi, 0x1du32); }
  if cpu.zf { pc = 0x448677; continue; }
  pc = 0x4485c6; continue;
 }
 0x4485c6 => {
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
  if cpu.zf { pc = 0x448610; continue; }
  pc = 0x4485f9; continue;
 }
 0x4485f9 => {
  { cpu.sub16((cpu.esi as u16), 0xcu16); }
  if cpu.zf { pc = 0x448610; continue; }
  pc = 0x4485ff; continue;
 }
 0x4485ff => {
  { cpu.sub16((cpu.esi as u16), 0x11u16); }
  if cpu.zf { pc = 0x448610; continue; }
  pc = 0x448605; continue;
 }
 0x448605 => {
  { cpu.sub32(cpu.esi, 0x9u32); }
  if cpu.zf { pc = 0x448619; continue; }
  pc = 0x44860a; continue;
 }
 0x44860a => {
  { cpu.sub32(cpu.esi, 0xau32); }
  if cpu.zf { pc = 0x448619; continue; }
  pc = 0x44860f; continue;
 }
 0x44860f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448610 => {
  { let v: u8 = cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edi)), 0x7fu8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448619 => {
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44861e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d69beu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69c2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69c4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69c6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
 0x448677 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x30u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d69cau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69d0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69d2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d69d4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edi)) | 0x40u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x5adcd5u32), 0x1u8); }
  if !cpu.zf { pc = 0x4486d2; continue; }
  pc = 0x4486cc; continue;
 }
 0x4486cc => {
  { let v: u8 = 0x1eu8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x4486df; continue;
 }
 0x4486d2 => {
  { cpu.sub8(cpu.r8(0x5adcd5u32), 0x2u8); }
  if !cpu.zf { pc = 0x4486df; continue; }
  pc = 0x4486db; continue;
 }
 0x4486db => {
  { let v: u8 = 0x6u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x4486df; continue;
 }
 0x4486df => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004486e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4486e8); }
 let mut pc: u32 = 0x4486e8;
 loop { match pc {
 0x4486e8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  cpu.push(0x4486f5); f_0047a31a(cpu); if cpu.returned_to != 0x4486f5 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x4486f9; continue; }
  pc = 0x4486f8; continue;
 }
 0x4486f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4486f9 => {
  { let v: u8 = cpu.or8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448783(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448783); }
 let mut pc: u32 = 0x448783;
 loop { match pc {
 0x448783 => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x44878d; continue;
 }
 0x44878d => {
  { cpu.lods(2, false); }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x44879b; continue; }
  pc = 0x448794; continue;
 }
 0x448794 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x26u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44878d; continue; }
  pc = 0x448799; continue;
 }
 0x448799 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44879b => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004487c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4487c3); }
 let mut pc: u32 = 0x4487c3;
 loop { match pc {
 0x4487c3 => {
  { let v: u32 = 0x559754u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x4487d2; continue;
 }
 0x4487d2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4487d2; continue; }
  pc = 0x4487d9; continue;
 }
 0x4487d9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004487da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4487da); }
 let mut pc: u32 = 0x4487da;
 loop { match pc {
 0x4487da => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4b86a3u32); }
  if !cpu.zf { pc = 0x4487ed; continue; }
  pc = 0x4487e3; continue;
 }
 0x4487e3 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4487ed; continue; }
  pc = 0x4487e9; continue;
 }
 0x4487e9 => {
  { let v: u8 = 0xffu8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x4487ed; continue;
 }
 0x4487ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004487ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4487ee); }
 let mut pc: u32 = 0x4487ee;
 loop { match pc {
 0x4487ee => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4487f7; continue; }
  pc = 0x4487f7; continue;
 }
 0x4487f7 => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x448807; continue;
 }
 0x448807 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x448817; continue; }
  pc = 0x44880e; continue;
 }
 0x44880e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x448815); f_00448823(cpu); if cpu.returned_to != 0x448815 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x448817; continue;
 }
 0x448817 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x448807; continue; }
  pc = 0x44881c; continue;
 }
 0x44881c => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448823(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448823); }
 let mut pc: u32 = 0x448823;
 loop { match pc {
 0x448823 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x1fu32.wrapping_add(cpu.esi))); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x44882d; continue; }
  pc = 0x44882a; continue;
 }
 0x44882a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  pc = 0x44882d; continue;
 }
 0x44882d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1472u32); }
  if cpu.zf { pc = 0x44899e; continue; }
  pc = 0x448842; continue;
 }
 0x448842 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x448863; continue; }
  pc = 0x44884b; continue;
 }
 0x44884b => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x448863; continue; }
  pc = 0x448853; continue;
 }
 0x448853 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x44886f; continue; }
  pc = 0x448863; continue;
 }
 0x448863 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x44899e; continue;
 }
 0x44886f => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x4488c2; continue; }
  pc = 0x448878; continue;
 }
 0x448878 => {
  { cpu.sub32(cpu.eax, 0x22eu32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x44887f; continue;
 }
 0x44887f => {
  { cpu.sub32(cpu.eax, 0x1d6u32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x448886; continue;
 }
 0x448886 => {
  { cpu.sub32(cpu.eax, 0x16dfu32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x44888d; continue;
 }
 0x44888d => {
  { cpu.sub32(cpu.eax, 0x16e3u32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x448894; continue;
 }
 0x448894 => {
  { cpu.sub32(cpu.eax, 0x16e7u32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x44889b; continue;
 }
 0x44889b => {
  { cpu.sub32(cpu.eax, 0x16ebu32); }
  if cpu.zf { pc = 0x4488a9; continue; }
  pc = 0x4488a2; continue;
 }
 0x4488a2 => {
  { cpu.sub32(cpu.eax, 0x16f1u32); }
  if !cpu.zf { pc = 0x4488b0; continue; }
  pc = 0x4488a9; continue;
 }
 0x4488a9 => {
  cpu.push(0x4488ae); f_00448c0d(cpu); if cpu.returned_to != 0x4488ae { return; }
  pc = 0x4488c2; continue;
 }
 0x4488b0 => {
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x4488be; continue; }
  pc = 0x4488b8; continue;
 }
 0x4488b8 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x4488c2; continue; }
  pc = 0x4488be; continue;
 }
 0x4488be => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x4488c2; continue;
 }
 0x4488c2 => {
  cpu.push(0x4488c7); f_00448d35(cpu); if cpu.returned_to != 0x4488c7 { return; }
  if cpu.cf { pc = 0x44899e; continue; }
  pc = 0x4488cd; continue;
 }
 0x4488cd => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffe0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44899e; continue; }
  pc = 0x4488df; continue;
 }
 0x4488df => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x44899e; continue; }
  pc = 0x4488eb; continue;
 }
 0x4488eb => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffe0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44899e; continue; }
  pc = 0x448900; continue;
 }
 0x448900 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x44899e; continue; }
  pc = 0x44890c; continue;
 }
 0x44890c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x448953; continue; }
  pc = 0x44891f; continue;
 }
 0x44891f => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xb89u16); }
  if cpu.zf { pc = 0x448948; continue; }
  pc = 0x44892c; continue;
 }
 0x44892c => {
  { cpu.sub16((cpu.ecx as u16), 0xb87u16); }
  if cpu.zf { pc = 0x448948; continue; }
  pc = 0x448933; continue;
 }
 0x448933 => {
  { cpu.sub16((cpu.ecx as u16), 0xb88u16); }
  if cpu.zf { pc = 0x448948; continue; }
  pc = 0x44893a; continue;
 }
 0x44893a => {
  { cpu.sub16((cpu.ecx as u16), 0xb8au16); }
  if cpu.zf { pc = 0x448948; continue; }
  pc = 0x448941; continue;
 }
 0x448941 => {
  { cpu.sub16((cpu.ecx as u16), 0xb8bu16); }
  if !cpu.zf { pc = 0x44894c; continue; }
  pc = 0x448948; continue;
 }
 0x448948 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44894c; continue;
 }
 0x44894c => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x448952); cpu.call(t); if cpu.returned_to != 0x448952 { return; }
  { cpu.popad(); }
  pc = 0x448953; continue;
 }
 0x448953 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x21u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1e1u16); }
  if cpu.zf { pc = 0x44899e; continue; }
  pc = 0x448969; continue;
 }
 0x448969 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x5adbecu32), 0x1u8); }
  if !cpu.zf { pc = 0x448983; continue; }
  pc = 0x448976; continue;
 }
 0x448976 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x179bu16); }
  if cpu.zf { pc = 0x448a09; continue; }
  pc = 0x448983; continue;
 }
 0x448983 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x44899f; continue; }
  pc = 0x448989; continue;
 }
 0x448989 => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x44899e; continue;
 }
 0x44899e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44899f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ebx as u16), 0x1686u16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489b8; continue;
 }
 0x4489b8 => {
  { cpu.sub16((cpu.ebx as u16), 0x164au16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489c3; continue;
 }
 0x4489c3 => {
  { cpu.sub16((cpu.ebx as u16), 0x201u16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489ce; continue;
 }
 0x4489ce => {
  { cpu.sub16((cpu.ebx as u16), 0x14d8u16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489d9; continue;
 }
 0x4489d9 => {
  { cpu.sub16((cpu.ebx as u16), 0x1652u16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489e4; continue;
 }
 0x4489e4 => {
  { cpu.sub16((cpu.ebx as u16), 0x164bu16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489ef; continue;
 }
 0x4489ef => {
  { cpu.sub16((cpu.ebx as u16), 0x1653u16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x4489fa; continue;
 }
 0x4489fa => {
  { cpu.sub16((cpu.ebx as u16), 0x164fu16); }
  if cpu.zf { pc = 0x448abc; continue; }
  pc = 0x448a05; continue;
 }
 0x448a05 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448a09 => {
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
  { let v: u16 = 0x125u16; cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x448a59); f_0047a2d9(cpu); if cpu.returned_to != 0x448a59 { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6bd8u32, v); }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x448a72; continue; }
  pc = 0x448a70; continue;
 }
 0x448a70 => {
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x448a72; continue;
 }
 0x448a72 => {
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
  { let v: u32 = cpu.r32(0x576e72u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5884a2u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x126u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x18u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448abc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x164au16); }
  if !cpu.zf { pc = 0x448adb; continue; }
  pc = 0x448ad8; continue;
 }
 0x448ad8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x448adb; continue;
 }
 0x448adb => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x701u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xe900u16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1569u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x713u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xff00u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5adc0fu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x4d677eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x705u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x4d677eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x705u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xfe00u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x4d677eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x705u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.pushad(); }
  cpu.push(0x448bde); f_0041c6a6(cpu); if cpu.returned_to != 0x448bde { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x448c05; continue; }
  pc = 0x448be3; continue;
 }
 0x448be3 => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  cpu.push(0x448c05); f_004862bf(cpu); if cpu.returned_to != 0x448c05 { return; }
  pc = 0x448c05; continue;
 }
 0x448c05 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448c07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448c07); }
 let mut pc: u32 = 0x448c07;
 loop { match pc {
 0x448c07 => {
  cpu.push(0x448c0c); f_0048321c(cpu); if cpu.returned_to != 0x448c0c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448c0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448c0d); }
 let mut pc: u32 = 0x448c0d;
 loop { match pc {
 0x448c0d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x448c1e); f_0047a2d9(cpu); if cpu.returned_to != 0x448c1e { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x448c2a; continue; }
  pc = 0x448c2a; continue;
 }
 0x448c2a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x22eu32); }
  if cpu.zf { pc = 0x448c52; continue; }
  pc = 0x448c35; continue;
 }
 0x448c35 => {
  { cpu.sub32(cpu.eax, 0x16ebu32); }
  if cpu.zf { pc = 0x448c52; continue; }
  pc = 0x448c3c; continue;
 }
 0x448c3c => {
  { cpu.sub32(cpu.eax, 0x16f1u32); }
  if cpu.zf { pc = 0x448c52; continue; }
  pc = 0x448c43; continue;
 }
 0x448c43 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x3u8); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x448cd4; continue; }
  pc = 0x448c50; continue;
 }
 0x448c50 => {
  pc = 0x448c61; continue;
 }
 0x448c52 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.cf { pc = 0x448cd4; continue; }
  pc = 0x448c5b; continue;
 }
 0x448c5b => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  pc = 0x448c61; continue;
 }
 0x448c61 => {
  cpu.push(0x448c66); f_0041c743(cpu); if cpu.returned_to != 0x448c66 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x448c79; continue; }
  pc = 0x448c6d; continue;
 }
 0x448c6d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x448c79); f_0047f231(cpu); if cpu.returned_to != 0x448c79 { return; }
  pc = 0x448c79; continue;
 }
 0x448c79 => {
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5adc54u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x5adc53u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x5cc644u32, v); }
  cpu.push(0x448c9a); f_00448cdd(cpu); if cpu.returned_to != 0x448c9a { return; }
  cpu.push(0x448c9f); f_00469d47(cpu); if cpu.returned_to != 0x448c9f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc644u32, v); }
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if cpu.zf { pc = 0x448cd4; continue; }
  pc = 0x448cb3; continue;
 }
 0x448cb3 => {
  pc = 0x448cd5; continue;
 }
 0x448cc3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x448cd4); f_0047f231(cpu); if cpu.returned_to != 0x448cd4 { return; }
  pc = 0x448cd4; continue;
 }
 0x448cd4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448cd5 => {
  { let v: u8 = cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0x8u8); cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { cpu.poll(); pc = 0x448cd4; continue; }
  pc = 0x448cdb; continue;
 }
 0x448cdb => {
  cpu.poll(); pc = 0x448cc3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448cdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448cdd); }
 let mut pc: u32 = 0x448cdd;
 loop { match pc {
 0x448cdd => {
  { let v: u32 = 0x0u32; cpu.w32(0x576e84u32, v); }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x448cf4); f_004712e3(cpu); if cpu.returned_to != 0x448cf4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  if cpu.cf { pc = 0x448d06; continue; }
  pc = 0x448cfd; continue;
 }
 0x448cfd => {
  cpu.push(0x448d02); f_0046e31e(cpu); if cpu.returned_to != 0x448d02 { return; }
  if cpu.cf { pc = 0x448d06; continue; }
  pc = 0x448d04; continue;
 }
 0x448d04 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448d06 => {
  { cpu.popad(); }
  { let v: u32 = 0x1u32; cpu.w32(0x576e84u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x56d396u32); cpu.mul8(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x448d2f; continue; }
  pc = 0x448d2b; continue;
 }
 0x448d2b => {
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x448d2f; continue;
 }
 0x448d2f => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448d35); }
 let mut pc: u32 = 0x448d35;
 loop { match pc {
 0x448d35 => {
  { cpu.sub16(cpu.r16(0x23u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x448d7c; continue; }
  pc = 0x448d3c; continue;
 }
 0x448d3c => {
  { let v: u16 = cpu.dec16(cpu.r16(0x23u32.wrapping_add(cpu.esi))); cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x448d7c; continue; }
  pc = 0x448d42; continue;
 }
 0x448d42 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5adc0eu32, v); }
  { let v: u32 = 0x50u32; cpu.edx = v; }
  cpu.push(0x448d66); f_004685fb(cpu); if cpu.returned_to != 0x448d66 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc647u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5adc0eu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x448d7c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448d7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448d7e); }
 let mut pc: u32 = 0x448d7e;
 loop { match pc {
 0x448d7e => {
  { let v: u32 = 0x55fdb8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x55ddb8u32; cpu.edi = v; }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x448da3; continue;
 }
 0x448da3 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x448dc2; continue; }
  pc = 0x448da8; continue;
 }
 0x448da8 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x448dc2; continue; }
  pc = 0x448db5; continue;
 }
 0x448db5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x448dc2; continue;
 }
 0x448dc2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x448da3; continue; }
  pc = 0x448dca; continue;
 }
 0x448dca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00448dcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x448dcc); }
 let mut pc: u32 = 0x448dcc;
 loop { match pc {
 0x448dcc => {
  cpu.push(0x448dd1); f_0044950f(cpu); if cpu.returned_to != 0x448dd1 { return; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x448e2d; continue; }
  pc = 0x448dda; continue;
 }
 0x448dda => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x448e87; continue; }
  pc = 0x448de7; continue;
 }
 0x448de7 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x448ef9; continue; }
  pc = 0x448df4; continue;
 }
 0x448df4 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x448f5d; continue; }
  pc = 0x448e01; continue;
 }
 0x448e01 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x448fcf; continue; }
  pc = 0x448e0e; continue;
 }
 0x448e0e => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x449033; continue; }
  pc = 0x448e1b; continue;
 }
 0x448e1b => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x4490a2; continue; }
  pc = 0x448e28; continue;
 }
 0x448e28 => {
  pc = 0x449111; continue;
 }
 0x448e2d => {
  { let v: u32 = 0x57650bu32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x448e3c); f_00449112(cpu); if cpu.returned_to != 0x448e3c { return; }
  { let v: u32 = 0x576558u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x448e4b); f_00449112(cpu); if cpu.returned_to != 0x448e4b { return; }
  { let v: u32 = 0x5765acu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x448e5a); f_00449112(cpu); if cpu.returned_to != 0x448e5a { return; }
  { let v: u32 = 0x576607u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x448e69); f_00449112(cpu); if cpu.returned_to != 0x448e69 { return; }
  { let v: u32 = 0x576662u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x448e78); f_00449112(cpu); if cpu.returned_to != 0x448e78 { return; }
  { let v: u32 = 0x5ae15cu32; cpu.esi = v; }
  cpu.push(0x448e82); f_004494f2(cpu); if cpu.returned_to != 0x448e82 { return; }
  pc = 0x449111; continue;
 }
 0x448e87 => {
  { let v: u32 = 0x576516u32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x448e96); f_00449112(cpu); if cpu.returned_to != 0x448e96 { return; }
  { let v: u32 = 0x576564u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x448ea5); f_00449112(cpu); if cpu.returned_to != 0x448ea5 { return; }
  { let v: u32 = 0x5765b9u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x448eb4); f_00449112(cpu); if cpu.returned_to != 0x448eb4 { return; }
  { let v: u32 = 0x576614u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x448ec3); f_00449112(cpu); if cpu.returned_to != 0x448ec3 { return; }
  { let v: u32 = 0x57666eu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x448ed2); f_00449112(cpu); if cpu.returned_to != 0x448ed2 { return; }
  { let v: u32 = 0x5ae5e4u32; cpu.esi = v; }
  cpu.push(0x448edc); f_004494f2(cpu); if cpu.returned_to != 0x448edc { return; }
  { let v: u32 = 0x58652bu32; cpu.esi = v; }
  cpu.push(0x448ee6); f_00449120(cpu); if cpu.returned_to != 0x448ee6 { return; }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a668du32, v); }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a6754u32, v); }
  pc = 0x449111; continue;
 }
 0x448ef9 => {
  { let v: u32 = 0x576521u32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x448f08); f_00449112(cpu); if cpu.returned_to != 0x448f08 { return; }
  { let v: u32 = 0x576570u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x448f17); f_00449112(cpu); if cpu.returned_to != 0x448f17 { return; }
  { let v: u32 = 0x5765c6u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x448f26); f_00449112(cpu); if cpu.returned_to != 0x448f26 { return; }
  { let v: u32 = 0x576621u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x448f35); f_00449112(cpu); if cpu.returned_to != 0x448f35 { return; }
  { let v: u32 = 0x57667au32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x448f44); f_00449112(cpu); if cpu.returned_to != 0x448f44 { return; }
  { let v: u32 = 0x5aea6cu32; cpu.esi = v; }
  cpu.push(0x448f4e); f_004494f2(cpu); if cpu.returned_to != 0x448f4e { return; }
  { let v: u32 = 0x586563u32; cpu.esi = v; }
  cpu.push(0x448f58); f_00449120(cpu); if cpu.returned_to != 0x448f58 { return; }
  pc = 0x449111; continue;
 }
 0x448f5d => {
  { let v: u32 = 0x57652cu32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x448f6c); f_00449112(cpu); if cpu.returned_to != 0x448f6c { return; }
  { let v: u32 = 0x57657cu32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x448f7b); f_00449112(cpu); if cpu.returned_to != 0x448f7b { return; }
  { let v: u32 = 0x5765d3u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x448f8a); f_00449112(cpu); if cpu.returned_to != 0x448f8a { return; }
  { let v: u32 = 0x57662eu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x448f99); f_00449112(cpu); if cpu.returned_to != 0x448f99 { return; }
  { let v: u32 = 0x576686u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x448fa8); f_00449112(cpu); if cpu.returned_to != 0x448fa8 { return; }
  { let v: u32 = 0x5aeef4u32; cpu.esi = v; }
  cpu.push(0x448fb2); f_004494f2(cpu); if cpu.returned_to != 0x448fb2 { return; }
  { let v: u32 = 0x58659bu32; cpu.esi = v; }
  cpu.push(0x448fbc); f_00449120(cpu); if cpu.returned_to != 0x448fbc { return; }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a668du32, v); }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a6754u32, v); }
  pc = 0x449111; continue;
 }
 0x448fcf => {
  { let v: u32 = 0x576537u32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x448fde); f_00449112(cpu); if cpu.returned_to != 0x448fde { return; }
  { let v: u32 = 0x576588u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x448fed); f_00449112(cpu); if cpu.returned_to != 0x448fed { return; }
  { let v: u32 = 0x5765e0u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x448ffc); f_00449112(cpu); if cpu.returned_to != 0x448ffc { return; }
  { let v: u32 = 0x57663bu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x44900b); f_00449112(cpu); if cpu.returned_to != 0x44900b { return; }
  { let v: u32 = 0x576692u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x44901a); f_00449112(cpu); if cpu.returned_to != 0x44901a { return; }
  { let v: u32 = 0x5af37cu32; cpu.esi = v; }
  cpu.push(0x449024); f_004494f2(cpu); if cpu.returned_to != 0x449024 { return; }
  { let v: u32 = 0x5865d3u32; cpu.esi = v; }
  cpu.push(0x44902e); f_00449120(cpu); if cpu.returned_to != 0x44902e { return; }
  pc = 0x449111; continue;
 }
 0x449033 => {
  { let v: u32 = 0x576542u32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x449042); f_00449112(cpu); if cpu.returned_to != 0x449042 { return; }
  { let v: u32 = 0x576594u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x449051); f_00449112(cpu); if cpu.returned_to != 0x449051 { return; }
  { let v: u32 = 0x5765edu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x449060); f_00449112(cpu); if cpu.returned_to != 0x449060 { return; }
  { let v: u32 = 0x576648u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x44906f); f_00449112(cpu); if cpu.returned_to != 0x44906f { return; }
  { let v: u32 = 0x57669eu32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x44907e); f_00449112(cpu); if cpu.returned_to != 0x44907e { return; }
  { let v: u32 = 0x5af804u32; cpu.esi = v; }
  cpu.push(0x449088); f_004494f2(cpu); if cpu.returned_to != 0x449088 { return; }
  { let v: u32 = 0x58660bu32; cpu.esi = v; }
  cpu.push(0x449092); f_00449120(cpu); if cpu.returned_to != 0x449092 { return; }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a668du32, v); }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a6754u32, v); }
  pc = 0x449111; continue;
 }
 0x4490a2 => {
  { let v: u32 = 0x57654du32; cpu.esi = v; }
  { let v: u32 = 0x5ba9acu32; cpu.edi = v; }
  cpu.push(0x4490b1); f_00449112(cpu); if cpu.returned_to != 0x4490b1 { return; }
  { let v: u32 = 0x5765a0u32; cpu.esi = v; }
  { let v: u32 = 0x5baa07u32; cpu.edi = v; }
  cpu.push(0x4490c0); f_00449112(cpu); if cpu.returned_to != 0x4490c0 { return; }
  { let v: u32 = 0x5765fau32; cpu.esi = v; }
  { let v: u32 = 0x5c4f7bu32; cpu.edi = v; }
  cpu.push(0x4490cf); f_00449112(cpu); if cpu.returned_to != 0x4490cf { return; }
  { let v: u32 = 0x576655u32; cpu.esi = v; }
  { let v: u32 = 0x5c4f6eu32; cpu.edi = v; }
  cpu.push(0x4490de); f_00449112(cpu); if cpu.returned_to != 0x4490de { return; }
  { let v: u32 = 0x5766aau32; cpu.esi = v; }
  { let v: u32 = 0x5c4f93u32; cpu.edi = v; }
  cpu.push(0x4490ed); f_00449112(cpu); if cpu.returned_to != 0x4490ed { return; }
  { let v: u32 = 0x5afc8cu32; cpu.esi = v; }
  cpu.push(0x4490f7); f_004494f2(cpu); if cpu.returned_to != 0x4490f7 { return; }
  { let v: u32 = 0x586643u32; cpu.esi = v; }
  cpu.push(0x449101); f_00449120(cpu); if cpu.returned_to != 0x449101 { return; }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a668du32, v); }
  { let v: u8 = 0x4eu8; cpu.w8(0x5a6754u32, v); }
  pc = 0x449111; continue;
 }
 0x449111 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449112(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449112); }
 let mut pc: u32 = 0x449112;
 loop { match pc {
 0x449112 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44911f; continue; }
  pc = 0x44911c; continue;
 }
 0x44911c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449112; continue; }
  pc = 0x44911f; continue;
 }
 0x44911f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449120); }
 let mut pc: u32 = 0x449120;
 loop { match pc {
 0x449120 => {
  { let v: u32 = 0x5864f3u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x44912a; continue;
 }
 0x44912a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x449139; continue; }
  pc = 0x449134; continue;
 }
 0x449134 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x449159; continue;
 }
 0x449139 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x449140; continue;
 }
 0x449140 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44914d; continue; }
  pc = 0x44914a; continue;
 }
 0x44914a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449140; continue; }
  pc = 0x44914d; continue;
 }
 0x44914d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44912a; continue; }
  pc = 0x449159; continue;
 }
 0x449159 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044915a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44915a); }
 let mut pc: u32 = 0x44915a;
 loop { match pc {
 0x44915a => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x4491ea; continue; }
  pc = 0x449167; continue;
 }
 0x449167 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x44919f; continue; }
  pc = 0x449170; continue;
 }
 0x449170 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x4491ab; continue; }
  pc = 0x449179; continue;
 }
 0x449179 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x4491bc; continue; }
  pc = 0x449182; continue;
 }
 0x449182 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x4491e0; continue; }
  pc = 0x44918b; continue;
 }
 0x44918b => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x4491c8; continue; }
  pc = 0x449194; continue;
 }
 0x449194 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x4491d4; continue; }
  pc = 0x44919d; continue;
 }
 0x44919d => {
  pc = 0x4491ea; continue;
 }
 0x44919f => {
  { let v: u32 = 0x579560u32; cpu.esi = v; }
  cpu.push(0x4491a9); f_004491f0(cpu); if cpu.returned_to != 0x4491a9 { return; }
  pc = 0x4491ea; continue;
 }
 0x4491ab => {
  { let v: u32 = 0x5795b0u32; cpu.esi = v; }
  cpu.push(0x4491b5); f_004491f0(cpu); if cpu.returned_to != 0x4491b5 { return; }
  cpu.push(0x4491ba); f_00449221(cpu); if cpu.returned_to != 0x4491ba { return; }
  pc = 0x4491ea; continue;
 }
 0x4491bc => {
  { let v: u32 = 0x57968cu32; cpu.esi = v; }
  cpu.push(0x4491c6); f_004491f0(cpu); if cpu.returned_to != 0x4491c6 { return; }
  pc = 0x4491ea; continue;
 }
 0x4491c8 => {
  { let v: u32 = 0x5796dcu32; cpu.esi = v; }
  cpu.push(0x4491d2); f_004491f0(cpu); if cpu.returned_to != 0x4491d2 { return; }
  pc = 0x4491ea; continue;
 }
 0x4491d4 => {
  { let v: u32 = 0x57972cu32; cpu.esi = v; }
  cpu.push(0x4491de); f_004491f0(cpu); if cpu.returned_to != 0x4491de { return; }
  pc = 0x4491ea; continue;
 }
 0x4491e0 => {
  { let v: u32 = 0x57963cu32; cpu.esi = v; }
  cpu.push(0x4491ea); f_004491f0(cpu); if cpu.returned_to != 0x4491ea { return; }
  pc = 0x4491ea; continue;
 }
 0x4491ea => {
  cpu.push(0x4491ef); f_00449387(cpu); if cpu.returned_to != 0x4491ef { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004491f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4491f0); }
 let mut pc: u32 = 0x4491f0;
 loop { match pc {
 0x4491f0 => {
  { let v: u32 = 0x4ae86cu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x4491fa; continue;
 }
 0x4491fa => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x449204; continue;
 }
 0x449204 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x449211; continue; }
  pc = 0x44920e; continue;
 }
 0x44920e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449204; continue; }
  pc = 0x449211; continue;
 }
 0x449211 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0xf0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4491fa; continue; }
  pc = 0x449220; continue;
 }
 0x449220 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449221); }
 let mut pc: u32 = 0x449221;
 loop { match pc {
 0x449221 => {
  { let v: u32 = 0x4ae77cu32; cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  pc = 0x44922b; continue;
 }
 0x44922b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x449243; continue;
 }
 0x449243 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x449250; continue; }
  pc = 0x44924d; continue;
 }
 0x44924d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449243; continue; }
  pc = 0x449250; continue;
 }
 0x449250 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44922b; continue; }
  pc = 0x449259; continue;
 }
 0x449259 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044925a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44925a); }
 let mut pc: u32 = 0x44925a;
 loop { match pc {
 0x44925a => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x449328; continue; }
  pc = 0x449267; continue;
 }
 0x449267 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x2u8); }
  if cpu.zf { pc = 0x4492a6; continue; }
  pc = 0x449270; continue;
 }
 0x449270 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x3u8); }
  if cpu.zf { pc = 0x4492e8; continue; }
  pc = 0x449279; continue;
 }
 0x449279 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x4u8); }
  if cpu.zf { pc = 0x4492fe; continue; }
  pc = 0x449282; continue;
 }
 0x449282 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x449314; continue; }
  pc = 0x44928f; continue;
 }
 0x44928f => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x6u8); }
  if cpu.zf { pc = 0x4492bc; continue; }
  pc = 0x449298; continue;
 }
 0x449298 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x7u8); }
  if cpu.zf { pc = 0x4492d2; continue; }
  pc = 0x4492a1; continue;
 }
 0x4492a1 => {
  pc = 0x449328; continue;
 }
 0x4492a6 => {
  { let v: u32 = 0x57977cu32; cpu.esi = v; }
  cpu.push(0x4492b0); f_00449346(cpu); if cpu.returned_to != 0x4492b0 { return; }
  { let v: u32 = 0x5d5607u32; cpu.esi = v; }
  cpu.push(0x4492ba); f_00449329(cpu); if cpu.returned_to != 0x4492ba { return; }
  pc = 0x449328; continue;
 }
 0x4492bc => {
  { let v: u32 = 0x5798acu32; cpu.esi = v; }
  cpu.push(0x4492c6); f_00449346(cpu); if cpu.returned_to != 0x4492c6 { return; }
  { let v: u32 = 0x5d5d8fu32; cpu.esi = v; }
  cpu.push(0x4492d0); f_00449329(cpu); if cpu.returned_to != 0x4492d0 { return; }
  pc = 0x449328; continue;
 }
 0x4492d2 => {
  { let v: u32 = 0x5798f8u32; cpu.esi = v; }
  cpu.push(0x4492dc); f_00449346(cpu); if cpu.returned_to != 0x4492dc { return; }
  { let v: u32 = 0x5d5c57u32; cpu.esi = v; }
  cpu.push(0x4492e6); f_00449329(cpu); if cpu.returned_to != 0x4492e6 { return; }
  pc = 0x449328; continue;
 }
 0x4492e8 => {
  { let v: u32 = 0x5797c8u32; cpu.esi = v; }
  cpu.push(0x4492f2); f_00449346(cpu); if cpu.returned_to != 0x4492f2 { return; }
  { let v: u32 = 0x5d579bu32; cpu.esi = v; }
  cpu.push(0x4492fc); f_00449329(cpu); if cpu.returned_to != 0x4492fc { return; }
  pc = 0x449328; continue;
 }
 0x4492fe => {
  { let v: u32 = 0x579814u32; cpu.esi = v; }
  cpu.push(0x449308); f_00449346(cpu); if cpu.returned_to != 0x449308 { return; }
  { let v: u32 = 0x5d592fu32; cpu.esi = v; }
  cpu.push(0x449312); f_00449329(cpu); if cpu.returned_to != 0x449312 { return; }
  pc = 0x449328; continue;
 }
 0x449314 => {
  { let v: u32 = 0x579860u32; cpu.esi = v; }
  cpu.push(0x44931e); f_00449346(cpu); if cpu.returned_to != 0x44931e { return; }
  { let v: u32 = 0x5d5ac3u32; cpu.esi = v; }
  cpu.push(0x449328); f_00449329(cpu); if cpu.returned_to != 0x449328 { return; }
  pc = 0x449328; continue;
 }
 0x449328 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449329(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449329); }
 let mut pc: u32 = 0x449329;
 loop { match pc {
 0x449329 => {
  { let v: u32 = 0x5d5477u32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x449333; continue;
 }
 0x449333 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x449345; continue; }
  pc = 0x44933a; continue;
 }
 0x44933a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449333; continue; }
  pc = 0x449345; continue;
 }
 0x449345 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449346(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449346); }
 let mut pc: u32 = 0x449346;
 loop { match pc {
 0x449346 => {
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x44934d; continue;
 }
 0x44934d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x52u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4650u32); cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  pc = 0x44936a; continue;
 }
 0x44936a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44937b; continue; }
  pc = 0x449370; continue;
 }
 0x449370 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44937b; continue; }
  pc = 0x449378; continue;
 }
 0x449378 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44936a; continue; }
  pc = 0x44937b; continue;
 }
 0x44937b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44934d; continue; }
  pc = 0x449386; continue;
 }
 0x449386 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449387(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449387); }
 let mut pc: u32 = 0x449387;
 loop { match pc {
 0x449387 => {
  { cpu.sub8(cpu.r8(0x56d314u32), 0x1u8); }
  if cpu.zf { pc = 0x4494f1; continue; }
  pc = 0x449394; continue;
 }
 0x449394 => {
  { let v: u32 = 0x4afb2cu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u16 = 0x14u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x5b0114u32; cpu.edi = v; }
  pc = 0x4493ac; continue;
 }
 0x4493ac => {
  { let v: u32 = 0xe8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4494f1; continue; }
  pc = 0x4493c2; continue;
 }
 0x4493c2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4ae77cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x0u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x13u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ae0c4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x65736544u32); }
  if cpu.zf { pc = 0x449422; continue; }
  pc = 0x4493ec; continue;
 }
 0x4493ec => {
  { let v: u32 = cpu.r32(0x5ae0c8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x636c6f56u32); }
  if cpu.zf { pc = 0x449422; continue; }
  pc = 0x4493f9; continue;
 }
 0x4493f9 => {
  { let v: u32 = cpu.r32(0x5ae0ccu32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x74637241u32); }
  if cpu.zf { pc = 0x449422; continue; }
  pc = 0x449406; continue;
 }
 0x449406 => {
  { let v: u32 = cpu.r32(0x5ae0d0u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x676e754au32); }
  if cpu.zf { pc = 0x449422; continue; }
  pc = 0x449413; continue;
 }
 0x449413 => {
  { let v: u32 = cpu.r32(0x5ae0d4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x79746943u32); }
  if cpu.zf { pc = 0x449422; continue; }
  pc = 0x449420; continue;
 }
 0x449420 => {
  pc = 0x449473; continue;
 }
 0x449422 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x449427; continue;
 }
 0x449427 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x449434; continue; }
  pc = 0x449431; continue;
 }
 0x449431 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449427; continue; }
  pc = 0x449434; continue;
 }
 0x449434 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x449466; continue; }
  pc = 0x44943e; continue;
 }
 0x44943e => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x449457; continue; }
  pc = 0x449443; continue;
 }
 0x449443 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x449473; continue; }
  pc = 0x449448; continue;
 }
 0x449448 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x33u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x449473; continue;
 }
 0x449457 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x32u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x449473; continue;
 }
 0x449466 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x449473; continue;
 }
 0x449473 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x4494a5; continue; }
  pc = 0x44947c; continue;
 }
 0x44947c => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x449485; continue;
 }
 0x449485 => {
  { let v: u8 = 0x45u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x44948d; continue;
 }
 0x44948d => {
  { let v: u8 = 0x58u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x449495; continue;
 }
 0x449495 => {
  { let v: u8 = 0x50u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x44949d; continue;
 }
 0x44949d => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494a5; continue;
 }
 0x4494a5 => {
  { cpu.and32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x4494e1; continue; }
  pc = 0x4494ac; continue;
 }
 0x4494ac => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494b5; continue;
 }
 0x4494b5 => {
  { let v: u8 = 0x52u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494bd; continue;
 }
 0x4494bd => {
  { let v: u8 = 0x45u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494c5; continue;
 }
 0x4494c5 => {
  { let v: u8 = 0x4du8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494cd; continue;
 }
 0x4494cd => {
  { let v: u8 = 0x49u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494d5; continue;
 }
 0x4494d5 => {
  { let v: u8 = 0x58u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x4494e1; continue; }
  pc = 0x4494dd; continue;
 }
 0x4494dd => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x4494e1; continue;
 }
 0x4494e1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x4493ac; continue;
 }
 0x4494f1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004494f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4494f2); }
 let mut pc: u32 = 0x4494f2;
 loop { match pc {
 0x4494f2 => {
  { let v: u32 = 0x5adcd8u32; cpu.edi = v; }
  { let v: u32 = 0x12cu32; cpu.ecx = v; }
  pc = 0x4494fc; continue;
 }
 0x4494fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x44950e; continue; }
  pc = 0x449503; continue;
 }
 0x449503 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4494fc; continue; }
  pc = 0x44950e; continue;
 }
 0x44950e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044950f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44950f); }
 let mut pc: u32 = 0x44950f;
 loop { match pc {
 0x44950f => {
  { cpu.pushad(); }
  { let v: u32 = 0x57676du32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x2u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x44951e; continue;
 }
 0x44951e => {
  { let v: u32 = 0x57681fu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x3u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x44952c; continue;
 }
 0x44952c => {
  { let v: u32 = 0x576ae7u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x4u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x44953a; continue;
 }
 0x44953a => {
  { let v: u32 = 0x5768d1u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x5u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x449548; continue;
 }
 0x449548 => {
  { let v: u32 = 0x576983u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x5u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x449556; continue;
 }
 0x449556 => {
  { let v: u32 = 0x576a35u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x576bd7u32), 0x5u8); }
  if cpu.zf { pc = 0x449569; continue; }
  pc = 0x449564; continue;
 }
 0x449564 => {
  { let v: u32 = 0x5766bbu32; cpu.esi = v; }
  pc = 0x449569; continue;
 }
 0x449569 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44956f); f_004030c0(cpu); if cpu.returned_to != 0x44956f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449574); }
 let mut pc: u32 = 0x449574;
 loop { match pc {
 0x449574 => {
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x449590; continue; }
  pc = 0x449584; continue;
 }
 0x449584 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x44958f); f_00449758(cpu); if cpu.returned_to != 0x44958f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x449590; continue;
 }
 0x449590 => {
  cpu.push(0x449595); f_004496ea(cpu); if cpu.returned_to != 0x449595 { return; }
  cpu.push(0x44959a); f_004496fc(cpu); if cpu.returned_to != 0x44959a { return; }
  cpu.push(0x44959f); f_004495a0(cpu); if cpu.returned_to != 0x44959f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004495a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4495a0); }
 let mut pc: u32 = 0x4495a0;
 loop { match pc {
 0x4495a0 => {
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0x4ad05cu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4495af; continue;
 }
 0x4495af => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4495bf; continue; }
  pc = 0x4495b5; continue;
 }
 0x4495b5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4495af; continue; }
  pc = 0x4495be; continue;
 }
 0x4495be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4495bf => {
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  cpu.push(0x4495ca); f_004496ea(cpu); if cpu.returned_to != 0x4495ca { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d68aeu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x5d627cu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b02e4u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5b02e4u32), cpu.eax); cpu.w32(0x5b02e4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449614); }
 let mut pc: u32 = 0x449614;
 loop { match pc {
 0x449614 => {
  { let v: u32 = 0x4ad05cu32; cpu.edi = v; }
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x449623; continue;
 }
 0x449623 => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x449638; continue; }
  pc = 0x449629; continue;
 }
 0x449629 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x449638; continue; }
  pc = 0x44962f; continue;
 }
 0x44962f => {
  cpu.push(0x449634); f_00449642(cpu); if cpu.returned_to != 0x449634 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x449638; continue;
 }
 0x449638 => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449623; continue; }
  pc = 0x449641; continue;
 }
 0x449641 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449642(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449642); }
 let mut pc: u32 = 0x449642;
 loop { match pc {
 0x449642 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x449657; continue; }
  pc = 0x44964b; continue;
 }
 0x44964b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x449656); f_00449758(cpu); if cpu.returned_to != 0x449656 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x449657; continue;
 }
 0x449657 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x449670; continue; }
  pc = 0x449664; continue;
 }
 0x449664 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x44966f); f_00449758(cpu); if cpu.returned_to != 0x44966f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x449670; continue;
 }
 0x449670 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4496e8; continue; }
  pc = 0x44967a; continue;
 }
 0x44967a => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x4496e8; continue; }
  pc = 0x449684; continue;
 }
 0x449684 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x4496ae; continue; }
  pc = 0x4496a2; continue;
 }
 0x4496a2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x4496ad); f_00449758(cpu); if cpu.returned_to != 0x4496ad { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4496ae; continue;
 }
 0x4496ae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x4496c2; continue; }
  pc = 0x4496b6; continue;
 }
 0x4496b6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x4496c1); f_00449758(cpu); if cpu.returned_to != 0x4496c1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4496c2; continue;
 }
 0x4496c2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x4496dd; continue; }
  pc = 0x4496d1; continue;
 }
 0x4496d1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.push(0x4496dc); f_00449758(cpu); if cpu.returned_to != 0x4496dc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4496dd; continue;
 }
 0x4496dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  cpu.push(0x4496e8); f_00413701(cpu); if cpu.returned_to != 0x4496e8 { return; }
  pc = 0x4496e8; continue;
 }
 0x4496e8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004496ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4496ea); }
 let mut pc: u32 = 0x4496ea;
 loop { match pc {
 0x4496ea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004496fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4496fc); }
 let mut pc: u32 = 0x4496fc;
 loop { match pc {
 0x4496fc => {
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x449706; continue;
 }
 0x449706 => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x449717; continue; }
  pc = 0x44970c; continue;
 }
 0x44970c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x449717; continue; }
  pc = 0x449711; continue;
 }
 0x449711 => {
  { let v: u8 = 0xffu8; cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x44971d; continue;
 }
 0x449717 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449706; continue; }
  pc = 0x44971d; continue;
 }
 0x44971d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044971e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44971e); }
 let mut pc: u32 = 0x44971e;
 loop { match pc {
 0x44971e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x44972a; continue;
 }
 0x44972a => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x449739; continue; }
  pc = 0x449730; continue;
 }
 0x449730 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x449739; continue; }
  pc = 0x449735; continue;
 }
 0x449735 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449739 => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44972a; continue; }
  pc = 0x44973f; continue;
 }
 0x44973f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449743); }
 let mut pc: u32 = 0x449743;
 loop { match pc {
 0x449743 => {
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x44974d; continue;
 }
 0x44974d => {
  { let v: u8 = 0xffu8; cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44974d; continue; }
  pc = 0x449757; continue;
 }
 0x449757 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449758(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449758); }
 let mut pc: u32 = 0x449758;
 loop { match pc {
 0x449758 => {
  cpu.push(0x44975d); f_0046cbbf(cpu); if cpu.returned_to != 0x44975d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449760); }
 let mut pc: u32 = 0x449760;
 loop { match pc {
 0x449760 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4e1e4au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e40u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e43u32, v); }
  cpu.push(0x44977a); f_0044a03c(cpu); if cpu.returned_to != 0x44977a { return; }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x44978a; continue; }
  pc = 0x449783; continue;
 }
 0x449783 => {
  { let v: u32 = 0x58c703u32; cpu.esi = v; }
  pc = 0x449798; continue;
 }
 0x44978a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b2104u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x449798; continue;
 }
 0x449798 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4497b7; continue; }
  pc = 0x4497aa; continue;
 }
 0x4497aa => {
  { cpu.sub8(cpu.r8(0x4d67f6u32), 0x2u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4498b2; continue; }
  pc = 0x4497b7; continue;
 }
 0x4497b7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x4497f7; continue; }
  pc = 0x4497e9; continue;
 }
 0x4497e9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x4497f7; continue;
 }
 0x4497f7 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x509290u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x44981e); f_00449b68(cpu); if cpu.returned_to != 0x44981e { return; }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x44982e; continue; }
  pc = 0x449827; continue;
 }
 0x449827 => {
  { let v: u32 = 0x58c733u32; cpu.esi = v; }
  pc = 0x44983c; continue;
 }
 0x44982e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b2294u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x44983c; continue;
 }
 0x44983c => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = 0x78u8; cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449887; continue; }
  pc = 0x449879; continue;
 }
 0x449879 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x449887; continue;
 }
 0x449887 => {
  { let v: u32 = 0x78u32; cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x509348u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x4498b1); f_00449b68(cpu); if cpu.returned_to != 0x4498b1 { return; }
  pc = 0x4498b1; continue;
 }
 0x4498b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4498b2 => {
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if !cpu.zf { pc = 0x4498f5; continue; }
  pc = 0x4498bb; continue;
 }
 0x4498bb => {
  { let v: u32 = 0x4e15c8u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x78u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x4498cb; continue;
 }
 0x4498cb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4498cb; continue; }
  pc = 0x4498d8; continue;
 }
 0x4498d8 => {
  { let v: u32 = 0x4e16b8u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x78u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x4498e8; continue;
 }
 0x4498e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4498e8; continue; }
  pc = 0x4498f5; continue;
 }
 0x4498f5 => {
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449905; continue; }
  pc = 0x4498fe; continue;
 }
 0x4498fe => {
  { let v: u32 = 0x58c703u32; cpu.esi = v; }
  pc = 0x449913; continue;
 }
 0x449905 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b2104u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x449913; continue;
 }
 0x449913 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x44995c; continue; }
  pc = 0x44994e; continue;
 }
 0x44994e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x44995c; continue;
 }
 0x44995c => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x509290u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x449983); f_00449b68(cpu); if cpu.returned_to != 0x449983 { return; }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449993; continue; }
  pc = 0x44998c; continue;
 }
 0x44998c => {
  { let v: u32 = 0x58c733u32; cpu.esi = v; }
  pc = 0x4499a1; continue;
 }
 0x449993 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b2294u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x4499a1; continue;
 }
 0x4499a1 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = 0x3cu8; cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x4499ec; continue; }
  pc = 0x4499de; continue;
 }
 0x4499de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x4499ec; continue;
 }
 0x4499ec => {
  { let v: u32 = 0x3cu32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x509348u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x449a16); f_00449b68(cpu); if cpu.returned_to != 0x449a16 { return; }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449a26; continue; }
  pc = 0x449a1f; continue;
 }
 0x449a1f => {
  { let v: u32 = 0x58c763u32; cpu.esi = v; }
  pc = 0x449a34; continue;
 }
 0x449a26 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b2424u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x449a34; continue;
 }
 0x449a34 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = 0x78u8; cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449a7f; continue; }
  pc = 0x449a71; continue;
 }
 0x449a71 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x449a7f; continue;
 }
 0x449a7f => {
  { let v: u32 = 0x78u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x50e8d0u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x449aa9); f_00449b68(cpu); if cpu.returned_to != 0x449aa9 { return; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { cpu.poll(); pc = 0x4498b1; continue; }
  pc = 0x449ab7; continue;
 }
 0x449ab7 => {
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449ac7; continue; }
  pc = 0x449ac0; continue;
 }
 0x449ac0 => {
  { let v: u32 = 0x58c793u32; cpu.esi = v; }
  pc = 0x449ad5; continue;
 }
 0x449ac7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b25b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x449ad5; continue;
 }
 0x449ad5 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5b2748u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3fu32, v); }
  { let v: u8 = 0xb4u8; cpu.w8(0x4e1e46u32, v); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x449b20; continue; }
  pc = 0x449b12; continue;
 }
 0x449b12 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6892u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6890u32, v); }
  pc = 0x449b20; continue;
 }
 0x449b20 => {
  { let v: u32 = 0xb4u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = 0x50e988u32; cpu.w32(0x4d6a2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2750u32, v); }
  cpu.push(0x449b4a); f_00449b68(cpu); if cpu.returned_to != 0x449b4a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449b4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449b4b); }
 let mut pc: u32 = 0x449b4b;
 loop { match pc {
 0x449b4b => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x449b55; continue;
 }
 0x449b55 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x449b5e; continue; }
  pc = 0x449b5c; continue;
 }
 0x449b5c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449b5e => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x449b55; continue; }
  pc = 0x449b66; continue;
 }
 0x449b66 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449b68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449b68); }
 let mut pc: u32 = 0x449b68;
 loop { match pc {
 0x449b68 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b2744u32, v); }
  pc = 0x449b72; continue;
 }
 0x449b72 => {
  { cpu.sub8(cpu.r8(0x4e15c8u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x449b92; continue; }
  pc = 0x449b7b; continue;
 }
 0x449b7b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5928d1u32, v); }
  cpu.push(0x449b90); f_00449b96(cpu); if cpu.returned_to != 0x449b90 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x449b92; continue;
 }
 0x449b92 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x449b72; continue; }
  pc = 0x449b95; continue;
 }
 0x449b95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449b96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449b96); }
 let mut pc: u32 = 0x449b96;
 loop { match pc {
 0x449b96 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5b274cu32, v); }
  { cpu.sub8(cpu.r8(0x5928d1u32), 0x0u8); }
  if !cpu.zf { pc = 0x449bdd; continue; }
  pc = 0x449ba5; continue;
 }
 0x449ba5 => {
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x449bd2; continue; }
  pc = 0x449bae; continue;
 }
 0x449bae => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x449bdd; continue; }
  pc = 0x449bb7; continue;
 }
 0x449bb7 => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x449bdd; continue; }
  pc = 0x449bc0; continue;
 }
 0x449bc0 => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x449bdd; continue; }
  pc = 0x449bc9; continue;
 }
 0x449bc9 => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x449bdd; continue; }
  pc = 0x449bd2; continue;
 }
 0x449bd2 => {
  cpu.push(0x449bd7); f_0044a03d(cpu); if cpu.returned_to != 0x449bd7 { return; }
  cpu.push(0x449bdc); f_00449bf0(cpu); if cpu.returned_to != 0x449bdc { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449bdd => {
  cpu.push(0x449be2); f_0044a03d(cpu); if cpu.returned_to != 0x449be2 { return; }
  cpu.push(0x449be7); f_0044a09f(cpu); if cpu.returned_to != 0x449be7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449bf0); }
 let mut pc: u32 = 0x449bf0;
 loop { match pc {
 0x449bf0 => {
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.esi = v; }
  { let v: u32 = cpu.r8(0x4e1e3fu32) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x449db3; continue; }
  pc = 0x449c05; continue;
 }
 0x449c05 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x449db3; continue; }
  pc = 0x449c14; continue;
 }
 0x449c14 => {
  { cpu.sub32(cpu.r32(0x2u32.wrapping_add(cpu.ebx)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x449c34; continue; }
  pc = 0x449c1d; continue;
 }
 0x449c1d => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.ebx)), 0x7265u16); }
  if !cpu.zf { pc = 0x449c34; continue; }
  pc = 0x449c25; continue;
 }
 0x449c25 => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x6u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.ebx), v); }
  pc = 0x449c34; continue;
 }
 0x449c34 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x449c7d; continue; }
  pc = 0x449c38; continue;
 }
 0x449c38 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5b2748u32), 0x4u32); cpu.w32(0x5b2748u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x449c45); f_00449e65(cpu); if cpu.returned_to != 0x449c45 { return; }
  cpu.push(0x449c4a); f_0044a64b(cpu); if cpu.returned_to != 0x449c4a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x449c7b; continue; }
  pc = 0x449c4d; continue;
 }
 0x449c4d => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if cpu.zf { pc = 0x449c63; continue; }
  pc = 0x449c5f; continue;
 }
 0x449c5f => {
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x449c63; continue;
 }
 0x449c63 => {
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x449c6e); f_0044a243(cpu); if cpu.returned_to != 0x449c6e { return; }
  if cpu.cf { pc = 0x449c7b; continue; }
  pc = 0x449c70; continue;
 }
 0x449c70 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1e3fu32)); cpu.w8(0x4e1e3fu32, v); }
  cpu.poll(); pc = 0x449bf0; continue;
 }
 0x449c7b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449c7d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e16b8u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.esi = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if cpu.zf { pc = 0x449d19; continue; }
  pc = 0x449ccb; continue;
 }
 0x449ccb => {
  { cpu.sub8((cpu.edx as u8), 0xeu8); }
  if cpu.zf { pc = 0x449d66; continue; }
  pc = 0x449cd4; continue;
 }
 0x449cd4 => {
  cpu.push(0x449cd9); f_00449e65(cpu); if cpu.returned_to != 0x449cd9 { return; }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x449cfe; continue; }
  pc = 0x449ce2; continue;
 }
 0x449ce2 => {
  { let v: u8 = cpu.r8(0x4e1e3fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x449cfe; continue;
 }
 0x449cfe => {
  cpu.push(0x449d03); f_0044a64b(cpu); if cpu.returned_to != 0x449d03 { return; }
  if cpu.cf { pc = 0x449d17; continue; }
  pc = 0x449d05; continue;
 }
 0x449d05 => {
  cpu.push(0x449d0a); f_0044a243(cpu); if cpu.returned_to != 0x449d0a { return; }
  if cpu.cf { pc = 0x449d17; continue; }
  pc = 0x449d0c; continue;
 }
 0x449d0c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1e3fu32)); cpu.w8(0x4e1e3fu32, v); }
  cpu.poll(); pc = 0x449bf0; continue;
 }
 0x449d17 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449d19 => {
  cpu.push(0x449d1e); f_00449e2e(cpu); if cpu.returned_to != 0x449d1e { return; }
  if cpu.cf { cpu.poll(); pc = 0x449d17; continue; }
  pc = 0x449d20; continue;
 }
 0x449d20 => {
  { let v: u8 = 0x3eu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x449d4a); f_00449de0(cpu); if cpu.returned_to != 0x449d4a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52bbf4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1e3fu32)); cpu.w8(0x4e1e3fu32, v); }
  cpu.poll(); pc = 0x449bf0; continue;
 }
 0x449d66 => {
  cpu.push(0x449d6b); f_00449e2e(cpu); if cpu.returned_to != 0x449d6b { return; }
  if cpu.cf { cpu.poll(); pc = 0x449d17; continue; }
  pc = 0x449d6d; continue;
 }
 0x449d6d => {
  { let v: u8 = 0x3eu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x449d97); f_00449de0(cpu); if cpu.returned_to != 0x449d97 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52bbf4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1e3fu32)); cpu.w8(0x4e1e3fu32, v); }
  cpu.poll(); pc = 0x449bf0; continue;
 }
 0x449db3 => {
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x449dde; continue; }
  pc = 0x449dbc; continue;
 }
 0x449dbc => {
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e3eu32, v); }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e15c8u32.wrapping_add(cpu.esi), v); }
  pc = 0x449dde; continue;
 }
 0x449dde => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449de0); }
 let mut pc: u32 = 0x449de0;
 loop { match pc {
 0x449de0 => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4650u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x449df1; continue;
 }
 0x449df1 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x449e1d; continue; }
  pc = 0x449e1a; continue;
 }
 0x449e1a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449e1d => {
  { let v: u32 = cpu.esi.wrapping_add(0x52u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x449df1; continue; }
  pc = 0x449e23; continue;
 }
 0x449e23 => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4650u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449e2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x449e2e); }
 let mut pc: u32 = 0x449e2e;
 loop { match pc {
 0x449e2e => {
  cpu.push(0x449e33); f_00449e65(cpu); if cpu.returned_to != 0x449e33 { return; }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x449e58; continue; }
  pc = 0x449e3c; continue;
 }
 0x449e3c => {
  { let v: u8 = cpu.r8(0x4e1e3fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x449e58; continue;
 }
 0x449e58 => {
  cpu.push(0x449e5d); f_0044a64b(cpu); if cpu.returned_to != 0x449e5d { return; }
  if cpu.cf { pc = 0x449e64; continue; }
  pc = 0x449e5f; continue;
 }
 0x449e5f => {
  cpu.push(0x449e64); f_0044a243(cpu); if cpu.returned_to != 0x449e64 { return; }
  pc = 0x449e64; continue;
 }
 0x449e64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00449e65(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x449e65); } r_00449e65(cpu, 0x449e65); }
pub fn f_0044a000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44a000); } r_00449e65(cpu, 0x44a000); }
fn r_00449e65(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x449e65 => {
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if cpu.zf { pc = 0x449e7c; continue; }
  pc = 0x449e6e; continue;
 }
 0x449e6e => {
  { let v: u16 = cpu.r16(0x4d69e2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d69e4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449e7c => {
  { let v: u16 = 0x0u16; cpu.w16(0x5b2758u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x5b275au32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x449eba; continue; }
  pc = 0x449ead; continue;
 }
 0x449ead => {
  { cpu.sub8(cpu.r8(0x4e1e46u32), 0x78u8); }
  if cpu.zf { pc = 0x449fd9; continue; }
  pc = 0x449eba; continue;
 }
 0x449eba => {
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x449f21; continue; }
  pc = 0x449ec6; continue;
 }
 0x449ec6 => {
  { let v: u16 = 0xfffeu16; cpu.w16(0x5b2758u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x5b275au32, v); }
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x449f21; continue; }
  pc = 0x449ee1; continue;
 }
 0x449ee1 => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x449f21; continue; }
  pc = 0x449eea; continue;
 }
 0x449eea => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x449f21; continue; }
  pc = 0x449ef3; continue;
 }
 0x449ef3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5b2758u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x5b275au32, v); }
  pc = 0x449f05; continue;
 }
 0x449f05 => {
  { let v: u32 = cpu.r32(0x5b2748u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6890u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6892u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449f21 => {
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x449f05; continue; }
  pc = 0x449f2a; continue;
 }
 0x449f2a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x449f75; continue; }
  pc = 0x449f33; continue;
 }
 0x449f33 => {
  { cpu.sub8(cpu.r8(0x4e1e47u32), 0x0u8); }
  if cpu.zf { pc = 0x449f75; continue; }
  pc = 0x449f3c; continue;
 }
 0x449f3c => {
  { cpu.sub8(cpu.r8(0x4e1e47u32), 0x2u8); }
  if cpu.zf { pc = 0x449f75; continue; }
  pc = 0x449f45; continue;
 }
 0x449f45 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b2744u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x56ccb2u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56d168u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x449fa3; continue;
 }
 0x449f75 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b2744u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x56ccb2u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56d162u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x449fa3; continue;
 }
 0x449fa3 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x5b390au32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3908u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6890u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6892u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5b2758u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5b275au32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5b2744u32)); cpu.w32(0x5b2744u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x449fd9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5b2758u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x5b275au32, v); }
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x44a018; continue; }
  pc = 0x449ff4; continue;
 }
 0x449ff4 => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x44a018; continue; }
  pc = 0x449ffd; continue;
 }
 0x449ffd => {
  { cpu.sub8(cpu.r8(0x4e16b8u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x44a018; continue; }
  pc = 0x44a006; continue;
 }
 0x44a000 => {
  { cpu.push(0x2b); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x44a018; continue; }
  pc = 0x44a006; continue;
 }
 0x44a006 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5b2758u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x5b275au32, v); }
  pc = 0x44a018; continue;
 }
 0x44a018 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x524a5eu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5b2758u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5b275au32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a03c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a03c); }
 let mut pc: u32 = 0x44a03c;
 loop { match pc {
 0x44a03c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a03d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a03d); }
 let mut pc: u32 = 0x44a03d;
 loop { match pc {
 0x44a03d => {
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if cpu.zf { pc = 0x44a051; continue; }
  pc = 0x44a046; continue;
 }
 0x44a046 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  pc = 0x44a07b; continue;
 }
 0x44a051 => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44a06e; continue; }
  pc = 0x44a05f; continue;
 }
 0x44a05f => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.cf || cpu.zf) { pc = 0x44a06e; continue; }
  pc = 0x44a069; continue;
 }
 0x44a069 => {
  { let v: u32 = 0x3cu32; cpu.eax = v; }
  pc = 0x44a06e; continue;
 }
 0x44a06e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15c8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x52u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44a07b; continue;
 }
 0x44a07b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c30u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e3eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e3fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a09f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a09f); }
 let mut pc: u32 = 0x44a09f;
 loop { match pc {
 0x44a09f => {
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c34u32, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e15c8u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15c1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6896u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1e3cu32, v); }
  { cpu.sub8(cpu.r8(0x58c701u32), 0x0u8); }
  if cpu.zf { pc = 0x44a10c; continue; }
  pc = 0x44a105; continue;
 }
 0x44a105 => {
  { let v: u32 = cpu.r32(0x5b2748u32).wrapping_add(0x4u32); cpu.w32(0x5b2748u32, v); }
  pc = 0x44a10c; continue;
 }
 0x44a10c => {
  cpu.push(0x44a111); f_00449e65(cpu); if cpu.returned_to != 0x44a111 { return; }
  cpu.push(0x44a116); f_0044a715(cpu); if cpu.returned_to != 0x44a116 { return; }
  cpu.push(0x44a11b); f_0044a707(cpu); if cpu.returned_to != 0x44a11b { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e16b8u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x44a14a; continue; }
  pc = 0x44a143; continue;
 }
 0x44a143 => {
  { let v: u8 = 0x5u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a176; continue;
 }
 0x44a14a => {
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x44a15b; continue; }
  pc = 0x44a14f; continue;
 }
 0x44a14f => {
  { let v: u8 = 0x15u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a176; continue;
 }
 0x44a15b => {
  { cpu.sub8((cpu.ebx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x44a16c; continue; }
  pc = 0x44a160; continue;
 }
 0x44a160 => {
  { let v: u8 = 0x16u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a176; continue;
 }
 0x44a16c => {
  { let v: u8 = 0x17u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a176; continue;
 }
 0x44a176 => {
  cpu.push(0x44a17b); f_0045b501(cpu); if cpu.returned_to != 0x44a17b { return; }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x44a1d7; continue; }
  pc = 0x44a184; continue;
 }
 0x44a184 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x44a1a4); f_00480006(cpu); if cpu.returned_to != 0x44a1a4 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x44a1a8; continue;
 }
 0x44a1a8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a64u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a1c0; continue; }
  pc = 0x44a1bb; continue;
 }
 0x44a1bb => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a1c5; continue; }
  pc = 0x44a1c0; continue;
 }
 0x44a1c0 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a1c5; continue;
 }
 0x44a1c5 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5b2754u32, v); }
  { let v: u32 = cpu.r32(0x4d6c34u32); cpu.ebx = v; }
  cpu.push(0x44a1d6); f_0044a1de(cpu); if cpu.returned_to != 0x44a1d6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a1d7 => {
  cpu.push(0x44a1dc); f_0047fcdb(cpu); if cpu.returned_to != 0x44a1dc { return; }
  cpu.poll(); pc = 0x44a1a8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a1de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a1de); }
 let mut pc: u32 = 0x44a1de;
 loop { match pc {
 0x44a1de => {
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
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a243(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a243); }
 let mut pc: u32 = 0x44a243;
 loop { match pc {
 0x44a243 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x4e16b8u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c34u32, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if cpu.zf { pc = 0x44a29a; continue; }
  pc = 0x44a297; continue;
 }
 0x44a297 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44a29a; continue;
 }
 0x44a29a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x44a618; continue; }
  pc = 0x44a2ae; continue;
 }
 0x44a2ae => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x44a309; continue; }
  pc = 0x44a2b3; continue;
 }
 0x44a2b3 => {
  { cpu.sub8((cpu.edx as u8), 0xfu8); }
  if cpu.zf { pc = 0x44a3d7; continue; }
  pc = 0x44a2bc; continue;
 }
 0x44a2bc => {
  { cpu.sub8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x44a3ff; continue; }
  pc = 0x44a2c5; continue;
 }
 0x44a2c5 => {
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x44a370; continue; }
  pc = 0x44a2ce; continue;
 }
 0x44a2ce => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if cpu.zf { pc = 0x44a44c; continue; }
  pc = 0x44a2d7; continue;
 }
 0x44a2d7 => {
  { cpu.sub8((cpu.edx as u8), 0xbu8); }
  if cpu.zf { pc = 0x44a4b3; continue; }
  pc = 0x44a2e0; continue;
 }
 0x44a2e0 => {
  { cpu.sub8((cpu.edx as u8), 0xcu8); }
  if cpu.zf { pc = 0x44a521; continue; }
  pc = 0x44a2e9; continue;
 }
 0x44a2e9 => {
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if cpu.zf { pc = 0x44a588; continue; }
  pc = 0x44a2f2; continue;
 }
 0x44a2f2 => {
  { cpu.sub8((cpu.edx as u8), 0x11u8); }
  if cpu.zf { pc = 0x44a440; continue; }
  pc = 0x44a2fb; continue;
 }
 0x44a2fb => {
  { cpu.sub8((cpu.edx as u8), 0x12u8); }
  if cpu.zf { pc = 0x44a434; continue; }
  pc = 0x44a304; continue;
 }
 0x44a304 => {
  f_0044a406(cpu); return;
 }
 0x44a309 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a30f); f_004819ab(cpu); if cpu.returned_to != 0x44a30f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a316; continue;
 }
 0x44a316 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a334); f_0044a1de(cpu); if cpu.returned_to != 0x44a334 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a344; continue; }
  pc = 0x44a342; continue;
 }
 0x44a342 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a344; continue;
 }
 0x44a344 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a364; continue; }
  pc = 0x44a35f; continue;
 }
 0x44a35f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a369; continue; }
  pc = 0x44a364; continue;
 }
 0x44a364 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a369; continue;
 }
 0x44a369 => {
  cpu.push(0x44a36e); f_0045b1b4(cpu); if cpu.returned_to != 0x44a36e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a370 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a376); f_004819ab(cpu); if cpu.returned_to != 0x44a376 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a37d; continue;
 }
 0x44a37d => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a39b); f_0044a1de(cpu); if cpu.returned_to != 0x44a39b { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a3ab; continue; }
  pc = 0x44a3a9; continue;
 }
 0x44a3a9 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a3ab; continue;
 }
 0x44a3ab => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a3cb; continue; }
  pc = 0x44a3c6; continue;
 }
 0x44a3c6 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a3d0; continue; }
  pc = 0x44a3cb; continue;
 }
 0x44a3cb => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a3d0; continue;
 }
 0x44a3d0 => {
  cpu.push(0x44a3d5); f_0045b210(cpu); if cpu.returned_to != 0x44a3d5 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a3d7 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5adc53u32, v); }
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
 0x44a3ff => {
  cpu.push(0x44a404); f_0044a406(cpu); if cpu.returned_to != 0x44a404 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a434 => {
  cpu.push(0x44a439); f_0044a406(cpu); if cpu.returned_to != 0x44a439 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a440 => {
  cpu.push(0x44a445); f_0044a406(cpu); if cpu.returned_to != 0x44a445 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a44c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a452); f_004819ab(cpu); if cpu.returned_to != 0x44a452 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a459; continue;
 }
 0x44a459 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a477); f_0044a1de(cpu); if cpu.returned_to != 0x44a477 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a487; continue; }
  pc = 0x44a485; continue;
 }
 0x44a485 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a487; continue;
 }
 0x44a487 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a4a7; continue; }
  pc = 0x44a4a2; continue;
 }
 0x44a4a2 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a4ac; continue; }
  pc = 0x44a4a7; continue;
 }
 0x44a4a7 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a4ac; continue;
 }
 0x44a4ac => {
  cpu.push(0x44a4b1); f_0045b26c(cpu); if cpu.returned_to != 0x44a4b1 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a4b3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a4b9); f_004819ab(cpu); if cpu.returned_to != 0x44a4b9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a4c0; continue;
 }
 0x44a4c0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a4de); f_0044a1de(cpu); if cpu.returned_to != 0x44a4de { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a4ee; continue; }
  pc = 0x44a4ec; continue;
 }
 0x44a4ec => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a4ee; continue;
 }
 0x44a4ee => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a515; continue; }
  pc = 0x44a510; continue;
 }
 0x44a510 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a51a; continue; }
  pc = 0x44a515; continue;
 }
 0x44a515 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a51a; continue;
 }
 0x44a51a => {
  cpu.push(0x44a51f); f_0045b3a4(cpu); if cpu.returned_to != 0x44a51f { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a521 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a527); f_004819ab(cpu); if cpu.returned_to != 0x44a527 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a52e; continue;
 }
 0x44a52e => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a54c); f_0044a1de(cpu); if cpu.returned_to != 0x44a54c { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a55c; continue; }
  pc = 0x44a55a; continue;
 }
 0x44a55a => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a55c; continue;
 }
 0x44a55c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a57c; continue; }
  pc = 0x44a577; continue;
 }
 0x44a577 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a581; continue; }
  pc = 0x44a57c; continue;
 }
 0x44a57c => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a581; continue;
 }
 0x44a581 => {
  cpu.push(0x44a586); f_0045b400(cpu); if cpu.returned_to != 0x44a586 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a588 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a58e); f_004819ab(cpu); if cpu.returned_to != 0x44a58e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a595; continue;
 }
 0x44a595 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x44a5b3); f_0044a1de(cpu); if cpu.returned_to != 0x44a5b3 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44a5c3; continue; }
  pc = 0x44a5c1; continue;
 }
 0x44a5c1 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44a5c3; continue;
 }
 0x44a5c3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5ba1b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a5ed; continue; }
  pc = 0x44a5e8; continue;
 }
 0x44a5e8 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a5f2; continue; }
  pc = 0x44a5ed; continue;
 }
 0x44a5ed => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a5f2; continue;
 }
 0x44a5f2 => {
  cpu.push(0x44a5f7); f_0045b2c8(cpu); if cpu.returned_to != 0x44a5f7 { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x8au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a618 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44a61e); f_004819ab(cpu); if cpu.returned_to != 0x44a61e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x44a649; continue; }
  pc = 0x44a621; continue;
 }
 0x44a621 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44a62f); f_0044a1de(cpu); if cpu.returned_to != 0x44a62f { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44a63d; continue; }
  pc = 0x44a638; continue;
 }
 0x44a638 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44a642; continue; }
  pc = 0x44a63d; continue;
 }
 0x44a63d => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44a642; continue;
 }
 0x44a642 => {
  cpu.push(0x44a647); f_0045b358(cpu); if cpu.returned_to != 0x44a647 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a649 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a406(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a406); }
 let mut pc: u32 = 0x44a406;
 loop { match pc {
 0x44a406 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44a432); f_0045c73e(cpu); if cpu.returned_to != 0x44a432 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a64b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a64b); }
 let mut pc: u32 = 0x44a64b;
 loop { match pc {
 0x44a64b => {
  cpu.push(0x44a650); f_0044a715(cpu); if cpu.returned_to != 0x44a650 { return; }
  if cpu.cf { pc = 0x44a6cb; continue; }
  pc = 0x44a652; continue;
 }
 0x44a652 => {
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x44a6cc; continue; }
  pc = 0x44a660; continue;
 }
 0x44a660 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x4e1e3eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x4e1e46u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x44a680); f_00480006(cpu); if cpu.returned_to != 0x44a680 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6a1cu32, v); }
  pc = 0x44a68a; continue;
 }
 0x44a68a => {
  cpu.push(0x44a68f); f_0044a707(cpu); if cpu.returned_to != 0x44a68f { return; }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4e1e47u32), 0x0u8); }
  if !cpu.zf { pc = 0x44a6a9; continue; }
  pc = 0x44a69d; continue;
 }
 0x44a69d => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a6a9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44a6ca; continue; }
  pc = 0x44a6b2; continue;
 }
 0x44a6b2 => {
  { cpu.sub8(cpu.r8(0x4d69d8u32), 0x0u8); }
  if cpu.zf { pc = 0x44a6ca; continue; }
  pc = 0x44a6bb; continue;
 }
 0x44a6bb => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x44a6ca; continue; }
  pc = 0x44a6c5; continue;
 }
 0x44a6c5 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a6ca; continue;
 }
 0x44a6ca => {
  { cpu.cf = false; }
  pc = 0x44a6cb; continue;
 }
 0x44a6cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44a6cc => {
  { cpu.sub8(cpu.r8(0x4e1e3fu32), 0x0u8); }
  if !cpu.zf { pc = 0x44a6dc; continue; }
  pc = 0x44a6d5; continue;
 }
 0x44a6d5 => {
  cpu.push(0x44a6da); f_0047fcdb(cpu); if cpu.returned_to != 0x44a6da { return; }
  cpu.poll(); pc = 0x44a68a; continue;
 }
 0x44a6dc => {
  { let v: u32 = cpu.r32(0x4d6a1cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x44a6fe); f_00480006(cpu); if cpu.returned_to != 0x44a6fe { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x44a68a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a707(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a707); }
 let mut pc: u32 = 0x44a707;
 loop { match pc {
 0x44a707 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e155cu32.wrapping_add(cpu.eax))); cpu.w8(0x4e155cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a715(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a715); }
 let mut pc: u32 = 0x44a715;
 loop { match pc {
 0x44a715 => {
  cpu.push(0x44a71a); f_00449b4b(cpu); if cpu.returned_to != 0x44a71a { return; }
  if cpu.cf { pc = 0x44a965; continue; }
  pc = 0x44a720; continue;
 }
 0x44a720 => {
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
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.esi = v; }
  { let v: u32 = cpu.r8(0x4e1e3fu32) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
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
  cpu.push(0x44a835); f_0041c743(cpu); if cpu.returned_to != 0x44a835 { return; }
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
  { let v: u32 = 0x4e9091u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4e93b6u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
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
  cpu.push(0x44a8e5); f_0041b794(cpu); if cpu.returned_to != 0x44a8e5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4eaf68u32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4eaf68u32; cpu.w32(0xa6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4eaf68u32; cpu.w32(0xb2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x87654321u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44a932); f_0046d574(cpu); if cpu.returned_to != 0x44a932 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4e1e47u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44a947); f_0044a966(cpu); if cpu.returned_to != 0x44a947 { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44a964); f_0048bcbb(cpu); if cpu.returned_to != 0x44a964 { return; }
  { cpu.cf = false; }
  pc = 0x44a965; continue;
 }
 0x44a965 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a966(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a966); }
 let mut pc: u32 = 0x44a966;
 loop { match pc {
 0x44a966 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x44a9a0; continue; }
  pc = 0x44a971; continue;
 }
 0x44a971 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44a9a0; continue; }
  pc = 0x44a97a; continue;
 }
 0x44a97a => {
  { cpu.sub8(cpu.r8(0x4e1e46u32), 0x78u8); }
  if !cpu.zf { pc = 0x44a9a0; continue; }
  pc = 0x44a983; continue;
 }
 0x44a983 => {
  { let v: u32 = cpu.r32(0x5b274cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x78u32); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, 0xau32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x524a5eu32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44a9a0; continue;
 }
 0x44a9a0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044a9a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44a9a4); }
 let mut pc: u32 = 0x44a9a4;
 loop { match pc {
 0x44a9a4 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44aa29; continue; }
  pc = 0x44a9ab; continue;
 }
 0x44a9ab => {
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xf4u8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x44a9fb; continue; }
  pc = 0x44a9bd; continue;
 }
 0x44a9bd => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44aa44; continue; }
  pc = 0x44a9c7; continue;
 }
 0x44a9c7 => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x44a9fb; continue; }
  pc = 0x44a9d1; continue;
 }
 0x44a9d1 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x22u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x44a9fb; continue; }
  pc = 0x44a9f0; continue;
 }
 0x44a9f0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0x20u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44a9fb; continue;
 }
 0x44a9fb => {
  cpu.push(0x44aa00); f_0044aa9c(cpu); if cpu.returned_to != 0x44aa00 { return; }
  if cpu.cf { pc = 0x44aa44; continue; }
  pc = 0x44aa02; continue;
 }
 0x44aa02 => {
  cpu.push(0x44aa07); f_0045c2d0(cpu); if cpu.returned_to != 0x44aa07 { return; }
  cpu.push(0x44aa0c); f_0044aa45(cpu); if cpu.returned_to != 0x44aa0c { return; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x44aa22; continue; }
  pc = 0x44aa20; continue;
 }
 0x44aa20 => {
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x44aa22; continue;
 }
 0x44aa22 => {
  cpu.poll(); pc = cpu.r32(0x5b275cu32.wrapping_add(cpu.ebx.wrapping_mul(2))); continue;
 }
 0x44aa29 => {
  { cpu.sub8(cpu.r8(0x5cf455u32), 0x0u8); }
  if cpu.zf { pc = 0x44aa44; continue; }
  pc = 0x44aa32; continue;
 }
 0x44aa32 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44aa38); f_0044bc7b(cpu); if cpu.returned_to != 0x44aa38 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44a9a4; continue; }
  pc = 0x44aa44; continue;
 }
 0x44aa44 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044aa45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44aa45); }
 let mut pc: u32 = 0x44aa45;
 loop { match pc {
 0x44aa45 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44aa9b; continue; }
  pc = 0x44aa4c; continue;
 }
 0x44aa4c => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc04u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44aa5e; continue;
 }
 0x44aa5e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44aa95; continue; }
  pc = 0x44aa63; continue;
 }
 0x44aa63 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x44aa7e; continue; }
  pc = 0x44aa77; continue;
 }
 0x44aa77 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x44aa95; continue; }
  pc = 0x44aa7e; continue;
 }
 0x44aa7e => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x44aa95; continue; }
  pc = 0x44aa87; continue;
 }
 0x44aa87 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44aa93); f_0043ffdb(cpu); if cpu.returned_to != 0x44aa93 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x44aa95; continue;
 }
 0x44aa95 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44aa5e; continue; }
  pc = 0x44aa9a; continue;
 }
 0x44aa9a => {
  { cpu.popad(); }
  pc = 0x44aa9b; continue;
 }
 0x44aa9b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044aa9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44aa9c); }
 let mut pc: u32 = 0x44aa9c;
 loop { match pc {
 0x44aa9c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  cpu.push(0x44aaa8); f_0045c21f(cpu); if cpu.returned_to != 0x44aaa8 { return; }
  if cpu.cf { pc = 0x44aaba; continue; }
  pc = 0x44aaaa; continue;
 }
 0x44aaaa => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  cpu.push(0x44aab5); f_0045c271(cpu); if cpu.returned_to != 0x44aab5 { return; }
  if cpu.cf { pc = 0x44aaba; continue; }
  pc = 0x44aab7; continue;
 }
 0x44aab7 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44aaba => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044aabd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44aabd); }
 let mut pc: u32 = 0x44aabd;
 loop { match pc {
 0x44aabd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044aabe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44aabe); }
 let mut pc: u32 = 0x44aabe;
 loop { match pc {
 0x44aabe => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44aac9; continue;
 }
 0x44aac9 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44aad4; continue;
 }
 0x44aad4 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44aadf; continue;
 }
 0x44aadf => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44aaea; continue;
 }
 0x44aaea => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44aaf5; continue;
 }
 0x44aaf5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab00; continue;
 }
 0x44ab00 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab0b; continue;
 }
 0x44ab0b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab12; continue;
 }
 0x44ab12 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab19; continue;
 }
 0x44ab19 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab20; continue;
 }
 0x44ab20 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab27; continue;
 }
 0x44ab27 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab2e; continue;
 }
 0x44ab2e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x68u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x6au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44ab52); f_00481755(cpu); if cpu.returned_to != 0x44ab52 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5928d2u32), 0x0u8); }
  if cpu.zf { pc = 0x44ab63; continue; }
  pc = 0x44ab5f; continue;
 }
 0x44ab5f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x44ab68; continue;
 }
 0x44ab63 => {
  cpu.push(0x44ab68); f_0046d525(cpu); if cpu.returned_to != 0x44ab68 { return; }
  pc = 0x44ab68; continue;
 }
 0x44ab68 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44ab78; continue; }
  pc = 0x44ab71; continue;
 }
 0x44ab71 => {
  cpu.push(0x44ab76); f_0044af41(cpu); if cpu.returned_to != 0x44ab76 { return; }
  pc = 0x44ab8e; continue;
 }
 0x44ab78 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4au8); }
  if cpu.zf { pc = 0x44ab8e; continue; }
  pc = 0x44ab7f; continue;
 }
 0x44ab7f => {
  { let v: u8 = 0x4au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ab8e; continue;
 }
 0x44ab8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ab8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ab8f); }
 let mut pc: u32 = 0x44ab8f;
 loop { match pc {
 0x44ab8f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44ab9a; continue;
 }
 0x44ab9a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44aba5; continue;
 }
 0x44aba5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abb0; continue;
 }
 0x44abb0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abbb; continue;
 }
 0x44abbb => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abc6; continue;
 }
 0x44abc6 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abd1; continue;
 }
 0x44abd1 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abdc; continue;
 }
 0x44abdc => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abe3; continue;
 }
 0x44abe3 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abea; continue;
 }
 0x44abea => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abf1; continue;
 }
 0x44abf1 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abf8; continue;
 }
 0x44abf8 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44abff; continue;
 }
 0x44abff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x68u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x6au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44ac23); f_00481755(cpu); if cpu.returned_to != 0x44ac23 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5928d2u32), 0x0u8); }
  if cpu.zf { pc = 0x44ac34; continue; }
  pc = 0x44ac30; continue;
 }
 0x44ac30 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x44ac39; continue;
 }
 0x44ac34 => {
  cpu.push(0x44ac39); f_0046d525(cpu); if cpu.returned_to != 0x44ac39 { return; }
  pc = 0x44ac39; continue;
 }
 0x44ac39 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44ac49; continue; }
  pc = 0x44ac42; continue;
 }
 0x44ac42 => {
  cpu.push(0x44ac47); f_0044af41(cpu); if cpu.returned_to != 0x44ac47 { return; }
  pc = 0x44ac5f; continue;
 }
 0x44ac49 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x49u8); }
  if cpu.zf { pc = 0x44ac5f; continue; }
  pc = 0x44ac50; continue;
 }
 0x44ac50 => {
  { let v: u8 = 0x49u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ac5f; continue;
 }
 0x44ac5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ad0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ad0e); }
 let mut pc: u32 = 0x44ad0e;
 loop { match pc {
 0x44ad0e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad19; continue;
 }
 0x44ad19 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad20; continue;
 }
 0x44ad20 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad27; continue;
 }
 0x44ad27 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad2e; continue;
 }
 0x44ad2e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad35; continue;
 }
 0x44ad35 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad3c; continue;
 }
 0x44ad3c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad43; continue;
 }
 0x44ad43 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad4a; continue;
 }
 0x44ad4a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad51; continue;
 }
 0x44ad51 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad58; continue;
 }
 0x44ad58 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad5f; continue;
 }
 0x44ad5f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad66; continue;
 }
 0x44ad66 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x68u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x6au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44ad85); f_00481755(cpu); if cpu.returned_to != 0x44ad85 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x15u8); }
  if cpu.zf { pc = 0x44ad9f; continue; }
  pc = 0x44ad90; continue;
 }
 0x44ad90 => {
  { let v: u8 = 0x15u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ad9f; continue;
 }
 0x44ad9f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b2798u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ae4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ae4d); }
 let mut pc: u32 = 0x44ae4d;
 loop { match pc {
 0x44ae4d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae58; continue;
 }
 0x44ae58 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae63; continue;
 }
 0x44ae63 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae6e; continue;
 }
 0x44ae6e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae79; continue;
 }
 0x44ae79 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae84; continue;
 }
 0x44ae84 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae8f; continue;
 }
 0x44ae8f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44ae9a; continue;
 }
 0x44ae9a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aea5; continue;
 }
 0x44aea5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aeb0; continue;
 }
 0x44aeb0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aebb; continue;
 }
 0x44aebb => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aec2; continue;
 }
 0x44aec2 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aec9; continue;
 }
 0x44aec9 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44aed0; continue;
 }
 0x44aed0 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x68u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x6au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x44aef8); f_00481755(cpu); if cpu.returned_to != 0x44aef8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5928d2u32), 0x0u8); }
  if cpu.zf { pc = 0x44af0c; continue; }
  pc = 0x44af05; continue;
 }
 0x44af05 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x44af11; continue;
 }
 0x44af0c => {
  cpu.push(0x44af11); f_0046d525(cpu); if cpu.returned_to != 0x44af11 { return; }
  pc = 0x44af11; continue;
 }
 0x44af11 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44af21; continue; }
  pc = 0x44af1a; continue;
 }
 0x44af1a => {
  cpu.push(0x44af1f); f_0044af41(cpu); if cpu.returned_to != 0x44af1f { return; }
  pc = 0x44af40; continue;
 }
 0x44af21 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x14u8); }
  if cpu.zf { pc = 0x44af40; continue; }
  pc = 0x44af28; continue;
 }
 0x44af28 => {
  { let v: u8 = 0x14u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44af40; continue;
 }
 0x44af40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044af41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44af41); }
 let mut pc: u32 = 0x44af41;
 loop { match pc {
 0x44af41 => {
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x44af50; continue; }
  pc = 0x44af4b; continue;
 }
 0x44af4b => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44af50 => {
  { let v: u8 = 0x42u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x44af7a); f_00481755(cpu); if cpu.returned_to != 0x44af7a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b2798u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044af86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44af86); }
 let mut pc: u32 = 0x44af86;
 loop { match pc {
 0x44ac60 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x44ac66; continue;
 }
 0x44ac66 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44ac87; continue; }
  pc = 0x44ac6b; continue;
 }
 0x44ac6b => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44ac82); f_0044ab8f(cpu); if cpu.returned_to != 0x44ac82 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44ac87; continue;
 }
 0x44ac87 => {
  { cpu.sub8(cpu.r8(0x5b2798u32), 0x0u8); }
  if !cpu.zf { pc = 0x44ac93; continue; }
  pc = 0x44ac90; continue;
 }
 0x44ac90 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44ac66; continue; }
  pc = 0x44ac93; continue;
 }
 0x44ac93 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ac9a => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x44aca0; continue;
 }
 0x44aca0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44acc1; continue; }
  pc = 0x44aca5; continue;
 }
 0x44aca5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44acbc); f_0044aabe(cpu); if cpu.returned_to != 0x44acbc { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44acc1; continue;
 }
 0x44acc1 => {
  { cpu.sub8(cpu.r8(0x5b2798u32), 0x0u8); }
  if !cpu.zf { pc = 0x44accd; continue; }
  pc = 0x44acca; continue;
 }
 0x44acca => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44aca0; continue; }
  pc = 0x44accd; continue;
 }
 0x44accd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44acd4 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x44acda; continue;
 }
 0x44acda => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44acfb; continue; }
  pc = 0x44acdf; continue;
 }
 0x44acdf => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44acf6); f_0044ad0e(cpu); if cpu.returned_to != 0x44acf6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44acfb; continue;
 }
 0x44acfb => {
  { cpu.sub8(cpu.r8(0x5b2798u32), 0x0u8); }
  if !cpu.zf { pc = 0x44ad07; continue; }
  pc = 0x44ad04; continue;
 }
 0x44ad04 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44acda; continue; }
  pc = 0x44ad07; continue;
 }
 0x44ad07 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ada7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b2798u32, v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x44ae40; continue; }
  pc = 0x44adb9; continue;
 }
 0x44adb9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x44acd4; continue; }
  pc = 0x44addc; continue;
 }
 0x44addc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x44ac60; continue; }
  pc = 0x44ade7; continue;
 }
 0x44ade7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { cpu.poll(); pc = 0x44ac9a; continue; }
  pc = 0x44adf2; continue;
 }
 0x44adf2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5928d2u32, v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x44ae12; continue; }
  pc = 0x44ae03; continue;
 }
 0x44ae03 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5928d2u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ae12; continue;
 }
 0x44ae12 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x44ae18; continue;
 }
 0x44ae18 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44ae33; continue; }
  pc = 0x44ae1d; continue;
 }
 0x44ae1d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44ae31); f_0044ae4d(cpu); if cpu.returned_to != 0x44ae31 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x44ae33; continue;
 }
 0x44ae33 => {
  { cpu.sub8(cpu.r8(0x5b2798u32), 0x0u8); }
  if !cpu.zf { pc = 0x44ae3f; continue; }
  pc = 0x44ae3c; continue;
 }
 0x44ae3c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44ae18; continue; }
  pc = 0x44ae3f; continue;
 }
 0x44ae3f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x44ae40; continue;
 }
 0x44ae40 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928d2u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44af86 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x44ada7; continue; }
  pc = 0x44af96; continue;
 }
 0x44af96 => {
  { let v: u8 = cpu.or8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44afa0); f_0044b9aa(cpu); if cpu.returned_to != 0x44afa0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044afde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44afde); }
 let mut pc: u32 = 0x44afde;
 loop { match pc {
 0x44afde => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044afdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44afdf); }
 let mut pc: u32 = 0x44afdf;
 loop { match pc {
 0x44afdf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044afe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44afe0); }
 let mut pc: u32 = 0x44afe0;
 loop { match pc {
 0x44afe0 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044afe2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44afe2); }
 let mut pc: u32 = 0x44afe2;
 loop { match pc {
 0x44afe2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044afe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44afe4); }
 let mut pc: u32 = 0x44afe4;
 loop { match pc {
 0x44afe4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b256); }
 let mut pc: u32 = 0x44b256;
 loop { match pc {
 0x44b256 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x44b2a8; continue; }
  pc = 0x44b263; continue;
 }
 0x44b263 => {
  cpu.push(0x44b268); f_0047315f(cpu); if cpu.returned_to != 0x44b268 { return; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0xau16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x8au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44b28e; continue; }
  pc = 0x44b281; continue;
 }
 0x44b281 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44b28e; continue;
 }
 0x44b28e => {
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44b2a8; continue;
 }
 0x44b2a8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b2aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b2aa); }
 let mut pc: u32 = 0x44b2aa;
 loop { match pc {
 0x44b2aa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44b2bb); f_004712e3(cpu); if cpu.returned_to != 0x44b2bb { return; }
  if !cpu.cf { pc = 0x44b2d1; continue; }
  pc = 0x44b2bd; continue;
 }
 0x44b2bd => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x44b2d1; continue; }
  pc = 0x44b2c9; continue;
 }
 0x44b2c9 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  cpu.push(0x44b2d1); f_00458a7f(cpu); if cpu.returned_to != 0x44b2d1 { return; }
  pc = 0x44b2d1; continue;
 }
 0x44b2d1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b2d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b2d3); }
 let mut pc: u32 = 0x44b2d3;
 loop { match pc {
 0x44b2d3 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x44b2f5; continue; }
  pc = 0x44b2db; continue;
 }
 0x44b2db => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44b2eb); f_004713ce(cpu); if cpu.returned_to != 0x44b2eb { return; }
  if !cpu.cf { pc = 0x44b2f5; continue; }
  pc = 0x44b2ed; continue;
 }
 0x44b2ed => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  cpu.push(0x44b2f5); f_00458a7f(cpu); if cpu.returned_to != 0x44b2f5 { return; }
  pc = 0x44b2f5; continue;
 }
 0x44b2f5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b776(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b776); }
 let mut pc: u32 = 0x44b776;
 loop { match pc {
 0x44b776 => {
  { let v: u16 = cpu.r16(0x57556fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x575571u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44b788); f_004713ce(cpu); if cpu.returned_to != 0x44b788 { return; }
  if cpu.cf { pc = 0x44b796; continue; }
  pc = 0x44b78a; continue;
 }
 0x44b78a => {
  cpu.push(0x44b78f); f_0046e362(cpu); if cpu.returned_to != 0x44b78f { return; }
  if cpu.cf { pc = 0x44b796; continue; }
  pc = 0x44b791; continue;
 }
 0x44b791 => {
  cpu.push(0x44b796); f_00416876(cpu); if cpu.returned_to != 0x44b796 { return; }
  pc = 0x44b796; continue;
 }
 0x44b796 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b797(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b797); }
 let mut pc: u32 = 0x44b797;
 loop { match pc {
 0x44b797 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x44b7c6); f_0045955b(cpu); if cpu.returned_to != 0x44b7c6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x44b7cc); f_00489637(cpu); if cpu.returned_to != 0x44b7cc { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { f_0044b9aa(cpu); return; }
  pc = 0x44b7d3; continue;
 }
 0x44b7d3 => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b7dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b7dd); }
 let mut pc: u32 = 0x44b7dd;
 loop { match pc {
 0x44b7dd => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x44b85d; continue; }
  pc = 0x44b7e7; continue;
 }
 0x44b7e7 => {
  cpu.push(0x44b7ec); f_00451959(cpu); if cpu.returned_to != 0x44b7ec { return; }
  if cpu.cf { pc = 0x44b849; continue; }
  pc = 0x44b7ee; continue;
 }
 0x44b7ee => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44b849; continue; }
  pc = 0x44b7f5; continue;
 }
 0x44b7f5 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x44b85d; continue; }
  pc = 0x44b7fb; continue;
 }
 0x44b7fb => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xc350u32); }
  if (cpu.sf != cpu.of) { pc = 0x44b849; continue; }
  pc = 0x44b82b; continue;
 }
 0x44b82b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x1388u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.cf || cpu.zf) { pc = 0x44b843; continue; }
  pc = 0x44b83e; continue;
 }
 0x44b83e => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  pc = 0x44b843; continue;
 }
 0x44b843 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44b849 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (cpu.sf == cpu.of) { pc = 0x44b857; continue; }
  pc = 0x44b855; continue;
 }
 0x44b855 => {
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44b857; continue;
 }
 0x44b857 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44b85d => {
  { let v: u8 = 0x7fu8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b864); }
 let mut pc: u32 = 0x44b864;
 loop { match pc {
 0x44b864 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x44b8a2; continue; }
  pc = 0x44b875; continue;
 }
 0x44b875 => {
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44b8a2; continue; }
  pc = 0x44b87a; continue;
 }
 0x44b87a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x44b890); f_0044b938(cpu); if cpu.returned_to != 0x44b890 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44b8ad; continue; }
  pc = 0x44b897; continue;
 }
 0x44b897 => {
  cpu.push(0x44b89c); f_00472f42(cpu); if cpu.returned_to != 0x44b89c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x44b8a2; continue;
 }
 0x44b8a2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44b8ad => {
  cpu.push(0x44b8b2); f_00458429(cpu); if cpu.returned_to != 0x44b8b2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x44b8bd); f_00489968(cpu); if cpu.returned_to != 0x44b8bd { return; }
  { let v: u32 = 0x9u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x44b929; continue; }
  pc = 0x44b8c9; continue;
 }
 0x44b8c9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x44b8fa; continue; }
  pc = 0x44b8d0; continue;
 }
 0x44b8d0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x44b90f; continue; }
  pc = 0x44b8d7; continue;
 }
 0x44b8d7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x44b924; continue; }
  pc = 0x44b8de; continue;
 }
 0x44b8de => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x44b916; continue; }
  pc = 0x44b8e5; continue;
 }
 0x44b8e5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44b901; continue; }
  pc = 0x44b8ec; continue;
 }
 0x44b8ec => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x44b908; continue; }
  pc = 0x44b8f3; continue;
 }
 0x44b8f3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44b91d; continue; }
  pc = 0x44b8fa; continue;
 }
 0x44b8fa => {
  { let v: u32 = 0x36u32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b901 => {
  { let v: u32 = 0x3au32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b908 => {
  { let v: u32 = 0x3bu32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b90f => {
  { let v: u32 = 0x37u32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b916 => {
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b91d => {
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b924 => {
  { let v: u32 = 0x35u32; cpu.ebx = v; }
  pc = 0x44b929; continue;
 }
 0x44b929 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b938); }
 let mut pc: u32 = 0x44b938;
 loop { match pc {
 0x44b938 => {
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x54u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x44b95b; continue; }
  pc = 0x44b956; continue;
 }
 0x44b956 => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x44b95b; continue;
 }
 0x44b95b => {
  { let v: u8 = 0x5u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b96a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b96a); }
 let mut pc: u32 = 0x44b96a;
 loop { match pc {
 0x44b96a => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44b984; continue; }
  pc = 0x44b97f; continue;
 }
 0x44b97f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44b989; continue; }
  pc = 0x44b984; continue;
 }
 0x44b984 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44b989; continue;
 }
 0x44b989 => {
  { let v: u8 = 0xffu8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b9a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b9a7); }
 let mut pc: u32 = 0x44b9a7;
 loop { match pc {
 0x44b9a7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044b9aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44b9aa); }
 let mut pc: u32 = 0x44b9aa;
 loop { match pc {
 0x44b9aa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x44b9bb; continue; }
  pc = 0x44b9b9; continue;
 }
 0x44b9b9 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44b9bb; continue;
 }
 0x44b9bb => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44b9d7; continue; }
  pc = 0x44b9bf; continue;
 }
 0x44b9bf => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44ba06; continue; }
  pc = 0x44b9c3; continue;
 }
 0x44b9c3 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44ba06; continue; }
  pc = 0x44b9c7; continue;
 }
 0x44b9c7 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44b9cd; continue; }
  pc = 0x44b9cb; continue;
 }
 0x44b9cb => {
  pc = 0x44ba11; continue;
 }
 0x44b9cd => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ba9e; continue;
 }
 0x44b9d7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x50u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x44ba9e; continue; }
  pc = 0x44b9eb; continue;
 }
 0x44b9eb => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x52u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x44ba9e; continue; }
  pc = 0x44b9ff; continue;
 }
 0x44b9ff => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ba11; continue;
 }
 0x44ba06 => {
  cpu.push(0x44ba0b); f_0044baa0(cpu); if cpu.returned_to != 0x44ba0b { return; }
  if cpu.cf { pc = 0x44ba9e; continue; }
  pc = 0x44ba11; continue;
 }
 0x44ba11 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44ba9e; continue; }
  pc = 0x44ba1c; continue;
 }
 0x44ba1c => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ba9e; continue; }
  pc = 0x44ba23; continue;
 }
 0x44ba23 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44ba38; continue; }
  pc = 0x44ba2a; continue;
 }
 0x44ba2a => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44ba69; continue; }
  pc = 0x44ba2e; continue;
 }
 0x44ba2e => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44ba69; continue; }
  pc = 0x44ba32; continue;
 }
 0x44ba32 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44ba69; continue; }
  pc = 0x44ba36; continue;
 }
 0x44ba36 => {
  pc = 0x44ba9e; continue;
 }
 0x44ba38 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x44ba70; continue; }
  pc = 0x44ba47; continue;
 }
 0x44ba47 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x44ba70; continue; }
  pc = 0x44ba56; continue;
 }
 0x44ba56 => {
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44ba63); f_0041b794(cpu); if cpu.returned_to != 0x44ba63 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  pc = 0x44ba9e; continue;
 }
 0x44ba69 => {
  cpu.push(0x44ba6e); f_0044bb60(cpu); if cpu.returned_to != 0x44ba6e { return; }
  if !cpu.cf { pc = 0x44ba9e; continue; }
  pc = 0x44ba70; continue;
 }
 0x44ba70 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44ba9e; continue; }
  pc = 0x44ba77; continue;
 }
 0x44ba77 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44ba7f); f_00454265(cpu); if cpu.returned_to != 0x44ba7f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ba9e; continue;
 }
 0x44ba9e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044baa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44baa0); }
 let mut pc: u32 = 0x44baa0;
 loop { match pc {
 0x44baa0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44bafb; continue; }
  pc = 0x44baa7; continue;
 }
 0x44baa7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44bafb; continue; }
  pc = 0x44baae; continue;
 }
 0x44baae => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x44bacc; continue; }
  pc = 0x44bab7; continue;
 }
 0x44bab7 => {
  { cpu.pushad(); }
  cpu.push(0x44babd); f_00451959(cpu); if cpu.returned_to != 0x44babd { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44bb57; continue; }
  pc = 0x44baca; continue;
 }
 0x44baca => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bacc => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44badc); f_0046e31e(cpu); if cpu.returned_to != 0x44badc { return; }
  if !cpu.cf { pc = 0x44bae2; continue; }
  pc = 0x44bade; continue;
 }
 0x44bade => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44baf4; continue;
 }
 0x44bae2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44bae9); f_004481e0(cpu); if cpu.returned_to != 0x44bae9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x44baef; continue; }
  pc = 0x44baed; continue;
 }
 0x44baed => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44baef => {
  cpu.push(0x44baf4); f_0042e0fa(cpu); if cpu.returned_to != 0x44baf4 { return; }
  pc = 0x44baf4; continue;
 }
 0x44baf4 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44baf9; continue;
 }
 0x44baf9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bafb => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xfb2eu16); }
  if cpu.zf { pc = 0x44bb3d; continue; }
  pc = 0x44bb04; continue;
 }
 0x44bb04 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xef1fu16); }
  if cpu.zf { pc = 0x44bb23; continue; }
  pc = 0x44bb0d; continue;
 }
 0x44bb0d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44bb1d); f_0042e0fa(cpu); if cpu.returned_to != 0x44bb1d { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44bb22; continue;
 }
 0x44bb22 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bb23 => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x509290u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44bb34; continue;
 }
 0x44bb34 => {
  cpu.push(0x44bb39); f_0044be78(cpu); if cpu.returned_to != 0x44bb39 { return; }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44bb3b; continue;
 }
 0x44bb3b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bb3d => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x52bbf0u32); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44bb4e; continue;
 }
 0x44bb4e => {
  cpu.push(0x44bb53); f_0044beac(cpu); if cpu.returned_to != 0x44bb53 { return; }
  if cpu.zf { pc = 0x44bb57; continue; }
  pc = 0x44bb55; continue;
 }
 0x44bb55 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bb57 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5au32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bb60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bb60); }
 let mut pc: u32 = 0x44bb60;
 loop { match pc {
 0x44bb60 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44bc13; continue; }
  pc = 0x44bb6b; continue;
 }
 0x44bb6b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44bc13; continue; }
  pc = 0x44bb76; continue;
 }
 0x44bb76 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xd80u16); }
  if cpu.zf { pc = 0x44bbbe; continue; }
  pc = 0x44bb7e; continue;
 }
 0x44bb7e => {
  { let v = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x44bb9a); f_00451959(cpu); if cpu.returned_to != 0x44bb9a { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x44bc68; continue; }
  pc = 0x44bbab; continue;
 }
 0x44bbab => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.edx = v; }
  cpu.push(0x44bbb6); f_0044be78(cpu); if cpu.returned_to != 0x44bbb6 { return; }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bbbc; continue;
 }
 0x44bbbc => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bbbe => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44bbcc); f_0046e31e(cpu); if cpu.returned_to != 0x44bbcc { return; }
  if !cpu.cf { pc = 0x44bbd2; continue; }
  pc = 0x44bbce; continue;
 }
 0x44bbce => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44bc0c; continue;
 }
 0x44bbd2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44bbd9); f_004481e0(cpu); if cpu.returned_to != 0x44bbd9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x44bbdf; continue; }
  pc = 0x44bbdd; continue;
 }
 0x44bbdd => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bbdf => {
  cpu.push(0x44bbe4); f_00420989(cpu); if cpu.returned_to != 0x44bbe4 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x17u16); }
  if !cpu.zf { pc = 0x44bbf7; continue; }
  pc = 0x44bbf3; continue;
 }
 0x44bbf3 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44bc0c; continue;
 }
 0x44bbf7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x44bc06; continue; }
  pc = 0x44bbfe; continue;
 }
 0x44bbfe => {
  { let v: u8 = cpu.r8(0x4e9f57u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44bc0c; continue;
 }
 0x44bc06 => {
  { let v: u8 = cpu.r8(0x4e9f14u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44bc0c; continue;
 }
 0x44bc0c => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bc11; continue;
 }
 0x44bc11 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bc13 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x44bc4f; continue; }
  pc = 0x44bc1b; continue;
 }
 0x44bc1b => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if cpu.zf { pc = 0x44bc36; continue; }
  pc = 0x44bc23; continue;
 }
 0x44bc23 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44bc31); f_00453a11(cpu); if cpu.returned_to != 0x44bc31 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { pc = 0x44bc68; continue; }
  pc = 0x44bc34; continue;
 }
 0x44bc34 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bc36 => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x509290u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bc46; continue;
 }
 0x44bc46 => {
  cpu.push(0x44bc4b); f_0044be78(cpu); if cpu.returned_to != 0x44bc4b { return; }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bc4d; continue;
 }
 0x44bc4d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bc4f => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x52bbf0u32); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bc5f; continue;
 }
 0x44bc5f => {
  cpu.push(0x44bc64); f_0044beac(cpu); if cpu.returned_to != 0x44bc64 { return; }
  if cpu.zf { pc = 0x44bc68; continue; }
  pc = 0x44bc66; continue;
 }
 0x44bc66 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bc68 => {
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44bc75); f_0041b794(cpu); if cpu.returned_to != 0x44bc75 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bc7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bc7b); }
 let mut pc: u32 = 0x44bc7b;
 loop { match pc {
 0x44bc7b => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44bcd1; continue; }
  pc = 0x44bc87; continue;
 }
 0x44bc87 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44bcd1; continue; }
  pc = 0x44bc8e; continue;
 }
 0x44bc8e => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xcu8); }
  if cpu.zf { pc = 0x44bc9c; continue; }
  pc = 0x44bc95; continue;
 }
 0x44bc95 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x44bcd1; continue; }
  pc = 0x44bc9c; continue;
 }
 0x44bc9c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x44bcaa; continue; }
  pc = 0x44bca3; continue;
 }
 0x44bca3 => {
  cpu.push(0x44bca8); f_0046b31b(cpu); if cpu.returned_to != 0x44bca8 { return; }
  if cpu.zf { pc = 0x44bcd1; continue; }
  pc = 0x44bcaa; continue;
 }
 0x44bcaa => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44bcb2); f_00454265(cpu); if cpu.returned_to != 0x44bcb2 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44bcd1; continue;
 }
 0x44bcd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bcd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bcd2); }
 let mut pc: u32 = 0x44bcd2;
 loop { match pc {
 0x44bcd2 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xf4u8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44bced); f_0041b794(cpu); if cpu.returned_to != 0x44bced { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44bd0b; continue; }
  pc = 0x44bcfd; continue;
 }
 0x44bcfd => {
  cpu.push(0x44bd02); f_0045c3c4(cpu); if cpu.returned_to != 0x44bd02 { return; }
  if cpu.cf { pc = 0x44bd08; continue; }
  pc = 0x44bd04; continue;
 }
 0x44bd04 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bd08 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bd0b => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b9d0cu32, v); }
  cpu.push(0x44bd1a); f_0044b256(cpu); if cpu.returned_to != 0x44bd1a { return; }
  cpu.push(0x44bd1f); f_0043ffdb(cpu); if cpu.returned_to != 0x44bd1f { return; }
  { cpu.sub32(cpu.r32(0x5b9d0cu32), 0x0u32); }
  if cpu.zf { pc = 0x44bd41; continue; }
  pc = 0x44bd28; continue;
 }
 0x44bd28 => {
  { let v: u32 = cpu.r32(0x5b9d0cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x559754u32); cpu.eax = v; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x99u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44bd41 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = (cpu.eax as u8); cpu.div8(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bd47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bd47); }
 let mut pc: u32 = 0x44bd47;
 loop { match pc {
 0x44bd47 => {
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x44bd61; continue; }
  pc = 0x44bd56; continue;
 }
 0x44bd56 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44bd5f); f_00456b9e(cpu); if cpu.returned_to != 0x44bd5f { return; }
  pc = 0x44bd92; continue;
 }
 0x44bd61 => {
  { cpu.sub16((cpu.edx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x44bd72; continue; }
  pc = 0x44bd67; continue;
 }
 0x44bd67 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44bd70); f_00456ca1(cpu); if cpu.returned_to != 0x44bd70 { return; }
  pc = 0x44bd92; continue;
 }
 0x44bd72 => {
  { cpu.sub16((cpu.edx as u16), 0x2u16); }
  if !cpu.zf { pc = 0x44bd83; continue; }
  pc = 0x44bd78; continue;
 }
 0x44bd78 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44bd81); f_00456da6(cpu); if cpu.returned_to != 0x44bd81 { return; }
  pc = 0x44bd92; continue;
 }
 0x44bd83 => {
  { cpu.sub16((cpu.edx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x44bd92; continue; }
  pc = 0x44bd89; continue;
 }
 0x44bd89 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x44bd92); f_00456ea4(cpu); if cpu.returned_to != 0x44bd92 { return; }
  pc = 0x44bd92; continue;
 }
 0x44bd92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bd93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bd93); }
 let mut pc: u32 = 0x44bd93;
 loop { match pc {
 0x44bd93 => {
  { let v: u32 = cpu.r16(0x4d6796u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6798u32) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4d68b0u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if cpu.zf { pc = 0x44bde0; continue; }
  pc = 0x44bdac; continue;
 }
 0x44bdac => {
  { let v: u32 = cpu.r16(0x4d6792u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6794u32) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x44bde0; continue; }
  pc = 0x44bdbf; continue;
 }
 0x44bdbf => {
  { let v: u32 = cpu.r16(0x4d679au32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d679cu32) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x44bde0; continue; }
  pc = 0x44bdd2; continue;
 }
 0x44bdd2 => {
  { let v: u32 = cpu.r16(0x4d679eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d67a0u32) as u32; cpu.ebx = v; }
  pc = 0x44bde0; continue;
 }
 0x44bde0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x48u32); cpu.ebx = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4d68aeu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44bdf6; continue;
 }
 0x44bdf6 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x44be68; continue; }
  pc = 0x44bdff; continue;
 }
 0x44bdff => {
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x44be68; continue; }
  pc = 0x44be06; continue;
 }
 0x44be06 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44be68; continue; }
  pc = 0x44be0d; continue;
 }
 0x44be0d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44be68; continue; }
  pc = 0x44be14; continue;
 }
 0x44be14 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44be3e); f_0044be72(cpu); if cpu.returned_to != 0x44be3e { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.edi), v); }
  pc = 0x44be68; continue;
 }
 0x44be68 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44bdf6; continue; }
  pc = 0x44be71; continue;
 }
 0x44be71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044be78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44be78); }
 let mut pc: u32 = 0x44be78;
 loop { match pc {
 0x44be78 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44bea5; continue; }
  pc = 0x44be91; continue;
 }
 0x44be91 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x44bea7; continue; }
  pc = 0x44bea5; continue;
 }
 0x44bea5 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  pc = 0x44bea7; continue;
 }
 0x44bea7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044beac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44beac); }
 let mut pc: u32 = 0x44beac;
 loop { match pc {
 0x44beac => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44bee8; continue; }
  pc = 0x44bed4; continue;
 }
 0x44bed4 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x44beea; continue; }
  pc = 0x44bee8; continue;
 }
 0x44bee8 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  pc = 0x44beea; continue;
 }
 0x44beea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044bef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44bef0); }
 let mut pc: u32 = 0x44bef0;
 loop { match pc {
 0x44bef0 => {
  { let v: u32 = 0x534c50u32; cpu.edi = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044beff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44beff); } r_0044beff(cpu, 0x44beff); }
pub fn f_0044c000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c000); } r_0044beff(cpu, 0x44c000); }
fn r_0044beff(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44beff => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44bf09); f_0044c7f9(cpu); if cpu.returned_to != 0x44bf09 { return; }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44bf0f; continue;
 }
 0x44bf0f => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x44c163; continue; }
  pc = 0x44bf17; continue;
 }
 0x44bf17 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44bf4c); f_004714a5(cpu); if cpu.returned_to != 0x44bf4c { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44bf53; continue;
 }
 0x44bf53 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44c014; continue; }
  pc = 0x44bf60; continue;
 }
 0x44bf60 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44bf95); f_004714a5(cpu); if cpu.returned_to != 0x44bf95 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44bf9c; continue;
 }
 0x44bf9c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44bfd1); f_004714a5(cpu); if cpu.returned_to != 0x44bfd1 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44bfd8; continue;
 }
 0x44bfd8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44c00d); f_004714a5(cpu); if cpu.returned_to != 0x44c00d { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44c014; continue;
 }
 0x44c000 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44c00d); f_004714a5(cpu); if cpu.returned_to != 0x44c00d { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c163; continue; }
  pc = 0x44c014; continue;
 }
 0x44c014 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44c035; continue; }
  pc = 0x44c021; continue;
 }
 0x44c021 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = 0x5b27aau32; cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  pc = 0x44c045; continue;
 }
 0x44c035 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = 0x5b27aau32; cpu.edi = v; }
  { let v: u16 = (cpu.ecx as u16) ^ (cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44c045; continue;
 }
 0x44c045 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x5b27a2u32), 0xffffu16); }
  if cpu.zf { pc = 0x44c079; continue; }
  pc = 0x44c053; continue;
 }
 0x44c053 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5b27a2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x5b27a4u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4b0u32; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  pc = 0x44c0b3; continue;
 }
 0x44c079 => {
  { let v: u32 = 0x4b0u32; cpu.ebp = v; }
  pc = 0x44c07e; continue;
 }
 0x44c07e => {
  { cpu.sub8(cpu.r8(0x5b27aau32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x44c11a; continue; }
  pc = 0x44c08b; continue;
 }
 0x44c08b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5b289au32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b289cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44c0b3; continue; }
  pc = 0x44c0ad; continue;
 }
 0x44c0ad => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44c0b3; continue;
 }
 0x44c0b3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x4d68aeu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x44c119; continue; }
  pc = 0x44c0c7; continue;
 }
 0x44c0c7 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d68b0u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x44c119; continue; }
  pc = 0x44c0d0; continue;
 }
 0x44c0d0 => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.r32(0x4d6bf4u32); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1f4cu32), 0x1u8); }
  if !cpu.zf { pc = 0x44c100; continue; }
  pc = 0x44c0e1; continue;
 }
 0x44c0e1 => {
  cpu.push(0x44c0e6); f_004712e3(cpu); if cpu.returned_to != 0x44c0e6 { return; }
  if !cpu.cf { pc = 0x44c100; continue; }
  pc = 0x44c0e8; continue;
 }
 0x44c0e8 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x44c100; continue; }
  pc = 0x44c0f5; continue;
 }
 0x44c0f5 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.popad(); }
  pc = 0x44c11a; continue;
 }
 0x44c100 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  cpu.push(0x44c10d); f_0044c176(cpu); if cpu.returned_to != 0x44c10d { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c11a; continue; }
  pc = 0x44c110; continue;
 }
 0x44c110 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b27aau32.wrapping_add(cpu.ecx), v); }
  pc = 0x44c148; continue;
 }
 0x44c119 => {
  { cpu.popad(); }
  pc = 0x44c11a; continue;
 }
 0x44c11a => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xf0u8); }
  if !cpu.zf { pc = 0x44c13d; continue; }
  pc = 0x44c121; continue;
 }
 0x44c121 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x44c128); f_0044c165(cpu); if cpu.returned_to != 0x44c128 { return; }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if !cpu.zf { pc = 0x44c13b; continue; }
  pc = 0x44c134; continue;
 }
 0x44c134 => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  pc = 0x44c13d; continue;
 }
 0x44c13b => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x44c13d; continue;
 }
 0x44c13d => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44c07e; continue; }
  pc = 0x44c144; continue;
 }
 0x44c144 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x44c163; continue;
 }
 0x44c148 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d68aeu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x4d68b0u32), (cpu.ebx as u16)); cpu.w16(0x4d68b0u32, v); }
  pc = 0x44c163; continue;
 }
 0x44c163 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c165(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c165); }
 let mut pc: u32 = 0x44c165;
 loop { match pc {
 0x44c165 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5b27aau32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c176(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c176); }
 let mut pc: u32 = 0x44c176;
 loop { match pc {
 0x44c176 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44c1f2; continue; }
  pc = 0x44c181; continue;
 }
 0x44c181 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6790u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44c1f2; continue; }
  pc = 0x44c18a; continue;
 }
 0x44c18a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.sf { pc = 0x44c1f2; continue; }
  pc = 0x44c18f; continue;
 }
 0x44c18f => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  if cpu.sf { pc = 0x44c1f2; continue; }
  pc = 0x44c194; continue;
 }
 0x44c194 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44c1b9; continue; }
  pc = 0x44c19b; continue;
 }
 0x44c19b => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.sf { pc = 0x44c1f2; continue; }
  pc = 0x44c1a0; continue;
 }
 0x44c1a0 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x7u32); cpu.ebx = v; }
  if cpu.sf { pc = 0x44c1f2; continue; }
  pc = 0x44c1a5; continue;
 }
 0x44c1a5 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44c1f2; continue; }
  pc = 0x44c1ae; continue;
 }
 0x44c1ae => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6790u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44c1f2; continue; }
  pc = 0x44c1b7; continue;
 }
 0x44c1b7 => {
  pc = 0x44c1ea; continue;
 }
 0x44c1b9 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x59dafdu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44c1ce); f_004716d0(cpu); if cpu.returned_to != 0x44c1ce { return; }
  if !cpu.cf { pc = 0x44c1e9; continue; }
  pc = 0x44c1d0; continue;
 }
 0x44c1d0 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x1fu16); }
  if !cpu.zf { pc = 0x44c1e9; continue; }
  pc = 0x44c1e6; continue;
 }
 0x44c1e6 => {
  { cpu.popad(); }
  pc = 0x44c1f2; continue;
 }
 0x44c1e9 => {
  { cpu.popad(); }
  pc = 0x44c1ea; continue;
 }
 0x44c1ea => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44c1f1); f_0041a689(cpu); if cpu.returned_to != 0x44c1f1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c1f2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c1f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c1f6); }
 let mut pc: u32 = 0x44c1f6;
 loop { match pc {
 0x44c1f6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b27a2u32, v); }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b27a6u32, v); }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b27a8u32, v); }
  { cpu.sub8(cpu.r8(0x58c558u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c231; continue; }
  pc = 0x44c227; continue;
 }
 0x44c227 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5b27a2u32, v); }
  pc = 0x44c231; continue;
 }
 0x44c231 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x44c30e; continue; }
  pc = 0x44c242; continue;
 }
 0x44c242 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44c24b); f_0044c7f9(cpu); if cpu.returned_to != 0x44c24b { return; }
  if cpu.cf { pc = 0x44c30e; continue; }
  pc = 0x44c251; continue;
 }
 0x44c251 => {
  cpu.push(0x44c256); f_0044c165(cpu); if cpu.returned_to != 0x44c256 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x1u32); cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44c25b; continue;
 }
 0x44c25b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44c305; continue; }
  pc = 0x44c264; continue;
 }
 0x44c264 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x44c305; continue; }
  pc = 0x44c27b; continue;
 }
 0x44c27b => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x44c305; continue; }
  pc = 0x44c291; continue;
 }
 0x44c291 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44c305; continue; }
  pc = 0x44c29e; continue;
 }
 0x44c29e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x44c305; continue; }
  pc = 0x44c2a5; continue;
 }
 0x44c2a5 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6bf4u32); cpu.push(v); }
  cpu.push(0x44c2b1); f_0044c310(cpu); if cpu.returned_to != 0x44c2b1 { return; }
  if cpu.cf { pc = 0x44c2fe; continue; }
  pc = 0x44c2b3; continue;
 }
 0x44c2b3 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x44c2db); f_00413337(cpu); if cpu.returned_to != 0x44c2db { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68b2u32, v); }
  { cpu.popad(); }
  pc = 0x44c2fe; continue;
 }
 0x44c2fe => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { cpu.popad(); }
  pc = 0x44c305; continue;
 }
 0x44c305 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44c25b; continue; }
  pc = 0x44c30e; continue;
 }
 0x44c30e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c310(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c310); } r_0044c310(cpu, 0x44c310); }
pub fn f_0044c444(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c444); } r_0044c310(cpu, 0x44c444); }
pub fn f_0044c494(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c494); } r_0044c310(cpu, 0x44c494); }
pub fn f_0044c4a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c4a9); } r_0044c310(cpu, 0x44c4a9); }
pub fn f_0044c4d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c4d9); } r_0044c310(cpu, 0x44c4d9); }
pub fn f_0044c55d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c55d); } r_0044c310(cpu, 0x44c55d); }
pub fn f_0044c5d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c5d6); } r_0044c310(cpu, 0x44c5d6); }
fn r_0044c310(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44c310 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x44c4a9; continue; }
  pc = 0x44c325; continue;
 }
 0x44c325 => {
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x44c346; continue; }
  pc = 0x44c333; continue;
 }
 0x44c333 => {
  cpu.push(0x44c338); f_0046d525(cpu); if cpu.returned_to != 0x44c338 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x57593eu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44c34c; continue;
 }
 0x44c346 => {
  { let v: u8 = cpu.r8(0x575917u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44c34c; continue;
 }
 0x44c34c => {
  { let v: u32 = 0x5b2d4cu32; cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x44c47f; continue; }
  pc = 0x44c35a; continue;
 }
 0x44c35a => {
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x44c48c; continue; }
  pc = 0x44c363; continue;
 }
 0x44c363 => {
  { let v: u32 = 0x5b2d5au32; cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x4d68aeu32), 0xd80u16); }
  if !cpu.zf { pc = 0x44c47f; continue; }
  pc = 0x44c377; continue;
 }
 0x44c377 => {
  { let v: u32 = 0x5b2d84u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x44c47f; continue; }
  pc = 0x44c38d; continue;
 }
 0x44c38d => {
  { let v: u32 = 0x5b2d92u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x44c3c8; continue; }
  pc = 0x44c399; continue;
 }
 0x44c399 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44c3ac); f_00420989(cpu); if cpu.returned_to != 0x44c3ac { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x3bu8); }
  if cpu.zf { pc = 0x44c3c2; continue; }
  pc = 0x44c3ba; continue;
 }
 0x44c3ba => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x44c3c2; continue; }
  pc = 0x44c3bf; continue;
 }
 0x44c3bf => {
  { cpu.popad(); }
  pc = 0x44c3c8; continue;
 }
 0x44c3c2 => {
  { cpu.popad(); }
  pc = 0x44c47f; continue;
 }
 0x44c3c8 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x44c451; continue; }
  pc = 0x44c3d1; continue;
 }
 0x44c3d1 => {
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x44c451; continue; }
  pc = 0x44c3de; continue;
 }
 0x44c3de => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6934u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6936u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6bf4u32); cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44c403); f_0046e31e(cpu); if cpu.returned_to != 0x44c403 { return; }
  if cpu.cf { pc = 0x44c43c; continue; }
  pc = 0x44c405; continue;
 }
 0x44c405 => {
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x44c41e); f_004712e3(cpu); if cpu.returned_to != 0x44c41e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  if cpu.cf { pc = 0x44c43c; continue; }
  pc = 0x44c427; continue;
 }
 0x44c427 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6934u32, v); }
  { cpu.popad(); }
  pc = 0x44c451; continue;
 }
 0x44c43c => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6934u32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x44c451; continue;
 }
 0x44c444 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x6934058fu32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xffffffb6u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffb6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1a750df9u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x1a750df9u32.wrapping_add(cpu.eax), v); }
  pc = 0x44c456; continue;
 }
 0x44c451 => {
  { cpu.sub8((cpu.ecx as u8), 0xdu8); }
  if !cpu.zf { pc = 0x44c470; continue; }
  pc = 0x44c456; continue;
 }
 0x44c456 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x57593eu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x44c3de; continue; }
  pc = 0x44c470; continue;
 }
 0x44c470 => {
  { let v: u32 = 0x5b2d76u32; cpu.esi = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x0u8); }
  if cpu.zf { pc = 0x44c47f; continue; }
  pc = 0x44c47a; continue;
 }
 0x44c47a => {
  { let v: u32 = 0x5b2d68u32; cpu.esi = v; }
  pc = 0x44c47f; continue;
 }
 0x44c47f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x5b2da0u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x44c48c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.div32(v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c494 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c4a9 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xa3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xad6u16); }
  if cpu.zf { pc = 0x44c4ce; continue; }
  pc = 0x44c4c2; continue;
 }
 0x44c4c2 => {
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x44c4ce; continue; }
  pc = 0x44c4c8; continue;
 }
 0x44c4c8 => {
  { cpu.sub16((cpu.eax as u16), 0xaceu16); }
  if !cpu.zf { pc = 0x44c4d2; continue; }
  pc = 0x44c4ce; continue;
 }
 0x44c4ce => {
  { let v: u16 = 0xac6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44c4d2; continue;
 }
 0x44c4d2 => {
  cpu.push(0x44c4d7); f_0041b7b0(cpu); if cpu.returned_to != 0x44c4d7 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c4d9 => {
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x4d68aeu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x44c529); f_00413337(cpu); if cpu.returned_to != 0x44c529 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x44c540); f_0041b7b0(cpu); if cpu.returned_to != 0x44c540 { return; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68b2u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c55d => {
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x44c5a2); f_00413337(cpu); if cpu.returned_to != 0x44c5a2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x44c5b9); f_0041b7b0(cpu); if cpu.returned_to != 0x44c5b9 { return; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68b2u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c5d6 => {
  { let v: u8 = cpu.r8(0x4e1f4cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68aeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d68b2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x4d68b0u32) as u32; cpu.eax = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44c649; continue; }
  pc = 0x44c604; continue;
 }
 0x44c604 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x44c632); f_00413337(cpu); if cpu.returned_to != 0x44c632 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u32 = cpu.r32(0x4d6aecu32); cpu.ebp = v; }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.push(0x44c649); f_0041b7b0(cpu); if cpu.returned_to != 0x44c649 { return; }
  pc = 0x44c649; continue;
 }
 0x44c649 => {
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68b2u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c666(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c666); }
 let mut pc: u32 = 0x44c666;
 loop { match pc {
 0x44c666 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44c670; continue; }
  pc = 0x44c66f; continue;
 }
 0x44c66f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c670 => {
  { cpu.sub8(cpu.r8(0x5b27a0u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c684; continue; }
  pc = 0x44c679; continue;
 }
 0x44c679 => {
  cpu.push(0x44c67e); f_004184ca(cpu); if cpu.returned_to != 0x44c67e { return; }
  cpu.push(0x44c683); f_0044c69e(cpu); if cpu.returned_to != 0x44c683 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c684 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c686(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c686); }
 let mut pc: u32 = 0x44c686;
 loop { match pc {
 0x44c686 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44c69d; continue; }
  pc = 0x44c68d; continue;
 }
 0x44c68d => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44c696); f_0044c7f9(cpu); if cpu.returned_to != 0x44c696 { return; }
  if cpu.cf { pc = 0x44c69d; continue; }
  pc = 0x44c698; continue;
 }
 0x44c698 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x44c69d; continue;
 }
 0x44c69d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c69e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c69e); } r_0044c69e(cpu, 0x44c69e); }
pub fn f_0044c744(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44c744); } r_0044c69e(cpu, 0x44c744); }
fn r_0044c69e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44c69e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44c72c; continue; }
  pc = 0x44c6a9; continue;
 }
 0x44c6a9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x44c72c; continue; }
  pc = 0x44c6b0; continue;
 }
 0x44c6b0 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6a28u32); cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  cpu.push(0x44c6cf); f_0044aa9c(cpu); if cpu.returned_to != 0x44c6cf { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a28u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44c72c; continue; }
  pc = 0x44c6d8; continue;
 }
 0x44c6d8 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44c6e2); f_0044c82f(cpu); if cpu.returned_to != 0x44c6e2 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44c733; continue; }
  pc = 0x44c6e5; continue;
 }
 0x44c6e5 => {
  { cpu.sub8(cpu.r8(0x5928d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c702; continue; }
  pc = 0x44c6ee; continue;
 }
 0x44c6ee => {
  { cpu.pushad(); }
  cpu.push(0x44c6f4); f_00403030(cpu); if cpu.returned_to != 0x44c6f4 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x44c702; continue; }
  pc = 0x44c6f9; continue;
 }
 0x44c6f9 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c702 => {
  { cpu.sub8(cpu.r8(0x4e1fbfu32), 0xffu8); }
  if cpu.zf { pc = 0x44c735; continue; }
  pc = 0x44c70b; continue;
 }
 0x44c70b => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf4u32; cpu.eax = v; }
  { let v = cpu.r8(0x4e1fbfu32); cpu.mul8(v); }
  { let v: u32 = 0x534c50u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x44c72e; continue; }
  pc = 0x44c72c; continue;
 }
 0x44c72c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c72e => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x44c733; continue;
 }
 0x44c733 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c735 => {
  { cpu.sub8(cpu.r8(0x5928d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c757; continue; }
  pc = 0x44c73e; continue;
 }
 0x44c73e => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44c757; continue; }
  pc = 0x44c747; continue;
 }
 0x44c744 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x44c747; continue;
 }
 0x44c747 => {
  { let v: u16 = cpu.r16(0x4d67eeu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6758u32)); }
  if cpu.zf { cpu.poll(); pc = 0x44c72c; continue; }
  pc = 0x44c757; continue;
 }
 0x44c757 => {
  { let v: u32 = 0x534c50u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44c75e; continue;
 }
 0x44c75e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1fbfu32)); cpu.w8(0x4e1fbfu32, v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44c775; continue; }
  pc = 0x44c769; continue;
 }
 0x44c769 => {
  { let v: u32 = cpu.esi.wrapping_add(0xf4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44c75e; continue; }
  pc = 0x44c773; continue;
 }
 0x44c773 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c775 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44c7c2; continue; }
  pc = 0x44c786; continue;
 }
 0x44c786 => {
  { cpu.sub8(cpu.r8(0x5928d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c7c2; continue; }
  pc = 0x44c78f; continue;
 }
 0x44c78f => {
  { let v: u16 = cpu.r16(0x4d67eeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x44c7c2; continue; }
  pc = 0x44c79b; continue;
 }
 0x44c79b => {
  { let v = cpu.r16(0x4d6758u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6758u32, v); }
  cpu.push(0x44c7ae); f_004184ca(cpu); if cpu.returned_to != 0x44c7ae { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6758u32, v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x44c7c2; continue; }
  pc = 0x44c7be; continue;
 }
 0x44c7be => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44c7c2; continue;
 }
 0x44c7c2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xeeu32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c7d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c7d4); }
 let mut pc: u32 = 0x44c7d4;
 loop { match pc {
 0x44c7d4 => {
  cpu.push(0x44c7d9); f_0044c7f9(cpu); if cpu.returned_to != 0x44c7d9 { return; }
  if cpu.cf { pc = 0x44c7f8; continue; }
  pc = 0x44c7db; continue;
 }
 0x44c7db => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x44c7e3; continue;
 }
 0x44c7e3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44c7f4; continue; }
  pc = 0x44c7e9; continue;
 }
 0x44c7e9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44c7f1); f_0044c896(cpu); if cpu.returned_to != 0x44c7f1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44c7f4; continue;
 }
 0x44c7f4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44c7e3; continue; }
  pc = 0x44c7f8; continue;
 }
 0x44c7f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c7f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c7f9); }
 let mut pc: u32 = 0x44c7f9;
 loop { match pc {
 0x44c7f9 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44c823; continue; }
  pc = 0x44c7fd; continue;
 }
 0x44c7fd => {
  { let v: u32 = 0x534c50u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44c804; continue;
 }
 0x44c804 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44c819; continue; }
  pc = 0x44c809; continue;
 }
 0x44c809 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x44c825; continue; }
  pc = 0x44c812; continue;
 }
 0x44c812 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x44c825; continue; }
  pc = 0x44c819; continue;
 }
 0x44c819 => {
  { let v: u32 = cpu.esi.wrapping_add(0xf4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44c804; continue; }
  pc = 0x44c823; continue;
 }
 0x44c823 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c825 => {
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c82f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c82f); }
 let mut pc: u32 = 0x44c82f;
 loop { match pc {
 0x44c82f => {
  cpu.push(0x44c834); f_0044c7f9(cpu); if cpu.returned_to != 0x44c834 { return; }
  if !cpu.cf { pc = 0x44c838; continue; }
  pc = 0x44c836; continue;
 }
 0x44c836 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c838 => {
  { cpu.sub8(cpu.r8(0x5928d3u32), 0x0u8); }
  if !cpu.zf { pc = 0x44c84c; continue; }
  pc = 0x44c841; continue;
 }
 0x44c841 => {
  { cpu.pushad(); }
  cpu.push(0x44c847); f_00403030(cpu); if cpu.returned_to != 0x44c847 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x44c850; continue; }
  pc = 0x44c84c; continue;
 }
 0x44c84c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c850 => {
  cpu.push(0x44c855); f_0044c896(cpu); if cpu.returned_to != 0x44c855 { return; }
  if cpu.cf { pc = 0x44c887; continue; }
  pc = 0x44c857; continue;
 }
 0x44c857 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44c889; continue; }
  pc = 0x44c85c; continue;
 }
 0x44c85c => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44c887; continue; }
  pc = 0x44c86f; continue;
 }
 0x44c86f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b27a0u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44c87f); f_0041a8a2(cpu); if cpu.returned_to != 0x44c87f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b27a0u32, v); }
  pc = 0x44c887; continue;
 }
 0x44c887 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c889 => {
  cpu.push(0x44c88e); f_0041879e(cpu); if cpu.returned_to != 0x44c88e { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c896(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c896); }
 let mut pc: u32 = 0x44c896;
 loop { match pc {
 0x44c896 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x44c8f5; continue; }
  pc = 0x44c8a2; continue;
 }
 0x44c8a2 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x44c8c0; continue; }
  pc = 0x44c8aa; continue;
 }
 0x44c8aa => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  if cpu.zf { pc = 0x44c8c0; continue; }
  pc = 0x44c8bb; continue;
 }
 0x44c8bb => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x44c8c0; continue;
 }
 0x44c8c0 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44c8ca; continue;
 }
 0x44c8ca => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x44c8d1; continue; }
  pc = 0x44c8cf; continue;
 }
 0x44c8cf => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44c8d1; continue;
 }
 0x44c8d1 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44c8ca; continue; }
  pc = 0x44c8d6; continue;
 }
 0x44c8d6 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44c8f3; continue; }
  pc = 0x44c8dd; continue;
 }
 0x44c8dd => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x44c8e4; continue; }
  pc = 0x44c8e2; continue;
 }
 0x44c8e2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x44c8e4; continue;
 }
 0x44c8e4 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  pc = 0x44c8f3; continue;
 }
 0x44c8f3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44c8f5 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044c8fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44c8fc); }
 let mut pc: u32 = 0x44c8fc;
 loop { match pc {
 0x44c8fc => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if cpu.zf { pc = 0x44ca90; continue; }
  pc = 0x44c909; continue;
 }
 0x44c909 => {
  { cpu.sub8(cpu.r8(0x5b38d5u32), 0x0u8); }
  if cpu.zf { pc = 0x44c91d; continue; }
  pc = 0x44c912; continue;
 }
 0x44c912 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b38d5u32)); cpu.w8(0x5b38d5u32, v); }
  pc = 0x44ca90; continue;
 }
 0x44c91d => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if cpu.zf { pc = 0x44c950; continue; }
  pc = 0x44c926; continue;
 }
 0x44c926 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5b2dc4u32)); cpu.w32(0x5b2dc4u32, v); }
  if !cpu.zf { pc = 0x44c950; continue; }
  pc = 0x44c92e; continue;
 }
 0x44c92e => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  cpu.push(0x44c93d); f_00471ec2(cpu); if cpu.returned_to != 0x44c93d { return; }
  { cpu.sub8(cpu.r8(0x58c492u32), 0x0u8); }
  if cpu.zf { pc = 0x44c94b; continue; }
  pc = 0x44c946; continue;
 }
 0x44c946 => {
  cpu.push(0x44c94b); f_0044ebcb(cpu); if cpu.returned_to != 0x44c94b { return; }
  pc = 0x44c94b; continue;
 }
 0x44c94b => {
  pc = 0x44ca90; continue;
 }
 0x44c950 => {
  cpu.push(0x44c955); f_0044cafa(cpu); if cpu.returned_to != 0x44c955 { return; }
  if cpu.cf { pc = 0x44ca90; continue; }
  pc = 0x44c95b; continue;
 }
 0x44c95b => {
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58c49cu32), 0x0u8); }
  if !cpu.zf { pc = 0x44c97b; continue; }
  pc = 0x44c971; continue;
 }
 0x44c971 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x44c99a; continue; }
  pc = 0x44c979; continue;
 }
 0x44c979 => {
  pc = 0x44c97f; continue;
 }
 0x44c97b => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44c99a; continue; }
  pc = 0x44c97f; continue;
 }
 0x44c97f => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x44c995); f_00471ec2(cpu); if cpu.returned_to != 0x44c995 { return; }
  pc = 0x44ca90; continue;
 }
 0x44c99a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3868u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b3869u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44c9c7; continue; }
  pc = 0x44c9b5; continue;
 }
 0x44c9b5 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44c9c7; continue; }
  pc = 0x44c9bc; continue;
 }
 0x44c9bc => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44ca90; continue; }
  pc = 0x44c9c7; continue;
 }
 0x44c9c7 => {
  cpu.push(0x44c9cc); f_0044cab9(cpu); if cpu.returned_to != 0x44c9cc { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44c9d4); f_00447617(cpu); if cpu.returned_to != 0x44c9d4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44c9dc); f_0044dca7(cpu); if cpu.returned_to != 0x44c9dc { return; }
  cpu.push(0x44c9e1); f_0044dbc4(cpu); if cpu.returned_to != 0x44c9e1 { return; }
  cpu.push(0x44c9e6); f_0044dcea(cpu); if cpu.returned_to != 0x44c9e6 { return; }
  cpu.push(0x44c9eb); f_0044de17(cpu); if cpu.returned_to != 0x44c9eb { return; }
  cpu.push(0x44c9f0); f_0044dc03(cpu); if cpu.returned_to != 0x44c9f0 { return; }
  cpu.push(0x44c9f5); f_0044d400(cpu); if cpu.returned_to != 0x44c9f5 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44c9fd); f_00447617(cpu); if cpu.returned_to != 0x44c9fd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44ca05); f_0044db21(cpu); if cpu.returned_to != 0x44ca05 { return; }
  cpu.push(0x44ca0a); f_0044da6a(cpu); if cpu.returned_to != 0x44ca0a { return; }
  cpu.push(0x44ca0f); f_0044d371(cpu); if cpu.returned_to != 0x44ca0f { return; }
  cpu.push(0x44ca14); f_0044cb3b(cpu); if cpu.returned_to != 0x44ca14 { return; }
  cpu.push(0x44ca19); f_004209f6(cpu); if cpu.returned_to != 0x44ca19 { return; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44ca48; continue; }
  pc = 0x44ca20; continue;
 }
 0x44ca20 => {
  { let v: u8 = cpu.r8(0x5b386du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44ca2a); f_0044e720(cpu); if cpu.returned_to != 0x44ca2a { return; }
  if cpu.cf { pc = 0x44ca43; continue; }
  pc = 0x44ca2c; continue;
 }
 0x44ca2c => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44ca3c; continue; }
  pc = 0x44ca30; continue;
 }
 0x44ca30 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44ca3c; continue; }
  pc = 0x44ca34; continue;
 }
 0x44ca34 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44ca3c; continue; }
  pc = 0x44ca38; continue;
 }
 0x44ca38 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if !cpu.zf { pc = 0x44ca48; continue; }
  pc = 0x44ca3c; continue;
 }
 0x44ca3c => {
  cpu.push(0x44ca41); f_0044e7a2(cpu); if cpu.returned_to != 0x44ca41 { return; }
  if !cpu.cf { pc = 0x44ca48; continue; }
  pc = 0x44ca43; continue;
 }
 0x44ca43 => {
  cpu.push(0x44ca48); f_0044ca91(cpu); if cpu.returned_to != 0x44ca48 { return; }
  pc = 0x44ca48; continue;
 }
 0x44ca48 => {
  cpu.push(0x44ca4d); f_0044e720(cpu); if cpu.returned_to != 0x44ca4d { return; }
  if !cpu.cf { pc = 0x44ca54; continue; }
  pc = 0x44ca4f; continue;
 }
 0x44ca4f => {
  cpu.push(0x44ca54); f_0044ddbd(cpu); if cpu.returned_to != 0x44ca54 { return; }
  pc = 0x44ca54; continue;
 }
 0x44ca54 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b3851u32)); cpu.w8(0x5b3851u32, v); }
  if !cpu.sf { pc = 0x44ca63; continue; }
  pc = 0x44ca5c; continue;
 }
 0x44ca5c => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3851u32, v); }
  pc = 0x44ca63; continue;
 }
 0x44ca63 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b3852u32)); cpu.w8(0x5b3852u32, v); }
  if !cpu.sf { pc = 0x44ca72; continue; }
  pc = 0x44ca6b; continue;
 }
 0x44ca6b => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3852u32, v); }
  pc = 0x44ca72; continue;
 }
 0x44ca72 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b3857u32)); cpu.w8(0x5b3857u32, v); }
  if !cpu.sf { pc = 0x44ca81; continue; }
  pc = 0x44ca7a; continue;
 }
 0x44ca7a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  pc = 0x44ca81; continue;
 }
 0x44ca81 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b3856u32)); cpu.w8(0x5b3856u32, v); }
  if !cpu.sf { pc = 0x44ca90; continue; }
  pc = 0x44ca89; continue;
 }
 0x44ca89 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  pc = 0x44ca90; continue;
 }
 0x44ca90 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ca91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ca91); }
 let mut pc: u32 = 0x44ca91;
 loop { match pc {
 0x44ca91 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x5b3894u32), 0x0u8); }
  if !cpu.zf { pc = 0x44cab7; continue; }
  pc = 0x44ca9b; continue;
 }
 0x44ca9b => {
  { let v: u32 = cpu.r32(0x5b2dbcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2dc0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x22u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cab7); cpu.call(t); if cpu.returned_to != 0x44cab7 { return; }
  pc = 0x44cab7; continue;
 }
 0x44cab7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cab9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cab9); }
 let mut pc: u32 = 0x44cab9;
 loop { match pc {
 0x44cab9 => {
  { let v: u32 = cpu.r32(0x5b2dd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2ddcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b2dc8u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5b2dccu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5b2dd0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x5b2dd4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dbcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2dc0u32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffd8u32); }
  if !cpu.zf { pc = 0x44caf9; continue; }
  pc = 0x44caf8; continue;
 }
 0x44caf8 => {
  {  }
  pc = 0x44caf9; continue;
 }
 0x44caf9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cafa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cafa); }
 let mut pc: u32 = 0x44cafa;
 loop { match pc {
 0x44cafa => {
  { let v: u32 = cpu.r32(0x5b2dbcu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x44cb27; continue; }
  pc = 0x44cb07; continue;
 }
 0x44cb07 => {
  { cpu.sub32(cpu.eax, 0xffffff90u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44cb27; continue; }
  pc = 0x44cb0c; continue;
 }
 0x44cb0c => {
  { let v: u32 = cpu.r32(0x5b2dc0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x44cb27; continue; }
  pc = 0x44cb19; continue;
 }
 0x44cb19 => {
  { cpu.sub32(cpu.eax, 0xffffffb0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44cb27; continue; }
  pc = 0x44cb1e; continue;
 }
 0x44cb1e => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b385du32, v); }
  pc = 0x44cb25; continue;
 }
 0x44cb25 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44cb27 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5b385du32)); cpu.w8(0x5b385du32, v); }
  { cpu.sub8(cpu.r8(0x5b385du32), 0x11u8); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44cb25; continue; }
  pc = 0x44cb36; continue;
 }
 0x44cb36 => {
  f_0044d990(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cb3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cb3b); }
 let mut pc: u32 = 0x44cb3b;
 loop { match pc {
 0x44cb3b => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0xfa0u32); }
  if (cpu.sf != cpu.of) { pc = 0x44cb5c; continue; }
  pc = 0x44cb45; continue;
 }
 0x44cb45 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4fu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xau32); cpu.ebx = v; }
  { let v: u32 = 0x127cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cb59); cpu.call(t); if cpu.returned_to != 0x44cb59 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44cb5c; continue;
 }
 0x44cb5c => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0xbb8u32); }
  if (cpu.sf != cpu.of) { pc = 0x44cb7d; continue; }
  pc = 0x44cb66; continue;
 }
 0x44cb66 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x47u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xau32); cpu.ebx = v; }
  { let v: u32 = 0x127cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cb7a); cpu.call(t); if cpu.returned_to != 0x44cb7a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44cb7d; continue;
 }
 0x44cb7d => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x7d0u32); }
  if (cpu.sf != cpu.of) { pc = 0x44cb9e; continue; }
  pc = 0x44cb87; continue;
 }
 0x44cb87 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4fu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = 0x127cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cb9b); cpu.call(t); if cpu.returned_to != 0x44cb9b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44cb9e; continue;
 }
 0x44cb9e => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.sf != cpu.of) { pc = 0x44cbbf; continue; }
  pc = 0x44cba8; continue;
 }
 0x44cba8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x47u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = 0x127cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cbbc); cpu.call(t); if cpu.returned_to != 0x44cbbc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44cbbf; continue;
 }
 0x44cbbf => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x1388u32); }
  if (cpu.sf != cpu.of) { pc = 0x44cbe0; continue; }
  pc = 0x44cbc9; continue;
 }
 0x44cbc9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4bu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x7u32); cpu.ebx = v; }
  { let v: u32 = 0x127cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44cbdd); cpu.call(t); if cpu.returned_to != 0x44cbdd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44cbe0; continue;
 }
 0x44cbe0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cbe1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cbe1); }
 let mut pc: u32 = 0x44cbe1;
 loop { match pc {
 0x44cbe1 => {
  { cpu.sub32(cpu.r32(0x5b3590u32), 0x5b3887u32); }
  if !cpu.zf { pc = 0x44cbff; continue; }
  pc = 0x44cbed; continue;
 }
 0x44cbed => {
  { cpu.sub32(cpu.r32(0x5b3594u32), cpu.ebp); }
  if !cpu.zf { pc = 0x44cbff; continue; }
  pc = 0x44cbf5; continue;
 }
 0x44cbf5 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  pc = 0x44cbff; continue;
 }
 0x44cbff => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b386eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b38d5u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44cc26; continue; }
  pc = 0x44cc1f; continue;
 }
 0x44cc1f => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b38d5u32, v); }
  pc = 0x44cc26; continue;
 }
 0x44cc26 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b385du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dc4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dc8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dccu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dd0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dd4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3855u32, v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x44ccb5; continue; }
  pc = 0x44cc6e; continue;
 }
 0x44cc6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c465u32), 0x0u8); }
  if cpu.zf { pc = 0x44ccab; continue; }
  pc = 0x44cc7b; continue;
 }
 0x44cc7b => {
  { let v: u16 = 0x80u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x5cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44cc9b; continue; }
  pc = 0x44cc8c; continue;
 }
 0x44cc8c => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44cc9b; continue;
 }
 0x44cc9b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44ccb5; continue;
 }
 0x44ccab => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44ccb5; continue;
 }
 0x44ccb5 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2ddcu32, v); }
  cpu.push(0x44cccb); f_0044cab9(cpu); if cpu.returned_to != 0x44cccb { return; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44ccee; continue; }
  pc = 0x44ccd2; continue;
 }
 0x44ccd2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44cd5d; continue; }
  pc = 0x44ccdd; continue;
 }
 0x44ccdd => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44cd5c; continue; }
  pc = 0x44cce4; continue;
 }
 0x44cce4 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ccee => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  cpu.push(0x44ccfc); f_0045d200(cpu); if cpu.returned_to != 0x44ccfc { return; }
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4d6a38u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5c4e6du32, v); }
  { let v: u32 = cpu.r8(0x4eab29u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5c4e71u32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x5eu32; cpu.ebx = v; }
  cpu.push(0x44cd57); f_0045d4b4(cpu); if cpu.returned_to != 0x44cd57 { return; }
  cpu.push(0x44cd5c); f_0045d1e7(cpu); if cpu.returned_to != 0x44cd5c { return; }
  pc = 0x44cd5c; continue;
 }
 0x44cd5c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44cd5d => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  cpu.push(0x44cd6b); f_0045d200(cpu); if cpu.returned_to != 0x44cd6b { return; }
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4d6a38u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5c4e6du32, v); }
  { let v: u32 = cpu.r8(0x4eab29u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5c4e71u32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x5eu32; cpu.ebx = v; }
  cpu.push(0x44cdc6); f_0045d4b4(cpu); if cpu.returned_to != 0x44cdc6 { return; }
  cpu.push(0x44cdcb); f_0045d1e7(cpu); if cpu.returned_to != 0x44cdcb { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cddb); }
 let mut pc: u32 = 0x44cddb;
 loop { match pc {
 0x44cddb => {
  cpu.push(0x44cde0); f_00471f00(cpu); if cpu.returned_to != 0x44cde0 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44cdf0; continue; }
  pc = 0x44cdee; continue;
 }
 0x44cdee => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44cdf0; continue;
 }
 0x44cdf0 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x5b3899u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x44ce05; continue; }
  pc = 0x44ce00; continue;
 }
 0x44ce00 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x44ce05; continue;
 }
 0x44ce05 => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44ce1d; continue; }
  pc = 0x44ce16; continue;
 }
 0x44ce16 => {
  { cpu.sub32(cpu.eax, 0x10u32); }
  if cpu.cf { pc = 0x44ce2f; continue; }
  pc = 0x44ce1b; continue;
 }
 0x44ce1b => {
  pc = 0x44ce2d; continue;
 }
 0x44ce1d => {
  { cpu.sub8(cpu.r8(0x4d67dau32), 0x14u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44ce28; continue; }
  pc = 0x44ce26; continue;
 }
 0x44ce26 => {
  cpu.poll(); pc = 0x44ce16; continue;
 }
 0x44ce28 => {
  { cpu.sub32(cpu.eax, 0x11u32); }
  if cpu.cf { pc = 0x44ce2f; continue; }
  pc = 0x44ce2d; continue;
 }
 0x44ce2d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x44ce2f; continue;
 }
 0x44ce2f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5b38adu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x575537u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3899u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ce4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ce4e); }
 let mut pc: u32 = 0x44ce4e;
 loop { match pc {
 0x44ce4e => {
  cpu.push(0x44ce53); f_00471f37(cpu); if cpu.returned_to != 0x44ce53 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44ce63; continue; }
  pc = 0x44ce61; continue;
 }
 0x44ce61 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44ce63; continue;
 }
 0x44ce63 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x5b3899u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x44ce78; continue; }
  pc = 0x44ce73; continue;
 }
 0x44ce73 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x44ce78; continue;
 }
 0x44ce78 => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x44cea3; continue; }
  pc = 0x44ce82; continue;
 }
 0x44ce82 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44cea3; continue; }
  pc = 0x44ce90; continue;
 }
 0x44ce90 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4d67dau32), 0x14u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x44cea3; continue; }
  pc = 0x44ce9e; continue;
 }
 0x44ce9e => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  pc = 0x44cea3; continue;
 }
 0x44cea3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5b38adu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x575537u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3899u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cec2); }
 let mut pc: u32 = 0x44cec2;
 loop { match pc {
 0x44cec2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44cecb; continue; }
  pc = 0x44cec9; continue;
 }
 0x44cec9 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44cecb; continue;
 }
 0x44cecb => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x5b3899u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x44cee0; continue; }
  pc = 0x44cedb; continue;
 }
 0x44cedb => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x44cee0; continue;
 }
 0x44cee0 => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5b38adu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x575537u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cef9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cef9); }
 let mut pc: u32 = 0x44cef9;
 loop { match pc {
 0x44cef9 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44cefe; continue;
 }
 0x44cefe => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44cf04); f_0044cddb(cpu); if cpu.returned_to != 0x44cf04 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x49u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44cf9e; continue; }
  pc = 0x44cf23; continue;
 }
 0x44cf23 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44cf9e; continue; }
  pc = 0x44cf2f; continue;
 }
 0x44cf2f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44cf9e; continue; }
  pc = 0x44cf34; continue;
 }
 0x44cf34 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44cf9e; continue; }
  pc = 0x44cf3c; continue;
 }
 0x44cf3c => {
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
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf6a; continue;
 }
 0x44cf6a => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf6e; continue;
 }
 0x44cf6e => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf72; continue;
 }
 0x44cf72 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf76; continue;
 }
 0x44cf76 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf7a; continue;
 }
 0x44cf7a => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf7e; continue;
 }
 0x44cf7e => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf82; continue;
 }
 0x44cf82 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf86; continue;
 }
 0x44cf86 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf8a; continue;
 }
 0x44cf8a => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf8e; continue;
 }
 0x44cf8e => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf92; continue;
 }
 0x44cf92 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x44cf9d; continue; }
  pc = 0x44cf96; continue;
 }
 0x44cf96 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44cefe; continue; }
  pc = 0x44cf9d; continue;
 }
 0x44cf9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44cf9e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44cefe; continue; }
  pc = 0x44cfa6; continue;
 }
 0x44cfa6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044cfa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44cfa7); }
 let mut pc: u32 = 0x44cfa7;
 loop { match pc {
 0x44cfa7 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44cfac; continue;
 }
 0x44cfac => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44cfb2); f_0044ce4e(cpu); if cpu.returned_to != 0x44cfb2 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x49u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44d050; continue; }
  pc = 0x44cfd1; continue;
 }
 0x44cfd1 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d050; continue; }
  pc = 0x44cfdd; continue;
 }
 0x44cfdd => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44d050; continue; }
  pc = 0x44cfe2; continue;
 }
 0x44cfe2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d050; continue; }
  pc = 0x44cfea; continue;
 }
 0x44cfea => {
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
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d018; continue;
 }
 0x44d018 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d01c; continue;
 }
 0x44d01c => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d020; continue;
 }
 0x44d020 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d024; continue;
 }
 0x44d024 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d028; continue;
 }
 0x44d028 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d02c; continue;
 }
 0x44d02c => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d030; continue;
 }
 0x44d030 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d034; continue;
 }
 0x44d034 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d038; continue;
 }
 0x44d038 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d03c; continue;
 }
 0x44d03c => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d040; continue;
 }
 0x44d040 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d044; continue;
 }
 0x44d044 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x44d04f; continue; }
  pc = 0x44d048; continue;
 }
 0x44d048 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44cfac; continue; }
  pc = 0x44d04f; continue;
 }
 0x44d04f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d050 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44cfac; continue; }
  pc = 0x44d058; continue;
 }
 0x44d058 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d059(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d059); }
 let mut pc: u32 = 0x44d059;
 loop { match pc {
 0x44d059 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44d05e; continue;
 }
 0x44d05e => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44d064); f_0044cddb(cpu); if cpu.returned_to != 0x44d064 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44d0f6; continue; }
  pc = 0x44d083; continue;
 }
 0x44d083 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d0f6; continue; }
  pc = 0x44d08f; continue;
 }
 0x44d08f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44d0f6; continue; }
  pc = 0x44d094; continue;
 }
 0x44d094 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d0f6; continue; }
  pc = 0x44d09c; continue;
 }
 0x44d09c => {
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
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0ca; continue;
 }
 0x44d0ca => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0ce; continue;
 }
 0x44d0ce => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0d2; continue;
 }
 0x44d0d2 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0d6; continue;
 }
 0x44d0d6 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0da; continue;
 }
 0x44d0da => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0de; continue;
 }
 0x44d0de => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0e2; continue;
 }
 0x44d0e2 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0e6; continue;
 }
 0x44d0e6 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0ea; continue;
 }
 0x44d0ea => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d0f5; continue; }
  pc = 0x44d0ee; continue;
 }
 0x44d0ee => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d05e; continue; }
  pc = 0x44d0f5; continue;
 }
 0x44d0f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d0f6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d05e; continue; }
  pc = 0x44d0fe; continue;
 }
 0x44d0fe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d0ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d0ff); }
 let mut pc: u32 = 0x44d0ff;
 loop { match pc {
 0x44d0ff => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44d104; continue;
 }
 0x44d104 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44d10a); f_0044ce4e(cpu); if cpu.returned_to != 0x44d10a { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44d19c; continue; }
  pc = 0x44d129; continue;
 }
 0x44d129 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d19c; continue; }
  pc = 0x44d135; continue;
 }
 0x44d135 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44d19c; continue; }
  pc = 0x44d13a; continue;
 }
 0x44d13a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d19c; continue; }
  pc = 0x44d142; continue;
 }
 0x44d142 => {
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
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d170; continue;
 }
 0x44d170 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d174; continue;
 }
 0x44d174 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d178; continue;
 }
 0x44d178 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d17c; continue;
 }
 0x44d17c => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d180; continue;
 }
 0x44d180 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d184; continue;
 }
 0x44d184 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d188; continue;
 }
 0x44d188 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d18c; continue;
 }
 0x44d18c => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d190; continue;
 }
 0x44d190 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d19b; continue; }
  pc = 0x44d194; continue;
 }
 0x44d194 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d104; continue; }
  pc = 0x44d19b; continue;
 }
 0x44d19b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d19c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d104; continue; }
  pc = 0x44d1a4; continue;
 }
 0x44d1a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d1a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d1a5); }
 let mut pc: u32 = 0x44d1a5;
 loop { match pc {
 0x44d1a5 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44d1aa; continue;
 }
 0x44d1aa => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44d1b0); f_0044ce4e(cpu); if cpu.returned_to != 0x44d1b0 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4bu32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44d282; continue; }
  pc = 0x44d1d3; continue;
 }
 0x44d1d3 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d282; continue; }
  pc = 0x44d1e3; continue;
 }
 0x44d1e3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44d282; continue; }
  pc = 0x44d1ec; continue;
 }
 0x44d1ec => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d282; continue; }
  pc = 0x44d1f8; continue;
 }
 0x44d1f8 => {
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
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d226; continue;
 }
 0x44d226 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d22a; continue;
 }
 0x44d22a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d22e; continue;
 }
 0x44d22e => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d232; continue;
 }
 0x44d232 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d236; continue;
 }
 0x44d236 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d23a; continue;
 }
 0x44d23a => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d23e; continue;
 }
 0x44d23e => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d242; continue;
 }
 0x44d242 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d246; continue;
 }
 0x44d246 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d24a; continue;
 }
 0x44d24a => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d24e; continue;
 }
 0x44d24e => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d252; continue;
 }
 0x44d252 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d256; continue;
 }
 0x44d256 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d25a; continue;
 }
 0x44d25a => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d25e; continue;
 }
 0x44d25e => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d262; continue;
 }
 0x44d262 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d266; continue;
 }
 0x44d266 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d26a; continue;
 }
 0x44d26a => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d26e; continue;
 }
 0x44d26e => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d272; continue;
 }
 0x44d272 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d276; continue;
 }
 0x44d276 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x44d281; continue; }
  pc = 0x44d27a; continue;
 }
 0x44d27a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d1aa; continue; }
  pc = 0x44d281; continue;
 }
 0x44d281 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d282 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d1aa; continue; }
  pc = 0x44d28a; continue;
 }
 0x44d28a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d28b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d28b); }
 let mut pc: u32 = 0x44d28b;
 loop { match pc {
 0x44d28b => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x44d290; continue;
 }
 0x44d290 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44d296); f_0044cddb(cpu); if cpu.returned_to != 0x44d296 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4bu32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44d368; continue; }
  pc = 0x44d2b9; continue;
 }
 0x44d2b9 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d368; continue; }
  pc = 0x44d2c9; continue;
 }
 0x44d2c9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x44d368; continue; }
  pc = 0x44d2d2; continue;
 }
 0x44d2d2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d368; continue; }
  pc = 0x44d2de; continue;
 }
 0x44d2de => {
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
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d30c; continue;
 }
 0x44d30c => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d310; continue;
 }
 0x44d310 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d314; continue;
 }
 0x44d314 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d318; continue;
 }
 0x44d318 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d31c; continue;
 }
 0x44d31c => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d320; continue;
 }
 0x44d320 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d324; continue;
 }
 0x44d324 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d328; continue;
 }
 0x44d328 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d32c; continue;
 }
 0x44d32c => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d330; continue;
 }
 0x44d330 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d334; continue;
 }
 0x44d334 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d338; continue;
 }
 0x44d338 => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d33c; continue;
 }
 0x44d33c => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d340; continue;
 }
 0x44d340 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d344; continue;
 }
 0x44d344 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d348; continue;
 }
 0x44d348 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d34c; continue;
 }
 0x44d34c => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d350; continue;
 }
 0x44d350 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d354; continue;
 }
 0x44d354 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d358; continue;
 }
 0x44d358 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d35c; continue;
 }
 0x44d35c => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44d367; continue; }
  pc = 0x44d360; continue;
 }
 0x44d360 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d290; continue; }
  pc = 0x44d367; continue;
 }
 0x44d367 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d368 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d290; continue; }
  pc = 0x44d370; continue;
 }
 0x44d370 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d371); }
 let mut pc: u32 = 0x44d371;
 loop { match pc {
 0x44d371 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44d38c; continue; }
  pc = 0x44d37d; continue;
 }
 0x44d37d => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44d38c; continue; }
  pc = 0x44d386; continue;
 }
 0x44d386 => {
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44d38c; continue;
 }
 0x44d38c => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x52u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4650u32); cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.w32(0x59da84u32, v); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x44d3cc; continue; }
  pc = 0x44d3b6; continue;
 }
 0x44d3b6 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esi)), 0x7265u16); }
  if !cpu.zf { pc = 0x44d3cc; continue; }
  pc = 0x44d3be; continue;
 }
 0x44d3be => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  pc = 0x44d3cc; continue;
 }
 0x44d3cc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5b3894u32), 0x0u8); }
  if cpu.zf { pc = 0x44d3e2; continue; }
  pc = 0x44d3de; continue;
 }
 0x44d3de => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44d3e2; continue;
 }
 0x44d3e2 => {
  cpu.push(0x44d3e7); f_0044cec2(cpu); if cpu.returned_to != 0x44d3e7 { return; }
  { let v: u16 = cpu.r16(0x575537u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x3eu32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44d3fc); cpu.call(t); if cpu.returned_to != 0x44d3fc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d400); }
 let mut pc: u32 = 0x44d400;
 loop { match pc {
 0x44d400 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44d43f; continue; }
  pc = 0x44d40a; continue;
 }
 0x44d40a => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44d43f; continue; }
  pc = 0x44d412; continue;
 }
 0x44d412 => {
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44d43f; continue; }
  pc = 0x44d41b; continue;
 }
 0x44d41b => {
  { let v: u16 = cpu.r16(0x4d67f0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44d435; continue; }
  pc = 0x44d428; continue;
 }
 0x44d428 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44d435; continue; }
  pc = 0x44d42f; continue;
 }
 0x44d42f => {
  { let v: u16 = cpu.r16(0x4d67f2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44d435; continue;
 }
 0x44d435 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b386eu32, v); }
  pc = 0x44d43f; continue;
 }
 0x44d43f => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5b3854u32, v); }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b386du32, v); }
  { cpu.sub32(cpu.r32(0x5b35acu32.wrapping_add(cpu.eax.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x44d4ad; continue; }
  pc = 0x44d49f; continue;
 }
 0x44d49f => {
  { let v: u8 = cpu.r8(0x5b386fu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44d4ac); f_0044e13f(cpu); if cpu.returned_to != 0x44d4ac { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d4ad => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5b3854u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x44d4ba); f_0044d522(cpu); if cpu.returned_to != 0x44d4ba { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1cu32); cpu.ebx = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44d4cc; continue;
 }
 0x44d4cc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x44d4d5; continue; }
  pc = 0x44d4d3; continue;
 }
 0x44d4d3 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44d4d5; continue;
 }
 0x44d4d5 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44d4cc; continue; }
  pc = 0x44d4db; continue;
 }
 0x44d4db => {
  { let v: u32 = (cpu.eax as u8) as u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44d4e9); f_0041b6f1(cpu); if cpu.returned_to != 0x44d4e9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4e1fe6u32), 0x30u8); }
  if !cpu.zf { pc = 0x44d504; continue; }
  pc = 0x44d4f3; continue;
 }
 0x44d4f3 => {
  { let v: u8 = cpu.r8(0x4e1fe7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fe6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fe7u32, v); }
  pc = 0x44d504; continue;
 }
 0x44d504 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x18u32); cpu.ebx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44d51e); f_0041cf28(cpu); if cpu.returned_to != 0x44d51e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d522(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d522); }
 let mut pc: u32 = 0x44d522;
 loop { match pc {
 0x44d522 => {
  { cpu.pushad(); }
  cpu.push(0x44d528); f_0045d200(cpu); if cpu.returned_to != 0x44d528 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5c4e6du32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4eab29u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5c4e71u32, v); }
  cpu.push(0x44d556); f_004209d3(cpu); if cpu.returned_to != 0x44d556 { return; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44d562; continue; }
  pc = 0x44d55d; continue;
 }
 0x44d55d => {
  cpu.push(0x44d562); f_00447617(cpu); if cpu.returned_to != 0x44d562 { return; }
  pc = 0x44d562; continue;
 }
 0x44d562 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44d56d); f_00420d85(cpu); if cpu.returned_to != 0x44d56d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x2au32; cpu.ecx = v; }
  { let v = cpu.r32(0x5431b4u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(v); }
  { let v: u32 = 0x5932dcu32; cpu.w32(0x5431b4u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44d597); cpu.call(t); if cpu.returned_to != 0x44d597 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x5431b4u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x14u32); cpu.ebx = v; }
  { let v: u32 = 0x1278u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44d5b6); cpu.call(t); if cpu.returned_to != 0x44d5b6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44d5c1); f_0045d1e7(cpu); if cpu.returned_to != 0x44d5c1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d5c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d5c5); }
 let mut pc: u32 = 0x44d5c5;
 loop { match pc {
 0x44d5c5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44d5ec; continue; }
  pc = 0x44d5df; continue;
 }
 0x44d5df => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44d5ec; continue;
 }
 0x44d5ec => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5b2dbcu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x5b2dc0u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0x43u32; cpu.eax = v; }
  { let v: u32 = 0x3eu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x5b3869u32), 0x3e8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44d61b; continue; }
  pc = 0x44d612; continue;
 }
 0x44d612 => {
  { cpu.sub8(cpu.r8(0x5b3868u32), 0x1u8); }
  if cpu.zf { pc = 0x44d61b; continue; }
  pc = 0x44d61b; continue;
 }
 0x44d61b => {
  { let v: u32 = 0x26u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44d626); f_004714a5(cpu); if cpu.returned_to != 0x44d626 { return; }
  if !cpu.cf { pc = 0x44d62e; continue; }
  pc = 0x44d628; continue;
 }
 0x44d628 => {
  cpu.push(0x44d62d); f_0044d7b5(cpu); if cpu.returned_to != 0x44d62d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d62e => {
  { let v: u32 = 0x43u32; cpu.eax = v; }
  { let v: u32 = 0x2fu32; cpu.ebx = v; }
  { let v: u32 = 0x26u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44d643); f_004714a5(cpu); if cpu.returned_to != 0x44d643 { return; }
  if !cpu.cf { pc = 0x44d64b; continue; }
  pc = 0x44d645; continue;
 }
 0x44d645 => {
  cpu.push(0x44d64a); f_0044d928(cpu); if cpu.returned_to != 0x44d64a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d64b => {
  { let v: u32 = 0x35u32; cpu.eax = v; }
  { let v: u32 = 0x15u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44d670; continue; }
  pc = 0x44d664; continue;
 }
 0x44d664 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  pc = 0x44d670; continue;
 }
 0x44d670 => {
  cpu.push(0x44d675); f_004714a5(cpu); if cpu.returned_to != 0x44d675 { return; }
  if !cpu.cf { pc = 0x44d67d; continue; }
  pc = 0x44d677; continue;
 }
 0x44d677 => {
  cpu.push(0x44d67c); f_0044d9c6(cpu); if cpu.returned_to != 0x44d67c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d67d => {
  { let v: u32 = 0x35u32; cpu.eax = v; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44d6a2; continue; }
  pc = 0x44d696; continue;
 }
 0x44d696 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  pc = 0x44d6a2; continue;
 }
 0x44d6a2 => {
  cpu.push(0x44d6a7); f_004714a5(cpu); if cpu.returned_to != 0x44d6a7 { return; }
  if !cpu.cf { pc = 0x44d6af; continue; }
  pc = 0x44d6a9; continue;
 }
 0x44d6a9 => {
  cpu.push(0x44d6ae); f_0044da18(cpu); if cpu.returned_to != 0x44d6ae { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d6af => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u32 = 0x16u32; cpu.ebx = v; }
  { let v: u32 = 0x2bu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x25u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44d6c4); f_004714a5(cpu); if cpu.returned_to != 0x44d6c4 { return; }
  if !cpu.cf { pc = 0x44d6cc; continue; }
  pc = 0x44d6c6; continue;
 }
 0x44d6c6 => {
  cpu.push(0x44d6cb); f_0044d7b5(cpu); if cpu.returned_to != 0x44d6cb { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d6cc => {
  cpu.push(0x44d6d1); f_0044d6ec(cpu); if cpu.returned_to != 0x44d6d1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d6d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d6d2); }
 let mut pc: u32 = 0x44d6d2;
 loop { match pc {
 0x44d6d2 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x44d6eb; continue; }
  pc = 0x44d6db; continue;
 }
 0x44d6db => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if !cpu.zf { pc = 0x44d6eb; continue; }
  pc = 0x44d6e4; continue;
 }
 0x44d6e4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3855u32, v); }
  pc = 0x44d6eb; continue;
 }
 0x44d6eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d6ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d6ec); }
 let mut pc: u32 = 0x44d6ec;
 loop { match pc {
 0x44d6ec => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x44d73b; continue; }
  pc = 0x44d6fc; continue;
 }
 0x44d6fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44d723; continue; }
  pc = 0x44d716; continue;
 }
 0x44d716 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44d723; continue;
 }
 0x44d723 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dc8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2dccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d73b => {
  { let v: u32 = cpu.r32(0x5b2dd0u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x5b2dd4u32)); cpu.eax = v; }
  if cpu.zf { pc = 0x44d764; continue; }
  pc = 0x44d748; continue;
 }
 0x44d748 => {
  cpu.push(0x44d74d); f_0044cab9(cpu); if cpu.returned_to != 0x44d74d { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2ddcu32, v); }
  pc = 0x44d764; continue;
 }
 0x44d764 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44d78b; continue; }
  pc = 0x44d77e; continue;
 }
 0x44d77e => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44d78b; continue;
 }
 0x44d78b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dd0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2dd4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dc8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b2dccu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3855u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d7b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d7b5); }
 let mut pc: u32 = 0x44d7b5;
 loop { match pc {
 0x44d7b5 => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if cpu.zf { pc = 0x44d7bf; continue; }
  pc = 0x44d7be; continue;
 }
 0x44d7be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d7bf => {
  { cpu.sub8(cpu.r8(0x5b3856u32), 0x0u8); }
  if cpu.zf { pc = 0x44d7c9; continue; }
  pc = 0x44d7c8; continue;
 }
 0x44d7c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d7c9 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5b3857u32, v); }
  f_0044d7d0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d7d0); }
 let mut pc: u32 = 0x44d7d0;
 loop { match pc {
 0x44d7d0 => {
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44d86d; continue; }
  pc = 0x44d7e1; continue;
 }
 0x44d7e1 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44d7fd; continue; }
  pc = 0x44d7e8; continue;
 }
 0x44d7e8 => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if !cpu.zf { pc = 0x44d7fb; continue; }
  pc = 0x44d7f1; continue;
 }
 0x44d7f1 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5b2dc4u32, v); }
  pc = 0x44d7fb; continue;
 }
 0x44d7fb => {
  pc = 0x44d863; continue;
 }
 0x44d7fd => {
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44d807); f_0044e79b(cpu); if cpu.returned_to != 0x44d807 { return; }
  if cpu.cf { pc = 0x44d863; continue; }
  pc = 0x44d809; continue;
 }
 0x44d809 => {
  { let v: u32 = 0x16u32; cpu.w32(0x5b2dc4u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44d823; continue; }
  pc = 0x44d81c; continue;
 }
 0x44d81c => {
  { let v: u32 = cpu.r32(0x5b2dc4u32).wrapping_add(0xau32); cpu.w32(0x5b2dc4u32, v); }
  pc = 0x44d823; continue;
 }
 0x44d823 => {
  { let v: u8 = cpu.r8(0x5b3853u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44d82d); f_0044d8fe(cpu); if cpu.returned_to != 0x44d82d { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x44d838); f_00416a62(cpu); if cpu.returned_to != 0x44d838 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44d864; continue; }
  pc = 0x44d840; continue;
 }
 0x44d840 => {
  cpu.push(0x44d845); f_0045d200(cpu); if cpu.returned_to != 0x44d845 { return; }
  cpu.push(0x44d84a); f_0041c6a6(cpu); if cpu.returned_to != 0x44d84a { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b35fcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x44d85e); f_0045d4b4(cpu); if cpu.returned_to != 0x44d85e { return; }
  cpu.push(0x44d863); f_0045d1e7(cpu); if cpu.returned_to != 0x44d863 { return; }
  pc = 0x44d863; continue;
 }
 0x44d863 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d864 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x5b2dc4u32), 0x12u32); cpu.w32(0x5b2dc4u32, v); }
  cpu.poll(); pc = 0x44d863; continue;
 }
 0x44d86d => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x44d880); f_00419d49(cpu); if cpu.returned_to != 0x44d880 { return; }
  { cpu.popad(); }
  cpu.push(0x44d886); f_0044d89b(cpu); if cpu.returned_to != 0x44d886 { return; }
  cpu.push(0x44d88b); f_00419954(cpu); if cpu.returned_to != 0x44d88b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  cpu.push(0x44d89a); f_00471ec2(cpu); if cpu.returned_to != 0x44d89a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d89b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d89b); }
 let mut pc: u32 = 0x44d89b;
 loop { match pc {
 0x44d89b => {
  { cpu.sub32(cpu.r32(0x5b3590u32), 0x5b3887u32); }
  if !cpu.zf { pc = 0x44d8b9; continue; }
  pc = 0x44d8a7; continue;
 }
 0x44d8a7 => {
  { cpu.sub32(cpu.r32(0x5b3594u32), cpu.ebp); }
  if !cpu.zf { pc = 0x44d8b9; continue; }
  pc = 0x44d8af; continue;
 }
 0x44d8af => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  pc = 0x44d8b9; continue;
 }
 0x44d8b9 => {
  cpu.push(0x44d8be); f_0044d8d9(cpu); if cpu.returned_to != 0x44d8be { return; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x10u32); }
  if cpu.zf { pc = 0x44d8d1; continue; }
  pc = 0x44d8ca; continue;
 }
 0x44d8ca => {
  { let v: u8 = 0x20u8; cpu.w8(0x5b3962u32, v); }
  pc = 0x44d8d1; continue;
 }
 0x44d8d1 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5b3867u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d8d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d8d9); }
 let mut pc: u32 = 0x44d8d9;
 loop { match pc {
 0x44d8d9 => {
  { cpu.pushad(); }
  { let v: u8 = 0xf7u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x522514u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  cpu.push(0x44d8fc); f_00413337(cpu); if cpu.returned_to != 0x44d8fc { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d8fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d8fe); }
 let mut pc: u32 = 0x44d8fe;
 loop { match pc {
 0x44d8fe => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x522514u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68aeu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d68b0u32, v); }
  cpu.push(0x44d926); f_00413337(cpu); if cpu.returned_to != 0x44d926 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d928(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d928); }
 let mut pc: u32 = 0x44d928;
 loop { match pc {
 0x44d928 => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if cpu.zf { pc = 0x44d932; continue; }
  pc = 0x44d931; continue;
 }
 0x44d931 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d932 => {
  { cpu.sub8(cpu.r8(0x5b3857u32), 0x0u8); }
  if cpu.zf { pc = 0x44d93c; continue; }
  pc = 0x44d93b; continue;
 }
 0x44d93b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d93c => {
  { let v: u8 = 0x3u8; cpu.w8(0x5b3856u32, v); }
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44d96b; continue; }
  pc = 0x44d950; continue;
 }
 0x44d950 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44d96b; continue; }
  pc = 0x44d957; continue;
 }
 0x44d957 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x44d962); f_00416a62(cpu); if cpu.returned_to != 0x44d962 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44d96a); f_0044d8fe(cpu); if cpu.returned_to != 0x44d96a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d96b => {
  { let v: u32 = 0x4u32; cpu.w32(0x5b2dc4u32, v); }
  { cpu.sub8(cpu.r8(0x58c492u32), 0x0u8); }
  if cpu.zf { pc = 0x44d97e; continue; }
  pc = 0x44d97e; continue;
 }
 0x44d97e => {
  f_0044d990(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d980); }
 let mut pc: u32 = 0x44d980;
 loop { match pc {
 0x44d980 => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if !cpu.zf { pc = 0x44d98a; continue; }
  pc = 0x44d989; continue;
 }
 0x44d989 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d98a => {
  cpu.push(0x44d98f); f_0044d990(cpu); if cpu.returned_to != 0x44d98f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d990); }
 let mut pc: u32 = 0x44d990;
 loop { match pc {
 0x44d990 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  { let v: u8 = cpu.r8(0x5b386eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44d9b4; continue; }
  pc = 0x44d9a1; continue;
 }
 0x44d9a1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3853u32, v); }
  { cpu.sub8(cpu.r8(0x5b3894u32), 0x0u8); }
  if !cpu.zf { pc = 0x44d9b4; continue; }
  pc = 0x44d9af; continue;
 }
 0x44d9af => {
  cpu.push(0x44d9b4); f_0044d7d0(cpu); if cpu.returned_to != 0x44d9b4 { return; }
  pc = 0x44d9b4; continue;
 }
 0x44d9b4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  cpu.push(0x44d9c3); f_00471ec2(cpu); if cpu.returned_to != 0x44d9c3 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044d9c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44d9c6); }
 let mut pc: u32 = 0x44d9c6;
 loop { match pc {
 0x44d9c6 => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if cpu.zf { pc = 0x44d9d0; continue; }
  pc = 0x44d9cf; continue;
 }
 0x44d9cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d9d0 => {
  { cpu.sub8(cpu.r8(0x5b3856u32), 0x0u8); }
  if cpu.zf { pc = 0x44d9da; continue; }
  pc = 0x44d9d9; continue;
 }
 0x44d9d9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d9da => {
  { cpu.sub8(cpu.r8(0x5b3857u32), 0x0u8); }
  if cpu.zf { pc = 0x44d9e4; continue; }
  pc = 0x44d9e3; continue;
 }
 0x44d9e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44d9e4 => {
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44da17; continue; }
  pc = 0x44d9f1; continue;
 }
 0x44d9f1 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5b3851u32, v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44da0c; continue; }
  pc = 0x44d9ff; continue;
 }
 0x44d9ff => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44da12; continue; }
  pc = 0x44da06; continue;
 }
 0x44da06 => {
  cpu.push(0x44da0b); f_0044d059(cpu); if cpu.returned_to != 0x44da0b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da0c => {
  cpu.push(0x44da11); f_0044cef9(cpu); if cpu.returned_to != 0x44da11 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da12 => {
  cpu.push(0x44da17); f_0044d28b(cpu); if cpu.returned_to != 0x44da17 { return; }
  pc = 0x44da17; continue;
 }
 0x44da17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044da18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44da18); }
 let mut pc: u32 = 0x44da18;
 loop { match pc {
 0x44da18 => {
  { cpu.sub32(cpu.r32(0x5b2dc4u32), 0x0u32); }
  if cpu.zf { pc = 0x44da22; continue; }
  pc = 0x44da21; continue;
 }
 0x44da21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da22 => {
  { cpu.sub8(cpu.r8(0x5b3856u32), 0x0u8); }
  if cpu.zf { pc = 0x44da2c; continue; }
  pc = 0x44da2b; continue;
 }
 0x44da2b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da2c => {
  { cpu.sub8(cpu.r8(0x5b3857u32), 0x0u8); }
  if cpu.zf { pc = 0x44da36; continue; }
  pc = 0x44da35; continue;
 }
 0x44da35 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da36 => {
  { let v: u32 = cpu.r32(0x5b3588u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44da69; continue; }
  pc = 0x44da43; continue;
 }
 0x44da43 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5b3852u32, v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44da5e; continue; }
  pc = 0x44da51; continue;
 }
 0x44da51 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44da64; continue; }
  pc = 0x44da58; continue;
 }
 0x44da58 => {
  cpu.push(0x44da5d); f_0044d0ff(cpu); if cpu.returned_to != 0x44da5d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da5e => {
  cpu.push(0x44da63); f_0044cfa7(cpu); if cpu.returned_to != 0x44da63 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44da64 => {
  cpu.push(0x44da69); f_0044d1a5(cpu); if cpu.returned_to != 0x44da69 { return; }
  pc = 0x44da69; continue;
 }
 0x44da69 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044da6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44da6a); }
 let mut pc: u32 = 0x44da6a;
 loop { match pc {
 0x44da6a => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44db1e; continue; }
  pc = 0x44da75; continue;
 }
 0x44da75 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x35u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44da89); cpu.call(t); if cpu.returned_to != 0x44da89 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44db1e; continue; }
  pc = 0x44da97; continue;
 }
 0x44da97 => {
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  cpu.push(0x44dac0); f_00485b77(cpu); if cpu.returned_to != 0x44dac0 { return; }
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
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
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
  { cpu.sub32(cpu.ecx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x44db1b; continue; }
  pc = 0x44daf7; continue;
 }
 0x44daf7 => {
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.sf { pc = 0x44db1b; continue; }
  pc = 0x44dafc; continue;
 }
 0x44dafc => {
  if cpu.zf { pc = 0x44db1b; continue; }
  pc = 0x44dafe; continue;
 }
 0x44dafe => {
  { let v = cpu.r32(0x4d6ba8u32); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6ba8u32, v); }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44db15); cpu.call(t); if cpu.returned_to != 0x44db15 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6ba8u32, v); }
  pc = 0x44db1b; continue;
 }
 0x44db1b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44db1e; continue;
 }
 0x44db1e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044db1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44db1f); }
 let mut pc: u32 = 0x44db1f;
 loop { match pc {
 0x44db1f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044db21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44db21); }
 let mut pc: u32 = 0x44db21;
 loop { match pc {
 0x44db21 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x43u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2fu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b3856u32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1269u32); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44db45); cpu.call(t); if cpu.returned_to != 0x44db45 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x43u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3eu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x1279u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44db61; continue; }
  pc = 0x44db5c; continue;
 }
 0x44db5c => {
  { let v: u32 = 0x126bu32; cpu.ecx = v; }
  pc = 0x44db61; continue;
 }
 0x44db61 => {
  { cpu.sub8(cpu.r8(0x5b3857u32), 0x0u8); }
  if cpu.zf { pc = 0x44db6b; continue; }
  pc = 0x44db6a; continue;
 }
 0x44db6a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x44db6b; continue;
 }
 0x44db6b => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44db71); cpu.call(t); if cpu.returned_to != 0x44db71 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44dbc3; continue; }
  pc = 0x44db7b; continue;
 }
 0x44db7b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b3851u32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x126du32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44db9c); cpu.call(t); if cpu.returned_to != 0x44db9c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b3852u32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x126fu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dbc0); cpu.call(t); if cpu.returned_to != 0x44dbc0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44dbc3; continue;
 }
 0x44dbc3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044dbc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44dbc4); }
 let mut pc: u32 = 0x44dbc4;
 loop { match pc {
 0x44dbc4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x44dbfc; continue; }
  pc = 0x44dbcf; continue;
 }
 0x44dbcf => {
  { let v: u32 = 0x1262u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44dbec; continue; }
  pc = 0x44dbdb; continue;
 }
 0x44dbdb => {
  { let v: u32 = 0x1263u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44dbec; continue; }
  pc = 0x44dbe7; continue;
 }
 0x44dbe7 => {
  { let v: u32 = 0x1264u32; cpu.ecx = v; }
  pc = 0x44dbec; continue;
 }
 0x44dbec => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dbf8); cpu.call(t); if cpu.returned_to != 0x44dbf8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dbfc => {
  { let v: u32 = 0x1265u32; cpu.ecx = v; }
  cpu.poll(); pc = 0x44dbec; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044dc03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44dc03); }
 let mut pc: u32 = 0x44dc03;
 loop { match pc {
 0x44dc03 => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1au32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x44dc8b; continue; }
  pc = 0x44dc0f; continue;
 }
 0x44dc0f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
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
  cpu.push(0x44dc4a); f_0043df26(cpu); if cpu.returned_to != 0x44dc4a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44dc5f); f_0041b6f1(cpu); if cpu.returned_to != 0x44dc5f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x25u8; cpu.w8(0x4e1fe8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fe9u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5bu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  cpu.push(0x44dc87); f_0041cf28(cpu); if cpu.returned_to != 0x44dc87 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dc8b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x5b384cu32; cpu.esi = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x59u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  cpu.push(0x44dca3); f_0041cf28(cpu); if cpu.returned_to != 0x44dca3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044dca7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44dca7); }
 let mut pc: u32 = 0x44dca7;
 loop { match pc {
 0x44dca7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1261u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dcb5); cpu.call(t); if cpu.returned_to != 0x44dcb5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x76cu32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x16u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dce6); cpu.call(t); if cpu.returned_to != 0x44dce6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044dcea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44dcea); }
 let mut pc: u32 = 0x44dcea;
 loop { match pc {
 0x44dcea => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3894u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x44dd66; continue; }
  pc = 0x44dcfc; continue;
 }
 0x44dcfc => {
  { let v: u32 = 0x1268u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44dd19; continue; }
  pc = 0x44dd08; continue;
 }
 0x44dd08 => {
  { let v: u32 = 0x1266u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44dd19; continue; }
  pc = 0x44dd14; continue;
 }
 0x44dd14 => {
  { let v: u32 = 0x1267u32; cpu.ecx = v; }
  pc = 0x44dd19; continue;
 }
 0x44dd19 => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dd25); cpu.call(t); if cpu.returned_to != 0x44dd25 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44dd2d); f_0044de01(cpu); if cpu.returned_to != 0x44dd2d { return; }
  { cpu.and8(cpu.r8(0x4d677eu32), 0x2u8); }
  if cpu.zf { pc = 0x44dd65; continue; }
  pc = 0x44dd36; continue;
 }
 0x44dd36 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1275u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44dd56; continue; }
  pc = 0x44dd45; continue;
 }
 0x44dd45 => {
  { let v: u32 = 0x1273u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44dd56; continue; }
  pc = 0x44dd51; continue;
 }
 0x44dd51 => {
  { let v: u32 = 0x1274u32; cpu.ecx = v; }
  pc = 0x44dd56; continue;
 }
 0x44dd56 => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x13u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dd62); cpu.call(t); if cpu.returned_to != 0x44dd62 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44dd65; continue;
 }
 0x44dd65 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dd66 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1266u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b385cu32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x44dd19; continue; }
  pc = 0x44dd77; continue;
 }
 0x44dd77 => {
  { let v: u32 = 0x1267u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b385au32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x44dd19; continue; }
  pc = 0x44dd85; continue;
 }
 0x44dd85 => {
  { let v: u32 = 0x1268u32; cpu.ecx = v; }
  cpu.poll(); pc = 0x44dd19; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ddbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ddbd); }
 let mut pc: u32 = 0x44ddbd;
 loop { match pc {
 0x44ddbd => {
  { cpu.sub8(cpu.r8(0x5b3894u32), 0x0u8); }
  if !cpu.zf { pc = 0x44de00; continue; }
  pc = 0x44ddc6; continue;
 }
 0x44ddc6 => {
  { let v: u32 = cpu.r32(0x5b2dbcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2dc0u32); cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x1277u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44dde3); cpu.call(t); if cpu.returned_to != 0x44dde3 { return; }
  { cpu.popad(); }
  { cpu.and8(cpu.r8(0x4d677eu32), 0x2u8); }
  if cpu.zf { pc = 0x44de00; continue; }
  pc = 0x44dded; continue;
 }
 0x44dded => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x1276u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44ddff); cpu.call(t); if cpu.returned_to != 0x44ddff { return; }
  { cpu.popad(); }
  pc = 0x44de00; continue;
 }
 0x44de00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044de01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44de01); }
 let mut pc: u32 = 0x44de01;
 loop { match pc {
 0x44de01 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b3894u32, v); }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44de16; continue; }
  pc = 0x44de0f; continue;
 }
 0x44de0f => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3894u32, v); }
  pc = 0x44de16; continue;
 }
 0x44de16 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044de17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44de17); }
 let mut pc: u32 = 0x44de17;
 loop { match pc {
 0x44de17 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if !cpu.zf { pc = 0x44de2b; continue; }
  pc = 0x44de22; continue;
 }
 0x44de22 => {
  cpu.push(0x44de27); f_0044df62(cpu); if cpu.returned_to != 0x44de27 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44de2b => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44df31; continue; }
  pc = 0x44de36; continue;
 }
 0x44de36 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44dec2; continue; }
  pc = 0x44de41; continue;
 }
 0x44de41 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44de48); f_00445c78(cpu); if cpu.returned_to != 0x44de48 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44de8b; continue; }
  pc = 0x44de4e; continue;
 }
 0x44de4e => {
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
  { let v: u32 = cpu.eax; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44de87); f_0044e079(cpu); if cpu.returned_to != 0x44de87 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44de8b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x5b388fu32; cpu.esi = v; }
  cpu.push(0x44dea2); f_0041cf28(cpu); if cpu.returned_to != 0x44dea2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dea6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x5b388fu32; cpu.esi = v; }
  cpu.push(0x44debe); f_0041cf28(cpu); if cpu.returned_to != 0x44debe { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dec2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x5b3853u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  cpu.push(0x44dee0); f_00485b77(cpu); if cpu.returned_to != 0x44dee0 { return; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44def4); f_00445c78(cpu); if cpu.returned_to != 0x44def4 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x44dea6; continue; }
  pc = 0x44defd; continue;
 }
 0x44defd => {
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44df2d); f_0044e079(cpu); if cpu.returned_to != 0x44df2d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44df31 => {
  { cpu.and8(cpu.r8(0x4d677eu32), 0x2u8); }
  if cpu.zf { pc = 0x44df5e; continue; }
  pc = 0x44df3a; continue;
 }
 0x44df3a => {
  { let v: u32 = 0x0u32; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44df5e); f_0044e079(cpu); if cpu.returned_to != 0x44df5e { return; }
  pc = 0x44df5e; continue;
 }
 0x44df5e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044df62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44df62); }
 let mut pc: u32 = 0x44df62;
 loop { match pc {
 0x44df62 => {
  { cpu.sub8(cpu.r8(0x5b385bu32), 0xffu8); }
  if !cpu.zf { pc = 0x44e048; continue; }
  pc = 0x44df6f; continue;
 }
 0x44df6f => {
  { cpu.sub8(cpu.r8(0x5b385cu32), 0xffu8); }
  if cpu.zf { pc = 0x44dfd9; continue; }
  pc = 0x44df78; continue;
 }
 0x44df78 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44df7f); f_00445c78(cpu); if cpu.returned_to != 0x44df7f { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x44dfa2; continue; }
  pc = 0x44df85; continue;
 }
 0x44df85 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5b35a8u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x44dfa2; continue;
 }
 0x44dfa2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44dfbf); f_0044e079(cpu); if cpu.returned_to != 0x44dfbf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44dfd9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e048 => {
  { cpu.and8(cpu.r8(0x4d677eu32), 0x2u8); }
  if cpu.zf { pc = 0x44e075; continue; }
  pc = 0x44e051; continue;
 }
 0x44e051 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  cpu.push(0x44e075); f_0044e079(cpu); if cpu.returned_to != 0x44e075 { return; }
  pc = 0x44e075; continue;
 }
 0x44e075 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e079(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e079); }
 let mut pc: u32 = 0x44e079;
 loop { match pc {
 0x44e079 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d00ebu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x44e096; continue; }
  pc = 0x44e087; continue;
 }
 0x44e087 => {
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3bu32; cpu.w32(0x5d00efu32, v); }
  pc = 0x44e096; continue;
 }
 0x44e096 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x44e0a0); f_0041b6f1(cpu); if cpu.returned_to != 0x44e0a0 { return; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d00efu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x44e0c1); f_0041b6f1(cpu); if cpu.returned_to != 0x44e0c1 { return; }
  { let v: u16 = cpu.r16(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  cpu.push(0x44e0db); f_0041cf28(cpu); if cpu.returned_to != 0x44e0db { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e0dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e0dc); }
 let mut pc: u32 = 0x44e0dc;
 loop { match pc {
 0x44e0dc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d00ebu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x44e0f9; continue; }
  pc = 0x44e0ea; continue;
 }
 0x44e0ea => {
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3bu32; cpu.w32(0x5d00efu32, v); }
  pc = 0x44e0f9; continue;
 }
 0x44e0f9 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x44e103); f_0041b6f1(cpu); if cpu.returned_to != 0x44e103 { return; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d00efu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x44e124); f_0041b6f1(cpu); if cpu.returned_to != 0x44e124 { return; }
  { let v: u16 = cpu.r16(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  cpu.push(0x44e13e); f_0041cbda(cpu); if cpu.returned_to != 0x44e13e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e13f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e13f); }
 let mut pc: u32 = 0x44e13f;
 loop { match pc {
 0x44e13f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c6feu32, v); }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44e157; continue; }
  pc = 0x44e150; continue;
 }
 0x44e150 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c6feu32, v); }
  pc = 0x44e157; continue;
 }
 0x44e157 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5b360cu32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5b3610u32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.ebx = v; }
  { let v: u32 = 0x4d75b4u32; cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = 0x5b3478u32; cpu.esi = v; }
  { let v: u32 = 0x5b3530u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x13u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1du32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x44e2b2; continue; }
  pc = 0x44e1bf; continue;
 }
 0x44e1bf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x15u8); }
  if cpu.zf { pc = 0x44e2b2; continue; }
  pc = 0x44e1c9; continue;
 }
 0x44e1c9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x16u8); }
  if cpu.zf { pc = 0x44e2b2; continue; }
  pc = 0x44e1d3; continue;
 }
 0x44e1d3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x17u8); }
  if cpu.zf { pc = 0x44e2b2; continue; }
  pc = 0x44e1dd; continue;
 }
 0x44e1dd => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.esi))); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.cf { pc = 0x44e1ee; continue; }
  pc = 0x44e1ea; continue;
 }
 0x44e1ea => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x44e1ee; continue;
 }
 0x44e1ee => {
  { let v: u16 = 0x64u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x64u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x44e214; continue; }
  pc = 0x44e200; continue;
 }
 0x44e200 => {
  { cpu.sub8(cpu.r8(0x4d677eu32), 0x0u8); }
  if !cpu.zf { pc = 0x44e26a; continue; }
  pc = 0x44e209; continue;
 }
 0x44e209 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x44e263; continue; }
  pc = 0x44e212; continue;
 }
 0x44e212 => {
  pc = 0x44e26a; continue;
 }
 0x44e214 => {
  { let v: u8 = 0x2u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x44e26a; continue; }
  pc = 0x44e221; continue;
 }
 0x44e221 => {
  { cpu.and8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x44e242; continue; }
  pc = 0x44e22a; continue;
 }
 0x44e22a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x96u32.wrapping_add(cpu.esi))); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x7u8); }
  if !cpu.zf { pc = 0x44e26a; continue; }
  pc = 0x44e239; continue;
 }
 0x44e239 => {
  { let v: u8 = cpu.or8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x40u8); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x44e26a; continue;
 }
 0x44e242 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44e25a; continue; }
  pc = 0x44e250; continue;
 }
 0x44e250 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x44e26a; continue;
 }
 0x44e25a => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x44e26a; continue;
 }
 0x44e263 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x7u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x44e26a; continue;
 }
 0x44e26a => {
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x44e28e); f_00487628(cpu); if cpu.returned_to != 0x44e28e { return; }
  cpu.push(0x44e293); f_00486d60(cpu); if cpu.returned_to != 0x44e293 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c6feu32, v); }
  { let v: u32 = 0x4d75b4u32; cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e2b2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x4d677eu32), 0x0u8); }
  if !cpu.zf { pc = 0x44e2e1; continue; }
  pc = 0x44e2d1; continue;
 }
 0x44e2d1 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x44e2e1; continue; }
  pc = 0x44e2da; continue;
 }
 0x44e2da => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)) & 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x44e2e1; continue;
 }
 0x44e2e1 => {
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5b3858u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x44e359; continue; }
  pc = 0x44e32b; continue;
 }
 0x44e32b => {
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x15u8); }
  if cpu.zf { pc = 0x44e359; continue; }
  pc = 0x44e341; continue;
 }
 0x44e341 => {
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x44e359; continue; }
  pc = 0x44e34d; continue;
 }
 0x44e34d => {
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x44e359; continue; }
  pc = 0x44e359; continue;
 }
 0x44e359 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b385eu32, v); }
  cpu.push(0x44e365); f_0048658b(cpu); if cpu.returned_to != 0x44e365 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b385eu32, v); }
  cpu.push(0x44e371); f_00486d60(cpu); if cpu.returned_to != 0x44e371 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c6feu32, v); }
  { let v: u32 = 0x4d75b4u32; cpu.w32(0x4d6ab4u32, v); }
  { let v: u32 = 0x4d8236u32; cpu.w32(0x4d6ac0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e390); }
 let mut pc: u32 = 0x44e390;
 loop { match pc {
 0x44e390 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x44e39a; continue; }
  pc = 0x44e399; continue;
 }
 0x44e399 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e39a => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x0u32); }
  if cpu.zf { pc = 0x44e502; continue; }
  pc = 0x44e3a7; continue;
 }
 0x44e3a7 => {
  { cpu.sub8(cpu.r8(0x59d9e2u32), 0x0u8); }
  if !cpu.zf { pc = 0x44e502; continue; }
  pc = 0x44e3b4; continue;
 }
 0x44e3b4 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x44e502; continue; }
  pc = 0x44e3c1; continue;
 }
 0x44e3c1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x44e502; continue; }
  pc = 0x44e3d5; continue;
 }
 0x44e3d5 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5b358cu32)); cpu.w32(0x5b358cu32, v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x5b358cu32), 0xaau32); }
  if (cpu.sf != cpu.of) { pc = 0x44e3f6; continue; }
  pc = 0x44e3e9; continue;
 }
 0x44e3e9 => {
  { cpu.and8(cpu.r8(0x4d677eu32), 0x2u8); }
  if !cpu.zf { pc = 0x44e502; continue; }
  pc = 0x44e3f6; continue;
 }
 0x44e3f6 => {
  { let v: u16 = 0x56u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0xd6u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x16u16; cpu.w16(0x5b3865u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44e447; continue; }
  pc = 0x44e423; continue;
 }
 0x44e423 => {
  { let v: u16 = 0xfau16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0x16eu16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x16u16; cpu.w16(0x5b3865u32, v); }
  pc = 0x44e447; continue;
 }
 0x44e447 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44e46e; continue; }
  pc = 0x44e461; continue;
 }
 0x44e461 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44e46e; continue;
 }
 0x44e46e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b385fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x44e497; continue; }
  pc = 0x44e477; continue;
 }
 0x44e477 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b3861u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44e497; continue; }
  pc = 0x44e480; continue;
 }
 0x44e480 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3863u32)); }
  if (cpu.sf != cpu.of) { pc = 0x44e497; continue; }
  pc = 0x44e489; continue;
 }
 0x44e489 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3865u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44e497; continue; }
  pc = 0x44e492; continue;
 }
 0x44e492 => {
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  pc = 0x44e497; continue;
 }
 0x44e497 => {
  { let v: u32 = cpu.r32(0x5b3590u32); cpu.esi = v; }
  { let v: u32 = 0x12d2u32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x5b387eu32); }
  if cpu.zf { pc = 0x44e4c9; continue; }
  pc = 0x44e4aa; continue;
 }
 0x44e4aa => {
  { let v: u32 = 0x12d4u32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x5b3882u32); }
  if cpu.zf { pc = 0x44e4c9; continue; }
  pc = 0x44e4b7; continue;
 }
 0x44e4b7 => {
  { let v: u32 = 0x12d8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x5b388bu32); }
  if cpu.zf { pc = 0x44e4c9; continue; }
  pc = 0x44e4c4; continue;
 }
 0x44e4c4 => {
  { let v: u32 = 0x12d6u32; cpu.ecx = v; }
  pc = 0x44e4c9; continue;
 }
 0x44e4c9 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x5b385fu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5b3863u32) as u32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x44e4ef); cpu.call(t); if cpu.returned_to != 0x44e4ef { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44e4f7); cpu.call(t); if cpu.returned_to != 0x44e4f7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x78u32); cpu.eax = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44e502); cpu.call(t); if cpu.returned_to != 0x44e502 { return; }
  pc = 0x44e502; continue;
 }
 0x44e502 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e503(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e503); }
 let mut pc: u32 = 0x44e503;
 loop { match pc {
 0x44e503 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x44e60f; continue; }
  pc = 0x44e517; continue;
 }
 0x44e517 => {
  cpu.push(0x44e51c); f_0044e710(cpu); if cpu.returned_to != 0x44e51c { return; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x5b2de4u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x5b2de0u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x575573u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x575577u32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x57557bu32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x44e56b; continue; }
  pc = 0x44e561; continue;
 }
 0x44e561 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x57557bu32, v); }
  pc = 0x44e56b; continue;
 }
 0x44e56b => {
  { let v: u32 = cpu.r32(0x57557bu32).wrapping_sub(0x20u32); cpu.w32(0x57557bu32, v); }
  { let v: u32 = 0x5b2de8u32; cpu.edi = v; }
  pc = 0x44e577; continue;
 }
 0x44e577 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44e605; continue; }
  pc = 0x44e583; continue;
 }
 0x44e583 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x44e605; continue; }
  pc = 0x44e59b; continue;
 }
 0x44e59b => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e605; continue; }
  pc = 0x44e5a2; continue;
 }
 0x44e5a2 => {
  cpu.push(0x44e5a7); f_0044e65c(cpu); if cpu.returned_to != 0x44e5a7 { return; }
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e5d1; continue; }
  pc = 0x44e5ae; continue;
 }
 0x44e5ae => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e5d1; continue; }
  pc = 0x44e5bc; continue;
 }
 0x44e5bc => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e5d1; continue; }
  pc = 0x44e5ca; continue;
 }
 0x44e5ca => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  pc = 0x44e5d1; continue;
 }
 0x44e5d1 => {
  cpu.push(0x44e5d6); f_0044e610(cpu); if cpu.returned_to != 0x44e5d6 { return; }
  { let v: u32 = cpu.r32(0x5b2de0u32).wrapping_add(0x10u32); cpu.w32(0x5b2de0u32, v); }
  { let v: u32 = cpu.r32(0x57557bu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x5b2de0u32), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x44e5fb; continue; }
  pc = 0x44e5ea; continue;
 }
 0x44e5ea => {
  { let v: u32 = cpu.r32(0x575577u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2de0u32, v); }
  { let v: u32 = cpu.r32(0x575573u32).wrapping_add(0x20u32); cpu.w32(0x575573u32, v); }
  pc = 0x44e5fb; continue;
 }
 0x44e5fb => {
  { let v: u32 = cpu.r32(0x575573u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2de4u32, v); }
  pc = 0x44e605; continue;
 }
 0x44e605 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e577; continue; }
  pc = 0x44e60f; continue;
 }
 0x44e60f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e610); }
 let mut pc: u32 = 0x44e610;
 loop { match pc {
 0x44e610 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if !cpu.zf { pc = 0x44e61b; continue; }
  pc = 0x44e61a; continue;
 }
 0x44e61a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e61b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5b2de4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2de0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = 0x12b5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44e637); cpu.call(t); if cpu.returned_to != 0x44e637 { return; }
  { let v: u32 = cpu.r32(0x5b2de4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2de0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x128eu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44e65a); cpu.call(t); if cpu.returned_to != 0x44e65a { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e65c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e65c); }
 let mut pc: u32 = 0x44e65c;
 loop { match pc {
 0x44e65c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5b2de4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2de0u32); cpu.ebx = v; }
  { let v: u16 = 0x12e2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44e681); cpu.call(t); if cpu.returned_to != 0x44e681 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5b2de4u32), 0x10u32); cpu.w32(0x5b2de4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e68e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e68e); }
 let mut pc: u32 = 0x44e68e;
 loop { match pc {
 0x44e68e => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x44e698; continue; }
  pc = 0x44e697; continue;
 }
 0x44e697 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e698 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if !cpu.zf { pc = 0x44e6ae; continue; }
  pc = 0x44e6a4; continue;
 }
 0x44e6a4 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if !cpu.zf { pc = 0x44e70e; continue; }
  pc = 0x44e6ae; continue;
 }
 0x44e6ae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44e6d5; continue; }
  pc = 0x44e6c8; continue;
 }
 0x44e6c8 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44e6d5; continue;
 }
 0x44e6d5 => {
  { let v: u32 = 0x5b2de8u32; cpu.esi = v; }
  { let v: u32 = 0x8cu32; cpu.ecx = v; }
  pc = 0x44e6df; continue;
 }
 0x44e6df => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x44e70e; continue; }
  pc = 0x44e6e7; continue;
 }
 0x44e6e7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x44e708; continue; }
  pc = 0x44e6eb; continue;
 }
 0x44e6eb => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x44e708; continue; }
  pc = 0x44e6f0; continue;
 }
 0x44e6f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44e708; continue; }
  pc = 0x44e6f9; continue;
 }
 0x44e6f9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x44e708; continue; }
  pc = 0x44e703; continue;
 }
 0x44e703 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e708 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e6df; continue; }
  pc = 0x44e70e; continue;
 }
 0x44e70e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e710); }
 let mut pc: u32 = 0x44e710;
 loop { match pc {
 0x44e710 => {
  { let v: u32 = 0x5b2de8u32; cpu.edi = v; }
  { let v: u32 = 0x1a4u32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e720); }
 let mut pc: u32 = 0x44e720;
 loop { match pc {
 0x44e720 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1570u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x44e739); f_0044ea30(cpu); if cpu.returned_to != 0x44e739 { return; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x524b74u32)); }
  if cpu.cf { pc = 0x44e761; continue; }
  pc = 0x44e743; continue;
 }
 0x44e743 => {
  pc = 0x44e75a; continue;
 }
 0x44e75a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e761 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x44e76d; continue;
 }
 0x44e76d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44e77a; continue; }
  pc = 0x44e774; continue;
 }
 0x44e774 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x44e784; continue; }
  pc = 0x44e77a; continue;
 }
 0x44e77a => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44e76d; continue; }
  pc = 0x44e782; continue;
 }
 0x44e782 => {
  cpu.poll(); pc = 0x44e75a; continue;
 }
 0x44e784 => {
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
pub fn f_0044e79b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44e79b); } r_0044e79b(cpu, 0x44e79b); }
pub fn f_0044e7a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44e7a2); } r_0044e79b(cpu, 0x44e7a2); }
fn r_0044e79b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44e79b => {
  cpu.push(0x44e7a0); f_0044e9cd(cpu); if cpu.returned_to != 0x44e7a0 { return; }
  if !cpu.cf { pc = 0x44e7cb; continue; }
  pc = 0x44e7a2; continue;
 }
 0x44e7a2 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44e7bc); f_0044e8bd(cpu); if cpu.returned_to != 0x44e7bc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x44e7ca; continue; }
  pc = 0x44e7c7; continue;
 }
 0x44e7c7 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44e7ca => {
  { cpu.popad(); }
  pc = 0x44e7cb; continue;
 }
 0x44e7cb => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e7cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e7cd); }
 let mut pc: u32 = 0x44e7cd;
 loop { match pc {
 0x44e7cd => {
  cpu.push(0x44e7d2); f_0044e720(cpu); if cpu.returned_to != 0x44e7d2 { return; }
  if cpu.cf { pc = 0x44e7d6; continue; }
  pc = 0x44e7d4; continue;
 }
 0x44e7d4 => {
  f_0044e79b(cpu); return;
 }
 0x44e7d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e7d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e7d7); }
 let mut pc: u32 = 0x44e7d7;
 loop { match pc {
 0x44e7d7 => {
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44e7e3; continue;
 }
 0x44e7e3 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e80d; continue; }
  pc = 0x44e7ea; continue;
 }
 0x44e7ea => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44e80d; continue; }
  pc = 0x44e7f3; continue;
 }
 0x44e7f3 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x44e80d; continue; }
  pc = 0x44e7fc; continue;
 }
 0x44e7fc => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44e80d; continue; }
  pc = 0x44e804; continue;
 }
 0x44e804 => {
  cpu.push(0x44e809); f_0044e9cd(cpu); if cpu.returned_to != 0x44e809 { return; }
  if !cpu.cf { pc = 0x44e80d; continue; }
  pc = 0x44e80b; continue;
 }
 0x44e80b => {
  { let v: u8 = cpu.inc8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x44e80d; continue;
 }
 0x44e80d => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e7e3; continue; }
  pc = 0x44e813; continue;
 }
 0x44e813 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e818(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e818); }
 let mut pc: u32 = 0x44e818;
 loop { match pc {
 0x44e818 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x44e824; continue;
 }
 0x44e824 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x44e85b; continue; }
  pc = 0x44e82d; continue;
 }
 0x44e82d => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x44e85b; continue; }
  pc = 0x44e834; continue;
 }
 0x44e834 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x44e85b; continue; }
  pc = 0x44e83b; continue;
 }
 0x44e83b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44e84f); f_0047a2d9(cpu); if cpu.returned_to != 0x44e84f { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x44e85b; continue; }
  pc = 0x44e85a; continue;
 }
 0x44e85a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x44e85b; continue;
 }
 0x44e85b => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e824; continue; }
  pc = 0x44e864; continue;
 }
 0x44e864 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44e87a; continue;
 }
 0x44e87a => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e8b1; continue; }
  pc = 0x44e881; continue;
 }
 0x44e881 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44e8b1; continue; }
  pc = 0x44e88a; continue;
 }
 0x44e88a => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x44e8b1; continue; }
  pc = 0x44e892; continue;
 }
 0x44e892 => {
  { let v: u8 = cpu.r8(0x4d6934u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x44e8b1; continue; }
  pc = 0x44e89c; continue;
 }
 0x44e89c => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44e8b1; continue; }
  pc = 0x44e8a4; continue;
 }
 0x44e8a4 => {
  cpu.push(0x44e8a9); f_0044e9cd(cpu); if cpu.returned_to != 0x44e8a9 { return; }
  if !cpu.cf { pc = 0x44e8b1; continue; }
  pc = 0x44e8ab; continue;
 }
 0x44e8ab => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e8b1; continue;
 }
 0x44e8b1 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e87a; continue; }
  pc = 0x44e8b7; continue;
 }
 0x44e8b7 => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e8bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e8bd); }
 let mut pc: u32 = 0x44e8bd;
 loop { match pc {
 0x44e8bd => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x44e8c9; continue;
 }
 0x44e8c9 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x44e900; continue; }
  pc = 0x44e8d2; continue;
 }
 0x44e8d2 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x44e900; continue; }
  pc = 0x44e8d9; continue;
 }
 0x44e8d9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x44e900; continue; }
  pc = 0x44e8e0; continue;
 }
 0x44e8e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44e8f4); f_0047a2d9(cpu); if cpu.returned_to != 0x44e8f4 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x44e900; continue; }
  pc = 0x44e8ff; continue;
 }
 0x44e8ff => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x44e900; continue;
 }
 0x44e900 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e8c9; continue; }
  pc = 0x44e909; continue;
 }
 0x44e909 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44e91f; continue;
 }
 0x44e91f => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e994; continue; }
  pc = 0x44e926; continue;
 }
 0x44e926 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44e994; continue; }
  pc = 0x44e92f; continue;
 }
 0x44e92f => {
  { cpu.sub32(cpu.r32(0x5b3895u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x44e943; continue; }
  pc = 0x44e93b; continue;
 }
 0x44e93b => {
  { cpu.sub32(cpu.r32(0x5b3895u32), cpu.ebp); }
  if cpu.zf { pc = 0x44e994; continue; }
  pc = 0x44e943; continue;
 }
 0x44e943 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x44e994; continue; }
  pc = 0x44e94b; continue;
 }
 0x44e94b => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44e960; continue; }
  pc = 0x44e953; continue;
 }
 0x44e953 => {
  cpu.push(0x44e958); f_0044e9cd(cpu); if cpu.returned_to != 0x44e958 { return; }
  if !cpu.cf { pc = 0x44e960; continue; }
  pc = 0x44e95a; continue;
 }
 0x44e95a => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e960; continue;
 }
 0x44e960 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e96d; continue; }
  pc = 0x44e967; continue;
 }
 0x44e967 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e96d; continue;
 }
 0x44e96d => {
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e97a; continue; }
  pc = 0x44e974; continue;
 }
 0x44e974 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e97a; continue;
 }
 0x44e97a => {
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e987; continue; }
  pc = 0x44e981; continue;
 }
 0x44e981 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e987; continue;
 }
 0x44e987 => {
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44e994; continue; }
  pc = 0x44e98e; continue;
 }
 0x44e98e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e994; continue;
 }
 0x44e994 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44e91f; continue; }
  pc = 0x44e99a; continue;
 }
 0x44e99a => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x44e9a4; continue;
 }
 0x44e9a4 => {
  { cpu.sub8(cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.sf { pc = 0x44e9be; continue; }
  pc = 0x44e9ad; continue;
 }
 0x44e9ad => {
  { let v: u32 = cpu.r32(0x5b3598u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x44e9be; continue; }
  pc = 0x44e9b8; continue;
 }
 0x44e9b8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  pc = 0x44e9be; continue;
 }
 0x44e9be => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x44e9a4; continue; }
  pc = 0x44e9c1; continue;
 }
 0x44e9c1 => {
  cpu.push(0x44e9c6); f_00448060(cpu); if cpu.returned_to != 0x44e9c6 { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044e9cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44e9cd); }
 let mut pc: u32 = 0x44e9cd;
 loop { match pc {
 0x44e9cd => {
  { cpu.pushad(); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44ea2a; continue; }
  pc = 0x44e9d2; continue;
 }
 0x44e9d2 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44ea2d; continue; }
  pc = 0x44ea1e; continue;
 }
 0x44ea1e => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x44ea2d; continue; }
  pc = 0x44ea22; continue;
 }
 0x44ea22 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x44ea2d; continue; }
  pc = 0x44ea26; continue;
 }
 0x44ea26 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x44ea2d; continue; }
  pc = 0x44ea2a; continue;
 }
 0x44ea2a => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ea2d => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ea30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ea30); }
 let mut pc: u32 = 0x44ea30;
 loop { match pc {
 0x44ea30 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44ea43; continue;
 }
 0x44ea43 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ea7c; continue; }
  pc = 0x44ea4a; continue;
 }
 0x44ea4a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44ea7c; continue; }
  pc = 0x44ea53; continue;
 }
 0x44ea53 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x44ea7c; continue; }
  pc = 0x44ea5c; continue;
 }
 0x44ea5c => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ea64; continue; }
  pc = 0x44ea63; continue;
 }
 0x44ea63 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x44ea64; continue;
 }
 0x44ea64 => {
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ea6c; continue; }
  pc = 0x44ea6b; continue;
 }
 0x44ea6b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x44ea6c; continue;
 }
 0x44ea6c => {
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ea74; continue; }
  pc = 0x44ea73; continue;
 }
 0x44ea73 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x44ea74; continue;
 }
 0x44ea74 => {
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ea7c; continue; }
  pc = 0x44ea7b; continue;
 }
 0x44ea7b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x44ea7c; continue;
 }
 0x44ea7c => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44ea43; continue; }
  pc = 0x44ea82; continue;
 }
 0x44ea82 => {
  { cpu.sub8(cpu.r8(0x4e1fccu32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.sf { pc = 0x44ea8c; continue; }
  pc = 0x44ea8b; continue;
 }
 0x44ea8b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x44ea8c; continue;
 }
 0x44ea8c => {
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
pub fn f_0044ea92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ea92); }
 let mut pc: u32 = 0x44ea92;
 loop { match pc {
 0x44ea92 => {
  { cpu.pushad(); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44eaa0); f_0044ea30(cpu); if cpu.returned_to != 0x44eaa0 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44eaac); f_0044ea30(cpu); if cpu.returned_to != 0x44eaac { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), (cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44eb45; continue; }
  pc = 0x44eab9; continue;
 }
 0x44eab9 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x4e1570u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44ead2; continue; }
  pc = 0x44eac8; continue;
 }
 0x44eac8 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x524b74u32)); }
  if cpu.cf { pc = 0x44eadd; continue; }
  pc = 0x44ead0; continue;
 }
 0x44ead0 => {
  pc = 0x44ead9; continue;
 }
 0x44ead2 => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x44eadd; continue; }
  pc = 0x44ead7; continue;
 }
 0x44ead7 => {
  pc = 0x44ead9; continue;
 }
 0x44ead9 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x44eaf6; continue;
 }
 0x44eadd => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x524b74u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44eaf0; continue; }
  pc = 0x44eaee; continue;
 }
 0x44eaee => {
  { let v: u8 = 0x63u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44eaf0; continue;
 }
 0x44eaf0 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), (cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x44eb45; continue; }
  pc = 0x44eaf4; continue;
 }
 0x44eaf4 => {
  if cpu.zf { pc = 0x44eb45; continue; }
  pc = 0x44eaf6; continue;
 }
 0x44eaf6 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44eb00; continue;
 }
 0x44eb00 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44eb3f; continue; }
  pc = 0x44eb07; continue;
 }
 0x44eb07 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44eb3f; continue; }
  pc = 0x44eb10; continue;
 }
 0x44eb10 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x44eb3f; continue; }
  pc = 0x44eb18; continue;
 }
 0x44eb18 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x44eb3f; continue; }
  pc = 0x44eb1d; continue;
 }
 0x44eb1d => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44eb3f; continue; }
  pc = 0x44eb24; continue;
 }
 0x44eb24 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44eb45; continue; }
  pc = 0x44eb3f; continue;
 }
 0x44eb3f => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44eb00; continue; }
  pc = 0x44eb45; continue;
 }
 0x44eb45 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44eb4f; continue;
 }
 0x44eb4f => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb56; continue;
 }
 0x44eb56 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb5f; continue;
 }
 0x44eb5f => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb67; continue;
 }
 0x44eb67 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb6c; continue;
 }
 0x44eb6c => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb73; continue;
 }
 0x44eb73 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb7a; continue;
 }
 0x44eb7a => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44eba8; continue; }
  pc = 0x44eb81; continue;
 }
 0x44eb81 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44eb8f; continue; }
  pc = 0x44eb88; continue;
 }
 0x44eb88 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44ebbf; continue; }
  pc = 0x44eb8f; continue;
 }
 0x44eb8f => {
  { let v: u16 = cpu.r16(0x4d67f0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44eb9e); f_004456c8(cpu); if cpu.returned_to != 0x44eb9e { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ebbf; continue;
 }
 0x44eba8 => {
  { let v: u16 = cpu.r16(0x4d67f2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x44ebb7); f_00445651(cpu); if cpu.returned_to != 0x44ebb7 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x44ebbf; continue;
 }
 0x44ebbf => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44eb4f; continue; }
  pc = 0x44ebc9; continue;
 }
 0x44ebc9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ebcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ebcb); }
 let mut pc: u32 = 0x44ebcb;
 loop { match pc {
 0x44ebcb => {
  { cpu.sub8(cpu.r8(0x58c492u32), 0x0u8); }
  if !cpu.zf { pc = 0x44ebd5; continue; }
  pc = 0x44ebd4; continue;
 }
 0x44ebd4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ebd5 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x44ec88; continue; }
  pc = 0x44ebe2; continue;
 }
 0x44ebe2 => {
  { let v: u32 = cpu.r32(0x58c466u32); cpu.esi = v; }
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x44ec88; continue; }
  pc = 0x44ebf2; continue;
 }
 0x44ebf2 => {
  { let v: u32 = cpu.esi; cpu.w32(0x58c466u32, v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x44ebe2; continue; }
  pc = 0x44ec09; continue;
 }
 0x44ec09 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x44ebe2; continue; }
  pc = 0x44ec12; continue;
 }
 0x44ec12 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x44ebe2; continue; }
  pc = 0x44ec19; continue;
 }
 0x44ec19 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  cpu.push(0x44ec32); f_00471ec2(cpu); if cpu.returned_to != 0x44ec32 { return; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x40u32); }
  if cpu.zf { pc = 0x44ec4e; continue; }
  pc = 0x44ec3e; continue;
 }
 0x44ec3e => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c465u32, v); }
  { cpu.sub8(cpu.r8(0x5b38d6u32), 0x0u8); }
  if !cpu.zf { pc = 0x44ec6c; continue; }
  pc = 0x44ec4e; continue;
 }
 0x44ec4e => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c465u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44ec5b); f_00419d49(cpu); if cpu.returned_to != 0x44ec5b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4d675cu32), 0x20u16); cpu.w16(0x4d675cu32, v); }
  pc = 0x44ec6c; continue;
 }
 0x44ec6c => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dc4u32, v); }
  cpu.push(0x44ec7b); f_0044cbe1(cpu); if cpu.returned_to != 0x44ec7b { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x44ec87); f_0041879e(cpu); if cpu.returned_to != 0x44ec87 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44ec88 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c492u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ec90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ec90); }
 let mut pc: u32 = 0x44ec90;
 loop { match pc {
 0x44ec90 => {
  { cpu.sub8(cpu.r8(0x58c492u32), 0x0u8); }
  if cpu.zf { pc = 0x44eca4; continue; }
  pc = 0x44ec99; continue;
 }
 0x44ec99 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5b2dc4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44eca4 => {
  { let v: u8 = cpu.r8(0x58c838u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5b38d6u32, v); }
  { let v: u32 = 0x58c46au32; cpu.w32(0x58c466u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x58c46au32, v); }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  cpu.push(0x44ecce); f_0044ecf9(cpu); if cpu.returned_to != 0x44ecce { return; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x44ecd8); f_0044ecf9(cpu); if cpu.returned_to != 0x44ecd8 { return; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  cpu.push(0x44ece2); f_0044ecf9(cpu); if cpu.returned_to != 0x44ece2 { return; }
  { let v: u32 = 0x58c46au32; cpu.w32(0x58c466u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c492u32, v); }
  cpu.push(0x44ecf8); f_0044ebcb(cpu); if cpu.returned_to != 0x44ecf8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ecf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ecf9); }
 let mut pc: u32 = 0x44ecf9;
 loop { match pc {
 0x44ecf9 => {
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58c466u32); cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44ed09; continue;
 }
 0x44ed09 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x44ed30; continue; }
  pc = 0x44ed10; continue;
 }
 0x44ed10 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x44ed30; continue; }
  pc = 0x44ed18; continue;
 }
 0x44ed18 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x44ed30; continue; }
  pc = 0x44ed23; continue;
 }
 0x44ed23 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x44ed30; continue; }
  pc = 0x44ed2b; continue;
 }
 0x44ed2b => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x44ed30; continue;
 }
 0x44ed30 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44ed09; continue; }
  pc = 0x44ed36; continue;
 }
 0x44ed36 => {
  { let v: u32 = cpu.edi; cpu.w32(0x58c466u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ed43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ed43); }
 let mut pc: u32 = 0x44ed43;
 loop { match pc {
 0x44ed43 => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44ed58; continue; }
  pc = 0x44ed52; continue;
 }
 0x44ed52 => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x44ed58; continue;
 }
 0x44ed58 => {
  { let v: u16 = cpu.r16(0x4d680eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x44ed87; continue; }
  pc = 0x44ed63; continue;
 }
 0x44ed63 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x44ed84; continue; }
  pc = 0x44ed6b; continue;
 }
 0x44ed6b => {
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x44ed7c; continue; }
  pc = 0x44ed74; continue;
 }
 0x44ed74 => {
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x44ed87; continue; }
  pc = 0x44ed7c; continue;
 }
 0x44ed7c => {
  { let v: u16 = cpu.r16(0x4d680eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44ed87; continue;
 }
 0x44ed84 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44ed87; continue;
 }
 0x44ed87 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d67f8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ed94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ed94); }
 let mut pc: u32 = 0x44ed94;
 loop { match pc {
 0x44ed94 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x44ee34; continue; }
  pc = 0x44eda2; continue;
 }
 0x44eda2 => {
  cpu.push(0x44eda7); f_00447617(cpu); if cpu.returned_to != 0x44eda7 { return; }
  cpu.push(0x44edac); f_004184ca(cpu); if cpu.returned_to != 0x44edac { return; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x44edee; continue; }
  pc = 0x44edc0; continue;
 }
 0x44edc0 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x4d6898u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44ede2); f_0043dd87(cpu); if cpu.returned_to != 0x44ede2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0xfff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfff8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44ee34; continue;
 }
 0x44edee => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x44ee34; continue; }
  pc = 0x44edfd; continue;
 }
 0x44edfd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x44ee0c; continue; }
  pc = 0x44ee07; continue;
 }
 0x44ee07 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44ee11; continue; }
  pc = 0x44ee0c; continue;
 }
 0x44ee0c => {
  { let v: u32 = 0x5b355cu32; cpu.ebp = v; }
  pc = 0x44ee11; continue;
 }
 0x44ee11 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebp as u16), 3u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x4d6898u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44ee2c); f_0043dd87(cpu); if cpu.returned_to != 0x44ee2c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x44ee34); f_0044eeac(cpu); if cpu.returned_to != 0x44ee34 { return; }
  pc = 0x44ee34; continue;
 }
 0x44ee34 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ee35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ee35); }
 let mut pc: u32 = 0x44ee35;
 loop { match pc {
 0x44ee35 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x20u32); }
  if cpu.zf { pc = 0x44eeab; continue; }
  pc = 0x44ee41; continue;
 }
 0x44ee41 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x44ee92); f_00447617(cpu); if cpu.returned_to != 0x44ee92 { return; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x18u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x44eeaa); f_0043dd87(cpu); if cpu.returned_to != 0x44eeaa { return; }
  { cpu.popad(); }
  pc = 0x44eeab; continue;
 }
 0x44eeab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044eeac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44eeac); }
 let mut pc: u32 = 0x44eeac;
 loop { match pc {
 0x44eeac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044eecb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44eecb); }
 let mut pc: u32 = 0x44eecb;
 loop { match pc {
 0x44eecb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5b3940u32, v); }
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x44f084; continue; }
  pc = 0x44eee3; continue;
 }
 0x44eee3 => {
  { cpu.sub8(cpu.r8(0x58c86eu32), 0x0u8); }
  if cpu.zf { pc = 0x44f084; continue; }
  pc = 0x44eef0; continue;
 }
 0x44eef0 => {
  cpu.push(0x44eef5); f_004184ca(cpu); if cpu.returned_to != 0x44eef5 { return; }
  { let v: u32 = 0x54aa58u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x575539u32, v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x44ef27; continue; }
  pc = 0x44ef1c; continue;
 }
 0x44ef1c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x575539u32, v); }
  pc = 0x44ef27; continue;
 }
 0x44ef27 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x44f084; continue; }
  pc = 0x44ef2f; continue;
 }
 0x44ef2f => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x44f084; continue; }
  pc = 0x44ef38; continue;
 }
 0x44ef38 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x44f084; continue; }
  pc = 0x44ef45; continue;
 }
 0x44ef45 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x44ef61; continue;
 }
 0x44ef61 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x44ef7e; continue; }
  pc = 0x44ef65; continue;
 }
 0x44ef65 => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x44ef7e; continue; }
  pc = 0x44ef6a; continue;
 }
 0x44ef6a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.poll(); pc = 0x44ef61; continue;
 }
 0x44ef7e => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x44efd3; continue; }
  pc = 0x44ef87; continue;
 }
 0x44ef87 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44ef95; continue; }
  pc = 0x44ef8e; continue;
 }
 0x44ef8e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x44ef9a; continue; }
  pc = 0x44ef95; continue;
 }
 0x44ef95 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  pc = 0x44f007; continue;
 }
 0x44ef9a => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44efa0); f_0044f16c(cpu); if cpu.returned_to != 0x44efa0 { return; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44f007; continue;
 }
 0x44efb4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44f007; continue;
 }
 0x44efd3 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { cpu.poll(); pc = 0x44efb4; continue; }
  pc = 0x44efdc; continue;
 }
 0x44efdc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x44efe3); f_00451959(cpu); if cpu.returned_to != 0x44efe3 { return; }
  if !cpu.cf { pc = 0x44efec; continue; }
  pc = 0x44efe5; continue;
 }
 0x44efe5 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44f084; continue;
 }
 0x44efec => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x44f007; continue;
 }
 0x44f007 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(0x6u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x5au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x44f01c); f_0043e337(cpu); if cpu.returned_to != 0x44f01c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(0x6u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x575539u32), 0x0u8); }
  if cpu.zf { pc = 0x44f07a; continue; }
  pc = 0x44f02e; continue;
 }
 0x44f02e => {
  { cpu.pushad(); }
  cpu.push(0x44f034); f_0044f085(cpu); if cpu.returned_to != 0x44f034 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44f07a; continue; }
  pc = 0x44f042; continue;
 }
 0x44f042 => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if !cpu.zf { pc = 0x44f056; continue; }
  pc = 0x44f051; continue;
 }
 0x44f051 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if cpu.zf { pc = 0x44f07a; continue; }
  pc = 0x44f056; continue;
 }
 0x44f056 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x6u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x6u32); cpu.edx = v; }
  { let v: u32 = 0x5au32; cpu.ebp = v; }
  cpu.push(0x44f06c); f_0043e38b(cpu); if cpu.returned_to != 0x44f06c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  pc = 0x44f07f; continue;
 }
 0x44f07a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  pc = 0x44f07f; continue;
 }
 0x44f07f => {
  cpu.push(0x44f084); f_0044f1d9(cpu); if cpu.returned_to != 0x44f084 { return; }
  pc = 0x44f084; continue;
 }
 0x44f084 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f085(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f085); }
 let mut pc: u32 = 0x44f085;
 loop { match pc {
 0x44f085 => {
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x44f16a; continue; }
  pc = 0x44f091; continue;
 }
 0x44f091 => {
  if !cpu.sf { pc = 0x44f095; continue; }
  pc = 0x44f093; continue;
 }
 0x44f093 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x44f095; continue;
 }
 0x44f095 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x44f0ac; continue; }
  pc = 0x44f099; continue;
 }
 0x44f099 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f168; continue;
 }
 0x44f0ac => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x44f0fc; continue; }
  pc = 0x44f0b3; continue;
 }
 0x44f0b3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x44f0da; continue; }
  pc = 0x44f0bf; continue;
 }
 0x44f0bf => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x44f0da; continue; }
  pc = 0x44f0d5; continue;
 }
 0x44f0d5 => {
  pc = 0x44f16a; continue;
 }
 0x44f0da => {
  { cpu.sub16((cpu.eax as u16), 0xd80u16); }
  if cpu.zf { pc = 0x44f16a; continue; }
  pc = 0x44f0e4; continue;
 }
 0x44f0e4 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f168; continue;
 }
 0x44f0fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44f10a; continue; }
  pc = 0x44f105; continue;
 }
 0x44f105 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44f10f; continue; }
  pc = 0x44f10a; continue;
 }
 0x44f10a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44f10f; continue;
 }
 0x44f10f => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x44f125; continue; }
  pc = 0x44f11d; continue;
 }
 0x44f11d => {
  { cpu.sub16((cpu.eax as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x44f150; continue; }
  pc = 0x44f123; continue;
 }
 0x44f123 => {
  pc = 0x44f168; continue;
 }
 0x44f125 => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x44f16a; continue; }
  pc = 0x44f137; continue;
 }
 0x44f137 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  pc = 0x44f168; continue;
 }
 0x44f150 => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f168; continue;
 }
 0x44f168 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f16a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f16c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f16c); }
 let mut pc: u32 = 0x44f16c;
 loop { match pc {
 0x44f16c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x44f1a4; continue; }
  pc = 0x44f17e; continue;
 }
 0x44f17e => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x44f18e; continue; }
  pc = 0x44f185; continue;
 }
 0x44f185 => {
  { cpu.sub16((cpu.ecx as u16), 0xd80u16); }
  if cpu.zf { pc = 0x44f1cd; continue; }
  pc = 0x44f18c; continue;
 }
 0x44f18c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f18e => {
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f1a4 => {
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  cpu.push(0x44f1b5); f_0045bc6e(cpu); if cpu.returned_to != 0x44f1b5 { return; }
  if cpu.cf { pc = 0x44f1c1; continue; }
  pc = 0x44f1b7; continue;
 }
 0x44f1b7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f1c1 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f1cd => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f1d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f1d9); }
 let mut pc: u32 = 0x44f1d9;
 loop { match pc {
 0x44f1d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44f1e1); f_00447617(cpu); if cpu.returned_to != 0x44f1e1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x44f1f4; continue; }
  pc = 0x44f1eb; continue;
 }
 0x44f1eb => {
  cpu.push(0x44f1f0); f_0047111f(cpu); if cpu.returned_to != 0x44f1f0 { return; }
  if !cpu.cf { pc = 0x44f1f4; continue; }
  pc = 0x44f1f2; continue;
 }
 0x44f1f2 => {
  pc = 0x44f255; continue;
 }
 0x44f1f4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xa3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x44f21f; continue; }
  pc = 0x44f203; continue;
 }
 0x44f203 => {
  { cpu.sub32(cpu.eax, 0xaceu32); }
  if cpu.zf { pc = 0x44f221; continue; }
  pc = 0x44f20a; continue;
 }
 0x44f20a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  pc = 0x44f2fb; continue;
 }
 0x44f21f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f221 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59da94u32, v); }
  cpu.push(0x44f231); f_004716d0(cpu); if cpu.returned_to != 0x44f231 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x59da94u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44f265; continue; }
  pc = 0x44f23d; continue;
 }
 0x44f23d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x44f20a; continue;
 }
 0x44f245 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaeau32; cpu.eax = v; }
  cpu.poll(); pc = 0x44f20a; continue;
 }
 0x44f24d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaceu32; cpu.eax = v; }
  cpu.poll(); pc = 0x44f20a; continue;
 }
 0x44f255 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xad6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x44f20a; continue;
 }
 0x44f25d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb1cu32; cpu.eax = v; }
  cpu.poll(); pc = 0x44f20a; continue;
 }
 0x44f265 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x44f2ab; continue; }
  pc = 0x44f26c; continue;
 }
 0x44f26c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x44f2ab; continue; }
  pc = 0x44f273; continue;
 }
 0x44f273 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x1cu8); }
  if cpu.zf { cpu.poll(); pc = 0x44f24d; continue; }
  pc = 0x44f297; continue;
 }
 0x44f297 => {
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x47u8); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44f245; continue; }
  pc = 0x44f2a0; continue;
 }
 0x44f2a0 => {
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x4du8); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44f245; continue; }
  pc = 0x44f2a9; continue;
 }
 0x44f2a9 => {
  cpu.poll(); pc = 0x44f23d; continue;
 }
 0x44f2ab => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6934u32, v); }
  { cpu.sub8(cpu.r8(0x4ea1b2u32.wrapping_add(cpu.eax)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4d6934u32), 0x1cu8); }
  if cpu.zf { cpu.poll(); pc = 0x44f24d; continue; }
  pc = 0x44f2da; continue;
 }
 0x44f2da => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4d6934u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4ea1b2u32.wrapping_add(cpu.eax)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x44f23d; continue; }
  pc = 0x44f2f0; continue;
 }
 0x44f2f0 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x44f24d; continue; }
  pc = 0x44f2f6; continue;
 }
 0x44f2f6 => {
  cpu.poll(); pc = 0x44f25d; continue;
 }
 0x44f2fb => {
  { cpu.sub8(cpu.r8(0x5b3966u32), 0x0u8); }
  if cpu.zf { pc = 0x44f315; continue; }
  pc = 0x44f304; continue;
 }
 0x44f304 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x55b6b4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x55b6b6u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f315; continue;
 }
 0x44f315 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b3938u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b393cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5b3940u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x57556au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f339(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f339); }
 let mut pc: u32 = 0x44f339;
 loop { match pc {
 0x44f339 => {
  { let v: u32 = cpu.r32(0x5b3938u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b393cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5b3940u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x44f355; continue; }
  pc = 0x44f34f; continue;
 }
 0x44f34f => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f355); cpu.call(t); if cpu.returned_to != 0x44f355 { return; }
  pc = 0x44f355; continue;
 }
 0x44f355 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f356(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f356); }
 let mut pc: u32 = 0x44f356;
 loop { match pc {
 0x44f356 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f357(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f357); }
 let mut pc: u32 = 0x44f357;
 loop { match pc {
 0x44f357 => {
  { cpu.sub8(cpu.r8(0x4e15c1u32), 0x0u8); }
  if !cpu.zf { pc = 0x44f3a4; continue; }
  pc = 0x44f360; continue;
 }
 0x44f360 => {
  { let v = cpu.r16(0x4d676cu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d676eu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x44f37c; continue; }
  pc = 0x44f377; continue;
 }
 0x44f377 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x44f381; continue; }
  pc = 0x44f37c; continue;
 }
 0x44f37c => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x44f381; continue;
 }
 0x44f381 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x4e15c3u32); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x4e15c3u32, v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x44f390); f_00441fe6(cpu); if cpu.returned_to != 0x44f390 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15c3u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d676cu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d676eu32, v); }
  pc = 0x44f3a4; continue;
 }
 0x44f3a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f3a5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44f3a5); } r_0044f3a5(cpu, 0x44f3a5); }
pub fn f_0044f3e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44f3e8); } r_0044f3a5(cpu, 0x44f3e8); }
fn r_0044f3a5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44f3a5 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x44f3ba; continue; }
  pc = 0x44f3ae; continue;
 }
 0x44f3ae => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f3cc; continue;
 }
 0x44f3ba => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44f3c0); f_00451959(cpu); if cpu.returned_to != 0x44f3c0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x44f3c4; continue; }
  pc = 0x44f3c3; continue;
 }
 0x44f3c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f3c4 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f3cc; continue;
 }
 0x44f3cc => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f3e7); f_00441f16(cpu); if cpu.returned_to != 0x44f3e7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f3e8 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44f3ee); f_00441e40(cpu); if cpu.returned_to != 0x44f3ee { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x44f3cc; continue; }
  pc = 0x44f3f7; continue;
 }
 0x44f3f7 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f404); f_0043dd2c(cpu); if cpu.returned_to != 0x44f404 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f405(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f405); }
 let mut pc: u32 = 0x44f405;
 loop { match pc {
 0x44f405 => {
  { cpu.sub8(cpu.r8(0x5b3962u32), 0x0u8); }
  if cpu.zf { pc = 0x44f566; continue; }
  pc = 0x44f412; continue;
 }
 0x44f412 => {
  { cpu.sub8(cpu.r8(0x4e156au32), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x44f566; continue; }
  pc = 0x44f41f; continue;
 }
 0x44f41f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44f446; continue; }
  pc = 0x44f439; continue;
 }
 0x44f439 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f446; continue;
 }
 0x44f446 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b394du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3951u32, v); }
  { let v: u32 = cpu.r16(0x5b394du32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5b3951u32) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44f496); f_004714a5(cpu); if cpu.returned_to != 0x44f496 { return; }
  if cpu.cf { pc = 0x44f566; continue; }
  pc = 0x44f49c; continue;
 }
 0x44f49c => {
  { let v: u16 = cpu.r16(0x5b394du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3951u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b3947u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3949u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b394fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x5b394bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b394cu32, v); }
  { let v: u16 = cpu.r16(0x5b3947u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3949u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693au32, v); }
  { let v: u8 = cpu.r8(0x5b394bu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x44f4fa; continue;
 }
 0x44f4fa => {
  { let v: u8 = cpu.r8(0x5b394bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x44f506; continue;
 }
 0x44f506 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x5b395eu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x5b3960u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x44f528); f_0044f988(cpu); if cpu.returned_to != 0x44f528 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x5b3960u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x5b395eu32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44f556; continue; }
  pc = 0x44f539; continue;
 }
 0x44f539 => {
  { cpu.sub8(cpu.r8(0x5b3964u32), 0x0u8); }
  if !cpu.zf { pc = 0x44f556; continue; }
  pc = 0x44f542; continue;
 }
 0x44f542 => {
  { cpu.pushad(); }
  { let v: u32 = 0x4f2u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f54e); cpu.call(t); if cpu.returned_to != 0x44f54e { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b394cu32, v); }
  pc = 0x44f556; continue;
 }
 0x44f556 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x44f506; continue; }
  pc = 0x44f55e; continue;
 }
 0x44f55e => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x44f4fa; continue; }
  pc = 0x44f566; continue;
 }
 0x44f566 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f567(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f567); }
 let mut pc: u32 = 0x44f567;
 loop { match pc {
 0x44f567 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.sf { pc = 0x44f946; continue; }
  pc = 0x44f57b; continue;
 }
 0x44f57b => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x10u32); }
  if cpu.zf { pc = 0x44f58e; continue; }
  pc = 0x44f587; continue;
 }
 0x44f587 => {
  { let v: u8 = 0x20u8; cpu.w8(0x5b3962u32, v); }
  pc = 0x44f58e; continue;
 }
 0x44f58e => {
  cpu.push(0x44f593); f_00447617(cpu); if cpu.returned_to != 0x44f593 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x44f5be; continue; }
  pc = 0x44f59c; continue;
 }
 0x44f59c => {
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x44f5be; continue; }
  pc = 0x44f5a7; continue;
 }
 0x44f5a7 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x44f5cd; continue; }
  pc = 0x44f5ab; continue;
 }
 0x44f5ab => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x44f5dc; continue; }
  pc = 0x44f5af; continue;
 }
 0x44f5af => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f5eb; continue;
 }
 0x44f5be => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f5eb; continue;
 }
 0x44f5cd => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f5eb; continue;
 }
 0x44f5dc => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f5eb; continue;
 }
 0x44f5eb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b3967u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3969u32, v); }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b35acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x44f670; continue; }
  pc = 0x44f663; continue;
 }
 0x44f663 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f670; continue;
 }
 0x44f670 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b394du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3951u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5b3963u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x44f6b9; continue; }
  pc = 0x44f6b3; continue;
 }
 0x44f6b3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x44f6b9; continue;
 }
 0x44f6b9 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5b395du32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5b395bu32, v); }
  { let v: u32 = cpu.r16(0x5b394du32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5b3951u32) as u32; cpu.edi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5b3967u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3969u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44f6f7); f_004714a5(cpu); if cpu.returned_to != 0x44f6f7 { return; }
  if !cpu.cf { pc = 0x44f8ae; continue; }
  pc = 0x44f6fd; continue;
 }
 0x44f6fd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5b3967u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x5b3969u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f714); f_0044f947(cpu); if cpu.returned_to != 0x44f714 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x5b3908u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b390au32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5b395du32), 0x2u8); }
  if cpu.zf { pc = 0x44f73c; continue; }
  pc = 0x44f734; continue;
 }
 0x44f734 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f73c; continue;
 }
 0x44f73c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5b3967u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5b3969u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b395bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x5u8); }
  if !cpu.zf { pc = 0x44f76a; continue; }
  pc = 0x44f75a; continue;
 }
 0x44f75a => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d69f4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d69f6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f79a; continue;
 }
 0x44f76a => {
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x6u8); }
  if !cpu.zf { pc = 0x44f783; continue; }
  pc = 0x44f773; continue;
 }
 0x44f773 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d6a14u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d6a16u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f79a; continue;
 }
 0x44f783 => {
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x4u8); }
  if !cpu.zf { pc = 0x44f79a; continue; }
  pc = 0x44f78c; continue;
 }
 0x44f78c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d69d4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5d69d6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f79a; continue;
 }
 0x44f79a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f7ae); cpu.call(t); if cpu.returned_to != 0x44f7ae { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  cpu.push(0x44f7c2); f_0044ee35(cpu); if cpu.returned_to != 0x44f7c2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  cpu.push(0x44f7d5); f_0047a2d9(cpu); if cpu.returned_to != 0x44f7d5 { return; }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5b3598u32.wrapping_add(cpu.eax.wrapping_mul(4)))); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x44f814; continue; }
  pc = 0x44f7e8; continue;
 }
 0x44f7e8 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f7f9); f_0044fb8f(cpu); if cpu.returned_to != 0x44f7f9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44f813; continue; }
  pc = 0x44f7fe; continue;
 }
 0x44f7fe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f80e); f_00471636(cpu); if cpu.returned_to != 0x44f80e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x44f845; continue; }
  pc = 0x44f813; continue;
 }
 0x44f813 => {
  { cpu.popad(); }
  pc = 0x44f814; continue;
 }
 0x44f814 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f837); cpu.call(t); if cpu.returned_to != 0x44f837 { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x5b395eu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f845 => {
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x5u8); }
  if !cpu.zf { pc = 0x44f86f; continue; }
  pc = 0x44f85f; continue;
 }
 0x44f85f => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d69f4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d69f6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f89f; continue;
 }
 0x44f86f => {
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x6u8); }
  if !cpu.zf { pc = 0x44f888; continue; }
  pc = 0x44f878; continue;
 }
 0x44f878 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d6a14u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d6a16u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f89f; continue;
 }
 0x44f888 => {
  { cpu.sub8(cpu.r8(0x5b3963u32), 0x4u8); }
  if !cpu.zf { pc = 0x44f89f; continue; }
  pc = 0x44f891; continue;
 }
 0x44f891 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d69d4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d69d6u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44f89f; continue;
 }
 0x44f89f => {
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44f8ae => {
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5b395bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f8be); cpu.call(t); if cpu.returned_to != 0x44f8be { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  cpu.push(0x44f8d1); f_0044ee35(cpu); if cpu.returned_to != 0x44f8d1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x44f8de; continue; }
  pc = 0x44f8dd; continue;
 }
 0x44f8dd => {
  {  }
  pc = 0x44f8de; continue;
 }
 0x44f8de => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  cpu.push(0x44f8ea); f_0044f988(cpu); if cpu.returned_to != 0x44f8ea { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44f92c; continue; }
  pc = 0x44f8ee; continue;
 }
 0x44f8ee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44f906); f_0044fc77(cpu); if cpu.returned_to != 0x44f906 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44f92c; continue; }
  pc = 0x44f90a; continue;
 }
 0x44f90a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4e852du32; cpu.w32(0x4d6bf8u32, v); }
  cpu.push(0x44f928); f_0041a775(cpu); if cpu.returned_to != 0x44f928 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x44f946; continue; }
  pc = 0x44f92c; continue;
 }
 0x44f92c => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x44f93d); cpu.call(t); if cpu.returned_to != 0x44f93d { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x5b395eu32, v); }
  pc = 0x44f946; continue;
 }
 0x44f946 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f947); }
 let mut pc: u32 = 0x44f947;
 loop { match pc {
 0x44f947 => {
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if (cpu.sf == cpu.of) { pc = 0x44f96b; continue; }
  pc = 0x44f94d; continue;
 }
 0x44f94d => {
  { cpu.sub16((cpu.ebx as u16), 0x1fu16); }
  if (cpu.sf != cpu.of) { pc = 0x44f95f; continue; }
  pc = 0x44f953; continue;
 }
 0x44f953 => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44f965; continue; }
  pc = 0x44f959; continue;
 }
 0x44f959 => {
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f95f => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f965 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f96b => {
  { cpu.sub16((cpu.ebx as u16), 0x1fu16); }
  if (cpu.sf != cpu.of) { pc = 0x44f97d; continue; }
  pc = 0x44f971; continue;
 }
 0x44f971 => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x44f983; continue; }
  pc = 0x44f977; continue;
 }
 0x44f977 => {
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f97d => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f983 => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x44f987; continue;
 }
 0x44f987 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044f988(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44f988); }
 let mut pc: u32 = 0x44f988;
 loop { match pc {
 0x44f988 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3964u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b396bu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5b396fu32, v); }
  cpu.push(0x44f9ad); f_0047a2d9(cpu); if cpu.returned_to != 0x44f9ad { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5d3986u32), 0x0u8); }
  if !cpu.zf { pc = 0x44f9c1; continue; }
  pc = 0x44f9ba; continue;
 }
 0x44f9ba => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  pc = 0x44f9c1; continue;
 }
 0x44f9c1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x5b3598u32.wrapping_add(cpu.eax.wrapping_mul(4)))); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x44fb77; continue; }
  pc = 0x44f9cf; continue;
 }
 0x44f9cf => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5b395du32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5b395bu32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5b396bu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5b396fu32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5b3967u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3969u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x44fa2b); f_004714a5(cpu); if cpu.returned_to != 0x44fa2b { return; }
  if !cpu.cf { pc = 0x44fafb; continue; }
  pc = 0x44fa31; continue;
 }
 0x44fa31 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5b3964u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5b3967u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x5b3969u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x44fa4f); f_0044f947(cpu); if cpu.returned_to != 0x44fa4f { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x5b3908u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b390au32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5b395du32), 0x2u8); }
  if cpu.zf { pc = 0x44fa77; continue; }
  pc = 0x44fa6f; continue;
 }
 0x44fa6f => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x44fa77; continue;
 }
 0x44fa77 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5b3967u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x5b3969u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b395bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x44faa9; continue; }
  pc = 0x44faa8; continue;
 }
 0x44faa8 => {
  {  }
  pc = 0x44faa9; continue;
 }
 0x44faa9 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  { cpu.pushad(); }
  cpu.push(0x44fab6); f_0044fb8f(cpu); if cpu.returned_to != 0x44fab6 { return; }
  if cpu.cf { pc = 0x44fada; continue; }
  pc = 0x44fab8; continue;
 }
 0x44fab8 => {
  { cpu.sub8(cpu.r8(0x5d3986u32), 0x0u8); }
  if cpu.zf { pc = 0x44fada; continue; }
  pc = 0x44fac1; continue;
 }
 0x44fac1 => {
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
  cpu.push(0x44fad4); f_00417e7e(cpu); if cpu.returned_to != 0x44fad4 { return; }
  { cpu.popad(); }
  pc = 0x44fb79; continue;
 }
 0x44fada => {
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44fafb => {
  { cpu.popad(); }
  { let v: u32 = cpu.r16(0x5b396bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5b396fu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5b396bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5b396fu32) as u32; cpu.ebx = v; }
  cpu.push(0x44fb1f); f_0044fc77(cpu); if cpu.returned_to != 0x44fb1f { return; }
  if cpu.cf { pc = 0x44fb77; continue; }
  pc = 0x44fb21; continue;
 }
 0x44fb21 => {
  { let v: u32 = cpu.r16(0x5b396bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x5b396fu32) as u32; cpu.ebx = v; }
  { cpu.pushad(); }
  cpu.push(0x44fb35); f_0044fb8f(cpu); if cpu.returned_to != 0x44fb35 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44fb77; continue; }
  pc = 0x44fb38; continue;
 }
 0x44fb38 => {
  { let v: u32 = cpu.r32(0x5b396bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b396fu32); cpu.ebx = v; }
  { cpu.pushad(); }
  cpu.push(0x44fb49); f_00472452(cpu); if cpu.returned_to != 0x44fb49 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x44fb77; continue; }
  pc = 0x44fb4c; continue;
 }
 0x44fb4c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44fb53); f_00471636(cpu); if cpu.returned_to != 0x44fb53 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x44fb77; continue; }
  pc = 0x44fb57; continue;
 }
 0x44fb57 => {
  { let v: u32 = cpu.r32(0x5b396bu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b396fu32); cpu.ebx = v; }
  { let v: u32 = 0x4e852du32; cpu.w32(0x4d6bf8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x44fb73); f_0041a775(cpu); if cpu.returned_to != 0x44fb73 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x44fb79; continue; }
  pc = 0x44fb77; continue;
 }
 0x44fb77 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44fb79 => {
  { cpu.sub16((cpu.eax as u16), 0xa0u16); }
  if !cpu.zf { pc = 0x44fb80; continue; }
  pc = 0x44fb7f; continue;
 }
 0x44fb7f => {
  {  }
  pc = 0x44fb80; continue;
 }
 0x44fb80 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044fb8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44fb8f); }
 let mut pc: u32 = 0x44fb8f;
 loop { match pc {
 0x44fb8f => {
  { cpu.pushad(); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x44fb9a; continue;
 }
 0x44fb9a => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x44fc5e; continue; }
  pc = 0x44fba5; continue;
 }
 0x44fba5 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x44fc5e; continue; }
  pc = 0x44fbb0; continue;
 }
 0x44fbb0 => {
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
  cpu.push(0x44fbdd); f_004714a5(cpu); if cpu.returned_to != 0x44fbdd { return; }
  if cpu.cf { pc = 0x44fc6e; continue; }
  pc = 0x44fbe3; continue;
 }
 0x44fbe3 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x20u32); cpu.esi = v; }
  cpu.push(0x44fc07); f_004714a5(cpu); if cpu.returned_to != 0x44fc07 { return; }
  if cpu.cf { pc = 0x44fc6e; continue; }
  pc = 0x44fc09; continue;
 }
 0x44fc09 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x44fc2d); f_004714a5(cpu); if cpu.returned_to != 0x44fc2d { return; }
  if cpu.cf { pc = 0x44fc6e; continue; }
  pc = 0x44fc2f; continue;
 }
 0x44fc2f => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x7u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0xfff0u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x44fc56); f_004714a5(cpu); if cpu.returned_to != 0x44fc56 { return; }
  if cpu.cf { pc = 0x44fc6e; continue; }
  pc = 0x44fc58; continue;
 }
 0x44fc58 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x44fc5e; continue;
 }
 0x44fc5e => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44fb9a; continue; }
  pc = 0x44fc6b; continue;
 }
 0x44fc6b => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44fc6e => {
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
pub fn f_0044fc77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44fc77); }
 let mut pc: u32 = 0x44fc77;
 loop { match pc {
 0x44fc77 => {
  { cpu.pushad(); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x44fc86; continue;
 }
 0x44fc86 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44fd58; continue; }
  pc = 0x44fc92; continue;
 }
 0x44fc92 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x44fd58; continue; }
  pc = 0x44fc9d; continue;
 }
 0x44fc9d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56d388u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4b8b48u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x11u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x56d388u32), 0x1u8); }
  if !cpu.zf { pc = 0x44fcdb; continue; }
  pc = 0x44fcd8; continue;
 }
 0x44fcd8 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x44fcdb; continue;
 }
 0x44fcdb => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6be0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6be4u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6bd8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4d6bdcu32)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x44fd27); f_004714a5(cpu); if cpu.returned_to != 0x44fd27 { return; }
  if cpu.cf { pc = 0x44fd66; continue; }
  pc = 0x44fd29; continue;
 }
 0x44fd29 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x20u32); cpu.esi = v; }
  cpu.push(0x44fd35); f_004714a5(cpu); if cpu.returned_to != 0x44fd35 { return; }
  if cpu.cf { pc = 0x44fd66; continue; }
  pc = 0x44fd37; continue;
 }
 0x44fd37 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x44fd43); f_004714a5(cpu); if cpu.returned_to != 0x44fd43 { return; }
  if cpu.cf { pc = 0x44fd66; continue; }
  pc = 0x44fd45; continue;
 }
 0x44fd45 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x20u32); cpu.edi = v; }
  cpu.push(0x44fd54); f_004714a5(cpu); if cpu.returned_to != 0x44fd54 { return; }
  if cpu.cf { pc = 0x44fd66; continue; }
  pc = 0x44fd56; continue;
 }
 0x44fd56 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x44fd58; continue;
 }
 0x44fd58 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44fc86; continue; }
  pc = 0x44fd63; continue;
 }
 0x44fd63 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x44fd66 => {
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
pub fn f_0044fd6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44fd6c); }
 let mut pc: u32 = 0x44fd6c;
 loop { match pc {
 0x44fd6c => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x44fda6; continue; }
  pc = 0x44fd75; continue;
 }
 0x44fd75 => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x44fd7f; continue;
 }
 0x44fd7f => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.sf { pc = 0x44fda0; continue; }
  pc = 0x44fd86; continue;
 }
 0x44fd86 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x44fda0; continue; }
  pc = 0x44fd90; continue;
 }
 0x44fd90 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44fda0; continue; }
  pc = 0x44fd97; continue;
 }
 0x44fd97 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x44fd9e); f_0044fda7(cpu); if cpu.returned_to != 0x44fd9e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x44fda0; continue;
 }
 0x44fda0 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44fd7f; continue; }
  pc = 0x44fda6; continue;
 }
 0x44fda6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044fda7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44fda7); }
 let mut pc: u32 = 0x44fda7;
 loop { match pc {
 0x44fda7 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x44fdd2; continue; }
  pc = 0x44fdae; continue;
 }
 0x44fdae => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x44fdcf); f_004465a4(cpu); if cpu.returned_to != 0x44fdcf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { cpu.poll(); pc = 0x44fda7; continue; }
  pc = 0x44fdd2; continue;
 }
 0x44fdd2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ff0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x44ff0c); }
 let mut pc: u32 = 0x44ff0c;
 loop { match pc {
 0x44ff0c => {
  { let v: u32 = 0x588b9bu32; cpu.edi = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x44ff1b; continue;
 }
 0x44ff1b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44ff1b; continue; }
  pc = 0x44ff26; continue;
 }
 0x44ff26 => {
  { let v: u32 = 0x58909bu32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x44ff35; continue;
 }
 0x44ff35 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x44ff35; continue; }
  pc = 0x44ff40; continue;
 }
 0x44ff40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0044ff41(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x44ff41); } r_0044ff41(cpu, 0x44ff41); }
pub fn f_00450000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450000); } r_0044ff41(cpu, 0x450000); }
pub fn f_00450009(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450009); } r_0044ff41(cpu, 0x450009); }
pub fn f_00450018(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450018); } r_0044ff41(cpu, 0x450018); }
pub fn f_00450019(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450019); } r_0044ff41(cpu, 0x450019); }
pub fn f_0045001d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45001d); } r_0044ff41(cpu, 0x45001d); }
pub fn f_00450026(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450026); } r_0044ff41(cpu, 0x450026); }
pub fn f_00450032(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450032); } r_0044ff41(cpu, 0x450032); }
pub fn f_00450036(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450036); } r_0044ff41(cpu, 0x450036); }
pub fn f_00450044(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450044); } r_0044ff41(cpu, 0x450044); }
pub fn f_00450045(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450045); } r_0044ff41(cpu, 0x450045); }
pub fn f_0045004b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45004b); } r_0044ff41(cpu, 0x45004b); }
pub fn f_0045004c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45004c); } r_0044ff41(cpu, 0x45004c); }
pub fn f_0045004f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45004f); } r_0044ff41(cpu, 0x45004f); }
pub fn f_00450054(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450054); } r_0044ff41(cpu, 0x450054); }
pub fn f_00450056(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450056); } r_0044ff41(cpu, 0x450056); }
pub fn f_00450064(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450064); } r_0044ff41(cpu, 0x450064); }
pub fn f_0045006c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45006c); } r_0044ff41(cpu, 0x45006c); }
pub fn f_00450071(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450071); } r_0044ff41(cpu, 0x450071); }
pub fn f_00450080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450080); } r_0044ff41(cpu, 0x450080); }
pub fn f_00450082(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450082); } r_0044ff41(cpu, 0x450082); }
pub fn f_00450090(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450090); } r_0044ff41(cpu, 0x450090); }
pub fn f_004500a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4500a8); } r_0044ff41(cpu, 0x4500a8); }
pub fn f_00450118(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450118); } r_0044ff41(cpu, 0x450118); }
pub fn f_00450121(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450121); } r_0044ff41(cpu, 0x450121); }
pub fn f_00450170(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x450170); } r_0044ff41(cpu, 0x450170); }
fn r_0044ff41(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x44ff41 => {
  { let v: u32 = 0x4d954cu32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u16 = 0x230u16; cpu.w16(0x4d6792u32, v); }
  { let v: u16 = 0x80u16; cpu.w16(0x4d6794u32, v); }
  { let v: u16 = 0x230u16; cpu.w16(0x4d6796u32, v); }
  { let v: u16 = 0x4e0u16; cpu.w16(0x4d6798u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7f8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67a2u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67a4u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67aau32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67acu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67aeu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67b0u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7fcu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67a6u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67a8u32, v); }
  { cpu.sub8(cpu.r8(0x59805cu32), 0x2u8); }
  if cpu.cf { pc = 0x450019; continue; }
  pc = 0x450006; continue;
 }
 0x450000 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x2u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  if cpu.cf { pc = 0x450019; continue; }
  pc = 0x450006; continue;
 }
 0x450006 => {
  { cpu.sub8(cpu.r8(0x4d67f6u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x4500c1; continue; }
  pc = 0x450013; continue;
 }
 0x450009 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if (cpu.sf == cpu.of) { pc = 0x4500c1; continue; }
  pc = 0x450013; continue;
 }
 0x450013 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4500c1; continue; }
  pc = 0x450019; continue;
 }
 0x450018 => {
  { let v: u8 = cpu.add8(cpu.r8(0xf65c75eeu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1))), ((cpu.ecx >> 8) as u8)); cpu.w8(0xf65c75eeu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let a = ((cpu.ebx >> 8) as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  cpu.unimplemented(0x450021, "Aas");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2))) ^ 0xbeu8; cpu.w8(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x42u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x86c61c75u32); cpu.eax = v; }
  pc = 0x450032; continue;
 }
 0x450019 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x45007a; continue; }
  pc = 0x45001e; continue;
 }
 0x45001d => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  pc = 0x45001e; continue;
 }
 0x45001e => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x45007a; continue; }
  pc = 0x450027; continue;
 }
 0x450026 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x450027; continue;
 }
 0x450027 => {
  { cpu.sub8(cpu.r8(0x4204u32.wrapping_add(cpu.esi)), 0x15u8); }
  if !cpu.zf { pc = 0x45004c; continue; }
  pc = 0x450030; continue;
 }
 0x450030 => {
  { let v: u8 = 0x15u8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x15u8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  pc = 0x450045; continue;
 }
 0x450032 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); }
  { let v: u8 = cpu.r8(0x438386c6u32).wrapping_add((cpu.edx as u8)); cpu.w8(0x438386c6u32, v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x448386c6u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x450044; continue;
 }
 0x450036 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x438386c6u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x448386c6u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x450044; continue;
 }
 0x450044 => {
  cpu.unimplemented(0x450044, "Aas");
  pc = 0x450045; continue;
 }
 0x450045 => {
  { let v: u8 = 0x3fu8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x45004c; continue;
 }
 0x45004b => {
  cpu.unimplemented(0x45004b, "Aas");
  pc = 0x45004c; continue;
 }
 0x45004c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x450053); f_004502ee(cpu); if cpu.returned_to != 0x450053 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x450054; continue;
 }
 0x45004f => {
  {  }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5eu32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x5eu32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6792u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6794u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x450086; continue; }
  pc = 0x45006b; continue;
 }
 0x450054 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6792u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6794u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x450086; continue; }
  pc = 0x45006b; continue;
 }
 0x450056 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6792u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6794u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x450086; continue; }
  pc = 0x45006b; continue;
 }
 0x450064 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x7500004eu32); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.r32(0x680b6u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x680e9u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x680e9u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  pc = 0x450079; continue;
 }
 0x45006b => {
  { let v: u32 = 0x680u32.wrapping_add(cpu.esi); cpu.esi = v; }
  pc = 0x450071; continue;
 }
 0x45006c => {
  { let v: u8 = 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(0x2b); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x450071; continue;
 }
 0x450071 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x680u32); cpu.ecx = v; }
  if !cpu.sf { pc = 0x450086; continue; }
  pc = 0x450079; continue;
 }
 0x450079 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45007a => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x450019; continue; }
  pc = 0x45007c; continue;
 }
 0x45007c => {
  { let v: u32 = 0x4db54cu32; cpu.esi = v; }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  pc = 0x450086; continue;
 }
 0x450080 => {
  { let v: u8 = cpu.r8(0x2000u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x2000u32.wrapping_add(cpu.ecx), v); }
  pc = 0x450086; continue;
 }
 0x450082 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x450086; continue;
 }
 0x450086 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x4500be; continue; }
  pc = 0x45008b; continue;
 }
 0x45008b => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x4500be; continue; }
  pc = 0x450094; continue;
 }
 0x450090 => {
  { let v: u8 = cpu.r8(0xbe802a74u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0xbe802a74u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x42u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x75u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  pc = 0x45009d; continue;
 }
 0x450094 => {
  { cpu.sub8(cpu.r8(0x4204u32.wrapping_add(cpu.esi)), 0x14u8); }
  if !cpu.zf { pc = 0x4500ab; continue; }
  pc = 0x45009d; continue;
 }
 0x45009d => {
  { let v: u8 = 0x3eu8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3eu8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x4500ab; continue;
 }
 0x4500a8 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x4500b0); f_004502ee(cpu); if cpu.returned_to != 0x4500b0 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6796u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6798u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4500ab => {
  cpu.push(0x4500b0); f_004502ee(cpu); if cpu.returned_to != 0x4500b0 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6796u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6798u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4500be => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x450086; continue; }
  pc = 0x4500c0; continue;
 }
 0x4500c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4500c1 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6792u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6796u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d679au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d679eu32, v); }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3a3du32, v); }
  pc = 0x4500f5; continue;
 }
 0x4500f5 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xeeu8); }
  if !cpu.zf { pc = 0x4501a0; continue; }
  pc = 0x4500fe; continue;
 }
 0x4500fe => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x4501a0; continue; }
  pc = 0x45010b; continue;
 }
 0x45010b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x450112); f_004502c4(cpu); if cpu.returned_to != 0x450112 { return; }
  cpu.push(0x450117); f_004502ee(cpu); if cpu.returned_to != 0x450117 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x450125); f_004501ff(cpu); if cpu.returned_to != 0x450125 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x45019f; continue; }
  pc = 0x45012c; continue;
 }
 0x450118 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x450125); f_004501ff(cpu); if cpu.returned_to != 0x450125 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x45019f; continue; }
  pc = 0x45012c; continue;
 }
 0x450121 => {
  { let v = cpu.st(0) + (cpu.r32(0x0u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.set_st(0, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x45019f; continue; }
  pc = 0x45012c; continue;
 }
 0x45012c => {
  cpu.push(0x450131); f_00450244(cpu); if cpu.returned_to != 0x450131 { return; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x450146; continue; }
  pc = 0x450136; continue;
 }
 0x450136 => {
  { let v: u8 = 0x3eu8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3eu8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x45019f; continue;
 }
 0x450146 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x450169; continue; }
  pc = 0x45014b; continue;
 }
 0x45014b => {
  { let v: u8 = 0x15u8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x15u8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x45019f; continue;
 }
 0x450169 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x45017e; continue; }
  pc = 0x45016e; continue;
 }
 0x45016e => {
  { let v: u8 = 0x40u8; cpu.w8(0x4083u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x40u8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x45019f; continue;
 }
 0x450170 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x40u8; cpu.w8(0x4084u32.wrapping_add(cpu.esi), v); }
  pc = 0x45019f; continue;
 }
 0x45017e => {
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x45019f; continue; }
  pc = 0x450183; continue;
 }
 0x450183 => {
  { let v: u8 = 0x3du8; cpu.w8(0x4384u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3du8; cpu.w8(0x4383u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x41u8; cpu.w8(0x4483u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x41u8; cpu.w8(0x4484u32.wrapping_add(cpu.esi), v); }
  pc = 0x45019f; continue;
 }
 0x45019f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x4501a0; continue;
 }
 0x4501a0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4500f5; continue; }
  pc = 0x4501a7; continue;
 }
 0x4501a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004501a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4501a8); }
 let mut pc: u32 = 0x4501a8;
 loop { match pc {
 0x4501a8 => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4501b4; continue;
 }
 0x4501b4 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4501c2; continue; }
  pc = 0x4501bb; continue;
 }
 0x4501bb => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4501b4; continue; }
  pc = 0x4501c0; continue;
 }
 0x4501c0 => {
  pc = 0x4501fb; continue;
 }
 0x4501c2 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x59dc1bu32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4501fb => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004501ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4501ff); }
 let mut pc: u32 = 0x4501ff;
 loop { match pc {
 0x4501ff => {
  { let v: u8 = cpu.r8(0x4204u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x14u8); }
  if cpu.zf { pc = 0x450235; continue; }
  pc = 0x45020b; continue;
 }
 0x45020b => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x450230; continue; }
  pc = 0x45020f; continue;
 }
 0x45020f => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x45022b; continue; }
  pc = 0x450219; continue;
 }
 0x450219 => {
  { cpu.sub8((cpu.eax as u8), 0x3cu8); }
  if cpu.zf { pc = 0x45023a; continue; }
  pc = 0x45021d; continue;
 }
 0x45021d => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if !cpu.zf { pc = 0x45022b; continue; }
  pc = 0x450227; continue;
 }
 0x450227 => {
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x45023f; continue; }
  pc = 0x45022b; continue;
 }
 0x45022b => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x450230 => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x450235 => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45023a => {
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45023f => {
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450244); }
 let mut pc: u32 = 0x450244;
 loop { match pc {
 0x450244 => {
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if cpu.zf { pc = 0x4502a1; continue; }
  pc = 0x45024a; continue;
 }
 0x45024a => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x450288; continue; }
  pc = 0x450250; continue;
 }
 0x450250 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x45026f; continue; }
  pc = 0x450256; continue;
 }
 0x450256 => {
  { cpu.sub16(cpu.r16(0x4d679eu32), 0x0u16); }
  if !cpu.zf { pc = 0x4502bf; continue; }
  pc = 0x450260; continue;
 }
 0x450260 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d679eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67a0u32, v); }
  pc = 0x4502b8; continue;
 }
 0x45026f => {
  { cpu.sub16(cpu.r16(0x4d679au32), 0x0u16); }
  if !cpu.zf { pc = 0x4502bf; continue; }
  pc = 0x450279; continue;
 }
 0x450279 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d679au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d679cu32, v); }
  pc = 0x4502b8; continue;
 }
 0x450288 => {
  { cpu.sub16(cpu.r16(0x4d6796u32), 0x0u16); }
  if !cpu.zf { pc = 0x4502bf; continue; }
  pc = 0x450292; continue;
 }
 0x450292 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6796u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6798u32, v); }
  pc = 0x4502b8; continue;
 }
 0x4502a1 => {
  { cpu.sub16(cpu.r16(0x4d6792u32), 0x0u16); }
  if !cpu.zf { pc = 0x4502bf; continue; }
  pc = 0x4502ab; continue;
 }
 0x4502ab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6792u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6794u32, v); }
  pc = 0x4502b8; continue;
 }
 0x4502b8 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5b3a3du32)); cpu.w8(0x5b3a3du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4502bf => {
  { let v: u16 = 0xffffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004502c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4502c4); }
 let mut pc: u32 = 0x4502c4;
 loop { match pc {
 0x4502c4 => {
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)), 0xeeu8); }
  if !cpu.zf { pc = 0x4502d6; continue; }
  pc = 0x4502ca; continue;
 }
 0x4502ca => {
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x4502d6; continue; }
  pc = 0x4502d3; continue;
 }
 0x4502d3 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x4502c4; continue;
 }
 0x4502d6 => {
  { cpu.sub8(cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)), 0xeeu8); }
  if !cpu.zf { pc = 0x4502ed; continue; }
  pc = 0x4502dc; continue;
 }
 0x4502dc => {
  { cpu.and8(cpu.r8(0x3f80u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x4502ed; continue; }
  pc = 0x4502e5; continue;
 }
 0x4502e5 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x80u32); cpu.esi = v; }
  cpu.poll(); pc = 0x4502d6; continue;
 }
 0x4502ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004502ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4502ee); }
 let mut pc: u32 = 0x4502ee;
 loop { match pc {
 0x4502ee => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4d954du32); cpu.esi = v; }
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
pub fn f_0045030c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45030c); } r_0045030c(cpu, 0x45030c); }
pub fn f_0045033e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x45033e); } r_0045030c(cpu, 0x45033e); }
fn r_0045030c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x45030c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.push(0x450316); f_00430178(cpu); if cpu.returned_to != 0x450316 { return; }
  cpu.push(0x45031b); f_00459598(cpu); if cpu.returned_to != 0x45031b { return; }
  cpu.push(0x450320); f_0041b18d(cpu); if cpu.returned_to != 0x450320 { return; }
  cpu.push(0x450325); f_004801f2(cpu); if cpu.returned_to != 0x450325 { return; }
  cpu.push(0x45032a); f_00480d74(cpu); if cpu.returned_to != 0x45032a { return; }
  cpu.push(0x45032f); f_004674cf(cpu); if cpu.returned_to != 0x45032f { return; }
  cpu.push(0x450334); f_00481818(cpu); if cpu.returned_to != 0x450334 { return; }
  cpu.push(0x450339); f_004487c3(cpu); if cpu.returned_to != 0x450339 { return; }
  cpu.push(0x45033e); f_004819ed(cpu); if cpu.returned_to != 0x45033e { return; }
  pc = 0x45033e; continue;
 }
 0x45033e => {
  cpu.push(0x450343); f_00450675(cpu); if cpu.returned_to != 0x450343 { return; }
  cpu.push(0x450348); f_0044710a(cpu); if cpu.returned_to != 0x450348 { return; }
  cpu.push(0x45034d); f_004800b3(cpu); if cpu.returned_to != 0x45034d { return; }
  cpu.push(0x450352); f_00450666(cpu); if cpu.returned_to != 0x450352 { return; }
  cpu.push(0x450357); f_004847b9(cpu); if cpu.returned_to != 0x450357 { return; }
  cpu.push(0x45035c); f_00450632(cpu); if cpu.returned_to != 0x45035c { return; }
  cpu.push(0x450361); f_00450656(cpu); if cpu.returned_to != 0x450361 { return; }
  cpu.push(0x450366); f_00450620(cpu); if cpu.returned_to != 0x450366 { return; }
  cpu.push(0x45036b); f_0045060e(cpu); if cpu.returned_to != 0x45036b { return; }
  cpu.push(0x450370); f_004821a5(cpu); if cpu.returned_to != 0x450370 { return; }
  cpu.push(0x450375); f_00485e4a(cpu); if cpu.returned_to != 0x450375 { return; }
  cpu.push(0x45037a); f_0047cabe(cpu); if cpu.returned_to != 0x45037a { return; }
  cpu.push(0x45037f); f_0047cad6(cpu); if cpu.returned_to != 0x45037f { return; }
  cpu.push(0x450384); f_0047d8b5(cpu); if cpu.returned_to != 0x450384 { return; }
  cpu.push(0x450389); f_0044bef0(cpu); if cpu.returned_to != 0x450389 { return; }
  cpu.push(0x45038e); f_00449743(cpu); if cpu.returned_to != 0x45038e { return; }
  cpu.push(0x450393); f_00456b24(cpu); if cpu.returned_to != 0x450393 { return; }
  cpu.push(0x450398); f_0047f5c4(cpu); if cpu.returned_to != 0x450398 { return; }
  cpu.push(0x45039d); f_00450fe8(cpu); if cpu.returned_to != 0x45039d { return; }
  { let v: u32 = 0x5159f4u32; cpu.edi = v; }
  cpu.push(0x4503a7); f_004674e8(cpu); if cpu.returned_to != 0x4503a7 { return; }
  { let v: u32 = 0x515d78u32; cpu.edi = v; }
  cpu.push(0x4503b1); f_004674e8(cpu); if cpu.returned_to != 0x4503b1 { return; }
  { let v: u32 = 0x5160fcu32; cpu.edi = v; }
  cpu.push(0x4503bb); f_004674e8(cpu); if cpu.returned_to != 0x4503bb { return; }
  { let v: u32 = 0x516480u32; cpu.edi = v; }
  cpu.push(0x4503c5); f_004674e8(cpu); if cpu.returned_to != 0x4503c5 { return; }
  cpu.push(0x4503ca); f_0046750e(cpu); if cpu.returned_to != 0x4503ca { return; }
  cpu.push(0x4503cf); f_004505e2(cpu); if cpu.returned_to != 0x4503cf { return; }
  cpu.push(0x4503d4); f_00450567(cpu); if cpu.returned_to != 0x4503d4 { return; }
  cpu.push(0x4503d9); f_0044e710(cpu); if cpu.returned_to != 0x4503d9 { return; }
  { let v: u16 = 0x30u16; cpu.w16(0x5a45e4u32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x5a45e0u32, v); }
  { let v: u16 = 0x32u16; cpu.w16(0x5a45e2u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58c62du32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d67fcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6800u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6804u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6808u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c44au32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c44eu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c452u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c456u32, v); }
  { let v: u16 = cpu.r16(0x5b3945u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x13ecu32; cpu.w32(0x5824abu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c87eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56d313u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5824a9u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5820f5u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5820f6u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1569u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e156au32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x576e72u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x576e76u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c28u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6c44u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1557u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d677eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15c5u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e40u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e43u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e4au32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e48u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5a48f1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f71u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6a60u32, v); }
  { let v: u32 = 0x5091d8u32; cpu.w32(0x4d6a34u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6888u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6758u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d688au32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a14u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fc0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c5cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b99f4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5a48f2u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4e1fccu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5b385au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5b385cu32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.w32(0x5b3974u32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.w32(0x5b3978u32, v); }
  { let v: u32 = 0x522514u32; cpu.w32(0x5b397cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b3a3eu32, v); }
  { let v: u32 = 0xddu32; cpu.eax = v; }
  { let v = cpu.r32(0x524b70u32); cpu.mul32(v); }
  { let v = cpu.r32(0x4d67f8u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x524b70u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x58c553u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450567(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450567); }
 let mut pc: u32 = 0x450567;
 loop { match pc {
 0x450567 => {
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d9548u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x45057d); f_004505d4(cpu); if cpu.returned_to != 0x45057d { return; }
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d954au32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x450593); f_004505d4(cpu); if cpu.returned_to != 0x450593 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v = cpu.r8(0x4d954au32); cpu.mul8(v); }
  { let v: u32 = 0x4dd4ccu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d9548u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x4505b5); f_004505d4(cpu); if cpu.returned_to != 0x4505b5 { return; }
  { let v: u32 = cpu.r16(0x4d9548u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x4dd54bu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d954au32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x4505d3); f_004505d4(cpu); if cpu.returned_to != 0x4505d3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004505d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4505d4); }
 let mut pc: u32 = 0x4505d4;
 loop { match pc {
 0x4505d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4505d4; continue; }
  pc = 0x4505e1; continue;
 }
 0x4505e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004505e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4505e2); }
 let mut pc: u32 = 0x4505e2;
 loop { match pc {
 0x4505e2 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4505e9; continue;
 }
 0x4505e9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4505e9; continue; }
  pc = 0x4505fa; continue;
 }
 0x4505fa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004505fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4505fb); }
 let mut pc: u32 = 0x4505fb;
 loop { match pc {
 0x4505fb => {
  { let v: u32 = 0x522514u32; cpu.edi = v; }
  { let v: u32 = 0x578u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045060e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45060e); }
 let mut pc: u32 = 0x45060e;
 loop { match pc {
 0x45060e => {
  { let v: u32 = 0x52195cu32; cpu.edi = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450620); }
 let mut pc: u32 = 0x450620;
 loop { match pc {
 0x450620 => {
  { let v: u32 = 0x520e1cu32; cpu.edi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450632(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450632); }
 let mut pc: u32 = 0x450632;
 loop { match pc {
 0x450632 => {
  { let v: u32 = 0x520cf0u32; cpu.edi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450643(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450643); }
 let mut pc: u32 = 0x450643;
 loop { match pc {
 0x450643 => {
  { let v: u32 = 0x4acdb4u32; cpu.edi = v; }
  { let v: u32 = 0x154u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450656); }
 let mut pc: u32 = 0x450656;
 loop { match pc {
 0x450656 => {
  { let v: u32 = cpu.r32(0x4d6b18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450666(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450666); }
 let mut pc: u32 = 0x450666;
 loop { match pc {
 0x450666 => {
  { let v: u32 = 0x4e8434u32; cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450675(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450675); }
 let mut pc: u32 = 0x450675;
 loop { match pc {
 0x450675 => {
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0x2b20u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450696(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450696); }
 let mut pc: u32 = 0x450696;
 loop { match pc {
 0x450696 => {
  { let v: u32 = 0x505310u32; cpu.edi = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x4acdb4u32; cpu.esi = v; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4506c3; continue;
 }
 0x4506c3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x4506cc; continue;
 }
 0x4506cc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x4506e7; continue; }
  pc = 0x4506d3; continue;
 }
 0x4506d3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1ffu32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4506cc; continue; }
  pc = 0x4506e7; continue;
 }
 0x4506e7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x22u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x4506c3; continue; }
  pc = 0x4506f1; continue;
 }
 0x4506f1 => {
  { let v: u32 = 0x4d954cu32; cpu.esi = v; }
  { let v: u32 = 0x505310u32; cpu.edi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  pc = 0x450700; continue;
 }
 0x450700 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1ffu32; cpu.eax = v; }
  { let v: u32 = 0x54a7d0u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x45078f; continue; }
  pc = 0x45071c; continue;
 }
 0x45071c => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x4acdb4u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4d69ecu32)); cpu.w32(0x4d69ecu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x4d954cu32); cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4d954cu32); cpu.esi = v; }
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
  if !cpu.zf { pc = 0x45076f; continue; }
  pc = 0x450768; continue;
 }
 0x450768 => {
  cpu.push(0x45076d); f_0041c743(cpu); if cpu.returned_to != 0x45076d { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x45076f; continue;
 }
 0x45076f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x1au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x450781; continue; }
  pc = 0x450779; continue;
 }
 0x450779 => {
  cpu.push(0x45077e); f_0041c743(cpu); if cpu.returned_to != 0x45077e { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  pc = 0x450781; continue;
 }
 0x450781 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x509180u32); }
  if !cpu.cf { pc = 0x450797; continue; }
  pc = 0x45078f; continue;
 }
 0x45078f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x450700; continue; }
  pc = 0x450797; continue;
 }
 0x450797 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450798); }
 let mut pc: u32 = 0x450798;
 loop { match pc {
 0x450798 => {
  cpu.push(0x45079d); f_00450c16(cpu); if cpu.returned_to != 0x45079d { return; }
  { cpu.pushad(); }
  cpu.push(0x4507a3); f_0040d1f0(cpu); if cpu.returned_to != 0x4507a3 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4eaab5u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4507b6); f_0040d400(cpu); if cpu.returned_to != 0x4507b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x4507e4; continue; }
  pc = 0x4507be; continue;
 }
 0x4507be => {
  { let v: u8 = 0x32u8; cpu.w8(0x524b74u32, v); }
  { let v: u32 = 0x4eaab5u32; cpu.edx = v; }
  { let v: u32 = 0x5242ccu32; cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6au32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x4507e1); f_00447961(cpu); if cpu.returned_to != 0x4507e1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4507e4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004507e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4507e6); }
 let mut pc: u32 = 0x4507e6;
 loop { match pc {
 0x4507e6 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x4507f0); f_00473e2a(cpu); if cpu.returned_to != 0x4507f0 { return; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x450844; continue; }
  pc = 0x450803; continue;
 }
 0x450803 => {
  cpu.push(0x450808); f_00450798(cpu); if cpu.returned_to != 0x450808 { return; }
  cpu.push(0x45080d); f_004281c4(cpu); if cpu.returned_to != 0x45080d { return; }
  cpu.push(0x450812); f_0046c6ae(cpu); if cpu.returned_to != 0x450812 { return; }
  cpu.push(0x450817); f_00450845(cpu); if cpu.returned_to != 0x450817 { return; }
  cpu.push(0x45081c); f_00450ab7(cpu); if cpu.returned_to != 0x45081c { return; }
  cpu.push(0x450821); f_0045088c(cpu); if cpu.returned_to != 0x450821 { return; }
  cpu.push(0x450826); f_00450a40(cpu); if cpu.returned_to != 0x450826 { return; }
  cpu.push(0x45082b); f_004508b2(cpu); if cpu.returned_to != 0x45082b { return; }
  cpu.push(0x450830); f_00450a5e(cpu); if cpu.returned_to != 0x450830 { return; }
  cpu.push(0x450835); f_004508da(cpu); if cpu.returned_to != 0x450835 { return; }
  cpu.push(0x45083a); f_00450a86(cpu); if cpu.returned_to != 0x45083a { return; }
  cpu.push(0x45083f); f_00450900(cpu); if cpu.returned_to != 0x45083f { return; }
  cpu.push(0x450844); f_00450926(cpu); if cpu.returned_to != 0x450844 { return; }
  pc = 0x450844; continue;
 }
 0x450844 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450845(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450845); }
 let mut pc: u32 = 0x450845;
 loop { match pc {
 0x450845 => {
  { let v: u32 = 0x0u32; cpu.w32(0x576464u32, v); }
  { let v: u32 = 0x4d954cu32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  cpu.push(0x45085e); f_00415463(cpu); if cpu.returned_to != 0x45085e { return; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576464u32, v); }
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  cpu.push(0x450879); f_00415463(cpu); if cpu.returned_to != 0x450879 { return; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x58c7c3u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x576464u32), cpu.edx); cpu.w32(0x576464u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045088c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45088c); }
 let mut pc: u32 = 0x45088c;
 loop { match pc {
 0x45088c => {
  { let v: u32 = 0x0u32; cpu.w32(0x57646cu32, v); }
  { let v: u32 = 0x4b7d84u32; cpu.esi = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  cpu.push(0x4508a5); f_00415463(cpu); if cpu.returned_to != 0x4508a5 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x57646cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004508b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4508b2); }
 let mut pc: u32 = 0x4508b2;
 loop { match pc {
 0x4508b2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x576480u32, v); }
  { let v: u32 = 0x5d6318u32; cpu.esi = v; }
  { let v: u32 = 0x5d658cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x4508cd); f_00415463(cpu); if cpu.returned_to != 0x4508cd { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576480u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004508da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4508da); }
 let mut pc: u32 = 0x4508da;
 loop { match pc {
 0x4508da => {
  { let v: u32 = 0x0u32; cpu.w32(0x576468u32, v); }
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  cpu.push(0x4508f3); f_00415463(cpu); if cpu.returned_to != 0x4508f3 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576468u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450900); }
 let mut pc: u32 = 0x450900;
 loop { match pc {
 0x450900 => {
  { let v: u32 = 0x0u32; cpu.w32(0x57647cu32, v); }
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x208u32; cpu.ecx = v; }
  cpu.push(0x450919); f_00415463(cpu); if cpu.returned_to != 0x450919 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x57647cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450926); }
 let mut pc: u32 = 0x450926;
 loop { match pc {
 0x450926 => {
  { let v: u32 = 0x5b3990u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x5b3997u32, v); }
  { let v: u32 = 0x4eaf80u32; cpu.ebx = v; }
  { let v: u32 = 0x5b3990u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450951); f_00447961(cpu); if cpu.returned_to != 0x450951 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b3997u32)); cpu.w8(0x5b3997u32, v); }
  { let v: u32 = 0x4eb480u32; cpu.ebx = v; }
  { let v: u32 = 0x5b3990u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450972); f_00447961(cpu); if cpu.returned_to != 0x450972 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b3997u32)); cpu.w8(0x5b3997u32, v); }
  { let v: u32 = 0x4eb980u32; cpu.ebx = v; }
  { let v: u32 = 0x5b3990u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450993); f_00447961(cpu); if cpu.returned_to != 0x450993 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b3997u32)); cpu.w8(0x5b3997u32, v); }
  { let v: u32 = 0x4ebe80u32; cpu.ebx = v; }
  { let v: u32 = 0x5b3990u32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x4509b4); f_00447961(cpu); if cpu.returned_to != 0x4509b4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x5b399du32; cpu.edx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x5b39a4u32, v); }
  { let v: u32 = 0x4ec380u32; cpu.ebx = v; }
  { let v: u32 = 0x5b399du32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x4509db); f_00447961(cpu); if cpu.returned_to != 0x4509db { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b39a4u32)); cpu.w8(0x5b39a4u32, v); }
  { let v: u32 = 0x4ec880u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x5b399du32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x4509fc); f_00447961(cpu); if cpu.returned_to != 0x4509fc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b39a4u32)); cpu.w8(0x5b39a4u32, v); }
  { let v: u32 = 0x4ecd80u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x5b399du32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450a1d); f_00447961(cpu); if cpu.returned_to != 0x450a1d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5b39a4u32)); cpu.w8(0x5b39a4u32, v); }
  { let v: u32 = 0x4ed280u32; cpu.ebx = v; }
  { let v: u32 = 0x500u32; cpu.ecx = v; }
  { let v: u32 = 0x5b399du32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450a3e); f_00447961(cpu); if cpu.returned_to != 0x450a3e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450a40); }
 let mut pc: u32 = 0x450a40;
 loop { match pc {
 0x450a40 => {
  { let v: u32 = 0x4eaacfu32; cpu.edx = v; }
  { let v: u32 = 0x5d6318u32; cpu.ebx = v; }
  { let v: u32 = 0x5d658cu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450a5c); f_00447961(cpu); if cpu.returned_to != 0x450a5c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450a5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450a5e); }
 let mut pc: u32 = 0x450a5e;
 loop { match pc {
 0x450a5e => {
  cpu.push(0x450a63); f_004505fb(cpu); if cpu.returned_to != 0x450a63 { return; }
  { cpu.pushad(); }
  cpu.push(0x450a69); f_0040d1f0(cpu); if cpu.returned_to != 0x450a69 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4eaad8u32; cpu.edx = v; }
  { let v: u32 = 0x522514u32; cpu.ebx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x450a84); f_00447961(cpu); if cpu.returned_to != 0x450a84 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450a86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450a86); }
 let mut pc: u32 = 0x450a86;
 loop { match pc {
 0x450a86 => {
  { cpu.pushad(); }
  cpu.push(0x450a8c); f_0040d1f0(cpu); if cpu.returned_to != 0x450a8c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4e1ff4u32; cpu.edi = v; }
  { let v: u32 = 0x208u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x4eaaf1u32; cpu.edx = v; }
  { let v: u32 = 0x4e1ff4u32; cpu.ebx = v; }
  { let v: u32 = 0x208u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x450ab5); f_00447961(cpu); if cpu.returned_to != 0x450ab5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450ab7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450ab7); }
 let mut pc: u32 = 0x450ab7;
 loop { match pc {
 0x450ab7 => {
  { cpu.pushad(); }
  cpu.push(0x450abd); f_0040d1f0(cpu); if cpu.returned_to != 0x450abd { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4eaac2u32; cpu.edx = v; }
  { let v: u32 = 0x4b7d84u32; cpu.ebx = v; }
  { let v: u32 = 0x5dcu32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x450ad8); f_00447961(cpu); if cpu.returned_to != 0x450ad8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450ada(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450ada); }
 let mut pc: u32 = 0x450ada;
 loop { match pc {
 0x450ada => {
  { let v: u32 = 0xf0u32; cpu.w32(0x58c640u32, v); }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0x58c644u32; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c644u32, v); }
  { let v: u32 = 0x4eaaaau32; cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x450b17); f_00447961(cpu); if cpu.returned_to != 0x450b17 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x64u32); }
  if (cpu.cf || cpu.zf) { pc = 0x450c06; continue; }
  pc = 0x450b26; continue;
 }
 0x450b26 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x450b32; continue;
 }
 0x450b32 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4d67dau32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x450b3e); f_0040d1f0(cpu); if cpu.returned_to != 0x450b3e { return; }
  cpu.push(0x450b43); f_0040d850(cpu); if cpu.returned_to != 0x450b43 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x5dfb00u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450b55); f_0040d400(cpu); if cpu.returned_to != 0x450b55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x450bd0; continue; }
  pc = 0x450b5d; continue;
 }
 0x450b5d => {
  { cpu.pushad(); }
  { let v: u32 = 0x5dfb00u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450b6f); f_0040d370(cpu); if cpu.returned_to != 0x450b6f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x450bd0; continue; }
  pc = 0x450b81; continue;
 }
 0x450b81 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x4ae77cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x450b9c); f_0041c901(cpu); if cpu.returned_to != 0x450b9c { return; }
  cpu.push(0x450ba1); f_0041c997(cpu); if cpu.returned_to != 0x450ba1 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x9eu32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x450bd0; continue; }
  pc = 0x450bab; continue;
 }
 0x450bab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c644u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x4d67dau32) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59dc1bu32.wrapping_add(cpu.ecx.wrapping_mul(2)), v); }
  pc = 0x450bd0; continue;
 }
 0x450bd0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x450bdf; continue; }
  pc = 0x450bd5; continue;
 }
 0x450bd5 => {
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  cpu.poll(); pc = 0x450b32; continue;
 }
 0x450bdf => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xa0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x450b32; continue; }
  pc = 0x450bec; continue;
 }
 0x450bec => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d67dau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x588b6du32, v); }
  cpu.push(0x450bfe); f_0040d1f0(cpu); if cpu.returned_to != 0x450bfe { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x588b6du32, v); }
  { cpu.popad(); }
  pc = 0x450c06; continue;
 }
 0x450c06 => {
  cpu.push(0x450c0b); f_00450c16(cpu); if cpu.returned_to != 0x450c0b { return; }
  cpu.push(0x450c10); f_0047abd8(cpu); if cpu.returned_to != 0x450c10 { return; }
  cpu.push(0x450c15); f_0044915a(cpu); if cpu.returned_to != 0x450c15 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450c16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450c16); }
 let mut pc: u32 = 0x450c16;
 loop { match pc {
 0x450c16 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = 0x4ea994u32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  cpu.push(0x450c40); f_00450c82(cpu); if cpu.returned_to != 0x450c40 { return; }
  { let v: u32 = 0x2eu32; cpu.eax = v; }
  { let v: u32 = 0x4eaab5u32; cpu.edi = v; }
  cpu.push(0x450c4f); f_00450c82(cpu); if cpu.returned_to != 0x450c4f { return; }
  { let v: u32 = 0x61u32; cpu.eax = v; }
  { let v: u32 = 0x4eaac2u32; cpu.edi = v; }
  cpu.push(0x450c5e); f_00450c82(cpu); if cpu.returned_to != 0x450c5e { return; }
  { let v: u32 = 0x84u32; cpu.eax = v; }
  { let v: u32 = 0x4eaad8u32; cpu.edi = v; }
  cpu.push(0x450c6d); f_00450c82(cpu); if cpu.returned_to != 0x450c6d { return; }
  { let v: u32 = 0x91u32; cpu.eax = v; }
  { let v: u32 = 0x4eaaf1u32; cpu.edi = v; }
  cpu.push(0x450c7c); f_00450c82(cpu); if cpu.returned_to != 0x450c7c { return; }
  cpu.push(0x450c81); f_00450cfe(cpu); if cpu.returned_to != 0x450c81 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450c82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450c82); }
 let mut pc: u32 = 0x450c82;
 loop { match pc {
 0x450c82 => {
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
pub fn f_00450c8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450c8d); }
 let mut pc: u32 = 0x450c8d;
 loop { match pc {
 0x450c8d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450cb3); f_0040d8b0(cpu); if cpu.returned_to != 0x450cb3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = 0x2eu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450cc5); f_0040d8b0(cpu); if cpu.returned_to != 0x450cc5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = 0x61u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450cd7); f_0040d8b0(cpu); if cpu.returned_to != 0x450cd7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = 0x84u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450ce9); f_0040d8b0(cpu); if cpu.returned_to != 0x450ce9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = 0x91u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x450cfb); f_0040d8b0(cpu); if cpu.returned_to != 0x450cfb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450cfe); }
 let mut pc: u32 = 0x450cfe;
 loop { match pc {
 0x450cfe => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7eau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x5b39aau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0x581a17u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450d2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450d2e); }
 let mut pc: u32 = 0x450d2e;
 loop { match pc {
 0x450d2e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x54acd1u32, v); }
  { let v: u32 = 0x54ad44u32; cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c49du32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4e5u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4e9u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4edu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c4e1u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c635u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fcbu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c434u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c439u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c43au32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c432u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c433u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58c435u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582420u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582424u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x582438u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58243cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x56c819u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58849eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b7u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d684cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15c1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15a5u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fb9u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6ad4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1564u32, v); }
  { let v: u16 = 0x1au16; cpu.w16(0x4d68d6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f51u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d68dau32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d68e0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e156fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x576bcbu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x576bc7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581b66u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581b70u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581b76u32, v); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67ceu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67d2u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67ccu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67d0u32, v); }
  { let v: u32 = 0x32u32; cpu.w32(0x575f24u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450e9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450e9d); }
 let mut pc: u32 = 0x450e9d;
 loop { match pc {
 0x450e9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450ef5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450ef5); }
 let mut pc: u32 = 0x450ef5;
 loop { match pc {
 0x450ef5 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x450f79; continue; }
  pc = 0x450f0e; continue;
 }
 0x450f0e => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e909au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e90b7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e90b8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e90bbu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e90b9u32, v); }
  { let v: u8 = 0xf6u8; cpu.w8(0x4e8512u32, v); }
  { let v: u8 = 0x8u8; cpu.w8(0x4e8579u32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x4ea178u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4ea195u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4ea196u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x4ea199u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e93bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e93deu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e9513u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e9532u32, v); }
  pc = 0x450fe2; continue;
 }
 0x450f79 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e909au32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e90b7u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e90b8u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e90bbu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e90b9u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e8579u32, v); }
  { let v: u8 = 0xf6u8; cpu.w8(0x4e8512u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea178u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea195u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea196u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea199u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e93bfu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e93deu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9513u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9532u32, v); }
  pc = 0x450fe2; continue;
 }
 0x450fe2 => {
  cpu.push(0x450fe7); f_00450fe8(cpu); if cpu.returned_to != 0x450fe7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00450fe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x450fe8); }
 let mut pc: u32 = 0x450fe8;
 loop { match pc {
 0x450fe8 => {
  { let v: u32 = 0x4e97f4u32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e97f4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9837u32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9837u32; cpu.edi = v; }
  { let v: u32 = 0x4e987au32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e987au32; cpu.edi = v; }
  { let v: u32 = 0x4e98bdu32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e98bdu32; cpu.edi = v; }
  { let v: u32 = 0x4e8b89u32; cpu.edi = v; }
  { let v: u32 = 0x4e8b46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8b89u32; cpu.edi = v; }
  { let v: u32 = 0x4e8bccu32; cpu.edi = v; }
  { let v: u32 = 0x4e8b46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8bccu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c0fu32; cpu.edi = v; }
  { let v: u32 = 0x4e8b46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8c0fu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c52u32; cpu.edi = v; }
  { let v: u32 = 0x4e8b46u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8c52u32; cpu.edi = v; }
  { let v: u32 = 0x4e90d4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9091u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e90d4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9117u32; cpu.edi = v; }
  { let v: u32 = 0x4e9091u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9117u32; cpu.edi = v; }
  { let v: u32 = 0x4e915au32; cpu.edi = v; }
  { let v: u32 = 0x4e9091u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e915au32; cpu.edi = v; }
  { let v: u32 = 0x4e919du32; cpu.edi = v; }
  { let v: u32 = 0x4e9091u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e919du32; cpu.edi = v; }
  { let v: u32 = 0x4e9223u32; cpu.edi = v; }
  { let v: u32 = 0x4e91e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9223u32; cpu.edi = v; }
  { let v: u32 = 0x4e9266u32; cpu.edi = v; }
  { let v: u32 = 0x4e91e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9266u32; cpu.edi = v; }
  { let v: u32 = 0x4e92a9u32; cpu.edi = v; }
  { let v: u32 = 0x4e91e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e92a9u32; cpu.edi = v; }
  { let v: u32 = 0x4e92ecu32; cpu.edi = v; }
  { let v: u32 = 0x4e91e0u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e92ecu32; cpu.edi = v; }
  { let v: u32 = 0x4e9943u32; cpu.edi = v; }
  { let v: u32 = 0x4e9900u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9943u32; cpu.edi = v; }
  { let v: u32 = 0x4e9986u32; cpu.edi = v; }
  { let v: u32 = 0x4e9900u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9986u32; cpu.edi = v; }
  { let v: u32 = 0x4e99c9u32; cpu.edi = v; }
  { let v: u32 = 0x4e9900u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e99c9u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a0cu32; cpu.edi = v; }
  { let v: u32 = 0x4e9900u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9a0cu32; cpu.edi = v; }
  { let v: u32 = 0x4e97f4u32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e97f4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9837u32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9837u32; cpu.edi = v; }
  { let v: u32 = 0x4e987au32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e987au32; cpu.edi = v; }
  { let v: u32 = 0x4e98bdu32; cpu.edi = v; }
  { let v: u32 = 0x4e97b1u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e98bdu32; cpu.edi = v; }
  { let v: u32 = 0x4e9cf8u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x45120e); f_00451642(cpu); if cpu.returned_to != 0x45120e { return; }
  { let v: u32 = 0x4e9d3cu32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9d3cu32; cpu.edi = v; }
  cpu.push(0x451229); f_00451642(cpu); if cpu.returned_to != 0x451229 { return; }
  { let v: u32 = 0x4e9d80u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9d80u32; cpu.edi = v; }
  cpu.push(0x451244); f_0045164c(cpu); if cpu.returned_to != 0x451244 { return; }
  cpu.push(0x451249); f_00451642(cpu); if cpu.returned_to != 0x451249 { return; }
  { let v: u32 = 0x4e9dc4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9dc4u32; cpu.edi = v; }
  cpu.push(0x451264); f_00451642(cpu); if cpu.returned_to != 0x451264 { return; }
  { let v: u32 = 0x4e9e08u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9e08u32; cpu.edi = v; }
  cpu.push(0x45127f); f_00451656(cpu); if cpu.returned_to != 0x45127f { return; }
  cpu.push(0x451284); f_00451642(cpu); if cpu.returned_to != 0x451284 { return; }
  { let v: u32 = 0x4e9ba4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9be8u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9be8u32; cpu.edi = v; }
  cpu.push(0x4512b0); f_00451647(cpu); if cpu.returned_to != 0x4512b0 { return; }
  { let v: u32 = 0x4e9c2cu32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9c70u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9c70u32; cpu.edi = v; }
  cpu.push(0x4512dc); f_00451651(cpu); if cpu.returned_to != 0x4512dc { return; }
  { let v: u32 = 0x4e9cb4u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9a94u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9a94u32; cpu.edi = v; }
  cpu.push(0x451308); f_00451647(cpu); if cpu.returned_to != 0x451308 { return; }
  { let v: u32 = 0x4e9ad8u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9ad8u32; cpu.edi = v; }
  cpu.push(0x451323); f_0045164c(cpu); if cpu.returned_to != 0x451323 { return; }
  { let v: u32 = 0x4e9b1cu32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9b1cu32; cpu.edi = v; }
  cpu.push(0x45133e); f_00451651(cpu); if cpu.returned_to != 0x45133e { return; }
  { let v: u32 = 0x4e9b60u32; cpu.edi = v; }
  { let v: u32 = 0x4e9a50u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9b60u32; cpu.edi = v; }
  cpu.push(0x451359); f_00451656(cpu); if cpu.returned_to != 0x451359 { return; }
  { let v: u32 = 0x4e965eu32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x45136f); f_00451642(cpu); if cpu.returned_to != 0x45136f { return; }
  { let v: u32 = 0x4e96a2u32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e96a2u32; cpu.edi = v; }
  cpu.push(0x45138a); f_00451642(cpu); if cpu.returned_to != 0x45138a { return; }
  { let v: u32 = 0x4e96e6u32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e96e6u32; cpu.edi = v; }
  cpu.push(0x4513a5); f_0045164c(cpu); if cpu.returned_to != 0x4513a5 { return; }
  cpu.push(0x4513aa); f_00451642(cpu); if cpu.returned_to != 0x4513aa { return; }
  { let v: u32 = 0x4e972au32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e972au32; cpu.edi = v; }
  cpu.push(0x4513c5); f_00451642(cpu); if cpu.returned_to != 0x4513c5 { return; }
  { let v: u32 = 0x4e976eu32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e976eu32; cpu.edi = v; }
  cpu.push(0x4513e0); f_00451656(cpu); if cpu.returned_to != 0x4513e0 { return; }
  cpu.push(0x4513e5); f_00451642(cpu); if cpu.returned_to != 0x4513e5 { return; }
  { let v: u32 = 0x4e93fau32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e93fau32; cpu.edi = v; }
  cpu.push(0x451400); f_00451647(cpu); if cpu.returned_to != 0x451400 { return; }
  cpu.push(0x451405); f_00451642(cpu); if cpu.returned_to != 0x451405 { return; }
  { let v: u32 = 0x4e943eu32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e943eu32; cpu.edi = v; }
  cpu.push(0x451420); f_0045164c(cpu); if cpu.returned_to != 0x451420 { return; }
  cpu.push(0x451425); f_00451642(cpu); if cpu.returned_to != 0x451425 { return; }
  { let v: u32 = 0x4e9482u32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9482u32; cpu.edi = v; }
  cpu.push(0x451440); f_00451651(cpu); if cpu.returned_to != 0x451440 { return; }
  cpu.push(0x451445); f_00451642(cpu); if cpu.returned_to != 0x451445 { return; }
  { let v: u32 = 0x4e94c6u32; cpu.edi = v; }
  { let v: u32 = 0x4e93b6u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e94c6u32; cpu.edi = v; }
  cpu.push(0x451460); f_00451656(cpu); if cpu.returned_to != 0x451460 { return; }
  cpu.push(0x451465); f_00451642(cpu); if cpu.returned_to != 0x451465 { return; }
  { let v: u32 = 0x4e954eu32; cpu.edi = v; }
  { let v: u32 = 0x4e950au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e954eu32; cpu.edi = v; }
  cpu.push(0x451480); f_00451647(cpu); if cpu.returned_to != 0x451480 { return; }
  cpu.push(0x451485); f_00451642(cpu); if cpu.returned_to != 0x451485 { return; }
  { let v: u32 = 0x4e9592u32; cpu.edi = v; }
  { let v: u32 = 0x4e950au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e9592u32; cpu.edi = v; }
  cpu.push(0x4514a0); f_00451642(cpu); if cpu.returned_to != 0x4514a0 { return; }
  { let v: u32 = 0x4e95d6u32; cpu.edi = v; }
  { let v: u32 = 0x4e950au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e95d6u32; cpu.edi = v; }
  cpu.push(0x4514bb); f_00451651(cpu); if cpu.returned_to != 0x4514bb { return; }
  cpu.push(0x4514c0); f_00451642(cpu); if cpu.returned_to != 0x4514c0 { return; }
  { let v: u32 = 0x4e961au32; cpu.edi = v; }
  { let v: u32 = 0x4e950au32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e961au32; cpu.edi = v; }
  cpu.push(0x4514db); f_00451642(cpu); if cpu.returned_to != 0x4514db { return; }
  { let v: u32 = 0x4e8f3eu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8f3eu32; cpu.edi = v; }
  cpu.push(0x4514f6); f_00451642(cpu); if cpu.returned_to != 0x4514f6 { return; }
  { let v: u32 = 0x4e8f82u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8f82u32; cpu.edi = v; }
  cpu.push(0x451511); f_00451642(cpu); if cpu.returned_to != 0x451511 { return; }
  { let v: u32 = 0x4e8fc6u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8fc6u32; cpu.edi = v; }
  cpu.push(0x45152c); f_0045164c(cpu); if cpu.returned_to != 0x45152c { return; }
  cpu.push(0x451531); f_00451642(cpu); if cpu.returned_to != 0x451531 { return; }
  { let v: u32 = 0x4e900au32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e900au32; cpu.edi = v; }
  cpu.push(0x45154c); f_00451642(cpu); if cpu.returned_to != 0x45154c { return; }
  { let v: u32 = 0x4e904eu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e904eu32; cpu.edi = v; }
  cpu.push(0x451567); f_00451656(cpu); if cpu.returned_to != 0x451567 { return; }
  cpu.push(0x45156c); f_00451642(cpu); if cpu.returned_to != 0x45156c { return; }
  { let v: u32 = 0x4e8deau32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8e2eu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8e2eu32; cpu.edi = v; }
  cpu.push(0x451598); f_00451647(cpu); if cpu.returned_to != 0x451598 { return; }
  { let v: u32 = 0x4e8e72u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8eb6u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8eb6u32; cpu.edi = v; }
  cpu.push(0x4515c4); f_00451651(cpu); if cpu.returned_to != 0x4515c4 { return; }
  { let v: u32 = 0x4e8efau32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8cdau32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8cdau32; cpu.edi = v; }
  cpu.push(0x4515f0); f_00451647(cpu); if cpu.returned_to != 0x4515f0 { return; }
  { let v: u32 = 0x4e8d1eu32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8d1eu32; cpu.edi = v; }
  cpu.push(0x45160b); f_0045164c(cpu); if cpu.returned_to != 0x45160b { return; }
  { let v: u32 = 0x4e8d62u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8d62u32; cpu.edi = v; }
  cpu.push(0x451626); f_00451651(cpu); if cpu.returned_to != 0x451626 { return; }
  { let v: u32 = 0x4e8da6u32; cpu.edi = v; }
  { let v: u32 = 0x4e8c96u32; cpu.esi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x4e8da6u32; cpu.edi = v; }
  cpu.push(0x451641); f_00451656(cpu); if cpu.returned_to != 0x451641 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451642(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451642); }
 let mut pc: u32 = 0x451642;
 loop { match pc {
 0x451642 => {
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451647(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451647); }
 let mut pc: u32 = 0x451647;
 loop { match pc {
 0x451647 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3eu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045164c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45164c); }
 let mut pc: u32 = 0x45164c;
 loop { match pc {
 0x45164c => {
  { let v: u8 = 0xffu8; cpu.w8(0x3fu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451651(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451651); }
 let mut pc: u32 = 0x451651;
 loop { match pc {
 0x451651 => {
  { let v: u8 = 0xffu8; cpu.w8(0x40u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451656); }
 let mut pc: u32 = 0x451656;
 loop { match pc {
 0x451656 => {
  { let v: u8 = 0xffu8; cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045165b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45165b); }
 let mut pc: u32 = 0x45165b;
 loop { match pc {
 0x45165b => {
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x5b3a3eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451678; continue; }
  pc = 0x451669; continue;
 }
 0x451669 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451686; continue; }
  pc = 0x45166d; continue;
 }
 0x45166d => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x45167f; continue; }
  pc = 0x451671; continue;
 }
 0x451671 => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x4516e3; continue;
 }
 0x451678 => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x45168d; continue;
 }
 0x45167f => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x45168d; continue;
 }
 0x451686 => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x4516e3; continue;
 }
 0x45168d => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e9be8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9c2cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9c70u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9cb4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e954eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9592u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e95d6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e961au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8e2eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8e72u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8eb6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8efau32.wrapping_add(cpu.eax), v); }
  pc = 0x451737; continue;
 }
 0x4516e3 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e9d3cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9d80u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9dc4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9e08u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e96a2u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e96e6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e972au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e976eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8f82u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8fc6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e900au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e904eu32.wrapping_add(cpu.eax), v); }
  pc = 0x451737; continue;
 }
 0x451737 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e8cdau32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8d1eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8d62u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e8da6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9a94u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9ad8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9b1cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9b60u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e93fau32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e943eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e9482u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4e94c6u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045178c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45178c); }
 let mut pc: u32 = 0x45178c;
 loop { match pc {
 0x45178c => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x5b3a42u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x451797); cpu.call(t); if cpu.returned_to != 0x451797 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  let t = cpu.r32(0x5b3a42u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x4517a1); cpu.call(t); if cpu.returned_to != 0x4517a1 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004517a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4517a3); }
 let mut pc: u32 = 0x4517a3;
 loop { match pc {
 0x4517a3 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5b3a3eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x4517b7; continue; }
  pc = 0x4517b0; continue;
 }
 0x4517b0 => {
  let t = cpu.r32(0x5b3a42u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x4517b7); cpu.call(t); if cpu.returned_to != 0x4517b7 { return; }
  pc = 0x4517b7; continue;
 }
 0x4517b7 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x5b3a3eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x4517ca; continue; }
  pc = 0x4517c3; continue;
 }
 0x4517c3 => {
  let t = cpu.r32(0x5b3a42u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x4517ca); cpu.call(t); if cpu.returned_to != 0x4517ca { return; }
  pc = 0x4517ca; continue;
 }
 0x4517ca => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004517cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4517cc); }
 let mut pc: u32 = 0x4517cc;
 loop { match pc {
 0x4517cc => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4517df; continue; }
  pc = 0x4517d0; continue;
 }
 0x4517d0 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4517ed; continue; }
  pc = 0x4517d4; continue;
 }
 0x4517d4 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4517e6; continue; }
  pc = 0x4517d8; continue;
 }
 0x4517d8 => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x451808; continue;
 }
 0x4517df => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x4517f4; continue;
 }
 0x4517e6 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x4517f4; continue;
 }
 0x4517ed => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x451808; continue;
 }
 0x4517f4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9be8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e954eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8e2eu32.wrapping_add(cpu.eax), v); }
  pc = 0x45181a; continue;
 }
 0x451808 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9d3cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e96a2u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8f82u32.wrapping_add(cpu.eax), v); }
  pc = 0x45181a; continue;
 }
 0x45181a => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8cdau32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9a94u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e93fau32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045182d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45182d); }
 let mut pc: u32 = 0x45182d;
 loop { match pc {
 0x45182d => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451840; continue; }
  pc = 0x451831; continue;
 }
 0x451831 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x45184e; continue; }
  pc = 0x451835; continue;
 }
 0x451835 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451847; continue; }
  pc = 0x451839; continue;
 }
 0x451839 => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x451869; continue;
 }
 0x451840 => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x451855; continue;
 }
 0x451847 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x451855; continue;
 }
 0x45184e => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x451869; continue;
 }
 0x451855 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9c2cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9592u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8e72u32.wrapping_add(cpu.eax), v); }
  pc = 0x45187b; continue;
 }
 0x451869 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9d80u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e96e6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8fc6u32.wrapping_add(cpu.eax), v); }
  pc = 0x45187b; continue;
 }
 0x45187b => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8d1eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9ad8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e943eu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045188e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45188e); }
 let mut pc: u32 = 0x45188e;
 loop { match pc {
 0x45188e => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4518a1; continue; }
  pc = 0x451892; continue;
 }
 0x451892 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4518af; continue; }
  pc = 0x451896; continue;
 }
 0x451896 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4518a8; continue; }
  pc = 0x45189a; continue;
 }
 0x45189a => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x4518ca; continue;
 }
 0x4518a1 => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x4518b6; continue;
 }
 0x4518a8 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x4518b6; continue;
 }
 0x4518af => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x4518ca; continue;
 }
 0x4518b6 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9c70u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e95d6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8eb6u32.wrapping_add(cpu.eax), v); }
  pc = 0x4518dc; continue;
 }
 0x4518ca => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9dc4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e972au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e900au32.wrapping_add(cpu.eax), v); }
  pc = 0x4518dc; continue;
 }
 0x4518dc => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8d62u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9b1cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9482u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004518ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4518ef); }
 let mut pc: u32 = 0x4518ef;
 loop { match pc {
 0x4518ef => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451902; continue; }
  pc = 0x4518f3; continue;
 }
 0x4518f3 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451910; continue; }
  pc = 0x4518f7; continue;
 }
 0x4518f7 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x451909; continue; }
  pc = 0x4518fb; continue;
 }
 0x4518fb => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x45192b; continue;
 }
 0x451902 => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x451917; continue;
 }
 0x451909 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x451917; continue;
 }
 0x451910 => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x45192b; continue;
 }
 0x451917 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9cb4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e961au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8efau32.wrapping_add(cpu.eax), v); }
  pc = 0x45193d; continue;
 }
 0x45192b => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9e08u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e976eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e904eu32.wrapping_add(cpu.eax), v); }
  pc = 0x45193d; continue;
 }
 0x45193d => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e8da6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e9b60u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e94c6u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451950); }
 let mut pc: u32 = 0x451950;
 loop { match pc {
 0x451950 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x451958; continue; }
  pc = 0x451957; continue;
 }
 0x451957 => {
  {  }
  pc = 0x451958; continue;
 }
 0x451958 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451959(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451959); }
 let mut pc: u32 = 0x451959;
 loop { match pc {
 0x451959 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c45au32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4519a4; continue; }
  pc = 0x451967; continue;
 }
 0x451967 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x45198e; continue; }
  pc = 0x451973; continue;
 }
 0x451973 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x451998; continue; }
  pc = 0x451989; continue;
 }
 0x451989 => {
  pc = 0x451a34; continue;
 }
 0x45198e => {
  { cpu.sub16((cpu.eax as u16), 0xd80u16); }
  if cpu.zf { pc = 0x451a34; continue; }
  pc = 0x451998; continue;
 }
 0x451998 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x4519e9; continue;
 }
 0x4519a4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x4519cc; continue; }
  pc = 0x4519b3; continue;
 }
 0x4519b3 => {
  { cpu.sub16((cpu.eax as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x4519bd; continue; }
  pc = 0x4519b9; continue;
 }
 0x4519b9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x451a34; continue;
 }
 0x4519bd => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4519e9; continue;
 }
 0x4519cc => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x52bbf0u32); cpu.esi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4519e9; continue;
 }
 0x4519e9 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x451a34; continue; }
  pc = 0x4519ef; continue;
 }
 0x4519ef => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x451a34; continue; }
  pc = 0x4519ff; continue;
 }
 0x4519ff => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x451a29; continue; }
  pc = 0x451a08; continue;
 }
 0x451a08 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ebx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x451a29; continue; }
  pc = 0x451a20; continue;
 }
 0x451a20 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c45au32, v); }
  pc = 0x451a34; continue;
 }
 0x451a29 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x451a34; continue; }
  pc = 0x451a32; continue;
 }
 0x451a32 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451a34 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00451ac6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x451ac6); }
 let mut pc: u32 = 0x451ac6;
 loop { match pc {
 0x451ac6 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x451adf; continue; }
  pc = 0x451acd; continue;
 }
 0x451acd => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x451adf; continue; }
  pc = 0x451ad4; continue;
 }
 0x451ad4 => {
  { cpu.sub8(cpu.r8(0x66u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x451d1b; continue; }
  pc = 0x451adf; continue;
 }
 0x451adf => {
  cpu.push(0x451ae4); f_0044b7dd(cpu); if cpu.returned_to != 0x451ae4 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x451bd7; continue; }
  pc = 0x451aef; continue;
 }
 0x451aef => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x451b01; continue; }
  pc = 0x451af6; continue;
 }
 0x451af6 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x451bd7; continue; }
  pc = 0x451b01; continue;
 }
 0x451b01 => {
  { cpu.pushad(); }
  cpu.push(0x451b07); f_00489968(cpu); if cpu.returned_to != 0x451b07 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x4ba552u32; cpu.edi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x451b12; continue;
 }
 0x451b12 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x451b1e; continue; }
  pc = 0x451b18; continue;
 }
 0x451b18 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x451b12; continue; }
  pc = 0x451b1d; continue;
 }
 0x451b1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451b1e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x451b34); f_0042f53c(cpu); if cpu.returned_to != 0x451b34 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  cpu.push(0x451b44); f_00476a47(cpu); if cpu.returned_to != 0x451b44 { return; }
  cpu.push(0x451b49); f_0048990c(cpu); if cpu.returned_to != 0x451b49 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x451bd6; continue; }
  pc = 0x451b5a; continue;
 }
 0x451b5a => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if !cpu.zf { pc = 0x451b6d; continue; }
  pc = 0x451b66; continue;
 }
 0x451b66 => {
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451bbb; continue;
 }
 0x451b6d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if !cpu.zf { pc = 0x451b7b; continue; }
  pc = 0x451b74; continue;
 }
 0x451b74 => {
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451bbb; continue;
 }
 0x451b7b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if !cpu.zf { pc = 0x451b89; continue; }
  pc = 0x451b82; continue;
 }
 0x451b82 => {
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451bbb; continue;
 }
 0x451b89 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x451b97; continue; }
  pc = 0x451b90; continue;
 }
 0x451b90 => {
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451bbb; continue;
 }
 0x451b97 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x451ba3; continue; }
  pc = 0x451b9e; continue;
 }
 0x451b9e => {
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451ba3; continue;
 }
 0x451ba3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x451baf; continue; }
  pc = 0x451baa; continue;
 }
 0x451baa => {
  { let v: u8 = 0x3cu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451baf; continue;
 }
 0x451baf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x451bbb; continue; }
  pc = 0x451bb6; continue;
 }
 0x451bb6 => {
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451bbb; continue;
 }
 0x451bbb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451bc9; continue; }
  pc = 0x451bc4; continue;
 }
 0x451bc4 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451bce; continue; }
  pc = 0x451bc9; continue;
 }
 0x451bc9 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451bce; continue;
 }
 0x451bce => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x451bd6; continue;
 }
 0x451bd6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451bd7 => {
  cpu.push(0x451bdc); f_00489637(cpu); if cpu.returned_to != 0x451bdc { return; }
  { let v: u16 = cpu.shr16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 4u32); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x451be9; continue; }
  pc = 0x451be4; continue;
 }
 0x451be4 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x48u32.wrapping_add(cpu.ebp))); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x451be9; continue;
 }
 0x451be9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x451bd6; continue; }
  pc = 0x451bfa; continue;
 }
 0x451bfa => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x451cd9; continue; }
  pc = 0x451c05; continue;
 }
 0x451c05 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x451c55; continue; }
  pc = 0x451c0c; continue;
 }
 0x451c0c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x451c97; continue; }
  pc = 0x451c17; continue;
 }
 0x451c17 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x451cb8; continue; }
  pc = 0x451c22; continue;
 }
 0x451c22 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x451cfa; continue; }
  pc = 0x451c2d; continue;
 }
 0x451c2d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x451c76; continue; }
  pc = 0x451c34; continue;
 }
 0x451c34 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451c47; continue; }
  pc = 0x451c42; continue;
 }
 0x451c42 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451c4c; continue; }
  pc = 0x451c47; continue;
 }
 0x451c47 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451c4c; continue;
 }
 0x451c4c => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451c55 => {
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451c68; continue; }
  pc = 0x451c63; continue;
 }
 0x451c63 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451c6d; continue; }
  pc = 0x451c68; continue;
 }
 0x451c68 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451c6d; continue;
 }
 0x451c6d => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451c76 => {
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451c89; continue; }
  pc = 0x451c84; continue;
 }
 0x451c84 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451c8e; continue; }
  pc = 0x451c89; continue;
 }
 0x451c89 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451c8e; continue;
 }
 0x451c8e => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451c97 => {
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451caa; continue; }
  pc = 0x451ca5; continue;
 }
 0x451ca5 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451caf; continue; }
  pc = 0x451caa; continue;
 }
 0x451caa => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451caf; continue;
 }
 0x451caf => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451cb8 => {
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451ccb; continue; }
  pc = 0x451cc6; continue;
 }
 0x451cc6 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451cd0; continue; }
  pc = 0x451ccb; continue;
 }
 0x451ccb => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451cd0; continue;
 }
 0x451cd0 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451cd9 => {
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451cec; continue; }
  pc = 0x451ce7; continue;
 }
 0x451ce7 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451cf1; continue; }
  pc = 0x451cec; continue;
 }
 0x451cec => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451cf1; continue;
 }
 0x451cf1 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451cfa => {
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x451d0d; continue; }
  pc = 0x451d08; continue;
 }
 0x451d08 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x451d12; continue; }
  pc = 0x451d0d; continue;
 }
 0x451d0d => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x451d12; continue;
 }
 0x451d12 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x451d1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452129(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452129); }
 let mut pc: u32 = 0x452129;
 loop { match pc {
 0x452129 => {
  { let v: u8 = 0xau8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452138); f_00489968(cpu); if cpu.returned_to != 0x452138 { return; }
  { let v: u8 = 0x8u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0045214e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x45214e); }
 let mut pc: u32 = 0x45214e;
 loop { match pc {
 0x45214e => {
  { let v: u8 = 0x31u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x452176); f_00481755(cpu); if cpu.returned_to != 0x452176 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x45217f); f_00451950(cpu); if cpu.returned_to != 0x45217f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452180); }
 let mut pc: u32 = 0x452180;
 loop { match pc {
 0x452180 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x452194; continue;
 }
 0x452194 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4521c6; continue; }
  pc = 0x452199; continue;
 }
 0x452199 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4521c6; continue; }
  pc = 0x4521af; continue;
 }
 0x4521af => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x7u8); }
  if cpu.zf { pc = 0x4521bb; continue; }
  pc = 0x4521b5; continue;
 }
 0x4521b5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4521c6; continue; }
  pc = 0x4521bb; continue;
 }
 0x4521bb => {
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4521c6 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x452194; continue; }
  pc = 0x4521cb; continue;
 }
 0x4521cb => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004521d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4521d0); }
 let mut pc: u32 = 0x4521d0;
 loop { match pc {
 0x4521d0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4521e4; continue;
 }
 0x4521e4 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x45220a; continue; }
  pc = 0x4521e9; continue;
 }
 0x4521e9 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x45220a; continue; }
  pc = 0x4521ff; continue;
 }
 0x4521ff => {
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x45220a => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4521e4; continue; }
  pc = 0x45220f; continue;
 }
 0x45220f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452214(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452214); }
 let mut pc: u32 = 0x452214;
 loop { match pc {
 0x452214 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x452273; continue; }
  pc = 0x45221b; continue;
 }
 0x45221b => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x452233; continue;
 }
 0x452233 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x45226d; continue; }
  pc = 0x452238; continue;
 }
 0x452238 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x45226d; continue; }
  pc = 0x45224c; continue;
 }
 0x45224c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x45226d; continue; }
  pc = 0x452253; continue;
 }
 0x452253 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x45225b); f_00454265(cpu); if cpu.returned_to != 0x45225b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x45226d; continue;
 }
 0x45226d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x452233; continue; }
  pc = 0x452272; continue;
 }
 0x452272 => {
  { cpu.popad(); }
  pc = 0x452273; continue;
 }
 0x452273 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004523a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4523a1); }
 let mut pc: u32 = 0x4523a1;
 loop { match pc {
 0x4523a1 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x4523e3; continue; }
  pc = 0x4523aa; continue;
 }
 0x4523aa => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xcu8); }
  if cpu.zf { pc = 0x4523e3; continue; }
  pc = 0x4523b1; continue;
 }
 0x4523b1 => {
  cpu.push(0x4523b6); f_00451959(cpu); if cpu.returned_to != 0x4523b6 { return; }
  if !cpu.cf { pc = 0x4523e3; continue; }
  pc = 0x4523b8; continue;
 }
 0x4523b8 => {
  { cpu.sub8(cpu.r8(0x58c45au32), 0x0u8); }
  if !cpu.zf { pc = 0x4523c6; continue; }
  pc = 0x4523c1; continue;
 }
 0x4523c1 => {
  cpu.push(0x4523c6); f_00454204(cpu); if cpu.returned_to != 0x4523c6 { return; }
  pc = 0x4523c6; continue;
 }
 0x4523c6 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4523e5; continue; }
  pc = 0x4523cd; continue;
 }
 0x4523cd => {
  cpu.push(0x4523d2); f_0044b9aa(cpu); if cpu.returned_to != 0x4523d2 { return; }
  cpu.push(0x4523d7); f_00489968(cpu); if cpu.returned_to != 0x4523d7 { return; }
  cpu.push(0x4523dc); f_0043ffdb(cpu); if cpu.returned_to != 0x4523dc { return; }
  cpu.push(0x4523e1); f_0043f830(cpu); if cpu.returned_to != 0x4523e1 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4523e3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4523e5 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x4523c1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004523ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4523ec); }
 let mut pc: u32 = 0x4523ec;
 loop { match pc {
 0x4523ec => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x4524ad; continue; }
  pc = 0x4523f9; continue;
 }
 0x4523f9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x452512; continue; }
  pc = 0x452404; continue;
 }
 0x452404 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x52u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x452512; continue; }
  pc = 0x45244d; continue;
 }
 0x45244d => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x452463); f_004712e3(cpu); if cpu.returned_to != 0x452463 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  if !cpu.cf { pc = 0x452476; continue; }
  pc = 0x45246c; continue;
 }
 0x45246c => {
  { let v: u32 = 0x4e9ed1u32; cpu.w32(0x5d6d3cu32, v); }
  pc = 0x452476; continue;
 }
 0x452476 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x45249f); f_00489ce0(cpu); if cpu.returned_to != 0x45249f { return; }
  { let v: u32 = 0x4e9e4bu32; cpu.w32(0x5d6d3cu32, v); }
  if cpu.cf { pc = 0x452512; continue; }
  pc = 0x4524ab; continue;
 }
 0x4524ab => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4524ad => {
  cpu.push(0x4524b2); f_00451959(cpu); if cpu.returned_to != 0x4524b2 { return; }
  if cpu.cf { pc = 0x452512; continue; }
  pc = 0x4524b4; continue;
 }
 0x4524b4 => {
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
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x452512; continue; }
  pc = 0x4524f7; continue;
 }
 0x4524f7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x45250e); f_00489ce0(cpu); if cpu.returned_to != 0x45250e { return; }
  if cpu.cf { pc = 0x452512; continue; }
  pc = 0x452510; continue;
 }
 0x452510 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452512 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00452514(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x452514); }
 let mut pc: u32 = 0x452514;
 loop { match pc {
 0x452514 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x452582; continue; }
  pc = 0x45251b; continue;
 }
 0x45251b => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x452582; continue; }
  pc = 0x452524; continue;
 }
 0x452524 => {
  cpu.push(0x452529); f_00451959(cpu); if cpu.returned_to != 0x452529 { return; }
  if cpu.cf { pc = 0x452567; continue; }
  pc = 0x45252b; continue;
 }
 0x45252b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x452539); f_00458258(cpu); if cpu.returned_to != 0x452539 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x452567; continue; }
  pc = 0x45253c; continue;
 }
 0x45253c => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x452567; continue; }
  pc = 0x452549; continue;
 }
 0x452549 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x452567; continue; }
  pc = 0x45254f; continue;
 }
 0x45254f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x4e15bdu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x452565); f_00489e05(cpu); if cpu.returned_to != 0x452565 { return; }
  if !cpu.cf { pc = 0x452582; continue; }
  pc = 0x452567; continue;
 }
 0x452567 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x452576); f_0044b9aa(cpu); if cpu.returned_to != 0x452576 { return; }
  cpu.push(0x45257b); f_00489968(cpu); if cpu.returned_to != 0x45257b { return; }
  cpu.push(0x452580); f_0043ffdb(cpu); if cpu.returned_to != 0x452580 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x452582 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
