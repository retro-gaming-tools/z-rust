#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010626a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10626a6); }
 let mut pc: u32 = 0x10626a6;
 loop { match pc {
 0x10626a6 => {
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.edi, 0x10ee910u32); }
  if cpu.cf { cpu.poll(); pc = 0x10626a6; continue; }
  pc = 0x10626af; continue;
 }
 0x10626af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010626b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10626b0); }
 let mut pc: u32 = 0x10626b0;
 loop { match pc {
 0x10626b0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010626b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10626b1); }
 let mut pc: u32 = 0x10626b1;
 loop { match pc {
 0x10626b1 => {
  cpu.push(0x10626b6); f_01068db3(cpu); if cpu.returned_to != 0x10626b6 { return; }
  if cpu.cf { pc = 0x10627ab; continue; }
  pc = 0x10626bc; continue;
 }
 0x10626bc => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10626d3); f_010576b2(cpu); if cpu.returned_to != 0x10626d3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x10627a9; continue; }
  pc = 0x10626dc; continue;
 }
 0x10626dc => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10627a7; continue; }
  pc = 0x10626ef; continue;
 }
 0x10626ef => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1062701; continue; }
  pc = 0x10626f5; continue;
 }
 0x10626f5 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106270e; continue;
 }
 0x1062701 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106270e; continue;
 }
 0x106270e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x106271f); f_01059dbe(cpu); if cpu.returned_to != 0x106271f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1062744); f_01036a75(cpu); if cpu.returned_to != 0x1062744 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x1062759); f_010632a5(cpu); if cpu.returned_to != 0x1062759 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10ee963u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef304u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4060u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x10e3d4du32, v); }
  { cpu.pushad(); }
  cpu.push(0x106278e); f_010628e9(cpu); if cpu.returned_to != 0x106278e { return; }
  if cpu.cf { pc = 0x1062795; continue; }
  pc = 0x1062790; continue;
 }
 0x1062790 => {
  cpu.push(0x1062795); f_01035fd6(cpu); if cpu.returned_to != 0x1062795 { return; }
  pc = 0x1062795; continue;
 }
 0x1062795 => {
  { cpu.popad(); }
  cpu.push(0x106279b); f_01057945(cpu); if cpu.returned_to != 0x106279b { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x10627a7); f_01062ff7(cpu); if cpu.returned_to != 0x10627a7 { return; }
  pc = 0x10627a7; continue;
 }
 0x10627a7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10627a9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10627ab => {
  cpu.push(0x10627b0); f_010576b2(cpu); if cpu.returned_to != 0x10627b0 { return; }
  cpu.push(0x10627b5); f_01068d2a(cpu); if cpu.returned_to != 0x10627b5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10627c5; continue; }
  pc = 0x10627b9; continue;
 }
 0x10627b9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106283b; continue; }
  pc = 0x10627bf; continue;
 }
 0x10627bf => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x106283b; continue; }
  pc = 0x10627c5; continue;
 }
 0x10627c5 => {
  cpu.push(0x10627ca); f_01057d3c(cpu); if cpu.returned_to != 0x10627ca { return; }
  cpu.push(0x10627cf); f_01018baf(cpu); if cpu.returned_to != 0x10627cf { return; }
  if !cpu.cf { pc = 0x10627d9; continue; }
  pc = 0x10627d1; continue;
 }
 0x10627d1 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x106283d; continue; }
  pc = 0x10627d9; continue;
 }
 0x10627d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10627e0); f_01009e97(cpu); if cpu.returned_to != 0x10627e0 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d64cu32); cpu.esi = v; }
  if !cpu.zf { pc = 0x106283d; continue; }
  pc = 0x10627ec; continue;
 }
 0x10627ec => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x1062825; continue; }
  pc = 0x106280d; continue;
 }
 0x106280d => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if !cpu.zf { pc = 0x1062825; continue; }
  pc = 0x1062816; continue;
 }
 0x1062816 => {
  { let v: u16 = cpu.r16(0x10ee95eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10ee960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106283d; continue;
 }
 0x1062825 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106282e); f_01018e6e(cpu); if cpu.returned_to != 0x106282e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106283d; continue; }
  pc = 0x1062834; continue;
 }
 0x1062834 => {
  cpu.push(0x1062839); f_01039557(cpu); if cpu.returned_to != 0x1062839 { return; }
  if cpu.cf { pc = 0x106283d; continue; }
  pc = 0x106283b; continue;
 }
 0x106283b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106283d => {
  cpu.push(0x1062842); f_0101a233(cpu); if cpu.returned_to != 0x1062842 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0xd80u16; cpu.w16(0x10e3e0fu32, v); }
  { cpu.sub8(cpu.r8(0x10ee967u32), 0x0u8); }
  if cpu.zf { pc = 0x106286e; continue; }
  pc = 0x1062861; continue;
 }
 0x1062861 => {
  { let v: u16 = cpu.r16(0x10ee95eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10ee960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106286e; continue;
 }
 0x106286e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e13u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x1062888); f_01059dbe(cpu); if cpu.returned_to != 0x1062888 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1062895); f_01036a75(cpu); if cpu.returned_to != 0x1062895 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x10628aa); f_010632a5(cpu); if cpu.returned_to != 0x10628aa { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef304u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d4du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d4fu32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  cpu.push(0x10628d6); f_01035f7d(cpu); if cpu.returned_to != 0x10628d6 { return; }
  cpu.push(0x10628db); f_01057945(cpu); if cpu.returned_to != 0x10628db { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x10628e7); f_01062ff7(cpu); if cpu.returned_to != 0x10628e7 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010628e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10628e9); }
 let mut pc: u32 = 0x10628e9;
 loop { match pc {
 0x10628e9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10628f9); f_0105dae1(cpu); if cpu.returned_to != 0x10628f9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1062915; continue; }
  pc = 0x10628fc; continue;
 }
 0x10628fc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1062956; continue; }
  pc = 0x1062900; continue;
 }
 0x1062900 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1u32); cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1062908; continue;
 }
 0x1062908 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1062911; continue; }
  pc = 0x106290d; continue;
 }
 0x106290d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1062956; continue; }
  pc = 0x1062911; continue;
 }
 0x1062911 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1062908; continue; }
  pc = 0x1062915; continue;
 }
 0x1062915 => {
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x106292d); f_010868d2(cpu); if cpu.returned_to != 0x106292d { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4060u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x106294d); f_010868d2(cpu); if cpu.returned_to != 0x106294d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4138u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1062959; continue; }
  pc = 0x1062956; continue;
 }
 0x1062956 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062959 => {
  cpu.push(0x106295e); f_010364e8(cpu); if cpu.returned_to != 0x106295e { return; }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062961(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062961); }
 let mut pc: u32 = 0x1062961;
 loop { match pc {
 0x1062961 => {
  cpu.push(0x1062966); f_010576b2(cpu); if cpu.returned_to != 0x1062966 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x106296c); f_01068db3(cpu); if cpu.returned_to != 0x106296c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1062995; continue; }
  pc = 0x106296f; continue;
 }
 0x106296f => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1062995; continue; }
  pc = 0x1062991; continue;
 }
 0x1062991 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1062997; continue;
 }
 0x1062995 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1062997; continue;
 }
 0x1062997 => {
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x106299e); f_010576b2(cpu); if cpu.returned_to != 0x106299e { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x10629c0; continue; }
  pc = 0x10629a8; continue;
 }
 0x10629a8 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x10629c0; continue; }
  pc = 0x10629ae; continue;
 }
 0x10629ae => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10629c0); f_01062bdc(cpu); if cpu.returned_to != 0x10629c0 { return; }
  pc = 0x10629c0; continue;
 }
 0x10629c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010629c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10629c1); }
 let mut pc: u32 = 0x10629c1;
 loop { match pc {
 0x10629c1 => {
  { cpu.sub8(cpu.r8(0x10ee928u32), 0x0u8); }
  if cpu.zf { pc = 0x1062b12; continue; }
  pc = 0x10629ce; continue;
 }
 0x10629ce => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x10629f3; continue; }
  pc = 0x10629d8; continue;
 }
 0x10629d8 => {
  { cpu.pushad(); }
  cpu.push(0x10629de); f_010576b2(cpu); if cpu.returned_to != 0x10629de { return; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x11056a1u32)); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x10629f3; continue; }
  pc = 0x10629e7; continue;
 }
 0x10629e7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee928u32, v); }
  pc = 0x1062b12; continue;
 }
 0x10629f3 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee928u32)); cpu.w8(0x10ee928u32, v); }
  if !cpu.zf { pc = 0x10629fb; continue; }
  pc = 0x10629fb; continue;
 }
 0x10629fb => {
  { let v: u16 = cpu.r16(0x10ee95cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x1062a11; continue; }
  pc = 0x1062a08; continue;
 }
 0x1062a08 => {
  { let v: u16 = 0xad7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1062ad6; continue;
 }
 0x1062a11 => {
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x1062a2c; continue; }
  pc = 0x1062a17; continue;
 }
 0x1062a17 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xadfu32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062a2c => {
  { cpu.sub16((cpu.ecx as u16), 0x2du16); }
  if !cpu.zf { pc = 0x1062a47; continue; }
  pc = 0x1062a32; continue;
 }
 0x1062a32 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xac3u32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062a47 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x1062a5f; continue; }
  pc = 0x1062a4d; continue;
 }
 0x1062a4d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xabbu32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062a5f => {
  { cpu.sub16((cpu.ecx as u16), 0x2bu16); }
  if !cpu.zf { pc = 0x1062a77; continue; }
  pc = 0x1062a65; continue;
 }
 0x1062a65 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xacbu32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062a77 => {
  { cpu.sub16((cpu.ecx as u16), 0x2cu16); }
  if !cpu.zf { pc = 0x1062a8f; continue; }
  pc = 0x1062a7d; continue;
 }
 0x1062a7d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xad3u32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062a8f => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x1062aa7; continue; }
  pc = 0x1062a95; continue;
 }
 0x1062a95 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb19u32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062aa7 => {
  { cpu.sub16((cpu.ecx as u16), 0xae4u16); }
  if !cpu.zf { pc = 0x1062ac0; continue; }
  pc = 0x1062aae; continue;
 }
 0x1062aae => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xae4u32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062ac0 => {
  { cpu.sub16((cpu.ecx as u16), 0x6u16); }
  if !cpu.zf { pc = 0x1062ad6; continue; }
  pc = 0x1062ac6; continue;
 }
 0x1062ac6 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb11u32); cpu.ecx = v; }
  pc = 0x1062ad6; continue;
 }
 0x1062ad6 => {
  { cpu.sub32(cpu.r32(0x10ee963u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1062af1; continue; }
  pc = 0x1062adf; continue;
 }
 0x1062adf => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10ee963u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1062afe; continue;
 }
 0x1062af1 => {
  { let v: u16 = cpu.r16(0x10ee958u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10ee95au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1062afe; continue;
 }
 0x1062afe => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1062b12); cpu.call(t); if cpu.returned_to != 0x1062b12 { return; }
  pc = 0x1062b12; continue;
 }
 0x1062b12 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062b13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062b13); }
 let mut pc: u32 = 0x1062b13;
 loop { match pc {
 0x1062b13 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x1062b2e; continue; }
  pc = 0x1062b25; continue;
 }
 0x1062b25 => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x1062b55; continue; }
  pc = 0x1062b2c; continue;
 }
 0x1062b2c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062b2e => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11757a4u32); cpu.esi = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1062b60; continue; }
  pc = 0x1062b3f; continue;
 }
 0x1062b3f => {
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  pc = 0x1062b4b; continue;
 }
 0x1062b4b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062b55 => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  cpu.poll(); pc = 0x1062b4b; continue;
 }
 0x1062b60 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x15u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062b6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062b6c); }
 let mut pc: u32 = 0x1062b6c;
 loop { match pc {
 0x1062b6c => {
  { let v: u16 = cpu.r16(0x10e3d09u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.r16(0x10e3ce1u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x10e3ce1u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1062b7f); f_010576b2(cpu); if cpu.returned_to != 0x1062b7f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062bdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062bdc); }
 let mut pc: u32 = 0x1062bdc;
 loop { match pc {
 0x1062bdc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1062c0b; continue; }
  pc = 0x1062be2; continue;
 }
 0x1062be2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1062be9); f_010338d0(cpu); if cpu.returned_to != 0x1062be9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xadfu32); cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1062c0a); cpu.call(t); if cpu.returned_to != 0x1062c0a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062c0b => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x1062c24; continue; }
  pc = 0x1062c11; continue;
 }
 0x1062c11 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xad3u32); cpu.ecx = v; }
  pc = 0x1062c35; continue;
 }
 0x1062c24 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xabbu32); cpu.ecx = v; }
  pc = 0x1062c35; continue;
 }
 0x1062c35 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1062c3b); cpu.call(t); if cpu.returned_to != 0x1062c3b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062c3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062c3c); }
 let mut pc: u32 = 0x1062c3c;
 loop { match pc {
 0x1062c3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062c3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062c3d); }
 let mut pc: u32 = 0x1062c3d;
 loop { match pc {
 0x1062c3d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 5u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x3ffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1062c87; continue; }
  pc = 0x1062c83; continue;
 }
 0x1062c83 => {
  { let v: u16 = 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1062c87; continue;
 }
 0x1062c87 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10e4028u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d01u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3cf9u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062ca4); }
 let mut pc: u32 = 0x1062ca4;
 loop { match pc {
 0x1062ca4 => {
  cpu.push(0x1062ca9); f_01062caa(cpu); if cpu.returned_to != 0x1062ca9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062caa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062caa); }
 let mut pc: u32 = 0x1062caa;
 loop { match pc {
 0x1062caa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062cab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062cab); }
 let mut pc: u32 = 0x1062cab;
 loop { match pc {
 0x1062cab => {
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062cbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062cbb); }
 let mut pc: u32 = 0x1062cbb;
 loop { match pc {
 0x1062cbb => {
  cpu.push(0x1062cc0); f_010599d1(cpu); if cpu.returned_to != 0x1062cc0 { return; }
  cpu.push(0x1062cc5); f_010576b2(cpu); if cpu.returned_to != 0x1062cc5 { return; }
  cpu.push(0x1062cca); f_01057d3c(cpu); if cpu.returned_to != 0x1062cca { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cfdu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3cffu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1062cdf); f_01068db3(cpu); if cpu.returned_to != 0x1062cdf { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x1062f20; continue; }
  pc = 0x1062ce7; continue;
 }
 0x1062ce7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1062ced); f_010576b2(cpu); if cpu.returned_to != 0x1062ced { return; }
  cpu.push(0x1062cf2); f_01057d3c(cpu); if cpu.returned_to != 0x1062cf2 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1062d81; continue; }
  pc = 0x1062cfc; continue;
 }
 0x1062cfc => {
  { cpu.sub16(cpu.r16(0x10e3db7u32), 0xaecu16); }
  if cpu.zf { pc = 0x1062d51; continue; }
  pc = 0x1062d07; continue;
 }
 0x1062d07 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x1062d51; continue; }
  pc = 0x1062d0d; continue;
 }
 0x1062d0d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1062d1f); f_0104885a(cpu); if cpu.returned_to != 0x1062d1f { return; }
  if cpu.cf { pc = 0x1062e00; continue; }
  pc = 0x1062d25; continue;
 }
 0x1062d25 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1062d37); f_01009e97(cpu); if cpu.returned_to != 0x1062d37 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1062d81; continue; }
  pc = 0x1062d48; continue;
 }
 0x1062d48 => {
  { cpu.pushad(); }
  cpu.push(0x1062d4e); f_01039557(cpu); if cpu.returned_to != 0x1062d4e { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1062d81; continue; }
  pc = 0x1062d51; continue;
 }
 0x1062d51 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1062d6a); f_01036a75(cpu); if cpu.returned_to != 0x1062d6a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee928u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062d81 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1062d88); f_01057d3c(cpu); if cpu.returned_to != 0x1062d88 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1062d8e); f_01018e6e(cpu); if cpu.returned_to != 0x1062d8e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x1062d93; continue; }
  pc = 0x1062d93; continue;
 }
 0x1062d93 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1062d9a); f_01009e97(cpu); if cpu.returned_to != 0x1062d9a { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d64cu32); cpu.esi = v; }
  if !cpu.zf { pc = 0x1062e02; continue; }
  pc = 0x1062da6; continue;
 }
 0x1062da6 => {
  { cpu.pushad(); }
  cpu.push(0x1062dac); f_01039557(cpu); if cpu.returned_to != 0x1062dac { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1062e02; continue; }
  pc = 0x1062daf; continue;
 }
 0x1062daf => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1062e00; continue; }
  pc = 0x1062db5; continue;
 }
 0x1062db5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x1062dc0); f_01059dbe(cpu); if cpu.returned_to != 0x1062dc0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1062de9); f_01036a75(cpu); if cpu.returned_to != 0x1062de9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee928u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  pc = 0x1062e00; continue;
 }
 0x1062e00 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062e02 => {
  cpu.push(0x1062e07); f_0101a233(cpu); if cpu.returned_to != 0x1062e07 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef210u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x1062e1f); f_01059dbe(cpu); if cpu.returned_to != 0x1062e1f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1062e3b); f_01036a75(cpu); if cpu.returned_to != 0x1062e3b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x1062e50); f_010632a5(cpu); if cpu.returned_to != 0x1062e50 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef304u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d4du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d4fu32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1062e84; continue; }
  pc = 0x1062e7d; continue;
 }
 0x1062e7d => {
  cpu.push(0x1062e82); f_01035f24(cpu); if cpu.returned_to != 0x1062e82 { return; }
  pc = 0x1062e89; continue;
 }
 0x1062e84 => {
  cpu.push(0x1062e89); f_01035fd6(cpu); if cpu.returned_to != 0x1062e89 { return; }
  pc = 0x1062e89; continue;
 }
 0x1062e89 => {
  cpu.push(0x1062e8e); f_01057945(cpu); if cpu.returned_to != 0x1062e8e { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x1062e9a); f_01062ff7(cpu); if cpu.returned_to != 0x1062e9a { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062f20 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x10e3e0fu32, v); }
  cpu.push(0x1062f31); f_01062fda(cpu); if cpu.returned_to != 0x1062f31 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4060u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x1062f6b); f_010632a5(cpu); if cpu.returned_to != 0x1062f6b { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10ee963u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef304u32, v); }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d0fu32, v); }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d11u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x1062fa9); f_01059dbe(cpu); if cpu.returned_to != 0x1062fa9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1062faf); f_01036a75(cpu); if cpu.returned_to != 0x1062faf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1062fc3; continue; }
  pc = 0x1062fbc; continue;
 }
 0x1062fbc => {
  cpu.push(0x1062fc1); f_01035f24(cpu); if cpu.returned_to != 0x1062fc1 { return; }
  pc = 0x1062fc8; continue;
 }
 0x1062fc3 => {
  cpu.push(0x1062fc8); f_01035fd6(cpu); if cpu.returned_to != 0x1062fc8 { return; }
  pc = 0x1062fc8; continue;
 }
 0x1062fc8 => {
  cpu.push(0x1062fcd); f_01057945(cpu); if cpu.returned_to != 0x1062fcd { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x1062fd9); f_01062ff7(cpu); if cpu.returned_to != 0x1062fd9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062e9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062e9c); }
 let mut pc: u32 = 0x1062e9c;
 loop { match pc {
 0x1062e9c => {
  { let v: u16 = 0xd80u16; cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e13u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xad3u32; cpu.eax = v; }
  cpu.push(0x1062ec6); f_01059dbe(cpu); if cpu.returned_to != 0x1062ec6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1062ecc); f_01036a75(cpu); if cpu.returned_to != 0x1062ecc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x1062ee1); f_010632a5(cpu); if cpu.returned_to != 0x1062ee1 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef304u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d4du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d4fu32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  cpu.push(0x1062f0d); f_01035f7d(cpu); if cpu.returned_to != 0x1062f0d { return; }
  cpu.push(0x1062f12); f_01057945(cpu); if cpu.returned_to != 0x1062f12 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x1062f1e); f_01062ff7(cpu); if cpu.returned_to != 0x1062f1e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062fda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062fda); }
 let mut pc: u32 = 0x1062fda;
 loop { match pc {
 0x1062fda => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1062fe6; continue; }
  pc = 0x1062fe0; continue;
 }
 0x1062fe0 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062fe6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1152e54u32); cpu.eax = v; }
  { let v: u16 = 0xef1fu16; cpu.w16(0x10e3e0fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062ff7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062ff7); }
 let mut pc: u32 = 0x1062ff7;
 loop { match pc {
 0x1062ff7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062ff8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062ff8); }
 let mut pc: u32 = 0x1062ff8;
 loop { match pc {
 0x1062ff8 => {
  { let v: u32 = 0x10c69c6u32; cpu.esi = v; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1063006); f_0103c5a5(cpu); if cpu.returned_to != 0x1063006 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063008); }
 let mut pc: u32 = 0x1063008;
 loop { match pc {
 0x1063008 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x1063025); f_01057d3c(cpu); if cpu.returned_to != 0x1063025 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.zf { pc = 0x10630cf; continue; }
  pc = 0x1063037; continue;
 }
 0x1063037 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x110567cu32), 0x0u8); }
  if cpu.zf { pc = 0x1063060; continue; }
  pc = 0x1063041; continue;
 }
 0x1063041 => {
  { cpu.pushad(); }
  cpu.push(0x1063047); f_0101b70a(cpu); if cpu.returned_to != 0x1063047 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1063060; continue; }
  pc = 0x106304a; continue;
 }
 0x106304a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10c6a48u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x10630e5; continue;
 }
 0x1063060 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d769u32, v); }
  cpu.push(0x106306d); f_010191d1(cpu); if cpu.returned_to != 0x106306d { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e40c0u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x10630cf; continue; }
  pc = 0x106307c; continue;
 }
 0x106307c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.ecx as u16), 0x1fu16); }
  if !cpu.zf { pc = 0x10630cf; continue; }
  pc = 0x106309b; continue;
 }
 0x106309b => {
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10630c3; continue; }
  pc = 0x10630b5; continue;
 }
 0x10630b5 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x10630c3; continue; }
  pc = 0x10630bb; continue;
 }
 0x10630bb => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10630c3; continue;
 }
 0x10630c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x10630e5; continue;
 }
 0x10630cf => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x10630d5); f_01018e6e(cpu); if cpu.returned_to != 0x10630d5 { return; }
  if cpu.cf { pc = 0x10630e5; continue; }
  pc = 0x10630d7; continue;
 }
 0x10630d7 => {
  cpu.push(0x10630dc); f_01059141(cpu); if cpu.returned_to != 0x10630dc { return; }
  if cpu.cf { pc = 0x10630e5; continue; }
  pc = 0x10630de; continue;
 }
 0x10630de => {
  cpu.push(0x10630e3); f_01058fff(cpu); if cpu.returned_to != 0x10630e3 { return; }
  if cpu.cf { pc = 0x1063113; continue; }
  pc = 0x10630e5; continue;
 }
 0x10630e5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x118ccc4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x118ccc6u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x118ccc8u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11a37ecu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11a37eeu32, v); }
  pc = 0x106311b; continue;
 }
 0x1063113 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106311b => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x106316a; continue; }
  pc = 0x1063136; continue;
 }
 0x1063136 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x106316a; continue; }
  pc = 0x106313f; continue;
 }
 0x106313f => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x106315a; continue; }
  pc = 0x1063148; continue;
 }
 0x1063148 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106316a; continue;
 }
 0x106315a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106316a; continue;
 }
 0x106316a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x110567cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1063184; continue; }
  pc = 0x106317f; continue;
 }
 0x106317f => {
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  pc = 0x1063184; continue;
 }
 0x1063184 => {
  cpu.push(0x1063189); f_01059dbe(cpu); if cpu.returned_to != 0x1063189 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.r32(0x118ccc4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e0fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x10631a7); f_01036a75(cpu); if cpu.returned_to != 0x10631a7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x10631c6); f_010632a5(cpu); if cpu.returned_to != 0x10631c6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10ef304u32, v); }
  cpu.push(0x10631d9); f_01035f7d(cpu); if cpu.returned_to != 0x10631d9 { return; }
  cpu.push(0x10631de); f_01057945(cpu); if cpu.returned_to != 0x10631de { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x10631ea); f_01062ff7(cpu); if cpu.returned_to != 0x10631ea { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010631f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10631f2); }
 let mut pc: u32 = 0x10631f2;
 loop { match pc {
 0x10631f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x1152e54u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10ee958u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10ee95au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee962u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xacbu32; cpu.eax = v; }
  cpu.push(0x1063254); f_01059dbe(cpu); if cpu.returned_to != 0x1063254 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1063261); f_01036a75(cpu); if cpu.returned_to != 0x1063261 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d09u32, v); }
  { let v: u8 = 0x17u8; cpu.w8(0x10ee928u32, v); }
  cpu.push(0x1063280); f_010632a5(cpu); if cpu.returned_to != 0x1063280 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10ef304u32, v); }
  cpu.push(0x1063293); f_01035f7d(cpu); if cpu.returned_to != 0x1063293 { return; }
  cpu.push(0x1063298); f_01057945(cpu); if cpu.returned_to != 0x1063298 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x10632a4); f_01062ff7(cpu); if cpu.returned_to != 0x10632a4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010632a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10632a5); }
 let mut pc: u32 = 0x10632a5;
 loop { match pc {
 0x10632a5 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10ee963u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x10632d4; continue; }
  pc = 0x10632c7; continue;
 }
 0x10632c7 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10632d4; continue;
 }
 0x10632d4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d5fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d61u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee962u32), 0x0u8); }
  if !cpu.zf { pc = 0x1063306; continue; }
  pc = 0x10632f9; continue;
 }
 0x10632f9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10ee958u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10ee95au32, v); }
  pc = 0x1063306; continue;
 }
 0x1063306 => {
  { let v: u16 = cpu.r16(0x10e3db7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10ee95cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee962u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106331c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106331c); }
 let mut pc: u32 = 0x106331c;
 loop { match pc {
 0x106331c => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1063323; continue; }
  pc = 0x1063322; continue;
 }
 0x1063322 => {
  {  }
  pc = 0x1063323; continue;
 }
 0x1063323 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063324); }
 let mut pc: u32 = 0x1063324;
 loop { match pc {
 0x1063324 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106335f; continue; }
  pc = 0x106332a; continue;
 }
 0x106332a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106334d; continue; }
  pc = 0x1063335; continue;
 }
 0x1063335 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1063353; continue; }
  pc = 0x106334b; continue;
 }
 0x106334b => {
  pc = 0x10633c5; continue;
 }
 0x106334d => {
  { cpu.sub16((cpu.eax as u16), 0xd80u16); }
  if cpu.zf { pc = 0x10633c5; continue; }
  pc = 0x1063353; continue;
 }
 0x1063353 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x10633a1; continue;
 }
 0x106335f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x1063385; continue; }
  pc = 0x106336d; continue;
 }
 0x106336d => {
  { cpu.sub16((cpu.eax as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x1063377; continue; }
  pc = 0x1063373; continue;
 }
 0x1063373 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10633c5; continue;
 }
 0x1063377 => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1152e54u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10633a1; continue;
 }
 0x1063385 => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11757a4u32); cpu.esi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10633a1; continue;
 }
 0x10633a1 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x10633c5; continue; }
  pc = 0x10633a7; continue;
 }
 0x10633a7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10633c5; continue; }
  pc = 0x10633ba; continue;
 }
 0x10633ba => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10633c5; continue; }
  pc = 0x10633c3; continue;
 }
 0x10633c3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10633c5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010633c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10633c7); }
 let mut pc: u32 = 0x10633c7;
 loop { match pc {
 0x10633c7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1063417; continue; }
  pc = 0x10633de; continue;
 }
 0x10633de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x1063403; continue; }
  pc = 0x10633e9; continue;
 }
 0x10633e9 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1063409; continue; }
  pc = 0x10633ff; continue;
 }
 0x10633ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063403 => {
  { cpu.sub16((cpu.eax as u16), 0xd80u16); }
  if cpu.zf { cpu.poll(); pc = 0x10633ff; continue; }
  pc = 0x1063409; continue;
 }
 0x1063409 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063417 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x106343b; continue; }
  pc = 0x1063423; continue;
 }
 0x1063423 => {
  { cpu.sub16((cpu.eax as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x106342d; continue; }
  pc = 0x1063429; continue;
 }
 0x1063429 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106342d => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106343b => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11757a4u32); cpu.esi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063457(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063457); }
 let mut pc: u32 = 0x1063457;
 loop { match pc {
 0x1063457 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x106346d; continue; }
  pc = 0x106345d; continue;
 }
 0x106345d => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x106346d; continue; }
  pc = 0x1063463; continue;
 }
 0x1063463 => {
  { cpu.sub8(cpu.r8(0x66u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10635f2; continue; }
  pc = 0x106346d; continue;
 }
 0x106346d => {
  cpu.push(0x1063472); f_0106b224(cpu); if cpu.returned_to != 0x1063472 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1063540; continue; }
  pc = 0x106347c; continue;
 }
 0x106347c => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x106348c; continue; }
  pc = 0x1063482; continue;
 }
 0x1063482 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1063540; continue; }
  pc = 0x106348c; continue;
 }
 0x106348c => {
  { cpu.pushad(); }
  cpu.push(0x1063492); f_0106a326(cpu); if cpu.returned_to != 0x1063492 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x10c7adbu32; cpu.edi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x106349d; continue;
 }
 0x106349d => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10634a9; continue; }
  pc = 0x10634a3; continue;
 }
 0x10634a3 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106349d; continue; }
  pc = 0x10634a8; continue;
 }
 0x10634a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10634a9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10634bf); f_0100b130(cpu); if cpu.returned_to != 0x10634bf { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  cpu.push(0x10634cf); f_0103b9a2(cpu); if cpu.returned_to != 0x10634cf { return; }
  cpu.push(0x10634d4); f_0106a2d3(cpu); if cpu.returned_to != 0x10634d4 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x106353f; continue; }
  pc = 0x10634e2; continue;
 }
 0x10634e2 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if !cpu.zf { pc = 0x10634f2; continue; }
  pc = 0x10634ec; continue;
 }
 0x10634ec => {
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063534; continue;
 }
 0x10634f2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if !cpu.zf { pc = 0x10634fe; continue; }
  pc = 0x10634f8; continue;
 }
 0x10634f8 => {
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063534; continue;
 }
 0x10634fe => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if !cpu.zf { pc = 0x106350a; continue; }
  pc = 0x1063504; continue;
 }
 0x1063504 => {
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063534; continue;
 }
 0x106350a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1063516; continue; }
  pc = 0x1063510; continue;
 }
 0x1063510 => {
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063534; continue;
 }
 0x1063516 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x1063520; continue; }
  pc = 0x106351c; continue;
 }
 0x106351c => {
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063520; continue;
 }
 0x1063520 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x106352a; continue; }
  pc = 0x1063526; continue;
 }
 0x1063526 => {
  { let v: u8 = 0x3cu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106352a; continue;
 }
 0x106352a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x1063534; continue; }
  pc = 0x1063530; continue;
 }
 0x1063530 => {
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063534; continue;
 }
 0x1063534 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x106353f; continue;
 }
 0x106353f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063540 => {
  cpu.push(0x1063545); f_0106a04e(cpu); if cpu.returned_to != 0x1063545 { return; }
  { let v: u16 = cpu.shr16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 4u32); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1063550; continue; }
  pc = 0x106354c; continue;
 }
 0x106354c => {
  { let v: u16 = cpu.inc16(cpu.r16(0x48u32.wrapping_add(cpu.ebp))); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063550; continue;
 }
 0x1063550 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x106353f; continue; }
  pc = 0x106355e; continue;
 }
 0x106355e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x10635d2; continue; }
  pc = 0x1063564; continue;
 }
 0x1063564 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1063592; continue; }
  pc = 0x106356a; continue;
 }
 0x106356a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x10635b2; continue; }
  pc = 0x1063570; continue;
 }
 0x1063570 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10635c2; continue; }
  pc = 0x1063576; continue;
 }
 0x1063576 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x10635e2; continue; }
  pc = 0x106357c; continue;
 }
 0x106357c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x10635a2; continue; }
  pc = 0x1063582; continue;
 }
 0x1063582 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063592 => {
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635a2 => {
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635b2 => {
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635c2 => {
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635d2 => {
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635e2 => {
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10635f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010639be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10639be); }
 let mut pc: u32 = 0x10639be;
 loop { match pc {
 0x10639be => {
  { let v: u8 = 0xau8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10639cb); f_0106a326(cpu); if cpu.returned_to != 0x10639cb { return; }
  { let v: u8 = 0x8u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010639df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10639df); }
 let mut pc: u32 = 0x10639df;
 loop { match pc {
 0x10639df => {
  { let v: u8 = 0x31u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1063a00); f_01049cd8(cpu); if cpu.returned_to != 0x1063a00 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1063a08); f_0106331c(cpu); if cpu.returned_to != 0x1063a08 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063a09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063a09); }
 let mut pc: u32 = 0x1063a09;
 loop { match pc {
 0x1063a09 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1063a1c; continue;
 }
 0x1063a1c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1063a4e; continue; }
  pc = 0x1063a21; continue;
 }
 0x1063a21 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1063a4e; continue; }
  pc = 0x1063a37; continue;
 }
 0x1063a37 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x7u8); }
  if cpu.zf { pc = 0x1063a43; continue; }
  pc = 0x1063a3d; continue;
 }
 0x1063a3d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1063a4e; continue; }
  pc = 0x1063a43; continue;
 }
 0x1063a43 => {
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063a4e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1063a1c; continue; }
  pc = 0x1063a53; continue;
 }
 0x1063a53 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063a58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063a58); }
 let mut pc: u32 = 0x1063a58;
 loop { match pc {
 0x1063a58 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1063a6b; continue;
 }
 0x1063a6b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1063a91; continue; }
  pc = 0x1063a70; continue;
 }
 0x1063a70 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1063a91; continue; }
  pc = 0x1063a86; continue;
 }
 0x1063a86 => {
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063a91 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1063a6b; continue; }
  pc = 0x1063a96; continue;
 }
 0x1063a96 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063a9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063a9b); }
 let mut pc: u32 = 0x1063a9b;
 loop { match pc {
 0x1063a9b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1063af3; continue; }
  pc = 0x1063aa1; continue;
 }
 0x1063aa1 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x1063ab8; continue;
 }
 0x1063ab8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1063aed; continue; }
  pc = 0x1063abd; continue;
 }
 0x1063abd => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x1063aed; continue; }
  pc = 0x1063ad0; continue;
 }
 0x1063ad0 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1063aed; continue; }
  pc = 0x1063ad6; continue;
 }
 0x1063ad6 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1063ade); f_0106586c(cpu); if cpu.returned_to != 0x1063ade { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063aed; continue;
 }
 0x1063aed => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1063ab8; continue; }
  pc = 0x1063af2; continue;
 }
 0x1063af2 => {
  { cpu.popad(); }
  pc = 0x1063af3; continue;
 }
 0x1063af3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063c10); }
 let mut pc: u32 = 0x1063c10;
 loop { match pc {
 0x1063c10 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x1063c46; continue; }
  pc = 0x1063c18; continue;
 }
 0x1063c18 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xcu8); }
  if cpu.zf { pc = 0x1063c46; continue; }
  pc = 0x1063c1e; continue;
 }
 0x1063c1e => {
  cpu.push(0x1063c23); f_01063324(cpu); if cpu.returned_to != 0x1063c23 { return; }
  if !cpu.cf { pc = 0x1063c46; continue; }
  pc = 0x1063c25; continue;
 }
 0x1063c25 => {
  cpu.push(0x1063c2a); f_01065810(cpu); if cpu.returned_to != 0x1063c2a { return; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1063c48; continue; }
  pc = 0x1063c30; continue;
 }
 0x1063c30 => {
  cpu.push(0x1063c35); f_0106b3b5(cpu); if cpu.returned_to != 0x1063c35 { return; }
  cpu.push(0x1063c3a); f_0106a326(cpu); if cpu.returned_to != 0x1063c3a { return; }
  cpu.push(0x1063c3f); f_010675fd(cpu); if cpu.returned_to != 0x1063c3f { return; }
  cpu.push(0x1063c44); f_01066eaf(cpu); if cpu.returned_to != 0x1063c44 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063c46 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063c48 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1063c25; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063c4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063c4e); }
 let mut pc: u32 = 0x1063c4e;
 loop { match pc {
 0x1063c4e => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x1063d02; continue; }
  pc = 0x1063c5a; continue;
 }
 0x1063c5a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1063d62; continue; }
  pc = 0x1063c64; continue;
 }
 0x1063c64 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x1063d62; continue; }
  pc = 0x1063ca8; continue;
 }
 0x1063ca8 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x1063cbc); f_01018e6e(cpu); if cpu.returned_to != 0x1063cbc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  if !cpu.cf { pc = 0x1063ccf; continue; }
  pc = 0x1063cc5; continue;
 }
 0x1063cc5 => {
  { let v: u32 = 0x10f718eu32; cpu.w32(0x1129904u32, v); }
  pc = 0x1063ccf; continue;
 }
 0x1063ccf => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1063cf4); f_0105c902(cpu); if cpu.returned_to != 0x1063cf4 { return; }
  { let v: u32 = 0x10f714bu32; cpu.w32(0x1129904u32, v); }
  if cpu.cf { pc = 0x1063d62; continue; }
  pc = 0x1063d00; continue;
 }
 0x1063d00 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063d02 => {
  cpu.push(0x1063d07); f_01063324(cpu); if cpu.returned_to != 0x1063d07 { return; }
  if cpu.cf { pc = 0x1063d62; continue; }
  pc = 0x1063d09; continue;
 }
 0x1063d09 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x1063d62; continue; }
  pc = 0x1063d49; continue;
 }
 0x1063d49 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1063d5e); f_0105c902(cpu); if cpu.returned_to != 0x1063d5e { return; }
  if cpu.cf { pc = 0x1063d62; continue; }
  pc = 0x1063d60; continue;
 }
 0x1063d60 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063d62 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063d64); }
 let mut pc: u32 = 0x1063d64;
 loop { match pc {
 0x1063d64 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1063dcd; continue; }
  pc = 0x1063d6a; continue;
 }
 0x1063d6a => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x1063dcd; continue; }
  pc = 0x1063d72; continue;
 }
 0x1063d72 => {
  cpu.push(0x1063d77); f_01063324(cpu); if cpu.returned_to != 0x1063d77 { return; }
  if cpu.cf { pc = 0x1063db4; continue; }
  pc = 0x1063d79; continue;
 }
 0x1063d79 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1063d87); f_01050098(cpu); if cpu.returned_to != 0x1063d87 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1063db4; continue; }
  pc = 0x1063d8a; continue;
 }
 0x1063d8a => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1063db4; continue; }
  pc = 0x1063d97; continue;
 }
 0x1063d97 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1063db4; continue; }
  pc = 0x1063d9c; continue;
 }
 0x1063d9c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1063db2); f_0105ca27(cpu); if cpu.returned_to != 0x1063db2 { return; }
  if !cpu.cf { pc = 0x1063dcd; continue; }
  pc = 0x1063db4; continue;
 }
 0x1063db4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1063dc1); f_0106b3b5(cpu); if cpu.returned_to != 0x1063dc1 { return; }
  cpu.push(0x1063dc6); f_0106a326(cpu); if cpu.returned_to != 0x1063dc6 { return; }
  cpu.push(0x1063dcb); f_010675fd(cpu); if cpu.returned_to != 0x1063dcb { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1063dcd => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063dcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063dcf); }
 let mut pc: u32 = 0x1063dcf;
 loop { match pc {
 0x1063dcf => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x1063de6); f_0108b513(cpu); if cpu.returned_to != 0x1063de6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1063dfd); f_01049cd8(cpu); if cpu.returned_to != 0x1063dfd { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1063e05); f_0106331c(cpu); if cpu.returned_to != 0x1063e05 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063e07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063e07); }
 let mut pc: u32 = 0x1063e07;
 loop { match pc {
 0x1063e07 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x20u8); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x1063fb6; continue; }
  pc = 0x1063e2e; continue;
 }
 0x1063e2e => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1063e44; continue; }
  pc = 0x1063e3a; continue;
 }
 0x1063e3a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063e44; continue;
 }
 0x1063e44 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x1063e56; continue; }
  pc = 0x1063e4a; continue;
 }
 0x1063e4a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1063e56; continue; }
  pc = 0x1063e50; continue;
 }
 0x1063e50 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1063e56; continue;
 }
 0x1063e56 => {
  cpu.push(0x1063e5b); f_0108c181(cpu); if cpu.returned_to != 0x1063e5b { return; }
  cpu.push(0x1063e60); f_01063dcf(cpu); if cpu.returned_to != 0x1063e60 { return; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1063fb6; continue; }
  pc = 0x1063e6a; continue;
 }
 0x1063e6a => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1063fb6; continue; }
  pc = 0x1063e74; continue;
 }
 0x1063e74 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x1063e88); f_01063fb7(cpu); if cpu.returned_to != 0x1063e88 { return; }
  if cpu.cf { pc = 0x1063fb6; continue; }
  pc = 0x1063e8e; continue;
 }
 0x1063e8e => {
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x1063ea6; continue; }
  pc = 0x1063e9e; continue;
 }
 0x1063e9e => {
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063ea6; continue;
 }
 0x1063ea6 => {
  cpu.push(0x1063eab); f_01064020(cpu); if cpu.returned_to != 0x1063eab { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee980u32, v); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x1063eca); f_01064020(cpu); if cpu.returned_to != 0x1063eca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee980u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x1063ee6; continue; }
  pc = 0x1063ed8; continue;
 }
 0x1063ed8 => {
  { let v: u8 = 0x41u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063f54; continue;
 }
 0x1063ee6 => {
  { let v: u8 = 0x5u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x75u32.wrapping_add(cpu.ebp)), 0x5cu8); }
  if (cpu.sf != cpu.of) { pc = 0x1063efc; continue; }
  pc = 0x1063ef0; continue;
 }
 0x1063ef0 => {
  { let v: u8 = 0x39u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063efc; continue;
 }
 0x1063efc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1063f44; continue; }
  pc = 0x1063f02; continue;
 }
 0x1063f02 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1063f34; continue; }
  pc = 0x1063f08; continue;
 }
 0x1063f08 => {
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1063f54; continue; }
  pc = 0x1063f11; continue;
 }
 0x1063f11 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1063f26; continue; }
  pc = 0x1063f17; continue;
 }
 0x1063f17 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1063f26; continue; }
  pc = 0x1063f1d; continue;
 }
 0x1063f1d => {
  { cpu.sub8(cpu.r8(0x10fd2bfu32), 0x0u8); }
  if cpu.zf { pc = 0x1063f54; continue; }
  pc = 0x1063f26; continue;
 }
 0x1063f26 => {
  { let v: u8 = 0x31u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063f54; continue;
 }
 0x1063f34 => {
  { let v: u8 = 0x47u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063f54; continue;
 }
 0x1063f44 => {
  { let v: u8 = 0x13u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10e3ec7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1063f54; continue;
 }
 0x1063f54 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1063f6a); f_01064250(cpu); if cpu.returned_to != 0x1063f6a { return; }
  if cpu.cf { pc = 0x1063f87; continue; }
  pc = 0x1063f6c; continue;
 }
 0x1063f6c => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x1063f87; continue; }
  pc = 0x1063f72; continue;
 }
 0x1063f72 => {
  { let v: u8 = cpu.r8(0x77u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1063f87; continue;
 }
 0x1063f87 => {
  cpu.push(0x1063f8c); f_01053613(cpu); if cpu.returned_to != 0x1063f8c { return; }
  if !cpu.cf { pc = 0x1063f9d; continue; }
  pc = 0x1063f8e; continue;
 }
 0x1063f8e => {
  cpu.push(0x1063f93); f_0106426e(cpu); if cpu.returned_to != 0x1063f93 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1063fa2; continue;
 }
 0x1063f9d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x55u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1063fa2; continue;
 }
 0x1063fa2 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1063fb6; continue; }
  pc = 0x1063fac; continue;
 }
 0x1063fac => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1063fb6; continue;
 }
 0x1063fb6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01063fb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1063fb7); }
 let mut pc: u32 = 0x1063fb7;
 loop { match pc {
 0x1063fb7 => {
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
  if (cpu.sf == cpu.of) { pc = 0x106401d; continue; }
  pc = 0x1063ff2; continue;
 }
 0x1063ff2 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x112aa40u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112aa3cu32, v); }
  { let v: u32 = cpu.shr32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112aa3cu32).wrapping_sub(cpu.edx); cpu.w32(0x112aa3cu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x112aa3cu32)); cpu.w32(0x112aa3cu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106401a); f_01063457(cpu); if cpu.returned_to != 0x106401a { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106401d => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01064020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1064020); }
 let mut pc: u32 = 0x1064020;
 loop { match pc {
 0x1064020 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106402a; continue;
 }
 0x106402a => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106403d; continue;
 }
 0x106403d => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106404a; continue;
 }
 0x106404a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x1064054; continue;
 }
 0x1064054 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x26u8); }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106405e; continue;
 }
 0x106405e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x10641a2; continue; }
  pc = 0x1064068; continue;
 }
 0x1064068 => {
  { cpu.pushad(); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x106407d); f_0108a349(cpu); if cpu.returned_to != 0x106407d { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1064192; continue; }
  pc = 0x1064084; continue;
 }
 0x1064084 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1064194; continue; }
  pc = 0x106408e; continue;
 }
 0x106408e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106409b; continue;
 }
 0x106409b => {
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10640b7; continue; }
  pc = 0x10640a5; continue;
 }
 0x10640a5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x11757b7u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10640b7; continue;
 }
 0x10640b7 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10640cb; continue; }
  pc = 0x10640c0; continue;
 }
 0x10640c0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10640d8; continue; }
  pc = 0x10640cb; continue;
 }
 0x10640cb => {
  { cpu.sub8(cpu.r8(0x11757e1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1064192; continue; }
  pc = 0x10640d8; continue;
 }
 0x10640d8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if !cpu.zf { pc = 0x1064192; continue; }
  pc = 0x10640e2; continue;
 }
 0x10640e2 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x106410e; continue; }
  pc = 0x10640eb; continue;
 }
 0x10640eb => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1064114; continue; }
  pc = 0x10640f5; continue;
 }
 0x10640f5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10640ff); f_010576b2(cpu); if cpu.returned_to != 0x10640ff { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1064192; continue; }
  pc = 0x106410c; continue;
 }
 0x106410c => {
  pc = 0x1064114; continue;
 }
 0x106410e => {
  { cpu.sub8(cpu.r8(0x6cu32.wrapping_add(cpu.esi)), 0x55u8); }
  if cpu.cf { pc = 0x1064192; continue; }
  pc = 0x1064114; continue;
 }
 0x1064114 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x106411e); f_0106a326(cpu); if cpu.returned_to != 0x106411e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x106412c; continue; }
  pc = 0x1064127; continue;
 }
 0x1064127 => {
  cpu.push(0x106412c); f_01065882(cpu); if cpu.returned_to != 0x106412c { return; }
  pc = 0x106412c; continue;
 }
 0x106412c => {
  { let v: u8 = 0xfbu8; cpu.w8(0x5au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x66u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x10ee980u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.esi)) | 0x80u8; cpu.w8(0x67u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1064162; continue; }
  pc = 0x1064150; continue;
 }
 0x1064150 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x1064158); f_0100bd7c(cpu); if cpu.returned_to != 0x1064158 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x1064162; continue; }
  pc = 0x106415b; continue;
 }
 0x106415b => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  pc = 0x1064162; continue;
 }
 0x1064162 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106416a); f_010512c6(cpu); if cpu.returned_to != 0x106416a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064178); f_01072d00(cpu); if cpu.returned_to != 0x1064178 { return; }
  if !cpu.cf { pc = 0x106418b; continue; }
  pc = 0x106417a; continue;
 }
 0x106417a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106418b; continue; }
  pc = 0x1064180; continue;
 }
 0x1064180 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106418b; continue; }
  pc = 0x1064186; continue;
 }
 0x1064186 => {
  cpu.push(0x106418b); f_010360cf(cpu); if cpu.returned_to != 0x106418b { return; }
  pc = 0x106418b; continue;
 }
 0x106418b => {
  { cpu.popad(); }
  cpu.push(0x1064191); f_0108b513(cpu); if cpu.returned_to != 0x1064191 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064192 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064194 => {
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x106419b); f_01065635(cpu); if cpu.returned_to != 0x106419b { return; }
  { cpu.popad(); }
  cpu.push(0x10641a1); f_0108b513(cpu); if cpu.returned_to != 0x10641a1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10641a2 => {
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x10641a9); f_01065635(cpu); if cpu.returned_to != 0x10641a9 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10641b3); f_0108b513(cpu); if cpu.returned_to != 0x10641b3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010641b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10641b4); }
 let mut pc: u32 = 0x10641b4;
 loop { match pc {
 0x10641b4 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10641d2; continue; }
  pc = 0x10641ca; continue;
 }
 0x10641ca => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10641d2; continue; }
  pc = 0x10641ce; continue;
 }
 0x10641ce => {
  { cpu.sub8((cpu.eax as u8), 0xfbu8); }
  if !cpu.zf { pc = 0x106420b; continue; }
  pc = 0x10641d2; continue;
 }
 0x10641d2 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1064201); f_010633c7(cpu); if cpu.returned_to != 0x1064201 { return; }
  if !cpu.cf { pc = 0x1064206; continue; }
  pc = 0x1064203; continue;
 }
 0x1064203 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106420b; continue;
 }
 0x1064206 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.zf { pc = 0x106420e; continue; }
  pc = 0x106420b; continue;
 }
 0x106420b => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106420e => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01064250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1064250); }
 let mut pc: u32 = 0x1064250;
 loop { match pc {
 0x1064250 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x106426c; continue; }
  pc = 0x1064256; continue;
 }
 0x1064256 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106426c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106426e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106426e); }
 let mut pc: u32 = 0x106426e;
 loop { match pc {
 0x106426e => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1064288; continue; }
  pc = 0x1064284; continue;
 }
 0x1064284 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1064288; continue;
 }
 0x1064288 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x10642ee; continue; }
  pc = 0x1064290; continue;
 }
 0x1064290 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x39u8); }
  if cpu.zf { pc = 0x10642ee; continue; }
  pc = 0x1064296; continue;
 }
 0x1064296 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x10642b1; continue; }
  pc = 0x10642a7; continue;
 }
 0x10642a7 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10642b1; continue; }
  pc = 0x10642af; continue;
 }
 0x10642af => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10642b1; continue;
 }
 0x10642b1 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10642bd; continue;
 }
 0x10642bd => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10642c9; continue; }
  pc = 0x10642c6; continue;
 }
 0x10642c6 => {
  { let v: u16 = cpu.shl16((cpu.edi as u16), 1u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x10642c9; continue;
 }
 0x10642c9 => {
  cpu.push(0x10642ce); f_01053764(cpu); if cpu.returned_to != 0x10642ce { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10642d5); f_01049cd8(cpu); if cpu.returned_to != 0x10642d5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10642dd); f_0106331c(cpu); if cpu.returned_to != 0x10642dd { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10642ee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10642f5); f_01050098(cpu); if cpu.returned_to != 0x10642f5 { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 2u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x106432b; continue; }
  pc = 0x1064303; continue;
 }
 0x1064303 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1064316; continue; }
  pc = 0x106430c; continue;
 }
 0x106430c => {
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 2u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1064316; continue; }
  pc = 0x1064314; continue;
 }
 0x1064314 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1064316; continue;
 }
 0x1064316 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x106432b; continue; }
  pc = 0x1064328; continue;
 }
 0x1064328 => {
  { let v: u32 = cpu.shr32(cpu.edi, 2u32); cpu.edi = v; }
  pc = 0x106432b; continue;
 }
 0x106432b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x10642bd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106432f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106432f); }
 let mut pc: u32 = 0x106432f;
 loop { match pc {
 0x106432f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x106438e; continue; }
  pc = 0x1064334; continue;
 }
 0x1064334 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106438f; continue; }
  pc = 0x1064341; continue;
 }
 0x1064341 => {
  if !cpu.zf { pc = 0x106438e; continue; }
  pc = 0x1064343; continue;
 }
 0x1064343 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1064444; continue; }
  pc = 0x106434d; continue;
 }
 0x106434d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106435c); f_01072d00(cpu); if cpu.returned_to != 0x106435c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1064373; continue; }
  pc = 0x1064360; continue;
 }
 0x1064360 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d736u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x1064373; continue;
 }
 0x1064373 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11296b5u32, v); }
  cpu.push(0x106437f); f_01050256(cpu); if cpu.returned_to != 0x106437f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296b5u32, v); }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106438e; continue;
 }
 0x106438e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106438f => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1064433; continue; }
  pc = 0x10643a3; continue;
 }
 0x10643a3 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x1064433; continue; }
  pc = 0x10643ad; continue;
 }
 0x10643ad => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x1064433; continue; }
  pc = 0x10643c9; continue;
 }
 0x10643c9 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106441a; continue; }
  pc = 0x10643cf; continue;
 }
 0x10643cf => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x10643e2); f_010536af(cpu); if cpu.returned_to != 0x10643e2 { return; }
  if cpu.cf { pc = 0x106442e; continue; }
  pc = 0x10643e4; continue;
 }
 0x10643e4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10643ef); f_01018938(cpu); if cpu.returned_to != 0x10643ef { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1064433; continue; }
  pc = 0x10643f3; continue;
 }
 0x10643f3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1064411); f_0105ca27(cpu); if cpu.returned_to != 0x1064411 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x106441a; continue; }
  pc = 0x1064414; continue;
 }
 0x1064414 => {
  cpu.push(0x1064419); f_010647ab(cpu); if cpu.returned_to != 0x1064419 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106441a => {
  cpu.push(0x106441f); f_0106453b(cpu); if cpu.returned_to != 0x106441f { return; }
  if !cpu.cf { pc = 0x1064422; continue; }
  pc = 0x1064421; continue;
 }
 0x1064421 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064422 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  cpu.push(0x106442d); f_010647ca(cpu); if cpu.returned_to != 0x106442d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106442e => {
  cpu.push(0x1064433); f_01065810(cpu); if cpu.returned_to != 0x1064433 { return; }
  pc = 0x1064433; continue;
 }
 0x1064433 => {
  cpu.push(0x1064438); f_010675fd(cpu); if cpu.returned_to != 0x1064438 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1064443; continue; }
  pc = 0x106443e; continue;
 }
 0x106443e => {
  cpu.push(0x1064443); f_01067331(cpu); if cpu.returned_to != 0x1064443 { return; }
  pc = 0x1064443; continue;
 }
 0x1064443 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064444 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106444c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106444c); }
 let mut pc: u32 = 0x106444c;
 loop { match pc {
 0x106444c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x106447f; continue; }
  pc = 0x1064451; continue;
 }
 0x1064451 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1064480; continue; }
  pc = 0x106445e; continue;
 }
 0x106445e => {
  if !cpu.zf { pc = 0x106447f; continue; }
  pc = 0x1064460; continue;
 }
 0x1064460 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1064533; continue; }
  pc = 0x106446a; continue;
 }
 0x106446a => {
  cpu.push(0x106446f); f_01050256(cpu); if cpu.returned_to != 0x106446f { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106447f; continue; }
  pc = 0x106447b; continue;
 }
 0x106447b => {
  { let v: u8 = 0x39u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106447f; continue;
 }
 0x106447f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064480 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1064522; continue; }
  pc = 0x1064490; continue;
 }
 0x1064490 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x1064522; continue; }
  pc = 0x106449a; continue;
 }
 0x106449a => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x1064522; continue; }
  pc = 0x10644b6; continue;
 }
 0x10644b6 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1064509; continue; }
  pc = 0x10644bc; continue;
 }
 0x10644bc => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x10644cf); f_010536af(cpu); if cpu.returned_to != 0x10644cf { return; }
  if cpu.cf { pc = 0x106451d; continue; }
  pc = 0x10644d1; continue;
 }
 0x10644d1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10644dd); f_01018938(cpu); if cpu.returned_to != 0x10644dd { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1064522; continue; }
  pc = 0x10644e2; continue;
 }
 0x10644e2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1064500); f_0105ca27(cpu); if cpu.returned_to != 0x1064500 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1064509; continue; }
  pc = 0x1064503; continue;
 }
 0x1064503 => {
  cpu.push(0x1064508); f_010647ab(cpu); if cpu.returned_to != 0x1064508 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064509 => {
  cpu.push(0x106450e); f_0106453b(cpu); if cpu.returned_to != 0x106450e { return; }
  if !cpu.cf { pc = 0x1064511; continue; }
  pc = 0x1064510; continue;
 }
 0x1064510 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064511 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  cpu.push(0x106451c); f_010647ca(cpu); if cpu.returned_to != 0x106451c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106451d => {
  cpu.push(0x1064522); f_01065810(cpu); if cpu.returned_to != 0x1064522 { return; }
  pc = 0x1064522; continue;
 }
 0x1064522 => {
  cpu.push(0x1064527); f_010675fd(cpu); if cpu.returned_to != 0x1064527 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1064532; continue; }
  pc = 0x106452d; continue;
 }
 0x106452d => {
  cpu.push(0x1064532); f_01067331(cpu); if cpu.returned_to != 0x1064532 { return; }
  pc = 0x1064532; continue;
 }
 0x1064532 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064533 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106453b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106453b); }
 let mut pc: u32 = 0x106453b;
 loop { match pc {
 0x106453b => {
  { cpu.sub8(cpu.r8(0x9au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x106455b; continue; }
  pc = 0x1064544; continue;
 }
 0x1064544 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x1064551); f_010647ab(cpu); if cpu.returned_to != 0x1064551 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x6u8; cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106455b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106455d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106455d); }
 let mut pc: u32 = 0x106455d;
 loop { match pc {
 0x106455d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x106458c; continue; }
  pc = 0x1064562; continue;
 }
 0x1064562 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106458d; continue; }
  pc = 0x106456c; continue;
 }
 0x106456c => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1064640; continue; }
  pc = 0x1064576; continue;
 }
 0x1064576 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x106458c; continue; }
  pc = 0x106457f; continue;
 }
 0x106457f => {
  cpu.push(0x1064584); f_01050307(cpu); if cpu.returned_to != 0x1064584 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106458c; continue;
 }
 0x106458c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106458d => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106462f; continue; }
  pc = 0x106459d; continue;
 }
 0x106459d => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x106462f; continue; }
  pc = 0x10645a7; continue;
 }
 0x10645a7 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x106462f; continue; }
  pc = 0x10645c3; continue;
 }
 0x10645c3 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1064616; continue; }
  pc = 0x10645c9; continue;
 }
 0x10645c9 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x10645dc); f_010536af(cpu); if cpu.returned_to != 0x10645dc { return; }
  if cpu.cf { pc = 0x106462a; continue; }
  pc = 0x10645de; continue;
 }
 0x10645de => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10645ea); f_01018938(cpu); if cpu.returned_to != 0x10645ea { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106462f; continue; }
  pc = 0x10645ef; continue;
 }
 0x10645ef => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x106460d); f_0105ca27(cpu); if cpu.returned_to != 0x106460d { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1064616; continue; }
  pc = 0x1064610; continue;
 }
 0x1064610 => {
  cpu.push(0x1064615); f_010647ab(cpu); if cpu.returned_to != 0x1064615 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064616 => {
  cpu.push(0x106461b); f_0106453b(cpu); if cpu.returned_to != 0x106461b { return; }
  if !cpu.cf { pc = 0x106461e; continue; }
  pc = 0x106461d; continue;
 }
 0x106461d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106461e => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  cpu.push(0x1064629); f_010647ca(cpu); if cpu.returned_to != 0x1064629 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106462a => {
  cpu.push(0x106462f); f_01065810(cpu); if cpu.returned_to != 0x106462f { return; }
  pc = 0x106462f; continue;
 }
 0x106462f => {
  cpu.push(0x1064634); f_010675fd(cpu); if cpu.returned_to != 0x1064634 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106463f; continue; }
  pc = 0x106463a; continue;
 }
 0x106463a => {
  cpu.push(0x106463f); f_01067331(cpu); if cpu.returned_to != 0x106463f { return; }
  pc = 0x106463f; continue;
 }
 0x106463f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064640 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01064645(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1064645); }
 let mut pc: u32 = 0x1064645;
 loop { match pc {
 0x1064645 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x10646b5; continue; }
  pc = 0x106464a; continue;
 }
 0x106464a => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x10646b5; continue; }
  pc = 0x1064657; continue;
 }
 0x1064657 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x106467a; continue; }
  pc = 0x1064659; continue;
 }
 0x1064659 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10646ea; continue; }
  pc = 0x1064663; continue;
 }
 0x1064663 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106466c); f_010503bb(cpu); if cpu.returned_to != 0x106466c { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10646b5; continue;
 }
 0x106467a => {
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp))); cpu.w8(0x9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x9cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x10646a0; continue; }
  pc = 0x1064691; continue;
 }
 0x1064691 => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x106469d; continue; }
  pc = 0x106469b; continue;
 }
 0x106469b => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106469d; continue;
 }
 0x106469d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10646a0; continue;
 }
 0x10646a0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x10646b6; continue; }
  pc = 0x10646aa; continue;
 }
 0x10646aa => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x10646b8; continue; }
  pc = 0x10646b2; continue;
 }
 0x10646b2 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10646b5; continue;
 }
 0x10646b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10646b6 => {
  pc = 0x10646ee; continue;
 }
 0x10646b8 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10646c5); f_010651eb(cpu); if cpu.returned_to != 0x10646c5 { return; }
  if !cpu.cf { pc = 0x10646d5; continue; }
  pc = 0x10646c7; continue;
 }
 0x10646c7 => {
  { cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10646d4; continue; }
  pc = 0x10646d0; continue;
 }
 0x10646d0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10646d4; continue;
 }
 0x10646d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10646d5 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  cpu.push(0x10646df); f_01047379(cpu); if cpu.returned_to != 0x10646df { return; }
  cpu.push(0x10646e4); f_0106a326(cpu); if cpu.returned_to != 0x10646e4 { return; }
  cpu.push(0x10646e9); f_010675fd(cpu); if cpu.returned_to != 0x10646e9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10646ea => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10646ee => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x10647a5; continue; }
  pc = 0x1064702; continue;
 }
 0x1064702 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x10647a5; continue; }
  pc = 0x106470c; continue;
 }
 0x106470c => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16) & 0x303u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10647a5; continue; }
  pc = 0x1064728; continue;
 }
 0x1064728 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106477b; continue; }
  pc = 0x106472e; continue;
 }
 0x106472e => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x1064741); f_010536af(cpu); if cpu.returned_to != 0x1064741 { return; }
  if cpu.cf { pc = 0x10647a5; continue; }
  pc = 0x1064743; continue;
 }
 0x1064743 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106474f); f_01018938(cpu); if cpu.returned_to != 0x106474f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10647a5; continue; }
  pc = 0x1064754; continue;
 }
 0x1064754 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1064772); f_0105ca27(cpu); if cpu.returned_to != 0x1064772 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x106477b; continue; }
  pc = 0x1064775; continue;
 }
 0x1064775 => {
  cpu.push(0x106477a); f_010647ab(cpu); if cpu.returned_to != 0x106477a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106477b => {
  cpu.push(0x1064780); f_0106453b(cpu); if cpu.returned_to != 0x1064780 { return; }
  if !cpu.cf { pc = 0x1064783; continue; }
  pc = 0x1064782; continue;
 }
 0x1064782 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064783 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9cu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1064797; continue; }
  pc = 0x1064792; continue;
 }
 0x1064792 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064797 => {
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10647a0); f_010647ca(cpu); if cpu.returned_to != 0x10647a0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10647a5 => {
  cpu.push(0x10647aa); f_010675fd(cpu); if cpu.returned_to != 0x10647aa { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010647ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10647ab); }
 let mut pc: u32 = 0x10647ab;
 loop { match pc {
 0x10647ab => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10647b8); f_010507b9(cpu); if cpu.returned_to != 0x10647b8 { return; }
  cpu.push(0x10647bd); f_0106a2d3(cpu); if cpu.returned_to != 0x10647bd { return; }
  cpu.push(0x10647c2); f_01069fde(cpu); if cpu.returned_to != 0x10647c2 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010647ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10647ca); }
 let mut pc: u32 = 0x10647ca;
 loop { match pc {
 0x10647ca => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x10647df); f_01064020(cpu); if cpu.returned_to != 0x10647df { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x1064823; continue; }
  pc = 0x10647e5; continue;
 }
 0x10647e5 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1064823; continue; }
  pc = 0x10647eb; continue;
 }
 0x10647eb => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x10647fb); f_0105f215(cpu); if cpu.returned_to != 0x10647fb { return; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  pc = 0x106480d; continue;
 }
 0x106480d => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1064818; continue; }
  pc = 0x1064812; continue;
 }
 0x1064812 => {
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x106481b; continue; }
  pc = 0x1064816; continue;
 }
 0x1064816 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1064818; continue;
 }
 0x1064818 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106480d; continue; }
  pc = 0x106481a; continue;
 }
 0x106481a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106481b => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1064822); f_0105304b(cpu); if cpu.returned_to != 0x1064822 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064823 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1064832; continue; }
  pc = 0x1064829; continue;
 }
 0x1064829 => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1064831); f_0105304b(cpu); if cpu.returned_to != 0x1064831 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064832 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106483c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106483c); } r_0106483c(cpu, 0x106483c); }
pub fn f_0106492e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106492e); } r_0106483c(cpu, 0x106492e); }
pub fn f_01064937(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064937); } r_0106483c(cpu, 0x1064937); }
pub fn f_01064947(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064947); } r_0106483c(cpu, 0x1064947); }
pub fn f_01064950(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064950); } r_0106483c(cpu, 0x1064950); }
pub fn f_01064960(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064960); } r_0106483c(cpu, 0x1064960); }
pub fn f_01064969(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064969); } r_0106483c(cpu, 0x1064969); }
pub fn f_01064979(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064979); } r_0106483c(cpu, 0x1064979); }
pub fn f_01064982(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064982); } r_0106483c(cpu, 0x1064982); }
pub fn f_01064a5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a5b); } r_0106483c(cpu, 0x1064a5b); }
pub fn f_01064a60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a60); } r_0106483c(cpu, 0x1064a60); }
pub fn f_01064a68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a68); } r_0106483c(cpu, 0x1064a68); }
pub fn f_01064a6d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a6d); } r_0106483c(cpu, 0x1064a6d); }
pub fn f_01064a75(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a75); } r_0106483c(cpu, 0x1064a75); }
pub fn f_01064a7a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a7a); } r_0106483c(cpu, 0x1064a7a); }
pub fn f_01064a82(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a82); } r_0106483c(cpu, 0x1064a82); }
pub fn f_01064a87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1064a87); } r_0106483c(cpu, 0x1064a87); }
fn r_0106483c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x106483c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x106499a; continue; }
  pc = 0x1064846; continue;
 }
 0x1064846 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x21u8); }
  if cpu.zf { pc = 0x106499a; continue; }
  pc = 0x1064850; continue;
 }
 0x1064850 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106499a; continue; }
  pc = 0x106485a; continue;
 }
 0x106485a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1064996; continue; }
  pc = 0x1064865; continue;
 }
 0x1064865 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x106489d; continue; }
  pc = 0x106487a; continue;
 }
 0x106487a => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x106489d; continue; }
  pc = 0x106487e; continue;
 }
 0x106487e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1064895; continue; }
  pc = 0x1064884; continue;
 }
 0x1064884 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1064996; continue; }
  pc = 0x1064893; continue;
 }
 0x1064893 => {
  pc = 0x106489d; continue;
 }
 0x1064895 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x1064996; continue; }
  pc = 0x106489d; continue;
 }
 0x106489d => {
  cpu.push(0x10648a2); f_01053613(cpu); if cpu.returned_to != 0x10648a2 { return; }
  if cpu.cf { pc = 0x1064992; continue; }
  pc = 0x10648a8; continue;
 }
 0x10648a8 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x10648cf; continue; }
  pc = 0x10648ca; continue;
 }
 0x10648ca => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x10648cf; continue;
 }
 0x10648cf => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef302u32), 0x0u8); }
  if !cpu.zf { pc = 0x10648f3; continue; }
  pc = 0x10648ec; continue;
 }
 0x10648ec => {
  { let v: u8 = 0x8u8; cpu.w8(0x10ef302u32, v); }
  pc = 0x10648f3; continue;
 }
 0x10648f3 => {
  { let v = cpu.r8(0x10ef302u32); cpu.div8(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1105e64u32, v); }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1064910; continue;
 }
 0x1064910 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e97u32, v); }
  cpu.poll(); pc = cpu.r32(0x112aa44u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x1064922 => {
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x106492a; continue; }
  pc = 0x1064927; continue;
 }
 0x1064927 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1064910; continue; }
  pc = 0x106492a; continue;
 }
 0x106492a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1064992; continue;
 }
 0x106492e => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064937 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064947 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064950 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064960 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064969 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064979 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064982 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e3e97u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1064922; continue;
 }
 0x1064992 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064996 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106499a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1064a91; continue; }
  pc = 0x10649a5; continue;
 }
 0x10649a5 => {
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10649e8; continue; }
  pc = 0x10649ba; continue;
 }
 0x10649ba => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10649e8; continue; }
  pc = 0x10649be; continue;
 }
 0x10649be => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x10649e0; continue; }
  pc = 0x10649c4; continue;
 }
 0x10649c4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1064a91; continue; }
  pc = 0x10649de; continue;
 }
 0x10649de => {
  pc = 0x10649e8; continue;
 }
 0x10649e0 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x1064a91; continue; }
  pc = 0x10649e8; continue;
 }
 0x10649e8 => {
  cpu.push(0x10649ed); f_01053613(cpu); if cpu.returned_to != 0x10649ed { return; }
  if cpu.cf { pc = 0x1064a8d; continue; }
  pc = 0x10649f3; continue;
 }
 0x10649f3 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x1064a1a; continue; }
  pc = 0x1064a15; continue;
 }
 0x1064a15 => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x1064a1a; continue;
 }
 0x1064a1a => {
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef302u32), 0x0u8); }
  if !cpu.zf { pc = 0x1064a3e; continue; }
  pc = 0x1064a37; continue;
 }
 0x1064a37 => {
  { let v: u8 = 0x8u8; cpu.w8(0x10ef302u32, v); }
  pc = 0x1064a3e; continue;
 }
 0x1064a3e => {
  { let v = cpu.r8(0x10ef302u32); cpu.div8(v); }
  { let v = cpu.r8(0x10e3e93u32); cpu.mul8(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x112aa64u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x1064a5b => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a60 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a68 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a6d => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a75 => {
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a7a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a82 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a87 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1064a8d; continue;
 }
 0x1064a8d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064a91 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01064a95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1064a95); }
 let mut pc: u32 = 0x1064a95;
 loop { match pc {
 0x1064a95 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11e2192u32, v); }
  { cpu.sub16((cpu.ecx as u16), 0xfb2eu16); }
  if cpu.zf { pc = 0x1064d21; continue; }
  pc = 0x1064aae; continue;
 }
 0x1064aae => {
  { cpu.sub16((cpu.ecx as u16), 0xef1fu16); }
  if cpu.zf { pc = 0x1064f6d; continue; }
  pc = 0x1064ab9; continue;
 }
 0x1064ab9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x1064acd; continue; }
  pc = 0x1064abf; continue;
 }
 0x1064abf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1064acd; continue; }
  pc = 0x1064ac5; continue;
 }
 0x1064ac5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x1064acd; continue; }
  pc = 0x1064acb; continue;
 }
 0x1064acb => {
  pc = 0x1064ad2; continue;
 }
 0x1064acd => {
  cpu.push(0x1064ad2); f_01047b13(cpu); if cpu.returned_to != 0x1064ad2 { return; }
  pc = 0x1064ad2; continue;
 }
 0x1064ad2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1064aea); f_0104885a(cpu); if cpu.returned_to != 0x1064aea { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1064d1f; continue; }
  pc = 0x1064af5; continue;
 }
 0x1064af5 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xadu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1064b24; continue; }
  pc = 0x1064b02; continue;
 }
 0x1064b02 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064b15); f_01039557(cpu); if cpu.returned_to != 0x1064b15 { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.cf { pc = 0x1064ce4; continue; }
  pc = 0x1064b1f; continue;
 }
 0x1064b1f => {
  pc = 0x1064ccc; continue;
 }
 0x1064b24 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x1064b2e); f_0101acb0(cpu); if cpu.returned_to != 0x1064b2e { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.cf { pc = 0x1064cb4; continue; }
  pc = 0x1064b38; continue;
 }
 0x1064b38 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x1064bc8; continue; }
  pc = 0x1064b42; continue;
 }
 0x1064b42 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if !cpu.zf { pc = 0x1064b76; continue; }
  pc = 0x1064b4b; continue;
 }
 0x1064b4b => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1064b6b); f_0106533e(cpu); if cpu.returned_to != 0x1064b6b { return; }
  if cpu.cf { pc = 0x1064ce4; continue; }
  pc = 0x1064b71; continue;
 }
 0x1064b71 => {
  pc = 0x1064cc0; continue;
 }
 0x1064b76 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064b82); f_01018e6e(cpu); if cpu.returned_to != 0x1064b82 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1064c45; continue; }
  pc = 0x1064b89; continue;
 }
 0x1064b89 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1064cc0; continue; }
  pc = 0x1064bac; continue;
 }
 0x1064bac => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1064ce4; continue; }
  pc = 0x1064bb6; continue;
 }
 0x1064bb6 => {
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x1u8); }
  if !cpu.zf { pc = 0x1064ccc; continue; }
  pc = 0x1064bc3; continue;
 }
 0x1064bc3 => {
  pc = 0x1064ce4; continue;
 }
 0x1064bc8 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1064bfa; continue; }
  pc = 0x1064bce; continue;
 }
 0x1064bce => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064bda); f_01018e6e(cpu); if cpu.returned_to != 0x1064bda { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1064bfa; continue; }
  pc = 0x1064bdd; continue;
 }
 0x1064bdd => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1064bfa; continue; }
  pc = 0x1064be6; continue;
 }
 0x1064be6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.zf { pc = 0x1064c45; continue; }
  pc = 0x1064bfa; continue;
 }
 0x1064bfa => {
  { cpu.pushad(); }
  cpu.push(0x1064c00); f_01018baf(cpu); if cpu.returned_to != 0x1064c00 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1064c10; continue; }
  pc = 0x1064c03; continue;
 }
 0x1064c03 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x1064c45; continue; }
  pc = 0x1064c0b; continue;
 }
 0x1064c0b => {
  pc = 0x1064ce4; continue;
 }
 0x1064c10 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1064c30); f_0106509f(cpu); if cpu.returned_to != 0x1064c30 { return; }
  if cpu.cf { pc = 0x1064ce4; continue; }
  pc = 0x1064c36; continue;
 }
 0x1064c36 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x1064c3f); f_01018f3f(cpu); if cpu.returned_to != 0x1064c3f { return; }
  if cpu.cf { pc = 0x1064cd8; continue; }
  pc = 0x1064c45; continue;
 }
 0x1064c45 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112ad0eu32), 0xffu8); }
  if cpu.zf { pc = 0x1064c96; continue; }
  pc = 0x1064c5d; continue;
 }
 0x1064c5d => {
  { cpu.sub8(cpu.r8(0x112ad0eu32), 0x1u8); }
  if !cpu.zf { pc = 0x1064c80; continue; }
  pc = 0x1064c66; continue;
 }
 0x1064c66 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1064c7d); f_01059da3(cpu); if cpu.returned_to != 0x1064c7d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x1064c96; continue;
 }
 0x1064c80 => {
  { cpu.sub8(cpu.r8(0x112ad0eu32), 0x2u8); }
  if !cpu.zf { pc = 0x1064c8b; continue; }
  pc = 0x1064c89; continue;
 }
 0x1064c89 => {
  pc = 0x1064c96; continue;
 }
 0x1064c8b => {
  { cpu.sub8(cpu.r8(0x112ad0eu32), 0x3u8); }
  if !cpu.zf { pc = 0x1064c96; continue; }
  pc = 0x1064c94; continue;
 }
 0x1064c94 => {
  pc = 0x1064c96; continue;
 }
 0x1064c96 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1064db0; continue; }
  pc = 0x1064ca0; continue;
 }
 0x1064ca0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1064cb1); f_0106a326(cpu); if cpu.returned_to != 0x1064cb1 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064cb4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0eu32, v); }
  cpu.poll(); pc = 0x1064c45; continue;
 }
 0x1064cc0 => {
  { let v: u8 = 0x2u8; cpu.w8(0x112ad0eu32, v); }
  cpu.poll(); pc = 0x1064c45; continue;
 }
 0x1064ccc => {
  { let v: u8 = 0xffu8; cpu.w8(0x112ad0eu32, v); }
  cpu.poll(); pc = 0x1064c45; continue;
 }
 0x1064cd8 => {
  { let v: u8 = 0x3u8; cpu.w8(0x112ad0eu32, v); }
  cpu.poll(); pc = 0x1064c45; continue;
 }
 0x1064ce4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1064cf6; continue; }
  pc = 0x1064cf0; continue;
 }
 0x1064cf0 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1064d1f; continue; }
  pc = 0x1064cf6; continue;
 }
 0x1064cf6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1064d00); f_0104885a(cpu); if cpu.returned_to != 0x1064d00 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1064db0; continue; }
  pc = 0x1064d0b; continue;
 }
 0x1064d0b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1064d1c); f_0106a326(cpu); if cpu.returned_to != 0x1064d1c { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064d1f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064d21 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1064d85; continue; }
  pc = 0x1064d38; continue;
 }
 0x1064d38 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x1064d67; continue; }
  pc = 0x1064d62; continue;
 }
 0x1064d62 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1064d8d; continue;
 }
 0x1064d67 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1064d97; continue; }
  pc = 0x1064d76; continue;
 }
 0x1064d76 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1eu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3cu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1064d84); f_010726f9(cpu); if cpu.returned_to != 0x1064d84 { return; }
  { cpu.popad(); }
  pc = 0x1064d85; continue;
 }
 0x1064d85 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1064d8d); f_01065810(cpu); if cpu.returned_to != 0x1064d8d { return; }
  pc = 0x1064d8d; continue;
 }
 0x1064d8d => {
  cpu.push(0x1064d92); f_0106506b(cpu); if cpu.returned_to != 0x1064d92 { return; }
  pc = 0x1064fe4; continue;
 }
 0x1064d97 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1064d9f); f_010536af(cpu); if cpu.returned_to != 0x1064d9f { return; }
  if cpu.cf { pc = 0x1064dae; continue; }
  pc = 0x1064da1; continue;
 }
 0x1064da1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1064df6; continue; }
  pc = 0x1064da7; continue;
 }
 0x1064da7 => {
  cpu.push(0x1064dac); f_0105ca22(cpu); if cpu.returned_to != 0x1064dac { return; }
  if !cpu.cf { pc = 0x1064df6; continue; }
  pc = 0x1064dae; continue;
 }
 0x1064dae => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1064db0; continue;
 }
 0x1064db0 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1064dc3; continue; }
  pc = 0x1064db6; continue;
 }
 0x1064db6 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1064dc3); f_0106a326(cpu); if cpu.returned_to != 0x1064dc3 { return; }
  pc = 0x1064dc3; continue;
 }
 0x1064dc3 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1064def; continue; }
  pc = 0x1064ddc; continue;
 }
 0x1064ddc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  cpu.push(0x1064ded); f_01047379(cpu); if cpu.returned_to != 0x1064ded { return; }
  pc = 0x1064df4; continue;
 }
 0x1064def => {
  cpu.push(0x1064df4); f_0106b3b5(cpu); if cpu.returned_to != 0x1064df4 { return; }
  pc = 0x1064df4; continue;
 }
 0x1064df4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064df6 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x11e2180u32, v); }
  cpu.push(0x1064e09); f_01065050(cpu); if cpu.returned_to != 0x1064e09 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1065034; continue; }
  pc = 0x1064e13; continue;
 }
 0x1064e13 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1064fe8; continue; }
  pc = 0x1064e1d; continue;
 }
 0x1064e1d => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.r32(0x10e3e93u32); cpu.push(v); }
  cpu.push(0x1064e36); f_010655de(cpu); if cpu.returned_to != 0x1064e36 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3e93u32, v); }
  if cpu.cf { pc = 0x1064e97; continue; }
  pc = 0x1064e3e; continue;
 }
 0x1064e3e => {
  cpu.push(0x1064e43); f_0104885a(cpu); if cpu.returned_to != 0x1064e43 { return; }
  if !cpu.cf { pc = 0x1064e9b; continue; }
  pc = 0x1064e45; continue;
 }
 0x1064e45 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1064e5a; continue; }
  pc = 0x1064e4b; continue;
 }
 0x1064e4b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1064e61; continue; }
  pc = 0x1064e51; continue;
 }
 0x1064e51 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1064e61; continue; }
  pc = 0x1064e5a; continue;
 }
 0x1064e5a => {
  cpu.push(0x1064e5f); f_0105ca22(cpu); if cpu.returned_to != 0x1064e5f { return; }
  if cpu.cf { pc = 0x1064e84; continue; }
  pc = 0x1064e61; continue;
 }
 0x1064e61 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1064e84; continue; }
  pc = 0x1064e67; continue;
 }
 0x1064e67 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1064e84; continue; }
  pc = 0x1064e72; continue;
 }
 0x1064e72 => {
  { cpu.sub8(cpu.r8(0x11e2192u32), 0x0u8); }
  if !cpu.zf { pc = 0x1064e84; continue; }
  pc = 0x1064e7b; continue;
 }
 0x1064e7b => {
  cpu.push(0x1064e80); f_0106506b(cpu); if cpu.returned_to != 0x1064e80 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064e84 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064e92); f_01063457(cpu); if cpu.returned_to != 0x1064e92 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  pc = 0x1064e97; continue;
 }
 0x1064e97 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064e9b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1064ea5); f_0104885a(cpu); if cpu.returned_to != 0x1064ea5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x1064f5d; continue; }
  pc = 0x1064ead; continue;
 }
 0x1064ead => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1064ece; continue; }
  pc = 0x1064eb3; continue;
 }
 0x1064eb3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1064ec2; continue; }
  pc = 0x1064eb9; continue;
 }
 0x1064eb9 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1064ece; continue; }
  pc = 0x1064ec2; continue;
 }
 0x1064ec2 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1064f5d; continue; }
  pc = 0x1064ecc; continue;
 }
 0x1064ecc => {
  pc = 0x1064f30; continue;
 }
 0x1064ece => {
  cpu.push(0x1064ed3); f_0105ca22(cpu); if cpu.returned_to != 0x1064ed3 { return; }
  if cpu.cf { pc = 0x1064ee7; continue; }
  pc = 0x1064ed5; continue;
 }
 0x1064ed5 => {
  { cpu.sub8(cpu.r8(0x9au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x1064f30; continue; }
  pc = 0x1064ede; continue;
 }
 0x1064ede => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1064f5d; continue;
 }
 0x1064ee7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1064dae; continue; }
  pc = 0x1064ef8; continue;
 }
 0x1064ef8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1064f08; continue; }
  pc = 0x1064eff; continue;
 }
 0x1064eff => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1064f13; continue; }
  pc = 0x1064f08; continue;
 }
 0x1064f08 => {
  cpu.push(0x1064f0d); f_0106483c(cpu); if cpu.returned_to != 0x1064f0d { return; }
  if !cpu.cf { pc = 0x1064f13; continue; }
  pc = 0x1064f0f; continue;
 }
 0x1064f0f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064f13 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1064f0f; continue; }
  pc = 0x1064f19; continue;
 }
 0x1064f19 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1064f27); f_01063457(cpu); if cpu.returned_to != 0x1064f27 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064f30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebx)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x1064f44; continue; }
  pc = 0x1064f3e; continue;
 }
 0x1064f3e => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1064f56; continue; }
  pc = 0x1064f44; continue;
 }
 0x1064f44 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1064f55); f_0106a326(cpu); if cpu.returned_to != 0x1064f55 { return; }
  { cpu.popad(); }
  pc = 0x1064f56; continue;
 }
 0x1064f56 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128b00u32, v); }
  pc = 0x1064f5d; continue;
 }
 0x1064f5d => {
  cpu.push(0x1064f62); f_01047a42(cpu); if cpu.returned_to != 0x1064f62 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1064f69); f_0106483c(cpu); if cpu.returned_to != 0x1064f69 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064f6d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
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
  if cpu.zf { pc = 0x1064fd1; continue; }
  pc = 0x1064faa; continue;
 }
 0x1064faa => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1064fd1; continue; }
  pc = 0x1064fb9; continue;
 }
 0x1064fb9 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1eu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3cu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1064fc7); f_010726f9(cpu); if cpu.returned_to != 0x1064fc7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1064fcf); f_0106506b(cpu); if cpu.returned_to != 0x1064fcf { return; }
  pc = 0x1064fe4; continue;
 }
 0x1064fd1 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1064df6; continue; }
  pc = 0x1064fdd; continue;
 }
 0x1064fdd => {
  cpu.push(0x1064fe2); f_01065810(cpu); if cpu.returned_to != 0x1064fe2 { return; }
  pc = 0x106503c; continue;
 }
 0x1064fe4 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1064fe8 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x106500c; continue; }
  pc = 0x1064ff6; continue;
 }
 0x1064ff6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x106500c; continue; }
  pc = 0x1064ffc; continue;
 }
 0x1064ffc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x106500c; continue; }
  pc = 0x1065002; continue;
 }
 0x1065002 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x106500c; continue; }
  pc = 0x1065008; continue;
 }
 0x1065008 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106500c; continue;
 }
 0x106500c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065034 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x106503c; continue;
 }
 0x106503c => {
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
pub fn f_01065050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065050); }
 let mut pc: u32 = 0x1065050;
 loop { match pc {
 0x1065050 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x106505e; continue; }
  pc = 0x106505e; continue;
 }
 0x106505e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x106506a; continue; }
  pc = 0x1065064; continue;
 }
 0x1065064 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x106506a; continue; }
  pc = 0x106506a; continue;
 }
 0x106506a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106506b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106506b); }
 let mut pc: u32 = 0x106506b;
 loop { match pc {
 0x106506b => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x106508f; continue; }
  pc = 0x1065071; continue;
 }
 0x1065071 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106507e); f_0106b3b5(cpu); if cpu.returned_to != 0x106507e { return; }
  cpu.push(0x1065083); f_0106a326(cpu); if cpu.returned_to != 0x1065083 { return; }
  cpu.push(0x1065088); f_010675fd(cpu); if cpu.returned_to != 0x1065088 { return; }
  cpu.push(0x106508d); f_01066eaf(cpu); if cpu.returned_to != 0x106508d { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106508f => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1065098); f_0106a326(cpu); if cpu.returned_to != 0x1065098 { return; }
  cpu.push(0x106509d); f_010675fd(cpu); if cpu.returned_to != 0x106509d { return; }
  cpu.poll(); pc = 0x106506b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106509f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106509f); }
 let mut pc: u32 = 0x106509f;
 loop { match pc {
 0x106509f => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10651b2; continue; }
  pc = 0x10650ca; continue;
 }
 0x10650ca => {
  { cpu.pushad(); }
  cpu.push(0x10650d0); f_0101ac6c(cpu); if cpu.returned_to != 0x10650d0 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x10650d7; continue;
 }
 0x10650d7 => {
  cpu.push(0x10650dc); f_010655b8(cpu); if cpu.returned_to != 0x10650dc { return; }
  if !cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x10650e2; continue;
 }
 0x10650e2 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x10650f1); f_0105f2ec(cpu); if cpu.returned_to != 0x10650f1 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112aa84u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112aa88u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x1065120); f_010655b8(cpu); if cpu.returned_to != 0x1065120 { return; }
  if !cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x1065126; continue;
 }
 0x1065126 => {
  { let v: u32 = 0x112aa84u32; cpu.esi = v; }
  { let v: u8 = 0x9u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x106512d; continue;
 }
 0x106512d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1065163; continue; }
  pc = 0x1065151; continue;
 }
 0x1065151 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1065163; continue; }
  pc = 0x106515a; continue;
 }
 0x106515a => {
  cpu.push(0x106515f); f_010655b8(cpu); if cpu.returned_to != 0x106515f { return; }
  if !cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x1065161; continue;
 }
 0x1065161 => {
  pc = 0x1065187; continue;
 }
 0x1065163 => {
  { let v: u32 = 0x71u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1065178; continue; }
  pc = 0x106516c; continue;
 }
 0x106516c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1065178; continue; }
  pc = 0x1065172; continue;
 }
 0x1065172 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x5u32); cpu.edx = v; }
  pc = 0x1065178; continue;
 }
 0x1065178 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1065180); f_010655b8(cpu); if cpu.returned_to != 0x1065180 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x1065187; continue;
 }
 0x1065187 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106512d; continue; }
  pc = 0x106518e; continue;
 }
 0x106518e => {
  cpu.push(0x1065193); f_010361d8(cpu); if cpu.returned_to != 0x1065193 { return; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x10651ae); f_01039557(cpu); if cpu.returned_to != 0x10651ae { return; }
  if cpu.cf { pc = 0x10651b2; continue; }
  pc = 0x10651b0; continue;
 }
 0x10651b0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10651b2 => {
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11bd2acu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x11bceacu32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x400u32); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x10651e2; continue; }
  pc = 0x10651d5; continue;
 }
 0x10651d5 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  pc = 0x10651e2; continue;
 }
 0x10651e2 => {
  { let v: u32 = cpu.esi; cpu.w32(0x11bd2acu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010651eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10651eb); }
 let mut pc: u32 = 0x10651eb;
 loop { match pc {
 0x10651eb => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1065330; continue; }
  pc = 0x1065216; continue;
 }
 0x1065216 => {
  { cpu.pushad(); }
  cpu.push(0x106521c); f_0101ac6c(cpu); if cpu.returned_to != 0x106521c { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1065330; continue; }
  pc = 0x1065223; continue;
 }
 0x1065223 => {
  { cpu.pushad(); }
  cpu.push(0x1065229); f_0101acb0(cpu); if cpu.returned_to != 0x1065229 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x106532c; continue; }
  pc = 0x1065230; continue;
 }
 0x1065230 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x106523d); f_01018e6e(cpu); if cpu.returned_to != 0x106523d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1065330; continue; }
  pc = 0x106524b; continue;
 }
 0x106524b => {
  { cpu.pushad(); }
  cpu.push(0x1065251); f_01018f3f(cpu); if cpu.returned_to != 0x1065251 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x106532e; continue; }
  pc = 0x1065258; continue;
 }
 0x1065258 => {
  cpu.push(0x106525d); f_01065591(cpu); if cpu.returned_to != 0x106525d { return; }
  if !cpu.cf { pc = 0x1065330; continue; }
  pc = 0x1065263; continue;
 }
 0x1065263 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  cpu.push(0x1065272); f_0105f2ec(cpu); if cpu.returned_to != 0x1065272 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112abecu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112abf0u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x10652a1); f_01065591(cpu); if cpu.returned_to != 0x10652a1 { return; }
  if !cpu.cf { pc = 0x1065330; continue; }
  pc = 0x10652a7; continue;
 }
 0x10652a7 => {
  { let v: u32 = 0x112abecu32; cpu.esi = v; }
  { let v: u8 = 0x24u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10652ae; continue;
 }
 0x10652ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10652e4; continue; }
  pc = 0x10652d2; continue;
 }
 0x10652d2 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10652e4; continue; }
  pc = 0x10652db; continue;
 }
 0x10652db => {
  cpu.push(0x10652e0); f_01065591(cpu); if cpu.returned_to != 0x10652e0 { return; }
  if !cpu.cf { pc = 0x1065330; continue; }
  pc = 0x10652e2; continue;
 }
 0x10652e2 => {
  pc = 0x1065308; continue;
 }
 0x10652e4 => {
  { let v: u32 = 0x71u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10652f9; continue; }
  pc = 0x10652ed; continue;
 }
 0x10652ed => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x10652f9; continue; }
  pc = 0x10652f3; continue;
 }
 0x10652f3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x5u32); cpu.edx = v; }
  pc = 0x10652f9; continue;
 }
 0x10652f9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1065301); f_01065591(cpu); if cpu.returned_to != 0x1065301 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x1065330; continue; }
  pc = 0x1065308; continue;
 }
 0x1065308 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10652ae; continue; }
  pc = 0x106530f; continue;
 }
 0x106530f => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x106532a); f_01039557(cpu); if cpu.returned_to != 0x106532a { return; }
  if cpu.cf { pc = 0x1065330; continue; }
  pc = 0x106532c; continue;
 }
 0x106532c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106532e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065330 => {
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106533e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106533e); }
 let mut pc: u32 = 0x106533e;
 loop { match pc {
 0x106533e => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1065475; continue; }
  pc = 0x1065369; continue;
 }
 0x1065369 => {
  cpu.push(0x106536e); f_010654c6(cpu); if cpu.returned_to != 0x106536e { return; }
  if !cpu.cf { pc = 0x1065475; continue; }
  pc = 0x1065374; continue;
 }
 0x1065374 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  cpu.push(0x1065383); f_0105f2ec(cpu); if cpu.returned_to != 0x1065383 { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112aaccu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112aad0u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x10653a2; continue; }
  pc = 0x106539b; continue;
 }
 0x106539b => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x10653a2; continue; }
  pc = 0x10653a0; continue;
 }
 0x10653a0 => {
  pc = 0x10653c5; continue;
 }
 0x10653a2 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_sub(0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10653bb); f_01050098(cpu); if cpu.returned_to != 0x10653bb { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x10653fb; continue; }
  pc = 0x10653c5; continue;
 }
 0x10653c5 => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  { let v = cpu.r32(0x10e413cu32); cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x10653ed); f_010654c6(cpu); if cpu.returned_to != 0x10653ed { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  if !cpu.cf { pc = 0x1065475; continue; }
  pc = 0x10653fb; continue;
 }
 0x10653fb => {
  { let v: u32 = 0x112aaccu32; cpu.esi = v; }
  { let v: u8 = 0x19u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1065402; continue;
 }
 0x1065402 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e413cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1065438; continue; }
  pc = 0x1065426; continue;
 }
 0x1065426 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1065438; continue; }
  pc = 0x106542f; continue;
 }
 0x106542f => {
  cpu.push(0x1065434); f_010654c6(cpu); if cpu.returned_to != 0x1065434 { return; }
  if !cpu.cf { pc = 0x1065475; continue; }
  pc = 0x1065436; continue;
 }
 0x1065436 => {
  pc = 0x1065460; continue;
 }
 0x1065438 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_sub(0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1065450); f_01050098(cpu); if cpu.returned_to != 0x1065450 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.cf { pc = 0x1065467; continue; }
  pc = 0x1065459; continue;
 }
 0x1065459 => {
  cpu.push(0x106545e); f_010654c6(cpu); if cpu.returned_to != 0x106545e { return; }
  if !cpu.cf { pc = 0x1065475; continue; }
  pc = 0x1065460; continue;
 }
 0x1065460 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1065402; continue; }
  pc = 0x1065467; continue;
 }
 0x1065467 => {
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065475 => {
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065483); }
 let mut pc: u32 = 0x1065483;
 loop { match pc {
 0x1065483 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d6f0u32, v); }
  cpu.push(0x1065493); f_010191d1(cpu); if cpu.returned_to != 0x1065493 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d6f0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x10654c3; continue; }
  pc = 0x106549f; continue;
 }
 0x106549f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c607cu32); }
  if cpu.zf { pc = 0x10654ba; continue; }
  pc = 0x10654a8; continue;
 }
 0x10654a8 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c6095u32); }
  if cpu.zf { pc = 0x10654ba; continue; }
  pc = 0x10654b1; continue;
 }
 0x10654b1 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c6063u32); }
  if !cpu.zf { pc = 0x10654c3; continue; }
  pc = 0x10654ba; continue;
 }
 0x10654ba => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10654c3 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010654c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10654c6); }
 let mut pc: u32 = 0x10654c6;
 loop { match pc {
 0x10654c6 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10654cf); f_010654da(cpu); if cpu.returned_to != 0x10654cf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x10654d7; continue; }
  pc = 0x10654d4; continue;
 }
 0x10654d4 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10654d7 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010654da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10654da); }
 let mut pc: u32 = 0x10654da;
 loop { match pc {
 0x10654da => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110f0b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x110f0b4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f78beu32), 0x0u8); }
  if cpu.zf { pc = 0x1065505; continue; }
  pc = 0x1065500; continue;
 }
 0x1065500 => {
  { let v: u8 = cpu.r8(0x10f78beu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1065505; continue;
 }
 0x1065505 => {
  { let v: u32 = cpu.eax; cpu.w32(0x110f0b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110f0bcu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x106551a; continue;
 }
 0x106551a => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1065588; continue; }
  pc = 0x1065522; continue;
 }
 0x1065522 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x10c607cu32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x1065532; continue;
 }
 0x1065532 => {
  { cpu.sub32(cpu.edi, 0x10c6095u32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x106553a; continue;
 }
 0x106553a => {
  { cpu.sub32(cpu.edi, 0x10c60e0u32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x1065542; continue;
 }
 0x1065542 => {
  { cpu.sub32(cpu.edi, 0x10c60f9u32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x106554a; continue;
 }
 0x106554a => {
  { cpu.sub32(cpu.edi, 0x10c6063u32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x1065552; continue;
 }
 0x1065552 => {
  { cpu.sub32(cpu.edi, 0x10c60aeu32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x106555a; continue;
 }
 0x106555a => {
  { cpu.sub32(cpu.edi, 0x10c60c7u32); }
  if cpu.zf { pc = 0x1065564; continue; }
  pc = 0x1065562; continue;
 }
 0x1065562 => {
  pc = 0x1065588; continue;
 }
 0x1065564 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106557b); f_01019011(cpu); if cpu.returned_to != 0x106557b { return; }
  if !cpu.cf { pc = 0x1065587; continue; }
  pc = 0x106557d; continue;
 }
 0x106557d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065587 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1065588; continue;
 }
 0x1065588 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106551a; continue; }
  pc = 0x106558e; continue;
 }
 0x106558e => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065591(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065591); }
 let mut pc: u32 = 0x1065591;
 loop { match pc {
 0x1065591 => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u8 = 0x8u8; cpu.w8(0x10f78beu32, v); }
  cpu.push(0x106559f); f_010654da(cpu); if cpu.returned_to != 0x106559f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10f78beu32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x10655ac; continue; }
  pc = 0x10655a9; continue;
 }
 0x10655a9 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10655ac => {
  cpu.push(0x10655b1); f_01009e97(cpu); if cpu.returned_to != 0x10655b1 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x10655a9; continue; }
  pc = 0x10655b5; continue;
 }
 0x10655b5 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010655b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10655b8); }
 let mut pc: u32 = 0x10655b8;
 loop { match pc {
 0x10655b8 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x10655cb); f_01009e97(cpu); if cpu.returned_to != 0x10655cb { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10655d4; continue; }
  pc = 0x10655cf; continue;
 }
 0x10655cf => {
  pc = 0x10655d7; continue;
 }
 0x10655d4 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10655d7 => {
  cpu.push(0x10655dc); f_0104885a(cpu); if cpu.returned_to != 0x10655dc { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010655de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10655de); }
 let mut pc: u32 = 0x10655de;
 loop { match pc {
 0x10655de => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1065633; continue; }
  pc = 0x10655e4; continue;
 }
 0x10655e4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10655f3); f_01050098(cpu); if cpu.returned_to != 0x10655f3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1065628; continue; }
  pc = 0x10655f6; continue;
 }
 0x10655f6 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x1065625; continue; }
  pc = 0x10655fd; continue;
 }
 0x10655fd => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1065628; continue; }
  pc = 0x1065608; continue;
 }
 0x1065608 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x11757dfu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee980u32)); }
  if !cpu.zf { pc = 0x1065628; continue; }
  pc = 0x106561e; continue;
 }
 0x106561e => {
  { let v: u8 = 0x1u8; cpu.w8(0x11e2192u32, v); }
  pc = 0x1065625; continue;
 }
 0x1065625 => {
  { cpu.cf = false; }
  pc = 0x1065631; continue;
 }
 0x1065628 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  pc = 0x1065631; continue;
 }
 0x1065631 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065633 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065635(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065635); }
 let mut pc: u32 = 0x1065635;
 loop { match pc {
 0x1065635 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x106564a; continue;
 }
 0x106564a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x1065654; continue;
 }
 0x1065654 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11c6322u32, v); }
  { cpu.sub8((cpu.ebx as u8), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10656b0; continue; }
  pc = 0x1065673; continue;
 }
 0x1065673 => {
  { cpu.sub8(cpu.r8(0x11c6321u32), 0x0u8); }
  if cpu.zf { pc = 0x1065685; continue; }
  pc = 0x106567c; continue;
 }
 0x106567c => {
  { cpu.sub8(cpu.r8(0x11c6322u32), 0x1u8); }
  if cpu.zf { pc = 0x10656b0; continue; }
  pc = 0x1065685; continue;
 }
 0x1065685 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x10656a4); f_0104885a(cpu); if cpu.returned_to != 0x10656a4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106580f; continue; }
  pc = 0x10656b0; continue;
 }
 0x10656b0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10656eb; continue; }
  pc = 0x10656b6; continue;
 }
 0x10656b6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x106580f; continue; }
  pc = 0x10656c0; continue;
 }
 0x10656c0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x10656ca; continue;
 }
 0x10656ca => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x10656d4; continue;
 }
 0x10656d4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10656e0; continue; }
  pc = 0x10656da; continue;
 }
 0x10656da => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x10656eb; continue; }
  pc = 0x10656e0; continue;
 }
 0x10656e0 => {
  cpu.push(0x10656e5); f_0105ca22(cpu); if cpu.returned_to != 0x10656e5 { return; }
  if cpu.cf { pc = 0x106580f; continue; }
  pc = 0x10656eb; continue;
 }
 0x10656eb => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1065701; continue; }
  pc = 0x10656f4; continue;
 }
 0x10656f4 => {
  cpu.push(0x10656f9); f_01047a18(cpu); if cpu.returned_to != 0x10656f9 { return; }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x10656ff; continue;
 }
 0x10656ff => {
  pc = 0x1065711; continue;
 }
 0x1065701 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1065711; continue; }
  pc = 0x1065707; continue;
 }
 0x1065707 => {
  { cpu.sub8(cpu.r8(0x6cu32.wrapping_add(cpu.ebp)), 0x55u8); }
  if cpu.cf { pc = 0x106580f; continue; }
  pc = 0x1065711; continue;
 }
 0x1065711 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x11757e1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x106580f; continue; }
  pc = 0x1065726; continue;
 }
 0x1065726 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x106577f; continue; }
  pc = 0x1065730; continue;
 }
 0x1065730 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106576f; continue; }
  pc = 0x106575d; continue;
 }
 0x106575d => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.edi)), 0xfb2eu16); }
  if !cpu.zf { pc = 0x106576f; continue; }
  pc = 0x1065765; continue;
 }
 0x1065765 => {
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x106580f; continue; }
  pc = 0x106576f; continue;
 }
 0x106576f => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x10657ba; continue;
 }
 0x106577f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0xef1fu16; cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x10657ba; continue;
 }
 0x10657ba => {
  { cpu.pushad(); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x10657e3; continue; }
  pc = 0x10657c1; continue;
 }
 0x10657c1 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10657ce); f_01072d00(cpu); if cpu.returned_to != 0x10657ce { return; }
  if !cpu.cf { pc = 0x10657e3; continue; }
  pc = 0x10657d0; continue;
 }
 0x10657d0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x10657e3; continue; }
  pc = 0x10657d6; continue;
 }
 0x10657d6 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x26u8); }
  if cpu.zf { pc = 0x10657e3; continue; }
  pc = 0x10657dc; continue;
 }
 0x10657dc => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x10657e3); f_010360cf(cpu); if cpu.returned_to != 0x10657e3 { return; }
  pc = 0x10657e3; continue;
 }
 0x10657e3 => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1065807; continue; }
  pc = 0x10657f9; continue;
 }
 0x10657f9 => {
  cpu.push(0x10657fe); f_0100bd7c(cpu); if cpu.returned_to != 0x10657fe { return; }
  if cpu.cf { pc = 0x1065807; continue; }
  pc = 0x1065800; continue;
 }
 0x1065800 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  pc = 0x1065807; continue;
 }
 0x1065807 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106580e); f_010512c6(cpu); if cpu.returned_to != 0x106580e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x106580f; continue;
 }
 0x106580f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065810); }
 let mut pc: u32 = 0x1065810;
 loop { match pc {
 0x1065810 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x42u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x106585e; continue; }
  pc = 0x1065825; continue;
 }
 0x1065825 => {
  { cpu.sub8(cpu.r8(0x43u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.cf { pc = 0x106585e; continue; }
  pc = 0x106582b; continue;
 }
 0x106582b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x43u32.wrapping_add(cpu.ebx))); cpu.w8(0x43u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1065833; continue;
 }
 0x1065833 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1065855; continue; }
  pc = 0x1065838; continue;
 }
 0x1065838 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1065852; continue; }
  pc = 0x1065850; continue;
 }
 0x1065850 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1065852; continue;
 }
 0x1065852 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065855; continue;
 }
 0x1065855 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1065833; continue; }
  pc = 0x106585a; continue;
 }
 0x106585a => {
  { let v: u8 = 0xaau8; cpu.w8(0x42u32.wrapping_add(cpu.ebx), v); }
  pc = 0x106585e; continue;
 }
 0x106585e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065860); }
 let mut pc: u32 = 0x1065860;
 loop { match pc {
 0x1065860 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x106586b; continue; }
  pc = 0x1065866; continue;
 }
 0x1065866 => {
  cpu.push(0x106586b); f_01065882(cpu); if cpu.returned_to != 0x106586b { return; }
  pc = 0x106586b; continue;
 }
 0x106586b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106586c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106586c); }
 let mut pc: u32 = 0x106586c;
 loop { match pc {
 0x106586c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1065881; continue; }
  pc = 0x1065872; continue;
 }
 0x1065872 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x106587c; continue; }
  pc = 0x1065876; continue;
 }
 0x1065876 => {
  { cpu.sub8((cpu.eax as u8), 0xfbu8); }
  if cpu.zf { pc = 0x106587c; continue; }
  pc = 0x106587a; continue;
 }
 0x106587a => {
  pc = 0x1065881; continue;
 }
 0x106587c => {
  cpu.push(0x1065881); f_01065882(cpu); if cpu.returned_to != 0x1065881 { return; }
  pc = 0x1065881; continue;
 }
 0x1065881 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065882(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065882); }
 let mut pc: u32 = 0x1065882;
 loop { match pc {
 0x1065882 => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065884(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065884); }
 let mut pc: u32 = 0x1065884;
 loop { match pc {
 0x1065884 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x10658c5; continue; }
  pc = 0x1065889; continue;
 }
 0x1065889 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10658c6; continue; }
  pc = 0x1065896; continue;
 }
 0x1065896 => {
  if !cpu.zf { pc = 0x10658c5; continue; }
  pc = 0x1065898; continue;
 }
 0x1065898 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10658cc; continue; }
  pc = 0x106589e; continue;
 }
 0x106589e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x10658ab; continue; }
  pc = 0x10658a4; continue;
 }
 0x10658a4 => {
  cpu.push(0x10658a9); f_01065980(cpu); if cpu.returned_to != 0x10658a9 { return; }
  pc = 0x10658bd; continue;
 }
 0x10658ab => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x10658b8; continue; }
  pc = 0x10658b1; continue;
 }
 0x10658b1 => {
  cpu.push(0x10658b6); f_01065abe(cpu); if cpu.returned_to != 0x10658b6 { return; }
  pc = 0x10658bd; continue;
 }
 0x10658b8 => {
  cpu.push(0x10658bd); f_01065910(cpu); if cpu.returned_to != 0x10658bd { return; }
  pc = 0x10658bd; continue;
 }
 0x10658bd => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10658c5; continue;
 }
 0x10658c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10658c6 => {
  cpu.push(0x10658cb); f_010675fd(cpu); if cpu.returned_to != 0x10658cb { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10658cc => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010658d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10658d0); }
 let mut pc: u32 = 0x10658d0;
 loop { match pc {
 0x10658d0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x106590f; continue; }
  pc = 0x10658d5; continue;
 }
 0x10658d5 => {
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10658ea; continue; }
  pc = 0x10658e2; continue;
 }
 0x10658e2 => {
  if cpu.cf { pc = 0x106590f; continue; }
  pc = 0x10658e4; continue;
 }
 0x10658e4 => {
  cpu.push(0x10658e9); f_010675fd(cpu); if cpu.returned_to != 0x10658e9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10658ea => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106590f); f_01052b4d(cpu); if cpu.returned_to != 0x106590f { return; }
  pc = 0x106590f; continue;
 }
 0x106590f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065910); }
 let mut pc: u32 = 0x1065910;
 loop { match pc {
 0x1065910 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x106591d); f_01072d00(cpu); if cpu.returned_to != 0x106591d { return; }
  if cpu.cf { pc = 0x106595c; continue; }
  pc = 0x106591f; continue;
 }
 0x106591f => {
  { cpu.sub8(cpu.r8(0x10f7dc9u32), 0x0u8); }
  if !cpu.zf { pc = 0x106595c; continue; }
  pc = 0x1065928; continue;
 }
 0x1065928 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1065948; continue; }
  pc = 0x106593e; continue;
 }
 0x106593e => {
  cpu.push(0x1065943); f_01012780(cpu); if cpu.returned_to != 0x1065943 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1065948; continue;
 }
 0x1065948 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc9u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x106595b); f_010356fe(cpu); if cpu.returned_to != 0x106595b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106595c; continue;
 }
 0x106595c => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x106596e); f_01053764(cpu); if cpu.returned_to != 0x106596e { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  cpu.push(0x106597e); f_0104a611(cpu); if cpu.returned_to != 0x106597e { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065980); }
 let mut pc: u32 = 0x1065980;
 loop { match pc {
 0x1065980 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x106598d); f_01072d00(cpu); if cpu.returned_to != 0x106598d { return; }
  if cpu.cf { pc = 0x10659c0; continue; }
  pc = 0x106598f; continue;
 }
 0x106598f => {
  { cpu.sub8(cpu.r8(0x10f7dc9u32), 0x0u8); }
  if !cpu.zf { pc = 0x10659c0; continue; }
  pc = 0x1065998; continue;
 }
 0x1065998 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10659b8; continue; }
  pc = 0x10659ae; continue;
 }
 0x10659ae => {
  cpu.push(0x10659b3); f_01012780(cpu); if cpu.returned_to != 0x10659b3 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x10659b8; continue;
 }
 0x10659b8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc9u32, v); }
  pc = 0x10659c0; continue;
 }
 0x10659c0 => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x10659da); f_01053764(cpu); if cpu.returned_to != 0x10659da { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  cpu.push(0x10659ea); f_010503bb(cpu); if cpu.returned_to != 0x10659ea { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010659ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10659ec); }
 let mut pc: u32 = 0x10659ec;
 loop { match pc {
 0x10659ec => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1065a29; continue; }
  pc = 0x10659f1; continue;
 }
 0x10659f1 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1065a19; continue; }
  pc = 0x10659fa; continue;
 }
 0x10659fa => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1065a1f; continue; }
  pc = 0x10659fc; continue;
 }
 0x10659fc => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1065a2a; continue; }
  pc = 0x1065a06; continue;
 }
 0x1065a06 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1065a0f); f_01065b38(cpu); if cpu.returned_to != 0x1065a0f { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065a29; continue;
 }
 0x1065a19 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065a29; continue;
 }
 0x1065a1f => {
  cpu.push(0x1065a24); f_0106b659(cpu); if cpu.returned_to != 0x1065a24 { return; }
  cpu.push(0x1065a29); f_010675fd(cpu); if cpu.returned_to != 0x1065a29 { return; }
  pc = 0x1065a29; continue;
 }
 0x1065a29 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065a2a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065a2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065a2e); }
 let mut pc: u32 = 0x1065a2e;
 loop { match pc {
 0x1065a2e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1065a71; continue; }
  pc = 0x1065a33; continue;
 }
 0x1065a33 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1065a61; continue; }
  pc = 0x1065a3c; continue;
 }
 0x1065a3c => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1065a67; continue; }
  pc = 0x1065a3e; continue;
 }
 0x1065a3e => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1065a72; continue; }
  pc = 0x1065a48; continue;
 }
 0x1065a48 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1065a57); f_01065980(cpu); if cpu.returned_to != 0x1065a57 { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065a71; continue;
 }
 0x1065a61 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065a71; continue;
 }
 0x1065a67 => {
  cpu.push(0x1065a6c); f_0106b659(cpu); if cpu.returned_to != 0x1065a6c { return; }
  cpu.push(0x1065a71); f_010675fd(cpu); if cpu.returned_to != 0x1065a71 { return; }
  pc = 0x1065a71; continue;
 }
 0x1065a71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065a72 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065a76); }
 let mut pc: u32 = 0x1065a76;
 loop { match pc {
 0x1065a76 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1065ab9; continue; }
  pc = 0x1065a7b; continue;
 }
 0x1065a7b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1065aa9; continue; }
  pc = 0x1065a84; continue;
 }
 0x1065a84 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1065aaf; continue; }
  pc = 0x1065a86; continue;
 }
 0x1065a86 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1065aba; continue; }
  pc = 0x1065a90; continue;
 }
 0x1065a90 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1065a9f); f_01065abe(cpu); if cpu.returned_to != 0x1065a9f { return; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065ab9; continue;
 }
 0x1065aa9 => {
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1065ab9; continue;
 }
 0x1065aaf => {
  cpu.push(0x1065ab4); f_0106b659(cpu); if cpu.returned_to != 0x1065ab4 { return; }
  cpu.push(0x1065ab9); f_010675fd(cpu); if cpu.returned_to != 0x1065ab9 { return; }
  pc = 0x1065ab9; continue;
 }
 0x1065ab9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065aba => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065abe); }
 let mut pc: u32 = 0x1065abe;
 loop { match pc {
 0x1065abe => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  cpu.push(0x1065acb); f_01072d00(cpu); if cpu.returned_to != 0x1065acb { return; }
  if cpu.cf { pc = 0x1065aee; continue; }
  pc = 0x1065acd; continue;
 }
 0x1065acd => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1065aed; continue; }
  pc = 0x1065ae3; continue;
 }
 0x1065ae3 => {
  cpu.push(0x1065ae8); f_01012780(cpu); if cpu.returned_to != 0x1065ae8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1065aed; continue;
 }
 0x1065aed => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1065aee; continue;
 }
 0x1065aee => {
  { let v: u32 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x1065b00); f_01053764(cpu); if cpu.returned_to != 0x1065b00 { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1128d44u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128d9du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296b5u32, v); }
  cpu.push(0x1065b28); f_0104a611(cpu); if cpu.returned_to != 0x1065b28 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296b5u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1128d9du32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065b38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065b38); }
 let mut pc: u32 = 0x1065b38;
 loop { match pc {
 0x1065b38 => {
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112ad10u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x112ad12u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0x1du8; cpu.w8(0x1128d9fu32, v); }
  cpu.push(0x1065b86); f_01049561(cpu); if cpu.returned_to != 0x1065b86 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065b90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065b90); }
 let mut pc: u32 = 0x1065b90;
 loop { match pc {
 0x1065b90 => {
  { let v: u32 = 0x10fd567u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1065b9a; continue;
 }
 0x1065b9a => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1065b9a; continue; }
  pc = 0x1065ba0; continue;
 }
 0x1065ba0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065ba1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065ba1); }
 let mut pc: u32 = 0x1065ba1;
 loop { match pc {
 0x1065ba1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10fd657u32, v); }
  { let v: u32 = 0x10fd567u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1065bb4; continue;
 }
 0x1065bb4 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1065bc2; continue; }
  pc = 0x1065bb9; continue;
 }
 0x1065bb9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10fd657u32)); cpu.w16(0x10fd657u32, v); }
  pc = 0x1065bc2; continue;
 }
 0x1065bc2 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1065bb4; continue; }
  pc = 0x1065bc5; continue;
 }
 0x1065bc5 => {
  { let v: u16 = cpu.r16(0x10fd657u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10fd659u32), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1065bda; continue; }
  pc = 0x1065bd4; continue;
 }
 0x1065bd4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10fd659u32, v); }
  pc = 0x1065bda; continue;
 }
 0x1065bda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065bdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065bdb); }
 let mut pc: u32 = 0x1065bdb;
 loop { match pc {
 0x1065bdb => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x12u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x1065c97; continue; }
  pc = 0x1065be4; continue;
 }
 0x1065be4 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x11e219cu32, v); }
  pc = 0x1065bfb; continue;
 }
 0x1065bfb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1065c02); f_010664e4(cpu); if cpu.returned_to != 0x1065c02 { return; }
  if !cpu.cf { pc = 0x1065c0b; continue; }
  pc = 0x1065c04; continue;
 }
 0x1065c04 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1065c97; continue;
 }
 0x1065c0b => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1065c12); f_0106a326(cpu); if cpu.returned_to != 0x1065c12 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb4u32, v); }
  cpu.push(0x1065c22); f_01065d85(cpu); if cpu.returned_to != 0x1065c22 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11e219cu32), 0xffu8); }
  if cpu.zf { pc = 0x1065c4b; continue; }
  pc = 0x1065c2f; continue;
 }
 0x1065c2f => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x11e219cu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1065c45); f_01066588(cpu); if cpu.returned_to != 0x1065c45 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x1065c84; continue;
 }
 0x1065c4b => {
  { let v: u8 = 0x1u8; cpu.w8(0x112ad30u32, v); }
  cpu.push(0x1065c57); f_01066278(cpu); if cpu.returned_to != 0x1065c57 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad30u32, v); }
  cpu.push(0x1065c63); f_0108bfab(cpu); if cpu.returned_to != 0x1065c63 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1065c95; continue; }
  pc = 0x1065c6b; continue;
 }
 0x1065c6b => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11e219cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11e2194u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x1065c84; continue;
 }
 0x1065c84 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1065bfb; continue; }
  pc = 0x1065c8e; continue;
 }
 0x1065c8e => {
  { let v: u32 = cpu.r32(0x11e2194u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065c95 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1065c97; continue;
 }
 0x1065c97 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065c99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065c99); }
 let mut pc: u32 = 0x1065c99;
 loop { match pc {
 0x1065c99 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1065d2e; continue; }
  pc = 0x1065ca3; continue;
 }
 0x1065ca3 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11e219cu32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1065cb1); f_0106a326(cpu); if cpu.returned_to != 0x1065cb1 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb4u32, v); }
  cpu.push(0x1065cc1); f_01065d85(cpu); if cpu.returned_to != 0x1065cc1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11e219cu32), 0xffu8); }
  if cpu.zf { pc = 0x1065cea; continue; }
  pc = 0x1065cce; continue;
 }
 0x1065cce => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x11e219cu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1065ce4); f_01066588(cpu); if cpu.returned_to != 0x1065ce4 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x1065d23; continue;
 }
 0x1065cea => {
  { let v: u8 = 0x1u8; cpu.w8(0x112ad30u32, v); }
  cpu.push(0x1065cf6); f_01066278(cpu); if cpu.returned_to != 0x1065cf6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad30u32, v); }
  cpu.push(0x1065d02); f_0108bfab(cpu); if cpu.returned_to != 0x1065d02 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1065d2c; continue; }
  pc = 0x1065d0a; continue;
 }
 0x1065d0a => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11e219cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11e2194u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x1065d23; continue;
 }
 0x1065d23 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11e2194u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065d2c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1065d2e; continue;
 }
 0x1065d2e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065d30); }
 let mut pc: u32 = 0x1065d30;
 loop { match pc {
 0x1065d30 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1065d84; continue; }
  pc = 0x1065d36; continue;
 }
 0x1065d36 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  pc = 0x1065d41; continue;
 }
 0x1065d41 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1065d7f; continue; }
  pc = 0x1065d46; continue;
 }
 0x1065d46 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x1065d5b); f_010675fd(cpu); if cpu.returned_to != 0x1065d5b { return; }
  cpu.push(0x1065d60); f_0105f215(cpu); if cpu.returned_to != 0x1065d60 { return; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1065d6f); f_0105f215(cpu); if cpu.returned_to != 0x1065d6f { return; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1065d7c); f_0106a04e(cpu); if cpu.returned_to != 0x1065d7c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1065d7f; continue;
 }
 0x1065d7f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1065d41; continue; }
  pc = 0x1065d84; continue;
 }
 0x1065d84 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01065d85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1065d85); }
 let mut pc: u32 = 0x1065d85;
 loop { match pc {
 0x1065d85 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5b1du32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1065dbb; continue; }
  pc = 0x1065da6; continue;
 }
 0x1065da6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  pc = 0x1065dbb; continue;
 }
 0x1065dbb => {
  cpu.push(0x1065dc0); f_010660f9(cpu); if cpu.returned_to != 0x1065dc0 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1065dc8); f_010512c6(cpu); if cpu.returned_to != 0x1065dc8 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1065e26; continue; }
  pc = 0x1065dd3; continue;
 }
 0x1065dd3 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1065dea; continue; }
  pc = 0x1065de8; continue;
 }
 0x1065de8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1065dea; continue;
 }
 0x1065dea => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10fd65bu32, v); }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x12u32.wrapping_add(cpu.ebx))); cpu.w8(0x12u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x1065e2c; continue; }
  pc = 0x1065e11; continue;
 }
 0x1065e11 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1065e27; continue; }
  pc = 0x1065e15; continue;
 }
 0x1065e15 => {
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1065e21); f_01066143(cpu); if cpu.returned_to != 0x1065e21 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x1065e2c; continue; }
  pc = 0x1065e26; continue;
 }
 0x1065e26 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065e27 => {
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1065e2c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd65bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc8u8; cpu.w8(0x10fd567u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x10fd65bu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x116e734u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1065e69; continue; }
  pc = 0x1065e5b; continue;
 }
 0x1065e5b => {
  cpu.push(0x1065e60); f_0100bd7c(cpu); if cpu.returned_to != 0x1065e60 { return; }
  if !cpu.cf { pc = 0x1065e69; continue; }
  pc = 0x1065e62; continue;
 }
 0x1065e62 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  pc = 0x1065e69; continue;
 }
 0x1065e69 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10fcc0eu32)); }
  if !cpu.zf { pc = 0x1065e85; continue; }
  pc = 0x1065e71; continue;
 }
 0x1065e71 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc0eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  pc = 0x1065e85; continue;
 }
 0x1065e85 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee934u32.wrapping_add(cpu.eax))); cpu.w8(0x10ee934u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1065ea8; continue; }
  pc = 0x1065e9b; continue;
 }
 0x1065e9b => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1065ea8; continue; }
  pc = 0x1065e9f; continue;
 }
 0x1065e9f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1065ea6); f_01086779(cpu); if cpu.returned_to != 0x1065ea6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1065ea8; continue;
 }
 0x1065ea8 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x10e4174u32)); }
  if !cpu.zf { pc = 0x1065eba; continue; }
  pc = 0x1065eb0; continue;
 }
 0x1065eb0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  pc = 0x1065eba; continue;
 }
 0x1065eba => {
  { cpu.sub32(cpu.r32(0x11e2198u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1065f1c; continue; }
  pc = 0x1065ec7; continue;
 }
 0x1065ec7 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1065eef; continue; }
  pc = 0x1065ed7; continue;
 }
 0x1065ed7 => {
  { let v: u32 = cpu.r32(0x11e2198u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1065eef; continue; }
  pc = 0x1065eea; continue;
 }
 0x1065eea => {
  cpu.push(0x1065eef); f_0103602f(cpu); if cpu.returned_to != 0x1065eef { return; }
  pc = 0x1065eef; continue;
 }
 0x1065eef => {
  { cpu.popad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11e2198u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x1065f11; continue; }
  pc = 0x1065efc; continue;
 }
 0x1065efc => {
  { cpu.sub32(cpu.ebp, 0x1152e54u32); }
  if (cpu.sf != cpu.of) { pc = 0x1065f11; continue; }
  pc = 0x1065f04; continue;
 }
 0x1065f04 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1065f11; continue; }
  pc = 0x1065f0c; continue;
 }
 0x1065f0c => {
  cpu.push(0x1065f11); f_01065810(cpu); if cpu.returned_to != 0x1065f11 { return; }
  pc = 0x1065f11; continue;
 }
 0x1065f11 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11e2198u32, v); }
  pc = 0x1065f1c; continue;
 }
 0x1065f1c => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x10fd65bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1065f2c); f_0105dae1(cpu); if cpu.returned_to != 0x1065f2c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1066029; continue; }
  pc = 0x1065f34; continue;
 }
 0x1065f34 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1065f3a); f_0105db89(cpu); if cpu.returned_to != 0x1065f3a { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3ce1u32)); }
  if !cpu.zf { pc = 0x1066029; continue; }
  pc = 0x1065f4e; continue;
 }
 0x1065f4e => {
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1065f6a; continue; }
  pc = 0x1065f55; continue;
 }
 0x1065f55 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1065ffc; continue;
 }
 0x1065f6a => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x10fd4c1u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10fd4c2u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x1065f84; continue;
 }
 0x1065f84 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1065fed; continue; }
  pc = 0x1065f8d; continue;
 }
 0x1065f8d => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1065fed; continue; }
  pc = 0x1065fa0; continue;
 }
 0x1065fa0 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1152e54u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x10fd4c2u32)); }
  if !cpu.cf { pc = 0x1065fed; continue; }
  pc = 0x1065fe1; continue;
 }
 0x1065fe1 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10fd4c2u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10fd4c1u32, v); }
  pc = 0x1065fed; continue;
 }
 0x1065fed => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1065f84; continue; }
  pc = 0x1065ff2; continue;
 }
 0x1065ff2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10fd4c1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1066029; continue; }
  pc = 0x1065ffc; continue;
 }
 0x1065ffc => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4c0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x106600d); f_01059200(cpu); if cpu.returned_to != 0x106600d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4c0u32, v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  cpu.push(0x1066029); f_0105d967(cpu); if cpu.returned_to != 0x1066029 { return; }
  pc = 0x1066029; continue;
 }
 0x1066029 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3ce1u32)); }
  if !cpu.zf { pc = 0x106603d; continue; }
  pc = 0x1066038; continue;
 }
 0x1066038 => {
  cpu.push(0x106603d); f_01059a31(cpu); if cpu.returned_to != 0x106603d { return; }
  pc = 0x106603d; continue;
 }
 0x106603d => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10fd65bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x10e40ecu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11c5bb4u32), 0x0u8); }
  if cpu.zf { pc = 0x10660f6; continue; }
  pc = 0x106606c; continue;
 }
 0x106606c => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10660b9; continue; }
  pc = 0x106607a; continue;
 }
 0x106607a => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10c6a4eu32)); cpu.w16(0x10c6a4eu32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10fd465u32)); cpu.w16(0x10fd465u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x10660a7; continue; }
  pc = 0x1066096; continue;
 }
 0x1066096 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x61a8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10660a7; continue; }
  pc = 0x10660a0; continue;
 }
 0x10660a0 => {
  cpu.push(0x10660a5); f_01087028(cpu); if cpu.returned_to != 0x10660a5 { return; }
  pc = 0x10660af; continue;
 }
 0x10660a7 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x10660af; continue;
 }
 0x10660af => {
  { let v: u32 = cpu.add32(cpu.r32(0x10c6a54u32), cpu.eax); cpu.w32(0x10c6a54u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10660f6; continue;
 }
 0x10660b9 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10c6a4cu32)); cpu.w16(0x10c6a4cu32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10fd45fu32)); cpu.w16(0x10fd45fu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x10660e6; continue; }
  pc = 0x10660d5; continue;
 }
 0x10660d5 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x61a8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10660e6; continue; }
  pc = 0x10660df; continue;
 }
 0x10660df => {
  cpu.push(0x10660e4); f_01087028(cpu); if cpu.returned_to != 0x10660e4 { return; }
  pc = 0x10660ee; continue;
 }
 0x10660e6 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x10660ee; continue;
 }
 0x10660ee => {
  { let v: u32 = cpu.add32(cpu.r32(0x10c6a50u32), cpu.eax); cpu.w32(0x10c6a50u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10660f6; continue;
 }
 0x10660f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010660f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10660f9); }
 let mut pc: u32 = 0x10660f9;
 loop { match pc {
 0x10660f9 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11e2198u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1066118; continue;
 }
 0x1066118 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x106613d; continue; }
  pc = 0x106611d; continue;
 }
 0x106611d => {
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x106613c; continue; }
  pc = 0x1066128; continue;
 }
 0x1066128 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x11e2198u32, v); }
  pc = 0x106613c; continue;
 }
 0x106613c => {
  { cpu.popad(); }
  pc = 0x106613d; continue;
 }
 0x106613d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066118; continue; }
  pc = 0x1066142; continue;
 }
 0x1066142 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066143); }
 let mut pc: u32 = 0x1066143;
 loop { match pc {
 0x1066143 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  if cpu.zf { pc = 0x10661e1; continue; }
  pc = 0x1066157; continue;
 }
 0x1066157 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1066166); f_01066213(cpu); if cpu.returned_to != 0x1066166 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x1066199); f_0106a326(cpu); if cpu.returned_to != 0x1066199 { return; }
  { cpu.popad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10661a6); f_0100b130(cpu); if cpu.returned_to != 0x10661a6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x10661be; continue; }
  pc = 0x10661ad; continue;
 }
 0x10661ad => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x10661b9; continue; }
  pc = 0x10661b2; continue;
 }
 0x10661b2 => {
  { cpu.sub8((cpu.ecx as u8), 0xfbu8); }
  if cpu.zf { pc = 0x10661b9; continue; }
  pc = 0x10661b7; continue;
 }
 0x10661b7 => {
  pc = 0x10661be; continue;
 }
 0x10661b9 => {
  cpu.push(0x10661be); f_01065882(cpu); if cpu.returned_to != 0x10661be { return; }
  pc = 0x10661be; continue;
 }
 0x10661be => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if !cpu.zf { pc = 0x10661dd; continue; }
  pc = 0x10661d8; continue;
 }
 0x10661d8 => {
  cpu.push(0x10661dd); f_01066e31(cpu); if cpu.returned_to != 0x10661dd { return; }
  pc = 0x10661dd; continue;
 }
 0x10661dd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10661e1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebx), v); }
  { cpu.pushad(); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3ce1u32)); }
  if !cpu.zf { pc = 0x1066209; continue; }
  pc = 0x1066204; continue;
 }
 0x1066204 => {
  cpu.push(0x1066209); f_01057945(cpu); if cpu.returned_to != 0x1066209 { return; }
  pc = 0x1066209; continue;
 }
 0x1066209 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1066211); f_0106a326(cpu); if cpu.returned_to != 0x1066211 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066213); }
 let mut pc: u32 = 0x1066213;
 loop { match pc {
 0x1066213 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3ce1u32)); }
  if !cpu.zf { pc = 0x1066235; continue; }
  pc = 0x1066225; continue;
 }
 0x1066225 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce1u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3debu32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x1066235; continue;
 }
 0x1066235 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3de9u32)); }
  if !cpu.zf { pc = 0x1066248; continue; }
  pc = 0x106623e; continue;
 }
 0x106623e => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3de9u32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x1066248; continue;
 }
 0x1066248 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10e4174u32), cpu.eax); }
  if !cpu.zf { pc = 0x1066274; continue; }
  pc = 0x1066262; continue;
 }
 0x1066262 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4174u32, v); }
  pc = 0x1066274; continue;
 }
 0x1066274 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066278(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1066278); } r_01066278(cpu, 0x1066278); }
pub fn f_010662d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10662d7); } r_01066278(cpu, 0x10662d7); }
fn r_01066278(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1066278 => {
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1066295; continue; }
  pc = 0x1066281; continue;
 }
 0x1066281 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  pc = 0x1066295; continue;
 }
 0x1066295 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10662ac; continue; }
  pc = 0x106629e; continue;
 }
 0x106629e => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10662d7; continue;
 }
 0x10662ac => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x106629e; continue; }
  pc = 0x10662b5; continue;
 }
 0x10662b5 => {
  { let v: u8 = 0x78u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  cpu.push(0x10662cf); f_010662d7(cpu); if cpu.returned_to != 0x10662cf { return; }
  if !cpu.cf { pc = 0x106636d; continue; }
  pc = 0x10662d5; continue;
 }
 0x10662d5 => {
  cpu.poll(); pc = 0x106629e; continue;
 }
 0x10662d7 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10662f4; continue; }
  pc = 0x10662dd; continue;
 }
 0x10662dd => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x10662f4; continue; }
  pc = 0x10662e3; continue;
 }
 0x10662e3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10fd567u32.wrapping_add(cpu.ebx)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x106631f; continue; }
  pc = 0x10662f4; continue;
 }
 0x10662f4 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10662d7; continue; }
  pc = 0x10662fb; continue;
 }
 0x10662fb => {
  {  }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1066308; continue;
 }
 0x1066308 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1066316; continue; }
  pc = 0x106630e; continue;
 }
 0x106630e => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x1066316; continue; }
  pc = 0x1066314; continue;
 }
 0x1066314 => {
  pc = 0x106631f; continue;
 }
 0x1066316 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066308; continue; }
  pc = 0x106631d; continue;
 }
 0x106631d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106631f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee934u32.wrapping_add(cpu.eax))); cpu.w8(0x10ee934u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e3f7cu32, v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1066346); f_01066588(cpu); if cpu.returned_to != 0x1066346 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x106634e); f_0108bfab(cpu); if cpu.returned_to != 0x106634e { return; }
  { cpu.popad(); }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x106636d; continue; }
  pc = 0x106635c; continue;
 }
 0x106635c => {
  { cpu.sub8(cpu.r8(0x112ad30u32), 0x0u8); }
  if cpu.zf { pc = 0x106636f; continue; }
  pc = 0x1066365; continue;
 }
 0x1066365 => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x106636d; continue;
 }
 0x106636d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106636f => {
  { let v: u32 = 0x1179b24u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1066379; continue;
 }
 0x1066379 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1066386; continue; }
  pc = 0x106637e; continue;
 }
 0x106637e => {
  { let v: u32 = cpu.esi.wrapping_add(0x52u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066379; continue; }
  pc = 0x1066383; continue;
 }
 0x1066383 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x52u32); cpu.esi = v; }
  pc = 0x1066386; continue;
 }
 0x1066386 => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10663b2; continue; }
  pc = 0x10663a0; continue;
 }
 0x10663a0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10663b2; continue; }
  pc = 0x10663a6; continue;
 }
 0x10663a6 => {
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112ae6eu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x10663bc; continue;
 }
 0x10663b2 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112ad36u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x10663bc; continue;
 }
 0x10663bc => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010663c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10663c7); }
 let mut pc: u32 = 0x10663c7;
 loop { match pc {
 0x10663c7 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10663e4; continue; }
  pc = 0x10663ce; continue;
 }
 0x10663ce => {
  cpu.push(0x10663d3); f_0106a269(cpu); if cpu.returned_to != 0x10663d3 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x10663e4; continue; }
  pc = 0x10663d8; continue;
 }
 0x10663d8 => {
  cpu.push(0x10663dd); f_0106a255(cpu); if cpu.returned_to != 0x10663dd { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10663e4; continue;
 }
 0x10663e4 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1066418; continue; }
  pc = 0x10663ed; continue;
 }
 0x10663ed => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10663f4); f_010675a1(cpu); if cpu.returned_to != 0x10663f4 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x106648c; continue; }
  pc = 0x10663fd; continue;
 }
 0x10663fd => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106648c; continue; }
  pc = 0x1066408; continue;
 }
 0x1066408 => {
  cpu.push(0x106640d); f_01050508(cpu); if cpu.returned_to != 0x106640d { return; }
  if cpu.cf { pc = 0x106648c; continue; }
  pc = 0x1066413; continue;
 }
 0x1066413 => {
  pc = 0x10664e3; continue;
 }
 0x1066418 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x106648c; continue; }
  pc = 0x1066422; continue;
 }
 0x1066422 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1066461; continue; }
  pc = 0x106642e; continue;
 }
 0x106642e => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x1066461; continue; }
  pc = 0x1066434; continue;
 }
 0x1066434 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1066457; continue; }
  pc = 0x106644a; continue;
 }
 0x106644a => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.esi)), 0xfbu8); }
  if cpu.zf { pc = 0x1066457; continue; }
  pc = 0x1066450; continue;
 }
 0x1066450 => {
  cpu.push(0x1066455); f_01053608(cpu); if cpu.returned_to != 0x1066455 { return; }
  if cpu.cf { pc = 0x1066461; continue; }
  pc = 0x1066457; continue;
 }
 0x1066457 => {
  cpu.push(0x106645c); f_010647ca(cpu); if cpu.returned_to != 0x106645c { return; }
  pc = 0x10664e3; continue;
 }
 0x1066461 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x106648c; continue; }
  pc = 0x1066467; continue;
 }
 0x1066467 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x106646e); f_01067515(cpu); if cpu.returned_to != 0x106646e { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x106648c; continue; }
  pc = 0x1066473; continue;
 }
 0x1066473 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106648c; continue; }
  pc = 0x106647a; continue;
 }
 0x106647a => {
  cpu.push(0x106647f); f_0108a349(cpu); if cpu.returned_to != 0x106647f { return; }
  if cpu.cf { pc = 0x106648c; continue; }
  pc = 0x1066481; continue;
 }
 0x1066481 => {
  cpu.push(0x1066486); f_0105304b(cpu); if cpu.returned_to != 0x1066486 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x10664d1; continue; }
  pc = 0x106648c; continue;
 }
 0x106648c => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x106649c); f_01067331(cpu); if cpu.returned_to != 0x106649c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10664b9; continue; }
  pc = 0x10664b1; continue;
 }
 0x10664b1 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10664b9); f_0106331c(cpu); if cpu.returned_to != 0x10664b9 { return; }
  pc = 0x10664b9; continue;
 }
 0x10664b9 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x10664d2; continue; }
  pc = 0x10664c6; continue;
 }
 0x10664c6 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x10664d2; continue; }
  pc = 0x10664cc; continue;
 }
 0x10664cc => {
  cpu.push(0x10664d1); f_010675fd(cpu); if cpu.returned_to != 0x10664d1 { return; }
  pc = 0x10664d1; continue;
 }
 0x10664d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10664d2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x10664e3; continue; }
  pc = 0x10664d8; continue;
 }
 0x10664d8 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10664e3); f_0106331c(cpu); if cpu.returned_to != 0x10664e3 { return; }
  pc = 0x10664e3; continue;
 }
 0x10664e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010664e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10664e4); }
 let mut pc: u32 = 0x10664e4;
 loop { match pc {
 0x10664e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757cbu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10664f6; continue;
 }
 0x10664f6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x1066504; continue; }
  pc = 0x10664fb; continue;
 }
 0x10664fb => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10664f6; continue; }
  pc = 0x1066500; continue;
 }
 0x1066500 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066504 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1152e54u32); cpu.eax = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066514(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066514); }
 let mut pc: u32 = 0x1066514;
 loop { match pc {
 0x1066514 => {
  { let v: u32 = 0x10c63d4u32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x113964cu32; cpu.ebp = v; }
  { let v: u32 = 0x112ad31u32; cpu.edx = v; }
  pc = 0x1066528; continue;
 }
 0x1066528 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10ee989u32, v); }
  cpu.push(0x1066539); f_0106654a(cpu); if cpu.returned_to != 0x1066539 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xe1u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x28u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066528; continue; }
  pc = 0x1066549; continue;
 }
 0x1066549 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106654a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106654a); }
 let mut pc: u32 = 0x106654a;
 loop { match pc {
 0x106654a => {
  { let v: u32 = 0xe1u32; cpu.ecx = v; }
  { let v: u8 = 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1066553; continue;
 }
 0x1066553 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x106657a; continue; }
  pc = 0x1066558; continue;
 }
 0x1066558 => {
  if cpu.sf { pc = 0x106657a; continue; }
  pc = 0x106655a; continue;
 }
 0x106655a => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee989u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106657a; continue;
 }
 0x106657a => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1066585; continue; }
  pc = 0x1066581; continue;
 }
 0x1066581 => {
  { let v: u8 = 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1066585; continue;
 }
 0x1066585 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066553; continue; }
  pc = 0x1066587; continue;
 }
 0x1066587 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066588(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066588); }
 let mut pc: u32 = 0x1066588;
 loop { match pc {
 0x1066588 => {
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x106660c; continue; }
  pc = 0x1066591; continue;
 }
 0x1066591 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x11757a4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x1066604; continue; }
  pc = 0x10665b8; continue;
 }
 0x10665b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.edi))); cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10665e9); f_01059da3(cpu); if cpu.returned_to != 0x10665e9 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066604 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x15u8); }
  if (cpu.sf != cpu.of) { pc = 0x106660e; continue; }
  pc = 0x106660b; continue;
 }
 0x106660b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x106660c; continue;
 }
 0x106660c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106660e => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x13u32); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  if cpu.ecx & 0xffff == 0 { cpu.poll(); pc = 0x106660b; continue; }
  pc = 0x106661f; continue;
 }
 0x106661f => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066631(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066631); }
 let mut pc: u32 = 0x1066631;
 loop { match pc {
 0x1066631 => {
  { let v: u32 = 0x11757a4u32; cpu.edi = v; }
  { let v: u32 = 0x10e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1179b24u32; cpu.edi = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11757b7u32; cpu.edi = v; }
  pc = 0x1066656; continue;
 }
 0x1066656 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.add32(cpu.edi, 0x33u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1066656; continue; }
  pc = 0x1066664; continue;
 }
 0x1066664 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066665(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066665); }
 let mut pc: u32 = 0x1066665;
 loop { match pc {
 0x1066665 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10c6a58u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1066678; continue;
 }
 0x1066678 => {
  { cpu.sub8(cpu.r8(0x42u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1066681; continue; }
  pc = 0x106667e; continue;
 }
 0x106667e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x42u32.wrapping_add(cpu.esi))); cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x1066681; continue;
 }
 0x1066681 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.esi))); cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x1066689; continue; }
  pc = 0x1066686; continue;
 }
 0x1066686 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x3du32.wrapping_add(cpu.esi))); cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  pc = 0x1066689; continue;
 }
 0x1066689 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1066697; continue; }
  pc = 0x106668f; continue;
 }
 0x106668f => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10666a4; continue; }
  pc = 0x1066695; continue;
 }
 0x1066695 => {
  pc = 0x10666a9; continue;
 }
 0x1066697 => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10666b2; continue; }
  pc = 0x106669d; continue;
 }
 0x106669d => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10c6a58u32)); cpu.w16(0x10c6a58u32, v); }
  pc = 0x10666a4; continue;
 }
 0x10666a4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10u32.wrapping_add(cpu.esi))); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x10666ba; continue; }
  pc = 0x10666a9; continue;
 }
 0x10666a9 => {
  cpu.push(0x10666ae); f_010666c0(cpu); if cpu.returned_to != 0x10666ae { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  pc = 0x10666b2; continue;
 }
 0x10666b2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x10666ba; continue;
 }
 0x10666ba => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1066678; continue; }
  pc = 0x10666bf; continue;
 }
 0x10666bf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010666c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10666c0); }
 let mut pc: u32 = 0x10666c0;
 loop { match pc {
 0x10666c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  pc = 0x10666c5; continue;
 }
 0x10666c5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x10666d2; continue;
 }
 0x10666d2 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10666e9; continue; }
  pc = 0x10666d7; continue;
 }
 0x10666d7 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10666df; continue; }
  pc = 0x10666dc; continue;
 }
 0x10666dc => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x10666df; continue;
 }
 0x10666df => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10666d2; continue; }
  pc = 0x10666e1; continue;
 }
 0x10666e1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10666c5; continue; }
  pc = 0x10666e6; continue;
 }
 0x10666e6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10666e9 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10666df; continue; }
  pc = 0x10666ee; continue;
 }
 0x10666ee => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.eax as u8)); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x10666df; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066710); }
 let mut pc: u32 = 0x1066710;
 loop { match pc {
 0x1066710 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) | 0x1u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) & 0xefu8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1066723; continue;
 }
 0x1066723 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x106673e; continue; }
  pc = 0x1066728; continue;
 }
 0x1066728 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x106673c); f_01067331(cpu); if cpu.returned_to != 0x106673c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106673e; continue;
 }
 0x106673e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1066723; continue; }
  pc = 0x1066743; continue;
 }
 0x1066743 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106674c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106674c); } r_0106674c(cpu, 0x106674c); }
pub fn f_010667a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10667a4); } r_0106674c(cpu, 0x10667a4); }
pub fn f_01066b00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1066b00); } r_0106674c(cpu, 0x1066b00); }
fn r_0106674c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x106674c => {
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x106677a; continue; }
  pc = 0x1066757; continue;
 }
 0x1066757 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  { let v: u16 = cpu.inc16(cpu.r16(0x81u32.wrapping_add(cpu.ebp))); cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0x7u16); }
  if cpu.zf { pc = 0x1066771; continue; }
  pc = 0x1066770; continue;
 }
 0x1066770 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066771 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106677a; continue;
 }
 0x106677a => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1066795; continue; }
  pc = 0x1066780; continue;
 }
 0x1066780 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1066786); f_01047a18(cpu); if cpu.returned_to != 0x1066786 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x10667fc; continue; }
  pc = 0x1066789; continue;
 }
 0x1066789 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1066af4; continue; }
  pc = 0x1066793; continue;
 }
 0x1066793 => {
  pc = 0x10667fd; continue;
 }
 0x1066795 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10667b4; continue; }
  pc = 0x106679e; continue;
 }
 0x106679e => {
  { let v: u8 = 0x4u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x10667b5; continue;
 }
 0x10667a4 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10667b4; continue; }
  pc = 0x10667aa; continue;
 }
 0x10667aa => {
  { let v: u8 = 0x8u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x10667b5; continue;
 }
 0x10667b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10667b5 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10667fc; continue; }
  pc = 0x10667bf; continue;
 }
 0x10667bf => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10667dd); f_01049cd8(cpu); if cpu.returned_to != 0x10667dd { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10667e6); f_0105373b(cpu); if cpu.returned_to != 0x10667e6 { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10667f0); f_0106698b(cpu); if cpu.returned_to != 0x10667f0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10667fc; continue;
 }
 0x10667fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10667fd => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x1066af4; continue; }
  pc = 0x1066807; continue;
 }
 0x1066807 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1066af4; continue; }
  pc = 0x1066811; continue;
 }
 0x1066811 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x106694c; continue; }
  pc = 0x106681e; continue;
 }
 0x106681e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.ebp))); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x106682a; continue; }
  pc = 0x1066826; continue;
 }
 0x1066826 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x106682a; continue;
 }
 0x106682a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1066858; continue; }
  pc = 0x106682f; continue;
 }
 0x106682f => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1066854; continue; }
  pc = 0x106683d; continue;
 }
 0x106683d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1066944; continue; }
  pc = 0x1066854; continue;
 }
 0x1066854 => {
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066858; continue;
 }
 0x1066858 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1066944; continue; }
  pc = 0x1066862; continue;
 }
 0x1066862 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1066869); f_01067515(cpu); if cpu.returned_to != 0x1066869 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x1066944; continue; }
  pc = 0x1066872; continue;
 }
 0x1066872 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1066944; continue; }
  pc = 0x106687d; continue;
 }
 0x106687d => {
  cpu.push(0x1066882); f_0108a349(cpu); if cpu.returned_to != 0x1066882 { return; }
  if cpu.cf { pc = 0x1066944; continue; }
  pc = 0x1066888; continue;
 }
 0x1066888 => {
  cpu.push(0x106688d); f_0108c181(cpu); if cpu.returned_to != 0x106688d { return; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x112b518u32, v); }
  cpu.push(0x10668a5); f_01065635(cpu); if cpu.returned_to != 0x10668a5 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10668bb); f_0105373b(cpu); if cpu.returned_to != 0x10668bb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0xcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e64u32, v); }
  cpu.push(0x10668de); f_0106483c(cpu); if cpu.returned_to != 0x10668de { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x10668f9; continue; }
  pc = 0x10668ef; continue;
 }
 0x10668ef => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10668f9; continue; }
  pc = 0x10668f7; continue;
 }
 0x10668f7 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10668f9; continue;
 }
 0x10668f9 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x106690a); f_01053764(cpu); if cpu.returned_to != 0x106690a { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1066913); f_01049cd8(cpu); if cpu.returned_to != 0x1066913 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106694b; continue; }
  pc = 0x1066920; continue;
 }
 0x1066920 => {
  { let v: u32 = cpu.r32(0x112b518u32); cpu.esi = v; }
  cpu.push(0x106692b); f_01064020(cpu); if cpu.returned_to != 0x106692b { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  cpu.push(0x1066938); f_0106698b(cpu); if cpu.returned_to != 0x1066938 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066944; continue;
 }
 0x1066944 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106694b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106694c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x74u32.wrapping_add(cpu.ebp))); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1066961); f_01067515(cpu); if cpu.returned_to != 0x1066961 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1066944; continue; }
  pc = 0x106696a; continue;
 }
 0x106696a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x1066944; continue; }
  pc = 0x1066979; continue;
 }
 0x1066979 => {
  cpu.push(0x106697e); f_0108a349(cpu); if cpu.returned_to != 0x106697e { return; }
  if cpu.cf { cpu.poll(); pc = 0x1066944; continue; }
  pc = 0x1066980; continue;
 }
 0x1066980 => {
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1066888; continue;
 }
 0x1066af4 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1066b0a; continue; }
  pc = 0x1066afd; continue;
 }
 0x1066afd => {
  { cpu.sub8(cpu.r8(0x10ef20du32), 0x2u8); }
  if !cpu.zf { pc = 0x1066be5; continue; }
  pc = 0x1066b0a; continue;
 }
 0x1066b00 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if !cpu.zf { pc = 0x1066be5; continue; }
  pc = 0x1066b0a; continue;
 }
 0x1066b0a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x14u32.wrapping_add(cpu.edi))); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x1066b16; continue; }
  pc = 0x1066b12; continue;
 }
 0x1066b12 => {
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1066b16; continue;
 }
 0x1066b16 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1066be5; continue; }
  pc = 0x1066b1f; continue;
 }
 0x1066b1f => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e4134u32, v); }
  cpu.push(0x1066b4a); f_01064a95(cpu); if cpu.returned_to != 0x1066b4a { return; }
  if cpu.cf { pc = 0x1066be5; continue; }
  pc = 0x1066b50; continue;
 }
 0x1066b50 => {
  { cpu.pushad(); }
  cpu.push(0x1066b56); f_0104885a(cpu); if cpu.returned_to != 0x1066b56 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1066b7e; continue; }
  pc = 0x1066b59; continue;
 }
 0x1066b59 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11e21a0u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11e21a2u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  cpu.push(0x1066b77); f_0106483c(cpu); if cpu.returned_to != 0x1066b77 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1066b9b; continue;
 }
 0x1066b7e => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1066b9b; continue; }
  pc = 0x1066b84; continue;
 }
 0x1066b84 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1066b99); f_0106b659(cpu); if cpu.returned_to != 0x1066b99 { return; }
  pc = 0x1066be5; continue;
 }
 0x1066b9b => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1066baf; continue; }
  pc = 0x1066ba1; continue;
 }
 0x1066ba1 => {
  { let v: u16 = cpu.r16(0x11e21a0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11e21a2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1066baf; continue;
 }
 0x1066baf => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1066bbe); f_01049cd8(cpu); if cpu.returned_to != 0x1066bbe { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x39u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1066be6; continue; }
  pc = 0x1066bc5; continue;
 }
 0x1066bc5 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1066be5; continue; }
  pc = 0x1066bce; continue;
 }
 0x1066bce => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1066bda; continue; }
  pc = 0x1066bd5; continue;
 }
 0x1066bd5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066bdd; continue;
 }
 0x1066bda => {
  { let v: u8 = cpu.inc8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066bdd; continue;
 }
 0x1066bdd => {
  { let v: u8 = cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066be5; continue;
 }
 0x1066be5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066be6 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1066c92; continue; }
  pc = 0x1066bf0; continue;
 }
 0x1066bf0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1066bff); f_0105373b(cpu); if cpu.returned_to != 0x1066bff { return; }
  cpu.push(0x1066c04); f_01064a95(cpu); if cpu.returned_to != 0x1066c04 { return; }
  if cpu.cf { pc = 0x1066c8e; continue; }
  pc = 0x1066c0a; continue;
 }
 0x1066c0a => {
  { let v = cpu.r32(0x10e4134u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x112b518u32, v); }
  cpu.push(0x1066c1b); f_0104885a(cpu); if cpu.returned_to != 0x1066c1b { return; }
  if cpu.cf { pc = 0x1066c8e; continue; }
  pc = 0x1066c1d; continue;
 }
 0x1066c1d => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  cpu.push(0x1066c31); f_0106483c(cpu); if cpu.returned_to != 0x1066c31 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1066c51; continue; }
  pc = 0x1066c47; continue;
 }
 0x1066c47 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1066c51; continue; }
  pc = 0x1066c4f; continue;
 }
 0x1066c4f => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1066c51; continue;
 }
 0x1066c51 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112b518u32); cpu.esi = v; }
  cpu.push(0x1066c68); f_01064020(cpu); if cpu.returned_to != 0x1066c68 { return; }
  cpu.push(0x1066c6d); f_01053764(cpu); if cpu.returned_to != 0x1066c6d { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  cpu.push(0x1066c82); f_0106698b(cpu); if cpu.returned_to != 0x1066c82 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066c8e; continue;
 }
 0x1066c8e => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066c92; continue;
 }
 0x1066c92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106698b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106698b); }
 let mut pc: u32 = 0x106698b;
 loop { match pc {
 0x106698b => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x1066ace; continue; }
  pc = 0x1066996; continue;
 }
 0x1066996 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x1066a69; continue; }
  pc = 0x10669a0; continue;
 }
 0x10669a0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x1066ab7; continue; }
  pc = 0x10669aa; continue;
 }
 0x10669aa => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x112b59eu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112b59cu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x112b520u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112b51cu32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0x20u8; cpu.w8(0x1128d9fu32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x1128da0u32, v); }
  cpu.push(0x10669f6); f_01049561(cpu); if cpu.returned_to != 0x10669f6 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x1128da0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  if cpu.cf { pc = 0x1066a0b; continue; }
  pc = 0x1066a08; continue;
 }
 0x1066a08 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066a0b; continue;
 }
 0x1066a0b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x112b5beu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112b5bcu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x112b560u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112b55cu32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0x20u8; cpu.w8(0x1128d9fu32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x1128da0u32, v); }
  cpu.push(0x1066a52); f_01049561(cpu); if cpu.returned_to != 0x1066a52 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x1128da0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  if cpu.cf { pc = 0x1066a67; continue; }
  pc = 0x1066a64; continue;
 }
 0x1066a64 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066a67; continue;
 }
 0x1066a67 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066a69 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1066a8a); f_01072d00(cpu); if cpu.returned_to != 0x1066a8a { return; }
  if cpu.cf { pc = 0x1066a9f; continue; }
  pc = 0x1066a8c; continue;
 }
 0x1066a8c => {
  { let v: u8 = 0x2u8; cpu.w8(0x110d736u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x1066a9f; continue;
 }
 0x1066a9f => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296b5u32, v); }
  cpu.push(0x1066aac); f_0104a611(cpu); if cpu.returned_to != 0x1066aac { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296b5u32, v); }
  if cpu.cf { pc = 0x1066ab5; continue; }
  pc = 0x1066ab5; continue;
 }
 0x1066ab5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066ab7 => {
  { let v: u8 = 0x19u8; cpu.w8(0x1128da1u32, v); }
  cpu.push(0x1066ac3); f_01049895(cpu); if cpu.returned_to != 0x1066ac3 { return; }
  if cpu.cf { pc = 0x1066acc; continue; }
  pc = 0x1066ac5; continue;
 }
 0x1066ac5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066acc; continue;
 }
 0x1066acc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066ace => {
  { let v: u8 = 0x19u8; cpu.w8(0x1128da1u32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x1128da2u32, v); }
  cpu.push(0x1066ae1); f_01049895(cpu); if cpu.returned_to != 0x1066ae1 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x1128da2u32, v); }
  if cpu.cf { pc = 0x1066aed; continue; }
  pc = 0x1066aea; continue;
 }
 0x1066aea => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066aed; continue;
 }
 0x1066aed => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066aef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066aef); }
 let mut pc: u32 = 0x1066aef;
 loop { match pc {
 0x1066aef => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066c93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066c93); }
 let mut pc: u32 = 0x1066c93;
 loop { match pc {
 0x1066c93 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1066cc2); f_01036a75(cpu); if cpu.returned_to != 0x1066cc2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066cca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066cca); }
 let mut pc: u32 = 0x1066cca;
 loop { match pc {
 0x1066cca => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1066d3d; continue; }
  pc = 0x1066cde; continue;
 }
 0x1066cde => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1066d3b; continue; }
  pc = 0x1066cea; continue;
 }
 0x1066cea => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c941u32, v); }
  cpu.push(0x1066d09); f_01050c5b(cpu); if cpu.returned_to != 0x1066d09 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c941u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1066d21; continue; }
  pc = 0x1066d19; continue;
 }
 0x1066d19 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1066d21; continue;
 }
 0x1066d21 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066d3b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066d3d => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1066d43); f_01066d7b(cpu); if cpu.returned_to != 0x1066d43 { return; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { cpu.poll(); pc = 0x1066d3b; continue; }
  pc = 0x1066d48; continue;
 }
 0x1066d48 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x8au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1066cca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066d7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066d7b); }
 let mut pc: u32 = 0x1066d7b;
 loop { match pc {
 0x1066d7b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4138u32, v); }
  pc = 0x1066d8f; continue;
 }
 0x1066d8f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e4138u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1066e05; continue; }
  pc = 0x1066dbb; continue;
 }
 0x1066dbb => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1066e05; continue; }
  pc = 0x1066dc0; continue;
 }
 0x1066dc0 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x1066e05; continue; }
  pc = 0x1066dc8; continue;
 }
 0x1066dc8 => {
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
  if cpu.zf { pc = 0x1066df9; continue; }
  pc = 0x1066df6; continue;
 }
 0x1066df6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1066d8f; continue; }
  pc = 0x1066df9; continue;
 }
 0x1066df9 => {
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066e05 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1066d8f; continue; }
  pc = 0x1066e09; continue;
 }
 0x1066e09 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066e10); }
 let mut pc: u32 = 0x1066e10;
 loop { match pc {
 0x1066e10 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1066e1c; continue; }
  pc = 0x1066e16; continue;
 }
 0x1066e16 => {
  cpu.push(0x1066e1b); f_01066e31(cpu); if cpu.returned_to != 0x1066e1b { return; }
  pc = 0x1066e1b; continue;
 }
 0x1066e1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066e1c => {
  { let v: u8 = 0xfu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1066e1b; continue; }
  pc = 0x1066e26; continue;
 }
 0x1066e26 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066e31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066e31); }
 let mut pc: u32 = 0x1066e31;
 loop { match pc {
 0x1066e31 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1066e44); f_0105f215(cpu); if cpu.returned_to != 0x1066e44 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1066e60); f_0105f2ec(cpu); if cpu.returned_to != 0x1066e60 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066e64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066e64); }
 let mut pc: u32 = 0x1066e64;
 loop { match pc {
 0x1066e64 => {
  { let v: u8 = cpu.r8(0x10ee92eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1066e9e; continue; }
  pc = 0x1066e75; continue;
 }
 0x1066e75 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x1066e9f; continue; }
  pc = 0x1066e7b; continue;
 }
 0x1066e7b => {
  cpu.push(0x1066e80); f_0106a2d3(cpu); if cpu.returned_to != 0x1066e80 { return; }
  if cpu.cf { pc = 0x1066e92; continue; }
  pc = 0x1066e82; continue;
 }
 0x1066e82 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1066e9e; continue; }
  pc = 0x1066e8b; continue;
 }
 0x1066e8b => {
  cpu.push(0x1066e90); f_01069fde(cpu); if cpu.returned_to != 0x1066e90 { return; }
  pc = 0x1066e9e; continue;
 }
 0x1066e92 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1066e99); f_010512c6(cpu); if cpu.returned_to != 0x1066e99 { return; }
  pc = 0x1066e99; continue;
 }
 0x1066e99 => {
  cpu.push(0x1066e9e); f_01066e31(cpu); if cpu.returned_to != 0x1066e9e { return; }
  pc = 0x1066e9e; continue;
 }
 0x1066e9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1066e9f => {
  cpu.push(0x1066ea4); f_0106a2d3(cpu); if cpu.returned_to != 0x1066ea4 { return; }
  if cpu.cf { cpu.poll(); pc = 0x1066e99; continue; }
  pc = 0x1066ea6; continue;
 }
 0x1066ea6 => {
  { let v: u8 = 0x8u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01066eaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1066eaf); }
 let mut pc: u32 = 0x1066eaf;
 loop { match pc {
 0x1066eaf => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1066f02; continue; }
  pc = 0x1066eb5; continue;
 }
 0x1066eb5 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x10671a9; continue; }
  pc = 0x1066ec5; continue;
 }
 0x1066ec5 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1066f9b; continue; }
  pc = 0x1066ecf; continue;
 }
 0x1066ecf => {
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1066edf; continue; }
  pc = 0x1066ed5; continue;
 }
 0x1066ed5 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1066f9b; continue; }
  pc = 0x1066edf; continue;
 }
 0x1066edf => {
  cpu.push(0x1066ee4); f_01053608(cpu); if cpu.returned_to != 0x1066ee4 { return; }
  if cpu.cf { pc = 0x1066f9b; continue; }
  pc = 0x1066eea; continue;
 }
 0x1066eea => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc45u32, v); }
  cpu.push(0x1066ef6); f_010647ca(cpu); if cpu.returned_to != 0x1066ef6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc45u32, v); }
  pc = 0x10671ae; continue;
 }
 0x1066f02 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1066f40; continue; }
  pc = 0x1066f08; continue;
 }
 0x1066f08 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x1066f9b; continue; }
  pc = 0x1066f12; continue;
 }
 0x1066f12 => {
  { let v: u32 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1152e80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1152e82u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1066f31); f_010507b9(cpu); if cpu.returned_to != 0x1066f31 { return; }
  cpu.push(0x1066f36); f_01069fde(cpu); if cpu.returned_to != 0x1066f36 { return; }
  cpu.push(0x1066f3b); f_0106a2d3(cpu); if cpu.returned_to != 0x1066f3b { return; }
  pc = 0x10671ae; continue;
 }
 0x1066f40 => {
  cpu.push(0x1066f45); f_0106b659(cpu); if cpu.returned_to != 0x1066f45 { return; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1066f7c; continue; }
  pc = 0x1066f51; continue;
 }
 0x1066f51 => {
  { cpu.sub8(cpu.r8(0x3eu32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x1066f7c; continue; }
  pc = 0x1066f57; continue;
 }
 0x1066f57 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1066f88; continue; }
  pc = 0x1066f5d; continue;
 }
 0x1066f5d => {
  cpu.push(0x1066f62); f_01053608(cpu); if cpu.returned_to != 0x1066f62 { return; }
  if cpu.cf { pc = 0x1066f7c; continue; }
  pc = 0x1066f64; continue;
 }
 0x1066f64 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc45u32, v); }
  cpu.push(0x1066f70); f_010647ca(cpu); if cpu.returned_to != 0x1066f70 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc45u32, v); }
  pc = 0x10671ae; continue;
 }
 0x1066f7c => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1066f9b; continue; }
  pc = 0x1066f82; continue;
 }
 0x1066f82 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  pc = 0x1066f88; continue;
 }
 0x1066f88 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1066f9b; continue; }
  pc = 0x1066f8e; continue;
 }
 0x1066f8e => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1066f96); f_01069fff(cpu); if cpu.returned_to != 0x1066f96 { return; }
  pc = 0x10671ae; continue;
 }
 0x1066f9b => {
  { let v: u8 = 0x4u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x40u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1067087; continue; }
  pc = 0x1066fab; continue;
 }
 0x1066fab => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1067197; continue; }
  pc = 0x1066fb8; continue;
 }
 0x1066fb8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1066fc5); f_0106331c(cpu); if cpu.returned_to != 0x1066fc5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x40u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x1067083; continue; }
  pc = 0x1066fd2; continue;
 }
 0x1066fd2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106700a; continue; }
  pc = 0x1066fd8; continue;
 }
 0x1066fd8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106700a; continue; }
  pc = 0x1066fde; continue;
 }
 0x1066fde => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1066fe6); f_010675a1(cpu); if cpu.returned_to != 0x1066fe6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x106700a; continue; }
  pc = 0x1066fec; continue;
 }
 0x1066fec => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106700a; continue; }
  pc = 0x1066ff3; continue;
 }
 0x1066ff3 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1066ff9); f_010671af(cpu); if cpu.returned_to != 0x1066ff9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106700a; continue; }
  pc = 0x1066ffc; continue;
 }
 0x1066ffc => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1067002); f_01050508(cpu); if cpu.returned_to != 0x1067002 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106700a; continue; }
  pc = 0x1067005; continue;
 }
 0x1067005 => {
  pc = 0x10671ae; continue;
 }
 0x106700a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1067012); f_01067a45(cpu); if cpu.returned_to != 0x1067012 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x1067036; continue; }
  pc = 0x1067017; continue;
 }
 0x1067017 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1067036; continue; }
  pc = 0x106701e; continue;
 }
 0x106701e => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1172f54u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x1067032); f_01058f92(cpu); if cpu.returned_to != 0x1067032 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106707e; continue;
 }
 0x1067036 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x112b5deu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106705a); f_010512c6(cpu); if cpu.returned_to != 0x106705a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1067060); f_01069fe6(cpu); if cpu.returned_to != 0x1067060 { return; }
  cpu.push(0x1067065); f_0106a04e(cpu); if cpu.returned_to != 0x1067065 { return; }
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x19u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106706e; continue; }
  pc = 0x106706d; continue;
 }
 0x106706d => {
  {  }
  pc = 0x106706e; continue;
 }
 0x106706e => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106707e; continue; }
  pc = 0x1067074; continue;
 }
 0x1067074 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x106707e; continue;
 }
 0x106707e => {
  pc = 0x10671ae; continue;
 }
 0x1067083 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10670eb; continue;
 }
 0x1067087 => {
  { let v: u8 = 0xffu8; cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x1067096; continue; }
  pc = 0x1067090; continue;
 }
 0x1067090 => {
  if !cpu.zf { pc = 0x1067197; continue; }
  pc = 0x1067096; continue;
 }
 0x1067096 => {
  { let v: u8 = 0x7u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.ebp))); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x10670dd; continue; }
  pc = 0x106709f; continue;
 }
 0x106709f => {
  { let v: u8 = 0x7u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10670ac); f_01053641(cpu); if cpu.returned_to != 0x10670ac { return; }
  if cpu.cf { pc = 0x10670dd; continue; }
  pc = 0x10670ae; continue;
 }
 0x10670ae => {
  cpu.push(0x10670b3); f_01053608(cpu); if cpu.returned_to != 0x10670b3 { return; }
  if !cpu.cf { pc = 0x10670c7; continue; }
  pc = 0x10670b5; continue;
 }
 0x10670b5 => {
  cpu.push(0x10670ba); f_0105f215(cpu); if cpu.returned_to != 0x10670ba { return; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10670c7; continue; }
  pc = 0x10670be; continue;
 }
 0x10670be => {
  cpu.push(0x10670c3); f_0105f215(cpu); if cpu.returned_to != 0x10670c3 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10670ce; continue;
 }
 0x10670c7 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10670ce; continue;
 }
 0x10670ce => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10670d6); f_0105f215(cpu); if cpu.returned_to != 0x10670d6 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10670dd; continue;
 }
 0x10670dd => {
  cpu.push(0x10670e2); f_0105f215(cpu); if cpu.returned_to != 0x10670e2 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10670eb; continue;
 }
 0x10670eb => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1067127; continue; }
  pc = 0x10670f1; continue;
 }
 0x10670f1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x10670fd; continue; }
  pc = 0x10670f7; continue;
 }
 0x10670f7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067127; continue;
 }
 0x10670fd => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1067103); f_010675a1(cpu); if cpu.returned_to != 0x1067103 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x1067127; continue; }
  pc = 0x1067109; continue;
 }
 0x1067109 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1067127; continue; }
  pc = 0x1067110; continue;
 }
 0x1067110 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1067116); f_010671af(cpu); if cpu.returned_to != 0x1067116 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1067127; continue; }
  pc = 0x1067119; continue;
 }
 0x1067119 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x106711f); f_01050508(cpu); if cpu.returned_to != 0x106711f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1067127; continue; }
  pc = 0x1067122; continue;
 }
 0x1067122 => {
  pc = 0x10671ae; continue;
 }
 0x1067127 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1067136; continue; }
  pc = 0x1067130; continue;
 }
 0x1067130 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  pc = 0x1067136; continue;
 }
 0x1067136 => {
  cpu.push(0x106713b); f_01067515(cpu); if cpu.returned_to != 0x106713b { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x1067199; continue; }
  pc = 0x1067140; continue;
 }
 0x1067140 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1067199; continue; }
  pc = 0x1067147; continue;
 }
 0x1067147 => {
  cpu.push(0x106714c); f_0108a349(cpu); if cpu.returned_to != 0x106714c { return; }
  if cpu.cf { pc = 0x1067199; continue; }
  pc = 0x106714e; continue;
 }
 0x106714e => {
  cpu.push(0x1067153); f_0108c35e(cpu); if cpu.returned_to != 0x1067153 { return; }
  if cpu.cf { pc = 0x1067199; continue; }
  pc = 0x1067155; continue;
 }
 0x1067155 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1067162; continue; }
  pc = 0x106715b; continue;
 }
 0x106715b => {
  cpu.push(0x1067160); f_01063e07(cpu); if cpu.returned_to != 0x1067160 { return; }
  pc = 0x10671ae; continue;
 }
 0x1067162 => {
  cpu.push(0x1067167); f_0108c181(cpu); if cpu.returned_to != 0x1067167 { return; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1152e82u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1152e80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1067189); f_010512c6(cpu); if cpu.returned_to != 0x1067189 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x106718f); f_01069fe6(cpu); if cpu.returned_to != 0x106718f { return; }
  cpu.push(0x1067194); f_0106a04e(cpu); if cpu.returned_to != 0x1067194 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1067197; continue;
 }
 0x1067197 => {
  pc = 0x10671ae; continue;
 }
 0x1067199 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x3u8); }
  if !cpu.zf { pc = 0x10671a9; continue; }
  pc = 0x10671a2; continue;
 }
 0x10671a2 => {
  cpu.push(0x10671a7); f_010671d0(cpu); if cpu.returned_to != 0x10671a7 { return; }
  if cpu.cf { pc = 0x10671ae; continue; }
  pc = 0x10671a9; continue;
 }
 0x10671a9 => {
  cpu.push(0x10671ae); f_01067331(cpu); if cpu.returned_to != 0x10671ae { return; }
  pc = 0x10671ae; continue;
 }
 0x10671ae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010671af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10671af); }
 let mut pc: u32 = 0x10671af;
 loop { match pc {
 0x10671af => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x10671cc; continue; }
  pc = 0x10671b5; continue;
 }
 0x10671b5 => {
  cpu.push(0x10671ba); f_01053987(cpu); if cpu.returned_to != 0x10671ba { return; }
  if !cpu.cf { pc = 0x10671c4; continue; }
  pc = 0x10671bc; continue;
 }
 0x10671bc => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x10671ce; continue; }
  pc = 0x10671c0; continue;
 }
 0x10671c0 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x10671cc; continue; }
  pc = 0x10671c4; continue;
 }
 0x10671c4 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), cpu.r8(0x99u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10671ce; continue; }
  pc = 0x10671cc; continue;
 }
 0x10671cc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10671ce => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010671d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10671d0); }
 let mut pc: u32 = 0x10671d0;
 loop { match pc {
 0x10671d0 => {
  { cpu.sub16(cpu.r16(0x10e3f1bu32), 0x0u16); }
  if cpu.zf { pc = 0x106732f; continue; }
  pc = 0x10671de; continue;
 }
 0x10671de => {
  { cpu.sub16(cpu.r16(0x11230c0u32), 0xffffu16); }
  if cpu.zf { pc = 0x106732f; continue; }
  pc = 0x10671ed; continue;
 }
 0x10671ed => {
  { let v: u32 = 0x1174a48u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x112b5dcu32, v); }
  pc = 0x10671fe; continue;
 }
 0x10671fe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1067319; continue; }
  pc = 0x106720d; continue;
 }
 0x106720d => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0xb8u8); }
  if !cpu.zf { pc = 0x1067319; continue; }
  pc = 0x1067217; continue;
 }
 0x1067217 => {
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
  cpu.push(0x1067243); f_01018ffd(cpu); if cpu.returned_to != 0x1067243 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x1067319; continue; }
  pc = 0x106724b; continue;
 }
 0x106724b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x106726f; continue; }
  pc = 0x106726c; continue;
 }
 0x106726c => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106726f; continue;
 }
 0x106726f => {
  { cpu.sub16((cpu.eax as u16), 0x6u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1067298; continue; }
  pc = 0x1067275; continue;
 }
 0x1067275 => {
  { cpu.sub16((cpu.ebx as u16), 0x6u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1067298; continue; }
  pc = 0x106727b; continue;
 }
 0x106727b => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x80u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x10u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1067297); f_0105b007(cpu); if cpu.returned_to != 0x1067297 { return; }
  { cpu.popad(); }
  pc = 0x1067298; continue;
 }
 0x1067298 => {
  cpu.push(0x106729d); f_010527d7(cpu); if cpu.returned_to != 0x106729d { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10672aa); f_01072d00(cpu); if cpu.returned_to != 0x10672aa { return; }
  if cpu.cf { pc = 0x1067312; continue; }
  pc = 0x10672ac; continue;
 }
 0x10672ac => {
  { cpu.sub8(cpu.r8(0x10f7dc9u32), 0x0u8); }
  if !cpu.zf { pc = 0x1067312; continue; }
  pc = 0x10672b5; continue;
 }
 0x10672b5 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10672d5; continue; }
  pc = 0x10672cb; continue;
 }
 0x10672cb => {
  cpu.push(0x10672d0); f_01012780(cpu); if cpu.returned_to != 0x10672d0 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x10672d5; continue;
 }
 0x10672d5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc9u32, v); }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10672e7); f_010356fe(cpu); if cpu.returned_to != 0x10672e7 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x79u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1067307; continue; }
  pc = 0x10672fd; continue;
 }
 0x10672fd => {
  cpu.push(0x1067302); f_01012780(cpu); if cpu.returned_to != 0x1067302 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1067307; continue;
 }
 0x1067307 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1067312); f_010356fe(cpu); if cpu.returned_to != 0x1067312 { return; }
  pc = 0x1067312; continue;
 }
 0x1067312 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112b5dcu32, v); }
  pc = 0x1067319; continue;
 }
 0x1067319 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10671fe; continue; }
  pc = 0x1067327; continue;
 }
 0x1067327 => {
  { cpu.sub8(cpu.r8(0x112b5dcu32), 0x0u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106732f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067331(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067331); }
 let mut pc: u32 = 0x1067331;
 loop { match pc {
 0x1067331 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x106738d; continue; }
  pc = 0x1067337; continue;
 }
 0x1067337 => {
  cpu.push(0x106733c); f_0106a395(cpu); if cpu.returned_to != 0x106733c { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1067346; continue; }
  pc = 0x1067340; continue;
 }
 0x1067340 => {
  cpu.push(0x1067345); f_0106a2d3(cpu); if cpu.returned_to != 0x1067345 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067346 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x106738d; continue; }
  pc = 0x1067352; continue;
 }
 0x1067352 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1067358); f_01069fde(cpu); if cpu.returned_to != 0x1067358 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x1067364; continue; }
  pc = 0x106735f; continue;
 }
 0x106735f => {
  cpu.push(0x1067364); f_01069fee(cpu); if cpu.returned_to != 0x1067364 { return; }
  pc = 0x1067364; continue;
 }
 0x1067364 => {
  cpu.push(0x1067369); f_010673d5(cpu); if cpu.returned_to != 0x1067369 { return; }
  if cpu.cf { pc = 0x106738d; continue; }
  pc = 0x106736b; continue;
 }
 0x106736b => {
  { cpu.sub8(cpu.r8(0x112b5ddu32), 0x0u8); }
  if !cpu.zf { pc = 0x1067393; continue; }
  pc = 0x1067374; continue;
 }
 0x1067374 => {
  cpu.push(0x1067379); f_010673a7(cpu); if cpu.returned_to != 0x1067379 { return; }
  if cpu.cf { pc = 0x1067393; continue; }
  pc = 0x106737b; continue;
 }
 0x106737b => {
  cpu.push(0x1067380); f_0106a04e(cpu); if cpu.returned_to != 0x1067380 { return; }
  { let v: u8 = 0x7u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x106738e; continue; }
  pc = 0x1067386; continue;
 }
 0x1067386 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106738e; continue; }
  pc = 0x106738d; continue;
 }
 0x106738d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106738e => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067393 => {
  cpu.push(0x1067398); f_010507b9(cpu); if cpu.returned_to != 0x1067398 { return; }
  cpu.push(0x106739d); f_0106a2d3(cpu); if cpu.returned_to != 0x106739d { return; }
  cpu.push(0x10673a2); f_01069fde(cpu); if cpu.returned_to != 0x10673a2 { return; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010673a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10673a7); }
 let mut pc: u32 = 0x10673a7;
 loop { match pc {
 0x10673a7 => {
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
pub fn f_010673d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10673d5); }
 let mut pc: u32 = 0x10673d5;
 loop { match pc {
 0x10673d5 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x106750d; continue; }
  pc = 0x10673df; continue;
 }
 0x10673df => {
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x106750d; continue; }
  pc = 0x10673ea; continue;
 }
 0x10673ea => {
  if cpu.sf { pc = 0x106750d; continue; }
  pc = 0x10673f0; continue;
 }
 0x10673f0 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10c63bcu32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x112b5ddu32), 0x0u8); }
  if !cpu.zf { pc = 0x106747c; continue; }
  pc = 0x1067413; continue;
 }
 0x1067413 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1067472; continue; }
  pc = 0x106741c; continue;
 }
 0x106741c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x38u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x106746c; continue; }
  pc = 0x1067435; continue;
 }
 0x1067435 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x106746c; continue; }
  pc = 0x106743b; continue;
 }
 0x106743b => {
  { cpu.pushad(); }
  cpu.push(0x1067441); f_010696df(cpu); if cpu.returned_to != 0x1067441 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1067450; continue; }
  pc = 0x1067444; continue;
 }
 0x1067444 => {
  { cpu.pushad(); }
  cpu.push(0x106744a); f_01053cbd(cpu); if cpu.returned_to != 0x106744a { return; }
  cpu.push(0x106744f); f_01053c82(cpu); if cpu.returned_to != 0x106744f { return; }
  { cpu.popad(); }
  pc = 0x1067450; continue;
 }
 0x1067450 => {
  pc = 0x106750a; continue;
 }
 0x106746c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1067508; continue;
 }
 0x1067472 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x106750d; continue; }
  pc = 0x106747c; continue;
 }
 0x106747c => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e4038u32)); cpu.edi = v; }
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
  cpu.push(0x10674b1); f_01053800(cpu); if cpu.returned_to != 0x10674b1 { return; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x10674bb); f_01053800(cpu); if cpu.returned_to != 0x10674bb { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x10674cb); f_01053800(cpu); if cpu.returned_to != 0x10674cb { return; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.edi)); cpu.imul16(v); }
  cpu.push(0x10674d9); f_01053800(cpu); if cpu.returned_to != 0x10674d9 { return; }
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
  { let v: u16 = cpu.r16(0x1152e82u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1152e80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1067508; continue;
 }
 0x1067508 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106750a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106750d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067515(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067515); }
 let mut pc: u32 = 0x1067515;
 loop { match pc {
 0x1067515 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x106751e; continue; }
  pc = 0x106751b; continue;
 }
 0x106751b => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106751e => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1067536; continue; }
  pc = 0x1067527; continue;
 }
 0x1067527 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1067536; continue; }
  pc = 0x1067530; continue;
 }
 0x1067530 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1067539; continue; }
  pc = 0x1067536; continue;
 }
 0x1067536 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067539 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x20u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1067536; continue; }
  pc = 0x1067545; continue;
 }
 0x1067545 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106756a; continue; }
  pc = 0x106754c; continue;
 }
 0x106754c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106756a; continue; }
  pc = 0x1067552; continue;
 }
 0x1067552 => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) ^ 0x8u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106756a; continue; }
  pc = 0x106755c; continue;
 }
 0x106755c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x74u32.wrapping_add(cpu.ebp))); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067570; continue;
 }
 0x106756a => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  pc = 0x1067570; continue;
 }
 0x1067570 => {
  { let v: u32 = cpu.r32(0x10e4558u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1067583); f_010676d3(cpu); if cpu.returned_to != 0x1067583 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4578u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40acu32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1067597); f_010678d4(cpu); if cpu.returned_to != 0x1067597 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.and8((cpu.edx as u8), 0x80u8); }
  if cpu.zf { pc = 0x106759f; continue; }
  pc = 0x106759d; continue;
 }
 0x106759d => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x106759f; continue;
 }
 0x106759f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010675a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10675a1); }
 let mut pc: u32 = 0x10675a1;
 loop { match pc {
 0x10675a1 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x10675b0; continue; }
  pc = 0x10675aa; continue;
 }
 0x10675aa => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10675b3; continue; }
  pc = 0x10675b0; continue;
 }
 0x10675b0 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10675b3 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) ^ 0x40u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x10675d7; continue; }
  pc = 0x10675c0; continue;
 }
 0x10675c0 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xa0u32.wrapping_add(cpu.ebp))); cpu.w8(0xa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0xa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10675dd; continue;
 }
 0x10675d7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  pc = 0x10675dd; continue;
 }
 0x10675dd => {
  { let v: u32 = cpu.r32(0x10e4558u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10675ea); f_01067728(cpu); if cpu.returned_to != 0x10675ea { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4598u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40acu32, v); }
  cpu.push(0x10675fc); f_01067781(cpu); if cpu.returned_to != 0x10675fc { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010675fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10675fd); }
 let mut pc: u32 = 0x10675fd;
 loop { match pc {
 0x10675fd => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1067614; continue; }
  pc = 0x1067610; continue;
 }
 0x1067610 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  pc = 0x1067614; continue;
 }
 0x1067614 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1067656; continue; }
  pc = 0x106761a; continue;
 }
 0x106761a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10676c1; continue; }
  pc = 0x1067624; continue;
 }
 0x1067624 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1067637); f_0105f215(cpu); if cpu.returned_to != 0x1067637 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1067653); f_0105f2ec(cpu); if cpu.returned_to != 0x1067653 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067656; continue;
 }
 0x1067656 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x10676c1; continue; }
  pc = 0x106765c; continue;
 }
 0x106765c => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if cpu.zf { pc = 0x106767d; continue; }
  pc = 0x1067662; continue;
 }
 0x1067662 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10676c1; continue; }
  pc = 0x1067668; continue;
 }
 0x1067668 => {
  cpu.push(0x106766d); f_0106a2d3(cpu); if cpu.returned_to != 0x106766d { return; }
  if !cpu.cf { pc = 0x10676c1; continue; }
  pc = 0x106766f; continue;
 }
 0x106766f => {
  cpu.push(0x1067674); f_0106a395(cpu); if cpu.returned_to != 0x1067674 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10676c1; continue; }
  pc = 0x1067678; continue;
 }
 0x1067678 => {
  cpu.push(0x106767d); f_0106a2d3(cpu); if cpu.returned_to != 0x106767d { return; }
  pc = 0x106767d; continue;
 }
 0x106767d => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x106768f; continue; }
  pc = 0x106768a; continue;
 }
 0x106768a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106768f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x106769e; continue; }
  pc = 0x1067698; continue;
 }
 0x1067698 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  pc = 0x106769e; continue;
 }
 0x106769e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10676a5); f_01067515(cpu); if cpu.returned_to != 0x10676a5 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x10676be; continue; }
  pc = 0x10676aa; continue;
 }
 0x10676aa => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10676be; continue; }
  pc = 0x10676b1; continue;
 }
 0x10676b1 => {
  cpu.push(0x10676b6); f_0108a349(cpu); if cpu.returned_to != 0x10676b6 { return; }
  if cpu.cf { pc = 0x10676be; continue; }
  pc = 0x10676b8; continue;
 }
 0x10676b8 => {
  cpu.push(0x10676bd); f_01063e07(cpu); if cpu.returned_to != 0x10676bd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10676be => {
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10676c1; continue;
 }
 0x10676c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010676c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10676c2); }
 let mut pc: u32 = 0x10676c2;
 loop { match pc {
 0x10676c2 => {
  { let v: u32 = 0x10e47cau32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  f_010676d3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010676ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10676ce); }
 let mut pc: u32 = 0x10676ce;
 loop { match pc {
 0x10676ce => {
  { let v: u32 = 0x10e47b8u32; cpu.esi = v; }
  f_010676d3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010676d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10676d3); }
 let mut pc: u32 = 0x10676d3;
 loop { match pc {
 0x10676d3 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0x11a0f95u32; cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10676e8; continue;
 }
 0x10676e8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x106771c; continue; }
  pc = 0x10676f5; continue;
 }
 0x10676f5 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x106771c; continue; }
  pc = 0x10676fd; continue;
 }
 0x10676fd => {
  { let v: u8 = cpu.r8(0x11a5df0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x106771c; continue; }
  pc = 0x1067707; continue;
 }
 0x1067707 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x11a3df0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106771c; continue; }
  pc = 0x106771a; continue;
 }
 0x106771a => {
  { cpu.movs(1, true); }
  pc = 0x106771c; continue;
 }
 0x106771c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10676e8; continue; }
  pc = 0x1067723; continue;
 }
 0x1067723 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067728); }
 let mut pc: u32 = 0x1067728;
 loop { match pc {
 0x1067728 => {
  { let v: u32 = 0x11a0f95u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x106773c; continue;
 }
 0x106773c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x1067775; continue; }
  pc = 0x1067749; continue;
 }
 0x1067749 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x1067775; continue; }
  pc = 0x1067751; continue;
 }
 0x1067751 => {
  { let v: u32 = 0x11a5ef0u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11a7ef0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1067775; continue; }
  pc = 0x1067760; continue;
 }
 0x1067760 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x11a5ef0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1067775; continue; }
  pc = 0x1067773; continue;
 }
 0x1067773 => {
  { cpu.movs(1, true); }
  pc = 0x1067775; continue;
 }
 0x1067775 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106773c; continue; }
  pc = 0x106777c; continue;
 }
 0x106777c => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067781(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067781); } r_01067781(cpu, 0x1067781); }
pub fn f_01067829(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067829); } r_01067781(cpu, 0x1067829); }
pub fn f_0106783e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106783e); } r_01067781(cpu, 0x106783e); }
pub fn f_01067853(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067853); } r_01067781(cpu, 0x1067853); }
pub fn f_01067868(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067868); } r_01067781(cpu, 0x1067868); }
pub fn f_01067880(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067880); } r_01067781(cpu, 0x1067880); }
pub fn f_01067895(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067895); } r_01067781(cpu, 0x1067895); }
pub fn f_010678aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10678aa); } r_01067781(cpu, 0x10678aa); }
pub fn f_010678bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10678bf); } r_01067781(cpu, 0x10678bf); }
fn r_01067781(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1067781 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee92fu32, v); }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1067799; continue;
 }
 0x1067799 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1067826; continue; }
  pc = 0x10677a2; continue;
 }
 0x10677a2 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x11a188cu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ee92fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10677ba; continue;
 }
 0x10677ba => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x10e40acu32); continue;
 }
 0x10677cf => {
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
  if (cpu.sf == cpu.of) { pc = 0x1067822; continue; }
  pc = 0x10677f7; continue;
 }
 0x10677f7 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10677ff; continue;
 }
 0x10677ff => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x1067803; continue;
 }
 0x1067803 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1067811); f_0105c935(cpu); if cpu.returned_to != 0x1067811 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x1067818; continue;
 }
 0x1067818 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x1067799; continue;
 }
 0x1067822 => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x10677ff; continue;
 }
 0x1067826 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067829 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067833; continue; }
  pc = 0x1067830; continue;
 }
 0x1067830 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1067833; continue;
 }
 0x1067833 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x106783c; continue;
 }
 0x106783c => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x106783e => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067845; continue; }
  pc = 0x1067842; continue;
 }
 0x1067842 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1067845; continue;
 }
 0x1067845 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x106784e; continue;
 }
 0x106784e => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x1067853 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x106785a; continue; }
  pc = 0x1067857; continue;
 }
 0x1067857 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106785a; continue;
 }
 0x106785a => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x1067863; continue;
 }
 0x1067863 => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x1067868 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067872; continue; }
  pc = 0x106786f; continue;
 }
 0x106786f => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1067872; continue;
 }
 0x1067872 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x106787b; continue;
 }
 0x106787b => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x1067880 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x1067888; continue;
 }
 0x1067888 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x1067890; continue;
 }
 0x1067890 => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x1067895 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x106789d; continue;
 }
 0x106789d => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10678a5; continue;
 }
 0x10678a5 => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x10678aa => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10678b2; continue;
 }
 0x10678b2 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10678ba; continue;
 }
 0x10678ba => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 0x10678bf => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10678c7; continue;
 }
 0x10678c7 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067799; continue; }
  pc = 0x10678cf; continue;
 }
 0x10678cf => {
  cpu.poll(); pc = 0x10677cf; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010678d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10678d4); } r_010678d4(cpu, 0x10678d4); }
pub fn f_0106799a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106799a); } r_010678d4(cpu, 0x106799a); }
pub fn f_010679af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10679af); } r_010678d4(cpu, 0x10679af); }
pub fn f_010679c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10679c4); } r_010678d4(cpu, 0x10679c4); }
pub fn f_010679d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10679d9); } r_010678d4(cpu, 0x10679d9); }
pub fn f_010679f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10679f1); } r_010678d4(cpu, 0x10679f1); }
pub fn f_01067a06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067a06); } r_010678d4(cpu, 0x1067a06); }
pub fn f_01067a1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067a1b); } r_010678d4(cpu, 0x1067a1b); }
pub fn f_01067a30(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067a30); } r_010678d4(cpu, 0x1067a30); }
fn r_010678d4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10678d4 => {
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10678dd; continue;
 }
 0x10678dd => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1067997; continue; }
  pc = 0x10678e6; continue;
 }
 0x10678e6 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067903; continue;
 }
 0x1067903 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x106790c; continue;
 }
 0x106790c => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067912; continue;
 }
 0x1067912 => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x106791b; continue;
 }
 0x106791b => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067921; continue;
 }
 0x1067921 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067927; continue;
 }
 0x1067927 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x10e40acu32); continue;
 }
 0x106793d => {
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
  if (cpu.sf == cpu.of) { pc = 0x1067993; continue; }
  pc = 0x1067964; continue;
 }
 0x1067964 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106796c; continue;
 }
 0x106796c => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067974; continue;
 }
 0x1067974 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1067982); f_0105ca27(cpu); if cpu.returned_to != 0x1067982 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067989; continue;
 }
 0x1067989 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x10678dd; continue;
 }
 0x1067993 => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x106796c; continue;
 }
 0x1067997 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106799a => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10679a4; continue; }
  pc = 0x10679a1; continue;
 }
 0x10679a1 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10679a4; continue;
 }
 0x10679a4 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x10679ad; continue;
 }
 0x10679ad => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x10679af => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10679b6; continue; }
  pc = 0x10679b3; continue;
 }
 0x10679b3 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10679b6; continue;
 }
 0x10679b6 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x10679bf; continue;
 }
 0x10679bf => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x10679c4 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10679cb; continue; }
  pc = 0x10679c8; continue;
 }
 0x10679c8 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10679cb; continue;
 }
 0x10679cb => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x10679d4; continue;
 }
 0x10679d4 => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x10679d9 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10679e3; continue; }
  pc = 0x10679e0; continue;
 }
 0x10679e0 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10679e3; continue;
 }
 0x10679e3 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x10679ec; continue;
 }
 0x10679ec => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x10679f1 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x10679f9; continue;
 }
 0x10679f9 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a01; continue;
 }
 0x1067a01 => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x1067a06 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a0e; continue;
 }
 0x1067a0e => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a16; continue;
 }
 0x1067a16 => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x1067a1b => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a23; continue;
 }
 0x1067a23 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a2b; continue;
 }
 0x1067a2b => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 0x1067a30 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a38; continue;
 }
 0x1067a38 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x10678dd; continue; }
  pc = 0x1067a40; continue;
 }
 0x1067a40 => {
  cpu.poll(); pc = 0x106793d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067a45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067a45); }
 let mut pc: u32 = 0x1067a45;
 loop { match pc {
 0x1067a45 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1067a54; continue; }
  pc = 0x1067a4e; continue;
 }
 0x1067a4e => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1067a57; continue; }
  pc = 0x1067a54; continue;
 }
 0x1067a54 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067a57 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4558u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1067a6a); f_01067a7d(cpu); if cpu.returned_to != 0x1067a6a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e45b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40acu32, v); }
  cpu.push(0x1067a7c); f_01067ad6(cpu); if cpu.returned_to != 0x1067a7c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067a7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067a7d); }
 let mut pc: u32 = 0x1067a7d;
 loop { match pc {
 0x1067a7d => {
  { let v: u32 = 0x11a0f95u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1067a91; continue;
 }
 0x1067a91 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x1067aca; continue; }
  pc = 0x1067a9e; continue;
 }
 0x1067a9e => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x1067aca; continue; }
  pc = 0x1067aa6; continue;
 }
 0x1067aa6 => {
  { let v: u32 = 0x11a7ff0u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11a9ff0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1067aca; continue; }
  pc = 0x1067ab5; continue;
 }
 0x1067ab5 => {
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x11a7ff0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x120u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1067aca; continue; }
  pc = 0x1067ac8; continue;
 }
 0x1067ac8 => {
  { cpu.movs(1, true); }
  pc = 0x1067aca; continue;
 }
 0x1067aca => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1067a91; continue; }
  pc = 0x1067ad1; continue;
 }
 0x1067ad1 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067ad6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067ad6); } r_01067ad6(cpu, 0x1067ad6); }
pub fn f_01067b9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067b9b); } r_01067ad6(cpu, 0x1067b9b); }
pub fn f_01067bb0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067bb0); } r_01067ad6(cpu, 0x1067bb0); }
pub fn f_01067bc5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067bc5); } r_01067ad6(cpu, 0x1067bc5); }
pub fn f_01067bda(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067bda); } r_01067ad6(cpu, 0x1067bda); }
pub fn f_01067bf2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067bf2); } r_01067ad6(cpu, 0x1067bf2); }
pub fn f_01067c07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067c07); } r_01067ad6(cpu, 0x1067c07); }
pub fn f_01067c1c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067c1c); } r_01067ad6(cpu, 0x1067c1c); }
pub fn f_01067c31(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067c31); } r_01067ad6(cpu, 0x1067c31); }
fn r_01067ad6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1067ad6 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee92fu32, v); }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1067aee; continue;
 }
 0x1067aee => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1067b98; continue; }
  pc = 0x1067af7; continue;
 }
 0x1067af7 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x1172f54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ee92fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0xau8); }
  if !cpu.zf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b10; continue;
 }
 0x1067b10 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.edi)), 0x34u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b16; continue;
 }
 0x1067b16 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b1c; continue;
 }
 0x1067b1c => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x6u8); }
  if cpu.zf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b22; continue;
 }
 0x1067b22 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b28; continue;
 }
 0x1067b28 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x10e40acu32); continue;
 }
 0x1067b3d => {
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
  if (cpu.sf == cpu.of) { pc = 0x1067b94; continue; }
  pc = 0x1067b65; continue;
 }
 0x1067b65 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067b6d; continue;
 }
 0x1067b6d => {
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b75; continue;
 }
 0x1067b75 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1067b83); f_0105c9ad(cpu); if cpu.returned_to != 0x1067b83 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067b8a; continue;
 }
 0x1067b8a => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x1067aee; continue;
 }
 0x1067b94 => {
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1067b6d; continue;
 }
 0x1067b98 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067b9b => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067ba5; continue; }
  pc = 0x1067ba2; continue;
 }
 0x1067ba2 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1067ba5; continue;
 }
 0x1067ba5 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067bae; continue;
 }
 0x1067bae => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067bb0 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067bb7; continue; }
  pc = 0x1067bb4; continue;
 }
 0x1067bb4 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1067bb7; continue;
 }
 0x1067bb7 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067bc0; continue;
 }
 0x1067bc0 => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067bc5 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067bcc; continue; }
  pc = 0x1067bc9; continue;
 }
 0x1067bc9 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1067bcc; continue;
 }
 0x1067bcc => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067bd5; continue;
 }
 0x1067bd5 => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067bda => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x1067be4; continue; }
  pc = 0x1067be1; continue;
 }
 0x1067be1 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1067be4; continue;
 }
 0x1067be4 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067bed; continue;
 }
 0x1067bed => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067bf2 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067bfa; continue;
 }
 0x1067bfa => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c02; continue;
 }
 0x1067c02 => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067c07 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c0f; continue;
 }
 0x1067c0f => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c17; continue;
 }
 0x1067c17 => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067c1c => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c24; continue;
 }
 0x1067c24 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c2c; continue;
 }
 0x1067c2c => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 0x1067c31 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c39; continue;
 }
 0x1067c39 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { cpu.poll(); pc = 0x1067aee; continue; }
  pc = 0x1067c41; continue;
 }
 0x1067c41 => {
  cpu.poll(); pc = 0x1067b3d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067c48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067c48); } r_01067c48(cpu, 0x1067c48); }
pub fn f_01067c59(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067c59); } r_01067c48(cpu, 0x1067c59); }
pub fn f_01067e21(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067e21); } r_01067c48(cpu, 0x1067e21); }
pub fn f_01067ebf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067ebf); } r_01067c48(cpu, 0x1067ebf); }
pub fn f_01067f20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067f20); } r_01067c48(cpu, 0x1067f20); }
pub fn f_01067f57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1067f57); } r_01067c48(cpu, 0x1067f57); }
fn r_01067c48(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1067c48 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x2710u16); }
  if !cpu.zf { pc = 0x1067c51; continue; }
  pc = 0x1067c50; continue;
 }
 0x1067c50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067c51 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1067c59u32; cpu.edx = v; }
  pc = 0x1067c70; continue;
 }
 0x1067c59 => {
  cpu.unimplemented(0x1067c59, "Arpl");
  cpu.unimplemented(0x1067c5c, "Arpl");
  { cpu.popad(); }
  if cpu.of { pc = 0x1067cc1; continue; }
  pc = 0x1067c62; continue;
 }
 0x1067c62 => {
  if cpu.of { pc = 0x1067cd3; continue; }
  pc = 0x1067c64; continue;
 }
 0x1067c64 => {
  if !cpu.cf { pc = 0x1067cc5; continue; }
  pc = 0x1067c66; continue;
 }
 0x1067c66 => {
  { cpu.popad(); }
  cpu.unimplemented(0x1067c67, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x1067cdc; continue; }
  pc = 0x1067c6c; continue;
 }
 0x1067c6c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1057193u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1067c7f; continue; }
  pc = 0x1067c79; continue;
 }
 0x1067c70 => {
  { let v: u32 = 0x1067c59u32; cpu.w32(0x1057193u32, v); }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067c7f; continue;
 }
 0x1067c79 => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esp); cpu.w32(0xffffff80u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x33u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), 69u32); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x83u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.ror8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 193u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1067c8e); cpu.call(0x5e91e292); if cpu.returned_to != 0x1067c8e { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x1067cbc; continue; }
  pc = 0x1067cac; continue;
 }
 0x1067c7f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x1067cbc; continue; }
  pc = 0x1067cac; continue;
 }
 0x1067cac => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1067cbc; continue; }
  pc = 0x1067cb2; continue;
 }
 0x1067cb2 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1067cbc; continue;
 }
 0x1067cbc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067cc6; continue;
 }
 0x1067cc1 => {
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.esp); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1067cc5; continue;
 }
 0x1067cc5 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.r32(0x40000000u32.wrapping_add(cpu.edi)), cpu.eax); }
  if cpu.zf { pc = 0x1067cef; continue; }
  pc = 0x1067ccf; continue;
 }
 0x1067cc6 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1067cef; continue; }
  pc = 0x1067ccf; continue;
 }
 0x1067ccf => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1067cdd); f_0101acb0(cpu); if cpu.returned_to != 0x1067cdd { return; }
  if cpu.cf { pc = 0x1067ce5; continue; }
  pc = 0x1067cdf; continue;
 }
 0x1067cd3 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x66u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  cpu.push(0x1067cdd); f_0101acb0(cpu); if cpu.returned_to != 0x1067cdd { return; }
  if cpu.cf { pc = 0x1067ce5; continue; }
  pc = 0x1067cdf; continue;
 }
 0x1067cdc => {
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.push(v); }
  pc = 0x1067cdf; continue;
 }
 0x1067cdf => {
  { cpu.popad(); }
  pc = 0x1067de7; continue;
 }
 0x1067ce5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067cef; continue;
 }
 0x1067cef => {
  { let v: u8 = 0x2fu8; cpu.w8(0x112b621u32, v); }
  { let v: u8 = 0x32u8; cpu.w8(0x112b620u32, v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1067d14; continue; }
  pc = 0x1067d06; continue;
 }
 0x1067d06 => {
  { let v: u8 = 0x38u8; cpu.w8(0x112b621u32, v); }
  { let v: u8 = 0x3au8; cpu.w8(0x112b620u32, v); }
  pc = 0x1067d14; continue;
 }
 0x1067d14 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x112b621u32)); }
  if !cpu.zf { pc = 0x1067d31; continue; }
  pc = 0x1067d1c; continue;
 }
 0x1067d1c => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1067d31; continue; }
  pc = 0x1067d25; continue;
 }
 0x1067d25 => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067de7; continue;
 }
 0x1067d31 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1067dc6; continue; }
  pc = 0x1067d3e; continue;
 }
 0x1067d3e => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067d4b; continue;
 }
 0x1067d4b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x112b620u32)); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067d57; continue;
 }
 0x1067d57 => {
  cpu.push(0x1067d5c); f_01067f57(cpu); if cpu.returned_to != 0x1067d5c { return; }
  if !cpu.cf { pc = 0x1067d6e; continue; }
  pc = 0x1067d5e; continue;
 }
 0x1067d5e => {
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067de7; continue;
 }
 0x1067d6e => {
  cpu.push(0x1067d73); f_01067f20(cpu); if cpu.returned_to != 0x1067d73 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xabu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x1067dac; continue; }
  pc = 0x1067d8f; continue;
 }
 0x1067d8f => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1067da3; continue; }
  pc = 0x1067d98; continue;
 }
 0x1067d98 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1067dac; continue;
 }
 0x1067da3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1067dac; continue;
 }
 0x1067dac => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1067de7; continue;
 }
 0x1067dc6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x112b620u32)); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067dce; continue;
 }
 0x1067dce => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067dd7; continue;
 }
 0x1067dd7 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if !cpu.zf { pc = 0x1067de7; continue; }
  pc = 0x1067de0; continue;
 }
 0x1067de0 => {
  { let v: u8 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067de7; continue;
 }
 0x1067de7 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1067eb6; continue; }
  pc = 0x1067df3; continue;
 }
 0x1067df3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1067dfe; continue; }
  pc = 0x1067df9; continue;
 }
 0x1067df9 => {
  cpu.push(0x1067dfe); f_01067eea(cpu); if cpu.returned_to != 0x1067dfe { return; }
  pc = 0x1067dfe; continue;
 }
 0x1067dfe => {
  { let v: u8 = cpu.r8(0x10f57eau32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x6du8); }
  if cpu.zf { pc = 0x1067e11; continue; }
  pc = 0x1067e08; continue;
 }
 0x1067e08 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067e11; continue;
 }
 0x1067e11 => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if (cpu.sf == cpu.of) { pc = 0x1067eb7; continue; }
  pc = 0x1067e19; continue;
 }
 0x1067e19 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1067e21u32; cpu.edx = v; }
  pc = 0x1067e2b; continue;
 }
 0x1067e21 => {
  cpu.unimplemented(0x1067e21, "Arpl");
  cpu.unimplemented(0x1067e24, "Arpl");
  if cpu.cf { pc = 0x1067e8e; continue; }
  pc = 0x1067e29; continue;
 }
 0x1067e29 => {
  if cpu.zf { pc = 0x1067e2b; continue; }
  pc = 0x1067e2b; continue;
 }
 0x1067e2b => {
  { let v: u32 = 0x1067e21u32; cpu.w32(0x1057193u32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1067e52; continue; }
  pc = 0x1067e3c; continue;
 }
 0x1067e3c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1067e4e; continue; }
  pc = 0x1067e48; continue;
 }
 0x1067e48 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1067e77; continue; }
  pc = 0x1067e4e; continue;
 }
 0x1067e4e => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067e77; continue;
 }
 0x1067e52 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1067e6a; continue; }
  pc = 0x1067e5d; continue;
 }
 0x1067e5d => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x8u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1067e6a; continue;
 }
 0x1067e6a => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x20u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1067e77; continue; }
  pc = 0x1067e75; continue;
 }
 0x1067e75 => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067e77; continue;
 }
 0x1067e77 => {
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let a = (cpu.ebx as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1067e99; continue; }
  pc = 0x1067e8b; continue;
 }
 0x1067e8b => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1067e99; continue; }
  pc = 0x1067e97; continue;
 }
 0x1067e8e => {
  cpu.unimplemented(0x1067e8e, "Aas");
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.r32(0x74003d7bu32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x74003d7bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067e99; continue;
 }
 0x1067e97 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067e99; continue;
 }
 0x1067e99 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x89u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1067ea3; continue; }
  pc = 0x1067ea1; continue;
 }
 0x1067ea1 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067ea3; continue;
 }
 0x1067ea3 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.cf { pc = 0x1067ea9; continue; }
  pc = 0x1067ea7; continue;
 }
 0x1067ea7 => {
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1067ea9; continue;
 }
 0x1067ea9 => {
  cpu.push(0x1067eae); f_0106a4c6(cpu); if cpu.returned_to != 0x1067eae { return; }
  cpu.push(0x1067eb3); f_0106a510(cpu); if cpu.returned_to != 0x1067eb3 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1067eb6; continue;
 }
 0x1067eb6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067eb7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1067ebfu32; cpu.edx = v; }
  pc = 0x1067ec9; continue;
 }
 0x1067ebf => {
  cpu.unimplemented(0x1067ebf, "Arpl");
  cpu.unimplemented(0x1067ec2, "Arpl");
  if (!cpu.cf && !cpu.zf) { pc = 0x1067f30; continue; }
  pc = 0x1067ec7; continue;
 }
 0x1067ec7 => {
  cpu.unimplemented(0x1067ec7, "Outsb");
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { let v: u32 = 0x6101067eu32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x64u32); cpu.esi = v; }
  let t = cpu.r32(0x112b624u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.push(0x1067ee5); cpu.call(t); if cpu.returned_to != 0x1067ee5 { return; }
  cpu.poll(); pc = 0x1067e19; continue;
 }
 0x1067ec9 => {
  { let v: u32 = 0x1067ebfu32; cpu.w32(0x1057193u32, v); }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x64u32); cpu.esi = v; }
  let t = cpu.r32(0x112b624u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.push(0x1067ee5); cpu.call(t); if cpu.returned_to != 0x1067ee5 { return; }
  cpu.poll(); pc = 0x1067e19; continue;
 }
 0x1067f20 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if !cpu.zf { pc = 0x1067f2b; continue; }
  pc = 0x1067f2a; continue;
 }
 0x1067f2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067f2b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.zf { pc = 0x1067f55; continue; }
  pc = 0x1067f47; continue;
 }
 0x1067f30 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x1067f6e; continue; }
  pc = 0x1067f32; continue;
 }
 0x1067f32 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.r32(0xb8u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.edi); cpu.w32(0xb8u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.zf { pc = 0x1067f55; continue; }
  pc = 0x1067f47; continue;
 }
 0x1067f47 => {
  cpu.push(0x1067f4c); f_01057945(cpu); if cpu.returned_to != 0x1067f4c { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3debu32, v); }
  pc = 0x1067f55; continue;
 }
 0x1067f55 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067f57 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1067f75; continue; }
  pc = 0x1067f64; continue;
 }
 0x1067f64 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x1067f85; continue; }
  pc = 0x1067f6a; continue;
 }
 0x1067f6a => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.edi)), 0x8u8); }
  pc = 0x1067f6e; continue;
 }
 0x1067f6e => {
  if !cpu.zf { pc = 0x1067f85; continue; }
  pc = 0x1067f70; continue;
 }
 0x1067f70 => {
  pc = 0x1067ff9; continue;
 }
 0x1067f75 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x1067f85; continue; }
  pc = 0x1067f7d; continue;
 }
 0x1067f7d => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x1067ff9; continue; }
  pc = 0x1067f83; continue;
 }
 0x1067f83 => {
  pc = 0x1067ffc; continue;
 }
 0x1067f85 => {
  { cpu.pushad(); }
  cpu.push(0x1067f8b); f_0106a326(cpu); if cpu.returned_to != 0x1067f8b { return; }
  cpu.push(0x1067f90); f_010675fd(cpu); if cpu.returned_to != 0x1067f90 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x1067fa2); f_01047379(cpu); if cpu.returned_to != 0x1067fa2 { return; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1067fb1; continue; }
  pc = 0x1067fa8; continue;
 }
 0x1067fa8 => {
  cpu.push(0x1067fad); f_0108a641(cpu); if cpu.returned_to != 0x1067fad { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x1067fb1; continue;
 }
 0x1067fb1 => {
  { cpu.popad(); }
  cpu.push(0x1067fb7); f_01068063(cpu); if cpu.returned_to != 0x1067fb7 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  cpu.push(0x1067fdf); f_01036a75(cpu); if cpu.returned_to != 0x1067fdf { return; }
  { cpu.pushad(); }
  pc = 0x1067fe0; continue;
 }
 0x1067fe0 => {
  { cpu.popad(); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067ff9 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067ffc => {
  { cpu.pushad(); }
  cpu.push(0x1068002); f_0106a326(cpu); if cpu.returned_to != 0x1068002 { return; }
  cpu.push(0x1068007); f_010675fd(cpu); if cpu.returned_to != 0x1068007 { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x1068019); f_01047379(cpu); if cpu.returned_to != 0x1068019 { return; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1068028; continue; }
  pc = 0x106801f; continue;
 }
 0x106801f => {
  cpu.push(0x1068024); f_0108a641(cpu); if cpu.returned_to != 0x1068024 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x1068028; continue;
 }
 0x1068028 => {
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  cpu.push(0x106805d); f_01036a75(cpu); if cpu.returned_to != 0x106805d { return; }
  { cpu.pushad(); }
  cpu.poll(); pc = 0x1067fe0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01067eea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1067eea); }
 let mut pc: u32 = 0x1067eea;
 loop { match pc {
 0x1067eea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x1067f18; continue; }
  pc = 0x1067ef4; continue;
 }
 0x1067ef4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x1067f18; continue; }
  pc = 0x1067efd; continue;
 }
 0x1067efd => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x1067f18; continue; }
  pc = 0x1067f09; continue;
 }
 0x1067f09 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x1067f18; continue; }
  pc = 0x1067f15; continue;
 }
 0x1067f15 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1067f18 => {
  cpu.push(0x1067f1d); f_01068390(cpu); if cpu.returned_to != 0x1067f1d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068063(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068063); }
 let mut pc: u32 = 0x1068063;
 loop { match pc {
 0x1068063 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10680a8; continue; }
  pc = 0x106806e; continue;
 }
 0x106806e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10680bd; continue; }
  pc = 0x1068072; continue;
 }
 0x1068072 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x106808b; continue; }
  pc = 0x1068076; continue;
 }
 0x1068076 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10680d8; continue;
 }
 0x106808b => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10680d8; continue;
 }
 0x10680a8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10680d8; continue;
 }
 0x10680bd => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10680d8; continue;
 }
 0x10680d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010680d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10680d9); }
 let mut pc: u32 = 0x10680d9;
 loop { match pc {
 0x10680d9 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x106819b; continue; }
  pc = 0x10680e9; continue;
 }
 0x10680e9 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1068107; continue; }
  pc = 0x10680fc; continue;
 }
 0x10680fc => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1068107; continue;
 }
 0x1068107 => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1068111; continue;
 }
 0x1068111 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x1068191; continue; }
  pc = 0x1068118; continue;
 }
 0x1068118 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x1068191; continue; }
  pc = 0x106811e; continue;
 }
 0x106811e => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1068129; continue; }
  pc = 0x1068126; continue;
 }
 0x1068126 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1068129; continue;
 }
 0x1068129 => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x1068191; continue; }
  pc = 0x106812e; continue;
 }
 0x106812e => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106813a; continue; }
  pc = 0x1068137; continue;
 }
 0x1068137 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106813a; continue;
 }
 0x106813a => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x1068191; continue; }
  pc = 0x106813f; continue;
 }
 0x106813f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10c5a6eu32); }
  if cpu.zf { pc = 0x1068152; continue; }
  pc = 0x106814a; continue;
 }
 0x106814a => {
  { cpu.sub32(cpu.edi, 0x10c5a55u32); }
  if !cpu.zf { pc = 0x1068191; continue; }
  pc = 0x1068152; continue;
 }
 0x1068152 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1068164; continue; }
  pc = 0x1068158; continue;
 }
 0x1068158 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1068164; continue; }
  pc = 0x106815e; continue;
 }
 0x106815e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106819b; continue; }
  pc = 0x1068164; continue;
 }
 0x1068164 => {
  cpu.push(0x1068169); f_01068d2a(cpu); if cpu.returned_to != 0x1068169 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x106819b; continue; }
  pc = 0x106816d; continue;
 }
 0x106816d => {
  cpu.push(0x1068172); f_0106819e(cpu); if cpu.returned_to != 0x1068172 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1068180); f_0104f36d(cpu); if cpu.returned_to != 0x1068180 { return; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068191 => {
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1068111; continue; }
  pc = 0x106819b; continue;
 }
 0x106819b => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106819e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106819e); }
 let mut pc: u32 = 0x106819e;
 loop { match pc {
 0x106819e => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x10c5a6eu32); }
  if !cpu.zf { pc = 0x10681bc; continue; }
  pc = 0x10681ba; continue;
 }
 0x10681ba => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10681bc; continue;
 }
 0x10681bc => {
  { let v = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10681c6; continue;
 }
 0x10681c6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10681e7; continue; }
  pc = 0x10681cb; continue;
 }
 0x10681cb => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x1152ee9u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x1152ee9u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x1152ee9u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0x1152ee9u32.wrapping_add(cpu.eax), v); }
  if !cpu.sf { pc = 0x10681e7; continue; }
  pc = 0x10681e0; continue;
 }
 0x10681e0 => {
  { let v: u8 = 0x7fu8; cpu.w8(0x1152ee9u32.wrapping_add(cpu.eax), v); }
  pc = 0x10681e7; continue;
 }
 0x10681e7 => {
  { let v: u8 = cpu.xor8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10681c6; continue; }
  pc = 0x10681ee; continue;
 }
 0x10681ee => {
  cpu.push(0x10681f3); f_010364a8(cpu); if cpu.returned_to != 0x10681f3 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010681f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10681f5); }
 let mut pc: u32 = 0x10681f5;
 loop { match pc {
 0x10681f5 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x106821d; continue; }
  pc = 0x10681fd; continue;
 }
 0x10681fd => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x1068220; continue; }
  pc = 0x1068207; continue;
 }
 0x1068207 => {
  { cpu.sub8((cpu.eax as u8), 0x6fu8); }
  if cpu.zf { pc = 0x1068230; continue; }
  pc = 0x106820b; continue;
 }
 0x106820b => {
  { cpu.sub8((cpu.eax as u8), 0x70u8); }
  if cpu.zf { pc = 0x1068240; continue; }
  pc = 0x106820f; continue;
 }
 0x106820f => {
  { cpu.sub8(cpu.r8(0x10fd43eu32), 0x0u8); }
  if cpu.zf { pc = 0x106821d; continue; }
  pc = 0x1068218; continue;
 }
 0x1068218 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1068250; continue; }
  pc = 0x106821d; continue;
 }
 0x106821d => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068220 => {
  { cpu.sub8(cpu.r8(0x10fd43du32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x106821d; continue; }
  pc = 0x1068229; continue;
 }
 0x1068229 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x106825d; continue; }
  pc = 0x106822e; continue;
 }
 0x106822e => {
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x1068230 => {
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x106821d; continue; }
  pc = 0x1068239; continue;
 }
 0x1068239 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x106826a; continue; }
  pc = 0x106823e; continue;
 }
 0x106823e => {
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x1068240 => {
  { cpu.sub8(cpu.r8(0x10fd440u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x106821d; continue; }
  pc = 0x1068249; continue;
 }
 0x1068249 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if !cpu.zf { pc = 0x1068277; continue; }
  pc = 0x106824e; continue;
 }
 0x106824e => {
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x1068250 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x106825b); f_01002935(cpu); if cpu.returned_to != 0x106825b { return; }
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x106825d => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x1068268); f_01002832(cpu); if cpu.returned_to != 0x1068268 { return; }
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x106826a => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x1068275); f_01002a3a(cpu); if cpu.returned_to != 0x1068275 { return; }
  cpu.poll(); pc = 0x106821d; continue;
 }
 0x1068277 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  cpu.push(0x1068282); f_01002b38(cpu); if cpu.returned_to != 0x1068282 { return; }
  cpu.poll(); pc = 0x106821d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068287(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068287); }
 let mut pc: u32 = 0x1068287;
 loop { match pc {
 0x1068287 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x10682d3; continue; }
  pc = 0x106828f; continue;
 }
 0x106828f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x10682d3; continue; }
  pc = 0x106829e; continue;
 }
 0x106829e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10682d3; continue; }
  pc = 0x10682a4; continue;
 }
 0x10682a4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10682b2); f_01018cce(cpu); if cpu.returned_to != 0x10682b2 { return; }
  if !cpu.cf { pc = 0x10682d6; continue; }
  pc = 0x10682b4; continue;
 }
 0x10682b4 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10682c5; continue; }
  pc = 0x10682c2; continue;
 }
 0x10682c2 => {
  { cpu.popad(); }
  pc = 0x10682d3; continue;
 }
 0x10682c5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.popad(); }
  cpu.push(0x10682d3); f_01068531(cpu); if cpu.returned_to != 0x10682d3 { return; }
  pc = 0x10682d3; continue;
 }
 0x10682d3 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10682d6 => {
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010682da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10682da); }
 let mut pc: u32 = 0x10682da;
 loop { match pc {
 0x10682da => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010682dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10682dd); }
 let mut pc: u32 = 0x10682dd;
 loop { match pc {
 0x10682dd => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1068324; continue; }
  pc = 0x10682e5; continue;
 }
 0x10682e5 => {
  { let v: u32 = 0x81u32; cpu.w32(0x10e40c0u32, v); }
  cpu.push(0x10682f4); f_01068659(cpu); if cpu.returned_to != 0x10682f4 { return; }
  if cpu.cf { pc = 0x10682f9; continue; }
  pc = 0x10682f6; continue;
 }
 0x10682f6 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10682f9 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x1068324; continue; }
  pc = 0x1068307; continue;
 }
 0x1068307 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  cpu.push(0x1068312); f_01047379(cpu); if cpu.returned_to != 0x1068312 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1068323); f_0106a04e(cpu); if cpu.returned_to != 0x1068323 { return; }
  { cpu.popad(); }
  pc = 0x1068324; continue;
 }
 0x1068324 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068327(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068327); }
 let mut pc: u32 = 0x1068327;
 loop { match pc {
 0x1068327 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106832a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106832a); }
 let mut pc: u32 = 0x106832a;
 loop { match pc {
 0x106832a => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106832d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106832d); }
 let mut pc: u32 = 0x106832d;
 loop { match pc {
 0x106832d => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x106834e; continue; }
  pc = 0x1068335; continue;
 }
 0x1068335 => {
  { let v: u32 = 0x81u32; cpu.w32(0x10e40c0u32, v); }
  cpu.push(0x1068344); f_01068659(cpu); if cpu.returned_to != 0x1068344 { return; }
  if cpu.cf { pc = 0x1068349; continue; }
  pc = 0x1068346; continue;
 }
 0x1068346 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068349 => {
  cpu.push(0x106834e); f_0106838f(cpu); if cpu.returned_to != 0x106834e { return; }
  pc = 0x106834e; continue;
 }
 0x106834e => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068351(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068351); }
 let mut pc: u32 = 0x1068351;
 loop { match pc {
 0x1068351 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8cu32.wrapping_add(cpu.ebp))); cpu.w16(0x8cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x8cu32.wrapping_add(cpu.ebp)), 0xc8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106837e; continue; }
  pc = 0x1068367; continue;
 }
 0x1068367 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11296abu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1068376); f_01050a52(cpu); if cpu.returned_to != 0x1068376 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296abu32, v); }
  pc = 0x106837e; continue;
 }
 0x106837e => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068381); }
 let mut pc: u32 = 0x1068381;
 loop { match pc {
 0x1068381 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11296abu32, v); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106838f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106838f); }
 let mut pc: u32 = 0x106838f;
 loop { match pc {
 0x106838f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068390); }
 let mut pc: u32 = 0x1068390;
 loop { match pc {
 0x1068390 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1068480; continue; }
  pc = 0x106839c; continue;
 }
 0x106839c => {
  { let v: u8 = cpu.or8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10683a5); f_010684bb(cpu); if cpu.returned_to != 0x10683a5 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10683b2); f_0108c1c5(cpu); if cpu.returned_to != 0x10683b2 { return; }
  if cpu.cf { pc = 0x1068480; continue; }
  pc = 0x10683b8; continue;
 }
 0x10683b8 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x1d3u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1068480; continue; }
  pc = 0x10683c4; continue;
 }
 0x10683c4 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcc46u32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x1068480; continue; }
  pc = 0x10683da; continue;
 }
 0x10683da => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  cpu.push(0x10683e6); f_010179e0(cpu); if cpu.returned_to != 0x10683e6 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x1068418; continue; }
  pc = 0x10683f1; continue;
 }
 0x10683f1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11eb12eu32, v); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcc46u32), 0x4u8); }
  if cpu.zf { pc = 0x106841d; continue; }
  pc = 0x1068404; continue;
 }
 0x1068404 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10fcc46u32)); }
  if !cpu.zf { pc = 0x106841d; continue; }
  pc = 0x1068411; continue;
 }
 0x1068411 => {
  cpu.push(0x1068416); f_01036364(cpu); if cpu.returned_to != 0x1068416 { return; }
  pc = 0x106841d; continue;
 }
 0x1068418 => {
  cpu.push(0x106841d); f_01036324(cpu); if cpu.returned_to != 0x106841d { return; }
  pc = 0x106841d; continue;
 }
 0x106841d => {
  { let v: u16 = 0x1feu16; cpu.w16(0x16u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106842a); f_01088003(cpu); if cpu.returned_to != 0x106842a { return; }
  cpu.push(0x106842f); f_0108bb0f(cpu); if cpu.returned_to != 0x106842f { return; }
  cpu.push(0x1068434); f_0101b533(cpu); if cpu.returned_to != 0x1068434 { return; }
  cpu.push(0x1068439); f_0101b263(cpu); if cpu.returned_to != 0x1068439 { return; }
  cpu.push(0x106843e); f_010268cc(cpu); if cpu.returned_to != 0x106843e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x1068444); f_01089e45(cpu); if cpu.returned_to != 0x1068444 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x106844a); f_01052350(cpu); if cpu.returned_to != 0x106844a { return; }
  { cpu.sub8(cpu.r8(0x5bu32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x1068480; continue; }
  pc = 0x1068450; continue;
 }
 0x1068450 => {
  { cpu.sub16(cpu.r16(0x36u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1068480; continue; }
  pc = 0x1068458; continue;
 }
 0x1068458 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11a188cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.push(0x106846b); f_0106977b(cpu); if cpu.returned_to != 0x106846b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1068480; continue; }
  pc = 0x106847b; continue;
 }
 0x106847b => {
  cpu.push(0x1068480); f_0106a326(cpu); if cpu.returned_to != 0x1068480 { return; }
  pc = 0x1068480; continue;
 }
 0x1068480 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068483); }
 let mut pc: u32 = 0x1068483;
 loop { match pc {
 0x1068483 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x106848e; continue;
 }
 0x106848e => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10684b3; continue; }
  pc = 0x1068495; continue;
 }
 0x1068495 => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if !cpu.zf { pc = 0x10684b3; continue; }
  pc = 0x106849a; continue;
 }
 0x106849a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10684b3; continue; }
  pc = 0x10684a2; continue;
 }
 0x10684a2 => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x10684b3; continue; }
  pc = 0x10684a8; continue;
 }
 0x10684a8 => {
  { let v: u8 = 0x4u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x10684b2); f_0102f75b(cpu); if cpu.returned_to != 0x10684b2 { return; }
  { cpu.popad(); }
  pc = 0x10684b3; continue;
 }
 0x10684b3 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x106848e; continue; }
  pc = 0x10684b9; continue;
 }
 0x10684b9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010684bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10684bb); }
 let mut pc: u32 = 0x10684bb;
 loop { match pc {
 0x10684bb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10684c9); f_0108c1c5(cpu); if cpu.returned_to != 0x10684c9 { return; }
  if cpu.cf { pc = 0x106852f; continue; }
  pc = 0x10684cb; continue;
 }
 0x10684cb => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x10684dd; continue;
 }
 0x10684dd => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1068526; continue; }
  pc = 0x10684e4; continue;
 }
 0x10684e4 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1068526; continue; }
  pc = 0x10684ea; continue;
 }
 0x10684ea => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x1068526; continue; }
  pc = 0x10684f3; continue;
 }
 0x10684f3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1068501); f_0108c1c5(cpu); if cpu.returned_to != 0x1068501 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x1068526; continue; }
  pc = 0x1068506; continue;
 }
 0x1068506 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.edi)) & 0xfbu8; cpu.w8(0x87u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1068520; continue; }
  pc = 0x1068519; continue;
 }
 0x1068519 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x1068520); f_010882f3(cpu); if cpu.returned_to != 0x1068520 { return; }
  pc = 0x1068520; continue;
 }
 0x1068520 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x1068526; continue;
 }
 0x1068526 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10684dd; continue; }
  pc = 0x106852f; continue;
 }
 0x106852f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068531(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068531); }
 let mut pc: u32 = 0x1068531;
 loop { match pc {
 0x1068531 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x81u32; cpu.w32(0x10e40c0u32, v); }
  cpu.push(0x1068542); f_01068659(cpu); if cpu.returned_to != 0x1068542 { return; }
  if cpu.cf { pc = 0x1068547; continue; }
  pc = 0x1068544; continue;
 }
 0x1068544 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068547 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10685d7; continue; }
  pc = 0x1068559; continue;
 }
 0x1068559 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x106856f); f_01047379(cpu); if cpu.returned_to != 0x106856f { return; }
  cpu.push(0x1068574); f_0106a326(cpu); if cpu.returned_to != 0x1068574 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x10685bf); f_01059dbe(cpu); if cpu.returned_to != 0x10685bf { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10685c5); f_01036a75(cpu); if cpu.returned_to != 0x10685c5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10685d7 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10685ee; continue; }
  pc = 0x10685e0; continue;
 }
 0x10685e0 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10685f3; continue; }
  pc = 0x10685ee; continue;
 }
 0x10685ee => {
  cpu.push(0x10685f3); f_01036398(cpu); if cpu.returned_to != 0x10685f3 { return; }
  pc = 0x10685f3; continue;
 }
 0x10685f3 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x1152e54u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.eax = v; }
  { let v: u32 = 0x14u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x1068645; continue; }
  pc = 0x1068640; continue;
 }
 0x1068640 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1068645; continue;
 }
 0x1068645 => {
  cpu.push(0x106864a); f_010316f5(cpu); if cpu.returned_to != 0x106864a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1068650); f_01067f20(cpu); if cpu.returned_to != 0x1068650 { return; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068659); }
 let mut pc: u32 = 0x1068659;
 loop { match pc {
 0x1068659 => {
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1068663; continue;
 }
 0x1068663 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1068671); f_0104f36d(cpu); if cpu.returned_to != 0x1068671 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e40c0u32)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x106868d; continue; }
  pc = 0x1068686; continue;
 }
 0x1068686 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1068663; continue; }
  pc = 0x106868b; continue;
 }
 0x106868b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106868d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106868f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106868f); }
 let mut pc: u32 = 0x106868f;
 loop { match pc {
 0x106868f => {
  cpu.push(0x1068694); f_01067c48(cpu); if cpu.returned_to != 0x1068694 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10686a7; continue; }
  pc = 0x10686a5; continue;
 }
 0x10686a5 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10686a7; continue;
 }
 0x10686a7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x61u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010686ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10686ab); }
 let mut pc: u32 = 0x10686ab;
 loop { match pc {
 0x10686ab => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
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
  cpu.push(0x10686d4); f_0104f36d(cpu); if cpu.returned_to != 0x10686d4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcb00u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x106882e; continue; }
  pc = 0x10686f1; continue;
 }
 0x10686f1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcb00u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x106882e; continue; }
  pc = 0x1068704; continue;
 }
 0x1068704 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcb00u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x106882e; continue; }
  pc = 0x106871a; continue;
 }
 0x106871a => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcb00u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x106882e; continue; }
  pc = 0x1068730; continue;
 }
 0x1068730 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  pc = 0x106873a; continue;
 }
 0x106873a => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x106878d; continue; }
  pc = 0x1068740; continue;
 }
 0x1068740 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x106878d; continue; }
  pc = 0x1068746; continue;
 }
 0x1068746 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x10e40c0u32)); }
  if cpu.zf { pc = 0x106878d; continue; }
  pc = 0x106874e; continue;
 }
 0x106874e => {
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x11294e8u32.wrapping_add(cpu.edx)) as u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.edx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106878d; continue; }
  pc = 0x106876b; continue;
 }
 0x106876b => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.edx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.esi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106878d; continue; }
  pc = 0x1068777; continue;
 }
 0x1068777 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x106878d; continue; }
  pc = 0x1068780; continue;
 }
 0x1068780 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x106882e; continue; }
  pc = 0x106878d; continue;
 }
 0x106878d => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x106873a; continue; }
  pc = 0x1068796; continue;
 }
 0x1068796 => {
  cpu.push(0x106879b); f_01068891(cpu); if cpu.returned_to != 0x106879b { return; }
  if cpu.cf { pc = 0x106882e; continue; }
  pc = 0x10687a1; continue;
 }
 0x10687a1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10687ae); f_0104f36d(cpu); if cpu.returned_to != 0x10687ae { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10687d7; continue; }
  pc = 0x10687c3; continue;
 }
 0x10687c3 => {
  { let v: u32 = cpu.r32(0x11b51b4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10687d7; continue; }
  pc = 0x10687ce; continue;
 }
 0x10687ce => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b51b8u32, v); }
  pc = 0x10687e5; continue;
 }
 0x10687d7 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x10687e5; continue; }
  pc = 0x10687dd; continue;
 }
 0x10687dd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1068802; continue;
 }
 0x10687e5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x1068802; continue;
 }
 0x1068802 => {
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
 0x106882e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x112b65au32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  cpu.push(0x1068852); f_010738df(cpu); if cpu.returned_to != 0x1068852 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  cpu.push(0x1068863); f_010738df(cpu); if cpu.returned_to != 0x1068863 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  cpu.push(0x1068874); f_010738df(cpu); if cpu.returned_to != 0x1068874 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xcu32); cpu.ebx = v; }
  cpu.push(0x1068885); f_010738df(cpu); if cpu.returned_to != 0x1068885 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x112b65au32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068891(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068891); }
 let mut pc: u32 = 0x1068891;
 loop { match pc {
 0x1068891 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x106889d; continue;
 }
 0x106889d => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10688ce; continue; }
  pc = 0x10688b9; continue;
 }
 0x10688b9 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10688ce; continue; }
  pc = 0x10688bd; continue;
 }
 0x10688bd => {
  { let v: u32 = cpu.edx.wrapping_sub(0x20u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x20u32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10688ce; continue; }
  pc = 0x10688c7; continue;
 }
 0x10688c7 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10688ce; continue; }
  pc = 0x10688cb; continue;
 }
 0x10688cb => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10688ce => {
  { let v: u32 = cpu.add32(cpu.esi, 0x88u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106889d; continue; }
  pc = 0x10688d6; continue;
 }
 0x10688d6 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010688d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10688d9); }
 let mut pc: u32 = 0x10688d9;
 loop { match pc {
 0x10688d9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef359u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1128afdu32, v); }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x10688ef); f_01054174(cpu); if cpu.returned_to != 0x10688ef { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if !cpu.cf { pc = 0x106894b; continue; }
  pc = 0x1068901; continue;
 }
 0x1068901 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1068942; continue; }
  pc = 0x106891f; continue;
 }
 0x106891f => {
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1068934); f_01018cce(cpu); if cpu.returned_to != 0x1068934 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x1068942; continue; }
  pc = 0x1068938; continue;
 }
 0x1068938 => {
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x106894a; continue;
 }
 0x1068942 => {
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x106894a; continue;
 }
 0x106894a => {
  { cpu.popad(); }
  pc = 0x106894b; continue;
 }
 0x106894b => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x36u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32).wrapping_sub(0x7u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_sub(0x7u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_sub(0x8u16); cpu.w16(0x10e3dbdu32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1068986); f_010689d4(cpu); if cpu.returned_to != 0x1068986 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32).wrapping_add(0x7u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x7u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add(0x8u16); cpu.w16(0x10e3dbdu32, v); }
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
  if cpu.cf { pc = 0x10689d3; continue; }
  pc = 0x10689c5; continue;
 }
 0x10689c5 => {
  { cpu.sub8(cpu.r8(0x1128afdu32), 0x0u8); }
  if !cpu.zf { pc = 0x10689d2; continue; }
  pc = 0x10689ce; continue;
 }
 0x10689ce => {
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10689d2; continue;
 }
 0x10689d2 => {
  { cpu.cf = false; }
  pc = 0x10689d3; continue;
 }
 0x10689d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010689d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10689d4); } r_010689d4(cpu, 0x10689d4); }
pub fn f_01068a46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068a46); } r_010689d4(cpu, 0x1068a46); }
pub fn f_01068a98(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068a98); } r_010689d4(cpu, 0x1068a98); }
pub fn f_01068ac5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068ac5); } r_010689d4(cpu, 0x1068ac5); }
pub fn f_01068b48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068b48); } r_010689d4(cpu, 0x1068b48); }
fn r_010689d4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10689d4 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40d8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40dcu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1068a3c; continue; }
  pc = 0x1068a14; continue;
 }
 0x1068a14 => {
  if cpu.sf { pc = 0x1068a29; continue; }
  pc = 0x1068a16; continue;
 }
 0x1068a16 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1068ac5; continue; }
  pc = 0x1068a1e; continue;
 }
 0x1068a1e => {
  if !cpu.sf { pc = 0x1068b84; continue; }
  pc = 0x1068a24; continue;
 }
 0x1068a24 => {
  pc = 0x1068ba8; continue;
 }
 0x1068a29 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1068b48; continue; }
  pc = 0x1068a31; continue;
 }
 0x1068a31 => {
  if !cpu.sf { pc = 0x1068bcc; continue; }
  pc = 0x1068a37; continue;
 }
 0x1068a37 => {
  pc = 0x1068bf0; continue;
 }
 0x1068a3c => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1068a44; continue; }
  pc = 0x1068a40; continue;
 }
 0x1068a40 => {
  if !cpu.sf { pc = 0x1068a98; continue; }
  pc = 0x1068a42; continue;
 }
 0x1068a42 => {
  pc = 0x1068a46; continue;
 }
 0x1068a44 => {
  { cpu.cf = false; }
  pc = 0x1068a45; continue;
 }
 0x1068a45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068a46 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068a4f; continue;
 }
 0x1068a4f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1068a75; continue; }
  pc = 0x1068a58; continue;
 }
 0x1068a58 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1068a75; continue; }
  pc = 0x1068a62; continue;
 }
 0x1068a62 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068a6b; continue;
 }
 0x1068a6b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068a75; continue;
 }
 0x1068a75 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x1068a84; continue; }
  pc = 0x1068a79; continue;
 }
 0x1068a79 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1068a8c; continue;
 }
 0x1068a84 => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1068a8c; continue;
 }
 0x1068a8c => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40dcu32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x1068a45; continue;
 }
 0x1068a98 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068aa1; continue;
 }
 0x1068aa1 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068aaa; continue;
 }
 0x1068aaa => {
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1068a75; continue; }
  pc = 0x1068ab9; continue;
 }
 0x1068ab9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1068a75; continue; }
  pc = 0x1068ac3; continue;
 }
 0x1068ac3 => {
  cpu.poll(); pc = 0x1068a62; continue;
 }
 0x1068ac5 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068ad2; continue;
 }
 0x1068ad2 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068adf; continue;
 }
 0x1068adf => {
  { let v: u32 = cpu.edi.wrapping_add(0x82u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1068b2c; continue; }
  pc = 0x1068aee; continue;
 }
 0x1068aee => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { pc = 0x1068b0c; continue; }
  pc = 0x1068af7; continue;
 }
 0x1068af7 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1068b2c; continue; }
  pc = 0x1068b06; continue;
 }
 0x1068b06 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  pc = 0x1068b0c; continue;
 }
 0x1068b0c => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068b19; continue;
 }
 0x1068b19 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068b2c; continue;
 }
 0x1068b2c => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1068b35; continue; }
  pc = 0x1068b30; continue;
 }
 0x1068b30 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1068b35; continue;
 }
 0x1068b35 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40d8u32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x1068a45; continue;
 }
 0x1068b48 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x1068a44; continue; }
  pc = 0x1068b55; continue;
 }
 0x1068b55 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1068b2c; continue; }
  pc = 0x1068b64; continue;
 }
 0x1068b64 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1068b0c; continue; }
  pc = 0x1068b6d; continue;
 }
 0x1068b6d => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1068b2c; continue; }
  pc = 0x1068b7c; continue;
 }
 0x1068b7c => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  cpu.poll(); pc = 0x1068b0c; continue;
 }
 0x1068b84 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1068b92); f_01068ac5(cpu); if cpu.returned_to != 0x1068b92 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1068b98); f_01068a98(cpu); if cpu.returned_to != 0x1068b98 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1068ba6; continue; }
  pc = 0x1068ba4; continue;
 }
 0x1068ba4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068ba6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068ba8 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1068bb6); f_01068ac5(cpu); if cpu.returned_to != 0x1068bb6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1068bbc); f_01068a46(cpu); if cpu.returned_to != 0x1068bbc { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1068bca; continue; }
  pc = 0x1068bc8; continue;
 }
 0x1068bc8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068bca => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068bcc => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1068bda); f_01068b48(cpu); if cpu.returned_to != 0x1068bda { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1068be0); f_01068a98(cpu); if cpu.returned_to != 0x1068be0 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1068bee; continue; }
  pc = 0x1068bec; continue;
 }
 0x1068bec => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068bee => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068bf0 => {
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1068bfe); f_01068b48(cpu); if cpu.returned_to != 0x1068bfe { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x1068c04); f_01068a46(cpu); if cpu.returned_to != 0x1068c04 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1068c12; continue; }
  pc = 0x1068c10; continue;
 }
 0x1068c10 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068c12 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068c14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068c14); }
 let mut pc: u32 = 0x1068c14;
 loop { match pc {
 0x1068c14 => {
  cpu.push(0x1068c19); f_01068f64(cpu); if cpu.returned_to != 0x1068c19 { return; }
  if cpu.cf { pc = 0x1068d29; continue; }
  pc = 0x1068c1f; continue;
 }
 0x1068c1f => {
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
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
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f6391u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f66b6u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1068cf2); f_01068d78(cpu); if cpu.returned_to != 0x1068cf2 { return; }
  cpu.push(0x1068cf7); f_0106868f(cpu); if cpu.returned_to != 0x1068cf7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee97cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee97cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10ee920u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x10ee97cu32), 0x0u8); }
  if cpu.zf { pc = 0x1068d29; continue; }
  pc = 0x1068d25; continue;
 }
 0x1068d25 => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1068d29; continue;
 }
 0x1068d29 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068d2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068d2a); }
 let mut pc: u32 = 0x1068d2a;
 loop { match pc {
 0x1068d2a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x1068d40; continue;
 }
 0x1068d40 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1068d55; continue; }
  pc = 0x1068d45; continue;
 }
 0x1068d45 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x1152ee9u32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  pc = 0x1068d55; continue;
 }
 0x1068d55 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1068d40; continue; }
  pc = 0x1068d5a; continue;
 }
 0x1068d5a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068d61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068d61); }
 let mut pc: u32 = 0x1068d61;
 loop { match pc {
 0x1068d61 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10ee920u32); cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x1068d76; continue; }
  pc = 0x1068d73; continue;
 }
 0x1068d73 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1068d76; continue;
 }
 0x1068d76 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068d78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068d78); }
 let mut pc: u32 = 0x1068d78;
 loop { match pc {
 0x1068d78 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068d92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068d92); }
 let mut pc: u32 = 0x1068d92;
 loop { match pc {
 0x1068d92 => {
  cpu.push(0x1068d97); f_01068db3(cpu); if cpu.returned_to != 0x1068d97 { return; }
  if cpu.cf { pc = 0x1068dad; continue; }
  pc = 0x1068d99; continue;
 }
 0x1068d99 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c941u32, v); }
  cpu.push(0x1068da5); f_010504cc(cpu); if cpu.returned_to != 0x1068da5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c941u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068dad => {
  cpu.push(0x1068db2); f_01039a6e(cpu); if cpu.returned_to != 0x1068db2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068db3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068db3); } r_01068db3(cpu, 0x1068db3); }
pub fn f_01068dfc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1068dfc); } r_01068db3(cpu, 0x1068dfc); }
fn r_01068db3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1068db3 => {
  { let v: u32 = 0x112b65cu32; cpu.w32(0x10e404cu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1068de5; continue; }
  pc = 0x1068dd8; continue;
 }
 0x1068dd8 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1068de5; continue;
 }
 0x1068de5 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1068e67; continue; }
  pc = 0x1068dee; continue;
 }
 0x1068dee => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1068dfc; continue;
 }
 0x1068dfc => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(0x2u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x2u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1068e17; continue;
 }
 0x1068e17 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1068e5f; continue; }
  pc = 0x1068e1d; continue;
 }
 0x1068e1d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x1068e5f; continue; }
  pc = 0x1068e23; continue;
 }
 0x1068e23 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1068e5f; continue; }
  pc = 0x1068e2c; continue;
 }
 0x1068e2c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1068e3b; continue; }
  pc = 0x1068e36; continue;
 }
 0x1068e36 => {
  cpu.push(0x1068e3b); f_01068f25(cpu); if cpu.returned_to != 0x1068e3b { return; }
  pc = 0x1068e3b; continue;
 }
 0x1068e3b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1068e5b; continue; }
  pc = 0x1068e44; continue;
 }
 0x1068e44 => {
  { cpu.sub16((cpu.esi as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1068e5b; continue; }
  pc = 0x1068e49; continue;
 }
 0x1068e49 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1068e5b; continue; }
  pc = 0x1068e4f; continue;
 }
 0x1068e4f => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x1068e5b; continue; }
  pc = 0x1068e55; continue;
 }
 0x1068e55 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068e5b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1068e5f; continue;
 }
 0x1068e5f => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1068e17; continue; }
  pc = 0x1068e67; continue;
 }
 0x1068e67 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068e69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068e69); }
 let mut pc: u32 = 0x1068e69;
 loop { match pc {
 0x1068e69 => {
  { let v: u32 = 0x112b65cu32; cpu.w32(0x10e404cu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1068e9b; continue; }
  pc = 0x1068e8e; continue;
 }
 0x1068e8e => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1068e9b; continue;
 }
 0x1068e9b => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1068f23; continue; }
  pc = 0x1068ea4; continue;
 }
 0x1068ea4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(0x2u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x2u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1068ecd; continue;
 }
 0x1068ecd => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1068f1b; continue; }
  pc = 0x1068ed3; continue;
 }
 0x1068ed3 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x1068f1b; continue; }
  pc = 0x1068ed9; continue;
 }
 0x1068ed9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1068f1b; continue; }
  pc = 0x1068edf; continue;
 }
 0x1068edf => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1068f1b; continue; }
  pc = 0x1068ee8; continue;
 }
 0x1068ee8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1068ef7; continue; }
  pc = 0x1068ef2; continue;
 }
 0x1068ef2 => {
  cpu.push(0x1068ef7); f_01068f25(cpu); if cpu.returned_to != 0x1068ef7 { return; }
  pc = 0x1068ef7; continue;
 }
 0x1068ef7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1068f17; continue; }
  pc = 0x1068f00; continue;
 }
 0x1068f00 => {
  { cpu.sub16((cpu.esi as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1068f17; continue; }
  pc = 0x1068f05; continue;
 }
 0x1068f05 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1068f17; continue; }
  pc = 0x1068f0b; continue;
 }
 0x1068f0b => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x1068f17; continue; }
  pc = 0x1068f11; continue;
 }
 0x1068f11 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068f17 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1068f1b; continue;
 }
 0x1068f1b => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1068ecd; continue; }
  pc = 0x1068f23; continue;
 }
 0x1068f23 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068f25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068f25); }
 let mut pc: u32 = 0x1068f25;
 loop { match pc {
 0x1068f25 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11294e8u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e404cu32)); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068f62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068f62); }
 let mut pc: u32 = 0x1068f62;
 loop { match pc {
 0x1068f62 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01068f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1068f64); }
 let mut pc: u32 = 0x1068f64;
 loop { match pc {
 0x1068f64 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e3fc0u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1068f80; continue; }
  pc = 0x1068f79; continue;
 }
 0x1068f79 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x1068f80; continue;
 }
 0x1068f80 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1068f92; continue; }
  pc = 0x1068f86; continue;
 }
 0x1068f86 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1068f80; continue; }
  pc = 0x1068f8e; continue;
 }
 0x1068f8e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1068f92 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069134(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069134); }
 let mut pc: u32 = 0x1069134;
 loop { match pc {
 0x1069134 => {
  { cpu.sub8(cpu.r8(0x9bu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106913e; continue; }
  pc = 0x106913d; continue;
 }
 0x106913d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106913e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x9bu32.wrapping_add(cpu.ebp))); cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069145(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069145); }
 let mut pc: u32 = 0x1069145;
 loop { match pc {
 0x1069145 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x10692eb; continue; }
  pc = 0x1069152; continue;
 }
 0x1069152 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10692e9; continue; }
  pc = 0x106915c; continue;
 }
 0x106915c => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x11b51a5u32, v); }
  cpu.push(0x106916a); f_01053987(cpu); if cpu.returned_to != 0x106916a { return; }
  if !cpu.cf { pc = 0x10692e9; continue; }
  pc = 0x1069170; continue;
 }
 0x1069170 => {
  { cpu.sub8(cpu.r8(0x5bu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10692e9; continue; }
  pc = 0x106917a; continue;
 }
 0x106917a => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x10692e9; continue; }
  pc = 0x1069185; continue;
 }
 0x1069185 => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11a188cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x1435u16); }
  if !cpu.zf { pc = 0x10692e9; continue; }
  pc = 0x10691a0; continue;
 }
 0x10691a0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11e21a9u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11e21abu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x900u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10692e9; continue; }
  pc = 0x10691e5; continue;
 }
 0x10691e5 => {
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10692e9; continue; }
  pc = 0x10691ef; continue;
 }
 0x10691ef => {
  cpu.push(0x10691f4); f_01069749(cpu); if cpu.returned_to != 0x10691f4 { return; }
  if !cpu.cf { pc = 0x1069220; continue; }
  pc = 0x10691f6; continue;
 }
 0x10691f6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x10691fe); f_010512c6(cpu); if cpu.returned_to != 0x10691fe { return; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.push(0x1069211); f_01047379(cpu); if cpu.returned_to != 0x1069211 { return; }
  { cpu.popad(); }
  cpu.push(0x1069217); f_01053cbd(cpu); if cpu.returned_to != 0x1069217 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10692e9; continue;
 }
 0x1069220 => {
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
  { let v: u16 = cpu.r16(0x11e21a9u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11e21abu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106924e); f_0104f36d(cpu); if cpu.returned_to != 0x106924e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1069256; continue;
 }
 0x1069256 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x11291d4u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.esi = v; }
  cpu.push(0x1069265); f_01053f27(cpu); if cpu.returned_to != 0x1069265 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x1069276; continue; }
  pc = 0x106926a; continue;
 }
 0x106926a => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1069256; continue; }
  pc = 0x1069273; continue;
 }
 0x1069273 => {
  { cpu.popad(); }
  pc = 0x10692e9; continue;
 }
 0x1069276 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c5f1eu32); }
  if cpu.zf { pc = 0x106928e; continue; }
  pc = 0x1069289; continue;
 }
 0x1069289 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106928e; continue;
 }
 0x106928e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  cpu.push(0x1069297); f_01067f20(cpu); if cpu.returned_to != 0x1069297 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x106929f); f_0105dae1(cpu); if cpu.returned_to != 0x106929f { return; }
  if cpu.cf { pc = 0x10692a6; continue; }
  pc = 0x10692a1; continue;
 }
 0x10692a1 => {
  cpu.push(0x10692a6); f_0105db89(cpu); if cpu.returned_to != 0x10692a6 { return; }
  pc = 0x10692a6; continue;
 }
 0x10692a6 => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c5f1eu32); }
  if cpu.zf { pc = 0x10692c8; continue; }
  pc = 0x10692b0; continue;
 }
 0x10692b0 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x10692c8; continue; }
  pc = 0x10692b6; continue;
 }
 0x10692b6 => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10692c4; continue; }
  pc = 0x10692ba; continue;
 }
 0x10692ba => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10692c4; continue; }
  pc = 0x10692bf; continue;
 }
 0x10692bf => {
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x10692c8; continue; }
  pc = 0x10692c4; continue;
 }
 0x10692c4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  pc = 0x10692c8; continue;
 }
 0x10692c8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  cpu.push(0x10692dc); f_01047379(cpu); if cpu.returned_to != 0x10692dc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10692e7); f_010692ed(cpu); if cpu.returned_to != 0x10692e7 { return; }
  pc = 0x10692eb; continue;
 }
 0x10692e9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10692eb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010692ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10692ed); }
 let mut pc: u32 = 0x10692ed;
 loop { match pc {
 0x10692ed => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10e4144u32, v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x112b6a0u32.wrapping_add(cpu.esi.wrapping_mul(4))) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x112b6a2u32.wrapping_add(cpu.esi.wrapping_mul(4))) as u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.esi)).wrapping_sub(0x8u16); cpu.w16(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x38u32.wrapping_add(cpu.esi)).wrapping_sub(0x8u16); cpu.w16(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1069338; continue;
 }
 0x1069338 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1069399; continue; }
  pc = 0x106933d; continue;
 }
 0x106933d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x11b51a5u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1069376; continue; }
  pc = 0x106935b; continue;
 }
 0x106935b => {
  { cpu.sub8(cpu.r8(0x36u32.wrapping_add(cpu.ebp)), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x1069376; continue; }
  pc = 0x1069360; continue;
 }
 0x1069360 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  cpu.push(0x1069373); f_01065c99(cpu); if cpu.returned_to != 0x1069373 { return; }
  { cpu.popad(); }
  pc = 0x1069394; continue;
 }
 0x1069376 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10e4144u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1069388); f_010507b9(cpu); if cpu.returned_to != 0x1069388 { return; }
  cpu.push(0x106938d); f_0106a2d3(cpu); if cpu.returned_to != 0x106938d { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x18u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1069394; continue;
 }
 0x1069394 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1069399; continue;
 }
 0x1069399 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1069338; continue; }
  pc = 0x106939e; continue;
 }
 0x106939e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106939f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106939f); }
 let mut pc: u32 = 0x106939f;
 loop { match pc {
 0x106939f => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x10693b7; continue; }
  pc = 0x10693a5; continue;
 }
 0x10693a5 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x10693b1; continue; }
  pc = 0x10693ab; continue;
 }
 0x10693ab => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x10693b7; continue; }
  pc = 0x10693b1; continue;
 }
 0x10693b1 => {
  cpu.push(0x10693b6); f_01063c10(cpu); if cpu.returned_to != 0x10693b6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10693b7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010693b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10693b9); }
 let mut pc: u32 = 0x10693b9;
 loop { match pc {
 0x10693b9 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x14u32); cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x10693c5; continue;
 }
 0x10693c5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x10693f4; continue; }
  pc = 0x10693ca; continue;
 }
 0x10693ca => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { cpu.and8(cpu.r8(0x1152edbu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x10693e9; continue; }
  pc = 0x10693d7; continue;
 }
 0x10693d7 => {
  { cpu.and8(cpu.r8(0x1152edbu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x10693f4; continue; }
  pc = 0x10693e0; continue;
 }
 0x10693e0 => {
  { cpu.and8(cpu.r8(0x1152e85u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x10693f4; continue; }
  pc = 0x10693e9; continue;
 }
 0x10693e9 => {
  { cpu.sub8(cpu.r8(0x1152eafu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10693f4; continue; }
  pc = 0x10693f2; continue;
 }
 0x10693f2 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10693f4; continue;
 }
 0x10693f4 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10693c5; continue; }
  pc = 0x10693f7; continue;
 }
 0x10693f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010696df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10696df); }
 let mut pc: u32 = 0x10696df;
 loop { match pc {
 0x10696df => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x10696fa; continue; }
  pc = 0x10696ea; continue;
 }
 0x10696ea => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10696f0); f_010576b2(cpu); if cpu.returned_to != 0x10696f0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x10696fa; continue; }
  pc = 0x10696f5; continue;
 }
 0x10696f5 => {
  cpu.push(0x10696fa); f_01057945(cpu); if cpu.returned_to != 0x10696fa { return; }
  pc = 0x10696fa; continue;
 }
 0x10696fa => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3debu32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x3du8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106970d); f_01069749(cpu); if cpu.returned_to != 0x106970d { return; }
  if cpu.cf { pc = 0x1069743; continue; }
  pc = 0x106970f; continue;
 }
 0x106970f => {
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11a188cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x21u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1069743; continue; }
  pc = 0x1069729; continue;
 }
 0x1069729 => {
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
 0x1069743 => {
  { let v: u8 = 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069749(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069749); }
 let mut pc: u32 = 0x1069749;
 loop { match pc {
 0x1069749 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x106975a; continue;
 }
 0x106975a => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1069768; continue; }
  pc = 0x1069760; continue;
 }
 0x1069760 => {
  { cpu.sub8(cpu.r8(0xafu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1069773; continue; }
  pc = 0x1069768; continue;
 }
 0x1069768 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106975a; continue; }
  pc = 0x1069770; continue;
 }
 0x1069770 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069773 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x1069768; continue; }
  pc = 0x1069778; continue;
 }
 0x1069778 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106977b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106977b); }
 let mut pc: u32 = 0x106977b;
 loop { match pc {
 0x106977b => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x55u8; cpu.w8(0x1fu32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106979d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106979d); }
 let mut pc: u32 = 0x106979d;
 loop { match pc {
 0x106979d => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10697a3); f_01052c6e(cpu); if cpu.returned_to != 0x10697a3 { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10697f0; continue; }
  pc = 0x10697a9; continue;
 }
 0x10697a9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x19u8); }
  if cpu.zf { pc = 0x10697b6; continue; }
  pc = 0x10697b4; continue;
 }
 0x10697b4 => {
  pc = 0x10697ce; continue;
 }
 0x10697b6 => {
  { let v: u16 = cpu.r16(0x112b7b0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x112b7b2u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10697ce; continue;
 }
 0x10697ce => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0x6u8; cpu.w8(0x15u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5bb4u32, v); }
  cpu.push(0x10697e6); f_01065d85(cpu); if cpu.returned_to != 0x10697e6 { return; }
  cpu.push(0x10697eb); f_01066278(cpu); if cpu.returned_to != 0x10697eb { return; }
  cpu.push(0x10697f0); f_0108bfab(cpu); if cpu.returned_to != 0x10697f0 { return; }
  pc = 0x10697f0; continue;
 }
 0x10697f0 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e3f84u32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1069814); f_01059da3(cpu); if cpu.returned_to != 0x1069814 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106981d); f_0106a326(cpu); if cpu.returned_to != 0x106981d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1069823); f_01036468(cpu); if cpu.returned_to != 0x1069823 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069855); }
 let mut pc: u32 = 0x1069855;
 loop { match pc {
 0x1069855 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1069895; continue; }
  pc = 0x106985b; continue;
 }
 0x106985b => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef379u32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1069896; continue; }
  pc = 0x106986c; continue;
 }
 0x106986c => {
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1069890; continue; }
  pc = 0x1069887; continue;
 }
 0x1069887 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xdfu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x1069890; continue;
 }
 0x1069890 => {
  cpu.push(0x1069895); f_0106b3b5(cpu); if cpu.returned_to != 0x1069895 { return; }
  pc = 0x1069895; continue;
 }
 0x1069895 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069896 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10698c3; continue; }
  pc = 0x10698b4; continue;
 }
 0x10698b4 => {
  cpu.push(0x10698b9); f_0106a395(cpu); if cpu.returned_to != 0x10698b9 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10698c3; continue; }
  pc = 0x10698bd; continue;
 }
 0x10698bd => {
  cpu.push(0x10698c2); f_0106a2d3(cpu); if cpu.returned_to != 0x10698c2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10698c3 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x10698d4; continue; }
  pc = 0x10698c9; continue;
 }
 0x10698c9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) & 0xdfu8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10698d4; continue;
 }
 0x10698d4 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x10698fc; continue; }
  pc = 0x10698e3; continue;
 }
 0x10698e3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10698f6; continue; }
  pc = 0x10698f0; continue;
 }
 0x10698f0 => {
  cpu.push(0x10698f5); f_010471ac(cpu); if cpu.returned_to != 0x10698f5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10698f6 => {
  cpu.push(0x10698fb); f_0106b3b5(cpu); if cpu.returned_to != 0x10698fb { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10698fc => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069c92(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069c92); } r_01069c92(cpu, 0x1069c92); }
pub fn f_01069cdc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069cdc); } r_01069c92(cpu, 0x1069cdc); }
pub fn f_01069cff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069cff); } r_01069c92(cpu, 0x1069cff); }
pub fn f_01069d55(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069d55); } r_01069c92(cpu, 0x1069d55); }
fn r_01069c92(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1069c92 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { pc = 0x1069cda; continue; }
  pc = 0x1069c9b; continue;
 }
 0x1069c9b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1069cb7; continue; }
  pc = 0x1069ca4; continue;
 }
 0x1069ca4 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0xfu8); }
  if cpu.zf { pc = 0x1069cda; continue; }
  pc = 0x1069cad; continue;
 }
 0x1069cad => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069cda; continue; }
  pc = 0x1069cb7; continue;
 }
 0x1069cb7 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x1069cc6; continue; }
  pc = 0x1069cbb; continue;
 }
 0x1069cbb => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1069cce; continue;
 }
 0x1069cc6 => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)) | 0x8u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1069cce; continue;
 }
 0x1069cce => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40dcu32, v); }
  { cpu.cf = true; }
  pc = 0x1069cdb; continue;
 }
 0x1069cda => {
  { cpu.cf = false; }
  pc = 0x1069cdb; continue;
 }
 0x1069cdb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069cdc => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069ce5; continue;
 }
 0x1069ce5 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069cee; continue;
 }
 0x1069cee => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1069cb7; continue; }
  pc = 0x1069cfd; continue;
 }
 0x1069cfd => {
  cpu.poll(); pc = 0x1069ca4; continue;
 }
 0x1069cff => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069d08; continue;
 }
 0x1069d08 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069d11; continue;
 }
 0x1069d11 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { pc = 0x1069d24; continue; }
  pc = 0x1069d1b; continue;
 }
 0x1069d1b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1069d3c; continue; }
  pc = 0x1069d24; continue;
 }
 0x1069d24 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069d2d; continue;
 }
 0x1069d2d => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069d3c; continue;
 }
 0x1069d3c => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1069d45; continue; }
  pc = 0x1069d40; continue;
 }
 0x1069d40 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)).wrapping_add(0x8u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1069d45; continue;
 }
 0x1069d45 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40d8u32, v); }
  { cpu.cf = true; }
  cpu.poll(); pc = 0x1069cdb; continue;
 }
 0x1069d55 => {
  { cpu.and8(cpu.r8(0x10e3dbbu32), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x1069cda; continue; }
  pc = 0x1069d62; continue;
 }
 0x1069d62 => {
  { cpu.and8(cpu.r8(0x10e3dbdu32), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1069d24; continue; }
  pc = 0x1069d6b; continue;
 }
 0x1069d6b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1069d3c; continue; }
  pc = 0x1069d74; continue;
 }
 0x1069d74 => {
  cpu.poll(); pc = 0x1069d24; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069e06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069e06); } r_01069e06(cpu, 0x1069e06); }
pub fn f_01069e7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069e7b); } r_01069e06(cpu, 0x1069e7b); }
pub fn f_01069ef0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069ef0); } r_01069e06(cpu, 0x1069ef0); }
pub fn f_01069f65(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1069f65); } r_01069e06(cpu, 0x1069f65); }
fn r_01069e06(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1069901 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef379u32, v); }
  { let v: u8 = cpu.r8(0x7bu32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x112b990u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1069a35; continue; }
  pc = 0x106991d; continue;
 }
 0x106991d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x106992e; continue; }
  pc = 0x1069922; continue;
 }
 0x1069922 => {
  cpu.push(0x1069927); f_0106a255(cpu); if cpu.returned_to != 0x1069927 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106992e; continue;
 }
 0x106992e => {
  cpu.push(0x1069933); f_010534d5(cpu); if cpu.returned_to != 0x1069933 { return; }
  if !cpu.cf { pc = 0x106993a; continue; }
  pc = 0x1069935; continue;
 }
 0x1069935 => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106993a => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x10699ce; continue; }
  pc = 0x1069946; continue;
 }
 0x1069946 => {
  { cpu.sub8(cpu.r8(0x10ef359u32), 0x0u8); }
  if !cpu.zf { pc = 0x10699ce; continue; }
  pc = 0x106994f; continue;
 }
 0x106994f => {
  cpu.push(0x1069954); f_01053190(cpu); if cpu.returned_to != 0x1069954 { return; }
  if !cpu.cf { pc = 0x106996b; continue; }
  pc = 0x1069956; continue;
 }
 0x1069956 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106996b => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1069977; continue; }
  pc = 0x1069971; continue;
 }
 0x1069971 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10699ce; continue;
 }
 0x1069977 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10699e7; continue; }
  pc = 0x106997d; continue;
 }
 0x106997d => {
  if cpu.sf { pc = 0x1069b26; continue; }
  pc = 0x1069983; continue;
 }
 0x1069983 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x10699e7; continue; }
  pc = 0x1069989; continue;
 }
 0x1069989 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x10699e7; continue; }
  pc = 0x1069992; continue;
 }
 0x1069992 => {
  { cpu.sub8(cpu.r8(0x64u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1069b31; continue; }
  pc = 0x106999c; continue;
 }
 0x106999c => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10699ab; continue; }
  pc = 0x10699a1; continue;
 }
 0x10699a1 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  pc = 0x10699ab; continue;
 }
 0x10699ab => {
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x10699ba; continue; }
  pc = 0x10699b0; continue;
 }
 0x10699b0 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  pc = 0x10699ba; continue;
 }
 0x10699ba => {
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x10699ca; continue; }
  pc = 0x10699c0; continue;
 }
 0x10699c0 => {
  cpu.push(0x10699c5); f_0106a326(cpu); if cpu.returned_to != 0x10699c5 { return; }
  f_01069855(cpu); return;
 }
 0x10699ca => {
  { let v: u8 = cpu.inc8(cpu.r8(0x64u32.wrapping_add(cpu.ebp))); cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10699ce => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  f_01069855(cpu); return;
 }
 0x10699e7 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10699ce; continue; }
  pc = 0x10699f2; continue;
 }
 0x10699f2 => {
  cpu.push(0x10699f7); f_0106a356(cpu); if cpu.returned_to != 0x10699f7 { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40d8u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1069a09; continue; }
  pc = 0x1069a04; continue;
 }
 0x1069a04 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  pc = 0x1069a09; continue;
 }
 0x1069a09 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40dcu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1069a1b; continue; }
  pc = 0x1069a16; continue;
 }
 0x1069a16 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  pc = 0x1069a1b; continue;
 }
 0x1069a1b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { cpu.poll(); pc = 0x10699ce; continue; }
  pc = 0x1069a2f; continue;
 }
 0x1069a2f => {
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10699ce; continue;
 }
 0x1069a35 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1069b2b; continue; }
  pc = 0x1069a41; continue;
 }
 0x1069a41 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1069a4d; continue; }
  pc = 0x1069a47; continue;
 }
 0x1069a47 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10699ce; continue;
 }
 0x1069a4d => {
  { cpu.sub8(cpu.r8(0x10ef372u32), 0x0u8); }
  if !cpu.zf { pc = 0x1069a5f; continue; }
  pc = 0x1069a56; continue;
 }
 0x1069a56 => {
  { cpu.sub8(cpu.r8(0x10ef359u32), 0x0u8); }
  if !cpu.zf { pc = 0x1069ad3; continue; }
  pc = 0x1069a5f; continue;
 }
 0x1069a5f => {
  cpu.push(0x1069a64); f_01053190(cpu); if cpu.returned_to != 0x1069a64 { return; }
  if !cpu.cf { pc = 0x1069a7b; continue; }
  pc = 0x1069a66; continue;
 }
 0x1069a66 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069a7b => {
  cpu.push(0x1069a80); f_01053367(cpu); if cpu.returned_to != 0x1069a80 { return; }
  if !cpu.cf { pc = 0x1069ad2; continue; }
  pc = 0x1069a82; continue;
 }
 0x1069a82 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  cpu.push(0x1069a9b); f_0106a356(cpu); if cpu.returned_to != 0x1069a9b { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40d8u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1069aad; continue; }
  pc = 0x1069aa8; continue;
 }
 0x1069aa8 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  pc = 0x1069aad; continue;
 }
 0x1069aad => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40dcu32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1069abf; continue; }
  pc = 0x1069aba; continue;
 }
 0x1069aba => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  pc = 0x1069abf; continue;
 }
 0x1069abf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1069ad2; continue; }
  pc = 0x1069acb; continue;
 }
 0x1069acb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x2u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x1069ad2; continue;
 }
 0x1069ad2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069ad3 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  { cpu.sub8(cpu.r8(0x112b658u32), 0x0u8); }
  if !cpu.zf { pc = 0x1069af1; continue; }
  pc = 0x1069af0; continue;
 }
 0x1069af0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069af1 => {
  { let v: u32 = cpu.r32(0x10e4148u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1069b1c); f_01050829(cpu); if cpu.returned_to != 0x1069b1c { return; }
  cpu.push(0x1069b21); f_0106a2d3(cpu); if cpu.returned_to != 0x1069b21 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069b26 => {
  cpu.poll(); pc = 0x10699e7; continue;
 }
 0x1069b2b => {
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069b31 => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10699e7; continue; }
  pc = 0x1069b42; continue;
 }
 0x1069b42 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e415cu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069b71; continue; }
  pc = 0x1069b6e; continue;
 }
 0x1069b6e => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1069b71; continue;
 }
 0x1069b71 => {
  { cpu.sub16((cpu.eax as u16), 0x60u16); }
  if !cpu.cf { pc = 0x1069c78; continue; }
  pc = 0x1069b7b; continue;
 }
 0x1069b7b => {
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069b87; continue; }
  pc = 0x1069b84; continue;
 }
 0x1069b84 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1069b87; continue;
 }
 0x1069b87 => {
  { cpu.sub16((cpu.eax as u16), 0x60u16); }
  if !cpu.cf { pc = 0x1069c78; continue; }
  pc = 0x1069b91; continue;
 }
 0x1069b91 => {
  { let v = cpu.r16(0x10e3e49u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e4bu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e4fu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3e4du32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3dc1u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e3fccu32); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1069bc2); f_0103b870(cpu); if cpu.returned_to != 0x1069bc2 { return; }
  pc = 0x1069bc2; continue;
 }
 0x1069bc2 => {
  { let v: u16 = cpu.r16(0x10e3e49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3e4bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069c00; continue; }
  pc = 0x1069bfd; continue;
 }
 0x1069bfd => {
  { cpu.cf = true; }
  pc = 0x1069c28; continue;
 }
 0x1069c00 => {
  { cpu.sub16(cpu.r16(0x10e3e4du32), 0x0u16); }
  if cpu.zf { pc = 0x1069c27; continue; }
  pc = 0x1069c0a; continue;
 }
 0x1069c0a => {
  { let v: u8 = 0x10u8; cpu.w8(0x10e3dc1u32, v); }
  { let v: u16 = cpu.r16(0x10e3e4fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e3fccu32); cpu.push(0x1069c1d); cpu.call(t); if cpu.returned_to != 0x1069c1d { return; }
  { let v: u16 = cpu.r16(0x10e3e4du32).wrapping_sub(0x10u16); cpu.w16(0x10e3e4du32, v); }
  cpu.poll(); pc = 0x1069bc2; continue;
 }
 0x1069c27 => {
  { cpu.cf = false; }
  pc = 0x1069c28; continue;
 }
 0x1069c28 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3fccu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3dc1u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e4du32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e4fu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e4bu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3e49u32, v); }
  if cpu.cf { cpu.poll(); pc = 0x10699e7; continue; }
  pc = 0x1069c5a; continue;
 }
 0x1069c5a => {
  cpu.push(0x1069c5f); f_0106a326(cpu); if cpu.returned_to != 0x1069c5f { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  f_01069855(cpu); return;
 }
 0x1069c78 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1069c83; continue; }
  pc = 0x1069c7e; continue;
 }
 0x1069c7e => {
  cpu.poll(); pc = 0x10699e7; continue;
 }
 0x1069c83 => {
  cpu.push(0x1069c88); f_0106a326(cpu); if cpu.returned_to != 0x1069c88 { return; }
  cpu.push(0x1069c8d); f_01067331(cpu); if cpu.returned_to != 0x1069c8d { return; }
  cpu.poll(); pc = 0x10699e7; continue;
 }
 0x1069e06 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069e7a; continue; }
  pc = 0x1069e23; continue;
 }
 0x1069e23 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1069e29; continue; }
  pc = 0x1069e27; continue;
 }
 0x1069e27 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x1069e29; continue;
 }
 0x1069e29 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069e3b; continue;
 }
 0x1069e3b => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1069e3c; continue;
 }
 0x1069e3c => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069e4e; continue; }
  pc = 0x1069e41; continue;
 }
 0x1069e41 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1069e4e; continue;
 }
 0x1069e4e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1069e3c; continue; }
  pc = 0x1069e54; continue;
 }
 0x1069e54 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  let t = cpu.r32(0x10e3fd8u32); cpu.push(0x1069e65); cpu.call(t); if cpu.returned_to != 0x1069e65 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1069901; continue; }
  pc = 0x1069e6c; continue;
 }
 0x1069e6c => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069e7a; continue;
 }
 0x1069e7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069e7b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069eef; continue; }
  pc = 0x1069e98; continue;
 }
 0x1069e98 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1069e9e; continue; }
  pc = 0x1069e9c; continue;
 }
 0x1069e9c => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x1069e9e; continue;
 }
 0x1069e9e => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069eb0; continue;
 }
 0x1069eb0 => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1069eb1; continue;
 }
 0x1069eb1 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069ebd; continue; }
  pc = 0x1069eb6; continue;
 }
 0x1069eb6 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1069ebd; continue;
 }
 0x1069ebd => {
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1069eb1; continue; }
  pc = 0x1069ec9; continue;
 }
 0x1069ec9 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  let t = cpu.r32(0x10e3fd8u32); cpu.push(0x1069eda); cpu.call(t); if cpu.returned_to != 0x1069eda { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1069901; continue; }
  pc = 0x1069ee1; continue;
 }
 0x1069ee1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069eef; continue;
 }
 0x1069eef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069ef0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069f64; continue; }
  pc = 0x1069f0d; continue;
 }
 0x1069f0d => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1069f13; continue; }
  pc = 0x1069f11; continue;
 }
 0x1069f11 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x1069f13; continue;
 }
 0x1069f13 => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069f25; continue;
 }
 0x1069f25 => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1069f26; continue;
 }
 0x1069f26 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069f38; continue; }
  pc = 0x1069f2b; continue;
 }
 0x1069f2b => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1069f38; continue;
 }
 0x1069f38 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1069f26; continue; }
  pc = 0x1069f3e; continue;
 }
 0x1069f3e => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  let t = cpu.r32(0x10e3fd8u32); cpu.push(0x1069f4f); cpu.call(t); if cpu.returned_to != 0x1069f4f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1069901; continue; }
  pc = 0x1069f56; continue;
 }
 0x1069f56 => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069f64; continue;
 }
 0x1069f64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1069f65 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1069fdd; continue; }
  pc = 0x1069f86; continue;
 }
 0x1069f86 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1069f8c; continue; }
  pc = 0x1069f8a; continue;
 }
 0x1069f8a => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x1069f8c; continue;
 }
 0x1069f8c => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4eu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069f9e; continue;
 }
 0x1069f9e => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1069f9f; continue;
 }
 0x1069f9f => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1069fab; continue; }
  pc = 0x1069fa4; continue;
 }
 0x1069fa4 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4au32.wrapping_add(cpu.ebp))); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1069fab; continue;
 }
 0x1069fab => {
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1069f9f; continue; }
  pc = 0x1069fb7; continue;
 }
 0x1069fb7 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  let t = cpu.r32(0x10e3fd8u32); cpu.push(0x1069fc8); cpu.call(t); if cpu.returned_to != 0x1069fc8 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1069901; continue; }
  pc = 0x1069fcf; continue;
 }
 0x1069fcf => {
  { let v: u8 = 0x0u8; cpu.w8(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), (cpu.esi as u16)); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { f_01069855(cpu); return; }
  pc = 0x1069fdd; continue;
 }
 0x1069fdd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069fde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069fde); }
 let mut pc: u32 = 0x1069fde;
 loop { match pc {
 0x1069fde => {
  { let v: u8 = 0x4u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069fe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069fe6); }
 let mut pc: u32 = 0x1069fe6;
 loop { match pc {
 0x1069fe6 => {
  { let v: u8 = 0x2u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069fee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069fee); }
 let mut pc: u32 = 0x1069fee;
 loop { match pc {
 0x1069fee => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1069ffe; continue; }
  pc = 0x1069ff7; continue;
 }
 0x1069ff7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x89u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1069ffe; continue;
 }
 0x1069ffe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01069fff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1069fff); }
 let mut pc: u32 = 0x1069fff;
 loop { match pc {
 0x1069fff => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106a047; continue; }
  pc = 0x106a005; continue;
 }
 0x106a005 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a038); f_010507b9(cpu); if cpu.returned_to != 0x106a038 { return; }
  cpu.push(0x106a03d); f_01069fde(cpu); if cpu.returned_to != 0x106a03d { return; }
  cpu.push(0x106a042); f_0106a2d3(cpu); if cpu.returned_to != 0x106a042 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  pc = 0x106a047; continue;
 }
 0x106a047 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a04e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a04e); }
 let mut pc: u32 = 0x106a04e;
 loop { match pc {
 0x106a04e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x106a05c; continue; }
  pc = 0x106a054; continue;
 }
 0x106a054 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x106a05c; continue; }
  pc = 0x106a05a; continue;
 }
 0x106a05a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a05c => {
  { cpu.pushad(); }
  cpu.push(0x106a062); f_01047224(cpu); if cpu.returned_to != 0x106a062 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106a0a0; continue; }
  pc = 0x106a06d; continue;
 }
 0x106a06d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106a0a0; continue; }
  pc = 0x106a073; continue;
 }
 0x106a073 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a05a; continue; }
  pc = 0x106a079; continue;
 }
 0x106a079 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a05a; continue; }
  pc = 0x106a07f; continue;
 }
 0x106a07f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a05a; continue; }
  pc = 0x106a085; continue;
 }
 0x106a085 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a05a; continue; }
  pc = 0x106a08b; continue;
 }
 0x106a08b => {
  { let v: u8 = 0x2u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0xfcu8); }
  if cpu.zf { pc = 0x106a099; continue; }
  pc = 0x106a095; continue;
 }
 0x106a095 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a099; continue;
 }
 0x106a099 => {
  cpu.push(0x106a09e); f_0106a234(cpu); if cpu.returned_to != 0x106a09e { return; }
  pc = 0x106a0a7; continue;
 }
 0x106a0a0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ecx)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.ecx), v); }
  pc = 0x106a0a7; continue;
 }
 0x106a0a7 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x106a0dc; continue; }
  pc = 0x106a0ad; continue;
 }
 0x106a0ad => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x106a0dc; continue; }
  pc = 0x106a0b3; continue;
 }
 0x106a0b3 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a0c6); f_010513df(cpu); if cpu.returned_to != 0x106a0c6 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x106a0dc; continue; }
  pc = 0x106a0d8; continue;
 }
 0x106a0d8 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  pc = 0x106a0dc; continue;
 }
 0x106a0dc => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a174; continue; }
  pc = 0x106a0e6; continue;
 }
 0x106a0e6 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a0f6; continue; }
  pc = 0x106a0f1; continue;
 }
 0x106a0f1 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x106a0f6; continue;
 }
 0x106a0f6 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x106a139; continue; }
  pc = 0x106a0fe; continue;
 }
 0x106a0fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a12b; continue; }
  pc = 0x106a11a; continue;
 }
 0x106a11a => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a1f5; continue; }
  pc = 0x106a126; continue;
 }
 0x106a126 => {
  pc = 0x106a1ef; continue;
 }
 0x106a12b => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106a1fb; continue; }
  pc = 0x106a134; continue;
 }
 0x106a134 => {
  pc = 0x106a1f5; continue;
 }
 0x106a139 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a166; continue; }
  pc = 0x106a155; continue;
 }
 0x106a155 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x106a1e9; continue; }
  pc = 0x106a161; continue;
 }
 0x106a161 => {
  pc = 0x106a1ef; continue;
 }
 0x106a166 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x106a201; continue; }
  pc = 0x106a16f; continue;
 }
 0x106a16f => {
  pc = 0x106a1fb; continue;
 }
 0x106a174 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a187; continue; }
  pc = 0x106a182; continue;
 }
 0x106a182 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x106a187; continue;
 }
 0x106a187 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x106a1bc; continue; }
  pc = 0x106a18f; continue;
 }
 0x106a18f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a1b5; continue; }
  pc = 0x106a1ab; continue;
 }
 0x106a1ab => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106a20d; continue; }
  pc = 0x106a1b3; continue;
 }
 0x106a1b3 => {
  pc = 0x106a213; continue;
 }
 0x106a1b5 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106a20d; continue; }
  pc = 0x106a1ba; continue;
 }
 0x106a1ba => {
  pc = 0x106a207; continue;
 }
 0x106a1bc => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106a1e2; continue; }
  pc = 0x106a1d8; continue;
 }
 0x106a1d8 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x106a1e9; continue; }
  pc = 0x106a1e0; continue;
 }
 0x106a1e0 => {
  pc = 0x106a213; continue;
 }
 0x106a1e2 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x106a201; continue; }
  pc = 0x106a1e7; continue;
 }
 0x106a1e7 => {
  pc = 0x106a207; continue;
 }
 0x106a1e9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a1ef => {
  { let v: u8 = 0x1u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a1f5 => {
  { let v: u8 = 0x2u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a1fb => {
  { let v: u8 = 0x3u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a201 => {
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a207 => {
  { let v: u8 = 0x5u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a20d => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a213 => {
  { let v: u8 = 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a217; continue;
 }
 0x106a217 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x106a224; continue; }
  pc = 0x106a21e; continue;
 }
 0x106a21e => {
  { let v: u16 = 0x1u16; cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a224; continue;
 }
 0x106a224 => {
  { let v: u8 = 0xffu8; cpu.w8(0x9eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a234(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a234); }
 let mut pc: u32 = 0x106a234;
 loop { match pc {
 0x106a234 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10f7e4au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a250; continue; }
  pc = 0x106a249; continue;
 }
 0x106a249 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a250; continue; }
  pc = 0x106a24e; continue;
 }
 0x106a24e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a250 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a255(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a255); }
 let mut pc: u32 = 0x106a255;
 loop { match pc {
 0x106a255 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10f7e4au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a269(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a269); }
 let mut pc: u32 = 0x106a269;
 loop { match pc {
 0x106a269 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x106a2d2; continue; }
  pc = 0x106a26f; continue;
 }
 0x106a26f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1053517u32; cpu.w32(0x10e3fd8u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x106a2a2; continue; }
  pc = 0x106a298; continue;
 }
 0x106a298 => {
  { let v: u32 = 0x10688d9u32; cpu.w32(0x10e3fd8u32, v); }
  pc = 0x106a2a2; continue;
 }
 0x106a2a2 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1129320u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x106a2b0); cpu.call(t); if cpu.returned_to != 0x106a2b0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106a2d2); f_01068d78(cpu); if cpu.returned_to != 0x106a2d2 { return; }
  pc = 0x106a2d2; continue;
 }
 0x106a2d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a2d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a2d3); }
 let mut pc: u32 = 0x106a2d3;
 loop { match pc {
 0x106a2d3 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x106a324; continue; }
  pc = 0x106a2d9; continue;
 }
 0x106a2d9 => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106a2f1; continue; }
  pc = 0x106a2ea; continue;
 }
 0x106a2ea => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a2f1; continue;
 }
 0x106a2f1 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x106a324; continue; }
  pc = 0x106a2f5; continue;
 }
 0x106a2f5 => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x106a312; continue; }
  pc = 0x106a304; continue;
 }
 0x106a304 => {
  cpu.push(0x106a309); f_01069fee(cpu); if cpu.returned_to != 0x106a309 { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106a311); f_010512c6(cpu); if cpu.returned_to != 0x106a311 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x106a312; continue;
 }
 0x106a312 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a31f); f_0106a04e(cpu); if cpu.returned_to != 0x106a31f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a324 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a326(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a326); }
 let mut pc: u32 = 0x106a326;
 loop { match pc {
 0x106a326 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x106a332; continue; }
  pc = 0x106a32c; continue;
 }
 0x106a32c => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x106a346; continue; }
  pc = 0x106a332; continue;
 }
 0x106a332 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x14u32); cpu.ebp = v; }
  cpu.push(0x106a33a); f_0106a366(cpu); if cpu.returned_to != 0x106a33a { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x14u32); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x106a346; continue;
 }
 0x106a346 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xcu32); cpu.ebp = v; }
  cpu.push(0x106a34e); f_0106a366(cpu); if cpu.returned_to != 0x106a34e { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0xcu32); cpu.ebp = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a356(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a356); }
 let mut pc: u32 = 0x106a356;
 loop { match pc {
 0x106a356 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xcu32); cpu.ebp = v; }
  cpu.push(0x106a35e); f_0106a366(cpu); if cpu.returned_to != 0x106a35e { return; }
  { let v: u32 = cpu.ebp.wrapping_sub(0xcu32); cpu.ebp = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xdfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a366); }
 let mut pc: u32 = 0x106a366;
 loop { match pc {
 0x106a366 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  pc = 0x106a369; continue;
 }
 0x106a369 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x106a38b; continue; }
  pc = 0x106a36d; continue;
 }
 0x106a36d => {
  { cpu.sub32(cpu.ebx, 0x10c7adbu32); }
  if cpu.cf { pc = 0x106a393; continue; }
  pc = 0x106a375; continue;
 }
 0x106a375 => {
  { cpu.sub32(cpu.ebx, 0x10e3cdbu32); }
  if !cpu.cf { pc = 0x106a393; continue; }
  pc = 0x106a37d; continue;
 }
 0x106a37d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.poll(); pc = 0x106a369; continue;
 }
 0x106a38b => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a393 => {
  cpu.poll(); pc = 0x106a38b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a395(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a395); }
 let mut pc: u32 = 0x106a395;
 loop { match pc {
 0x106a395 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106a3a7; continue; }
  pc = 0x106a39b; continue;
 }
 0x106a39b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x106a3ae; continue; }
  pc = 0x106a3a2; continue;
 }
 0x106a3a2 => {
  pc = 0x106a42b; continue;
 }
 0x106a3a7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x106a400; continue; }
  pc = 0x106a3ae; continue;
 }
 0x106a3ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x106a3c9; continue; }
  pc = 0x106a3bb; continue;
 }
 0x106a3bb => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x106a3cd; continue; }
  pc = 0x106a3c9; continue;
 }
 0x106a3c9 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a3cd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x106a41b; continue; }
  pc = 0x106a3dd; continue;
 }
 0x106a3dd => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xfdu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x106a400; continue; }
  pc = 0x106a3e9; continue;
 }
 0x106a3e9 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x106a400; continue; }
  pc = 0x106a3ed; continue;
 }
 0x106a3ed => {
  cpu.push(0x106a3f2); f_0100b130(cpu); if cpu.returned_to != 0x106a3f2 { return; }
  cpu.push(0x106a3f7); f_0106a484(cpu); if cpu.returned_to != 0x106a3f7 { return; }
  cpu.push(0x106a3fc); f_0106a451(cpu); if cpu.returned_to != 0x106a3fc { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a400 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106a417; continue; }
  pc = 0x106a40d; continue;
 }
 0x106a40d => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xf7u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x106a417; continue;
 }
 0x106a417 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a41b => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x106a42b; continue; }
  pc = 0x106a41f; continue;
 }
 0x106a41f => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x106a42b; continue; }
  pc = 0x106a424; continue;
 }
 0x106a424 => {
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a42b => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x106a3a7; continue; }
  pc = 0x106a440; continue;
 }
 0x106a440 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x106a3a7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a451(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a451); }
 let mut pc: u32 = 0x106a451;
 loop { match pc {
 0x106a451 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x106a47e; continue; }
  pc = 0x106a458; continue;
 }
 0x106a458 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a466); f_01018f3f(cpu); if cpu.returned_to != 0x106a466 { return; }
  if !cpu.cf { pc = 0x106a47e; continue; }
  pc = 0x106a468; continue;
 }
 0x106a468 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x40u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x40u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a480; continue; }
  pc = 0x106a477; continue;
 }
 0x106a477 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x106a47e); f_01050829(cpu); if cpu.returned_to != 0x106a47e { return; }
  pc = 0x106a47e; continue;
 }
 0x106a47e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a480 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a484); }
 let mut pc: u32 = 0x106a484;
 loop { match pc {
 0x106a484 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e4154u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a49a); f_01018e6e(cpu); if cpu.returned_to != 0x106a49a { return; }
  if !cpu.cf { pc = 0x106a4be; continue; }
  pc = 0x106a49c; continue;
 }
 0x106a49c => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x106a4be; continue; }
  pc = 0x106a4a8; continue;
 }
 0x106a4a8 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x40u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x40u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a4be; continue; }
  pc = 0x106a4b9; continue;
 }
 0x106a4b9 => {
  cpu.push(0x106a4be); f_01050829(cpu); if cpu.returned_to != 0x106a4be { return; }
  pc = 0x106a4be; continue;
 }
 0x106a4be => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a4c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a4c6); }
 let mut pc: u32 = 0x106a4c6;
 loop { match pc {
 0x106a4c6 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a4d6; continue; }
  pc = 0x106a4cf; continue;
 }
 0x106a4cf => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  pc = 0x106a4d6; continue;
 }
 0x106a4d6 => {
  { cpu.sub8(cpu.r8(0x88u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x106a50b; continue; }
  pc = 0x106a4de; continue;
 }
 0x106a4de => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x88u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.sf { pc = 0x106a4ea; continue; }
  pc = 0x106a4e8; continue;
 }
 0x106a4e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x106a4ea; continue;
 }
 0x106a4ea => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a4f0; continue; }
  pc = 0x106a4ee; continue;
 }
 0x106a4ee => {
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106a4f0; continue;
 }
 0x106a4f0 => {
  { let v: u32 = cpu.r32(0x10f7e5cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x106a50c; continue; }
  pc = 0x106a4fc; continue;
 }
 0x106a4fc => {
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xb2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a50b; continue;
 }
 0x106a50b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a50c => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a54e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a54e); }
 let mut pc: u32 = 0x106a54e;
 loop { match pc {
 0x106a54e => {
  { cpu.sub16(cpu.r16(0x54u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if !cpu.zf { pc = 0x106a583; continue; }
  pc = 0x106a556; continue;
 }
 0x106a556 => {
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x106a566); f_0106a585(cpu); if cpu.returned_to != 0x106a566 { return; }
  if !cpu.cf { pc = 0x106a583; continue; }
  pc = 0x106a568; continue;
 }
 0x106a568 => {
  cpu.push(0x106a56d); f_01069855(cpu); if cpu.returned_to != 0x106a56d { return; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  cpu.push(0x106a577); f_01047379(cpu); if cpu.returned_to != 0x106a577 { return; }
  cpu.push(0x106a57c); f_0106a326(cpu); if cpu.returned_to != 0x106a57c { return; }
  cpu.push(0x106a581); f_010675fd(cpu); if cpu.returned_to != 0x106a581 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a583 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a585(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a585); }
 let mut pc: u32 = 0x106a585;
 loop { match pc {
 0x106a585 => {
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
  if !cpu.cf { pc = 0x106a5c0; continue; }
  pc = 0x106a5bd; continue;
 }
 0x106a5bd => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a5c0 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a5c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a5c4); }
 let mut pc: u32 = 0x106a5c4;
 loop { match pc {
 0x106a5c4 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106a63e; continue; }
  pc = 0x106a5ca; continue;
 }
 0x106a5ca => {
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106a612; continue; }
  pc = 0x106a5da; continue;
 }
 0x106a5da => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x106a658; continue; }
  pc = 0x106a5e3; continue;
 }
 0x106a5e3 => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x106a612; continue; }
  pc = 0x106a5ec; continue;
 }
 0x106a5ec => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x22u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x106a612; continue; }
  pc = 0x106a607; continue;
 }
 0x106a607 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0x20u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106a612; continue;
 }
 0x106a612 => {
  cpu.push(0x106a617); f_0106a6aa(cpu); if cpu.returned_to != 0x106a617 { return; }
  if cpu.cf { pc = 0x106a658; continue; }
  pc = 0x106a619; continue;
 }
 0x106a619 => {
  cpu.push(0x106a61e); f_01053cbd(cpu); if cpu.returned_to != 0x106a61e { return; }
  cpu.push(0x106a623); f_0106a659(cpu); if cpu.returned_to != 0x106a623 { return; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x106a637; continue; }
  pc = 0x106a635; continue;
 }
 0x106a635 => {
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x106a637; continue;
 }
 0x106a637 => {
  cpu.poll(); pc = cpu.r32(0x112b994u32.wrapping_add(cpu.ebx.wrapping_mul(2))); continue;
 }
 0x106a63e => {
  { cpu.sub8(cpu.r8(0x11e21bdu32), 0x0u8); }
  if cpu.zf { pc = 0x106a658; continue; }
  pc = 0x106a647; continue;
 }
 0x106a647 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106a64d); f_0106b659(cpu); if cpu.returned_to != 0x106a64d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x106a5c4; continue; }
  pc = 0x106a658; continue;
 }
 0x106a658 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a659); }
 let mut pc: u32 = 0x106a659;
 loop { match pc {
 0x106a659 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106a6a9; continue; }
  pc = 0x106a65f; continue;
 }
 0x106a65f => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757b8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x106a670; continue;
 }
 0x106a670 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106a6a3; continue; }
  pc = 0x106a675; continue;
 }
 0x106a675 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x106a68e; continue; }
  pc = 0x106a688; continue;
 }
 0x106a688 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x106a6a3; continue; }
  pc = 0x106a68e; continue;
 }
 0x106a68e => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x106a6a3; continue; }
  pc = 0x106a696; continue;
 }
 0x106a696 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x106a6a1); f_010675fd(cpu); if cpu.returned_to != 0x106a6a1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x106a6a3; continue;
 }
 0x106a6a3 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106a670; continue; }
  pc = 0x106a6a8; continue;
 }
 0x106a6a8 => {
  { cpu.popad(); }
  pc = 0x106a6a9; continue;
 }
 0x106a6a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a6aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a6aa); }
 let mut pc: u32 = 0x106a6aa;
 loop { match pc {
 0x106a6aa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  cpu.push(0x106a6b6); f_01053c0d(cpu); if cpu.returned_to != 0x106a6b6 { return; }
  if cpu.cf { pc = 0x106a6c8; continue; }
  pc = 0x106a6b8; continue;
 }
 0x106a6b8 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  cpu.push(0x106a6c3); f_01053c5f(cpu); if cpu.returned_to != 0x106a6c3 { return; }
  if cpu.cf { pc = 0x106a6c8; continue; }
  pc = 0x106a6c5; continue;
 }
 0x106a6c5 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a6c8 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a6cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a6cb); }
 let mut pc: u32 = 0x106a6cb;
 loop { match pc {
 0x106a6cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a6cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a6cc); }
 let mut pc: u32 = 0x106a6cc;
 loop { match pc {
 0x106a6cc => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a6d6; continue;
 }
 0x106a6d6 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a6e0; continue;
 }
 0x106a6e0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a6ea; continue;
 }
 0x106a6ea => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a6f4; continue;
 }
 0x106a6f4 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a6fe; continue;
 }
 0x106a6fe => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a704; continue;
 }
 0x106a704 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a70a; continue;
 }
 0x106a70a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a710; continue;
 }
 0x106a710 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a716; continue;
 }
 0x106a716 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a71c; continue;
 }
 0x106a71c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a722; continue;
 }
 0x106a722 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a728; continue;
 }
 0x106a728 => {
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
  cpu.push(0x106a748); f_01049cd8(cpu); if cpu.returned_to != 0x106a748 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11296e5u32), 0x0u8); }
  if cpu.zf { pc = 0x106a758; continue; }
  pc = 0x106a754; continue;
 }
 0x106a754 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x106a75d; continue;
 }
 0x106a758 => {
  cpu.push(0x106a75d); f_01068d2a(cpu); if cpu.returned_to != 0x106a75d { return; }
  pc = 0x106a75d; continue;
 }
 0x106a75d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x106a76d; continue; }
  pc = 0x106a766; continue;
 }
 0x106a766 => {
  cpu.push(0x106a76b); f_0106aacb(cpu); if cpu.returned_to != 0x106a76b { return; }
  pc = 0x106a77f; continue;
 }
 0x106a76d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4au8); }
  if cpu.zf { pc = 0x106a77f; continue; }
  pc = 0x106a773; continue;
 }
 0x106a773 => {
  { let v: u8 = 0x4au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a77f; continue;
 }
 0x106a77f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a780); }
 let mut pc: u32 = 0x106a780;
 loop { match pc {
 0x106a780 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a78a; continue;
 }
 0x106a78a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a794; continue;
 }
 0x106a794 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a79e; continue;
 }
 0x106a79e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7a8; continue;
 }
 0x106a7a8 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7b2; continue;
 }
 0x106a7b2 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7b8; continue;
 }
 0x106a7b8 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7be; continue;
 }
 0x106a7be => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7c4; continue;
 }
 0x106a7c4 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7ca; continue;
 }
 0x106a7ca => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7d0; continue;
 }
 0x106a7d0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7d6; continue;
 }
 0x106a7d6 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a7dc; continue;
 }
 0x106a7dc => {
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
  cpu.push(0x106a7fc); f_01049cd8(cpu); if cpu.returned_to != 0x106a7fc { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11296e5u32), 0x0u8); }
  if cpu.zf { pc = 0x106a80c; continue; }
  pc = 0x106a808; continue;
 }
 0x106a808 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x106a811; continue;
 }
 0x106a80c => {
  cpu.push(0x106a811); f_01068d2a(cpu); if cpu.returned_to != 0x106a811 { return; }
  pc = 0x106a811; continue;
 }
 0x106a811 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x106a821; continue; }
  pc = 0x106a81a; continue;
 }
 0x106a81a => {
  cpu.push(0x106a81f); f_0106aacb(cpu); if cpu.returned_to != 0x106a81f { return; }
  pc = 0x106a833; continue;
 }
 0x106a821 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x49u8); }
  if cpu.zf { pc = 0x106a833; continue; }
  pc = 0x106a827; continue;
 }
 0x106a827 => {
  { let v: u8 = 0x49u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a833; continue;
 }
 0x106a833 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a8df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a8df); }
 let mut pc: u32 = 0x106a8df;
 loop { match pc {
 0x106a8df => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a8e5; continue;
 }
 0x106a8e5 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a8eb; continue;
 }
 0x106a8eb => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a8f1; continue;
 }
 0x106a8f1 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a8f7; continue;
 }
 0x106a8f7 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a8fd; continue;
 }
 0x106a8fd => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a903; continue;
 }
 0x106a903 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a909; continue;
 }
 0x106a909 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a90f; continue;
 }
 0x106a90f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a915; continue;
 }
 0x106a915 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a91b; continue;
 }
 0x106a91b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a921; continue;
 }
 0x106a921 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a927; continue;
 }
 0x106a927 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x68u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x6au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106a942); f_01049cd8(cpu); if cpu.returned_to != 0x106a942 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x15u8); }
  if cpu.zf { pc = 0x106a957; continue; }
  pc = 0x106a94b; continue;
 }
 0x106a94b => {
  { let v: u8 = 0x15u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a957; continue;
 }
 0x106a957 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112b9d0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106a9f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106a9f7); }
 let mut pc: u32 = 0x106a9f7;
 loop { match pc {
 0x106a9f7 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa01; continue;
 }
 0x106aa01 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa0b; continue;
 }
 0x106aa0b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa15; continue;
 }
 0x106aa15 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa1f; continue;
 }
 0x106aa1f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1au8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa29; continue;
 }
 0x106aa29 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa33; continue;
 }
 0x106aa33 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa3d; continue;
 }
 0x106aa3d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa47; continue;
 }
 0x106aa47 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa4d; continue;
 }
 0x106aa4d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa53; continue;
 }
 0x106aa53 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa59; continue;
 }
 0x106aa59 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa5f; continue;
 }
 0x106aa5f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x29u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aa65; continue;
 }
 0x106aa65 => {
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
  cpu.push(0x106aa88); f_01049cd8(cpu); if cpu.returned_to != 0x106aa88 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11296e5u32), 0x0u8); }
  if cpu.zf { pc = 0x106aa9b; continue; }
  pc = 0x106aa94; continue;
 }
 0x106aa94 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x106aaa0; continue;
 }
 0x106aa9b => {
  cpu.push(0x106aaa0); f_01068d2a(cpu); if cpu.returned_to != 0x106aaa0 { return; }
  pc = 0x106aaa0; continue;
 }
 0x106aaa0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x106aab0; continue; }
  pc = 0x106aaa9; continue;
 }
 0x106aaa9 => {
  cpu.push(0x106aaae); f_0106aacb(cpu); if cpu.returned_to != 0x106aaae { return; }
  pc = 0x106aaca; continue;
 }
 0x106aab0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x14u8); }
  if cpu.zf { pc = 0x106aaca; continue; }
  pc = 0x106aab6; continue;
 }
 0x106aab6 => {
  { let v: u8 = 0x14u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x106aaca; continue;
 }
 0x106aaca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106aacb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106aacb); }
 let mut pc: u32 = 0x106aacb;
 loop { match pc {
 0x106aacb => {
  { cpu.sub8(cpu.r8(0x95u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106aad8; continue; }
  pc = 0x106aad4; continue;
 }
 0x106aad4 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106aad8 => {
  { let v: u8 = 0x42u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x68u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6au32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106aafb); f_01049cd8(cpu); if cpu.returned_to != 0x106aafb { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112b9d0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab06); }
 let mut pc: u32 = 0x106ab06;
 loop { match pc {
 0x106a834 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x106a83a; continue;
 }
 0x106a83a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106a85b; continue; }
  pc = 0x106a83f; continue;
 }
 0x106a83f => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106a856); f_0106a780(cpu); if cpu.returned_to != 0x106a856 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106a85b; continue;
 }
 0x106a85b => {
  { cpu.sub8(cpu.r8(0x112b9d0u32), 0x0u8); }
  if !cpu.zf { pc = 0x106a867; continue; }
  pc = 0x106a864; continue;
 }
 0x106a864 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106a83a; continue; }
  pc = 0x106a867; continue;
 }
 0x106a867 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a86d => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x106a873; continue;
 }
 0x106a873 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106a894; continue; }
  pc = 0x106a878; continue;
 }
 0x106a878 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106a88f); f_0106a6cc(cpu); if cpu.returned_to != 0x106a88f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106a894; continue;
 }
 0x106a894 => {
  { cpu.sub8(cpu.r8(0x112b9d0u32), 0x0u8); }
  if !cpu.zf { pc = 0x106a8a0; continue; }
  pc = 0x106a89d; continue;
 }
 0x106a89d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106a873; continue; }
  pc = 0x106a8a0; continue;
 }
 0x106a8a0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a8a6 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x106a8ac; continue;
 }
 0x106a8ac => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106a8cd; continue; }
  pc = 0x106a8b1; continue;
 }
 0x106a8b1 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106a8c8); f_0106a8df(cpu); if cpu.returned_to != 0x106a8c8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106a8cd; continue;
 }
 0x106a8cd => {
  { cpu.sub8(cpu.r8(0x112b9d0u32), 0x0u8); }
  if !cpu.zf { pc = 0x106a8d9; continue; }
  pc = 0x106a8d6; continue;
 }
 0x106a8d6 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106a8ac; continue; }
  pc = 0x106a8d9; continue;
 }
 0x106a8d9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106a95f => {
  { let v: u8 = 0x0u8; cpu.w8(0x112b9d0u32, v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106a9eb; continue; }
  pc = 0x106a96c; continue;
 }
 0x106a96c => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a8a6; continue; }
  pc = 0x106a98b; continue;
 }
 0x106a98b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { cpu.poll(); pc = 0x106a834; continue; }
  pc = 0x106a995; continue;
 }
 0x106a995 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a86d; continue; }
  pc = 0x106a99f; continue;
 }
 0x106a99f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11296e5u32, v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106a9bd; continue; }
  pc = 0x106a9af; continue;
 }
 0x106a9af => {
  { let v: u8 = 0x1u8; cpu.w8(0x11296e5u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106a9bd; continue;
 }
 0x106a9bd => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x106a9c3; continue;
 }
 0x106a9c3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106a9de; continue; }
  pc = 0x106a9c8; continue;
 }
 0x106a9c8 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x106a9dc); f_0106a9f7(cpu); if cpu.returned_to != 0x106a9dc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x106a9de; continue;
 }
 0x106a9de => {
  { cpu.sub8(cpu.r8(0x112b9d0u32), 0x0u8); }
  if !cpu.zf { pc = 0x106a9ea; continue; }
  pc = 0x106a9e7; continue;
 }
 0x106a9e7 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106a9c3; continue; }
  pc = 0x106a9ea; continue;
 }
 0x106a9ea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x106a9eb; continue;
 }
 0x106a9eb => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296e5u32, v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106ab06 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x106a95f; continue; }
  pc = 0x106ab15; continue;
 }
 0x106ab15 => {
  { let v: u8 = cpu.or8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106ab1e); f_0106b3b5(cpu); if cpu.returned_to != 0x106ab1e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab20); }
 let mut pc: u32 = 0x106ab20;
 loop { match pc {
 0x106ab20 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u8 = 0xefu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106ab38; continue;
 }
 0x106ab38 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x106ab4b; continue; }
  pc = 0x106ab41; continue;
 }
 0x106ab41 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & (cpu.edx as u8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106ab4b; continue;
 }
 0x106ab4b => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106ab38; continue; }
  pc = 0x106ab53; continue;
 }
 0x106ab53 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab58); }
 let mut pc: u32 = 0x106ab58;
 loop { match pc {
 0x106ab58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab59); }
 let mut pc: u32 = 0x106ab59;
 loop { match pc {
 0x106ab59 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab5a); }
 let mut pc: u32 = 0x106ab5a;
 loop { match pc {
 0x106ab5a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab5c); }
 let mut pc: u32 = 0x106ab5c;
 loop { match pc {
 0x106ab5c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ab5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ab5e); }
 let mut pc: u32 = 0x106ab5e;
 loop { match pc {
 0x106ab5e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ad8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ad8a); }
 let mut pc: u32 = 0x106ad8a;
 loop { match pc {
 0x106ad8a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x106add1; continue; }
  pc = 0x106ad96; continue;
 }
 0x106ad96 => {
  cpu.push(0x106ad9b); f_01066cca(cpu); if cpu.returned_to != 0x106ad9b { return; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add(0xau16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x8au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x106adbb; continue; }
  pc = 0x106adb0; continue;
 }
 0x106adb0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106adbb; continue;
 }
 0x106adbb => {
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106add1; continue;
 }
 0x106add1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106add3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106add3); }
 let mut pc: u32 = 0x106add3;
 loop { match pc {
 0x106add3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106ade3); f_01018e6e(cpu); if cpu.returned_to != 0x106ade3 { return; }
  if !cpu.cf { pc = 0x106adf9; continue; }
  pc = 0x106ade5; continue;
 }
 0x106ade5 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x106adf9; continue; }
  pc = 0x106adf1; continue;
 }
 0x106adf1 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  cpu.push(0x106adf9); f_01050829(cpu); if cpu.returned_to != 0x106adf9 { return; }
  pc = 0x106adf9; continue;
 }
 0x106adf9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106adfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106adfb); }
 let mut pc: u32 = 0x106adfb;
 loop { match pc {
 0x106adfb => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x106ae1b; continue; }
  pc = 0x106ae02; continue;
 }
 0x106ae02 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106ae11); f_01018f3f(cpu); if cpu.returned_to != 0x106ae11 { return; }
  if !cpu.cf { pc = 0x106ae1b; continue; }
  pc = 0x106ae13; continue;
 }
 0x106ae13 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  cpu.push(0x106ae1b); f_01050829(cpu); if cpu.returned_to != 0x106ae1b { return; }
  pc = 0x106ae1b; continue;
 }
 0x106ae1b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b048); }
 let mut pc: u32 = 0x106b048;
 loop { match pc {
 0x106b048 => {
  { let v: u32 = 0x3au32; cpu.ebx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b1c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b1c0); }
 let mut pc: u32 = 0x106b1c0;
 loop { match pc {
 0x106b1c0 => {
  { let v: u16 = cpu.r16(0x11056a6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11056a8u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106b1d2); f_01018f3f(cpu); if cpu.returned_to != 0x106b1d2 { return; }
  if cpu.cf { pc = 0x106b1e0; continue; }
  pc = 0x106b1d4; continue;
 }
 0x106b1d4 => {
  cpu.push(0x106b1d9); f_0101acb0(cpu); if cpu.returned_to != 0x106b1d9 { return; }
  if cpu.cf { pc = 0x106b1e0; continue; }
  pc = 0x106b1db; continue;
 }
 0x106b1db => {
  cpu.push(0x106b1e0); f_010361d8(cpu); if cpu.returned_to != 0x106b1e0 { return; }
  pc = 0x106b1e0; continue;
 }
 0x106b1e0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b1e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b1e1); }
 let mut pc: u32 = 0x106b1e1;
 loop { match pc {
 0x106b1e1 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106b20e); f_010512c6(cpu); if cpu.returned_to != 0x106b20e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x106b214); f_0106a04e(cpu); if cpu.returned_to != 0x106b214 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { f_0106b3b5(cpu); return; }
  pc = 0x106b21b; continue;
 }
 0x106b21b => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u8 = cpu.neg8(cpu.r8(0x5au32.wrapping_add(cpu.ebp))); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b224); }
 let mut pc: u32 = 0x106b224;
 loop { match pc {
 0x106b224 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x106b29d; continue; }
  pc = 0x106b22d; continue;
 }
 0x106b22d => {
  cpu.push(0x106b232); f_01063324(cpu); if cpu.returned_to != 0x106b232 { return; }
  if cpu.cf { pc = 0x106b28b; continue; }
  pc = 0x106b234; continue;
 }
 0x106b234 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x106b28b; continue; }
  pc = 0x106b23a; continue;
 }
 0x106b23a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x106b29d; continue; }
  pc = 0x106b240; continue;
 }
 0x106b240 => {
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
  if (cpu.sf != cpu.of) { pc = 0x106b28b; continue; }
  pc = 0x106b26e; continue;
 }
 0x106b26e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x1388u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.cf || cpu.zf) { pc = 0x106b286; continue; }
  pc = 0x106b281; continue;
 }
 0x106b281 => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  pc = 0x106b286; continue;
 }
 0x106b286 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b28b => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (cpu.sf == cpu.of) { pc = 0x106b298; continue; }
  pc = 0x106b296; continue;
 }
 0x106b296 => {
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b298; continue;
 }
 0x106b298 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b29d => {
  { let v: u8 = 0x7fu8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b2a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b2a3); }
 let mut pc: u32 = 0x106b2a3;
 loop { match pc {
 0x106b2a3 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x106b2d8; continue; }
  pc = 0x106b2b2; continue;
 }
 0x106b2b2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x106b2c7); f_0106b361(cpu); if cpu.returned_to != 0x106b2c7 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106b2e2; continue; }
  pc = 0x106b2cd; continue;
 }
 0x106b2cd => {
  cpu.push(0x106b2d2); f_01066aef(cpu); if cpu.returned_to != 0x106b2d2 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x106b2d8; continue;
 }
 0x106b2d8 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b2e2 => {
  cpu.push(0x106b2e7); f_01050245(cpu); if cpu.returned_to != 0x106b2e7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x106b2f1); f_0106a326(cpu); if cpu.returned_to != 0x106b2f1 { return; }
  { let v: u32 = 0x9u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x106b355; continue; }
  pc = 0x106b2fc; continue;
 }
 0x106b2fc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106b326; continue; }
  pc = 0x106b302; continue;
 }
 0x106b302 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106b33b; continue; }
  pc = 0x106b308; continue;
 }
 0x106b308 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106b350; continue; }
  pc = 0x106b30e; continue;
 }
 0x106b30e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106b342; continue; }
  pc = 0x106b314; continue;
 }
 0x106b314 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106b32d; continue; }
  pc = 0x106b31a; continue;
 }
 0x106b31a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106b334; continue; }
  pc = 0x106b320; continue;
 }
 0x106b320 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106b349; continue; }
  pc = 0x106b326; continue;
 }
 0x106b326 => {
  { let v: u32 = 0x36u32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b32d => {
  { let v: u32 = 0x3au32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b334 => {
  { let v: u32 = 0x3bu32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b33b => {
  { let v: u32 = 0x37u32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b342 => {
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b349 => {
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b350 => {
  { let v: u32 = 0x35u32; cpu.ebx = v; }
  pc = 0x106b355; continue;
 }
 0x106b355 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b361(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b361); }
 let mut pc: u32 = 0x106b361;
 loop { match pc {
 0x106b361 => {
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x54u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106b37f; continue; }
  pc = 0x106b37b; continue;
 }
 0x106b37b => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x106b37f; continue;
 }
 0x106b37f => {
  { let v: u8 = 0x5u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b38c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b38c); }
 let mut pc: u32 = 0x106b38c;
 loop { match pc {
 0x106b38c => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
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
pub fn f_0106b3b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b3b2); }
 let mut pc: u32 = 0x106b3b2;
 loop { match pc {
 0x106b3b2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b3b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b3b5); }
 let mut pc: u32 = 0x106b3b5;
 loop { match pc {
 0x106b3b5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x106b3c5; continue; }
  pc = 0x106b3c3; continue;
 }
 0x106b3c3 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b3c5; continue;
 }
 0x106b3c5 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x106b3e0; continue; }
  pc = 0x106b3c9; continue;
 }
 0x106b3c9 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x106b40a; continue; }
  pc = 0x106b3cd; continue;
 }
 0x106b3cd => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x106b40a; continue; }
  pc = 0x106b3d1; continue;
 }
 0x106b3d1 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x106b3d7; continue; }
  pc = 0x106b3d5; continue;
 }
 0x106b3d5 => {
  pc = 0x106b411; continue;
 }
 0x106b3d7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x106b490; continue;
 }
 0x106b3e0 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x50u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x106b490; continue; }
  pc = 0x106b3f2; continue;
 }
 0x106b3f2 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x52u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x106b490; continue; }
  pc = 0x106b404; continue;
 }
 0x106b404 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  pc = 0x106b411; continue;
 }
 0x106b40a => {
  cpu.push(0x106b40f); f_0106b492(cpu); if cpu.returned_to != 0x106b40f { return; }
  if cpu.cf { pc = 0x106b490; continue; }
  pc = 0x106b411; continue;
 }
 0x106b411 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106b490; continue; }
  pc = 0x106b417; continue;
 }
 0x106b417 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x106b490; continue; }
  pc = 0x106b41d; continue;
 }
 0x106b41d => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x106b432; continue; }
  pc = 0x106b424; continue;
 }
 0x106b424 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x106b460; continue; }
  pc = 0x106b428; continue;
 }
 0x106b428 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x106b460; continue; }
  pc = 0x106b42c; continue;
 }
 0x106b42c => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x106b460; continue; }
  pc = 0x106b430; continue;
 }
 0x106b430 => {
  pc = 0x106b490; continue;
 }
 0x106b432 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x106b467; continue; }
  pc = 0x106b440; continue;
 }
 0x106b440 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x106b467; continue; }
  pc = 0x106b44e; continue;
 }
 0x106b44e => {
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106b45a); f_01059da3(cpu); if cpu.returned_to != 0x106b45a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  pc = 0x106b490; continue;
 }
 0x106b460 => {
  cpu.push(0x106b465); f_0106b546(cpu); if cpu.returned_to != 0x106b465 { return; }
  if !cpu.cf { pc = 0x106b490; continue; }
  pc = 0x106b467; continue;
 }
 0x106b467 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106b490; continue; }
  pc = 0x106b46d; continue;
 }
 0x106b46d => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x106b475); f_0106586c(cpu); if cpu.returned_to != 0x106b475 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106b490; continue;
 }
 0x106b490 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b492(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b492); }
 let mut pc: u32 = 0x106b492;
 loop { match pc {
 0x106b492 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106b4e8; continue; }
  pc = 0x106b498; continue;
 }
 0x106b498 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106b4e8; continue; }
  pc = 0x106b49e; continue;
 }
 0x106b49e => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if cpu.zf { pc = 0x106b4bb; continue; }
  pc = 0x106b4a6; continue;
 }
 0x106b4a6 => {
  { cpu.pushad(); }
  cpu.push(0x106b4ac); f_01063324(cpu); if cpu.returned_to != 0x106b4ac { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x106b53e; continue; }
  pc = 0x106b4b9; continue;
 }
 0x106b4b9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b4bb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106b4c9); f_0101ac6c(cpu); if cpu.returned_to != 0x106b4c9 { return; }
  if !cpu.cf { pc = 0x106b4cf; continue; }
  pc = 0x106b4cb; continue;
 }
 0x106b4cb => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b4e1; continue;
 }
 0x106b4cf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106b4d6); f_01039557(cpu); if cpu.returned_to != 0x106b4d6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x106b4dc; continue; }
  pc = 0x106b4da; continue;
 }
 0x106b4da => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b4dc => {
  cpu.push(0x106b4e1); f_01009e97(cpu); if cpu.returned_to != 0x106b4e1 { return; }
  pc = 0x106b4e1; continue;
 }
 0x106b4e1 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b4e6; continue;
 }
 0x106b4e6 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b4e8 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xfb2eu16); }
  if cpu.zf { pc = 0x106b525; continue; }
  pc = 0x106b4f0; continue;
 }
 0x106b4f0 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xef1fu16); }
  if cpu.zf { pc = 0x106b50c; continue; }
  pc = 0x106b4f8; continue;
 }
 0x106b4f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106b506); f_01009e97(cpu); if cpu.returned_to != 0x106b506 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b50b; continue;
 }
 0x106b50b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b50c => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1152e54u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b51c; continue;
 }
 0x106b51c => {
  cpu.push(0x106b521); f_0106b83f(cpu); if cpu.returned_to != 0x106b521 { return; }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b523; continue;
 }
 0x106b523 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b525 => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x11757a4u32); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b535; continue;
 }
 0x106b535 => {
  cpu.push(0x106b53a); f_0106b855(cpu); if cpu.returned_to != 0x106b53a { return; }
  if cpu.zf { pc = 0x106b53e; continue; }
  pc = 0x106b53c; continue;
 }
 0x106b53c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b53e => {
  { let v: u16 = 0x0u16; cpu.w16(0x5au32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b546(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b546); }
 let mut pc: u32 = 0x106b546;
 loop { match pc {
 0x106b546 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106b5f2; continue; }
  pc = 0x106b550; continue;
 }
 0x106b550 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106b5f2; continue; }
  pc = 0x106b55a; continue;
 }
 0x106b55a => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xd80u16); }
  if cpu.zf { pc = 0x106b59e; continue; }
  pc = 0x106b562; continue;
 }
 0x106b562 => {
  { let v = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  cpu.push(0x106b57b); f_01063324(cpu); if cpu.returned_to != 0x106b57b { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x106b647; continue; }
  pc = 0x106b58b; continue;
 }
 0x106b58b => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.edx = v; }
  cpu.push(0x106b596); f_0106b83f(cpu); if cpu.returned_to != 0x106b596 { return; }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b59c; continue;
 }
 0x106b59c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b59e => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106b5ac); f_0101ac6c(cpu); if cpu.returned_to != 0x106b5ac { return; }
  if !cpu.cf { pc = 0x106b5b2; continue; }
  pc = 0x106b5ae; continue;
 }
 0x106b5ae => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b5eb; continue;
 }
 0x106b5b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x106b5b9); f_01039557(cpu); if cpu.returned_to != 0x106b5b9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x106b5bf; continue; }
  pc = 0x106b5bd; continue;
 }
 0x106b5bd => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b5bf => {
  cpu.push(0x106b5c4); f_0104f36d(cpu); if cpu.returned_to != 0x106b5c4 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x17u16); }
  if !cpu.zf { pc = 0x106b5d7; continue; }
  pc = 0x106b5d3; continue;
 }
 0x106b5d3 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b5eb; continue;
 }
 0x106b5d7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x106b5e5; continue; }
  pc = 0x106b5dd; continue;
 }
 0x106b5dd => {
  { let v: u8 = cpu.r8(0x10f7214u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b5eb; continue;
 }
 0x106b5e5 => {
  { let v: u8 = cpu.r8(0x10f71d1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106b5eb; continue;
 }
 0x106b5eb => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b5f0; continue;
 }
 0x106b5f0 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b5f2 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x106b62e; continue; }
  pc = 0x106b5fa; continue;
 }
 0x106b5fa => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if cpu.zf { pc = 0x106b615; continue; }
  pc = 0x106b602; continue;
 }
 0x106b602 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106b610); f_0106509f(cpu); if cpu.returned_to != 0x106b610 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { pc = 0x106b647; continue; }
  pc = 0x106b613; continue;
 }
 0x106b613 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b615 => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1152e54u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b625; continue;
 }
 0x106b625 => {
  cpu.push(0x106b62a); f_0106b83f(cpu); if cpu.returned_to != 0x106b62a { return; }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b62c; continue;
 }
 0x106b62c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b62e => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x11757a4u32); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b63e; continue;
 }
 0x106b63e => {
  cpu.push(0x106b643); f_0106b855(cpu); if cpu.returned_to != 0x106b643 { return; }
  if cpu.zf { pc = 0x106b647; continue; }
  pc = 0x106b645; continue;
 }
 0x106b645 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b647 => {
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106b653); f_01059da3(cpu); if cpu.returned_to != 0x106b653 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b659); }
 let mut pc: u32 = 0x106b659;
 loop { match pc {
 0x106b659 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x106b6a7; continue; }
  pc = 0x106b665; continue;
 }
 0x106b665 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106b6a7; continue; }
  pc = 0x106b66b; continue;
 }
 0x106b66b => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xcu8); }
  if cpu.zf { pc = 0x106b677; continue; }
  pc = 0x106b671; continue;
 }
 0x106b671 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x106b6a7; continue; }
  pc = 0x106b677; continue;
 }
 0x106b677 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x106b684; continue; }
  pc = 0x106b67d; continue;
 }
 0x106b67d => {
  cpu.push(0x106b682); f_01047a18(cpu); if cpu.returned_to != 0x106b682 { return; }
  if cpu.zf { pc = 0x106b6a7; continue; }
  pc = 0x106b684; continue;
 }
 0x106b684 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x106b68c); f_0106586c(cpu); if cpu.returned_to != 0x106b68c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x30u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106b6a7; continue;
 }
 0x106b6a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b6a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b6a8); }
 let mut pc: u32 = 0x106b6a8;
 loop { match pc {
 0x106b6a8 => {
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xb1u32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106b6c1); f_01059da3(cpu); if cpu.returned_to != 0x106b6c1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106b6dd; continue; }
  pc = 0x106b6cf; continue;
 }
 0x106b6cf => {
  cpu.push(0x106b6d4); f_01053da5(cpu); if cpu.returned_to != 0x106b6d4 { return; }
  if cpu.cf { pc = 0x106b6da; continue; }
  pc = 0x106b6d6; continue;
 }
 0x106b6d6 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b6da => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b6dd => {
  { let v: u32 = 0x0u32; cpu.w32(0x1129390u32, v); }
  cpu.push(0x106b6ec); f_0106ad8a(cpu); if cpu.returned_to != 0x106b6ec { return; }
  cpu.push(0x106b6f1); f_010675fd(cpu); if cpu.returned_to != 0x106b6f1 { return; }
  { cpu.sub32(cpu.r32(0x1129390u32), 0x0u32); }
  if cpu.zf { pc = 0x106b712; continue; }
  pc = 0x106b6fa; continue;
 }
 0x106b6fa => {
  { let v: u32 = cpu.r32(0x1129390u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x11a188cu32); cpu.eax = v; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x99u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b712 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = (cpu.eax as u8); cpu.div8(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b718); }
 let mut pc: u32 = 0x106b718;
 loop { match pc {
 0x106b718 => {
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x106b732; continue; }
  pc = 0x106b727; continue;
 }
 0x106b727 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x106b730); f_01002832(cpu); if cpu.returned_to != 0x106b730 { return; }
  pc = 0x106b763; continue;
 }
 0x106b732 => {
  { cpu.sub16((cpu.edx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x106b743; continue; }
  pc = 0x106b738; continue;
 }
 0x106b738 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x106b741); f_01002935(cpu); if cpu.returned_to != 0x106b741 { return; }
  pc = 0x106b763; continue;
 }
 0x106b743 => {
  { cpu.sub16((cpu.edx as u16), 0x2u16); }
  if !cpu.zf { pc = 0x106b754; continue; }
  pc = 0x106b749; continue;
 }
 0x106b749 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x106b752); f_01002a3a(cpu); if cpu.returned_to != 0x106b752 { return; }
  pc = 0x106b763; continue;
 }
 0x106b754 => {
  { cpu.sub16((cpu.edx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x106b763; continue; }
  pc = 0x106b75a; continue;
 }
 0x106b75a => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x106b763); f_01002b38(cpu); if cpu.returned_to != 0x106b763 { return; }
  pc = 0x106b763; continue;
 }
 0x106b763 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b764(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b764); }
 let mut pc: u32 = 0x106b764;
 loop { match pc {
 0x106b764 => {
  { let v: u32 = cpu.r16(0x10e3d1fu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d21u32) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10e3e11u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if cpu.zf { pc = 0x106b7b1; continue; }
  pc = 0x106b77d; continue;
 }
 0x106b77d => {
  { let v: u32 = cpu.r16(0x10e3d1bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d1du32) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x106b7b1; continue; }
  pc = 0x106b790; continue;
 }
 0x106b790 => {
  { let v: u32 = cpu.r16(0x10e3d23u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d25u32) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x106b7b1; continue; }
  pc = 0x106b7a3; continue;
 }
 0x106b7a3 => {
  { let v: u32 = cpu.r16(0x10e3d27u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d29u32) as u32; cpu.ebx = v; }
  pc = 0x106b7b1; continue;
 }
 0x106b7b1 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x48u32); cpu.ebx = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10e3e0fu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x106b7c7; continue;
 }
 0x106b7c7 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x106b82f; continue; }
  pc = 0x106b7cf; continue;
 }
 0x106b7cf => {
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x106b82f; continue; }
  pc = 0x106b7d6; continue;
 }
 0x106b7d6 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x106b82f; continue; }
  pc = 0x106b7dc; continue;
 }
 0x106b7dc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x106b82f; continue; }
  pc = 0x106b7e2; continue;
 }
 0x106b7e2 => {
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
  cpu.push(0x106b807); f_0106b839(cpu); if cpu.returned_to != 0x106b807 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.edi), v); }
  pc = 0x106b82f; continue;
 }
 0x106b82f => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x106b7c7; continue; }
  pc = 0x106b838; continue;
 }
 0x106b838 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b83f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b83f); }
 let mut pc: u32 = 0x106b83f;
 loop { match pc {
 0x106b83f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b855); }
 let mut pc: u32 = 0x106b855;
 loop { match pc {
 0x106b855 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106b882(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106b882); }
 let mut pc: u32 = 0x106b882;
 loop { match pc {
 0x106b882 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x106b88a; continue; }
  pc = 0x106b889; continue;
 }
 0x106b889 => {
  {  }
  pc = 0x106b88a; continue;
 }
 0x106b88a => {
  { cpu.sub16((cpu.ecx as u16), 0x1c2au16); }
  if (cpu.cf || cpu.zf) { pc = 0x106b8a1; continue; }
  pc = 0x106b891; continue;
 }
 0x106b891 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x105592cu32, v); }
  cpu.push(0x106b89f); f_01054675(cpu); if cpu.returned_to != 0x106b89f { return; }
  pc = 0x106b8c5; continue;
 }
 0x106b8a1 => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x106b8c8; continue; }
  pc = 0x106b8af; continue;
 }
 0x106b8af => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  pc = 0x106b8c5; continue;
 }
 0x106b8c5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106b8c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106bf86(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106bf86); } r_0106bf86(cpu, 0x106bf86); }
pub fn f_0106bfae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106bfae); } r_0106bf86(cpu, 0x106bfae); }
fn r_0106bf86(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x106bf86 => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x10e4978u32); }
  if cpu.zf { pc = 0x106c056; continue; }
  pc = 0x106bf9b; continue;
 }
 0x106bf9b => {
  { let v: u32 = 0x10e4970u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x106c056; continue; }
  pc = 0x106bfa8; continue;
 }
 0x106bfa8 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x106bfae; continue;
 }
 0x106bfae => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x106c056; continue; }
  pc = 0x106bfb6; continue;
 }
 0x106bfb6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x106bfbf; continue;
 }
 0x106bfbf => {
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x106c008; continue; }
  pc = 0x106bfc3; continue;
 }
 0x106bfc3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x106bfc3; continue; }
  pc = 0x106bfcb; continue;
 }
 0x106bfcb => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x106bfcb; continue; }
  pc = 0x106bfd3; continue;
 }
 0x106bfd3 => {
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10e3fc4u32, v); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10e3fc8u32, v); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_sub(0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x6u32); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x10e3fc4u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x6u32); cpu.edi = v; }
  cpu.poll(); pc = 0x106bfbf; continue;
 }
 0x106c008 => {
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
  { let v: u32 = 0x10e3fc4u32; cpu.esi = v; }
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
  cpu.push(0x106c045); f_0106bfae(cpu); if cpu.returned_to != 0x106c045 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  cpu.push(0x106c053); f_0106bfae(cpu); if cpu.returned_to != 0x106c053 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x106c056; continue;
 }
 0x106c056 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c057(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c057); }
 let mut pc: u32 = 0x106c057;
 loop { match pc {
 0x106c057 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10c052cu32, v); }
  cpu.push(0x106c065); f_0106bf86(cpu); if cpu.returned_to != 0x106c065 { return; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10e4978u32); cpu.ecx = v; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106c0ea; continue; }
  pc = 0x106c077; continue;
 }
 0x106c077 => {
  { let v: u32 = 0x10e4978u32; cpu.esi = v; }
  pc = 0x106c07c; continue;
 }
 0x106c07c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef324u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x106c106; continue; }
  pc = 0x106c08f; continue;
 }
 0x106c08f => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x106c0eb; continue; }
  pc = 0x106c09d; continue;
 }
 0x106c09d => {
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10fcc16u32), 0x1u8); }
  if !cpu.zf { pc = 0x106c0ae; continue; }
  pc = 0x106c0a9; continue;
 }
 0x106c0a9 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  pc = 0x106c0ae; continue;
 }
 0x106c0ae => {
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x106c0b5); cpu.call(t); if cpu.returned_to != 0x106c0b5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1105661u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1105663u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1105665u32, v); }
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x106c0e2); cpu.call(t); if cpu.returned_to != 0x106c0e2 { return; }
  pc = 0x106c0e2; continue;
 }
 0x106c0e2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x106c07c; continue; }
  pc = 0x106c0ea; continue;
 }
 0x106c0ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c0eb => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106c104); cpu.call(t); if cpu.returned_to != 0x106c104 { return; }
  cpu.poll(); pc = 0x106c0e2; continue;
 }
 0x106c106 => {
  { let v = cpu.r32(0x10e4100u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112be26u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10e55fau32); cpu.esi = v; }
  pc = 0x106c12a; continue;
 }
 0x106c12a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10c052cu32, v); }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x106c14d; continue; }
  pc = 0x106c13a; continue;
 }
 0x106c13a => {
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4130u32, v); }
  pc = 0x106c157; continue;
 }
 0x106c14d => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  pc = 0x106c157; continue;
 }
 0x106c157 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x106c25b; continue; }
  pc = 0x106c165; continue;
 }
 0x106c165 => {
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10fcc16u32), 0x1u8); }
  if !cpu.zf { pc = 0x106c176; continue; }
  pc = 0x106c171; continue;
 }
 0x106c171 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  pc = 0x106c176; continue;
 }
 0x106c176 => {
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x106c17d); cpu.call(t); if cpu.returned_to != 0x106c17d { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x106c1ac; continue; }
  pc = 0x106c18e; continue;
 }
 0x106c18e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4100u32)); }
  if !cpu.sf { pc = 0x106c1a5; continue; }
  pc = 0x106c1a3; continue;
 }
 0x106c1a3 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x106c1a5; continue;
 }
 0x106c1a5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4100u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106c1ac; continue;
 }
 0x106c1ac => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x106c1d6; continue; }
  pc = 0x106c1b2; continue;
 }
 0x106c1b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c1cf; continue; }
  pc = 0x106c1c7; continue;
 }
 0x106c1c7 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106c1cf; continue;
 }
 0x106c1cf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106c1d6; continue;
 }
 0x106c1d6 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x106c1fd; continue; }
  pc = 0x106c1dc; continue;
 }
 0x106c1dc => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106c1fd; continue; }
  pc = 0x106c1ed; continue;
 }
 0x106c1ed => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c237; continue; }
  pc = 0x106c1f6; continue;
 }
 0x106c1f6 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e4108u32, v); }
  pc = 0x106c1fd; continue;
 }
 0x106c1fd => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x106c211; continue; }
  pc = 0x106c203; continue;
 }
 0x106c203 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x112be26u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106c211; continue;
 }
 0x106c211 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x106c21f; continue; }
  pc = 0x106c217; continue;
 }
 0x106c217 => {
  { let v: u16 = cpu.r8(0xeu32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c21f; continue;
 }
 0x106c21f => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x106c22f; continue; }
  pc = 0x106c225; continue;
 }
 0x106c225 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x106c22c); cpu.call(t); if cpu.returned_to != 0x106c22c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106c237; continue;
 }
 0x106c22f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106c236); cpu.call(t); if cpu.returned_to != 0x106c236 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106c237; continue;
 }
 0x106c237 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x106c245; continue; }
  pc = 0x106c23d; continue;
 }
 0x106c23d => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  cpu.poll(); pc = 0x106c12a; continue;
 }
 0x106c245 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4100u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.poll(); pc = 0x106c0e2; continue;
 }
 0x106c25b => {
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x106c268; continue; }
  pc = 0x106c266; continue;
 }
 0x106c266 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x106c268; continue;
 }
 0x106c268 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x106c28e; continue; }
  pc = 0x106c286; continue;
 }
 0x106c286 => {
  { let v: u16 = cpu.r8(0xeu32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c28e; continue;
 }
 0x106c28e => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x106c295); cpu.call(t); if cpu.returned_to != 0x106c295 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x106c237; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c298); }
 let mut pc: u32 = 0x106c298;
 loop { match pc {
 0x106c298 => {
  { cpu.sub8(cpu.r8(0x110ed8au32), 0x0u8); }
  if !cpu.zf { pc = 0x106c304; continue; }
  pc = 0x106c2a1; continue;
 }
 0x106c2a1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xbu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106c2b9); f_0104f36d(cpu); if cpu.returned_to != 0x106c2b9 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10f742cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x93u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x106c2e3; continue; }
  pc = 0x106c2d3; continue;
 }
 0x106c2d3 => {
  if (cpu.sf != cpu.of) { pc = 0x106c2dd; continue; }
  pc = 0x106c2d5; continue;
 }
 0x106c2d5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.w8(0x93u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106c2e3; continue;
 }
 0x106c2dd => {
  { let v: u8 = cpu.dec8(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.w8(0x93u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106c2e3; continue;
 }
 0x106c2e3 => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.cf { pc = 0x106c306; continue; }
  pc = 0x106c2ec; continue;
 }
 0x106c2ec => {
  { cpu.sub8(cpu.r8(0x93u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x106c304; continue; }
  pc = 0x106c2f5; continue;
 }
 0x106c2f5 => {
  cpu.push(0x106c2fa); f_0106c74a(cpu); if cpu.returned_to != 0x106c2fa { return; }
  if !cpu.cf { pc = 0x106c2fd; continue; }
  pc = 0x106c2fc; continue;
 }
 0x106c2fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c2fd => {
  cpu.push(0x106c302); f_0106c30d(cpu); if cpu.returned_to != 0x106c302 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c304 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c306 => {
  cpu.push(0x106c30b); f_0106c700(cpu); if cpu.returned_to != 0x106c30b { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c30d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c30d); }
 let mut pc: u32 = 0x106c30d;
 loop { match pc {
 0x106c30d => {
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106c321; continue; }
  pc = 0x106c31b; continue;
 }
 0x106c31b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106c325; continue; }
  pc = 0x106c321; continue;
 }
 0x106c321 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106c325; continue;
 }
 0x106c325 => {
  cpu.push(0x106c32a); f_0105f2ec(cpu); if cpu.returned_to != 0x106c32a { return; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if !cpu.zf { pc = 0x106c335; continue; }
  pc = 0x106c330; continue;
 }
 0x106c330 => {
  cpu.push(0x106c335); f_010526ae(cpu); if cpu.returned_to != 0x106c335 { return; }
  pc = 0x106c335; continue;
 }
 0x106c335 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if !cpu.zf { pc = 0x106c340; continue; }
  pc = 0x106c33b; continue;
 }
 0x106c33b => {
  cpu.push(0x106c340); f_010525ce(cpu); if cpu.returned_to != 0x106c340 { return; }
  pc = 0x106c340; continue;
 }
 0x106c340 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c424; continue; }
  pc = 0x106c355; continue;
 }
 0x106c355 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106c424; continue; }
  pc = 0x106c366; continue;
 }
 0x106c366 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c424; continue; }
  pc = 0x106c37f; continue;
 }
 0x106c37f => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106c424; continue; }
  pc = 0x106c390; continue;
 }
 0x106c390 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10e55fau32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x44u8); }
  if cpu.zf { pc = 0x106c425; continue; }
  pc = 0x106c3bd; continue;
 }
 0x106c3bd => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x43u8); }
  if cpu.zf { pc = 0x106c571; continue; }
  pc = 0x106c3c7; continue;
 }
 0x106c3c7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x112ba18u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
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
  if cpu.zf { pc = 0x106c40b; continue; }
  pc = 0x106c407; continue;
 }
 0x106c407 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106c40b; continue;
 }
 0x106c40b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x106c424; continue;
 }
 0x106c424 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c425 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112be25u32, v); }
  { cpu.sub8(cpu.r8(0x112be20u32), 0x4u8); }
  if cpu.zf { pc = 0x106c43f; continue; }
  pc = 0x106c435; continue;
 }
 0x106c435 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c4b4; continue; }
  pc = 0x106c43d; continue;
 }
 0x106c43d => {
  pc = 0x106c447; continue;
 }
 0x106c43f => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c4b4; continue; }
  pc = 0x106c447; continue;
 }
 0x106c447 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112be21u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112be23u32, v); }
  { let v: u16 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x112ba18u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x93u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x106c496; continue; }
  pc = 0x106c492; continue;
 }
 0x106c492 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106c496; continue;
 }
 0x106c496 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u16 = cpu.r16(0x112be21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112be23u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c4b4; continue;
 }
 0x106c4b4 => {
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x112be20u32), 0x4u8); }
  if cpu.zf { pc = 0x106c4d0; continue; }
  pc = 0x106c4c6; continue;
 }
 0x106c4c6 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c4e8; continue; }
  pc = 0x106c4ce; continue;
 }
 0x106c4ce => {
  pc = 0x106c4d8; continue;
 }
 0x106c4d0 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c4e8; continue; }
  pc = 0x106c4d8; continue;
 }
 0x106c4d8 => {
  { let v: u16 = cpu.r16(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c50b; continue;
 }
 0x106c4e8 => {
  { cpu.sub8(cpu.r8(0x112be20u32), 0x4u8); }
  if cpu.zf { pc = 0x106c4ff; continue; }
  pc = 0x106c4f1; continue;
 }
 0x106c4f1 => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c50b; continue; }
  pc = 0x106c4f9; continue;
 }
 0x106c4f9 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c50b; continue;
 }
 0x106c4ff => {
  { cpu.sub16(cpu.r16(0x32u32.wrapping_add(cpu.ebp)), 0x7u16); }
  if (cpu.sf == cpu.of) { pc = 0x106c50b; continue; }
  pc = 0x106c507; continue;
 }
 0x106c507 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c50b; continue;
 }
 0x106c50b => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x44u32; cpu.ebx = v; }
  { let v: u32 = 0x112ba18u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x106c545; continue; }
  pc = 0x106c53d; continue;
 }
 0x106c53d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106c547; continue;
 }
 0x106c545 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106c547; continue;
 }
 0x106c547 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x32u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c571 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112be20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112be21u32, v); }
  { let v: u16 = cpu.r8(0x93u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112be23u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x43u32; cpu.ebx = v; }
  { let v: u32 = 0x112ba18u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
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
  if cpu.zf { pc = 0x106c5da; continue; }
  pc = 0x106c5d6; continue;
 }
 0x106c5d6 => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106c5da; continue;
 }
 0x106c5da => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { cpu.sub8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106c5fa; continue; }
  pc = 0x106c5f1; continue;
 }
 0x106c5f1 => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  pc = 0x106c6f7; continue;
 }
 0x106c5fa => {
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x7fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112be21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112be23u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106c619; continue;
 }
 0x106c619 => {
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x106c676; continue; }
  pc = 0x106c61f; continue;
 }
 0x106c61f => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x106c676; continue; }
  pc = 0x106c628; continue;
 }
 0x106c628 => {
  { cpu.sub8(cpu.r8(0x81u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if (cpu.sf == cpu.of) { pc = 0x106c676; continue; }
  pc = 0x106c631; continue;
 }
 0x106c631 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x81u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x112be20u32), 0x1u8); }
  if !cpu.zf { pc = 0x106c656; continue; }
  pc = 0x106c650; continue;
 }
 0x106c650 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1074u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106c65a; continue;
 }
 0x106c656 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x187du16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106c65a; continue;
 }
 0x106c65a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x40u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x106c676; continue;
 }
 0x106c676 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1737u16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x106c68e; continue; }
  pc = 0x106c688; continue;
 }
 0x106c688 => {
  { let v: u16 = 0x1738u16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x106c68e; continue;
 }
 0x106c68e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x40u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x106c6a6; continue; }
  pc = 0x106c6a2; continue;
 }
 0x106c6a2 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x106c6a6; continue;
 }
 0x106c6a6 => {
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x106c619; continue; }
  pc = 0x106c6bb; continue;
 }
 0x106c6bb => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e401cu32, v); }
  { cpu.and8(cpu.r8(0x7cu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x106c6f7; continue; }
  pc = 0x106c6c7; continue;
 }
 0x106c6c7 => {
  { let v: u8 = cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1739u16; cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  pc = 0x106c6f7; continue;
 }
 0x106c6f7 => {
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c700); }
 let mut pc: u32 = 0x106c700;
 loop { match pc {
 0x106c700 => {
  { cpu.pushad(); }
  cpu.push(0x106c706); f_0105f215(cpu); if cpu.returned_to != 0x106c706 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x106c748; continue; }
  pc = 0x106c70b; continue;
 }
 0x106c70b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x106c748); f_01075c8b(cpu); if cpu.returned_to != 0x106c748 { return; }
  pc = 0x106c748; continue;
 }
 0x106c748 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c74a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c74a); }
 let mut pc: u32 = 0x106c74a;
 loop { match pc {
 0x106c74a => {
  { cpu.pushad(); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2402u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x106c77c; continue; }
  pc = 0x106c762; continue;
 }
 0x106c762 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11296abu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106c771); f_01050a52(cpu); if cpu.returned_to != 0x106c771 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296abu32, v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c77c => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106c77f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106c77f); }
 let mut pc: u32 = 0x106c77f;
 loop { match pc {
 0x106c77f => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x106c7b5); f_01018ffd(cpu); if cpu.returned_to != 0x106c7b5 { return; }
  if cpu.cf { pc = 0x106c849; continue; }
  pc = 0x106c7bb; continue;
 }
 0x106c7bb => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x38u32); cpu.ebx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x106c7dc); f_01018ffd(cpu); if cpu.returned_to != 0x106c7dc { return; }
  if cpu.cf { pc = 0x106c895; continue; }
  pc = 0x106c7e2; continue;
 }
 0x106c7e2 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x106c846; continue; }
  pc = 0x106c7eb; continue;
 }
 0x106c7eb => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c846; continue; }
  pc = 0x106c7f5; continue;
 }
 0x106c7f5 => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x38u32); cpu.ebx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x106c816); f_01018ffd(cpu); if cpu.returned_to != 0x106c816 { return; }
  if cpu.cf { pc = 0x106c8ba; continue; }
  pc = 0x106c81c; continue;
 }
 0x106c81c => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106c846; continue; }
  pc = 0x106c826; continue;
 }
 0x106c826 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x106c844); f_01018ffd(cpu); if cpu.returned_to != 0x106c844 { return; }
  if cpu.cf { pc = 0x106c86f; continue; }
  pc = 0x106c846; continue;
 }
 0x106c846 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c849 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x91u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c86f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x91u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c895 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106c8ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d253(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d253); }
 let mut pc: u32 = 0x106d253;
 loop { match pc {
 0x106d253 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10e3e95u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x10e3e95u32), 0x8u8); }
  if !cpu.zf { pc = 0x106d271; continue; }
  pc = 0x106d26b; continue;
 }
 0x106d26b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d271 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d277(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d277); }
 let mut pc: u32 = 0x106d277;
 loop { match pc {
 0x106d277 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x106d2d2; continue; }
  pc = 0x106d280; continue;
 }
 0x106d280 => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x106d28d; continue; }
  pc = 0x106d286; continue;
 }
 0x106d286 => {
  cpu.push(0x106d28b); f_0106d3b5(cpu); if cpu.returned_to != 0x106d28b { return; }
  pc = 0x106d2bc; continue;
 }
 0x106d28d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x112b9d8u32.wrapping_add(cpu.esi)); cpu.push(0x106d2a0); cpu.call(t); if cpu.returned_to != 0x106d2a0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x106d2b3); f_0106d253(cpu); if cpu.returned_to != 0x106d2b3 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  let t = cpu.r32(0x112b9dcu32.wrapping_add(cpu.esi)); cpu.push(0x106d2bc); cpu.call(t); if cpu.returned_to != 0x106d2bc { return; }
  pc = 0x106d2bc; continue;
 }
 0x106d2bc => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x106d2ce); f_0106d253(cpu); if cpu.returned_to != 0x106d2ce { return; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d2d2 => {
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff7u32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0xfffffff7u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d2d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d2d7); }
 let mut pc: u32 = 0x106d2d7;
 loop { match pc {
 0x106d2d7 => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x106d2fe; continue; }
  pc = 0x106d2ee; continue;
 }
 0x106d2ee => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5c0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106d353; continue;
 }
 0x106d2fe => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5b8u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3e2du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e2fu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106d327; continue; }
  pc = 0x106d317; continue;
 }
 0x106d317 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106d339; continue; }
  pc = 0x106d31d; continue;
 }
 0x106d31d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x6u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106d349; continue;
 }
 0x106d327 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bc90u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x112bc92u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106d349; continue;
 }
 0x106d339 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112bcf0u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x112bcf2u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106d349; continue;
 }
 0x106d349 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x106d3ad; continue;
 }
 0x106d353 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106d361; continue; }
  pc = 0x106d359; continue;
 }
 0x106d359 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106d373; continue; }
  pc = 0x106d35f; continue;
 }
 0x106d35f => {
  pc = 0x106d385; continue;
 }
 0x106d361 => {
  { let v: u16 = cpu.r16(0x112bcb0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112bcb2u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106d395; continue;
 }
 0x106d373 => {
  { let v: u16 = cpu.r16(0x112bcd0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112bcd2u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106d395; continue;
 }
 0x106d385 => {
  { let v: u16 = cpu.r16(0x112bc70u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112bc72u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106d395; continue;
 }
 0x106d395 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e2du32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x10e3e2fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x106d3ad; continue;
 }
 0x106d3ad => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d3b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d3b5); }
 let mut pc: u32 = 0x106d3b5;
 loop { match pc {
 0x106d3b5 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106d3e2; continue; }
  pc = 0x106d3cc; continue;
 }
 0x106d3cc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106d3d8; continue; }
  pc = 0x106d3d2; continue;
 }
 0x106d3d2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x106d3ec; continue; }
  pc = 0x106d3d8; continue;
 }
 0x106d3d8 => {
  { let v: u16 = cpu.r16(0x112bc40u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106d3f4; continue;
 }
 0x106d3e2 => {
  { let v: u16 = cpu.r16(0x112bc30u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106d3f4; continue;
 }
 0x106d3ec => {
  { let v: u16 = cpu.r16(0x112bc20u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106d3f4; continue;
 }
 0x106d3f4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e2du32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e2fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x215u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d425(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d425); }
 let mut pc: u32 = 0x106d425;
 loop { match pc {
 0x106d425 => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d42a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d42a); }
 let mut pc: u32 = 0x106d42a;
 loop { match pc {
 0x106d42a => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d56c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d56c); }
 let mut pc: u32 = 0x106d56c;
 loop { match pc {
 0x106d56c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x106d574); f_0106d85c(cpu); if cpu.returned_to != 0x106d574 { return; }
  if cpu.cf { pc = 0x106d5fc; continue; }
  pc = 0x106d57a; continue;
 }
 0x106d57a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.r32(0x10e4134u32).wrapping_add(0x4u32); cpu.w32(0x10e4134u32, v); }
  cpu.push(0x106d5a1); f_0105f178(cpu); if cpu.returned_to != 0x106d5a1 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d5c4; continue; }
  pc = 0x106d5a7; continue;
 }
 0x106d5a7 => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d5c4); f_0106d7f0(cpu); if cpu.returned_to != 0x106d5c4 { return; }
  pc = 0x106d5c4; continue;
 }
 0x106d5c4 => {
  cpu.push(0x106d5c9); f_0106d85c(cpu); if cpu.returned_to != 0x106d5c9 { return; }
  if cpu.cf { pc = 0x106d5fc; continue; }
  pc = 0x106d5cb; continue;
 }
 0x106d5cb => {
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d5d7); f_0105f178(cpu); if cpu.returned_to != 0x106d5d7 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d5fc; continue; }
  pc = 0x106d5dd; continue;
 }
 0x106d5dd => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d5fc); f_0106d7f0(cpu); if cpu.returned_to != 0x106d5fc { return; }
  pc = 0x106d5fc; continue;
 }
 0x106d5fc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d600); }
 let mut pc: u32 = 0x106d600;
 loop { match pc {
 0x106d600 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x106d608); f_0106d85c(cpu); if cpu.returned_to != 0x106d608 { return; }
  if cpu.cf { pc = 0x106d692; continue; }
  pc = 0x106d60e; continue;
 }
 0x106d60e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.r32(0x10e4134u32).wrapping_add(0x4u32); cpu.w32(0x10e4134u32, v); }
  cpu.push(0x106d637); f_0105f178(cpu); if cpu.returned_to != 0x106d637 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d65a; continue; }
  pc = 0x106d63d; continue;
 }
 0x106d63d => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d65a); f_0106d7f0(cpu); if cpu.returned_to != 0x106d65a { return; }
  pc = 0x106d65a; continue;
 }
 0x106d65a => {
  cpu.push(0x106d65f); f_0106d85c(cpu); if cpu.returned_to != 0x106d65f { return; }
  if cpu.cf { pc = 0x106d692; continue; }
  pc = 0x106d661; continue;
 }
 0x106d661 => {
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d66d); f_0105f178(cpu); if cpu.returned_to != 0x106d66d { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d692; continue; }
  pc = 0x106d673; continue;
 }
 0x106d673 => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d692); f_0106d7f0(cpu); if cpu.returned_to != 0x106d692 { return; }
  pc = 0x106d692; continue;
 }
 0x106d692 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d696(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d696); }
 let mut pc: u32 = 0x106d696;
 loop { match pc {
 0x106d696 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x106d69e); f_0106d85c(cpu); if cpu.returned_to != 0x106d69e { return; }
  if cpu.cf { pc = 0x106d6ea; continue; }
  pc = 0x106d6a0; continue;
 }
 0x106d6a0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  cpu.push(0x106d6b4); f_0105f178(cpu); if cpu.returned_to != 0x106d6b4 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x10e3e95u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7cu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106d6ea; continue;
 }
 0x106d6ea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d6ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d6ee); }
 let mut pc: u32 = 0x106d6ee;
 loop { match pc {
 0x106d6ee => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x106d6f6); f_0106d85c(cpu); if cpu.returned_to != 0x106d6f6 { return; }
  if cpu.cf { pc = 0x106d793; continue; }
  pc = 0x106d6fc; continue;
 }
 0x106d6fc => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.r32(0x10e4134u32).wrapping_add(0x4u32); cpu.w32(0x10e4134u32, v); }
  cpu.push(0x106d724); f_0105f178(cpu); if cpu.returned_to != 0x106d724 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d751; continue; }
  pc = 0x106d72a; continue;
 }
 0x106d72a => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d743); f_0105f178(cpu); if cpu.returned_to != 0x106d743 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d74d); f_0106d7f0(cpu); if cpu.returned_to != 0x106d74d { return; }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106d751; continue;
 }
 0x106d751 => {
  cpu.push(0x106d756); f_0105f178(cpu); if cpu.returned_to != 0x106d756 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106d793; continue; }
  pc = 0x106d75c; continue;
 }
 0x106d75c => {
  cpu.push(0x106d761); f_0106d85c(cpu); if cpu.returned_to != 0x106d761 { return; }
  if cpu.cf { pc = 0x106d793; continue; }
  pc = 0x106d763; continue;
 }
 0x106d763 => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d785); f_0105f178(cpu); if cpu.returned_to != 0x106d785 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x106d78f); f_0106d7f0(cpu); if cpu.returned_to != 0x106d78f { return; }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106d793; continue;
 }
 0x106d793 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
