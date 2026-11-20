#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00401000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401000); }
 let mut pc: u32 = 0x401000;
 loop { match pc {
 0x401000 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40100a); f_00401080(cpu); if cpu.returned_to != 0x40100a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401010); }
 let mut pc: u32 = 0x401010;
 loop { match pc {
 0x401010 => {
  cpu.push(0x401015); f_0040d280(cpu); if cpu.returned_to != 0x401015 { return; }
  { let v: u32 = cpu.r32(0x5f1f44u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x401020); f_0040d180(cpu); if cpu.returned_to != 0x401020 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x5df900u32; cpu.push(v); }
  cpu.push(0x40102f); f_00403db0(cpu); if cpu.returned_to != 0x40102f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401040); }
 let mut pc: u32 = 0x401040;
 loop { match pc {
 0x401040 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5f1f44u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401050); }
 let mut pc: u32 = 0x401050;
 loop { match pc {
 0x401050 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18b4u32, v); }
  if cpu.zf { pc = 0x401070; continue; }
  pc = 0x40106b; continue;
 }
 0x40106b => {
  pc = 0x40a460; continue;
 }
 0x401070 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a460 => {
  { let v: u32 = cpu.r32(0x5e18b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de70cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5de708u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401080); }
 let mut pc: u32 = 0x401080;
 loop { match pc {
 0x401080 => {
  cpu.push(0x401085); f_00403ca0(cpu); if cpu.returned_to != 0x401085 { return; }
  f_00403d40(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401090); }
 let mut pc: u32 = 0x401090;
 loop { match pc {
 0x401090 => {
  { let v: u32 = cpu.r32(0x5df72cu32); cpu.eax = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4010c5; continue; }
  pc = 0x40109a; continue;
 }
 0x40109a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x78u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 9u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0xfffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4010b2); f_00403b10(cpu); if cpu.returned_to != 0x4010b2 { return; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x100u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5df72cu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4010c5; continue;
 }
 0x4010c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004010d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4010d0); }
 let mut pc: u32 = 0x4010d0;
 loop { match pc {
 0x4010d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4010da); f_00404710(cpu); if cpu.returned_to != 0x4010da { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004010e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4010e0); }
 let mut pc: u32 = 0x4010e0;
 loop { match pc {
 0x4010e0 => {
  pc = 0x404770; continue;
 }
 0x404770 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404797; continue; }
  pc = 0x404779; continue;
 }
 0x404779 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5ea6c0u32; cpu.esi = v; }
  pc = 0x40477f; continue;
 }
 0x40477f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40478b; continue; }
  pc = 0x404785; continue;
 }
 0x404785 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.push(0x40478b); cpu.call(t); if cpu.returned_to != 0x40478b { return; }
  pc = 0x40478b; continue;
 }
 0x40478b => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eaac0u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40477f; continue; }
  pc = 0x404796; continue;
 }
 0x404796 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x404797; continue;
 }
 0x404797 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004010f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4010f0); }
 let mut pc: u32 = 0x4010f0;
 loop { match pc {
 0x4010f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4010fa); f_00404680(cpu); if cpu.returned_to != 0x4010fa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401100); }
 let mut pc: u32 = 0x401100;
 loop { match pc {
 0x401100 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40110a); f_00404810(cpu); if cpu.returned_to != 0x40110a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401110); }
 let mut pc: u32 = 0x401110;
 loop { match pc {
 0x401110 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x40111e); f_004048b0(cpu); if cpu.returned_to != 0x40111e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401130); }
 let mut pc: u32 = 0x401130;
 loop { match pc {
 0x401130 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40113a); f_00404580(cpu); if cpu.returned_to != 0x40113a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401140); }
 let mut pc: u32 = 0x401140;
 loop { match pc {
 0x401140 => {
  f_004047d0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401150); }
 let mut pc: u32 = 0x401150;
 loop { match pc {
 0x401150 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x401176; continue; }
  pc = 0x401158; continue;
 }
 0x401158 => {
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x401176; continue; }
  pc = 0x40115d; continue;
 }
 0x40115d => {
  { let v: u32 = cpu.r32(0x5df880u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5df880u32, v); }
  pc = 0x4046a0; continue;
 }
 0x401176 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4046a0 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404700; continue; }
  pc = 0x4046a9; continue;
 }
 0x4046a9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x4046ae; continue;
 }
 0x4046ae => {
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ea2c0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4046f2; continue; }
  pc = 0x4046be; continue;
 }
 0x4046be => {
  { let v: u32 = cpu.r32(0x5df880u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1du32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.edx)); cpu.push(0x4046f2); cpu.call(t); if cpu.returned_to != 0x4046f2 { return; }
  pc = 0x4046f2; continue;
 }
 0x4046f2 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x400u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4046ae; continue; }
  pc = 0x4046fd; continue;
 }
 0x4046fd => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404700; continue;
 }
 0x404700 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401180); }
 let mut pc: u32 = 0x401180;
 loop { match pc {
 0x401180 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x401191); f_004048b0(cpu); if cpu.returned_to != 0x401191 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004011a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4011a0); }
 let mut pc: u32 = 0x4011a0;
 loop { match pc {
 0x4011a0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4011a6); f_00401be0(cpu); if cpu.returned_to != 0x4011a6 { return; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ecefcu32, v); }
  pc = 0x4011b5; continue;
 }
 0x4011b5 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4011c2); f_00401bc0(cpu); if cpu.returned_to != 0x4011c2 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x4011f5; continue; }
  pc = 0x4011cf; continue;
 }
 0x4011cf => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4011e1); f_00401bc0(cpu); if cpu.returned_to != 0x4011e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4011ea); f_00401640(cpu); if cpu.returned_to != 0x4011ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4012bb; continue; }
  pc = 0x4011f5; continue;
 }
 0x4011f5 => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef8u32, v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4011b5; continue; }
  pc = 0x401207; continue;
 }
 0x401207 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef8u32, v); }
  { cpu.sub32(cpu.ecx, 0x80u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecefcu32, v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4011b5; continue; }
  pc = 0x401223; continue;
 }
 0x401223 => {
  { let v = 0x17u32; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ecef8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ecefcu32, v); }
  cpu.push(0x401238); f_004017b0(cpu); if cpu.returned_to != 0x401238 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x401269; continue; }
  pc = 0x40123f; continue;
 }
 0x40123f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x401245); f_004013e0(cpu); if cpu.returned_to != 0x401245 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4012bb; continue; }
  pc = 0x40124c; continue;
 }
 0x40124c => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.edx = v; }
  { let v = 0x17u32; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = 0x17u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecef8u32, v); }
  cpu.push(0x401262); f_004017b0(cpu); if cpu.returned_to != 0x401262 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40123f; continue; }
  pc = 0x401269; continue;
 }
 0x401269 => {
  { let v = 0x33u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ecef8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ecefcu32, v); }
  cpu.push(0x40127e); f_004017b0(cpu); if cpu.returned_to != 0x40127e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4012bf; continue; }
  pc = 0x401285; continue;
 }
 0x401285 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40128b); f_004012d0(cpu); if cpu.returned_to != 0x40128b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4012bb; continue; }
  pc = 0x401292; continue;
 }
 0x401292 => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.edx = v; }
  { let v = 0x33u32; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecef8u32, v); }
  cpu.push(0x4012a8); f_004017b0(cpu); if cpu.returned_to != 0x4012a8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x401285; continue; }
  pc = 0x4012af; continue;
 }
 0x4012af => {
  cpu.push(0x4012b4); f_00401950(cpu); if cpu.returned_to != 0x4012b4 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4012bb => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4012bf => {
  cpu.push(0x4012c4); f_00401950(cpu); if cpu.returned_to != 0x4012c4 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004012d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4012d0); } r_004012d0(cpu, 0x4012d0); }
pub fn f_004012f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4012f4); } r_004012d0(cpu, 0x4012f4); }
fn r_004012d0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4012d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v = 0x33u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4012f3); f_00401510(cpu); if cpu.returned_to != 0x4012f3 { return; }
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef4u32, v); }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf04u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecef0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecf00u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x401328; continue;
 }
 0x4012f4 => {
  { let v: u32 = cpu.eax | 0x5ecef0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef4u32, v); }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf04u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecef0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecf00u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x401328; continue;
 }
 0x401328 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x401340; continue; }
  pc = 0x40132e; continue;
 }
 0x40132e => {
  { let v: u32 = cpu.esi.wrapping_add(0xffu32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xd7u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x401328; continue; }
  pc = 0x40133c; continue;
 }
 0x40133c => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401340 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x401372); f_00401380(cpu); if cpu.returned_to != 0x401372 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x40137a); f_004030c0(cpu); if cpu.returned_to != 0x40137a { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401380); }
 let mut pc: u32 = 0x401380;
 loop { match pc {
 0x401380 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4d954cu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4013c6); f_00401bc0(cpu); if cpu.returned_to != 0x4013c6 { return; }
  { let v: u32 = 0xffffc000u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004013e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4013e0); }
 let mut pc: u32 = 0x4013e0;
 loop { match pc {
 0x4013e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x401404); f_00401510(cpu); if cpu.returned_to != 0x401404 { return; }
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf00u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5ecf04u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecef0u32, v); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecef4u32, v); }
  if !cpu.zf { pc = 0x40147c; continue; }
  pc = 0x401441; continue;
 }
 0x401441 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x401449); f_00401bc0(cpu); if cpu.returned_to != 0x401449 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x40145c; continue; }
  pc = 0x401454; continue;
 }
 0x401454 => {
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.edx = v; }
  pc = 0x401481; continue;
 }
 0x40145c => {
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecef4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf04u32, v); }
  pc = 0x4014a7; continue;
 }
 0x40147c => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x401495; continue; }
  pc = 0x401481; continue;
 }
 0x401481 => {
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ecef0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf00u32, v); }
  pc = 0x4014a7; continue;
 }
 0x401495 => {
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecef4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecf04u32, v); }
  pc = 0x4014a7; continue;
 }
 0x4014a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4014a9; continue;
 }
 0x4014a9 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x4014c2; continue; }
  pc = 0x4014af; continue;
 }
 0x4014af => {
  { let v: u32 = cpu.esi.wrapping_add(0xffu32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xd7u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4014a9; continue; }
  pc = 0x4014bd; continue;
 }
 0x4014bd => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4014c2 => {
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x4014f4); f_00401380(cpu); if cpu.returned_to != 0x4014f4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x4014fc); f_004030c0(cpu); if cpu.returned_to != 0x4014fc { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401510); }
 let mut pc: u32 = 0x401510;
 loop { match pc {
 0x401510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ecef0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5ecef4u32, v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x40152e; continue;
 }
 0x40152e => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x401565; continue; }
  pc = 0x40153c; continue;
 }
 0x40153c => {
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x401550); f_00401850(cpu); if cpu.returned_to != 0x401550 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x401565; continue; }
  pc = 0x401557; continue;
 }
 0x401557 => {
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef0u32, v); }
  cpu.poll(); pc = 0x40152e; continue;
 }
 0x401565 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x40156b; continue;
 }
 0x40156b => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x40157b; continue; }
  pc = 0x401575; continue;
 }
 0x401575 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x4015a5; continue; }
  pc = 0x40157b; continue;
 }
 0x40157b => {
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef0u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40158e); f_00401850(cpu); if cpu.returned_to != 0x40158e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4015a5; continue; }
  pc = 0x401595; continue;
 }
 0x401595 => {
  { let v: u32 = cpu.r32(0x5ecef4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xffffff80u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef4u32, v); }
  cpu.poll(); pc = 0x40156b; continue;
 }
 0x4015a5 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5ecf00u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5ecf04u32, v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x4015b8; continue;
 }
 0x4015b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x4015c7; continue; }
  pc = 0x4015c1; continue;
 }
 0x4015c1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x4015f1; continue; }
  pc = 0x4015c7; continue;
 }
 0x4015c7 => {
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4015da); f_00401850(cpu); if cpu.returned_to != 0x4015da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4015f1; continue; }
  pc = 0x4015e1; continue;
 }
 0x4015e1 => {
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecf00u32, v); }
  cpu.poll(); pc = 0x4015b8; continue;
 }
 0x4015f1 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x4015f7; continue;
 }
 0x4015f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x401606; continue; }
  pc = 0x401600; continue;
 }
 0x401600 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x401636; continue; }
  pc = 0x401606; continue;
 }
 0x401606 => {
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecf00u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40161a); f_00401850(cpu); if cpu.returned_to != 0x40161a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x401636; continue; }
  pc = 0x401621; continue;
 }
 0x401621 => {
  { let v: u32 = cpu.r32(0x5ecf04u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecf04u32, v); }
  cpu.poll(); pc = 0x4015f7; continue;
 }
 0x401636 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401640(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x401640); } r_00401640(cpu, 0x401640); }
pub fn f_00401698(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x401698); } r_00401640(cpu, 0x401698); }
fn r_00401640(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x401640 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x103u32); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(0xfffffffeu32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x401663); f_004018e0(cpu); if cpu.returned_to != 0x401663 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4016b8; continue; }
  pc = 0x40166a; continue;
 }
 0x40166a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x4a4270u32; cpu.esi = v; }
  pc = 0x401671; continue;
 }
 0x401671 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40167a); f_00401740(cpu); if cpu.returned_to != 0x40167a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x401693; continue; }
  pc = 0x401681; continue;
 }
 0x401681 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0x4a4294u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x401671; continue; }
  pc = 0x40168d; continue;
 }
 0x40168d => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401693 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.eax = v; }
  pc = 0x401698; continue;
 }
 0x401698 => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffffffeu32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(0xfffffffdu32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4016ac); f_004016c0(cpu); if cpu.returned_to != 0x4016ac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4016b8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004016c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4016c0); }
 let mut pc: u32 = 0x4016c0;
 loop { match pc {
 0x4016c0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a4298u32.wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a429cu32.wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x4016e7; continue;
 }
 0x4016e7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x401700; continue; }
  pc = 0x4016ed; continue;
 }
 0x4016ed => {
  { let v: u32 = cpu.eax.wrapping_add(0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xd7u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4016e7; continue; }
  pc = 0x4016fb; continue;
 }
 0x4016fb => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401700 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x5u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40172b); f_00401380(cpu); if cpu.returned_to != 0x40172b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x401733); f_004030c0(cpu); if cpu.returned_to != 0x401733 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401740); }
 let mut pc: u32 = 0x401740;
 loop { match pc {
 0x401740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x40174e; continue;
 }
 0x40174e => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x401750; continue;
 }
 0x401750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x401757); f_00401780(cpu); if cpu.returned_to != 0x401757 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x401776; continue; }
  pc = 0x40175e; continue;
 }
 0x40175e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x8u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x401750; continue; }
  pc = 0x401766; continue;
 }
 0x401766 => {
  { let v: u32 = cpu.edi.wrapping_add(0x78u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x8u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40174e; continue; }
  pc = 0x40176f; continue;
 }
 0x40176f => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401776 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401780); }
 let mut pc: u32 = 0x401780;
 loop { match pc {
 0x401780 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x40178d; continue; }
  pc = 0x40178a; continue;
 }
 0x40178a => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40178d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x4017a1; continue; }
  pc = 0x40179b; continue;
 }
 0x40179b => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4017a1 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004017b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4017b0); }
 let mut pc: u32 = 0x4017b0;
 loop { match pc {
 0x4017b0 => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x4017bf; continue;
 }
 0x4017bf => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4017cc); f_00401bc0(cpu); if cpu.returned_to != 0x4017cc { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4017de; continue; }
  pc = 0x4017da; continue;
 }
 0x4017da => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { pc = 0x4017f7; continue; }
  pc = 0x4017de; continue;
 }
 0x4017de => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4017f0); f_00401850(cpu); if cpu.returned_to != 0x4017f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x401827; continue; }
  pc = 0x4017f7; continue;
 }
 0x4017f7 => {
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef8u32, v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4017bf; continue; }
  pc = 0x401809; continue;
 }
 0x401809 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ecef8u32, v); }
  { cpu.sub32(cpu.ecx, 0x80u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ecefcu32, v); }
  if (cpu.sf == cpu.of) { pc = 0x401840; continue; }
  pc = 0x401825; continue;
 }
 0x401825 => {
  cpu.poll(); pc = 0x4017bf; continue;
 }
 0x401827 => {
  { let v: u32 = cpu.r32(0x5ecefcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ecef8u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40183a); f_00401bc0(cpu); if cpu.returned_to != 0x40183a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401840 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401850); }
 let mut pc: u32 = 0x401850;
 loop { match pc {
 0x401850 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xd629u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.cf { pc = 0x4018cf; continue; }
  pc = 0x401863; continue;
 }
 0x401863 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x40186b; continue;
 }
 0x40186b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x4018b6; continue; }
  pc = 0x401873; continue;
 }
 0x401873 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x4018b6; continue; }
  pc = 0x401883; continue;
 }
 0x401883 => {
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4018b6; continue; }
  pc = 0x401894; continue;
 }
 0x401894 => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x4018b6; continue; }
  pc = 0x4018a5; continue;
 }
 0x4018a5 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4018c6; continue; }
  pc = 0x4018b6; continue;
 }
 0x4018b6 => {
  { let v: u32 = cpu.ecx.wrapping_add(0xffu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x40186b; continue; }
  pc = 0x4018c0; continue;
 }
 0x4018c0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4018c6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4018cf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004018e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4018e0); }
 let mut pc: u32 = 0x4018e0;
 loop { match pc {
 0x4018e0 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xd629u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.cf { pc = 0x40193d; continue; }
  pc = 0x4018f3; continue;
 }
 0x4018f3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x4018fb; continue;
 }
 0x4018fb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x401924; continue; }
  pc = 0x401903; continue;
 }
 0x401903 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x401924; continue; }
  pc = 0x401913; continue;
 }
 0x401913 => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.zf { pc = 0x401934; continue; }
  pc = 0x401924; continue;
 }
 0x401924 => {
  { let v: u32 = cpu.ecx.wrapping_add(0xffu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if cpu.cf { cpu.poll(); pc = 0x4018fb; continue; }
  pc = 0x40192e; continue;
 }
 0x40192e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401934 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40193d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401950(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x401950); } r_00401950(cpu, 0x401950); }
pub fn f_004019be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4019be); } r_00401950(cpu, 0x4019be); }
fn r_00401950(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x401950 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xd629u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if !cpu.cf { pc = 0x401bb5; continue; }
  pc = 0x401983; continue;
 }
 0x401983 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x401b89; continue; }
  pc = 0x40198d; continue;
 }
 0x40198d => {
  { let v: u32 = 0xffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  if !cpu.cf { pc = 0x401b89; continue; }
  pc = 0x40199f; continue;
 }
 0x40199f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x401b6b; continue; }
  pc = 0x4019ab; continue;
 }
 0x4019ab => {
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x4019c1; continue; }
  pc = 0x4019b9; continue;
 }
 0x4019b9 => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a07; continue;
 }
 0x4019be => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xebu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x4019c1; continue;
 }
 0x4019c1 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x4019d7; continue; }
  pc = 0x4019cf; continue;
 }
 0x4019cf => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a07; continue;
 }
 0x4019d7 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x4019f0; continue; }
  pc = 0x4019e6; continue;
 }
 0x4019e6 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a07; continue;
 }
 0x4019f0 => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x401a07; continue; }
  pc = 0x4019ff; continue;
 }
 0x4019ff => {
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a07; continue;
 }
 0x401a07 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x401a41; continue; }
  pc = 0x401a0d; continue;
 }
 0x401a0d => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x401a39; continue; }
  pc = 0x401a17; continue;
 }
 0x401a17 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x401a39; continue; }
  pc = 0x401a21; continue;
 }
 0x401a21 => {
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a72; continue;
 }
 0x401a39 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a72; continue;
 }
 0x401a41 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ebx); }
  if cpu.zf { pc = 0x401a72; continue; }
  pc = 0x401a47; continue;
 }
 0x401a47 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x401a6a; continue; }
  pc = 0x401a4f; continue;
 }
 0x401a4f => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x401a6a; continue; }
  pc = 0x401a59; continue;
 }
 0x401a59 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  pc = 0x401a72; continue;
 }
 0x401a6a => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x401a72; continue;
 }
 0x401a72 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), cpu.ebx); }
  if cpu.zf { pc = 0x401b6b; continue; }
  pc = 0x401a7c; continue;
 }
 0x401a7c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x401aa5); f_00401bc0(cpu); if cpu.returned_to != 0x401aa5 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x401ad0); f_00401bc0(cpu); if cpu.returned_to != 0x401ad0 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x401ae0; continue; }
  pc = 0x401adc; continue;
 }
 0x401adc => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x401af2; continue; }
  pc = 0x401ae0; continue;
 }
 0x401ae0 => {
  { cpu.sub8((cpu.ebx as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x401ae9; continue; }
  pc = 0x401ae5; continue;
 }
 0x401ae5 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x401af2; continue; }
  pc = 0x401ae9; continue;
 }
 0x401ae9 => {
  { cpu.sub8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x401b4f; continue; }
  pc = 0x401aee; continue;
 }
 0x401aee => {
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x401b4f; continue; }
  pc = 0x401af2; continue;
 }
 0x401af2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x401af9); f_004018e0(cpu); if cpu.returned_to != 0x401af9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x401b4f; continue; }
  pc = 0x401b00; continue;
 }
 0x401b00 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x401b09; continue;
 }
 0x401b09 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x401b1e; continue; }
  pc = 0x401b0e; continue;
 }
 0x401b0e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xffu32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0xd7u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x401b09; continue; }
  pc = 0x401b1c; continue;
 }
 0x401b1c => {
  pc = 0x401b45; continue;
 }
 0x401b1e => {
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x401b42); f_00401380(cpu); if cpu.returned_to != 0x401b42 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x401b45; continue;
 }
 0x401b45 => {
  { cpu.sub32(cpu.esi, 0xd7u32); }
  if cpu.zf { pc = 0x401bab; continue; }
  pc = 0x401b4d; continue;
 }
 0x401b4d => {
  pc = 0x401b51; continue;
 }
 0x401b4f => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x401b51; continue;
 }
 0x401b51 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x401b6b; continue;
 }
 0x401b6b => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xffu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xd629u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x40199f; continue; }
  pc = 0x401b89; continue;
 }
 0x401b89 => {
  { let v: u32 = cpu.esi.wrapping_add(0xffu32); cpu.esi = v; }
  { let v: u32 = 0xd629u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if cpu.cf { cpu.poll(); pc = 0x401983; continue; }
  pc = 0x401ba1; continue;
 }
 0x401ba1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401bab => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401bb5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401bc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401bc0); }
 let mut pc: u32 = 0x401bc0;
 loop { match pc {
 0x401bc0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x80u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = 0x4d954cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401be0); }
 let mut pc: u32 = 0x401be0;
 loop { match pc {
 0x401be0 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xd7u32; cpu.esi = v; }
  pc = 0x401bed; continue;
 }
 0x401bed => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x3fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xffu32); cpu.edx = v; }
  { cpu.stos(4, true); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.stos(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x401bed; continue; }
  pc = 0x401c04; continue;
 }
 0x401c04 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401c10); }
 let mut pc: u32 = 0x401c10;
 loop { match pc {
 0x401c10 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x218u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4a4300u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a1080u32); cpu.push(0x401c2c); cpu.call(t); if cpu.returned_to != 0x401c2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x401d3c; continue; }
  pc = 0x401c34; continue;
 }
 0x401c34 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a1070u32); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a106cu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x401c50; continue;
 }
 0x401c4c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x401c50; continue;
 }
 0x401c50 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x401d2b; continue; }
  pc = 0x401c58; continue;
 }
 0x401c58 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x41u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x401c68); cpu.call(t); if cpu.returned_to != 0x401c68 { return; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if !cpu.zf { pc = 0x401d2b; continue; }
  pc = 0x401c71; continue;
 }
 0x401c71 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a42f4u32; cpu.push(v); }
  cpu.push(0x401c80); f_004030c0(cpu); if cpu.returned_to != 0x401c80 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x13cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x401cb0); cpu.call(t); if cpu.returned_to != 0x401cb0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x401d2b; continue; }
  pc = 0x401cb4; continue;
 }
 0x401cb4 => {
  { let v: u32 = 0x4a42ecu32; cpu.esi = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x401cbd; continue;
 }
 0x401cbd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x401ce1; continue; }
  pc = 0x401cc5; continue;
 }
 0x401cc5 => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x401cdd; continue; }
  pc = 0x401cc9; continue;
 }
 0x401cc9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x401ce1; continue; }
  pc = 0x401cd3; continue;
 }
 0x401cd3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x401cbd; continue; }
  pc = 0x401cdd; continue;
 }
 0x401cdd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x401ce6; continue;
 }
 0x401ce1 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x401ce6; continue;
 }
 0x401ce6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x401d2b; continue; }
  pc = 0x401cea; continue;
 }
 0x401cea => {
  { let v: u32 = 0x4a42e0u32; cpu.esi = v; }
  { let v: u32 = 0x128u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x401cf6; continue;
 }
 0x401cf6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x401d1a; continue; }
  pc = 0x401cfe; continue;
 }
 0x401cfe => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x401d16; continue; }
  pc = 0x401d02; continue;
 }
 0x401d02 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x401d1a; continue; }
  pc = 0x401d0c; continue;
 }
 0x401d0c => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x401cf6; continue; }
  pc = 0x401d16; continue;
 }
 0x401d16 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x401d1f; continue;
 }
 0x401d1a => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x401d1f; continue;
 }
 0x401d1f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x401d2b; continue; }
  pc = 0x401d23; continue;
 }
 0x401d23 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x401d2b; continue;
 }
 0x401d2b => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x401c4c; continue; }
  pc = 0x401d35; continue;
 }
 0x401d35 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x401d3c; continue;
 }
 0x401d3c => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x58c6f4u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x218u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401d50); }
 let mut pc: u32 = 0x401d50;
 loop { match pc {
 0x401d50 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x170u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  { let v: u32 = cpu.edi; cpu.w32(0x5eca38u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5eca28u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5eca34u32, v); }
  if !cpu.zf { pc = 0x401d8e; continue; }
  pc = 0x401d76; continue;
 }
 0x401d76 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x401d82); f_00413302(cpu); if cpu.returned_to != 0x401d82 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.edx = v; }
  pc = 0x401d96; continue;
 }
 0x401d8e => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x401d96; continue;
 }
 0x401d96 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x401db3; continue; }
  pc = 0x401d9a; continue;
 }
 0x401d9a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a4468u32; cpu.push(v); }
  cpu.push(0x401da5); f_004030c0(cpu); if cpu.returned_to != 0x401da5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x170u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401db3 => {
  { cpu.sub32(cpu.edx, cpu.edi); }
  if !cpu.zf { pc = 0x401dfc; continue; }
  pc = 0x401db7; continue;
 }
 0x401db7 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5eca3cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = 0x11u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x50u32.wrapping_add(cpu.edx)); cpu.push(0x401dcb); cpu.call(t); if cpu.returned_to != 0x401dcb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x402450u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.push(0x401ddf); cpu.call(t); if cpu.returned_to != 0x401ddf { return; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x401dfc; continue; }
  pc = 0x401de3; continue;
 }
 0x401de3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4a4444u32; cpu.push(v); }
  cpu.push(0x401dee); f_004030c0(cpu); if cpu.returned_to != 0x401dee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x170u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401dfc => {
  { let v: u32 = cpu.r32(0x5deef0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5df734u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df730u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4a442cu32; cpu.push(v); }
  cpu.push(0x401e1a); f_004030c0(cpu); if cpu.returned_to != 0x401e1a { return; }
  { let v: u32 = cpu.r32(0x5deef0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df734u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df730u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x54u32.wrapping_add(cpu.edx)); cpu.push(0x401e3c); cpu.call(t); if cpu.returned_to != 0x401e3c { return; }
  { let v = 0x4a4410u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x401e48); f_004030c0(cpu); if cpu.returned_to != 0x401e48 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x401e68; continue; }
  pc = 0x401e4f; continue;
 }
 0x401e4f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4a43f8u32; cpu.push(v); }
  cpu.push(0x401e5a); f_004030c0(cpu); if cpu.returned_to != 0x401e5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x170u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401e68 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x16cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.push(0x401e80); cpu.call(t); if cpu.returned_to != 0x401e80 { return; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x401e9d; continue; }
  pc = 0x401e84; continue;
 }
 0x401e84 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a43e0u32; cpu.push(v); }
  cpu.push(0x401e8f); f_004030c0(cpu); if cpu.returned_to != 0x401e8f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x170u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x401e9d => {
  { cpu.and32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x2000000u32); }
  if cpu.zf { pc = 0x401eb4; continue; }
  pc = 0x401ea7; continue;
 }
 0x401ea7 => {
  { let v = 0x4a43c4u32; cpu.push(v); }
  cpu.push(0x401eb1); f_004030c0(cpu); if cpu.returned_to != 0x401eb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x401eb4; continue;
 }
 0x401eb4 => {
  { cpu.and32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x4000000u32); }
  if cpu.zf { pc = 0x401ecf; continue; }
  pc = 0x401ebe; continue;
 }
 0x401ebe => {
  { let v: u32 = 0x1u32; cpu.w32(0x5eca2cu32, v); }
  { let v = 0x4a43a8u32; cpu.push(v); }
  pc = 0x401eda; continue;
 }
 0x401ecf => {
  { let v: u32 = cpu.edi; cpu.w32(0x5eca2cu32, v); }
  { let v = 0x4a4384u32; cpu.push(v); }
  pc = 0x401eda; continue;
 }
 0x401eda => {
  cpu.push(0x401edf); f_004030c0(cpu); if cpu.returned_to != 0x401edf { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, 0x400000u32); }
  { let v: u32 = 0x2u32; cpu.w32(0x5eca38u32, v); }
  if cpu.zf { pc = 0x401f5a; continue; }
  pc = 0x401ef7; continue;
 }
 0x401ef7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x200u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x401f11; continue; }
  pc = 0x401f04; continue;
 }
 0x401f04 => {
  { let v = 0x4a4364u32; cpu.push(v); }
  cpu.push(0x401f0e); f_004030c0(cpu); if cpu.returned_to != 0x401f0e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x401f11; continue;
 }
 0x401f11 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x401f67; continue; }
  pc = 0x401f18; continue;
 }
 0x401f18 => {
  { let v = 0x4a4340u32; cpu.push(v); }
  cpu.push(0x401f22); f_004030c0(cpu); if cpu.returned_to != 0x401f22 { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x401f67; continue; }
  pc = 0x401f2d; continue;
 }
 0x401f2d => {
  { let v: u32 = cpu.r32(0x5deef0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 21u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.cf { pc = 0x401f67; continue; }
  pc = 0x401f41; continue;
 }
 0x401f41 => {
  { let v = 0x4a4324u32; cpu.push(v); }
  cpu.push(0x401f4b); f_004030c0(cpu); if cpu.returned_to != 0x401f4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x5eca30u32, v); }
  pc = 0x401f67; continue;
 }
 0x401f5a => {
  { let v = 0x4a4304u32; cpu.push(v); }
  cpu.push(0x401f64); f_004030c0(cpu); if cpu.returned_to != 0x401f64 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x401f67; continue;
 }
 0x401f67 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5df388u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x170u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x401f80); }
 let mut pc: u32 = 0x401f80;
 loop { match pc {
 0x401f80 => {
  { let v: u32 = cpu.r32(0x5df398u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x401f98; continue; }
  pc = 0x401f8c; continue;
 }
 0x401f8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x401f92); cpu.call(t); if cpu.returned_to != 0x401f92 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5df398u32, v); }
  pc = 0x401f98; continue;
 }
 0x401f98 => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x401fad; continue; }
  pc = 0x401fa1; continue;
 }
 0x401fa1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x401fa7); cpu.call(t); if cpu.returned_to != 0x401fa7 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5df728u32, v); }
  pc = 0x401fad; continue;
 }
 0x401fad => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x401fc2; continue; }
  pc = 0x401fb6; continue;
 }
 0x401fb6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x401fbc); cpu.call(t); if cpu.returned_to != 0x401fbc { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5df700u32, v); }
  pc = 0x401fc2; continue;
 }
 0x401fc2 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), cpu.esi); }
  if cpu.zf { pc = 0x401fdd; continue; }
  pc = 0x401fc8; continue;
 }
 0x401fc8 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x401fdd; continue; }
  pc = 0x401fd1; continue;
 }
 0x401fd1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x401fd7); cpu.call(t); if cpu.returned_to != 0x401fd7 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5df388u32, v); }
  pc = 0x401fdd; continue;
 }
 0x401fdd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00401ff0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x401ff0); } r_00401ff0(cpu, 0x401ff0); }
pub fn f_00402000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x402000); } r_00401ff0(cpu, 0x402000); }
fn r_00401ff0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x401ff0 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x64u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x402004; continue; }
  pc = 0x401ffd; continue;
 }
 0x401ffd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x402000; continue;
 }
 0x402000 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402004 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11c4u32); cpu.push(0x402010); cpu.call(t); if cpu.returned_to != 0x402010 { return; }
  { let v: u32 = 0x64u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x402022; continue;
 }
 0x402022 => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1000400u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.push(0x40203d); cpu.call(t); if cpu.returned_to != 0x40203d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402064; continue; }
  pc = 0x402041; continue;
 }
 0x402041 => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.push(0x402050); cpu.call(t); if cpu.returned_to != 0x402050 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402079; continue; }
  pc = 0x402054; continue;
 }
 0x402054 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x402022; continue; }
  pc = 0x40205a; continue;
 }
 0x40205a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402064 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a44a4u32; cpu.push(v); }
  cpu.push(0x40206f); f_004030c0(cpu); if cpu.returned_to != 0x40206f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402079 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a4488u32; cpu.push(v); }
  cpu.push(0x402084); f_004030c0(cpu); if cpu.returned_to != 0x402084 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402090); }
 let mut pc: u32 = 0x402090;
 loop { match pc {
 0x402090 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x204u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x9cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x16cu32; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.push(0x4020b7); cpu.call(t); if cpu.returned_to != 0x4020b7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4020c5; continue; }
  pc = 0x4020bb; continue;
 }
 0x4020bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4020c5 => {
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x5eca38u32); cpu.eax = v; }
  { let v: u32 = 0x6cu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x402155; continue; }
  pc = 0x4020e4; continue;
 }
 0x4020e4 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x21u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x218u32; cpu.w32(0x7cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x5df700u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x402110); cpu.call(t); if cpu.returned_to != 0x402110 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4021c2; continue; }
  pc = 0x402118; continue;
 }
 0x402118 => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = 0x5df728u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.push(0x402135); cpu.call(t); if cpu.returned_to != 0x402135 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40229e; continue; }
  pc = 0x40213d; continue;
 }
 0x40213d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a458cu32; cpu.push(v); }
  cpu.push(0x402148); f_004030c0(cpu); if cpu.returned_to != 0x402148 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402155 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32; cpu.w32(0x78u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x4021a7; continue; }
  pc = 0x402167; continue;
 }
 0x402167 => {
  { let v = 0x4a4568u32; cpu.push(v); }
  let t = cpu.r32(0x4a107cu32); cpu.push(0x402172); cpu.call(t); if cpu.returned_to != 0x402172 { return; }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x5df700u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x402189); cpu.call(t); if cpu.returned_to != 0x402189 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4021c2; continue; }
  pc = 0x40218d; continue;
 }
 0x40218d => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(0x402198); cpu.call(t); if cpu.returned_to != 0x402198 { return; }
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5df728u32, v); }
  pc = 0x4022a3; continue;
 }
 0x4021a7 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x5df700u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x4021be); cpu.call(t); if cpu.returned_to != 0x4021be { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4021da; continue; }
  pc = 0x4021c2; continue;
 }
 0x4021c2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a454cu32; cpu.push(v); }
  cpu.push(0x4021cd); f_004030c0(cpu); if cpu.returned_to != 0x4021cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4021da => {
  { let v: u32 = cpu.r32(0x5df394u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5df390u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4021f0); f_00402310(cpu); if cpu.returned_to != 0x4021f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5df728u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402213; continue; }
  pc = 0x4021fc; continue;
 }
 0x4021fc => {
  { let v = 0x4a4530u32; cpu.push(v); }
  cpu.push(0x402206); f_004030c0(cpu); if cpu.returned_to != 0x402206 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402213 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5df398u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.push(0x402227); cpu.call(t); if cpu.returned_to != 0x402227 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402242; continue; }
  pc = 0x40222b; continue;
 }
 0x40222b => {
  { let v = 0x4a451cu32; cpu.push(v); }
  cpu.push(0x402235); f_004030c0(cpu); if cpu.returned_to != 0x402235 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402242 => {
  { let v: u32 = cpu.r32(0x5df398u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x402256); cpu.call(t); if cpu.returned_to != 0x402256 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402271; continue; }
  pc = 0x40225a; continue;
 }
 0x40225a => {
  { let v = 0x4a44fcu32; cpu.push(v); }
  cpu.push(0x402264); f_004030c0(cpu); if cpu.returned_to != 0x402264 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402271 => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df398u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.push(0x402283); cpu.call(t); if cpu.returned_to != 0x402283 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40229e; continue; }
  pc = 0x402287; continue;
 }
 0x402287 => {
  { let v = 0x4a44d8u32; cpu.push(v); }
  cpu.push(0x402291); f_004030c0(cpu); if cpu.returned_to != 0x402291 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40229e => {
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  pc = 0x4022a3; continue;
 }
 0x4022a3 => {
  { let v: u32 = 0x7cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.w32(0x7cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x54u32.wrapping_add(cpu.edx)); cpu.push(0x4022b6); cpu.call(t); if cpu.returned_to != 0x4022b6 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4a44c0u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5df59cu32, v); }
  cpu.push(0x4022cc); f_004030c0(cpu); if cpu.returned_to != 0x4022cc { return; }
  { let v: u32 = cpu.r32(0x5eca30u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4022f7; continue; }
  pc = 0x4022d8; continue;
 }
 0x4022d8 => {
  { let v: u32 = cpu.r32(0x5df59cu32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.push(0x4022f7); cpu.call(t); if cpu.returned_to != 0x4022f7 { return; }
  pc = 0x4022f7; continue;
 }
 0x4022f7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x204u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402310); }
 let mut pc: u32 = 0x402310;
 loop { match pc {
 0x402310 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x6cu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x7u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x40u32; cpu.w32(0x78u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x40234d; continue; }
  pc = 0x402344; continue;
 }
 0x402344 => {
  { let v: u32 = cpu.r32(0x5eca34u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402355; continue; }
  pc = 0x40234d; continue;
 }
 0x40234d => {
  { let v: u32 = 0x840u32; cpu.w32(0x78u32.wrapping_add(cpu.esp), v); }
  pc = 0x402355; continue;
 }
 0x402355 => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x402382); cpu.call(t); if cpu.returned_to != 0x402382 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4023c1; continue; }
  pc = 0x402386; continue;
 }
 0x402386 => {
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4023dc; continue; }
  pc = 0x402391; continue;
 }
 0x402391 => {
  { let v: u32 = cpu.r32(0x5eca30u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4023dc; continue; }
  pc = 0x40239a; continue;
 }
 0x40239a => {
  { let v: u32 = cpu.r32(0x5df59cu32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.push(0x4023b8); cpu.call(t); if cpu.returned_to != 0x4023b8 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4023c1 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4a45b0u32; cpu.push(v); }
  cpu.push(0x4023d1); f_004030c0(cpu); if cpu.returned_to != 0x4023d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x4023dc; continue;
 }
 0x4023dc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004023f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4023f0); }
 let mut pc: u32 = 0x4023f0;
 loop { match pc {
 0x4023f0 => {
  f_00401ff0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402400); }
 let mut pc: u32 = 0x402400;
 loop { match pc {
 0x402400 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  let t = cpu.r32(0x4a1084u32); cpu.push(0x40240d); cpu.call(t); if cpu.returned_to != 0x40240d { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1104u32); cpu.push(0x402414); cpu.call(t); if cpu.returned_to != 0x402414 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402420); }
 let mut pc: u32 = 0x402420;
 loop { match pc {
 0x402420 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x402445; continue; }
  pc = 0x402429; continue;
 }
 0x402429 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a1090u32); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x402433); cpu.call(t); if cpu.returned_to != 0x402433 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a108cu32); cpu.push(0x40243a); cpu.call(t); if cpu.returned_to != 0x40243a { return; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x40243d); cpu.call(t); if cpu.returned_to != 0x40243d { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1088u32); cpu.push(0x402444); cpu.call(t); if cpu.returned_to != 0x402444 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x402445; continue;
 }
 0x402445 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402450); }
 let mut pc: u32 = 0x402450;
 loop { match pc {
 0x402450 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4a45d0u32; cpu.push(v); }
  cpu.push(0x40246c); f_004030c0(cpu); if cpu.returned_to != 0x40246c { return; }
  { let v: u32 = cpu.r32(0x5eca3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4024a6; continue; }
  pc = 0x402479; continue;
 }
 0x402479 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eca34u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if !cpu.zf { pc = 0x4024a6; continue; }
  pc = 0x40248c; continue;
 }
 0x40248c => {
  { let v: u32 = cpu.r32(0x5eca38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if !cpu.zf { pc = 0x4024a6; continue; }
  pc = 0x402499; continue;
 }
 0x402499 => {
  { let v: u32 = cpu.r32(0x5eca3cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x4024ce; continue; }
  pc = 0x4024a6; continue;
 }
 0x4024a6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eca40u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eca44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eca48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eca3cu32, v); }
  pc = 0x4024ce; continue;
 }
 0x4024ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004024e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4024e0); }
 let mut pc: u32 = 0x4024e0;
 loop { match pc {
 0x4024e0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 16u32); cpu.edi = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x402506; continue; }
  pc = 0x4024ff; continue;
 }
 0x4024ff => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x402514; continue;
 }
 0x402506 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10f0u32); cpu.push(0x402514); cpu.call(t); if cpu.returned_to != 0x402514 { return; }
  pc = 0x402514; continue;
 }
 0x402514 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x402528; continue; }
  pc = 0x40251b; continue;
 }
 0x40251b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402528 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40252e); f_004025b0(cpu); if cpu.returned_to != 0x40252e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x402590; continue; }
  pc = 0x402537; continue;
 }
 0x402537 => {
  { let v: u32 = cpu.r32(0x4a1104u32); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x402540); cpu.call(t); if cpu.returned_to != 0x402540 { return; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  let t = cpu.r32(0x4a108cu32); cpu.push(0x402554); cpu.call(t); if cpu.returned_to != 0x402554 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a1100u32); cpu.push(0x40255e); cpu.call(t); if cpu.returned_to != 0x40255e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40256d; continue; }
  pc = 0x402562; continue;
 }
 0x402562 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a1088u32); cpu.push(0x402569); cpu.call(t); if cpu.returned_to != 0x402569 { return; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40256f; continue;
 }
 0x40256d => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40256f; continue;
 }
 0x40256f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40258c; continue; }
  pc = 0x402573; continue;
 }
 0x402573 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x402576); cpu.call(t); if cpu.returned_to != 0x402576 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4a10fcu32); cpu.push(0x40258c); cpu.call(t); if cpu.returned_to != 0x40258c { return; }
  pc = 0x40258c; continue;
 }
 0x40258c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x402590; continue;
 }
 0x402590 => {
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x40259c; continue; }
  pc = 0x402595; continue;
 }
 0x402595 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4a1094u32); cpu.push(0x40259c); cpu.call(t); if cpu.returned_to != 0x40259c { return; }
  pc = 0x40259c; continue;
 }
 0x40259c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004025b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4025b0); } r_004025b0(cpu, 0x4025b0); }
pub fn f_0040261e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40261e); } r_004025b0(cpu, 0x40261e); }
pub fn f_004026e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4026e8); } r_004025b0(cpu, 0x4026e8); }
fn r_004025b0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4025b0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x4025ca; continue; }
  pc = 0x4025c0; continue;
 }
 0x4025c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4025ca => {
  { let v: u32 = cpu.r32(0x4a10a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x4025d8); cpu.call(t); if cpu.returned_to != 0x4025d8 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0xeu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a10f8u32); cpu.push(0x4025ec); cpu.call(t); if cpu.returned_to != 0x4025ec { return; }
  { cpu.sub32(cpu.eax, 0xeu32); }
  if cpu.zf { pc = 0x4025fb; continue; }
  pc = 0x4025f1; continue;
 }
 0x4025f1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4025fb => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.esp)), 0x4d42u16); }
  if cpu.zf { pc = 0x40260d; continue; }
  pc = 0x402604; continue;
 }
 0x402604 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4au32.wrapping_add(cpu.esp), v); }
  let t = cpu.ebx; cpu.push(0x40260d); cpu.call(t); if cpu.returned_to != 0x40260d { return; }
  pc = 0x40260d; continue;
 }
 0x40260d => {
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a10f8u32); cpu.push(0x40261b); cpu.call(t); if cpu.returned_to != 0x40261b { return; }
  { cpu.sub32(cpu.eax, 0x28u32); }
  pc = 0x40261e; continue;
 }
 0x40261e => {
  if cpu.zf { pc = 0x40262a; continue; }
  pc = 0x402620; continue;
 }
 0x402620 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40262a => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if !cpu.zf { pc = 0x40264b; continue; }
  pc = 0x402632; continue;
 }
 0x402632 => {
  { cpu.sub16(cpu.r16(0x52u32.wrapping_add(cpu.esp)), 0x8u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40264b; continue; }
  pc = 0x40263a; continue;
 }
 0x40263a => {
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x402651; continue;
 }
 0x40264b => {
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x402651; continue;
 }
 0x402651 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xcu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x4026cd; continue; }
  pc = 0x40265e; continue;
 }
 0x40265e => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = 0xffffffe4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x28u32; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5eu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x6cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.esp), v); }
  let t = cpu.ebx; cpu.push(0x4026c5); cpu.call(t); if cpu.returned_to != 0x4026c5 { return; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x4026cd; continue;
 }
 0x4026cd => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.esp)), cpu.edi); }
  if !cpu.zf { pc = 0x4026f9; continue; }
  pc = 0x4026d3; continue;
 }
 0x4026d3 => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1fu32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x1ffffffcu32; cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.r32(0x4cu32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x4026f9; continue;
 }
 0x4026e8 => {
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffce6u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.r32(0x4cu32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x4026f9; continue;
 }
 0x4026f9 => {
  { cpu.sub32(cpu.edx, cpu.edi); }
  if !cpu.zf { pc = 0x402710; continue; }
  pc = 0x4026fd; continue;
 }
 0x4026fd => {
  { cpu.sub16(cpu.r16(0x52u32.wrapping_add(cpu.esp)), 0x8u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x402710; continue; }
  pc = 0x402705; continue;
 }
 0x402705 => {
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  pc = 0x402710; continue;
 }
 0x402710 => {
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  let t = cpu.r32(0x4a1084u32); cpu.push(0x402736); cpu.call(t); if cpu.returned_to != 0x402736 { return; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x402748; continue; }
  pc = 0x40273e; continue;
 }
 0x40273e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402748 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1104u32); cpu.push(0x40274f); cpu.call(t); if cpu.returned_to != 0x40274f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x28u32); }
  if (cpu.cf || cpu.zf) { pc = 0x402796; continue; }
  pc = 0x402765; continue;
 }
 0x402765 => {
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4a10f8u32); cpu.edi = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x40277a); cpu.call(t); if cpu.returned_to != 0x40277a { return; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x40279c; continue; }
  pc = 0x402781; continue;
 }
 0x402781 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a1088u32); cpu.push(0x40278c); cpu.call(t); if cpu.returned_to != 0x40278c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402796 => {
  { let v: u32 = cpu.r32(0x4a10f8u32); cpu.edi = v; }
  pc = 0x40279c; continue;
 }
 0x40279c => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.esp)), 0x0u16); }
  if cpu.zf { pc = 0x402806; continue; }
  pc = 0x4027a6; continue;
 }
 0x4027a6 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0xcu32); }
  if !cpu.zf { pc = 0x4027f9; continue; }
  pc = 0x4027ad; continue;
 }
 0x4027ad => {
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4027ba); cpu.call(t); if cpu.returned_to != 0x4027ba { return; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x402806; continue; }
  pc = 0x4027c1; continue;
 }
 0x4027c1 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x4027d3; continue;
 }
 0x4027d3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x12u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4027d3; continue; }
  pc = 0x4027f7; continue;
 }
 0x4027f7 => {
  pc = 0x402806; continue;
 }
 0x4027f9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x402806); cpu.call(t); if cpu.returned_to != 0x402806 { return; }
  pc = 0x402806; continue;
 }
 0x402806 => {
  { let v: u32 = cpu.r32(0x3eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402822; continue; }
  pc = 0x40280e; continue;
 }
 0x40280e => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10a8u32); cpu.push(0x402822); cpu.call(t); if cpu.returned_to != 0x402822 { return; }
  pc = 0x402822; continue;
 }
 0x402822 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a108cu32); cpu.push(0x40282d); cpu.call(t); if cpu.returned_to != 0x40282d { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402840); }
 let mut pc: u32 = 0x402840;
 loop { match pc {
 0x402840 => {
  { let v: u32 = cpu.r32(0x5df590u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5ec8c4u32; cpu.push(v); }
  { let v = 0x500u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x402857); f_00412220(cpu); if cpu.returned_to != 0x402857 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40286b; continue; }
  pc = 0x40285b; continue;
 }
 0x40285b => {
  { let v = 0x4a45e0u32; cpu.push(v); }
  cpu.push(0x402865); f_004030c0(cpu); if cpu.returned_to != 0x402865 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40286b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402880); }
 let mut pc: u32 = 0x402880;
 loop { match pc {
 0x402880 => {
  { let v: u32 = cpu.r32(0x5ec8c4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402899; continue; }
  pc = 0x402889; continue;
 }
 0x402889 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x40288f); cpu.call(t); if cpu.returned_to != 0x40288f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ec8c4u32, v); }
  pc = 0x402899; continue;
 }
 0x402899 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004028a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4028a0); }
 let mut pc: u32 = 0x4028a0;
 loop { match pc {
 0x4028a0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4a15e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a15ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4a15f0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4a15f4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5ec8c4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x4028dc; continue; }
  pc = 0x4028d2; continue;
 }
 0x4028d2 => {
  cpu.push(0x4028d7); f_00402840(cpu); if cpu.returned_to != 0x4028d7 { return; }
  { let v: u32 = cpu.r32(0x5ec8c4u32); cpu.eax = v; }
  pc = 0x4028dc; continue;
 }
 0x4028dc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x5ec8e0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.push(0x4028ee); cpu.call(t); if cpu.returned_to != 0x4028ee { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402905; continue; }
  pc = 0x4028f2; continue;
 }
 0x4028f2 => {
  { let v = 0x4a4654u32; cpu.push(v); }
  cpu.push(0x4028fc); f_004030c0(cpu); if cpu.returned_to != 0x4028fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402905 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v = 0x4122a0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.push(0x402915); cpu.call(t); if cpu.returned_to != 0x402915 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402941; continue; }
  pc = 0x402919; continue;
 }
 0x402919 => {
  { let v = 0x4a463cu32; cpu.push(v); }
  cpu.push(0x402923); f_004030c0(cpu); if cpu.returned_to != 0x402923 { return; }
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x402931); cpu.call(t); if cpu.returned_to != 0x402931 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ec8e0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402941 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.push(0x402955); cpu.call(t); if cpu.returned_to != 0x402955 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402960; continue; }
  pc = 0x402959; continue;
 }
 0x402959 => {
  { let v = 0x4a4614u32; cpu.push(v); }
  pc = 0x402974; continue;
 }
 0x402960 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x40296b); cpu.call(t); if cpu.returned_to != 0x40296b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402997; continue; }
  pc = 0x40296f; continue;
 }
 0x40296f => {
  { let v = 0x4a45fcu32; cpu.push(v); }
  pc = 0x402974; continue;
 }
 0x402974 => {
  cpu.push(0x402979); f_004030c0(cpu); if cpu.returned_to != 0x402979 { return; }
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x402987); cpu.call(t); if cpu.returned_to != 0x402987 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ec8e0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402997 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1fa9u32, v); }
  { let v: u32 = 0x2cu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x4029b6); cpu.call(t); if cpu.returned_to != 0x4029b6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4029cc; continue; }
  pc = 0x4029ba; continue;
 }
 0x4029ba => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x4029cc; continue; }
  pc = 0x4029c7; continue;
 }
 0x4029c7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa9u32, v); }
  pc = 0x4029cc; continue;
 }
 0x4029cc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004029e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4029e0); }
 let mut pc: u32 = 0x4029e0;
 loop { match pc {
 0x4029e0 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402a04; continue; }
  pc = 0x4029e9; continue;
 }
 0x4029e9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.push(0x4029ef); cpu.call(t); if cpu.returned_to != 0x4029ef { return; }
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4029fa); cpu.call(t); if cpu.returned_to != 0x4029fa { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ec8e0u32, v); }
  pc = 0x402a04; continue;
 }
 0x402a04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_00402a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402a10); }
 let mut pc: u32 = 0x402a10;
 loop { match pc {
 0x402a10 => {
  { let v: u32 = cpu.r32(0x5ec8e0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x402aea; continue; }
  pc = 0x402a21; continue;
 }
 0x402a21 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x5ec8d0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.push(0x402a2e); cpu.call(t); if cpu.returned_to != 0x402a2e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402aea; continue; }
  pc = 0x402a36; continue;
 }
 0x402a36 => {
  { let v: u32 = cpu.r32(0x5ec8d0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ec8d4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ec8dcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5eca00u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5ec8deu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(8)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5eca08u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 7u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5eca1cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eca20u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eca00u32, v); }
  if (cpu.sf == cpu.of) { pc = 0x402a93; continue; }
  pc = 0x402a8b; continue;
 }
 0x402a8b => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5eca00u32, v); }
  pc = 0x402a93; continue;
 }
 0x402a93 => {
  { let v: u32 = cpu.r32(0x5eca0cu32); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x402aa3; continue; }
  pc = 0x402a9d; continue;
 }
 0x402a9d => {
  { let v: u32 = cpu.edi; cpu.w32(0x5eca00u32, v); }
  pc = 0x402aa3; continue;
 }
 0x402aa3 => {
  { let v: u32 = cpu.r32(0x5eca04u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5eca10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eca04u32, v); }
  if (cpu.sf == cpu.of) { pc = 0x402ac3; continue; }
  pc = 0x402abb; continue;
 }
 0x402abb => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5eca04u32, v); }
  pc = 0x402ac3; continue;
 }
 0x402ac3 => {
  { let v: u32 = cpu.r32(0x5eca14u32); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x402ad3; continue; }
  pc = 0x402acd; continue;
 }
 0x402acd => {
  { let v: u32 = cpu.edi; cpu.w32(0x5eca04u32, v); }
  pc = 0x402ad3; continue;
 }
 0x402ad3 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5eca24u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402aea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402b00); }
 let mut pc: u32 = 0x402b00;
 loop { match pc {
 0x402b00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x402b37; continue; }
  pc = 0x402b0a; continue;
 }
 0x402b0a => {
  cpu.poll(); pc = cpu.r32(0x402b40u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x402b11 => {
  { let v: u32 = cpu.r32(0x5eca00u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b17 => {
  { let v: u32 = cpu.r32(0x5eca04u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b1d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5eca24u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b25 => {
  { let v: u32 = cpu.r32(0x5eca18u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b2b => {
  { let v: u32 = cpu.r32(0x5eca1cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b31 => {
  { let v: u32 = cpu.r32(0x5eca20u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402b37 => {
  cpu.push(0x402b3c); f_00402a10(cpu); if cpu.returned_to != 0x402b3c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402b60); }
 let mut pc: u32 = 0x402b60;
 loop { match pc {
 0x402b60 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4a15f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a15fcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4a1600u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4a1604u32); cpu.eax = v; }
  { let v = 0x4a46dcu32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x402b93); f_004030c0(cpu); if cpu.returned_to != 0x402b93 { return; }
  { let v: u32 = cpu.r32(0x5ec8c4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402bb1; continue; }
  pc = 0x402b9f; continue;
 }
 0x402b9f => {
  cpu.push(0x402ba4); f_00402840(cpu); if cpu.returned_to != 0x402ba4 { return; }
  { let v: u32 = cpu.r32(0x5ec8c4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402bb1; continue; }
  pc = 0x402bad; continue;
 }
 0x402bad => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402bb1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x5dda9cu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.push(0x402bc3); cpu.call(t); if cpu.returned_to != 0x402bc3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402bda; continue; }
  pc = 0x402bc7; continue;
 }
 0x402bc7 => {
  { let v = 0x4a46b8u32; cpu.push(v); }
  cpu.push(0x402bd1); f_004030c0(cpu); if cpu.returned_to != 0x402bd1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402bda => {
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v = 0x4132c0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.push(0x402bea); cpu.call(t); if cpu.returned_to != 0x402bea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402c16; continue; }
  pc = 0x402bee; continue;
 }
 0x402bee => {
  { let v = 0x4a469cu32; cpu.push(v); }
  cpu.push(0x402bf8); f_004030c0(cpu); if cpu.returned_to != 0x402bf8 { return; }
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x402c06); cpu.call(t); if cpu.returned_to != 0x402c06 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5dda9cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402c16 => {
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.push(0x402c2a); cpu.call(t); if cpu.returned_to != 0x402c2a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402c35; continue; }
  pc = 0x402c2e; continue;
 }
 0x402c2e => {
  { let v = 0x4a4688u32; cpu.push(v); }
  pc = 0x402c49; continue;
 }
 0x402c35 => {
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x402c40); cpu.call(t); if cpu.returned_to != 0x402c40 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x402c6c; continue; }
  pc = 0x402c44; continue;
 }
 0x402c44 => {
  { let v = 0x4a4674u32; cpu.push(v); }
  pc = 0x402c49; continue;
 }
 0x402c49 => {
  cpu.push(0x402c4e); f_004030c0(cpu); if cpu.returned_to != 0x402c4e { return; }
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x402c5c); cpu.call(t); if cpu.returned_to != 0x402c5c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5dda9cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402c6c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402c80); }
 let mut pc: u32 = 0x402c80;
 loop { match pc {
 0x402c80 => {
  { let v = 0x4a46f0u32; cpu.push(v); }
  cpu.push(0x402c8a); f_004030c0(cpu); if cpu.returned_to != 0x402c8a { return; }
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402cb1; continue; }
  pc = 0x402c96; continue;
 }
 0x402c96 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.push(0x402c9c); cpu.call(t); if cpu.returned_to != 0x402c9c { return; }
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x402ca7); cpu.call(t); if cpu.returned_to != 0x402ca7 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5dda9cu32, v); }
  pc = 0x402cb1; continue;
 }
 0x402cb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402d10); }
 let mut pc: u32 = 0x402d10;
 loop { match pc {
 0x402cc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x402cc5; continue;
 }
 0x402cc5 => {
  { let v: u8 = cpu.r8(0x5ec7c0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5ec900u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x402d02; continue; }
  pc = 0x402cd5; continue;
 }
 0x402cd5 => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x402cfc; continue; }
  pc = 0x402cda; continue;
 }
 0x402cda => {
  { cpu.sub8((cpu.eax as u8), 0xd3u8); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5ec8c0u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5eca25u32, v); }
  if cpu.zf { pc = 0x402cf1; continue; }
  pc = 0x402ce9; continue;
 }
 0x402ce9 => {
  { cpu.sub8((cpu.eax as u8), 0xd0u8); }
  if cpu.zf { pc = 0x402cf1; continue; }
  pc = 0x402ced; continue;
 }
 0x402ced => {
  { cpu.sub8((cpu.eax as u8), 0xc8u8); }
  if !cpu.zf { pc = 0x402cfc; continue; }
  pc = 0x402cf1; continue;
 }
 0x402cf1 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5eca25u32, v); }
  pc = 0x402cfc; continue;
 }
 0x402cfc => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5ec900u32.wrapping_add(cpu.eax), v); }
  pc = 0x402d02; continue;
 }
 0x402d02 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x402cc5; continue; }
  pc = 0x402d0a; continue;
 }
 0x402d0a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402d10 => {
  { let v: u32 = cpu.r32(0x5dda9cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402d29; continue; }
  pc = 0x402d19; continue;
 }
 0x402d19 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x5ec7c0u32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.push(0x402d29); cpu.call(t); if cpu.returned_to != 0x402d29 { return; }
  pc = 0x402d29; continue;
 }
 0x402d29 => {
  cpu.poll(); pc = 0x402cc0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402d30); }
 let mut pc: u32 = 0x402d30;
 loop { match pc {
 0x402d30 => {
  cpu.push(0x402d35); f_00402d10(cpu); if cpu.returned_to != 0x402d35 { return; }
  cpu.push(0x402d3a); f_00402dd0(cpu); if cpu.returned_to != 0x402d3a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x402d42; continue; }
  pc = 0x402d3e; continue;
 }
 0x402d3e => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402d42 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ec8c0u32, v); }
  { let v: u8 = cpu.r8(0x5eca25u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402d60); }
 let mut pc: u32 = 0x402d60;
 loop { match pc {
 0x402d60 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5eca18u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402d70); }
 let mut pc: u32 = 0x402d70;
 loop { match pc {
 0x402d70 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5eca18u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402d80); }
 let mut pc: u32 = 0x402d80;
 loop { match pc {
 0x402d80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x402d97; continue; }
  pc = 0x402d8c; continue;
 }
 0x402d8c => {
  { let v: u32 = cpu.eax; cpu.w32(0x5eca08u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eca0cu32, v); }
  pc = 0x402d97; continue;
 }
 0x402d97 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x402dae; continue; }
  pc = 0x402da3; continue;
 }
 0x402da3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5eca10u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eca14u32, v); }
  pc = 0x402dae; continue;
 }
 0x402dae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402db0); }
 let mut pc: u32 = 0x402db0;
 loop { match pc {
 0x402db0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eca00u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eca04u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402dd0); }
 let mut pc: u32 = 0x402dd0;
 loop { match pc {
 0x402dd0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5ec8c0u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402de0); }
 let mut pc: u32 = 0x402de0;
 loop { match pc {
 0x402de0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x1eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x402ecf; continue; }
  pc = 0x402e0a; continue;
 }
 0x402e0a => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x402efcu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x402ed8u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x402e19 => {
  cpu.push(0x402e1e); f_00402d70(cpu); if cpu.returned_to != 0x402e1e { return; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402e25 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x402e2c); f_00402b00(cpu); if cpu.returned_to != 0x402e2c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x402e36); f_00402b00(cpu); if cpu.returned_to != 0x402e36 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec484u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x402e45); f_00402b00(cpu); if cpu.returned_to != 0x402e45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec488u32, v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x402e54); f_00402b00(cpu); if cpu.returned_to != 0x402e54 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec48cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402e63 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x402e6a); f_00402db0(cpu); if cpu.returned_to != 0x402e6a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402e74 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x402e7f); f_00402d80(cpu); if cpu.returned_to != 0x402e7f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402e89 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x402e94); f_00402d80(cpu); if cpu.returned_to != 0x402e94 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402e9e => {
  cpu.push(0x402ea3); f_00402d60(cpu); if cpu.returned_to != 0x402ea3 { return; }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402eaa => {
  cpu.push(0x402eaf); f_00402d70(cpu); if cpu.returned_to != 0x402eaf { return; }
  { let v: u32 = 0x1fu32; cpu.w32(0x5ec480u32, v); }
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402ec0 => {
  cpu.push(0x402ec5); f_00402d60(cpu); if cpu.returned_to != 0x402ec5 { return; }
  { let v: u32 = 0x20u32; cpu.w32(0x5ec480u32, v); }
  pc = 0x402ecf; continue;
 }
 0x402ecf => {
  { let v: u32 = 0x5ec480u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402f20); }
 let mut pc: u32 = 0x402f20;
 loop { match pc {
 0x402f20 => {
  cpu.push(0x402f25); f_004028a0(cpu); if cpu.returned_to != 0x402f25 { return; }
  cpu.push(0x402f2a); f_00404dc0(cpu); if cpu.returned_to != 0x402f2a { return; }
  f_00402d60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402f30); }
 let mut pc: u32 = 0x402f30;
 loop { match pc {
 0x402f30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x402f3d; continue; }
  pc = 0x402f38; continue;
 }
 0x402f38 => {
  f_00402b60(cpu); return;
 }
 0x402f3d => {
  f_00402c80(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402f70); }
 let mut pc: u32 = 0x402f70;
 loop { match pc {
 0x402f70 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10acu32); cpu.push(0x402f7e); cpu.call(t); if cpu.returned_to != 0x402f7e { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x182u32); cpu.eax = v; }
  if cpu.zf { pc = 0x402fa1; continue; }
  pc = 0x402f89; continue;
 }
 0x402f89 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x64u32); cpu.eax = v; }
  if cpu.zf { pc = 0x402f99; continue; }
  pc = 0x402f8e; continue;
 }
 0x402f8e => {
  { let v: u32 = cpu.eax.wrapping_sub(0x64u32); cpu.eax = v; }
  { let v: u16 = 0x5u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402f99 => {
  { let v: u16 = 0x4u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402fa1 => {
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402fb0); }
 let mut pc: u32 = 0x402fb0;
 loop { match pc {
 0x402fb0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00402fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x402fc0); }
 let mut pc: u32 = 0x402fc0;
 loop { match pc {
 0x402fc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x0u32); cpu.eax = v; }
  if cpu.zf { pc = 0x402ff9; continue; }
  pc = 0x402fc9; continue;
 }
 0x402fc9 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x402fe4; continue; }
  pc = 0x402fcc; continue;
 }
 0x402fcc => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40300d; continue; }
  pc = 0x402fcf; continue;
 }
 0x402fcf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x402fe0); f_0048cd40(cpu); if cpu.returned_to != 0x402fe0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402fe4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x402ff5); f_0048cd40(cpu); if cpu.returned_to != 0x402ff5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x402ff9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40300a); f_0048cd40(cpu); if cpu.returned_to != 0x40300a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x40300d; continue;
 }
 0x40300d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403030); }
 let mut pc: u32 = 0x403030;
 loop { match pc {
 0x403030 => {
  { cpu.and8(cpu.r8(0x5ec7eau32), 0x80u8); }
  if cpu.zf { pc = 0x40303f; continue; }
  pc = 0x403039; continue;
 }
 0x403039 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40303f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5ec7f6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040304c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40304c); }
 let mut pc: u32 = 0x40304c;
 loop { match pc {
 0x40304c => {
  {  }
  {  }
  {  }
  {  }
  f_00403050(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403050); }
 let mut pc: u32 = 0x403050;
 loop { match pc {
 0x403050 => {
  { cpu.and8(cpu.r8(0x5ec7ddu32), 0x80u8); }
  if cpu.zf { pc = 0x40305f; continue; }
  pc = 0x403059; continue;
 }
 0x403059 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40305f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5ec85du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403080); }
 let mut pc: u32 = 0x403080;
 loop { match pc {
 0x403080 => {
  { cpu.and8(cpu.r8(0x5ec7f8u32), 0x80u8); }
  if cpu.zf { pc = 0x40308f; continue; }
  pc = 0x403089; continue;
 }
 0x403089 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40308f => {
  { let v: u32 = cpu.r32(0x5ec878u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004030a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4030a0); }
 let mut pc: u32 = 0x4030a0;
 loop { match pc {
 0x4030a0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5ec7feu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004030b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4030b0); }
 let mut pc: u32 = 0x4030b0;
 loop { match pc {
 0x4030b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4030ba); f_004030c0(cpu); if cpu.returned_to != 0x4030ba { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004030c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4030c0); }
 let mut pc: u32 = 0x4030c0;
 loop { match pc {
 0x4030c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004030d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4030d0); }
 let mut pc: u32 = 0x4030d0;
 loop { match pc {
 0x4030d0 => {
  cpu.push(0x4030d5); f_004030f0(cpu); if cpu.returned_to != 0x4030d5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4030e2; continue; }
  pc = 0x4030d9; continue;
 }
 0x4030d9 => {
  cpu.push(0x4030de); f_004030f0(cpu); if cpu.returned_to != 0x4030de { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x4030d9; continue; }
  pc = 0x4030e2; continue;
 }
 0x4030e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004030f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4030f0); }
 let mut pc: u32 = 0x4030f0;
 loop { match pc {
 0x4030f0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddaa0u32, v); }
  cpu.push(0x4030ff); f_00402d30(cpu); if cpu.returned_to != 0x4030ff { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x403107; continue; }
  pc = 0x403104; continue;
 }
 0x403104 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403107 => {
  { let v: u32 = cpu.r8(0x5df780u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddaa0u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403120); }
 let mut pc: u32 = 0x403120;
 loop { match pc {
 0x403120 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x403126); f_0040d290(cpu); if cpu.returned_to != 0x403126 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x403130); f_0040d180(cpu); if cpu.returned_to != 0x403130 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x5df900u32; cpu.push(v); }
  { let v = 0x4a4718u32; cpu.push(v); }
  cpu.push(0x403142); f_004030c0(cpu); if cpu.returned_to != 0x403142 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x40314a); f_0048d0a0(cpu); if cpu.returned_to != 0x40314a { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5df900u32; cpu.push(v); }
  cpu.push(0x403163); f_00407090(cpu); if cpu.returned_to != 0x403163 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4031a5; continue; }
  pc = 0x40316a; continue;
 }
 0x40316a => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x403171); f_00402b00(cpu); if cpu.returned_to != 0x403171 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x40317b); f_00402d70(cpu); if cpu.returned_to != 0x40317b { return; }
  { let v = 0x4a4708u32; cpu.push(v); }
  cpu.push(0x403185); f_004030c0(cpu); if cpu.returned_to != 0x403185 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x40318d); f_00401ff0(cpu); if cpu.returned_to != 0x40318d { return; }
  { let v = 0x3e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x403199); f_00407130(cpu); if cpu.returned_to != 0x403199 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x4031a5; continue; }
  pc = 0x4031a0; continue;
 }
 0x4031a0 => {
  cpu.push(0x4031a5); f_00402d60(cpu); if cpu.returned_to != 0x4031a5 { return; }
  pc = 0x4031a5; continue;
 }
 0x4031a5 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004031b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4031b0); }
 let mut pc: u32 = 0x4031b0;
 loop { match pc {
 0x4031b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x5ec450u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u8 = 0x80u8; cpu.w8(0x5ebee4u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 1); }
  if !cpu.zf { pc = 0x4031f3; continue; }
  pc = 0x4031e2; continue;
 }
 0x4031e2 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5ebee4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5ec450u32; cpu.w32(0x5ebee5u32.wrapping_add(cpu.eax), v); }
  pc = 0x4031f3; continue;
 }
 0x4031f3 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x5ebdb0u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.cmps(1, 1); }
  if !cpu.zf { pc = 0x403216; continue; }
  pc = 0x403205; continue;
 }
 0x403205 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ebee4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5ebdb0u32; cpu.w32(0x5ebee5u32.wrapping_add(cpu.eax), v); }
  pc = 0x403216; continue;
 }
 0x403216 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x5ec460u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.cmps(1, 1); }
  if !cpu.zf { pc = 0x403239; continue; }
  pc = 0x403228; continue;
 }
 0x403228 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5ebee4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5ec460u32; cpu.w32(0x5ebee5u32.wrapping_add(cpu.eax), v); }
  pc = 0x403239; continue;
 }
 0x403239 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403250); }
 let mut pc: u32 = 0x403250;
 loop { match pc {
 0x403250 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ddb08u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ebee4u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x40328a; continue; }
  pc = 0x403273; continue;
 }
 0x403273 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5ebed4u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4031b0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.push(0x403289); cpu.call(t); if cpu.returned_to != 0x403289 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x40328a; continue;
 }
 0x40328a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403290); }
 let mut pc: u32 = 0x403290;
 loop { match pc {
 0x403290 => {
  { let v: u32 = cpu.r32(0x4a1628u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a162cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4a1630u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec450u32, v); }
  { let v: u32 = cpu.r32(0x4a1634u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ec454u32, v); }
  { let v: u32 = cpu.r32(0x4a1618u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec458u32, v); }
  { let v: u32 = cpu.r32(0x4a161cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec45cu32, v); }
  { let v: u32 = cpu.r32(0x4a1620u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ebdb0u32, v); }
  { let v: u32 = cpu.r32(0x4a1624u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ebdb4u32, v); }
  { let v: u32 = cpu.r32(0x4a1608u32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebdb8u32, v); }
  { let v: u32 = cpu.r32(0x4a160cu32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ebdbcu32, v); }
  { let v: u32 = cpu.r32(0x4a1610u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec460u32, v); }
  { let v: u32 = cpu.r32(0x4a1614u32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5ddb08u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec464u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ec468u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec46cu32, v); }
  cpu.push(0x40332a); f_004132f6(cpu); if cpu.returned_to != 0x40332a { return; }
  { let v: u32 = cpu.r32(0x5ebdc0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403336; continue; }
  pc = 0x403333; continue;
 }
 0x403333 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403336 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x403390u32; cpu.push(v); }
  cpu.push(0x403343); f_004132f0(cpu); if cpu.returned_to != 0x403343 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x40334c); f_00403250(cpu); if cpu.returned_to != 0x40334c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x403356); f_00403250(cpu); if cpu.returned_to != 0x403356 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x403360); f_00403250(cpu); if cpu.returned_to != 0x403360 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x40336a); f_00403250(cpu); if cpu.returned_to != 0x40336a { return; }
  { let v: u32 = cpu.r32(0x5ddb08u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403386; continue; }
  pc = 0x403376; continue;
 }
 0x403376 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x40337c); cpu.call(t); if cpu.returned_to != 0x40337c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb08u32, v); }
  pc = 0x403386; continue;
 }
 0x403386 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403390); }
 let mut pc: u32 = 0x403390;
 loop { match pc {
 0x403390 => {
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x5ebdd4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = 0x5ebed4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebdd0u32, v); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403420); }
 let mut pc: u32 = 0x403420;
 loop { match pc {
 0x403420 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403455; continue; }
  pc = 0x403429; continue;
 }
 0x403429 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5eb430u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eac00u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x5eac00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5eac04u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x68u32.wrapping_add(cpu.edx)); cpu.push(0x403455); cpu.call(t); if cpu.returned_to != 0x403455 { return; }
  pc = 0x403455; continue;
 }
 0x403455 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403460(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x403460); } r_00403460(cpu, 0x403460); }
pub fn f_0040348e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40348e); } r_00403460(cpu, 0x40348e); }
fn r_00403460(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x403460 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4034c3; continue; }
  pc = 0x40346f; continue;
 }
 0x40346f => {
  { let v: u32 = cpu.r32(0x5eb430u32); cpu.ecx = v; }
  { let v: u32 = 0x408u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x5eac00u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.push(0x40349d); cpu.call(t); if cpu.returned_to != 0x40349d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4034c3; continue; }
  pc = 0x4034a1; continue;
 }
 0x40348e => {
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.push(0x40349d); cpu.call(t); if cpu.returned_to != 0x40349d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4034c3; continue; }
  pc = 0x4034a1; continue;
 }
 0x4034a1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4034c3; continue; }
  pc = 0x4034a9; continue;
 }
 0x4034a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eac04u32); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5eac00u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4034c3; continue;
 }
 0x4034c3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004034d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4034d0); }
 let mut pc: u32 = 0x4034d0;
 loop { match pc {
 0x4034d0 => {
  { let v: u32 = cpu.r32(0x5de964u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x4034ec; continue; }
  pc = 0x4034de; continue;
 }
 0x4034de => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x4034e8; continue; }
  pc = 0x4034e1; continue;
 }
 0x4034e1 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x4034f3; continue; }
  pc = 0x4034e4; continue;
 }
 0x4034e4 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x4034f7; continue;
 }
 0x4034e8 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x4034f7; continue;
 }
 0x4034ec => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x4034f7; continue;
 }
 0x4034f3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x4034f7; continue;
 }
 0x4034f7 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4035ee; continue; }
  pc = 0x403504; continue;
 }
 0x403504 => {
  { let v: u32 = cpu.r32(0x5ddb74u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddb74u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5eb020u32, v); }
  { let v: u16 = cpu.r16(0x575d16u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eb026u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5eb024u32, v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x5eb02au32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5eb022u32, v); }
  { let v: u32 = cpu.r32(0x5eb430u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5eb026u32); cpu.ecx = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x5eb022u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4035b5; continue; }
  pc = 0x403572; continue;
 }
 0x403572 => {
  { let v: u32 = cpu.r8(0x5eb02fu32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02eu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5eb02du32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02cu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5eb02bu32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02au32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4a472cu32; cpu.push(v); }
  cpu.push(0x4035b0); f_004030c0(cpu); if cpu.returned_to != 0x4035b0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  pc = 0x4035c6; continue;
 }
 0x4035b5 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4a4720u32; cpu.push(v); }
  cpu.push(0x4035c3); f_004030c0(cpu); if cpu.returned_to != 0x4035c3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x4035c6; continue;
 }
 0x4035c6 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x5eb020u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x68u32.wrapping_add(cpu.ecx)); cpu.push(0x4035db); cpu.call(t); if cpu.returned_to != 0x4035db { return; }
  { let v: u32 = cpu.r16(0x5eb024u32) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5eb026u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddb78u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  pc = 0x4035ee; continue;
 }
 0x4035ee => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403600); }
 let mut pc: u32 = 0x403600;
 loop { match pc {
 0x403600 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4a4744u32; cpu.push(v); }
  cpu.push(0x403614); f_004030c0(cpu); if cpu.returned_to != 0x403614 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403620); }
 let mut pc: u32 = 0x403620;
 loop { match pc {
 0x403620 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x400u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40373d; continue; }
  pc = 0x40363b; continue;
 }
 0x40363b => {
  cpu.push(0x403640); f_004102a0(cpu); if cpu.returned_to != 0x403640 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403649; continue; }
  pc = 0x403644; continue;
 }
 0x403644 => {
  cpu.push(0x403649); f_00415180(cpu); if cpu.returned_to != 0x403649 { return; }
  pc = 0x403649; continue;
 }
 0x403649 => {
  { let v: u32 = cpu.r32(0x5eb430u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x5eb020u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x40au32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x40367b); cpu.call(t); if cpu.returned_to != 0x40367b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40373d; continue; }
  pc = 0x403683; continue;
 }
 0x403683 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40373d; continue; }
  pc = 0x40368f; continue;
 }
 0x40368f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eb020u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x5eb022u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x5eb024u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5eb02au32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x5eb022u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5eb026u32); cpu.ecx = v; }
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x403718; continue; }
  pc = 0x4036d5; continue;
 }
 0x4036d5 => {
  { let v: u32 = cpu.r8(0x5eb02fu32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02eu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5eb02du32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02cu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5eb02bu32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5eb02au32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4a4764u32; cpu.push(v); }
  cpu.push(0x403713); f_004030c0(cpu); if cpu.returned_to != 0x403713 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  pc = 0x403729; continue;
 }
 0x403718 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4a4754u32; cpu.push(v); }
  cpu.push(0x403726); f_004030c0(cpu); if cpu.returned_to != 0x403726 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x403729; continue;
 }
 0x403729 => {
  { let v: u32 = cpu.r16(0x5eb024u32) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eb026u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddb78u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x40373d; continue;
 }
 0x40373d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403750); }
 let mut pc: u32 = 0x403750;
 loop { match pc {
 0x403750 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { let v: u32 = 0x80004005u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x403765; continue; }
  pc = 0x40375f; continue;
 }
 0x40375f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x403765); cpu.call(t); if cpu.returned_to != 0x403765 { return; }
  pc = 0x403765; continue;
 }
 0x403765 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403770); }
 let mut pc: u32 = 0x403770;
 loop { match pc {
 0x403770 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x40377f); f_00403a00(cpu); if cpu.returned_to != 0x40377f { return; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x403790); f_004132fc(cpu); if cpu.returned_to != 0x403790 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x4037b9; continue; }
  pc = 0x403796; continue;
 }
 0x403796 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4037b9; continue; }
  pc = 0x40379e; continue;
 }
 0x40379e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = 0x5ddb70u32; cpu.push(v); }
  { let v = 0x4a1558u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4037ad); cpu.call(t); if cpu.returned_to != 0x4037ad { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x4037b9); cpu.call(t); if cpu.returned_to != 0x4037b9 { return; }
  pc = 0x4037b9; continue;
 }
 0x4037b9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004037c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4037c0); }
 let mut pc: u32 = 0x4037c0;
 loop { match pc {
 0x4037c0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80004005u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x403815; continue; }
  pc = 0x4037f4; continue;
 }
 0x4037f4 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x403815); cpu.call(t); if cpu.returned_to != 0x403815 { return; }
  pc = 0x403815; continue;
 }
 0x403815 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403820); }
 let mut pc: u32 = 0x403820;
 loop { match pc {
 0x403820 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df884u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x80004005u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x44u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x40387a; continue; }
  pc = 0x40385f; continue;
 }
 0x40385f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x40387a; continue;
 }
 0x40387a => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403892; continue; }
  pc = 0x403883; continue;
 }
 0x403883 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.push(0x403890); cpu.call(t); if cpu.returned_to != 0x403890 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x403892; continue;
 }
 0x403892 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x50u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004038a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4038a0); }
 let mut pc: u32 = 0x4038a0;
 loop { match pc {
 0x4038a0 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { let v: u32 = 0x80004005u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4038ba; continue; }
  pc = 0x4038af; continue;
 }
 0x4038af => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x4038ba); cpu.call(t); if cpu.returned_to != 0x4038ba { return; }
  pc = 0x4038ba; continue;
 }
 0x4038ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004038c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4038c0); }
 let mut pc: u32 = 0x4038c0;
 loop { match pc {
 0x4038c0 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { let v: u32 = 0x80004005u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4038e9; continue; }
  pc = 0x4038cf; continue;
 }
 0x4038cf => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.push(0x4038e9); cpu.call(t); if cpu.returned_to != 0x4038e9 { return; }
  pc = 0x4038e9; continue;
 }
 0x4038e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004038f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4038f0); }
 let mut pc: u32 = 0x4038f0;
 loop { match pc {
 0x4038f0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x5df884u32); cpu.eax = v; }
  { let v: u32 = 0x80004005u32; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x50u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x403932; continue; }
  pc = 0x403917; continue;
 }
 0x403917 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x403932; continue;
 }
 0x403932 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40395c; continue; }
  pc = 0x40393b; continue;
 }
 0x40393b => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.push(0x40395a); cpu.call(t); if cpu.returned_to != 0x40395a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40395c; continue;
 }
 0x40395c => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x50u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403970); }
 let mut pc: u32 = 0x403970;
 loop { match pc {
 0x403970 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x80004005u32; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x50u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x4039b1; continue; }
  pc = 0x403996; continue;
 }
 0x403996 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x4039b1; continue;
 }
 0x4039b1 => {
  { let v: u32 = cpu.r32(0x5df884u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4039d5; continue; }
  pc = 0x4039ba; continue;
 }
 0x4039ba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x4039d5; continue;
 }
 0x4039d5 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4039ed; continue; }
  pc = 0x4039de; continue;
 }
 0x4039de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.push(0x4039eb); cpu.call(t); if cpu.returned_to != 0x4039eb { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x4039ed; continue;
 }
 0x4039ed => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x50u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403a00); }
 let mut pc: u32 = 0x403a00;
 loop { match pc {
 0x403a00 => {
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { let v: u32 = 0x80004005u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x403a41; continue; }
  pc = 0x403a0f; continue;
 }
 0x403a0f => {
  { let v: u32 = cpu.r32(0x5eb42cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403a31; continue; }
  pc = 0x403a18; continue;
 }
 0x403a18 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x403a1e); f_00402420(cpu); if cpu.returned_to != 0x403a1e { return; }
  { let v: u32 = cpu.r32(0x5ddb70u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eb42cu32, v); }
  pc = 0x403a31; continue;
 }
 0x403a31 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x403a37); cpu.call(t); if cpu.returned_to != 0x403a37 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb70u32, v); }
  pc = 0x403a41; continue;
 }
 0x403a41 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403a50); }
 let mut pc: u32 = 0x403a50;
 loop { match pc {
 0x403a50 => {
  { let v: u32 = cpu.r32(0x5ddb88u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.w32(0x4a4788u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575f20u32, v); }
  if !cpu.zf { pc = 0x403aae; continue; }
  pc = 0x403a6d; continue;
 }
 0x403a6d => {
  { let v = 0x4a478cu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a10b0u32); cpu.push(0x403a7e); cpu.call(t); if cpu.returned_to != 0x403a7e { return; }
  { let v = 0x30000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x403ff0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a4780u32; cpu.push(v); }
  { let v = 0x5eabe0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddb88u32, v); }
  let t = cpu.r32(0x4a12acu32); cpu.push(0x403aa1); cpu.call(t); if cpu.returned_to != 0x403aa1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403aae; continue; }
  pc = 0x403aa5; continue;
 }
 0x403aa5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403aae => {
  { let v: u32 = cpu.r32(0x4a4780u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x34u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4a4788u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575f20u32, v); }
  let t = cpu.r32(0x4a12b0u32); cpu.push(0x403ad3); cpu.call(t); if cpu.returned_to != 0x403ad3 { return; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  { let v: u32 = cpu.eax; cpu.w32(0x5eab8cu32, v); }
  if cpu.zf { pc = 0x403aed; continue; }
  pc = 0x403ae3; continue;
 }
 0x403ae3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eab8cu32, v); }
  pc = 0x403aed; continue;
 }
 0x403aed => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403afe; continue; }
  pc = 0x403af1; continue;
 }
 0x403af1 => {
  { let v = 0xffffu32; cpu.push(v); }
  cpu.push(0x403afb); f_00403b10(cpu); if cpu.returned_to != 0x403afb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x403afe; continue;
 }
 0x403afe => {
  cpu.push(0x403b03); f_00403bb0(cpu); if cpu.returned_to != 0x403b03 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403b10); }
 let mut pc: u32 = 0x403b10;
 loop { match pc {
 0x403b10 => {
  { let v: u32 = cpu.r32(0x5eab8cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403b46; continue; }
  pc = 0x403b19; continue;
 }
 0x403b19 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x403b37; continue; }
  pc = 0x403b2f; continue;
 }
 0x403b2f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a12a8u32); cpu.push(0x403b37); cpu.call(t); if cpu.returned_to != 0x403b37 { return; }
  pc = 0x403b37; continue;
 }
 0x403b37 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4a47a4u32; cpu.push(v); }
  cpu.push(0x403b42); f_004030c0(cpu); if cpu.returned_to != 0x403b42 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x403b46; continue;
 }
 0x403b46 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403b50); }
 let mut pc: u32 = 0x403b50;
 loop { match pc {
 0x403b50 => {
  { let v: u32 = cpu.r32(0x5ddb98u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x403b86; continue; }
  pc = 0x403b5b; continue;
 }
 0x403b5b => {
  { let v: u32 = cpu.r32(0x4a12a4u32); cpu.edi = v; }
  { let v: u32 = 0x5eaae0u32; cpu.esi = v; }
  pc = 0x403b66; continue;
 }
 0x403b66 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x403b71); cpu.call(t); if cpu.returned_to != 0x403b71 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x54u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eab88u32); }
  if cpu.cf { cpu.poll(); pc = 0x403b66; continue; }
  pc = 0x403b7c; continue;
 }
 0x403b7c => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb98u32, v); }
  pc = 0x403b86; continue;
 }
 0x403b86 => {
  { let v: u32 = 0x5eaae0u32; cpu.esi = v; }
  pc = 0x403b8b; continue;
 }
 0x403b8b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403ba0; continue; }
  pc = 0x403b91; continue;
 }
 0x403b91 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x403b97); f_00402420(cpu); if cpu.returned_to != 0x403b97 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x403ba0; continue;
 }
 0x403ba0 => {
  { let v: u32 = cpu.esi.wrapping_add(0x54u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eab88u32); }
  if cpu.cf { cpu.poll(); pc = 0x403b8b; continue; }
  pc = 0x403bab; continue;
 }
 0x403bab => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403bb0); }
 let mut pc: u32 = 0x403bb0;
 loop { match pc {
 0x403bb0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5eaae0u32; cpu.esi = v; }
  { let v: u32 = 0x800u32; cpu.edi = v; }
  pc = 0x403bbc; continue;
 }
 0x403bbc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x403bc5); f_00402400(cpu); if cpu.returned_to != 0x403bc5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403bde; continue; }
  pc = 0x403bce; continue;
 }
 0x403bce => {
  { let v: u32 = cpu.esi.wrapping_add(0x54u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eab88u32); }
  if cpu.cf { cpu.poll(); pc = 0x403bbc; continue; }
  pc = 0x403bd9; continue;
 }
 0x403bd9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403bde => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403bf0); }
 let mut pc: u32 = 0x403bf0;
 loop { match pc {
 0x403bf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5eaae0u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x403c00; continue;
 }
 0x403c00 => {
  { cpu.sub32(cpu.r32(0x5ddb98u32), cpu.ebx); }
  if cpu.zf { pc = 0x403c17; continue; }
  pc = 0x403c08; continue;
 }
 0x403c08 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a12a4u32); cpu.push(0x403c17); cpu.call(t); if cpu.returned_to != 0x403c17 { return; }
  pc = 0x403c17; continue;
 }
 0x403c17 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.ebx; cpu.w32(0x48u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x54u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eab88u32); }
  if cpu.cf { cpu.poll(); pc = 0x403c00; continue; }
  pc = 0x403c33; continue;
 }
 0x403c33 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ddb98u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403c40); }
 let mut pc: u32 = 0x403c40;
 loop { match pc {
 0x403c40 => {
  { let v: u32 = cpu.r32(0x5ddb90u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403c6c; continue; }
  pc = 0x403c49; continue;
 }
 0x403c49 => {
  { let v: u32 = cpu.r32(0x5ddb94u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x403c6c; continue; }
  pc = 0x403c52; continue;
 }
 0x403c52 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403c62; continue; }
  pc = 0x403c5b; continue;
 }
 0x403c5b => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a12a0u32); cpu.push(0x403c62); cpu.call(t); if cpu.returned_to != 0x403c62 { return; }
  pc = 0x403c62; continue;
 }
 0x403c62 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5ddb94u32, v); }
  pc = 0x403c6c; continue;
 }
 0x403c6c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403c70); }
 let mut pc: u32 = 0x403c70;
 loop { match pc {
 0x403c70 => {
  { let v: u32 = cpu.r32(0x5ddb94u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403c9c; continue; }
  pc = 0x403c79; continue;
 }
 0x403c79 => {
  { let v: u32 = cpu.r32(0x5ddb90u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403c9c; continue; }
  pc = 0x403c82; continue;
 }
 0x403c82 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403c92; continue; }
  pc = 0x403c8b; continue;
 }
 0x403c8b => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a129cu32); cpu.push(0x403c92); cpu.call(t); if cpu.returned_to != 0x403c92 { return; }
  pc = 0x403c92; continue;
 }
 0x403c92 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb94u32, v); }
  pc = 0x403c9c; continue;
 }
 0x403c9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403ca0); }
 let mut pc: u32 = 0x403ca0;
 loop { match pc {
 0x403ca0 => {
  { let v: u32 = cpu.r32(0x5ddb90u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403d3e; continue; }
  pc = 0x403cad; continue;
 }
 0x403cad => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.w32(0x5eaac4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403cdc; continue; }
  pc = 0x403cc0; continue;
 }
 0x403cc0 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1294u32); cpu.push(0x403cc7); cpu.call(t); if cpu.returned_to != 0x403cc7 { return; }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403cdc; continue; }
  pc = 0x403cd0; continue;
 }
 0x403cd0 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a127cu32); cpu.push(0x403cd7); cpu.call(t); if cpu.returned_to != 0x403cd7 { return; }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  pc = 0x403cdc; continue;
 }
 0x403cdc => {
  { let v: u32 = cpu.r32(0x5ddb88u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x403d15; continue; }
  pc = 0x403ce6; continue;
 }
 0x403ce6 => {
  { let v = 0x7d0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10b4u32); cpu.push(0x403cf2); cpu.call(t); if cpu.returned_to != 0x403cf2 { return; }
  { cpu.sub32(cpu.eax, 0x102u32); }
  if !cpu.zf { pc = 0x403d10; continue; }
  pc = 0x403cf9; continue;
 }
 0x403cf9 => {
  { let v = 0x4a47b4u32; cpu.push(v); }
  cpu.push(0x403d03); f_004030c0(cpu); if cpu.returned_to != 0x403d03 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xc8u32; cpu.w32(0x5eaac4u32, v); }
  pc = 0x403d10; continue;
 }
 0x403d10 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  pc = 0x403d15; continue;
 }
 0x403d15 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403d2a; continue; }
  pc = 0x403d19; continue;
 }
 0x403d19 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1298u32); cpu.push(0x403d20); cpu.call(t); if cpu.returned_to != 0x403d20 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eabe0u32, v); }
  pc = 0x403d2a; continue;
 }
 0x403d2a => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb94u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb90u32, v); }
  pc = 0x403d3e; continue;
 }
 0x403d3e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403d40); }
 let mut pc: u32 = 0x403d40;
 loop { match pc {
 0x403d40 => {
  { let v: u32 = cpu.r32(0x5ddb8cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403d58; continue; }
  pc = 0x403d49; continue;
 }
 0x403d49 => {
  cpu.push(0x403d4e); f_0040a7a0(cpu); if cpu.returned_to != 0x403d4e { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb8cu32, v); }
  pc = 0x403d58; continue;
 }
 0x403d58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403d60); }
 let mut pc: u32 = 0x403d60;
 loop { match pc {
 0x403d60 => {
  cpu.push(0x403d65); f_00403ca0(cpu); if cpu.returned_to != 0x403d65 { return; }
  cpu.push(0x403d6a); f_00403d40(cpu); if cpu.returned_to != 0x403d6a { return; }
  cpu.push(0x403d6f); f_00403b50(cpu); if cpu.returned_to != 0x403d6f { return; }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403d89; continue; }
  pc = 0x403d78; continue;
 }
 0x403d78 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1298u32); cpu.push(0x403d7f); cpu.call(t); if cpu.returned_to != 0x403d7f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eabe0u32, v); }
  pc = 0x403d89; continue;
 }
 0x403d89 => {
  { let v: u32 = cpu.r32(0x5ddb88u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403d99; continue; }
  pc = 0x403d92; continue;
 }
 0x403d92 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10b8u32); cpu.push(0x403d99); cpu.call(t); if cpu.returned_to != 0x403d99 { return; }
  pc = 0x403d99; continue;
 }
 0x403d99 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddb88u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403db0); }
 let mut pc: u32 = 0x403db0;
 loop { match pc {
 0x403db0 => {
  cpu.push(0x403db5); f_00403ca0(cpu); if cpu.returned_to != 0x403db5 { return; }
  cpu.push(0x403dba); f_00403d40(cpu); if cpu.returned_to != 0x403dba { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddba0u32, v); }
  cpu.push(0x403dcd); f_00403e10(cpu); if cpu.returned_to != 0x403dcd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x403e04; continue; }
  pc = 0x403dd4; continue;
 }
 0x403dd4 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddb8cu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5eaac4u32, v); }
  let t = cpu.r32(0x4a129cu32); cpu.push(0x403df0); cpu.call(t); if cpu.returned_to != 0x403df0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x403dff; continue; }
  pc = 0x403df4; continue;
 }
 0x403df4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5ddb90u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403dff => {
  { let v: u32 = 0xdeadu32; cpu.eax = v; }
  pc = 0x403e04; continue;
 }
 0x403e04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x403e10); }
 let mut pc: u32 = 0x403e10;
 loop { match pc {
 0x403e10 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x403e52; continue; }
  pc = 0x403e24; continue;
 }
 0x403e24 => {
  { let v = 0x30000u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x403ff0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a4780u32; cpu.push(v); }
  { let v = 0x5eabe0u32; cpu.push(v); }
  let t = cpu.r32(0x4a12acu32); cpu.push(0x403e41); cpu.call(t); if cpu.returned_to != 0x403e41 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x403e52; continue; }
  pc = 0x403e45; continue;
 }
 0x403e45 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403e52 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x403e5c); f_0040a480(cpu); if cpu.returned_to != 0x403e5c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x403e70; continue; }
  pc = 0x403e63; continue;
 }
 0x403e63 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403e70 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v: u32 = 0x5eaba0u32; cpu.edi = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5e18a4u32); cpu.ecx = v; }
  { let v = 0x80000001u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a1288u32); cpu.push(0x403ea9); cpu.call(t); if cpu.returned_to != 0x403ea9 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x403ebf; continue; }
  pc = 0x403ead; continue;
 }
 0x403ead => {
  cpu.push(0x403eb2); f_0040a7a0(cpu); if cpu.returned_to != 0x403eb2 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403ebf => {
  cpu.push(0x403ec4); f_00403bf0(cpu); if cpu.returned_to != 0x403ec4 { return; }
  { let v: u32 = cpu.r32(0x4a128cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4a1290u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab88u32, v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eaac0u32, v); }
  pc = 0x403ee3; continue;
 }
 0x403ee3 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x5eaae0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x800u32; cpu.w32(0x5eab24u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab2cu32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x403f19); f_0040a940(cpu); if cpu.returned_to != 0x403f19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x403f2e; continue; }
  pc = 0x403f20; continue;
 }
 0x403f20 => {
  { cpu.sub32(cpu.eax, 0xffffff99u32); }
  if !cpu.zf { pc = 0x403fca; continue; }
  pc = 0x403f29; continue;
 }
 0x403f29 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x403f2e; continue;
 }
 0x403f2e => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eab28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eaae8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ddb98u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { pc = 0x403f73; continue; }
  pc = 0x403f57; continue;
 }
 0x403f57 => {
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.ecx = v; }
  { let v: u32 = 0x5eaae0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x403f69); cpu.call(t); if cpu.returned_to != 0x403f69 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x403fdc; continue; }
  pc = 0x403f6d; continue;
 }
 0x403f6d => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  pc = 0x403f73; continue;
 }
 0x403f73 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = 0x5eaae0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x403f90); cpu.call(t); if cpu.returned_to != 0x403f90 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x403fb0; continue; }
  pc = 0x403f94; continue;
 }
 0x403f94 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.ebp, cpu.esi); }
  if !cpu.zf { pc = 0x403fb0; continue; }
  pc = 0x403f9a; continue;
 }
 0x403f9a => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x2u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eaac0u32, v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x403ee3; continue; }
  pc = 0x403fb0; continue;
 }
 0x403fb0 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5ddb98u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eaac0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403fca => {
  cpu.push(0x403fcf); f_0040a7a0(cpu); if cpu.returned_to != 0x403fcf { return; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x403fdc => {
  cpu.push(0x403fe1); f_0040a7a0(cpu); if cpu.returned_to != 0x403fe1 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00403ff0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x403ff0); } r_00403ff0(cpu, 0x403ff0); }
pub fn f_0040402a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40402a); } r_00403ff0(cpu, 0x40402a); }
pub fn f_00404040(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404040); } r_00403ff0(cpu, 0x404040); }
fn r_00403ff0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x403ff0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x3c9u32); cpu.eax = v; }
  if cpu.zf { pc = 0x404058; continue; }
  pc = 0x403ffc; continue;
 }
 0x403ffc => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x404003; continue;
 }
 0x404003 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xf0u32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0xb0u8); }
  if !cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x404022; continue;
 }
 0x404022 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xff00u32; cpu.ecx = v; }
  pc = 0x40402a; continue;
 }
 0x40402a => {
  { cpu.sub32(cpu.ecx, 0x2700u32); }
  if cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x404036; continue;
 }
 0x404036 => {
  { cpu.sub32(cpu.ecx, 0x700u32); }
  if !cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x404042; continue;
 }
 0x404040 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x404042; continue;
 }
 0x404042 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5eaba0u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x404058 => {
  { let v: u32 = cpu.r32(0x5eaac4u32); cpu.eax = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x40406a; continue;
 }
 0x40406a => {
  { let v: u32 = cpu.r32(0x5eab88u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x12cu32); }
  { let v: u32 = cpu.edx; cpu.w32(0x5eab88u32, v); }
  if !cpu.zf { pc = 0x40409d; continue; }
  pc = 0x40407e; continue;
 }
 0x40407e => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x4041ba; continue; }
  pc = 0x404087; continue;
 }
 0x404087 => {
  { let v: u32 = cpu.r32(0x5ddb88u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eaac4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10bcu32); cpu.push(0x404099); cpu.call(t); if cpu.returned_to != 0x404099 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40409d => {
  { cpu.sub32(cpu.eax, 0x64u32); }
  if !cpu.zf { pc = 0x4040c2; continue; }
  pc = 0x4040a2; continue;
 }
 0x4040a2 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x4041ba; continue; }
  pc = 0x4040ab; continue;
 }
 0x4040ab => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5eaac4u32, v); }
  { let v: u32 = cpu.r32(0x5ddb88u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10bcu32); cpu.push(0x4040be); cpu.call(t); if cpu.returned_to != 0x4040be { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x4040c2 => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5eaac8u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eaae8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ddb90u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x4040f9; continue;
 }
 0x4040f9 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.zf { pc = 0x4041ba; continue; }
  pc = 0x404101; continue;
 }
 0x404101 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5eab20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x5eaae0u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x800u32; cpu.w32(0x5eab24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab2cu32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab28u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eab30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x404130); f_0040a940(cpu); if cpu.returned_to != 0x404130 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x404154; continue; }
  pc = 0x404137; continue;
 }
 0x404137 => {
  { cpu.sub32(cpu.eax, 0xffffff99u32); }
  if !cpu.zf { pc = 0x40418c; continue; }
  pc = 0x40413c; continue;
 }
 0x40413c => {
  { let v: u32 = 0x1u32; cpu.w32(0x5ddba4u32, v); }
  { let v: u32 = 0x12cu32; cpu.w32(0x5eaac4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x404154 => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eab28u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x5eaae0u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5eaae8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5eabe0u32); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1290u32); cpu.push(0x404188); cpu.call(t); if cpu.returned_to != 0x404188 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x404195; continue; }
  pc = 0x40418c; continue;
 }
 0x40418c => {
  cpu.push(0x404191); f_0040a7a0(cpu); if cpu.returned_to != 0x404191 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x404195 => {
  { let v: u32 = cpu.r32(0x5eaac0u32); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eaac0u32, v); }
  { let v: u32 = cpu.r32(0x5eab88u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eab88u32, v); }
  pc = 0x4041ba; continue;
 }
 0x4041ba => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404000); }
 let mut pc: u32 = 0x404000;
 loop { match pc {
 0x404000 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x8b14244cu32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x8b14244cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16527);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004041c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4041c0); }
 let mut pc: u32 = 0x4041c0;
 loop { match pc {
 0x4041c0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x40426c; continue; }
  pc = 0x4041d1; continue;
 }
 0x4041d1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40426c; continue; }
  pc = 0x4041dd; continue;
 }
 0x4041dd => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40426c; continue; }
  pc = 0x4041e9; continue;
 }
 0x4041e9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.push(0x404209); cpu.call(t); if cpu.returned_to != 0x404209 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40426c; continue; }
  pc = 0x40420d; continue;
 }
 0x40420d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40424a; continue; }
  pc = 0x40422d; continue;
 }
 0x40422d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x40424a; continue;
 }
 0x40424a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.push(0x404261); cpu.call(t); if cpu.returned_to != 0x404261 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40426c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404280); }
 let mut pc: u32 = 0x404280;
 loop { match pc {
 0x404280 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x404291); f_0040d370(cpu); if cpu.returned_to != 0x404291 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40429d; continue; }
  pc = 0x40429a; continue;
 }
 0x40429a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40429d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4042a3); f_0040d4a0(cpu); if cpu.returned_to != 0x4042a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea2a4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4042bd; continue; }
  pc = 0x4042af; continue;
 }
 0x4042af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4042b5); f_0040d640(cpu); if cpu.returned_to != 0x4042b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4042bd => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4042c3); f_00402400(cpu); if cpu.returned_to != 0x4042c3 { return; }
  { let v: u32 = cpu.r32(0x5ea2a4u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4042d8); f_0048d270(cpu); if cpu.returned_to != 0x4042d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4042e1); f_0048d190(cpu); if cpu.returned_to != 0x4042e1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004042f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4042f0); }
 let mut pc: u32 = 0x4042f0;
 loop { match pc {
 0x4042f0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x404343; continue; }
  pc = 0x4042fc; continue;
 }
 0x4042fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.push(0x40431c); cpu.call(t); if cpu.returned_to != 0x40431c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x404343; continue; }
  pc = 0x404320; continue;
 }
 0x404320 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.push(0x40433a); cpu.call(t); if cpu.returned_to != 0x40433a { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404343 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404350(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404350); } r_00404350(cpu, 0x404350); }
pub fn f_00404352(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404352); } r_00404350(cpu, 0x404352); }
fn r_00404350(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404350 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404382; continue; }
  pc = 0x40437e; continue;
 }
 0x404352 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x33u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rol8(((cpu.ebx >> 8) as u8), 68u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404382; continue; }
  pc = 0x40437e; continue;
 }
 0x40437e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404382 => {
  { let v: u32 = 0x5622u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x100e0u32; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x1au32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.push(0x4043e3); cpu.call(t); if cpu.returned_to != 0x4043e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x4043ed; continue; }
  pc = 0x4043e7; continue;
 }
 0x4043e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4043ed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404400); }
 let mut pc: u32 = 0x404400;
 loop { match pc {
 0x404400 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404417; continue; }
  pc = 0x404409; continue;
 }
 0x404409 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404417; continue; }
  pc = 0x404411; continue;
 }
 0x404411 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x404417); cpu.call(t); if cpu.returned_to != 0x404417 { return; }
  pc = 0x404417; continue;
 }
 0x404417 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404420(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404420); } r_00404420(cpu, 0x404420); }
pub fn f_00404441(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404441); } r_00404420(cpu, 0x404441); }
pub fn f_00404444(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404444); } r_00404420(cpu, 0x404444); }
pub fn f_0040444b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40444b); } r_00404420(cpu, 0x40444b); }
fn r_00404420(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404420 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404488; continue; }
  pc = 0x40442a; continue;
 }
 0x40442a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x404488; continue; }
  pc = 0x404432; continue;
 }
 0x404432 => {
  { let v: u32 = cpu.r32(0x5df880u32); cpu.eax = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x404440; continue; }
  pc = 0x40443c; continue;
 }
 0x40443c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404440 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  pc = 0x40444c; continue;
 }
 0x404441 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  pc = 0x40444c; continue;
 }
 0x404444 => {
  { let v: u8 = cpu.r8(0xc1d08b0eu32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xc1d08b0eu32.wrapping_add(cpu.ebx), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x404453; continue; }
  pc = 0x40444c; continue;
 }
 0x40444b => {
  { cpu.pop(); }
  pc = 0x40444c; continue;
 }
 0x40444c => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  pc = 0x404453; continue;
 }
 0x404453 => {
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.ecx)); cpu.push(0x40446e); cpu.call(t); if cpu.returned_to != 0x40446e { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.push(0x404476); cpu.call(t); if cpu.returned_to != 0x404476 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.push(0x404482); cpu.call(t); if cpu.returned_to != 0x404482 { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404488 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404490(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404490); } r_00404490(cpu, 0x404490); }
pub fn f_00404541(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404541); } r_00404490(cpu, 0x404541); }
pub fn f_0040454d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40454d); } r_00404490(cpu, 0x40454d); }
fn r_00404490(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404490 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x4044c4; continue; }
  pc = 0x4044bb; continue;
 }
 0x4044bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4044c4 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4044ce); f_00404280(cpu); if cpu.returned_to != 0x4044ce { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { pc = 0x4044e0; continue; }
  pc = 0x4044d7; continue;
 }
 0x4044d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4044e0 => {
  { let v: u32 = cpu.r32(0x5ea2a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x12u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x100e2u32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.push(0x404543); cpu.call(t); if cpu.returned_to != 0x404543 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x404568; continue; }
  pc = 0x404547; continue;
 }
 0x404541 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x85u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.ecx)), 68u32); cpu.w8(0xffffff8bu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) & 0x2cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x404557); f_004041c0(cpu); if cpu.returned_to != 0x404557 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40456c; continue; }
  pc = 0x40455e; continue;
 }
 0x404547 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x404557); f_004041c0(cpu); if cpu.returned_to != 0x404557 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40456c; continue; }
  pc = 0x40455e; continue;
 }
 0x40454d => {
  { let v: u8 = (cpu.eax as u8) & 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x404557); f_004041c0(cpu); if cpu.returned_to != 0x404557 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40456c; continue; }
  pc = 0x40455e; continue;
 }
 0x40455e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x404568); cpu.call(t); if cpu.returned_to != 0x404568 { return; }
  pc = 0x404568; continue;
 }
 0x404568 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40456c; continue;
 }
 0x40456c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x404572); f_00402420(cpu); if cpu.returned_to != 0x404572 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404580); }
 let mut pc: u32 = 0x404580;
 loop { match pc {
 0x404580 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ea2c0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404634; continue; }
  pc = 0x4045a2; continue;
 }
 0x4045a2 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x404634; continue; }
  pc = 0x4045aa; continue;
 }
 0x4045aa => {
  { let v: u32 = cpu.r32(0x5df880u32); cpu.eax = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4045ba; continue; }
  pc = 0x4045b4; continue;
 }
 0x4045b4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4045ba => {
  { let v: u32 = cpu.r32(0x1du32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.ecx)); cpu.push(0x4045e7); cpu.call(t); if cpu.returned_to != 0x4045e7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.push(0x4045ef); cpu.call(t); if cpu.returned_to != 0x4045ef { return; }
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x404603; continue; }
  pc = 0x4045f5; continue;
 }
 0x4045f5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.push(0x404601); cpu.call(t); if cpu.returned_to != 0x404601 { return; }
  pc = 0x40460f; continue;
 }
 0x404603 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.push(0x40460f); cpu.call(t); if cpu.returned_to != 0x40460f { return; }
  pc = 0x40460f; continue;
 }
 0x40460f => {
  { cpu.sub32(cpu.eax, 0x88780096u32); }
  if !cpu.zf { pc = 0x40462a; continue; }
  pc = 0x404616; continue;
 }
 0x404616 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x4aaef8u32; cpu.push(v); }
  cpu.push(0x404621); f_004030c0(cpu); if cpu.returned_to != 0x404621 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.push(0x40462a); cpu.call(t); if cpu.returned_to != 0x40462a { return; }
  pc = 0x40462a; continue;
 }
 0x40462a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404634; continue; }
  pc = 0x40462e; continue;
 }
 0x40462e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404634 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404640); }
 let mut pc: u32 = 0x404640;
 loop { match pc {
 0x404640 => {
  { let v: u32 = cpu.r8(0x5922ffu32) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5922fbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5df880u32, v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  { let v: u32 = 0x100u32; cpu.eax = v; }
  if cpu.zf { pc = 0x404661; continue; }
  pc = 0x40465b; continue;
 }
 0x40465b => {
  { let v: u32 = cpu.r32(0x5df880u32).wrapping_add(cpu.eax); cpu.w32(0x5df880u32, v); }
  pc = 0x404661; continue;
 }
 0x404661 => {
  { let v: u32 = cpu.r8(0x5922fdu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5df72cu32, v); }
  { let v: u8 = cpu.r8(0x5922f9u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x40467e; continue; }
  pc = 0x404678; continue;
 }
 0x404678 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5df72cu32), cpu.eax); cpu.w32(0x5df72cu32, v); }
  pc = 0x40467e; continue;
 }
 0x40467e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404680); }
 let mut pc: u32 = 0x404680;
 loop { match pc {
 0x404680 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40469e; continue; }
  pc = 0x404689; continue;
 }
 0x404689 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40469e; continue; }
  pc = 0x404698; continue;
 }
 0x404698 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.push(0x40469e); cpu.call(t); if cpu.returned_to != 0x40469e { return; }
  pc = 0x40469e; continue;
 }
 0x40469e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404710(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404710); } r_00404710(cpu, 0x404710); }
pub fn f_00404742(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404742); } r_00404710(cpu, 0x404742); }
pub fn f_00404747(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404747); } r_00404710(cpu, 0x404747); }
pub fn f_00404754(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404754); } r_00404710(cpu, 0x404754); }
fn r_00404710(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404710 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404767; continue; }
  pc = 0x404719; continue;
 }
 0x404719 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x404767; continue; }
  pc = 0x404728; continue;
 }
 0x404728 => {
  { let v: u32 = cpu.r32(0x5ea2c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df880u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1du32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi & 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.edx)); cpu.push(0x404765); cpu.call(t); if cpu.returned_to != 0x404765 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404767; continue;
 }
 0x404742 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.edx)); cpu.push(0x404765); cpu.call(t); if cpu.returned_to != 0x404765 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404767; continue;
 }
 0x404747 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x404749; continue; }
  pc = 0x404749; continue;
 }
 0x404749 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.edx)); cpu.push(0x404765); cpu.call(t); if cpu.returned_to != 0x404765 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404767; continue;
 }
 0x404754 => {
  { let v: u8 = cpu.r8(0xe0c18004u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0xe0c18004u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x3cu32.wrapping_add(cpu.edx)); cpu.push(0x404765); cpu.call(t); if cpu.returned_to != 0x404765 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404767; continue;
 }
 0x404767 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004047a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4047a0); }
 let mut pc: u32 = 0x4047a0;
 loop { match pc {
 0x4047a0 => {
  { let v: u32 = cpu.r32(0x5ddba8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4047c3; continue; }
  pc = 0x4047a9; continue;
 }
 0x4047a9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x5ea6c0u32; cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddba8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4047c3; continue;
 }
 0x4047c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004047d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4047d0); }
 let mut pc: u32 = 0x4047d0;
 loop { match pc {
 0x4047d0 => {
  cpu.push(0x4047d5); f_004047a0(cpu); if cpu.returned_to != 0x4047d5 { return; }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404802; continue; }
  pc = 0x4047de; continue;
 }
 0x4047de => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5ea6c0u32; cpu.esi = v; }
  pc = 0x4047e4; continue;
 }
 0x4047e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4047f6; continue; }
  pc = 0x4047ea; continue;
 }
 0x4047ea => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x4047f0); cpu.call(t); if cpu.returned_to != 0x4047f0 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x4047f6; continue;
 }
 0x4047f6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x5eaac0u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4047e4; continue; }
  pc = 0x404801; continue;
 }
 0x404801 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x404802; continue;
 }
 0x404802 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404810); }
 let mut pc: u32 = 0x404810;
 loop { match pc {
 0x404810 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4a47e4u32, v); }
  cpu.push(0x404821); f_004047a0(cpu); if cpu.returned_to != 0x404821 { return; }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404883; continue; }
  pc = 0x40482a; continue;
 }
 0x40482a => {
  { let v: u32 = cpu.r32(0x4aaee0u32.wrapping_add(cpu.edi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5ea2a0u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x404883; continue; }
  pc = 0x40483c; continue;
 }
 0x40483c => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x40483f; continue;
 }
 0x40483f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x404845); f_00404490(cpu); if cpu.returned_to != 0x404845 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea6c0u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea2c0u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x404875; continue; }
  pc = 0x404867; continue;
 }
 0x404867 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4aaf10u32; cpu.push(v); }
  cpu.push(0x404872); f_004030c0(cpu); if cpu.returned_to != 0x404872 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x404875; continue;
 }
 0x404875 => {
  { let v: u32 = cpu.r32(0x21u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x21u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x21u32); cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x40483f; continue; }
  pc = 0x404883; continue;
 }
 0x404883 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404890); }
 let mut pc: u32 = 0x404890;
 loop { match pc {
 0x404890 => {
  { let v: u32 = cpu.r32(0x4a47e4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4048a3; continue; }
  pc = 0x40489a; continue;
 }
 0x40489a => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4048a0); f_00404810(cpu); if cpu.returned_to != 0x4048a0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4048a3; continue;
 }
 0x4048a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004048b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4048b0); }
 let mut pc: u32 = 0x4048b0;
 loop { match pc {
 0x4048b0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x0u32); cpu.eax = v; }
  if cpu.zf { pc = 0x4048c9; continue; }
  pc = 0x4048ba; continue;
 }
 0x4048ba => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x4048da; continue; }
  pc = 0x4048bd; continue;
 }
 0x4048bd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xbbu32.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x4048de; continue;
 }
 0x4048c9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  { let v: u32 = 0x7fu32.wrapping_add(cpu.eax); cpu.esi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4048de; continue; }
  pc = 0x4048d5; continue;
 }
 0x4048d5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  pc = 0x4048de; continue;
 }
 0x4048da => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x4048de; continue;
 }
 0x4048de => {
  { cpu.sub32(cpu.esi, 0x100u32); }
  if (cpu.sf != cpu.of) { pc = 0x4048eb; continue; }
  pc = 0x4048e6; continue;
 }
 0x4048e6 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4048eb => {
  { let v: u32 = cpu.r32(0x5ea6c0u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4048fb; continue; }
  pc = 0x4048f6; continue;
 }
 0x4048f6 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4048fb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x404901); f_00404580(cpu); if cpu.returned_to != 0x404901 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404910); }
 let mut pc: u32 = 0x404910;
 loop { match pc {
 0x404910 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x400u32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x100u32; cpu.esi = v; }
  pc = 0x404927; continue;
 }
 0x404927 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffdu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x404927; continue; }
  pc = 0x404953; continue;
 }
 0x404953 => {
  { let v: u32 = cpu.r32(0x5de974u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x40496e; continue; }
  pc = 0x40495c; continue;
 }
 0x40495c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x40496e); cpu.call(t); if cpu.returned_to != 0x40496e { return; }
  pc = 0x40496e; continue;
 }
 0x40496e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x400u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404980); }
 let mut pc: u32 = 0x404980;
 loop { match pc {
 0x404980 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5de974u32); cpu.eax = v; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xeu32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x4049c3; continue; }
  pc = 0x4049af; continue;
 }
 0x4049af => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x4049c3); cpu.call(t); if cpu.returned_to != 0x4049c3 { return; }
  pc = 0x4049c3; continue;
 }
 0x4049c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004049d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4049d0); }
 let mut pc: u32 = 0x4049d0;
 loop { match pc {
 0x4049d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x400u32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x100u32; cpu.esi = v; }
  pc = 0x4049e7; continue;
 }
 0x4049e7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffbu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffdu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4049e7; continue; }
  pc = 0x404a0b; continue;
 }
 0x404a0b => {
  { let v: u32 = cpu.r32(0x5de974u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x404a26; continue; }
  pc = 0x404a14; continue;
 }
 0x404a14 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x404a26); cpu.call(t); if cpu.returned_to != 0x404a26 { return; }
  pc = 0x404a26; continue;
 }
 0x404a26 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x400u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404a30); }
 let mut pc: u32 = 0x404a30;
 loop { match pc {
 0x404a30 => {
  { let v: u32 = cpu.r32(0x4aaf2cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6ba8u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ba0u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x404a51; continue; }
  pc = 0x404a49; continue;
 }
 0x404a49 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x404a51; continue;
 }
 0x404a51 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5e2468u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x404a7a; continue; }
  pc = 0x404a6a; continue;
 }
 0x404a6a => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x404a7a; continue; }
  pc = 0x404a73; continue;
 }
 0x404a73 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404a7a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x404a96; continue; }
  pc = 0x404a86; continue;
 }
 0x404a86 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x404a96; continue; }
  pc = 0x404a8f; continue;
 }
 0x404a8f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404a96 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5e2460u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x404abc; continue; }
  pc = 0x404aa2; continue;
 }
 0x404aa2 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5e2460u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.eax))); }
  if (cpu.sf != cpu.of) { pc = 0x404abc; continue; }
  pc = 0x404ab5; continue;
 }
 0x404ab5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404abc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x404ade; continue; }
  pc = 0x404ac3; continue;
 }
 0x404ac3 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x404ade; continue; }
  pc = 0x404ad7; continue;
 }
 0x404ad7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404ade => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404af0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404af0); } r_00404af0(cpu, 0x404af0); }
pub fn f_00404b4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404b4b); } r_00404af0(cpu, 0x404b4b); }
pub fn f_00404b4f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404b4f); } r_00404af0(cpu, 0x404b4f); }
fn r_00404af0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404af0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5e2468u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x404b19; continue; }
  pc = 0x404b0b; continue;
 }
 0x404b0b => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x404b19; continue; }
  pc = 0x404b14; continue;
 }
 0x404b14 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404b19 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1e0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x404b3c; continue; }
  pc = 0x404b29; continue;
 }
 0x404b29 => {
  { let v: u32 = 0x1e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x404b3c; continue; }
  pc = 0x404b37; continue;
 }
 0x404b37 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404b3c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5e2460u32); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x404b60; continue; }
  pc = 0x404b48; continue;
 }
 0x404b48 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5e2460u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.eax))); }
  if (cpu.sf != cpu.of) { pc = 0x404b60; continue; }
  pc = 0x404b5b; continue;
 }
 0x404b4b => {
  { let v: u8 = cpu.r8(0x5e24600du32.wrapping_add(cpu.ebx)) | (cpu.ecx as u8); cpu.w8(0x5e24600du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x3b088b0au32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x3b088b0au32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))), ((cpu.ebx >> 8) as u8)); cpu.w8(0x33u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.rcr8(cpu.r8(0x5eu32.wrapping_add(cpu.edi)), 195u32); cpu.w8(0x5eu32.wrapping_add(cpu.edi), v); }
  pc = 0x404b60; continue;
 }
 0x404b4f => {
  { let v: u8 = (cpu.eax as u8) & 0x5eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3b088b0au32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x3b088b0au32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))), ((cpu.ebx >> 8) as u8)); cpu.w8(0x33u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.rcr8(cpu.r8(0x5eu32.wrapping_add(cpu.edi)), 195u32); cpu.w8(0x5eu32.wrapping_add(cpu.edi), v); }
  pc = 0x404b60; continue;
 }
 0x404b5b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404b60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x404b82; continue; }
  pc = 0x404b67; continue;
 }
 0x404b67 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x404b82; continue; }
  pc = 0x404b7d; continue;
 }
 0x404b7d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x404b82 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404b90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404b90); }
 let mut pc: u32 = 0x404b90;
 loop { match pc {
 0x404b90 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df700u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x404bc9); cpu.call(t); if cpu.returned_to != 0x404bc9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404c30); }
 let mut pc: u32 = 0x404c30;
 loop { match pc {
 0x404c30 => {
  { let v: u32 = cpu.r32(0x5ea260u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404c50; continue; }
  pc = 0x404c39; continue;
 }
 0x404c39 => {
  { let v = 0x10000u32; cpu.push(v); }
  cpu.push(0x404c43); f_00402400(cpu); if cpu.returned_to != 0x404c43 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea260u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea264u32, v); }
  pc = 0x404c50; continue;
 }
 0x404c50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404c44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404c44); } r_00404c44(cpu, 0x404c44); }
pub fn f_00404c54(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404c54); } r_00404c44(cpu, 0x404c54); }
fn r_00404c44(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404c44 => {
  { let at = 0x0u32.wrapping_add(cpu.ebx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  { cpu.pushad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x64a3005eu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x90c3005eu32, v); }
  {  }
  {  }
  pc = 0x404c54; continue;
 }
 0x404c54 => {
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  f_00404c60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404c60); }
 let mut pc: u32 = 0x404c60;
 loop { match pc {
 0x404c60 => {
  { let v: u32 = cpu.r32(0x5ddbacu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x404cab; continue; }
  pc = 0x404c6c; continue;
 }
 0x404c6c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbacu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbccu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbbcu32, v); }
  cpu.push(0x404c86); f_004056e0(cpu); if cpu.returned_to != 0x404c86 { return; }
  cpu.push(0x404c8b); f_00404c30(cpu); if cpu.returned_to != 0x404c8b { return; }
  { let v: u32 = 0x1d85u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x5e2480u32; cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1e0u32; cpu.ecx = v; }
  { let v: u32 = 0x5e9aa0u32; cpu.edi = v; }
  { cpu.stos(4, true); }
  cpu.push(0x404caa); f_00405330(cpu); if cpu.returned_to != 0x404caa { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x404cab; continue;
 }
 0x404cab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x404cb0); }
 let mut pc: u32 = 0x404cb0;
 loop { match pc {
 0x404cb0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x404cfc); f_00404a30(cpu); if cpu.returned_to != 0x404cfc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404d26; continue; }
  pc = 0x404d03; continue;
 }
 0x404d03 => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404d26; continue; }
  pc = 0x404d0c; continue;
 }
 0x404d0c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x11u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.push(0x404d26); cpu.call(t); if cpu.returned_to != 0x404d26 { return; }
  pc = 0x404d26; continue;
 }
 0x404d26 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404d30(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404d30); } r_00404d30(cpu, 0x404d30); }
pub fn f_00404d40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404d40); } r_00404d30(cpu, 0x404d40); }
fn r_00404d30(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404d30 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x404d40; continue;
 }
 0x404d40 => {
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x404d7c); f_00404a30(cpu); if cpu.returned_to != 0x404d7c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404db7; continue; }
  pc = 0x404d83; continue;
 }
 0x404d83 => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404db7; continue; }
  pc = 0x404d8c; continue;
 }
 0x404d8c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.push(0x404da6); cpu.call(t); if cpu.returned_to != 0x404da6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x404db7; continue; }
  pc = 0x404daa; continue;
 }
 0x404daa => {
  { let v = 0x4aaf34u32; cpu.push(v); }
  cpu.push(0x404db4); f_004030c0(cpu); if cpu.returned_to != 0x404db4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x404db7; continue;
 }
 0x404db7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404dc0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404dc0); } r_00404dc0(cpu, 0x404dc0); }
pub fn f_00404e43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404e43); } r_00404dc0(cpu, 0x404e43); }
fn r_00404dc0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404dc0 => {
  { let v: u32 = cpu.r32(0x5ddbc8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404e70; continue; }
  pc = 0x404dcd; continue;
 }
 0x404dcd => {
  { let v: u32 = cpu.r32(0x5ea280u32); cpu.eax = v; }
  { let v: u32 = 0x5ea280u32; cpu.w32(0x5ea270u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x5ea240u32; cpu.w32(0x5ea274u32, v); }
  if !cpu.zf { pc = 0x404e02; continue; }
  pc = 0x404dea; continue;
 }
 0x404dea => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x404df7); f_00402310(cpu); if cpu.returned_to != 0x404df7 { return; }
  { let v: u32 = cpu.r32(0x5ea270u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x404e02; continue;
 }
 0x404e02 => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x404e25; continue; }
  pc = 0x404e0d; continue;
 }
 0x404e0d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x404e1a); f_00402310(cpu); if cpu.returned_to != 0x404e1a { return; }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x404e25; continue;
 }
 0x404e25 => {
  { let v: u32 = cpu.r32(0x5ea278u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404e43; continue; }
  pc = 0x404e2e; continue;
 }
 0x404e2e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x404e3b); f_00402310(cpu); if cpu.returned_to != 0x404e3b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea278u32, v); }
  pc = 0x404e43; continue;
 }
 0x404e43 => {
  { let v: u32 = cpu.r32(0x5ea270u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4aaf30u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbc8u32, v); }
  pc = 0x404e70; continue;
 }
 0x404e70 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00404e80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404e80); } r_00404e80(cpu, 0x404e80); }
pub fn f_00404f40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404f40); } r_00404e80(cpu, 0x404f40); }
pub fn f_00404f4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x404f4e); } r_00404e80(cpu, 0x404f4e); }
fn r_00404e80(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x404e80 => {
  { let v: u32 = cpu.r32(0x5e2474u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x404e9a; continue; }
  pc = 0x404e90; continue;
 }
 0x404e90 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4aaf30u32, v); }
  pc = 0x404e9a; continue;
 }
 0x404e9a => {
  { let v: u32 = cpu.r32(0x5ea278u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x404fa5; continue; }
  pc = 0x404ea8; continue;
 }
 0x404ea8 => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4aaf30u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if cpu.zf { pc = 0x404fa5; continue; }
  pc = 0x404ebd; continue;
 }
 0x404ebd => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x404fa5; continue; }
  pc = 0x404ec6; continue;
 }
 0x404ec6 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4aaf30u32, v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x404efc); cpu.call(t); if cpu.returned_to != 0x404efc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x404fa5; continue; }
  pc = 0x404f04; continue;
 }
 0x404f04 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x404f10; continue;
 }
 0x404f10 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x404f12; continue;
 }
 0x404f12 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x404f69; continue; }
  pc = 0x404f20; continue;
 }
 0x404f20 => {
  { cpu.sub8((cpu.ecx as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x404f2c; continue; }
  pc = 0x404f25; continue;
 }
 0x404f25 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x404f2c; continue; }
  pc = 0x404f2a; continue;
 }
 0x404f2a => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ebx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x404f2c; continue;
 }
 0x404f2c => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x404f40; continue; }
  pc = 0x404f39; continue;
 }
 0x404f39 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x404f40; continue; }
  pc = 0x404f3e; continue;
 }
 0x404f3e => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ebx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x404f40; continue;
 }
 0x404f40 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x404f54; continue; }
  pc = 0x404f4d; continue;
 }
 0x404f4d => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x404f54; continue; }
  pc = 0x404f52; continue;
 }
 0x404f4e => {
  { cpu.cf = true; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)) & ((cpu.ebx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x404f52; continue;
 }
 0x404f52 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ebx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x404f54; continue;
 }
 0x404f54 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x404f80; continue; }
  pc = 0x404f60; continue;
 }
 0x404f60 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x404f80; continue; }
  pc = 0x404f65; continue;
 }
 0x404f65 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ebx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x404f80; continue;
 }
 0x404f69 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x404f80; continue;
 }
 0x404f80 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.ebp, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x404f12; continue; }
  pc = 0x404f8a; continue;
 }
 0x404f8a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x404f10; continue; }
  pc = 0x404f95; continue;
 }
 0x404f95 => {
  { let v: u32 = cpu.r32(0x5ea278u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x404fa5); cpu.call(t); if cpu.returned_to != 0x404fa5 { return; }
  pc = 0x404fa5; continue;
 }
 0x404fa5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405030(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405030); } r_00405030(cpu, 0x405030); }
pub fn f_00405040(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405040); } r_00405030(cpu, 0x405040); }
pub fn f_00405055(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405055); } r_00405030(cpu, 0x405055); }
fn r_00405030(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x405030 => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x405042; continue;
 }
 0x405040 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x405042; continue;
 }
 0x405042 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x40504b; continue;
 }
 0x40504b => {
  { let v: u32 = cpu.r32(0x5ddbc0u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x405082; continue; }
  pc = 0x405055; continue;
 }
 0x405055 => {
  { let v: u32 = cpu.r16(0x4d6752u32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6754u32) as i16 as i32 as u32; cpu.edx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x405075); f_00402b00(cpu); if cpu.returned_to != 0x405075 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x405080; continue;
 }
 0x405080 => {
  pc = 0x4050af; continue;
 }
 0x405082 => {
  { let v: u32 = cpu.r32(0x5ea268u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ea26cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddbc0u32, v); }
  pc = 0x4050af; continue;
 }
 0x4050af => {
  { let v: u32 = cpu.r32(0x4aaf2cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4050e3; continue; }
  pc = 0x4050b8; continue;
 }
 0x4050b8 => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6752u32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x4d6754u32) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x28u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x4050e3; continue;
 }
 0x4050e3 => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x405122); f_00404af0(cpu); if cpu.returned_to != 0x405122 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x40512d; continue;
 }
 0x40512d => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4051c4; continue; }
  pc = 0x405191; continue;
 }
 0x405191 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.push(0x4051a8); cpu.call(t); if cpu.returned_to != 0x4051a8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4051bf; continue; }
  pc = 0x4051ac; continue;
 }
 0x4051ac => {
  { let v = 0x4aaf60u32; cpu.push(v); }
  cpu.push(0x4051b6); f_004030c0(cpu); if cpu.returned_to != 0x4051b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4051bf => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  pc = 0x4051c4; continue;
 }
 0x4051c4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x4051d4; continue;
 }
 0x4051d4 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x11u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ea278u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x4051f6); cpu.call(t); if cpu.returned_to != 0x4051f6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405207; continue; }
  pc = 0x4051fa; continue;
 }
 0x4051fa => {
  { let v = 0x4aaf50u32; cpu.push(v); }
  cpu.push(0x405204); f_004030c0(cpu); if cpu.returned_to != 0x405204 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x405207; continue;
 }
 0x405207 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405210); }
 let mut pc: u32 = 0x405210;
 loop { match pc {
 0x405210 => {
  cpu.push(0x405215); f_00405030(cpu); if cpu.returned_to != 0x405215 { return; }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ea270u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ea274u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea270u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405230); }
 let mut pc: u32 = 0x405230;
 loop { match pc {
 0x405230 => {
  cpu.push(0x405235); f_00403080(cpu); if cpu.returned_to != 0x405235 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405248; continue; }
  pc = 0x405239; continue;
 }
 0x405239 => {
  cpu.push(0x40523e); f_004030a0(cpu); if cpu.returned_to != 0x40523e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405248; continue; }
  pc = 0x405242; continue;
 }
 0x405242 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405248 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405250); }
 let mut pc: u32 = 0x405250;
 loop { match pc {
 0x405250 => {
  { let v: u32 = cpu.r32(0x5ddbccu32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  if cpu.zf { pc = 0x405285; continue; }
  pc = 0x40525f; continue;
 }
 0x40525f => {
  cpu.push(0x405264); f_00411e40(cpu); if cpu.returned_to != 0x405264 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405271; continue; }
  pc = 0x405268; continue;
 }
 0x405268 => {
  cpu.push(0x40526d); f_00405230(cpu); if cpu.returned_to != 0x40526d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40527c; continue; }
  pc = 0x405271; continue;
 }
 0x405271 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5ddbc4u32, v); }
  cpu.push(0x40527c); f_0041ac57(cpu); if cpu.returned_to != 0x40527c { return; }
  pc = 0x40527c; continue;
 }
 0x40527c => {
  { let v: u32 = cpu.r32(0x5ddbccu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40525f; continue; }
  pc = 0x405285; continue;
 }
 0x405285 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4052a0; continue; }
  pc = 0x40528d; continue;
 }
 0x40528d => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4052a0; continue; }
  pc = 0x405296; continue;
 }
 0x405296 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x58u32.wrapping_add(cpu.ecx)); cpu.push(0x4052a0); cpu.call(t); if cpu.returned_to != 0x4052a0 { return; }
  pc = 0x4052a0; continue;
 }
 0x4052a0 => {
  cpu.push(0x4052a5); f_00402d10(cpu); if cpu.returned_to != 0x4052a5 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x4052ac); f_00402b00(cpu); if cpu.returned_to != 0x4052ac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x4052b4); f_00411e40(cpu); if cpu.returned_to != 0x4052b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4052c1; continue; }
  pc = 0x4052b8; continue;
 }
 0x4052b8 => {
  cpu.push(0x4052bd); f_00405230(cpu); if cpu.returned_to != 0x4052bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4052cc; continue; }
  pc = 0x4052c1; continue;
 }
 0x4052c1 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5ddbc4u32, v); }
  cpu.push(0x4052cc); f_0041ac57(cpu); if cpu.returned_to != 0x4052cc { return; }
  pc = 0x4052cc; continue;
 }
 0x4052cc => {
  cpu.push(0x4052d1); f_00416f5d(cpu); if cpu.returned_to != 0x4052d1 { return; }
  cpu.push(0x4052d6); f_0040d660(cpu); if cpu.returned_to != 0x4052d6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004052e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4052e0); }
 let mut pc: u32 = 0x4052e0;
 loop { match pc {
 0x404fb0 => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x405024; continue; }
  pc = 0x404fbd; continue;
 }
 0x404fbd => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x405024; continue; }
  pc = 0x404fc2; continue;
 }
 0x404fc2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x405024; continue; }
  pc = 0x404fc9; continue;
 }
 0x404fc9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x405024; continue; }
  pc = 0x404ffb; continue;
 }
 0x404ffb => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x405013); cpu.call(t); if cpu.returned_to != 0x405013 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405024; continue; }
  pc = 0x405017; continue;
 }
 0x405017 => {
  { let v = 0x4aaf40u32; cpu.push(v); }
  cpu.push(0x405021); f_004030c0(cpu); if cpu.returned_to != 0x405021 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x405024; continue;
 }
 0x405024 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4052e0 => {
  cpu.push(0x4052e5); f_00416f5d(cpu); if cpu.returned_to != 0x4052e5 { return; }
  cpu.push(0x4052ea); f_0040d660(cpu); if cpu.returned_to != 0x4052ea { return; }
  cpu.push(0x4052ef); f_00405210(cpu); if cpu.returned_to != 0x4052ef { return; }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x58u32.wrapping_add(cpu.ecx)); cpu.push(0x4052fe); cpu.call(t); if cpu.returned_to != 0x4052fe { return; }
  { let v: u32 = cpu.r32(0x5df700u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405311; continue; }
  pc = 0x405307; continue;
 }
 0x405307 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.push(0x405311); cpu.call(t); if cpu.returned_to != 0x405311 { return; }
  pc = 0x405311; continue;
 }
 0x405311 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x405318); f_00405250(cpu); if cpu.returned_to != 0x405318 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.poll(); pc = 0x404fb0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405320); }
 let mut pc: u32 = 0x405320;
 loop { match pc {
 0x405320 => {
  cpu.push(0x405325); f_004052e0(cpu); if cpu.returned_to != 0x405325 { return; }
  f_004052e0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405330); }
 let mut pc: u32 = 0x405330;
 loop { match pc {
 0x404bd0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5e2460u32, v); }
  { let v: u32 = 0x280u32; cpu.w32(0x5e2468u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x5e2464u32, v); }
  { let v: u32 = 0x1b8u32; cpu.w32(0x5e246cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405330 => {
  { let v: u32 = cpu.r32(0x4aaf2cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40535c; continue; }
  pc = 0x405339; continue;
 }
 0x405339 => {
  cpu.push(0x40533e); f_00411de0(cpu); if cpu.returned_to != 0x40533e { return; }
  cpu.push(0x405343); f_00401ff0(cpu); if cpu.returned_to != 0x405343 { return; }
  cpu.push(0x405348); f_00405440(cpu); if cpu.returned_to != 0x405348 { return; }
  cpu.push(0x40534d); f_00405390(cpu); if cpu.returned_to != 0x40534d { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4aaf2cu32, v); }
  cpu.poll(); pc = 0x404bd0; continue;
 }
 0x40535c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405360); }
 let mut pc: u32 = 0x405360;
 loop { match pc {
 0x404c00 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x280u32; cpu.w32(0x5e2468u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2460u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2464u32, v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x5e246cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405360 => {
  { cpu.sub32(cpu.r32(0x4aaf2cu32), 0x1u32); }
  if cpu.zf { pc = 0x40538c; continue; }
  pc = 0x405369; continue;
 }
 0x405369 => {
  cpu.push(0x40536e); f_00411de0(cpu); if cpu.returned_to != 0x40536e { return; }
  cpu.push(0x405373); f_00401ff0(cpu); if cpu.returned_to != 0x405373 { return; }
  cpu.push(0x405378); f_00405440(cpu); if cpu.returned_to != 0x405378 { return; }
  cpu.push(0x40537d); f_00405390(cpu); if cpu.returned_to != 0x40537d { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x4aaf2cu32, v); }
  cpu.poll(); pc = 0x404c00; continue;
 }
 0x40538c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405390); }
 let mut pc: u32 = 0x405390;
 loop { match pc {
 0x405390 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5e2480u32; cpu.edi = v; }
  pc = 0x405397; continue;
 }
 0x405397 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x4053b4; continue; }
  pc = 0x40539d; continue;
 }
 0x40539d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x4053a5); cpu.call(t); if cpu.returned_to != 0x4053a5 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4053ab); f_00402420(cpu); if cpu.returned_to != 0x4053ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x4053b4; continue;
 }
 0x4053b4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x5e9a90u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405397; continue; }
  pc = 0x4053bf; continue;
 }
 0x4053bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004053d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4053d0); }
 let mut pc: u32 = 0x4053d0;
 loop { match pc {
 0x4053d0 => {
  { let v: u32 = cpu.r32(0x5ddbc8u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x405435; continue; }
  pc = 0x4053dc; continue;
 }
 0x4053dc => {
  { let v: u32 = cpu.r32(0x5ea270u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4053fb; continue; }
  pc = 0x4053e7; continue;
 }
 0x4053e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x4053ed); cpu.call(t); if cpu.returned_to != 0x4053ed { return; }
  { let v: u32 = cpu.r32(0x5ea270u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea270u32, v); }
  pc = 0x4053fb; continue;
 }
 0x4053fb => {
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40541a; continue; }
  pc = 0x405406; continue;
 }
 0x405406 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x40540c); cpu.call(t); if cpu.returned_to != 0x40540c { return; }
  { let v: u32 = cpu.r32(0x5ea274u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea274u32, v); }
  pc = 0x40541a; continue;
 }
 0x40541a => {
  { let v: u32 = cpu.r32(0x5ea278u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40542f; continue; }
  pc = 0x405423; continue;
 }
 0x405423 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x405429); cpu.call(t); if cpu.returned_to != 0x405429 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea278u32, v); }
  pc = 0x40542f; continue;
 }
 0x40542f => {
  { let v: u32 = cpu.esi; cpu.w32(0x5ddbc8u32, v); }
  pc = 0x405435; continue;
 }
 0x405435 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405440(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405440); } r_00405440(cpu, 0x405440); }
pub fn f_00405449(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405449); } r_00405440(cpu, 0x405449); }
pub fn f_0040544c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40544c); } r_00405440(cpu, 0x40544c); }
pub fn f_0040544e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40544e); } r_00405440(cpu, 0x40544e); }
fn r_00405440(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x405440 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5e9aa0u32; cpu.edi = v; }
  pc = 0x405447; continue;
 }
 0x405447 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  pc = 0x405449; continue;
 }
 0x405449 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x405468; continue; }
  pc = 0x40544d; continue;
 }
 0x40544c => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.r32(0x74c08506u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { cpu.push(0x2b); }
  pc = 0x405453; continue;
 }
 0x40544d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405459; continue; }
  pc = 0x405453; continue;
 }
 0x40544e => {
  { cpu.push(0x2b); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405459; continue; }
  pc = 0x405453; continue;
 }
 0x405453 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x405459); cpu.call(t); if cpu.returned_to != 0x405459 { return; }
  pc = 0x405459; continue;
 }
 0x405459 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40545f); f_00402420(cpu); if cpu.returned_to != 0x40545f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x405468; continue;
 }
 0x405468 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x5ea220u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405447; continue; }
  pc = 0x405473; continue;
 }
 0x405473 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405480(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405480); } r_00405480(cpu, 0x405480); }
pub fn f_00405551(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405551); } r_00405480(cpu, 0x405551); }
fn r_00405480(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x405480 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x40549d; continue;
 }
 0x40549d => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5e9aa0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x4054bf; continue;
 }
 0x4054bf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405525; continue; }
  pc = 0x4054c9; continue;
 }
 0x4054c9 => {
  { let v: u32 = 0x10u32; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4054d4); f_00402400(cpu); if cpu.returned_to != 0x4054d4 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x4056b1; continue; }
  pc = 0x4054e1; continue;
 }
 0x4054e1 => {
  { let v: u32 = cpu.r32(0x4aaf2cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v = 0x0u32; cpu.push(v); }
  if !cpu.zf { pc = 0x40550b; continue; }
  pc = 0x4054f0; continue;
 }
 0x4054f0 => {
  { let v: u32 = 0x20u32; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4054fc); f_00402310(cpu); if cpu.returned_to != 0x4054fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40566e; continue; }
  pc = 0x405509; continue;
 }
 0x405509 => {
  pc = 0x40551f; continue;
 }
 0x40550b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x405512); f_00402310(cpu); if cpu.returned_to != 0x405512 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405696; continue; }
  pc = 0x40551f; continue;
 }
 0x40551f => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x405525; continue;
 }
 0x405525 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = 0x6cu32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x405547); cpu.call(t); if cpu.returned_to != 0x405547 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40560b; continue; }
  pc = 0x40554f; continue;
 }
 0x40554f => {
  { let v: u32 = cpu.r32(0x4aaf2cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4055d7; continue; }
  pc = 0x405558; continue;
 }
 0x405551 => {
  { cpu.scas(4, 0); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x8b7f75c0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0x8b7f75c0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) & 0x54u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x40556f; continue;
 }
 0x405558 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x40556f; continue;
 }
 0x40556f => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ebp = v; }
  pc = 0x40557d; continue;
 }
 0x40557d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40557d; continue; }
  pc = 0x40559c; continue;
 }
 0x40559c => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x130u32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40556f; continue; }
  pc = 0x4055c0; continue;
 }
 0x4055c0 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x4055d1); cpu.call(t); if cpu.returned_to != 0x4055d1 { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x40560b; continue;
 }
 0x4055d7 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x4055e0; continue;
 }
 0x4055e0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4055e2; continue;
 }
 0x4055e2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4055e2; continue; }
  pc = 0x4055ee; continue;
 }
 0x4055ee => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x130u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4055e0; continue; }
  pc = 0x4055fd; continue;
 }
 0x4055fd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x40560b); cpu.call(t); if cpu.returned_to != 0x40560b { return; }
  pc = 0x40560b; continue;
 }
 0x40560b => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x14u32); }
  { let v: u32 = cpu.ebp; cpu.w32(0xa4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4054bf; continue; }
  pc = 0x40563e; continue;
 }
 0x40563e => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1400u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xcu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40549d; continue; }
  pc = 0x40565e; continue;
 }
 0x40565e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40566e => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x405674); f_00402420(cpu); if cpu.returned_to != 0x405674 { return; }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9aa0u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405696 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40569c); f_00402420(cpu); if cpu.returned_to != 0x40569c { return; }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9aa0u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x4056b1; continue;
 }
 0x4056b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004056c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4056c0); }
 let mut pc: u32 = 0x4056c0;
 loop { match pc {
 0x4056c0 => {
  cpu.push(0x4056c5); f_0041c00e(cpu); if cpu.returned_to != 0x4056c5 { return; }
  { let v: u32 = cpu.r32(0x5ea27cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea27cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2470u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004056e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4056e0); }
 let mut pc: u32 = 0x4056e0;
 loop { match pc {
 0x4056e0 => {
  { let v: u32 = cpu.r32(0x5ddbb8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40570b; continue; }
  pc = 0x4056e9; continue;
 }
 0x4056e9 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4056c0u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  let t = cpu.r32(0x4a1284u32); cpu.push(0x4056fc); cpu.call(t); if cpu.returned_to != 0x4056fc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbb4u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbb8u32, v); }
  pc = 0x40570b; continue;
 }
 0x40570b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405710); }
 let mut pc: u32 = 0x405710;
 loop { match pc {
 0x405710 => {
  { let v: u32 = cpu.r32(0x5ddbb8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405725; continue; }
  pc = 0x405719; continue;
 }
 0x405719 => {
  { let v: u32 = cpu.r32(0x5ddbb4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1280u32); cpu.push(0x405725); cpu.call(t); if cpu.returned_to != 0x405725 { return; }
  pc = 0x405725; continue;
 }
 0x405725 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5ddbb8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405730); }
 let mut pc: u32 = 0x405730;
 loop { match pc {
 0x405730 => {
  cpu.push(0x405735); f_00406fd0(cpu); if cpu.returned_to != 0x405735 { return; }
  cpu.push(0x40573a); f_00402880(cpu); if cpu.returned_to != 0x40573a { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x405741); f_00401f80(cpu); if cpu.returned_to != 0x405741 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405750); }
 let mut pc: u32 = 0x405750;
 loop { match pc {
 0x405750 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4aaf90u32; cpu.push(v); }
  cpu.push(0x40575b); f_004030c0(cpu); if cpu.returned_to != 0x40575b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x405763); f_00401ff0(cpu); if cpu.returned_to != 0x405763 { return; }
  cpu.push(0x405768); f_00411de0(cpu); if cpu.returned_to != 0x405768 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbbcu32, v); }
  cpu.push(0x405777); f_00405710(cpu); if cpu.returned_to != 0x405777 { return; }
  cpu.push(0x40577c); f_004047d0(cpu); if cpu.returned_to != 0x40577c { return; }
  cpu.push(0x405781); f_00405440(cpu); if cpu.returned_to != 0x405781 { return; }
  cpu.push(0x405786); f_00405390(cpu); if cpu.returned_to != 0x405786 { return; }
  cpu.push(0x40578b); f_004053d0(cpu); if cpu.returned_to != 0x40578b { return; }
  cpu.push(0x405790); f_004029e0(cpu); if cpu.returned_to != 0x405790 { return; }
  cpu.push(0x405795); f_00402c80(cpu); if cpu.returned_to != 0x405795 { return; }
  { let v: u32 = cpu.r32(0x5de974u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4057ac; continue; }
  pc = 0x4057a0; continue;
 }
 0x4057a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x4057a6); cpu.call(t); if cpu.returned_to != 0x4057a6 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5de974u32, v); }
  pc = 0x4057ac; continue;
 }
 0x4057ac => {
  { let v = 0x4aaf7cu32; cpu.push(v); }
  cpu.push(0x4057b6); f_004030c0(cpu); if cpu.returned_to != 0x4057b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x4057be); f_00405730(cpu); if cpu.returned_to != 0x4057be { return; }
  { let v: u32 = cpu.r32(0x5ea260u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4057dc; continue; }
  pc = 0x4057c7; continue;
 }
 0x4057c7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4057cd); f_00402420(cpu); if cpu.returned_to != 0x4057cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea264u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea260u32, v); }
  pc = 0x4057dc; continue;
 }
 0x4057dc => {
  { let v: u32 = cpu.r32(0x5ea220u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4057f4; continue; }
  pc = 0x4057e5; continue;
 }
 0x4057e5 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4057eb); f_00402420(cpu); if cpu.returned_to != 0x4057eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea220u32, v); }
  pc = 0x4057f4; continue;
 }
 0x4057f4 => {
  { let v = 0x4aaf70u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddbacu32, v); }
  cpu.push(0x405804); f_004030c0(cpu); if cpu.returned_to != 0x405804 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405810); }
 let mut pc: u32 = 0x405810;
 loop { match pc {
 0x405810 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4aafdcu32; cpu.push(v); }
  cpu.push(0x40581b); f_004030c0(cpu); if cpu.returned_to != 0x40581b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x405823); f_00401ff0(cpu); if cpu.returned_to != 0x405823 { return; }
  cpu.push(0x405828); f_00411de0(cpu); if cpu.returned_to != 0x405828 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbbcu32, v); }
  cpu.push(0x405837); f_004047d0(cpu); if cpu.returned_to != 0x405837 { return; }
  cpu.push(0x40583c); f_00405440(cpu); if cpu.returned_to != 0x40583c { return; }
  cpu.push(0x405841); f_00405390(cpu); if cpu.returned_to != 0x405841 { return; }
  cpu.push(0x405846); f_004053d0(cpu); if cpu.returned_to != 0x405846 { return; }
  cpu.push(0x40584b); f_004029e0(cpu); if cpu.returned_to != 0x40584b { return; }
  cpu.push(0x405850); f_00402c80(cpu); if cpu.returned_to != 0x405850 { return; }
  { let v: u32 = cpu.r32(0x5de974u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x405867; continue; }
  pc = 0x40585b; continue;
 }
 0x40585b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x405861); cpu.call(t); if cpu.returned_to != 0x405861 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x5de974u32, v); }
  pc = 0x405867; continue;
 }
 0x405867 => {
  { let v = 0x4aafc8u32; cpu.push(v); }
  cpu.push(0x405871); f_004030c0(cpu); if cpu.returned_to != 0x405871 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x405879); f_00406fd0(cpu); if cpu.returned_to != 0x405879 { return; }
  cpu.push(0x40587e); f_00402880(cpu); if cpu.returned_to != 0x40587e { return; }
  { let v: u32 = cpu.r32(0x5ea260u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40589c; continue; }
  pc = 0x405887; continue;
 }
 0x405887 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40588d); f_00402420(cpu); if cpu.returned_to != 0x40588d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea264u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea260u32, v); }
  pc = 0x40589c; continue;
 }
 0x40589c => {
  { let v: u32 = cpu.r32(0x5ea220u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4058b4; continue; }
  pc = 0x4058a5; continue;
 }
 0x4058a5 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4058ab); f_00402420(cpu); if cpu.returned_to != 0x4058ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ea220u32, v); }
  pc = 0x4058b4; continue;
 }
 0x4058b4 => {
  { let v = 0x4aafb8u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddbacu32, v); }
  cpu.push(0x4058c4); f_004030c0(cpu); if cpu.returned_to != 0x4058c4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004058d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4058d0); }
 let mut pc: u32 = 0x4058d0;
 loop { match pc {
 0x4058d0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x405944; continue; }
  pc = 0x4058e0; continue;
 }
 0x4058e0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x4058f9); cpu.call(t); if cpu.returned_to != 0x4058f9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405944; continue; }
  pc = 0x4058fd; continue;
 }
 0x4058fd => {
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1e0u32; cpu.edi = v; }
  pc = 0x405912; continue;
 }
 0x405912 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0xa0u32; cpu.edx = v; }
  pc = 0x405919; continue;
 }
 0x405919 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x405919; continue; }
  pc = 0x405926; continue;
 }
 0x405926 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x405912; continue; }
  pc = 0x405931; continue;
 }
 0x405931 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x405940); cpu.call(t); if cpu.returned_to != 0x405940 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x405944; continue;
 }
 0x405944 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405950(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405950); } r_00405950(cpu, 0x405950); }
pub fn f_00405955(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x405955); } r_00405950(cpu, 0x405955); }
fn r_00405950(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x405950 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4059ba; continue; }
  pc = 0x40595e; continue;
 }
 0x405955 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4059ba; continue; }
  pc = 0x40595e; continue;
 }
 0x40595e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x405977); cpu.call(t); if cpu.returned_to != 0x405977 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4059ba; continue; }
  pc = 0x40597b; continue;
 }
 0x40597b => {
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1e0u32; cpu.edi = v; }
  pc = 0x40598b; continue;
 }
 0x40598b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0xa0u32; cpu.edx = v; }
  pc = 0x405992; continue;
 }
 0x405992 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x405992; continue; }
  pc = 0x40599f; continue;
 }
 0x40599f => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40598b; continue; }
  pc = 0x4059ac; continue;
 }
 0x4059ac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.push(0x4059b7); cpu.call(t); if cpu.returned_to != 0x4059b7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4059ba; continue;
 }
 0x4059ba => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004059c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4059c0); }
 let mut pc: u32 = 0x4059c0;
 loop { match pc {
 0x4059c0 => {
  { let v: u32 = cpu.r32(0x5ddbccu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405a07; continue; }
  pc = 0x4059c9; continue;
 }
 0x4059c9 => {
  { let v: u32 = cpu.r32(0x5ea220u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405a07; continue; }
  pc = 0x4059d2; continue;
 }
 0x4059d2 => {
  { let v: u32 = cpu.r32(0x5de71cu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x405a07; continue; }
  pc = 0x4059dc; continue;
 }
 0x4059dc => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4059e2); f_00405950(cpu); if cpu.returned_to != 0x4059e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x4059ea); f_004052e0(cpu); if cpu.returned_to != 0x4059ea { return; }
  { let v: u32 = cpu.r32(0x5ea220u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4059f5); f_00405950(cpu); if cpu.returned_to != 0x4059f5 { return; }
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405a07; continue; }
  pc = 0x405a01; continue;
 }
 0x405a01 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.push(0x405a07); cpu.call(t); if cpu.returned_to != 0x405a07 { return; }
  pc = 0x405a07; continue;
 }
 0x405a07 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405a10); }
 let mut pc: u32 = 0x405a10;
 loop { match pc {
 0x405a10 => {
  { let v = 0x4ab014u32; cpu.push(v); }
  cpu.push(0x405a1a); f_004030c0(cpu); if cpu.returned_to != 0x405a1a { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4aaf28u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbccu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbbcu32, v); }
  cpu.push(0x405a36); f_004047d0(cpu); if cpu.returned_to != 0x405a36 { return; }
  cpu.push(0x405a3b); f_00405440(cpu); if cpu.returned_to != 0x405a3b { return; }
  cpu.push(0x405a40); f_00405390(cpu); if cpu.returned_to != 0x405a40 { return; }
  cpu.push(0x405a45); f_004053d0(cpu); if cpu.returned_to != 0x405a45 { return; }
  cpu.push(0x405a4a); f_004029e0(cpu); if cpu.returned_to != 0x405a4a { return; }
  cpu.push(0x405a4f); f_00402c80(cpu); if cpu.returned_to != 0x405a4f { return; }
  { let v = 0x4ab000u32; cpu.push(v); }
  cpu.push(0x405a59); f_004030c0(cpu); if cpu.returned_to != 0x405a59 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405a60); }
 let mut pc: u32 = 0x405a60;
 loop { match pc {
 0x405a60 => {
  { let v = 0x4ab03cu32; cpu.push(v); }
  cpu.push(0x405a6a); f_004030c0(cpu); if cpu.returned_to != 0x405a6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbccu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbbcu32, v); }
  { let v = 0x4ab028u32; cpu.push(v); }
  cpu.push(0x405a83); f_004030c0(cpu); if cpu.returned_to != 0x405a83 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405a90); }
 let mut pc: u32 = 0x405a90;
 loop { match pc {
 0x405a90 => {
  { let v = 0x4ab064u32; cpu.push(v); }
  cpu.push(0x405a9a); f_004030c0(cpu); if cpu.returned_to != 0x405a9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x405aa2); f_004028a0(cpu); if cpu.returned_to != 0x405aa2 { return; }
  cpu.push(0x405aa7); f_00402b60(cpu); if cpu.returned_to != 0x405aa7 { return; }
  cpu.push(0x405aac); f_00404dc0(cpu); if cpu.returned_to != 0x405aac { return; }
  cpu.push(0x405ab1); f_00404890(cpu); if cpu.returned_to != 0x405ab1 { return; }
  cpu.push(0x405ab6); f_00416eb1(cpu); if cpu.returned_to != 0x405ab6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbccu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbbcu32, v); }
  cpu.push(0x405ac7); f_004059c0(cpu); if cpu.returned_to != 0x405ac7 { return; }
  { let v = 0x4ab028u32; cpu.push(v); }
  cpu.push(0x405ad1); f_004030c0(cpu); if cpu.returned_to != 0x405ad1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405ae0); }
 let mut pc: u32 = 0x405ae0;
 loop { match pc {
 0x405ae0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x405b5b; continue; }
  pc = 0x405aee; continue;
 }
 0x405aee => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x405b07); cpu.call(t); if cpu.returned_to != 0x405b07 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405b5b; continue; }
  pc = 0x405b0b; continue;
 }
 0x405b0b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.r32(0x7cu32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405b4f; continue; }
  pc = 0x405b2c; continue;
 }
 0x405b2c => {
  { let v: u8 = cpu.r8(0x84u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  pc = 0x405b4f; continue;
 }
 0x405b4f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.push(0x405b5a); cpu.call(t); if cpu.returned_to != 0x405b5a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x405b5b; continue;
 }
 0x405b5b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405b60); }
 let mut pc: u32 = 0x405b60;
 loop { match pc {
 0x405b60 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x405bc7; continue; }
  pc = 0x405b6e; continue;
 }
 0x405b6e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x405b87); cpu.call(t); if cpu.returned_to != 0x405b87 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405bc7; continue; }
  pc = 0x405b8b; continue;
 }
 0x405b8b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x78u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405bbc; continue; }
  pc = 0x405ba8; continue;
 }
 0x405ba8 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x405bb0; continue;
 }
 0x405bb0 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x405bb0; continue; }
  pc = 0x405bbb; continue;
 }
 0x405bbb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x405bbc; continue;
 }
 0x405bbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x405bc7); cpu.call(t); if cpu.returned_to != 0x405bc7 { return; }
  pc = 0x405bc7; continue;
 }
 0x405bc7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405bd0); }
 let mut pc: u32 = 0x405bd0;
 loop { match pc {
 0x405bd0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x405c5c; continue; }
  pc = 0x405bdf; continue;
 }
 0x405bdf => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xd5u32); }
  if !cpu.zf { pc = 0x405bf9; continue; }
  pc = 0x405bed; continue;
 }
 0x405bed => {
  { let v: u8 = 0xd4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xd5u8; cpu.w8(0x80u32.wrapping_add(cpu.esp), v); }
  pc = 0x405c04; continue;
 }
 0x405bf9 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x405c04; continue;
 }
 0x405c04 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x405c1d); cpu.call(t); if cpu.returned_to != 0x405c1d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405c5c; continue; }
  pc = 0x405c21; continue;
 }
 0x405c21 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x7cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x405c5c); cpu.call(t); if cpu.returned_to != 0x405c5c { return; }
  pc = 0x405c5c; continue;
 }
 0x405c5c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405c70); }
 let mut pc: u32 = 0x405c70;
 loop { match pc {
 0x405c70 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x7cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1d84u32); }
  { let v = cpu.edi; cpu.push(v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405c90; continue; }
  pc = 0x405c86; continue;
 }
 0x405c86 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405c90 => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405ca3; continue; }
  pc = 0x405c99; continue;
 }
 0x405c99 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405ca3 => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x405caa); f_00402400(cpu); if cpu.returned_to != 0x405caa { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x405cbb; continue; }
  pc = 0x405cb3; continue;
 }
 0x405cb3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405cbb => {
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x405ce8; continue; }
  pc = 0x405cc7; continue;
 }
 0x405cc7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x405cd2); f_004030c0(cpu); if cpu.returned_to != 0x405cd2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x405cdb); f_00402420(cpu); if cpu.returned_to != 0x405cdb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405ce8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffffffcu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x405d12); f_00402310(cpu); if cpu.returned_to != 0x405d12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405d2f; continue; }
  pc = 0x405d1c; continue;
 }
 0x405d1c => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x405d22); f_00402420(cpu); if cpu.returned_to != 0x405d22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405d2f => {
  { let v: u32 = 0x6cu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x405d4b); cpu.call(t); if cpu.returned_to != 0x405d4b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x405dfc; continue; }
  pc = 0x405d5a; continue;
 }
 0x405d5a => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x405d78; continue;
 }
 0x405d78 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x405dda; continue; }
  pc = 0x405d84; continue;
 }
 0x405d84 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x30u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405dc9; continue; }
  pc = 0x405d96; continue;
 }
 0x405d96 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x405d9d; continue;
 }
 0x405d9d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x405db8; continue; }
  pc = 0x405da4; continue;
 }
 0x405da4 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x405db8; continue; }
  pc = 0x405da8; continue;
 }
 0x405da8 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x405db8; continue;
 }
 0x405db8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405d9d; continue; }
  pc = 0x405dc9; continue;
 }
 0x405dc9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405d84; continue; }
  pc = 0x405dda; continue;
 }
 0x405dda => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x405d78; continue; }
  pc = 0x405dee; continue;
 }
 0x405dee => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x405dfc); cpu.call(t); if cpu.returned_to != 0x405dfc { return; }
  pc = 0x405dfc; continue;
 }
 0x405dfc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405e10); }
 let mut pc: u32 = 0x405e10;
 loop { match pc {
 0x405e10 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1d84u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405e2c; continue; }
  pc = 0x405e22; continue;
 }
 0x405e22 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405e2c => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x405e40; continue; }
  pc = 0x405e36; continue;
 }
 0x405e36 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405e40 => {
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x405e78; continue; }
  pc = 0x405e57; continue;
 }
 0x405e57 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x405e62); f_004030c0(cpu); if cpu.returned_to != 0x405e62 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x405e6b); f_00402420(cpu); if cpu.returned_to != 0x405e6b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405e78 => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp & 0xfffffffcu32; cpu.ebp = v; }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.push(0x405ea8); cpu.call(t); if cpu.returned_to != 0x405ea8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x405f39; continue; }
  pc = 0x405eb0; continue;
 }
 0x405eb0 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x405ec0; continue;
 }
 0x405ec0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x405f18; continue; }
  pc = 0x405ec8; continue;
 }
 0x405ec8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x2cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405f0b; continue; }
  pc = 0x405edc; continue;
 }
 0x405edc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x405ee2; continue;
 }
 0x405ee2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x405ef9; continue; }
  pc = 0x405ee9; continue;
 }
 0x405ee9 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x405ef9; continue; }
  pc = 0x405eed; continue;
 }
 0x405eed => {
  { let v: u8 = cpu.r8(0x90u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x405ef9; continue;
 }
 0x405ef9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405ee2; continue; }
  pc = 0x405f0b; continue;
 }
 0x405f0b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x405ec8; continue; }
  pc = 0x405f14; continue;
 }
 0x405f14 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x405f18; continue;
 }
 0x405f18 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x405ec0; continue; }
  pc = 0x405f2c; continue;
 }
 0x405f2c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x405f39); cpu.call(t); if cpu.returned_to != 0x405f39 { return; }
  pc = 0x405f39; continue;
 }
 0x405f39 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00405f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x405f50); }
 let mut pc: u32 = 0x405f50;
 loop { match pc {
 0x405f50 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1d84u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x405f70; continue; }
  pc = 0x405f66; continue;
 }
 0x405f66 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405f70 => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x405f83; continue; }
  pc = 0x405f79; continue;
 }
 0x405f79 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405f83 => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x405f8a); f_00402400(cpu); if cpu.returned_to != 0x405f8a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x405f9f; continue; }
  pc = 0x405f97; continue;
 }
 0x405f97 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405f9f => {
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.edi.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x405fcc; continue; }
  pc = 0x405fab; continue;
 }
 0x405fab => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x405fb6); f_004030c0(cpu); if cpu.returned_to != 0x405fb6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x405fbf); f_00402420(cpu); if cpu.returned_to != 0x405fbf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x405fcc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edi & 0xfffffffcu32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x405ff6); f_00402310(cpu); if cpu.returned_to != 0x405ff6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406012; continue; }
  pc = 0x405fff; continue;
 }
 0x405fff => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x406005); f_00402420(cpu); if cpu.returned_to != 0x406005 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406012 => {
  { let v: u32 = 0x6cu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x40602d); cpu.call(t); if cpu.returned_to != 0x40602d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4060a0; continue; }
  pc = 0x406031; continue;
 }
 0x406031 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x40603d; continue;
 }
 0x40603d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x40607f; continue; }
  pc = 0x406045; continue;
 }
 0x406045 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x28u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406076; continue; }
  pc = 0x406059; continue;
 }
 0x406059 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x40605c; continue;
 }
 0x40605c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x406064; continue; }
  pc = 0x406062; continue;
 }
 0x406062 => {
  { let v: u8 = (cpu.eax as u8) | 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x406064; continue;
 }
 0x406064 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40605c; continue; }
  pc = 0x406076; continue;
 }
 0x406076 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406045; continue; }
  pc = 0x40607f; continue;
 }
 0x40607f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40603d; continue; }
  pc = 0x40608b; continue;
 }
 0x40608b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x40609c); cpu.call(t); if cpu.returned_to != 0x40609c { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x4060a0; continue;
 }
 0x4060a0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004060b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4060b0); }
 let mut pc: u32 = 0x4060b0;
 loop { match pc {
 0x4060b0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1e0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf == cpu.of) { pc = 0x406142; continue; }
  pc = 0x4060cf; continue;
 }
 0x4060cf => {
  { let v: u32 = cpu.r32(0x5e9aa0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x406142; continue; }
  pc = 0x4060da; continue;
 }
 0x4060da => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x406107); f_00404a30(cpu); if cpu.returned_to != 0x406107 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406142; continue; }
  pc = 0x40610e; continue;
 }
 0x40610e => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406142; continue; }
  pc = 0x406117; continue;
 }
 0x406117 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.push(0x406131); cpu.call(t); if cpu.returned_to != 0x406131 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406142; continue; }
  pc = 0x406135; continue;
 }
 0x406135 => {
  { let v = 0x4ab0a0u32; cpu.push(v); }
  cpu.push(0x40613f); f_004030c0(cpu); if cpu.returned_to != 0x40613f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x406142; continue;
 }
 0x406142 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406150); }
 let mut pc: u32 = 0x406150;
 loop { match pc {
 0x406150 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1d84u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x406210; continue; }
  pc = 0x406161; continue;
 }
 0x406161 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4061b9; continue; }
  pc = 0x406169; continue;
 }
 0x406169 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40616f); f_00405f50(cpu); if cpu.returned_to != 0x40616f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e9a90u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406210; continue; }
  pc = 0x40617f; continue;
 }
 0x40617f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1d84u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x406193); f_00404cb0(cpu); if cpu.returned_to != 0x406193 { return; }
  { let v: u32 = cpu.r32(0x5e9a90u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4061a4); cpu.call(t); if cpu.returned_to != 0x4061a4 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4061aa); f_00402420(cpu); if cpu.returned_to != 0x4061aa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9a90u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4061b9 => {
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4061e0; continue; }
  pc = 0x4061c4; continue;
 }
 0x4061c4 => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4061d0); f_00405c70(cpu); if cpu.returned_to != 0x4061d0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406210; continue; }
  pc = 0x4061de; continue;
 }
 0x4061de => {
  pc = 0x4061fd; continue;
 }
 0x4061e0 => {
  { cpu.sub32(cpu.esi, 0x2au32); }
  if cpu.zf { pc = 0x4061ed; continue; }
  pc = 0x4061e5; continue;
 }
 0x4061e5 => {
  { cpu.sub32(cpu.esi, 0xb8u32); }
  if !cpu.zf { pc = 0x4061fd; continue; }
  pc = 0x4061ed; continue;
 }
 0x4061ed => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4061fa); f_00405e10(cpu); if cpu.returned_to != 0x4061fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4061fd; continue;
 }
 0x4061fd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40620d); f_00404cb0(cpu); if cpu.returned_to != 0x40620d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x406210; continue;
 }
 0x406210 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406220); }
 let mut pc: u32 = 0x406220;
 loop { match pc {
 0x406220 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1d84u32); }
  { let v = cpu.edi; cpu.push(v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40635c; continue; }
  pc = 0x406232; continue;
 }
 0x406232 => {
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x406243; continue; }
  pc = 0x40623d; continue;
 }
 0x40623d => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40625e; continue; }
  pc = 0x406243; continue;
 }
 0x406243 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x406258); f_00406150(cpu); if cpu.returned_to != 0x406258 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40625e => {
  { cpu.sub32(cpu.esi, 0x2au32); }
  if cpu.zf { pc = 0x40626b; continue; }
  pc = 0x406263; continue;
 }
 0x406263 => {
  { cpu.sub32(cpu.esi, 0xb8u32); }
  if !cpu.zf { pc = 0x40627a; continue; }
  pc = 0x40626b; continue;
 }
 0x40626b => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406277); f_00405e10(cpu); if cpu.returned_to != 0x406277 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40627a; continue;
 }
 0x40627a => {
  { let v: u8 = cpu.r8(0x593a9au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4062c6; continue; }
  pc = 0x406283; continue;
 }
 0x406283 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4062a5; continue; }
  pc = 0x40628a; continue;
 }
 0x40628a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40629f); f_00406150(cpu); if cpu.returned_to != 0x40629f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4062a5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4062ad); f_00405e10(cpu); if cpu.returned_to != 0x4062ad { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4062c0); f_00404cb0(cpu); if cpu.returned_to != 0x4062c0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4062c6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406312; continue; }
  pc = 0x4062ce; continue;
 }
 0x4062ce => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x4062f2; continue; }
  pc = 0x4062da; continue;
 }
 0x4062da => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4062ec); f_00406150(cpu); if cpu.returned_to != 0x4062ec { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4062f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4062f9); f_00405e10(cpu); if cpu.returned_to != 0x4062f9 { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40630c); f_00404cb0(cpu); if cpu.returned_to != 0x40630c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406312 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406318); f_00405f50(cpu); if cpu.returned_to != 0x406318 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e9a90u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40635c; continue; }
  pc = 0x406324; continue;
 }
 0x406324 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1d84u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x406338); f_00404cb0(cpu); if cpu.returned_to != 0x406338 { return; }
  { let v: u32 = cpu.r32(0x5e9a90u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x406349); cpu.call(t); if cpu.returned_to != 0x406349 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40634f); f_00402420(cpu); if cpu.returned_to != 0x40634f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9a90u32, v); }
  pc = 0x40635c; continue;
 }
 0x40635c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406360); }
 let mut pc: u32 = 0x406360;
 loop { match pc {
 0x406360 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x40642a; continue; }
  pc = 0x406376; continue;
 }
 0x406376 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x40638f); cpu.call(t); if cpu.returned_to != 0x40638f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40642a; continue; }
  pc = 0x406397; continue;
 }
 0x406397 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x80u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40641e; continue; }
  pc = 0x4063c2; continue;
 }
 0x4063c2 => {
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x4063df; continue;
 }
 0x4063df => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4063fa; continue; }
  pc = 0x4063e3; continue;
 }
 0x4063e3 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x4063eb; continue;
 }
 0x4063eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4063eb; continue; }
  pc = 0x4063f6; continue;
 }
 0x4063f6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x4063fa; continue;
 }
 0x4063fa => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x4063df; continue; }
  pc = 0x40641c; continue;
 }
 0x40641c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x40641e; continue;
 }
 0x40641e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x406429); cpu.call(t); if cpu.returned_to != 0x406429 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x40642a; continue;
 }
 0x40642a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406430); }
 let mut pc: u32 = 0x406430;
 loop { match pc {
 0x406430 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x4064fb; continue; }
  pc = 0x406444; continue;
 }
 0x406444 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x40645d); cpu.call(t); if cpu.returned_to != 0x40645d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4064fb; continue; }
  pc = 0x406465; continue;
 }
 0x406465 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x406492; continue;
 }
 0x406492 => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  pc = 0x40649d; continue;
 }
 0x40649d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x140u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40649d; continue; }
  pc = 0x4064c5; continue;
 }
 0x4064c5 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x140u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x406492; continue; }
  pc = 0x4064e8; continue;
 }
 0x4064e8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x4064f7); cpu.call(t); if cpu.returned_to != 0x4064f7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4064fb; continue;
 }
 0x4064fb => {
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406500); }
 let mut pc: u32 = 0x406500;
 loop { match pc {
 0x406500 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x406586; continue; }
  pc = 0x40650e; continue;
 }
 0x40650e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x406527); cpu.call(t); if cpu.returned_to != 0x406527 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406586; continue; }
  pc = 0x40652b; continue;
 }
 0x40652b => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x29u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(2)); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406579; continue; }
  pc = 0x40655f; continue;
 }
 0x40655f => {
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x406566; continue;
 }
 0x406566 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x406566; continue; }
  pc = 0x406579; continue;
 }
 0x406579 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.push(0x406584); cpu.call(t); if cpu.returned_to != 0x406584 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x406586; continue;
 }
 0x406586 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406590); }
 let mut pc: u32 = 0x406590;
 loop { match pc {
 0x406590 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40661a; continue; }
  pc = 0x40659e; continue;
 }
 0x40659e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x4065b7); cpu.call(t); if cpu.returned_to != 0x4065b7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40661a; continue; }
  pc = 0x4065bb; continue;
 }
 0x4065bb => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x29u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40660d; continue; }
  pc = 0x4065ef; continue;
 }
 0x4065ef => {
  { let v: u8 = cpu.r8(0x88u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4065f6; continue;
 }
 0x4065f6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4065f6; continue; }
  pc = 0x40660d; continue;
 }
 0x40660d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.push(0x406618); cpu.call(t); if cpu.returned_to != 0x406618 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x40661a; continue;
 }
 0x40661a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406620); }
 let mut pc: u32 = 0x406620;
 loop { match pc {
 0x406620 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x4066ec; continue; }
  pc = 0x406635; continue;
 }
 0x406635 => {
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xd5u32); }
  if !cpu.zf { pc = 0x40664f; continue; }
  pc = 0x406643; continue;
 }
 0x406643 => {
  { let v: u8 = 0xd4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xd5u8; cpu.w8(0x88u32.wrapping_add(cpu.esp), v); }
  pc = 0x40665a; continue;
 }
 0x40664f => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x40665a; continue;
 }
 0x40665a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x406673); cpu.call(t); if cpu.returned_to != 0x406673 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4066ec; continue; }
  pc = 0x406677; continue;
 }
 0x406677 => {
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x29u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x90u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.push(0x4066ec); cpu.call(t); if cpu.returned_to != 0x4066ec { return; }
  pc = 0x4066ec; continue;
 }
 0x4066ec => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406700); }
 let mut pc: u32 = 0x406700;
 loop { match pc {
 0x406700 => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x7cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x40671a; continue; }
  pc = 0x406710; continue;
 }
 0x406710 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40671a => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x406721); f_00402400(cpu); if cpu.returned_to != 0x406721 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x406732; continue; }
  pc = 0x40672a; continue;
 }
 0x40672a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406732 => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x406766; continue; }
  pc = 0x406745; continue;
 }
 0x406745 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x406750); f_004030c0(cpu); if cpu.returned_to != 0x406750 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x406759); f_00402420(cpu); if cpu.returned_to != 0x406759 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406766 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffffffcu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x406792); f_00402310(cpu); if cpu.returned_to != 0x406792 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4067af; continue; }
  pc = 0x40679c; continue;
 }
 0x40679c => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4067a2); f_00402420(cpu); if cpu.returned_to != 0x4067a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4067af => {
  { let v: u32 = 0x6cu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x4067cb); cpu.call(t); if cpu.returned_to != 0x4067cb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x406897; continue; }
  pc = 0x4067da; continue;
 }
 0x4067da => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x4067f8; continue;
 }
 0x4067f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x40686f; continue; }
  pc = 0x406804; continue;
 }
 0x406804 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x30u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406862; continue; }
  pc = 0x40681a; continue;
 }
 0x40681a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x406820; continue;
 }
 0x406820 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x40683b; continue; }
  pc = 0x406827; continue;
 }
 0x406827 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x40683b; continue; }
  pc = 0x40682b; continue;
 }
 0x40682b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40683b; continue;
 }
 0x40683b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406820; continue; }
  pc = 0x40685e; continue;
 }
 0x40685e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x406862; continue;
 }
 0x406862 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406804; continue; }
  pc = 0x40686f; continue;
 }
 0x40686f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x4067f8; continue; }
  pc = 0x406889; continue;
 }
 0x406889 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.ecx)); cpu.push(0x406897); cpu.call(t); if cpu.returned_to != 0x406897 { return; }
  pc = 0x406897; continue;
 }
 0x406897 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004068b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4068b0); }
 let mut pc: u32 = 0x4068b0;
 loop { match pc {
 0x4068b0 => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x4068ca; continue; }
  pc = 0x4068c0; continue;
 }
 0x4068c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4068ca => {
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x406909; continue; }
  pc = 0x4068e8; continue;
 }
 0x4068e8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x4068f3); f_004030c0(cpu); if cpu.returned_to != 0x4068f3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4068fc); f_00402420(cpu); if cpu.returned_to != 0x4068fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406909 => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x5u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp & 0xfffffffcu32; cpu.ebp = v; }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.push(0x406939); cpu.call(t); if cpu.returned_to != 0x406939 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4069e4; continue; }
  pc = 0x406941; continue;
 }
 0x406941 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x406951; continue;
 }
 0x406951 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x4069bd; continue; }
  pc = 0x406959; continue;
 }
 0x406959 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x2cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4069b0; continue; }
  pc = 0x40696f; continue;
 }
 0x40696f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x406975; continue;
 }
 0x406975 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x40698c; continue; }
  pc = 0x40697c; continue;
 }
 0x40697c => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if (cpu.sf == cpu.of) { pc = 0x40698c; continue; }
  pc = 0x406980; continue;
 }
 0x406980 => {
  { let v: u8 = cpu.r8(0x90u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40698c; continue;
 }
 0x40698c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406975; continue; }
  pc = 0x4069b0; continue;
 }
 0x4069b0 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406959; continue; }
  pc = 0x4069b9; continue;
 }
 0x4069b9 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x4069bd; continue;
 }
 0x4069bd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x406951; continue; }
  pc = 0x4069d7; continue;
 }
 0x4069d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x4069e4); cpu.call(t); if cpu.returned_to != 0x4069e4 { return; }
  pc = 0x4069e4; continue;
 }
 0x4069e4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004069f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4069f0); }
 let mut pc: u32 = 0x4069f0;
 loop { match pc {
 0x4069f0 => {
  { let v: u32 = cpu.r32(0x5ddbbcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x406a0a; continue; }
  pc = 0x406a00; continue;
 }
 0x406a00 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406a0a => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x406a11); f_00402400(cpu); if cpu.returned_to != 0x406a11 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x406a26; continue; }
  pc = 0x406a1e; continue;
 }
 0x406a1e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406a26 => {
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5431b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x406a5a; continue; }
  pc = 0x406a39; continue;
 }
 0x406a39 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab088u32; cpu.push(v); }
  cpu.push(0x406a44); f_004030c0(cpu); if cpu.returned_to != 0x406a44 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x406a4d); f_00402420(cpu); if cpu.returned_to != 0x406a4d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406a5a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edi & 0xfffffffcu32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x406a86); f_00402310(cpu); if cpu.returned_to != 0x406a86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406aa2; continue; }
  pc = 0x406a8f; continue;
 }
 0x406a8f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x406a95); f_00402420(cpu); if cpu.returned_to != 0x406a95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406aa2 => {
  { let v: u32 = 0x6cu32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.push(0x406abd); cpu.call(t); if cpu.returned_to != 0x406abd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406b55; continue; }
  pc = 0x406ac5; continue;
 }
 0x406ac5 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x406ad5; continue;
 }
 0x406ad5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x406b32; continue; }
  pc = 0x406add; continue;
 }
 0x406add => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x2cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406b29; continue; }
  pc = 0x406af3; continue;
 }
 0x406af3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x406af9; continue;
 }
 0x406af9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x406b01; continue; }
  pc = 0x406aff; continue;
 }
 0x406aff => {
  { let v: u8 = (cpu.eax as u8) | 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x406b01; continue;
 }
 0x406b01 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406af9; continue; }
  pc = 0x406b25; continue;
 }
 0x406b25 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x406b29; continue;
 }
 0x406b29 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406add; continue; }
  pc = 0x406b32; continue;
 }
 0x406b32 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x406ad5; continue; }
  pc = 0x406b48; continue;
 }
 0x406b48 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x406b55); cpu.call(t); if cpu.returned_to != 0x406b55 { return; }
  pc = 0x406b55; continue;
 }
 0x406b55 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x78u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406b60); }
 let mut pc: u32 = 0x406b60;
 loop { match pc {
 0x406b60 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1e0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf == cpu.of) { pc = 0x406bf9; continue; }
  pc = 0x406b86; continue;
 }
 0x406b86 => {
  { let v: u32 = cpu.r32(0x5e9aa0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x406bf9; continue; }
  pc = 0x406b91; continue;
 }
 0x406b91 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x406bbe); f_00404a30(cpu); if cpu.returned_to != 0x406bbe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406bf9; continue; }
  pc = 0x406bc5; continue;
 }
 0x406bc5 => {
  { let v: u32 = cpu.r32(0x5df728u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406bf9; continue; }
  pc = 0x406bce; continue;
 }
 0x406bce => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x406be8); cpu.call(t); if cpu.returned_to != 0x406be8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406bf9; continue; }
  pc = 0x406bec; continue;
 }
 0x406bec => {
  { let v = 0x4ab0a0u32; cpu.push(v); }
  cpu.push(0x406bf6); f_004030c0(cpu); if cpu.returned_to != 0x406bf6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x406bf9; continue;
 }
 0x406bf9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406c00); }
 let mut pc: u32 = 0x406c00;
 loop { match pc {
 0x406c00 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 1u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406c52; continue; }
  pc = 0x406c19; continue;
 }
 0x406c19 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x406c23); f_004069f0(cpu); if cpu.returned_to != 0x406c23 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e9a90u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406cdb; continue; }
  pc = 0x406c33; continue;
 }
 0x406c33 => {
  { let v = 0x1d84u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x406c3f); f_00404cb0(cpu); if cpu.returned_to != 0x406c3f { return; }
  { let v: u32 = cpu.r32(0x5e9a90u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x406c50); cpu.call(t); if cpu.returned_to != 0x406c50 { return; }
  pc = 0x406c93; continue;
 }
 0x406c52 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xb8u32); }
  if !cpu.zf { pc = 0x406caa; continue; }
  pc = 0x406c5e; continue;
 }
 0x406c5e => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406c6a); f_00406700(cpu); if cpu.returned_to != 0x406c6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e9a90u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406cdb; continue; }
  pc = 0x406c76; continue;
 }
 0x406c76 => {
  { let v = 0x1d84u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x406c82); f_00404cb0(cpu); if cpu.returned_to != 0x406c82 { return; }
  { let v: u32 = cpu.r32(0x5e9a90u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x406c93); cpu.call(t); if cpu.returned_to != 0x406c93 { return; }
  pc = 0x406c93; continue;
 }
 0x406c93 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406c99); f_00402420(cpu); if cpu.returned_to != 0x406c99 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9a90u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406caa => {
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406cd0; continue; }
  pc = 0x406cb5; continue;
 }
 0x406cb5 => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406cc2); f_00406700(cpu); if cpu.returned_to != 0x406cc2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406cdb; continue; }
  pc = 0x406cd0; continue;
 }
 0x406cd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x406cd8); f_00404cb0(cpu); if cpu.returned_to != 0x406cd8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x406cdb; continue;
 }
 0x406cdb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406ce0); }
 let mut pc: u32 = 0x406ce0;
 loop { match pc {
 0x406ce0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406d06; continue; }
  pc = 0x406cf0; continue;
 }
 0x406cf0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x406d06; continue; }
  pc = 0x406cf8; continue;
 }
 0x406cf8 => {
  { let v: u8 = cpu.r8(0x593a9au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x406d11; continue; }
  pc = 0x406d02; continue;
 }
 0x406d02 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406d0d; continue; }
  pc = 0x406d06; continue;
 }
 0x406d06 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x406d25; continue;
 }
 0x406d0d => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x406d3b; continue;
 }
 0x406d11 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x406d60; continue; }
  pc = 0x406d19; continue;
 }
 0x406d19 => {
  { let v: u32 = cpu.r32(0x5932d6u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x406d3a; continue; }
  pc = 0x406d23; continue;
 }
 0x406d23 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x406d25; continue;
 }
 0x406d25 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x406d35); f_00406c00(cpu); if cpu.returned_to != 0x406d35 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406d3a => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x406d3b; continue;
 }
 0x406d3b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406d41); f_004068b0(cpu); if cpu.returned_to != 0x406d41 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x406d5b); f_00404cb0(cpu); if cpu.returned_to != 0x406d5b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406d60 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406d66); f_004069f0(cpu); if cpu.returned_to != 0x406d66 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e9a90u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406db0; continue; }
  pc = 0x406d72; continue;
 }
 0x406d72 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 41u32); cpu.ecx = v; }
  { let v = 0x1d84u32; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x406d8c); f_00404cb0(cpu); if cpu.returned_to != 0x406d8c { return; }
  { let v: u32 = cpu.r32(0x5e9a90u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x406d9d); cpu.call(t); if cpu.returned_to != 0x406d9d { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406da3); f_00402420(cpu); if cpu.returned_to != 0x406da3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e9a90u32, v); }
  pc = 0x406db0; continue;
 }
 0x406db0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406dc0); }
 let mut pc: u32 = 0x406dc0;
 loop { match pc {
 0x406dc0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 1u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406df6; continue; }
  pc = 0x406de0; continue;
 }
 0x406de0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406de8); f_00406700(cpu); if cpu.returned_to != 0x406de8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406e01; continue; }
  pc = 0x406df6; continue;
 }
 0x406df6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x406dfe); f_00404d30(cpu); if cpu.returned_to != 0x406dfe { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x406e01; continue;
 }
 0x406e01 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406e10); }
 let mut pc: u32 = 0x406e10;
 loop { match pc {
 0x406e10 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 1u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406e4d; continue; }
  pc = 0x406e30; continue;
 }
 0x406e30 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x406e38); f_00406700(cpu); if cpu.returned_to != 0x406e38 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2480u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406e58; continue; }
  pc = 0x406e46; continue;
 }
 0x406e46 => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x406e4d; continue;
 }
 0x406e4d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x406e55); f_00404d30(cpu); if cpu.returned_to != 0x406e55 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x406e58; continue;
 }
 0x406e58 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406e60); }
 let mut pc: u32 = 0x406e60;
 loop { match pc {
 0x406e60 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x7cu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df728u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x406f59; continue; }
  pc = 0x406e76; continue;
 }
 0x406e76 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x6cu32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x406e90); cpu.call(t); if cpu.returned_to != 0x406e90 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x406f59; continue; }
  pc = 0x406e98; continue;
 }
 0x406e98 => {
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = 0x29u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406f4a; continue; }
  pc = 0x406eda; continue;
 }
 0x406eda => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x406eec; continue;
 }
 0x406eec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x406f2e; continue; }
  pc = 0x406ef2; continue;
 }
 0x406ef2 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  pc = 0x406ef9; continue;
 }
 0x406ef9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x406ef9; continue; }
  pc = 0x406f1e; continue;
 }
 0x406f1e => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x406f2e; continue;
 }
 0x406f2e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x406eec; continue; }
  pc = 0x406f4a; continue;
 }
 0x406f4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x406f56); cpu.call(t); if cpu.returned_to != 0x406f56 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x406f59; continue;
 }
 0x406f59 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406f60); }
 let mut pc: u32 = 0x406f60;
 loop { match pc {
 0x406f60 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406f7c; continue; }
  pc = 0x406f69; continue;
 }
 0x406f69 => {
  { let v = 0x4ab0e4u32; cpu.push(v); }
  cpu.push(0x406f73); f_004030c0(cpu); if cpu.returned_to != 0x406f73 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406f7c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5e2458u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x406f8a); f_00413308(cpu); if cpu.returned_to != 0x406f8a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406f9e; continue; }
  pc = 0x406f8e; continue;
 }
 0x406f8e => {
  { let v = 0x4ab0c8u32; cpu.push(v); }
  cpu.push(0x406f98); f_004030c0(cpu); if cpu.returned_to != 0x406f98 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406f9e => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.push(0x406fb0); cpu.call(t); if cpu.returned_to != 0x406fb0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406fc9; continue; }
  pc = 0x406fb4; continue;
 }
 0x406fb4 => {
  cpu.push(0x406fb9); f_00406fd0(cpu); if cpu.returned_to != 0x406fb9 { return; }
  { let v = 0x4ab0acu32; cpu.push(v); }
  cpu.push(0x406fc3); f_004030c0(cpu); if cpu.returned_to != 0x406fc3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x406fc9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406fd0); }
 let mut pc: u32 = 0x406fd0;
 loop { match pc {
 0x406fd0 => {
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x406fe9; continue; }
  pc = 0x406fd9; continue;
 }
 0x406fd9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x406fdf); cpu.call(t); if cpu.returned_to != 0x406fdf { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e2458u32, v); }
  pc = 0x406fe9; continue;
 }
 0x406fe9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00406ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x406ff0); }
 let mut pc: u32 = 0x406ff0;
 loop { match pc {
 0x406ff0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x406ffd); f_00406f60(cpu); if cpu.returned_to != 0x406ffd { return; }
  { let v: u32 = cpu.r32(0x5e2458u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407085; continue; }
  pc = 0x407009; continue;
 }
 0x407009 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x14u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x407040); cpu.call(t); if cpu.returned_to != 0x407040 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x407078; continue; }
  pc = 0x407044; continue;
 }
 0x407044 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.push(0x407054); cpu.call(t); if cpu.returned_to != 0x407054 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x407065; continue; }
  pc = 0x407058; continue;
 }
 0x407058 => {
  { let v = 0x4ab128u32; cpu.push(v); }
  cpu.push(0x407062); f_004030c0(cpu); if cpu.returned_to != 0x407062 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407065; continue;
 }
 0x407065 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(0x40706f); cpu.call(t); if cpu.returned_to != 0x40706f { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407078 => {
  { let v = 0x4ab100u32; cpu.push(v); }
  cpu.push(0x407082); f_004030c0(cpu); if cpu.returned_to != 0x407082 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407085; continue;
 }
 0x407085 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407090); }
 let mut pc: u32 = 0x407090;
 loop { match pc {
 0x407090 => {
  cpu.push(0x407095); f_00404c30(cpu); if cpu.returned_to != 0x407095 { return; }
  cpu.push(0x40709a); f_004056e0(cpu); if cpu.returned_to != 0x40709a { return; }
  { let v: u32 = cpu.r32(0x5ea260u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4070af); f_004077a0(cpu); if cpu.returned_to != 0x4070af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4070b7; continue; }
  pc = 0x4070b6; continue;
 }
 0x4070b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4070b7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4070c1); f_0040c270(cpu); if cpu.returned_to != 0x4070c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x4070c9); f_00407c00(cpu); if cpu.returned_to != 0x4070c9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbe0u32, v); }
  cpu.push(0x4070d3); f_00407c10(cpu); if cpu.returned_to != 0x4070d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbd8u32, v); }
  if !cpu.zf { pc = 0x4070e6; continue; }
  pc = 0x4070dc; continue;
 }
 0x4070dc => {
  { let v: u32 = 0x2b11u32; cpu.w32(0x5ddbd8u32, v); }
  pc = 0x4070e6; continue;
 }
 0x4070e6 => {
  cpu.push(0x4070eb); f_00407c20(cpu); if cpu.returned_to != 0x4070eb { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbdcu32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4070fe; continue; }
  pc = 0x4070f9; continue;
 }
 0x4070f9 => {
  cpu.push(0x4070fe); f_00407c30(cpu); if cpu.returned_to != 0x4070fe { return; }
  pc = 0x4070fe; continue;
 }
 0x4070fe => {
  { let v = 0x5ddbf0u32; cpu.push(v); }
  { let v = 0x5ddbfcu32; cpu.push(v); }
  { let v = 0x5ddbd4u32; cpu.push(v); }
  { let v = 0x5ddbecu32; cpu.push(v); }
  { let v = 0x5ddbe4u32; cpu.push(v); }
  { let v = 0x5ddbf8u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbf4u32, v); }
  cpu.push(0x407126); f_00407c50(cpu); if cpu.returned_to != 0x407126 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407130); }
 let mut pc: u32 = 0x407130;
 loop { match pc {
 0x407130 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbe8u32, v); }
  { let v: u32 = cpu.r32(0x5ddbfcu32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4aaf28u32, v); }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  if !cpu.zf { pc = 0x40716f; continue; }
  pc = 0x407168; continue;
 }
 0x407168 => {
  { let v: u32 = 0x3f3f3f3fu32; cpu.eax = v; }
  pc = 0x407171; continue;
 }
 0x40716f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x407171; continue;
 }
 0x407171 => {
  { let v: u32 = 0x5ec4a0u32; cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.stos(4, true); }
  cpu.push(0x40717e); f_004076e0(cpu); if cpu.returned_to != 0x40717e { return; }
  { let v: u32 = cpu.r32(0x5ea27cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ddbe0u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddc00u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x40719c); f_00407740(cpu); if cpu.returned_to != 0x40719c { return; }
  cpu.push(0x4071a1); f_00407cb0(cpu); if cpu.returned_to != 0x4071a1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4073c0; continue; }
  pc = 0x4071a9; continue;
 }
 0x4071a9 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x4071b3; continue;
 }
 0x4071b3 => {
  cpu.push(0x4071b8); f_00407f60(cpu); if cpu.returned_to != 0x4071b8 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40727c; continue; }
  pc = 0x4071c3; continue;
 }
 0x4071c3 => {
  { let v: u32 = cpu.r32(0x5ec7a0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ec7a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ec7a4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x407219; continue; }
  pc = 0x4071db; continue;
 }
 0x4071db => {
  { let v: u32 = cpu.r32(0x5ec470u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ec7a0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ec7a8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5ec7a4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec7a0u32, v); }
  if !cpu.zf { pc = 0x407265; continue; }
  pc = 0x40720b; continue;
 }
 0x40720b => {
  { let v: u32 = cpu.r32(0x5ec7b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ec7a0u32, v); }
  pc = 0x407265; continue;
 }
 0x407219 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ec470u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ec470u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ec7b0u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ec7b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec7a0u32, v); }
  pc = 0x407265; continue;
 }
 0x407265 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40727c; continue; }
  pc = 0x40726d; continue;
 }
 0x40726d => {
  { let v: u32 = cpu.r32(0x5e2454u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x407279); f_00404420(cpu); if cpu.returned_to != 0x407279 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40727c; continue;
 }
 0x40727c => {
  { let v: u32 = cpu.and32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x4072c2; continue; }
  pc = 0x407281; continue;
 }
 0x407281 => {
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x5ec4a0u32; cpu.push(v); }
  { let v = 0x5ec4a0u32; cpu.push(v); }
  cpu.push(0x407295); f_0040c240(cpu); if cpu.returned_to != 0x407295 { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4072b8; continue; }
  pc = 0x4072a0; continue;
 }
 0x4072a0 => {
  { let v: u32 = cpu.r32(0x5ddbf8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4072ab); f_004076e0(cpu); if cpu.returned_to != 0x4072ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x4072c2; continue;
 }
 0x4072b8 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x4072bf); f_004076e0(cpu); if cpu.returned_to != 0x4072bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4072c2; continue;
 }
 0x4072c2 => {
  { let v: u32 = cpu.r32(0x5ddbecu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407314; continue; }
  pc = 0x4072cb; continue;
 }
 0x4072cb => {
  { let v: u32 = cpu.r32(0x5ddbe4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407314; continue; }
  pc = 0x4072d4; continue;
 }
 0x4072d4 => {
  cpu.push(0x4072d9); f_00407bf0(cpu); if cpu.returned_to != 0x4072d9 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x4072e0); f_00407be0(cpu); if cpu.returned_to != 0x4072e0 { return; }
  { let v: u32 = cpu.r32(0x5ddbe4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { pc = 0x407314; continue; }
  pc = 0x4072ec; continue;
 }
 0x4072ec => {
  { let v: u32 = cpu.r32(0x5ddbf0u32); cpu.eax = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if !cpu.zf { pc = 0x407302; continue; }
  pc = 0x4072fb; continue;
 }
 0x4072fb => {
  { let v: u32 = 0x3f3f3f3fu32; cpu.eax = v; }
  pc = 0x407304; continue;
 }
 0x407302 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x407304; continue;
 }
 0x407304 => {
  { let v: u32 = 0x5ec4a0u32; cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.stos(4, true); }
  cpu.push(0x407311); f_004076e0(cpu); if cpu.returned_to != 0x407311 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407314; continue;
 }
 0x407314 => {
  { let v: u8 = cpu.r8(0x5ec7c1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x407326; continue; }
  pc = 0x40731d; continue;
 }
 0x40731d => {
  { let v: u32 = cpu.r32(0x4aaf28u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407359; continue; }
  pc = 0x407326; continue;
 }
 0x407326 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbf8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbe4u32, v); }
  { let v: u32 = 0x30u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddbd4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddbecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbf0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddbfcu32, v); }
  pc = 0x407359; continue;
 }
 0x407359 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40736e; continue; }
  pc = 0x407361; continue;
 }
 0x407361 => {
  cpu.push(0x407366); f_004074b0(cpu); if cpu.returned_to != 0x407366 { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x407493; continue; }
  pc = 0x40736e; continue;
 }
 0x40736e => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x407379; continue; }
  pc = 0x407372; continue;
 }
 0x407372 => {
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.esp)), 0x4u8); }
  if !cpu.zf { pc = 0x407395; continue; }
  pc = 0x407379; continue;
 }
 0x407379 => {
  cpu.push(0x40737e); f_004074b0(cpu); if cpu.returned_to != 0x40737e { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x40738b; continue; }
  pc = 0x407382; continue;
 }
 0x407382 => {
  cpu.push(0x407387); f_004074b0(cpu); if cpu.returned_to != 0x407387 { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x407382; continue; }
  pc = 0x40738b; continue;
 }
 0x40738b => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x5ddbe0u32)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x407395; continue;
 }
 0x407395 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x4073a2); f_00407f70(cpu); if cpu.returned_to != 0x4073a2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4073cd; continue; }
  pc = 0x4073a6; continue;
 }
 0x4073a6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4073cd; continue; }
  pc = 0x4073ae; continue;
 }
 0x4073ae => {
  cpu.push(0x4073b3); f_00407740(cpu); if cpu.returned_to != 0x4073b3 { return; }
  cpu.push(0x4073b8); f_00407cb0(cpu); if cpu.returned_to != 0x4073b8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x4071b3; continue; }
  pc = 0x4073c0; continue;
 }
 0x4073c0 => {
  cpu.push(0x4073c5); f_00407f50(cpu); if cpu.returned_to != 0x4073c5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4073cd => {
  { let v: u32 = cpu.r32(0x5ddbe4u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40740f; continue; }
  pc = 0x4073d7; continue;
 }
 0x4073d7 => {
  { let v: u32 = cpu.r32(0x5ddbf0u32); cpu.eax = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if !cpu.zf { pc = 0x4073ed; continue; }
  pc = 0x4073e6; continue;
 }
 0x4073e6 => {
  { let v: u32 = 0x3f3f3f3fu32; cpu.eax = v; }
  pc = 0x4073ef; continue;
 }
 0x4073ed => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4073ef; continue;
 }
 0x4073ef => {
  { let v: u32 = 0x5ec4a0u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.stos(4, true); }
  cpu.push(0x4073fd); f_004076e0(cpu); if cpu.returned_to != 0x4073fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x407405); f_00407b50(cpu); if cpu.returned_to != 0x407405 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40740f => {
  { let v: u32 = cpu.r32(0x5ddbecu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4074a0; continue; }
  pc = 0x40741c; continue;
 }
 0x40741c => {
  { let v: u32 = cpu.r32(0x5ddbf0u32); cpu.eax = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if !cpu.zf { pc = 0x407432; continue; }
  pc = 0x40742b; continue;
 }
 0x40742b => {
  { let v: u32 = 0x3f3f3f3fu32; cpu.eax = v; }
  pc = 0x407434; continue;
 }
 0x407432 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x407434; continue;
 }
 0x407434 => {
  { let v: u32 = 0x5ec4a0u32; cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.stos(4, true); }
  cpu.push(0x407441); f_004076e0(cpu); if cpu.returned_to != 0x407441 { return; }
  { let v: u32 = cpu.r32(0x5ddbe4u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x4074a0; continue; }
  pc = 0x407451; continue;
 }
 0x407451 => {
  cpu.push(0x407456); f_00407740(cpu); if cpu.returned_to != 0x407456 { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407467; continue; }
  pc = 0x40745e; continue;
 }
 0x40745e => {
  cpu.push(0x407463); f_004074b0(cpu); if cpu.returned_to != 0x407463 { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x407493; continue; }
  pc = 0x407467; continue;
 }
 0x407467 => {
  cpu.push(0x40746c); f_004074b0(cpu); if cpu.returned_to != 0x40746c { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x407479; continue; }
  pc = 0x407470; continue;
 }
 0x407470 => {
  cpu.push(0x407475); f_004074b0(cpu); if cpu.returned_to != 0x407475 { return; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x407470; continue; }
  pc = 0x407479; continue;
 }
 0x407479 => {
  { let v: u32 = cpu.r32(0x5ddbe0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.sf { cpu.poll(); pc = 0x407451; continue; }
  pc = 0x407484; continue;
 }
 0x407484 => {
  cpu.push(0x407489); f_00407b50(cpu); if cpu.returned_to != 0x407489 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407493 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4074a0 => {
  cpu.push(0x4074a5); f_00407b50(cpu); if cpu.returned_to != 0x4074a5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004074b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4074b0); } r_004074b0(cpu, 0x4074b0); }
pub fn f_004074cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4074cc); } r_004074b0(cpu, 0x4074cc); }
fn r_004074b0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4074b0 => {
  pc = 0x4074c0; continue;
 }
 0x4074c0 => {
  { let v: u32 = cpu.r32(0x5ea27cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ddc00u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x5ddbe8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x10624dd3u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 31u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4074cc => {
  { let v: u32 = cpu.ror32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 175u32); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ddbe8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x10624dd3u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 31u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407500); }
 let mut pc: u32 = 0x407500;
 loop { match pc {
 0x407500 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40750a); f_00402400(cpu); if cpu.returned_to != 0x40750a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407510); }
 let mut pc: u32 = 0x407510;
 loop { match pc {
 0x407510 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40751a); f_00402420(cpu); if cpu.returned_to != 0x40751a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407520); }
 let mut pc: u32 = 0x407520;
 loop { match pc {
 0x407520 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5df700u32); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x4075f0; continue; }
  pc = 0x407535; continue;
 }
 0x407535 => {
  { let v: u32 = cpu.r32(0x5df388u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407548; continue; }
  pc = 0x40753e; continue;
 }
 0x40753e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x58u32.wrapping_add(cpu.ecx)); cpu.push(0x407548); cpu.call(t); if cpu.returned_to != 0x407548 { return; }
  pc = 0x407548; continue;
 }
 0x407548 => {
  { let v = 0x5ddf30u32; cpu.push(v); }
  cpu.push(0x407552); f_004049d0(cpu); if cpu.returned_to != 0x407552 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x6cu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.push(0x40756f); cpu.call(t); if cpu.returned_to != 0x40756f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4075f0; continue; }
  pc = 0x407573; continue;
 }
 0x407573 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.esi = v; }
  pc = 0x40758d; continue;
 }
 0x40758d => {
  { let v: u32 = cpu.r32(0x5df6c4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x0u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x4075ca; continue; }
  pc = 0x407598; continue;
 }
 0x407598 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x4075bd; continue; }
  pc = 0x40759b; continue;
 }
 0x40759b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x4075d8; continue; }
  pc = 0x40759e; continue;
 }
 0x40759e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4075b2); f_00421cdb(cpu); if cpu.returned_to != 0x4075b2 { return; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  pc = 0x4075d8; continue;
 }
 0x4075bd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4075c5); f_00421d00(cpu); if cpu.returned_to != 0x4075c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x4075d4; continue;
 }
 0x4075ca => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4075d1); f_00421c7c(cpu); if cpu.returned_to != 0x4075d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4075d4; continue;
 }
 0x4075d4 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x4075d8; continue;
 }
 0x4075d8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40758d; continue; }
  pc = 0x4075e4; continue;
 }
 0x4075e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.push(0x4075f0); cpu.call(t); if cpu.returned_to != 0x4075f0 { return; }
  pc = 0x4075f0; continue;
 }
 0x4075f0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407600); }
 let mut pc: u32 = 0x407600;
 loop { match pc {
 0x407600 => {
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.esp)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407629; continue; }
  pc = 0x407621; continue;
 }
 0x407621 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x407629; continue;
 }
 0x407629 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407630); }
 let mut pc: u32 = 0x407630;
 loop { match pc {
 0x407630 => {
  cpu.push(0x407635); f_00402d10(cpu); if cpu.returned_to != 0x407635 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x40763c); f_00402b00(cpu); if cpu.returned_to != 0x40763c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x407644); f_00411e40(cpu); if cpu.returned_to != 0x407644 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40764d; continue; }
  pc = 0x407648; continue;
 }
 0x407648 => {
  f_0041ac57(cpu); return;
 }
 0x40764d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407650); }
 let mut pc: u32 = 0x407650;
 loop { match pc {
 0x407650 => {
  cpu.push(0x407655); f_00407630(cpu); if cpu.returned_to != 0x407655 { return; }
  { let v: u32 = cpu.r32(0x5ea260u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407660); f_00407520(cpu); if cpu.returned_to != 0x407660 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407670); }
 let mut pc: u32 = 0x407670;
 loop { match pc {
 0x407670 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2448u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x140u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e244cu32, v); }
  if !cpu.zf { pc = 0x4076d3; continue; }
  pc = 0x40768e; continue;
 }
 0x40768e => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0xc8u32); }
  if !cpu.zf { pc = 0x4076d3; continue; }
  pc = 0x407698; continue;
 }
 0x407698 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.w8(0x4ab14cu32, v); }
  { let v: u32 = 0x19u32; cpu.edi = v; }
  pc = 0x4076a6; continue;
 }
 0x4076a6 => {
  { let v: u32 = 0x28u32; cpu.esi = v; }
  pc = 0x4076ab; continue;
 }
 0x4076ab => {
  cpu.push(0x4076b0); f_004212c5(cpu); if cpu.returned_to != 0x4076b0 { return; }
  { let v: u32 = cpu.r32(0x5e244cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e244cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4076ab; continue; }
  pc = 0x4076c2; continue;
 }
 0x4076c2 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x8c0u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e244cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4076a6; continue; }
  pc = 0x4076d1; continue;
 }
 0x4076d1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4076d3; continue;
 }
 0x4076d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004076e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4076e0); }
 let mut pc: u32 = 0x4076e0;
 loop { match pc {
 0x4076e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  if !cpu.zf { pc = 0x407706; continue; }
  pc = 0x4076f1; continue;
 }
 0x4076f1 => {
  { let v: u32 = 0x5ec4a0u32; cpu.esi = v; }
  { let v: u32 = 0x5ddf30u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ddc10u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407706 => {
  { let v: u32 = 0x5ddf30u32; cpu.esi = v; }
  { let v: u32 = 0x5ddc28u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = 0x5ec4a0u32; cpu.esi = v; }
  { let v: u32 = 0x5de240u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ddc20u32, v); }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc10u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407740); }
 let mut pc: u32 = 0x407740;
 loop { match pc {
 0x407740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ddc10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x407798; continue; }
  pc = 0x40774e; continue;
 }
 0x40774e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40778e; continue; }
  pc = 0x407752; continue;
 }
 0x407752 => {
  { let v: u32 = cpu.r32(0x5ddc20u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x40775a; continue;
 }
 0x40775a => {
  { let v: u8 = cpu.r8(0x5ddc28u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x5de240u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u8) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.esi); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5ddf30u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x300u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40775a; continue; }
  pc = 0x407783; continue;
 }
 0x407783 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddc20u32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407798; continue; }
  pc = 0x40778e; continue;
 }
 0x40778e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5ddc10u32, v); }
  pc = 0x407798; continue;
 }
 0x407798 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004077a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4077a0); }
 let mut pc: u32 = 0x4077a0;
 loop { match pc {
 0x4077a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4ab158u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5de238u32, v); }
  cpu.push(0x4077bc); f_0048cf60(cpu); if cpu.returned_to != 0x4077bc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2450u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4077d6; continue; }
  pc = 0x4077c8; continue;
 }
 0x4077c8 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4077d6 => {
  { let v = 0x68u32; cpu.push(v); }
  { let v = 0x5de548u32; cpu.push(v); }
  cpu.push(0x4077e2); f_00407a40(cpu); if cpu.returned_to != 0x4077e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407a29; continue; }
  pc = 0x4077ed; continue;
 }
 0x4077ed => {
  { cpu.sub8(cpu.r8(0x5de548u32), 0x4au8); }
  if !cpu.zf { pc = 0x407a29; continue; }
  pc = 0x4077fa; continue;
 }
 0x4077fa => {
  { cpu.sub8(cpu.r8(0x5de549u32), 0x56u8); }
  if !cpu.zf { pc = 0x407a29; continue; }
  pc = 0x407807; continue;
 }
 0x407807 => {
  { let v: u32 = cpu.r16(0x5de59cu32) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2444u32, v); }
  cpu.push(0x40781c); f_00407500(cpu); if cpu.returned_to != 0x40781c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc1cu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40783d; continue; }
  pc = 0x407828; continue;
 }
 0x407828 => {
  cpu.push(0x40782d); f_00407b50(cpu); if cpu.returned_to != 0x40782d { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40783d => {
  { let v: u32 = cpu.r16(0x5de59cu32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40784e); f_00407a40(cpu); if cpu.returned_to != 0x40784e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40786a; continue; }
  pc = 0x407855; continue;
 }
 0x407855 => {
  cpu.push(0x40785a); f_00407b50(cpu); if cpu.returned_to != 0x40785a { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40786a => {
  { let v: u32 = cpu.r16(0x5de59cu32) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5de5b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de230u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de540u32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4078d5; continue; }
  pc = 0x407891; continue;
 }
 0x407891 => {
  { let v: u32 = cpu.r32(0x5ddc1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x40789b; continue;
 }
 0x40789b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de230u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4078b5; continue; }
  pc = 0x4078ab; continue;
 }
 0x4078ab => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4078b5; continue; }
  pc = 0x4078b3; continue;
 }
 0x4078b3 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x4078b5; continue;
 }
 0x4078b5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x4078c7; continue; }
  pc = 0x4078b9; continue;
 }
 0x4078b9 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4078b9; continue; }
  pc = 0x4078c1; continue;
 }
 0x4078c1 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5de540u32, v); }
  pc = 0x4078c7; continue;
 }
 0x4078c7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40789b; continue; }
  pc = 0x4078d5; continue;
 }
 0x4078d5 => {
  { let v: u32 = cpu.r32(0x5e2444u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de540u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5e2444u32, v); }
  cpu.push(0x4078f1); f_00407500(cpu); if cpu.returned_to != 0x4078f1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc14u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407912; continue; }
  pc = 0x4078fd; continue;
 }
 0x4078fd => {
  cpu.push(0x407902); f_00407b50(cpu); if cpu.returned_to != 0x407902 { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407912 => {
  cpu.push(0x407917); f_00407a80(cpu); if cpu.returned_to != 0x407917 { return; }
  { let v: u32 = cpu.r32(0x5e2444u32); cpu.eax = v; }
  { let v = 0x40000u32; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x40000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2444u32, v); }
  cpu.push(0x407930); f_00404350(cpu); if cpu.returned_to != 0x407930 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2454u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40793e); f_004042f0(cpu); if cpu.returned_to != 0x40793e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec7b0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40795f; continue; }
  pc = 0x40794a; continue;
 }
 0x40794a => {
  cpu.push(0x40794f); f_00407b50(cpu); if cpu.returned_to != 0x40794f { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40795f => {
  { let v: u32 = cpu.r32(0x5e2444u32); cpu.edi = v; }
  { let v: u32 = 0x40000u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec7acu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ec7a0u32, v); }
  { let v: u32 = cpu.r32(0x5de5a0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec7a4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5e2444u32, v); }
  cpu.push(0x40798e); f_00407500(cpu); if cpu.returned_to != 0x40798e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddf2cu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4079af; continue; }
  pc = 0x40799a; continue;
 }
 0x40799a => {
  cpu.push(0x40799f); f_00407b50(cpu); if cpu.returned_to != 0x40799f { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4079af => {
  { let v: u32 = cpu.r32(0x5de540u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5de230u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4079c0; continue; }
  pc = 0x4079be; continue;
 }
 0x4079be => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x4079c0; continue;
 }
 0x4079c0 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4079c6); f_00407aa0(cpu); if cpu.returned_to != 0x4079c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4079d8; continue; }
  pc = 0x4079cd; continue;
 }
 0x4079cd => {
  cpu.push(0x4079d2); f_00407b50(cpu); if cpu.returned_to != 0x4079d2 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4079d8 => {
  { let v: u32 = cpu.r32(0x5e2450u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddc18u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddc0cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5b4u32, v); }
  if cpu.zf { pc = 0x407a20; continue; }
  pc = 0x4079ff; continue;
 }
 0x4079ff => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407a20; continue; }
  pc = 0x407a03; continue;
 }
 0x407a03 => {
  { let v: u32 = cpu.r16(0x5de598u32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x5de59au32) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407a1d); f_00407600(cpu); if cpu.returned_to != 0x407a1d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x407a20; continue;
 }
 0x407a20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407a29 => {
  cpu.push(0x407a2e); f_00407b50(cpu); if cpu.returned_to != 0x407a2e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407a40); }
 let mut pc: u32 = 0x407a40;
 loop { match pc {
 0x407a40 => {
  { let v: u32 = cpu.r32(0x5e2450u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x407a58); f_0048d270(cpu); if cpu.returned_to != 0x407a58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407a63; continue; }
  pc = 0x407a5f; continue;
 }
 0x407a5f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407a63 => {
  { let v: u32 = cpu.r32(0x5de238u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de238u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407a80); }
 let mut pc: u32 = 0x407a80;
 loop { match pc {
 0x407a80 => {
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5ddf28u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de234u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc08u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407aa0); }
 let mut pc: u32 = 0x407aa0;
 loop { match pc {
 0x407aa0 => {
  { let v: u32 = cpu.r32(0x5de230u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407ab1; continue; }
  pc = 0x407aaf; continue;
 }
 0x407aaf => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x407ab1; continue;
 }
 0x407ab1 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x407b47; continue; }
  pc = 0x407ab9; continue;
 }
 0x407ab9 => {
  { let v: u32 = cpu.r32(0x5ddf28u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5de540u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x407b47; continue; }
  pc = 0x407aca; continue;
 }
 0x407aca => {
  { let v: u32 = cpu.r32(0x5ddc08u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x407aee; continue; }
  pc = 0x407adf; continue;
 }
 0x407adf => {
  { let v: u32 = 0x7u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407aee => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x407af5); f_00407a40(cpu); if cpu.returned_to != 0x407af5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407b09; continue; }
  pc = 0x407afc; continue;
 }
 0x407afc => {
  { let v: u32 = 0x5u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407b09 => {
  { let v: u32 = cpu.r32(0x5ddc08u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5de540u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc08u32, v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x407b2d; continue; }
  pc = 0x407b27; continue;
 }
 0x407b27 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddc08u32, v); }
  pc = 0x407b2d; continue;
 }
 0x407b2d => {
  { let v: u32 = cpu.r32(0x5ddf28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5de230u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddf28u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de230u32, v); }
  pc = 0x407b47; continue;
 }
 0x407b47 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407b50); }
 let mut pc: u32 = 0x407b50;
 loop { match pc {
 0x407b50 => {
  { let v: u32 = cpu.r32(0x5ec7b0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407b6a; continue; }
  pc = 0x407b5c; continue;
 }
 0x407b5c => {
  { let v: u32 = cpu.r32(0x5e2454u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407b67); f_00404400(cpu); if cpu.returned_to != 0x407b67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407b6a; continue;
 }
 0x407b6a => {
  { let v: u32 = cpu.r32(0x5ddf2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407b7c; continue; }
  pc = 0x407b73; continue;
 }
 0x407b73 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407b79); f_00407510(cpu); if cpu.returned_to != 0x407b79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407b7c; continue;
 }
 0x407b7c => {
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407b8e; continue; }
  pc = 0x407b85; continue;
 }
 0x407b85 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407b8b); f_00407510(cpu); if cpu.returned_to != 0x407b8b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407b8e; continue;
 }
 0x407b8e => {
  { let v: u32 = cpu.r32(0x5ddc1cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407ba0; continue; }
  pc = 0x407b97; continue;
 }
 0x407b97 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407b9d); f_00407510(cpu); if cpu.returned_to != 0x407b9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x407ba0; continue;
 }
 0x407ba0 => {
  { let v: u32 = cpu.r32(0x5e2450u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x407bb2; continue; }
  pc = 0x407ba9; continue;
 }
 0x407ba9 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407baf); f_0048d190(cpu); if cpu.returned_to != 0x407baf { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x407bb2; continue;
 }
 0x407bb2 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5ec7b0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddf2cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddc14u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5ddc1cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5e2450u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407be0); }
 let mut pc: u32 = 0x407be0;
 loop { match pc {
 0x407be0 => {
  { let v: u32 = cpu.r32(0x5ddc18u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407bf0); }
 let mut pc: u32 = 0x407bf0;
 loop { match pc {
 0x407bf0 => {
  { let v: u32 = cpu.r16(0x5de59cu32) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407c00); }
 let mut pc: u32 = 0x407c00;
 loop { match pc {
 0x407c00 => {
  { let v: u32 = cpu.r16(0x5de59eu32) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407c10); }
 let mut pc: u32 = 0x407c10;
 loop { match pc {
 0x407c10 => {
  { let v: u32 = cpu.r32(0x5de5a4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407c20); }
 let mut pc: u32 = 0x407c20;
 loop { match pc {
 0x407c20 => {
  { let v: u8 = cpu.r8(0x5de5a8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407c30); }
 let mut pc: u32 = 0x407c30;
 loop { match pc {
 0x407c30 => {
  { let v: u8 = cpu.r8(0x5de5a8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x407c41; continue; }
  pc = 0x407c3c; continue;
 }
 0x407c3c => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  pc = 0x407c41; continue;
 }
 0x407c41 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407c50); }
 let mut pc: u32 = 0x407c50;
 loop { match pc {
 0x407c50 => {
  { let v: u8 = cpu.r8(0x5de5afu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x5de5afu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5de5afu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 6u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x5de5afu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5de54au32) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5de54bu32) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407cb0); }
 let mut pc: u32 = 0x407cb0;
 loop { match pc {
 0x407cb0 => {
  { let v: u32 = cpu.r32(0x5e2450u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407cc6; continue; }
  pc = 0x407cb9; continue;
 }
 0x407cb9 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407cc6 => {
  { let v: u32 = cpu.r32(0x5ddc0cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407cd5; continue; }
  pc = 0x407ccf; continue;
 }
 0x407ccf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407cd5 => {
  { let v: u32 = cpu.r32(0x5de5b0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407ce0); f_00407aa0(cpu); if cpu.returned_to != 0x407ce0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407cef; continue; }
  pc = 0x407ce7; continue;
 }
 0x407ce7 => {
  cpu.push(0x407cec); f_00407b50(cpu); if cpu.returned_to != 0x407cec { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407cef => {
  { let v: u32 = cpu.r32(0x5ddc18u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ddc1cu32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407d07); f_00407e90(cpu); if cpu.returned_to != 0x407d07 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x407d16; continue; }
  pc = 0x407d0e; continue;
 }
 0x407d0e => {
  cpu.push(0x407d13); f_00407b50(cpu); if cpu.returned_to != 0x407d13 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407d16 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407d1c); f_00407d50(cpu); if cpu.returned_to != 0x407d1c { return; }
  { let v: u32 = cpu.r32(0x5ddc18u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc18u32, v); }
  cpu.push(0x407d2f); f_00407650(cpu); if cpu.returned_to != 0x407d2f { return; }
  { let v: u32 = cpu.r16(0x5de59cu32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ddc18u32); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x407d4a; continue; }
  pc = 0x407d45; continue;
 }
 0x407d45 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5ddc0cu32, v); }
  pc = 0x407d4a; continue;
 }
 0x407d4a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407d50); }
 let mut pc: u32 = 0x407d50;
 loop { match pc {
 0x407d50 => {
  { let v: u32 = cpu.r32(0x5ddc18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ddc1cu32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de5bcu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407d88; continue; }
  pc = 0x407d73; continue;
 }
 0x407d73 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x407e5d; continue; }
  pc = 0x407d7e; continue;
 }
 0x407d7e => {
  { let v: u32 = 0x80u32; cpu.w32(0x5de5bcu32, v); }
  pc = 0x407d88; continue;
 }
 0x407d88 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x407d8c; continue;
 }
 0x407d8c => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x407dae; continue; }
  pc = 0x407d93; continue;
 }
 0x407d93 => {
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x5ec4a0u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x300u32); cpu.edx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x5de5bcu32) | 0x2u32; cpu.w32(0x5de5bcu32, v); }
  pc = 0x407dae; continue;
 }
 0x407dae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407e5a; continue; }
  pc = 0x407db9; continue;
 }
 0x407db9 => {
  { let v: u32 = cpu.r8(0xeu32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x0u32); cpu.eax = v; }
  if cpu.zf { pc = 0x407e39; continue; }
  pc = 0x407dc2; continue;
 }
 0x407dc2 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x407e09; continue; }
  pc = 0x407dc5; continue;
 }
 0x407dc5 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x407dd7; continue; }
  pc = 0x407dc8; continue;
 }
 0x407dc8 => {
  { let v: u32 = cpu.r32(0x5de5bcu32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5bcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407dd7 => {
  { let v: u32 = cpu.r32(0x5de5b4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407e2a; continue; }
  pc = 0x407de0; continue;
 }
 0x407de0 => {
  { let v: u32 = cpu.r32(0x4ab154u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x4ab150u32)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407df7); f_00407600(cpu); if cpu.returned_to != 0x407df7 { return; }
  { let v: u32 = cpu.r32(0x5de5bcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5bcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407e09 => {
  { let v: u32 = cpu.r32(0x5de5b4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407e2a; continue; }
  pc = 0x407e12; continue;
 }
 0x407e12 => {
  { let v: u32 = cpu.r32(0x4ab154u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4ab150u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x407e27); f_00407670(cpu); if cpu.returned_to != 0x407e27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x407e2a; continue;
 }
 0x407e2a => {
  { let v: u32 = cpu.r32(0x5de5bcu32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5bcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407e39 => {
  { let v: u32 = cpu.r32(0x5de5b4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x407e5a; continue; }
  pc = 0x407e42; continue;
 }
 0x407e42 => {
  { let v: u32 = cpu.r32(0x4ab154u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x4ab150u32)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x407e57); f_004030c0(cpu); if cpu.returned_to != 0x407e57 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x407e5a; continue;
 }
 0x407e5a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407e5d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5ec470u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ec7a8u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x5de5bcu32, v); }
  cpu.poll(); pc = 0x407d8c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407e90); }
 let mut pc: u32 = 0x407e90;
 loop { match pc {
 0x407e90 => {
  { let v: u32 = cpu.r32(0x5ddf28u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x407eac; continue; }
  pc = 0x407e9e; continue;
 }
 0x407e9e => {
  { let v: u32 = 0xau32; cpu.w32(0x5de5b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407eac => {
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5de540u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de234u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x407ee1; continue; }
  pc = 0x407ec8; continue;
 }
 0x407ec8 => {
  { let v: u32 = cpu.r32(0x5ddf28u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de234u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ddf28u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407ee1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ddf2cu32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ddf2cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x5ddf28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ddc14u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ddf28u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de234u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407f50); }
 let mut pc: u32 = 0x407f50;
 loop { match pc {
 0x407f50 => {
  { let v: u32 = cpu.r32(0x5de5b8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de5b8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407f60); }
 let mut pc: u32 = 0x407f60;
 loop { match pc {
 0x407f60 => {
  { let v: u32 = cpu.r32(0x5de5bcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de5bcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407f70); }
 let mut pc: u32 = 0x407f70;
 loop { match pc {
 0x407f70 => {
  { let v: u32 = cpu.r32(0x5ddc0cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407f80); }
 let mut pc: u32 = 0x407f80;
 loop { match pc {
 0x407f80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xd48bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x80u32; cpu.w32(0xd493u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xd48fu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xd497u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20000u32; cpu.w32(0xd49bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xd49fu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xd4a3u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407fcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x407fcc); }
 let mut pc: u32 = 0x407fcc;
 loop { match pc {
 0x407fcc => {
  {  }
  {  }
  {  }
  {  }
  f_00407fd0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00407fd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x407fd0); } r_00407fd0(cpu, 0x407fd0); }
pub fn f_00407fff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x407fff); } r_00407fd0(cpu, 0x407fff); }
fn r_00407fd0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x407fd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x407fe4; continue; }
  pc = 0x407fdf; continue;
 }
 0x407fdf => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x407fe4 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x407ffc; continue; }
  pc = 0x407fee; continue;
 }
 0x407fee => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.push(0x407ff2); cpu.call(t); if cpu.returned_to != 0x407ff2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x407ffc; continue;
 }
 0x407ffc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x407fff; continue;
 }
 0x407fff => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408000); }
 let mut pc: u32 = 0x408000;
 loop { match pc {
 0x408000 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x46894016u32.wrapping_add(cpu.ebx)) & (cpu.ecx as u8); cpu.w8(0x46894016u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.ror8(cpu.r8(0x89410a04u32.wrapping_add(cpu.edx)), 78u32); cpu.w8(0x89410a04u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xffffffc3u32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); cpu.w8(0xffffffc3u32.wrapping_add(cpu.esi), v); }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  {  }
  f_00408020(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x408020); } r_00408020(cpu, 0x408020); }
pub fn f_00408044(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x408044); } r_00408020(cpu, 0x408044); }
fn r_00408020(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x408020 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x40803d; continue; }
  pc = 0x40802f; continue;
 }
 0x40802f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.push(0x408033); cpu.call(t); if cpu.returned_to != 0x408033 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x40803d; continue;
 }
 0x40803d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x408044 => {
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408060); }
 let mut pc: u32 = 0x408060;
 loop { match pc {
 0x408060 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x408087; continue; }
  pc = 0x408079; continue;
 }
 0x408079 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.push(0x40807d); cpu.call(t); if cpu.returned_to != 0x40807d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x408087; continue;
 }
 0x408087 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4080cd; continue; }
  pc = 0x4080a4; continue;
 }
 0x4080a4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x4080ad; continue;
 }
 0x4080ad => {
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x38u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.edx, 0xfffu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4080ad; continue; }
  pc = 0x4080c9; continue;
 }
 0x4080c9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4080cd => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004080e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4080e0); }
 let mut pc: u32 = 0x4080e0;
 loop { match pc {
 0x4080e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x3fbu32; cpu.ecx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x20202020u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { cpu.stos(2, false); }
  { let v: u32 = 0xfeeu32; cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x408100; continue;
 }
 0x408100 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x408122; continue; }
  pc = 0x40810b; continue;
 }
 0x40810b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x408111); f_00407fd0(cpu); if cpu.returned_to != 0x408111 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x408195; continue; }
  pc = 0x408119; continue;
 }
 0x408119 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0xffu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x408122; continue;
 }
 0x408122 => {
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x408152; continue; }
  pc = 0x408128; continue;
 }
 0x408128 => {
  cpu.push(0x40812d); f_00407fd0(cpu); if cpu.returned_to != 0x40812d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x408195; continue; }
  pc = 0x408137; continue;
 }
 0x408137 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40813e); f_00408020(cpu); if cpu.returned_to != 0x40813e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x38u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xfffu32; cpu.edi = v; }
  cpu.poll(); pc = 0x408100; continue;
 }
 0x408152 => {
  cpu.push(0x408157); f_00407fd0(cpu); if cpu.returned_to != 0x408157 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x408195; continue; }
  pc = 0x408161; continue;
 }
 0x408161 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x408167); f_00407fd0(cpu); if cpu.returned_to != 0x408167 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x408195; continue; }
  pc = 0x40816f; continue;
 }
 0x40816f => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebp | cpu.ecx; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40818b); f_00408060(cpu); if cpu.returned_to != 0x40818b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.poll(); pc = 0x408100; continue;
 }
 0x408195 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004081a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4081a0); }
 let mut pc: u32 = 0x4081a0;
 loop { match pc {
 0x4081a0 => {
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4081d0; continue; }
  pc = 0x4081a9; continue;
 }
 0x4081a9 => {
  { let v = 0x5de5e8u32; cpu.push(v); }
  { let v = 0x5de698u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xf003fu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4ab164u32; cpu.push(v); }
  { let v = 0x80000002u32; cpu.push(v); }
  let t = cpu.r32(0x4a100cu32); cpu.push(0x4081d0); cpu.call(t); if cpu.returned_to != 0x4081d0 { return; }
  pc = 0x4081d0; continue;
 }
 0x4081d0 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x408730u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x9fu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11c0u32); cpu.push(0x4081ed); cpu.call(t); if cpu.returned_to != 0x4081ed { return; }
  { let v: u32 = cpu.r32(0x5de694u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408200); }
 let mut pc: u32 = 0x408200;
 loop { match pc {
 0x408200 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x408a90u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xa4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11c0u32); cpu.push(0x40821d); cpu.call(t); if cpu.returned_to != 0x40821d { return; }
  { let v: u32 = cpu.r32(0x5de694u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408230); }
 let mut pc: u32 = 0x408230;
 loop { match pc {
 0x408230 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4096b0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xa1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5de5ecu32, v); }
  let t = cpu.r32(0x4a11c0u32); cpu.push(0x408257); cpu.call(t); if cpu.returned_to != 0x408257 { return; }
  { let v: u32 = cpu.r32(0x5de694u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408260); }
 let mut pc: u32 = 0x408260;
 loop { match pc {
 0x408260 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x408c50u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xa0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11c0u32); cpu.push(0x40827d); cpu.call(t); if cpu.returned_to != 0x40827d { return; }
  { let v: u32 = cpu.r32(0x5de694u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408290); }
 let mut pc: u32 = 0x408290;
 loop { match pc {
 0x408290 => {
  { let v: u32 = cpu.r32(0x5eb430u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4082ac; continue; }
  pc = 0x408299; continue;
 }
 0x408299 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40829f); f_004038a0(cpu); if cpu.returned_to != 0x40829f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eb430u32, v); }
  pc = 0x4082ac; continue;
 }
 0x4082ac => {
  f_00403750(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004082c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4082c0); }
 let mut pc: u32 = 0x4082c0;
 loop { match pc {
 0x4082c0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xf003fu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4a1330u32; cpu.push(v); }
  { let v = 0x80000002u32; cpu.push(v); }
  let t = cpu.r32(0x4a1004u32); cpu.push(0x4082dd); cpu.call(t); if cpu.returned_to != 0x4082dd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408306; continue; }
  pc = 0x4082e1; continue;
 }
 0x4082e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4ab15cu32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab190u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a1010u32); cpu.push(0x4082fb); cpu.call(t); if cpu.returned_to != 0x4082fb { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a1008u32); cpu.push(0x408306); cpu.call(t); if cpu.returned_to != 0x408306 { return; }
  pc = 0x408306; continue;
 }
 0x408306 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408310); }
 let mut pc: u32 = 0x408310;
 loop { match pc {
 0x408310 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x4ab15cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4a1330u32; cpu.push(v); }
  { let v = 0x80000002u32; cpu.push(v); }
  let t = cpu.r32(0x4a1004u32); cpu.push(0x408336); cpu.call(t); if cpu.returned_to != 0x408336 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408369; continue; }
  pc = 0x40833a; continue;
 }
 0x40833a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4ab15cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4ab190u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a1000u32); cpu.push(0x40835e); cpu.call(t); if cpu.returned_to != 0x40835e { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1008u32); cpu.push(0x408369); cpu.call(t); if cpu.returned_to != 0x408369 { return; }
  pc = 0x408369; continue;
 }
 0x408369 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408370); }
 let mut pc: u32 = 0x408370;
 loop { match pc {
 0x408370 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x408379); f_00408310(cpu); if cpu.returned_to != 0x408379 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x408383); f_004081a0(cpu); if cpu.returned_to != 0x408383 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4085bc; continue; }
  pc = 0x40838e; continue;
 }
 0x40838e => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if cpu.zf { pc = 0x4083f3; continue; }
  pc = 0x408393; continue;
 }
 0x408393 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x408399); f_00408200(cpu); if cpu.returned_to != 0x408399 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x4083f3; continue; }
  pc = 0x4083a1; continue;
 }
 0x4083a1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4085bc; continue; }
  pc = 0x4083a9; continue;
 }
 0x4083a9 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x4083c7; continue; }
  pc = 0x4083ae; continue;
 }
 0x4083ae => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4083b4); f_00408260(cpu); if cpu.returned_to != 0x4083b4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408405; continue; }
  pc = 0x4083bb; continue;
 }
 0x4083bb => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x4083e0; continue; }
  pc = 0x4083c0; continue;
 }
 0x4083c0 => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if !cpu.zf { pc = 0x40840f; continue; }
  pc = 0x4083c5; continue;
 }
 0x4083c5 => {
  cpu.poll(); pc = 0x4083ae; continue;
 }
 0x4083c7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4083cd); f_00408230(cpu); if cpu.returned_to != 0x4083cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408405; continue; }
  pc = 0x4083d4; continue;
 }
 0x4083d4 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x4083e0; continue; }
  pc = 0x4083d9; continue;
 }
 0x4083d9 => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if !cpu.zf { pc = 0x40840f; continue; }
  pc = 0x4083de; continue;
 }
 0x4083de => {
  cpu.poll(); pc = 0x4083c7; continue;
 }
 0x4083e0 => {
  cpu.push(0x4083e5); f_00408290(cpu); if cpu.returned_to != 0x4083e5 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4083eb); f_00408200(cpu); if cpu.returned_to != 0x4083eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x4083a1; continue; }
  pc = 0x4083f3; continue;
 }
 0x4083f3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4083f9); f_004081a0(cpu); if cpu.returned_to != 0x4083f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40838e; continue; }
  pc = 0x408400; continue;
 }
 0x408400 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x408405 => {
  cpu.push(0x40840a); f_00408290(cpu); if cpu.returned_to != 0x40840a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40840f => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a11c0u32); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x40ffb0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x85u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x40842a); cpu.call(t); if cpu.returned_to != 0x40842a { return; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x4085b2; continue; }
  pc = 0x408437; continue;
 }
 0x408437 => {
  { let v: u32 = cpu.r32(0x575d16u32); cpu.ecx = v; }
  { let v: u32 = 0x432u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.r32(0x5ebda0u32); cpu.ecx = v; }
  { let v: u32 = 0x3fdu32; cpu.w32(0x5df6c0u32, v); }
  { let v: u32 = 0x435u32; cpu.w32(0x5df748u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5df39cu32, v); }
  if cpu.zf { pc = 0x4084a3; continue; }
  pc = 0x408465; continue;
 }
 0x408465 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x40847f; continue; }
  pc = 0x40846a; continue;
 }
 0x40846a => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4102f0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xaeu32; cpu.push(v); }
  pc = 0x4084d9; continue;
 }
 0x40847f => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4102f0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5df6c0u32, v); }
  { let v: u32 = 0x436u32; cpu.w32(0x5df748u32, v); }
  { let v = 0xaeu32; cpu.push(v); }
  pc = 0x4084d9; continue;
 }
 0x4084a3 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x4084b7; continue; }
  pc = 0x4084a8; continue;
 }
 0x4084a8 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x40eb30u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x4084d4; continue;
 }
 0x4084b7 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x40eb30u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5df6c0u32, v); }
  { let v: u32 = 0x436u32; cpu.w32(0x5df748u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4084d4; continue;
 }
 0x4084d4 => {
  { let v = 0xadu32; cpu.push(v); }
  pc = 0x4084d9; continue;
 }
 0x4084d9 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4084dc); cpu.call(t); if cpu.returned_to != 0x4084dc { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x4085b2; continue; }
  pc = 0x4084e4; continue;
 }
 0x4084e4 => {
  { let v: u32 = cpu.r32(0x4a1268u32); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x5de980u32, v); }
  let t = cpu.esi; cpu.push(0x4084f8); cpu.call(t); if cpu.returned_to != 0x4084f8 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4d67dau32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x56c9dau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5978c6u32, v); }
  cpu.push(0x408511); f_004107f0(cpu); if cpu.returned_to != 0x408511 { return; }
  { let v: u32 = cpu.r32(0x5ebda0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5df70cu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5def60u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5df70cu32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e155au32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x59805cu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x59440du32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x581b6fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594435u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67f6u32, v); }
  cpu.push(0x408551); f_004155ef(cpu); if cpu.returned_to != 0x408551 { return; }
  cpu.push(0x408556); f_00421e68(cpu); if cpu.returned_to != 0x408556 { return; }
  cpu.push(0x40855b); f_00410cb0(cpu); if cpu.returned_to != 0x40855b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408576; continue; }
  pc = 0x40855f; continue;
 }
 0x40855f => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de980u32, v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408576); cpu.call(t); if cpu.returned_to != 0x408576 { return; }
  pc = 0x408576; continue;
 }
 0x408576 => {
  { let v: u32 = cpu.edi; cpu.w32(0x5def60u32, v); }
  cpu.push(0x408581); f_004107f0(cpu); if cpu.returned_to != 0x408581 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5df70cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e155au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59805cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59440du32, v); }
  let t = cpu.esi; cpu.push(0x40859f); cpu.call(t); if cpu.returned_to != 0x40859f { return; }
  { let v: u32 = cpu.r32(0x5df72cu32); cpu.eax = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4085b2; continue; }
  pc = 0x4085a9; continue;
 }
 0x4085a9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4085af); f_0040e4a0(cpu); if cpu.returned_to != 0x4085af { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4085b2; continue;
 }
 0x4085b2 => {
  cpu.push(0x4085b7); f_00408290(cpu); if cpu.returned_to != 0x4085b7 { return; }
  cpu.push(0x4085bc); f_00403a00(cpu); if cpu.returned_to != 0x4085bc { return; }
  pc = 0x4085bc; continue;
 }
 0x4085bc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004085d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4085d0); }
 let mut pc: u32 = 0x4085d0;
 loop { match pc {
 0x4085d0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1010u32); cpu.push(0x4085ef); cpu.call(t); if cpu.returned_to != 0x4085ef { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004085f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4085f0); }
 let mut pc: u32 = 0x4085f0;
 loop { match pc {
 0x4085f0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1010u32); cpu.push(0x40860f); cpu.call(t); if cpu.returned_to != 0x40860f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408610); }
 let mut pc: u32 = 0x408610;
 loop { match pc {
 0x408610 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a1000u32); cpu.push(0x408633); cpu.call(t); if cpu.returned_to != 0x408633 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408640); }
 let mut pc: u32 = 0x408640;
 loop { match pc {
 0x408640 => {
  { let v: u32 = cpu.r32(0x5e18bcu32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18bcu32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5ebdcfu32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), 0x1u8); }
  if cpu.zf { pc = 0x4086c1; continue; }
  pc = 0x408666; continue;
 }
 0x408666 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x180u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x40867f); cpu.call(t); if cpu.returned_to != 0x40867f { return; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x408688); f_00402400(cpu); if cpu.returned_to != 0x408688 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408696; continue; }
  pc = 0x40868f; continue;
 }
 0x40868f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x408696 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  if cpu.zf { pc = 0x4086c1; continue; }
  pc = 0x4086b7; continue;
 }
 0x4086b7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x19au32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x4086c1); cpu.call(t); if cpu.returned_to != 0x4086c1 { return; }
  pc = 0x4086c1; continue;
 }
 0x4086c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004086d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4086d0); }
 let mut pc: u32 = 0x4086d0;
 loop { match pc {
 0x4086d0 => {
  { let v: u32 = cpu.r32(0x5de5c0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408718; continue; }
  pc = 0x4086dc; continue;
 }
 0x4086dc => {
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x18bu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4086ee); cpu.call(t); if cpu.returned_to != 0x4086ee { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x408718; continue; }
  pc = 0x4086f6; continue;
 }
 0x4086f6 => {
  { let v: u32 = cpu.r32(0x5de5c0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x199u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408706); cpu.call(t); if cpu.returned_to != 0x408706 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408713; continue; }
  pc = 0x40870a; continue;
 }
 0x40870a => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x408710); f_00402420(cpu); if cpu.returned_to != 0x408710 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x408713; continue;
 }
 0x408713 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4086f6; continue; }
  pc = 0x408718; continue;
 }
 0x408718 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de5c0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408730(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x408730); } r_00408730(cpu, 0x408730); }
pub fn f_00408748(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x408748); } r_00408730(cpu, 0x408748); }
pub fn f_0040874c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40874c); } r_00408730(cpu, 0x40874c); }
pub fn f_004087ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4087ff); } r_00408730(cpu, 0x4087ff); }
fn r_00408730(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x408730 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x7cu32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x110u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x40893a; continue; }
  pc = 0x408746; continue;
 }
 0x408746 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x408793; continue; }
  pc = 0x408749; continue;
 }
 0x408748 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x408749; continue;
 }
 0x408749 => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x408784; continue; }
  pc = 0x40875a; continue;
 }
 0x40874c => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x10e9c1c8u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x10e9c1c8u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x408784; continue; }
  pc = 0x40875a; continue;
 }
 0x40875a => {
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x188u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408770); cpu.call(t); if cpu.returned_to != 0x408770 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5c4u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408784 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x3feu32); cpu.eax = v; }
  if cpu.zf { pc = 0x4087e5; continue; }
  pc = 0x408790; continue;
 }
 0x408790 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40879f; continue; }
  pc = 0x408793; continue;
 }
 0x408793 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x40879f => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de694u32, v); }
  cpu.push(0x4087ae); f_004086d0(cpu); if cpu.returned_to != 0x4087ae { return; }
  cpu.push(0x4087b3); f_00403a00(cpu); if cpu.returned_to != 0x4087b3 { return; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x4087c3); cpu.call(t); if cpu.returned_to != 0x4087c3 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x4087d6); cpu.call(t); if cpu.returned_to != 0x4087d6 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x4087e5 => {
  cpu.push(0x4087ea); f_00403a00(cpu); if cpu.returned_to != 0x4087ea { return; }
  { let v: u32 = cpu.r32(0x5de5c4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x408903; continue; }
  pc = 0x4087fe; continue;
 }
 0x4087fe => {
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x199u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x40880f); cpu.call(t); if cpu.returned_to != 0x40880f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x5e18c0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40886a; continue; }
  pc = 0x408826; continue;
 }
 0x4087ff => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x5de5d0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x199u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x40880f); cpu.call(t); if cpu.returned_to != 0x40880f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x5e18c0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40886a; continue; }
  pc = 0x408826; continue;
 }
 0x408826 => {
  { let v: u32 = 0x5ebee4u32; cpu.esi = v; }
  pc = 0x40882b; continue;
 }
 0x40882b => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x408835); f_0040bd60(cpu); if cpu.returned_to != 0x408835 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40885a; continue; }
  pc = 0x40883c; continue;
 }
 0x40883c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40884c; continue; }
  pc = 0x408842; continue;
 }
 0x408842 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5e18c0u32, v); }
  pc = 0x40884c; continue;
 }
 0x40884c => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x40885a; continue; }
  pc = 0x408850; continue;
 }
 0x408850 => {
  { let v: u32 = 0x4u32; cpu.w32(0x5e18c0u32, v); }
  pc = 0x40885a; continue;
 }
 0x40885a => {
  { let v: u32 = cpu.r32(0x5ebdd0u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x115u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40882b; continue; }
  pc = 0x40886a; continue;
 }
 0x40886a => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x408903; continue; }
  pc = 0x408872; continue;
 }
 0x408872 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de5d8u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5dcu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de5e0u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5e4u32, v); }
  cpu.push(0x40889b); f_00403770(cpu); if cpu.returned_to != 0x40889b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4088da; continue; }
  pc = 0x4088a2; continue;
 }
 0x4088a2 => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x4088bc); cpu.call(t); if cpu.returned_to != 0x4088bc { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4088cb); cpu.call(t); if cpu.returned_to != 0x4088cb { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x4088da => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x4088f4); cpu.call(t); if cpu.returned_to != 0x4088f4 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x408903); cpu.call(t); if cpu.returned_to != 0x408903 { return; }
  pc = 0x408903; continue;
 }
 0x408903 => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x40891d); cpu.call(t); if cpu.returned_to != 0x40891d { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x40892b); cpu.call(t); if cpu.returned_to != 0x40892b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x40893a => {
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x3fdu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x40894d); cpu.call(t); if cpu.returned_to != 0x40894d { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5d0u32, v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x408a7d; continue; }
  pc = 0x40895c; continue;
 }
 0x40895c => {
  { let v: u32 = cpu.r32(0x5de6b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5c0u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x40897f; continue; }
  pc = 0x40896b; continue;
 }
 0x40896b => {
  cpu.push(0x408970); f_00403290(cpu); if cpu.returned_to != 0x408970 { return; }
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x5de6b0u32, v); }
  pc = 0x40897f; continue;
 }
 0x40897f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x408640u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5e18bcu32, v); }
  cpu.push(0x408990); f_004132f0(cpu); if cpu.returned_to != 0x408990 { return; }
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11c8u32); cpu.push(0x40899d); cpu.call(t); if cpu.returned_to != 0x40899d { return; }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de5c4u32, v); }
  { let v: u32 = 0x27u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x408a53; continue; }
  pc = 0x4089be; continue;
 }
 0x4089be => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4ab198u32; cpu.push(v); }
  cpu.push(0x4089d2); f_00408610(cpu); if cpu.returned_to != 0x4089d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408a53; continue; }
  pc = 0x4089d9; continue;
 }
 0x4089d9 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x27u32; cpu.push(v); }
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4089ea); f_0040bf60(cpu); if cpu.returned_to != 0x4089ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x5de5d8u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4089fc); f_0040be70(cpu); if cpu.returned_to != 0x4089fc { return; }
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x18bu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408a11); cpu.call(t); if cpu.returned_to != 0x408a11 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x408a53; continue; }
  pc = 0x408a15; continue;
 }
 0x408a15 => {
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x199u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408a24); cpu.call(t); if cpu.returned_to != 0x408a24 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5de5d8u32; cpu.push(v); }
  cpu.push(0x408a2f); f_0040bd60(cpu); if cpu.returned_to != 0x408a2f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x408a4d; continue; }
  pc = 0x408a36; continue;
 }
 0x408a36 => {
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x18bu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  let t = cpu.edi; cpu.push(0x408a47); cpu.call(t); if cpu.returned_to != 0x408a47 { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x408a15; continue; }
  pc = 0x408a4b; continue;
 }
 0x408a4b => {
  pc = 0x408a53; continue;
 }
 0x408a4d => {
  { let v: u32 = cpu.esi; cpu.w32(0x5de5c4u32, v); }
  pc = 0x408a53; continue;
 }
 0x408a53 => {
  { let v: u32 = cpu.r32(0x5de5c4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x186u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408a68); cpu.call(t); if cpu.returned_to != 0x408a68 { return; }
  { let v: u32 = cpu.r32(0x5de5d0u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x188u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408a78); cpu.call(t); if cpu.returned_to != 0x408a78 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5c4u32, v); }
  pc = 0x408a7d; continue;
 }
 0x408a7d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x7cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408a90); }
 let mut pc: u32 = 0x408a90;
 loop { match pc {
 0x408a90 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x110u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x408c06; continue; }
  pc = 0x408aa1; continue;
 }
 0x408aa1 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x408b3d; continue; }
  pc = 0x408aa8; continue;
 }
 0x408aa8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x408b21; continue; }
  pc = 0x408ab3; continue;
 }
 0x408ab3 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x403u32); cpu.eax = v; }
  if cpu.zf { pc = 0x408af5; continue; }
  pc = 0x408ac1; continue;
 }
 0x408ac1 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x408b21; continue; }
  pc = 0x408ac4; continue;
 }
 0x408ac4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x404u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x408add); cpu.call(t); if cpu.returned_to != 0x408add { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x408c3c; continue; }
  pc = 0x408ae6; continue;
 }
 0x408ae6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5eb434u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408af5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x403u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x408b0e); cpu.call(t); if cpu.returned_to != 0x408b0e { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x408c3c; continue; }
  pc = 0x408b17; continue;
 }
 0x408b17 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5eb434u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408b21 => {
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x3feu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x408bc8; continue; }
  pc = 0x408b33; continue;
 }
 0x408b33 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x3bu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x408b8b; continue; }
  pc = 0x408b38; continue;
 }
 0x408b38 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x408b44; continue; }
  pc = 0x408b3d; continue;
 }
 0x408b3d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408b44 => {
  { let v: u32 = cpu.r32(0x5eb434u32); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x5de694u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408b61; continue; }
  pc = 0x408b57; continue;
 }
 0x408b57 => {
  { let v: u32 = 0x3u32; cpu.w32(0x5de694u32, v); }
  pc = 0x408b61; continue;
 }
 0x408b61 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408b6e); cpu.call(t); if cpu.returned_to != 0x408b6e { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408b81); cpu.call(t); if cpu.returned_to != 0x408b81 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408b8b => {
  { let v: u32 = 0x2u32; cpu.w32(0x5de694u32, v); }
  cpu.push(0x408b9a); f_004086d0(cpu); if cpu.returned_to != 0x408b9a { return; }
  cpu.push(0x408b9f); f_00403a00(cpu); if cpu.returned_to != 0x408b9f { return; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408bac); cpu.call(t); if cpu.returned_to != 0x408bac { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408bbe); cpu.call(t); if cpu.returned_to != 0x408bbe { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408bc8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de694u32, v); }
  cpu.push(0x408bd7); f_004086d0(cpu); if cpu.returned_to != 0x408bd7 { return; }
  cpu.push(0x408bdc); f_00403a00(cpu); if cpu.returned_to != 0x408bdc { return; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408be9); cpu.call(t); if cpu.returned_to != 0x408be9 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408bfc); cpu.call(t); if cpu.returned_to != 0x408bfc { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408c06 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4a11d0u32); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf1u32; cpu.push(v); }
  { let v = 0x403u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408c21); cpu.call(t); if cpu.returned_to != 0x408c21 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xf1u32; cpu.push(v); }
  { let v = 0x404u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408c32); cpu.call(t); if cpu.returned_to != 0x408c32 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x5eb434u32, v); }
  pc = 0x408c3c; continue;
 }
 0x408c3c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00408c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x408c50); }
 let mut pc: u32 = 0x408c50;
 loop { match pc {
 0x408c50 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x80u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x110u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x408fcd; continue; }
  pc = 0x408c69; continue;
 }
 0x408c69 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x408e31; continue; }
  pc = 0x408c70; continue;
 }
 0x408c70 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xfffffc17u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x52u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x408e31; continue; }
  pc = 0x408c8e; continue;
 }
 0x408c8e => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x409134u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x409110u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x408c9d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x429u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x408cb9); cpu.call(t); if cpu.returned_to != 0x408cb9 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x4090fe; continue; }
  pc = 0x408cc2; continue;
 }
 0x408cc2 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebda0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4ab15cu32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408ce3 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x42au32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x408cff); cpu.call(t); if cpu.returned_to != 0x408cff { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x4090fe; continue; }
  pc = 0x408d08; continue;
 }
 0x408d08 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebda0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4ab15cu32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408d29 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x408d45); cpu.call(t); if cpu.returned_to != 0x408d45 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x4090fe; continue; }
  pc = 0x408d4e; continue;
 }
 0x408d4e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebda0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4ab15cu32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408d6f => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x300u32); }
  if !cpu.zf { pc = 0x408e31; continue; }
  pc = 0x408d7d; continue;
 }
 0x408d7d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4a1218u32); cpu.edi = v; }
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408d99); cpu.call(t); if cpu.returned_to != 0x408d99 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408db4; continue; }
  pc = 0x408d9d; continue;
 }
 0x408d9d => {
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x3e9u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408dac); cpu.call(t); if cpu.returned_to != 0x408dac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408db4; continue; }
  pc = 0x408db0; continue;
 }
 0x408db0 => {
  { let v = 0x5u32; cpu.push(v); }
  pc = 0x408db6; continue;
 }
 0x408db4 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x408db6; continue;
 }
 0x408db6 => {
  { let v = 0x43bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x408dc2); cpu.call(t); if cpu.returned_to != 0x408dc2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11d8u32); cpu.push(0x408dc9); cpu.call(t); if cpu.returned_to != 0x408dc9 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408ddb => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x300u32); }
  if !cpu.zf { pc = 0x408e31; continue; }
  pc = 0x408de5; continue;
 }
 0x408de5 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4a1218u32); cpu.edi = v; }
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408e01); cpu.call(t); if cpu.returned_to != 0x408e01 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408e1c; continue; }
  pc = 0x408e05; continue;
 }
 0x408e05 => {
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x3e9u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x408e14); cpu.call(t); if cpu.returned_to != 0x408e14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408e1c; continue; }
  pc = 0x408e18; continue;
 }
 0x408e18 => {
  { let v = 0x5u32; cpu.push(v); }
  pc = 0x408e1e; continue;
 }
 0x408e1c => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x408e1e; continue;
 }
 0x408e1e => {
  { let v = 0x43bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x408e2a); cpu.call(t); if cpu.returned_to != 0x408e2a { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11d8u32); cpu.push(0x408e31); cpu.call(t); if cpu.returned_to != 0x408e31 { return; }
  pc = 0x408e31; continue;
 }
 0x408e31 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408e40 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de694u32, v); }
  cpu.push(0x408e4f); f_004086d0(cpu); if cpu.returned_to != 0x408e4f { return; }
  cpu.push(0x408e54); f_00403a00(cpu); if cpu.returned_to != 0x408e54 { return; }
  pc = 0x408ef5; continue;
 }
 0x408e59 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5de694u32, v); }
  pc = 0x408f50; continue;
 }
 0x408e68 => {
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408ed3; continue; }
  pc = 0x408e71; continue;
 }
 0x408e71 => {
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x5de5d8u32; cpu.push(v); }
  cpu.push(0x408e80); f_0040bce0(cpu); if cpu.returned_to != 0x408e80 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4eu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x408e94); f_0040bf30(cpu); if cpu.returned_to != 0x408e94 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x27u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4ab198u32; cpu.push(v); }
  cpu.push(0x408ea8); f_004085f0(cpu); if cpu.returned_to != 0x408ea8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x33u32; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x4ab1b4u32; cpu.push(v); }
  cpu.push(0x408ebc); f_004085d0(cpu); if cpu.returned_to != 0x408ebc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x33u32; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x4ab1a8u32; cpu.push(v); }
  cpu.push(0x408ed0); f_004085d0(cpu); if cpu.returned_to != 0x408ed0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x408ed3; continue;
 }
 0x408ed3 => {
  { let v: u32 = cpu.r32(0x5ebda0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  cpu.push(0x408ee4); f_00403820(cpu); if cpu.returned_to != 0x408ee4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408f2a; continue; }
  pc = 0x408eeb; continue;
 }
 0x408eeb => {
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  pc = 0x408ef5; continue;
 }
 0x408ef5 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408f05); cpu.call(t); if cpu.returned_to != 0x408f05 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408f18); cpu.call(t); if cpu.returned_to != 0x408f18 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408f2a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x5eb430u32; cpu.push(v); }
  cpu.push(0x408f3f); f_004037c0(cpu); if cpu.returned_to != 0x408f3f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x408f84; continue; }
  pc = 0x408f46; continue;
 }
 0x408f46 => {
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  pc = 0x408f50; continue;
 }
 0x408f50 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408f60); cpu.call(t); if cpu.returned_to != 0x408f60 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408f72); cpu.call(t); if cpu.returned_to != 0x408f72 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408f84 => {
  cpu.push(0x408f89); f_004082c0(cpu); if cpu.returned_to != 0x408f89 { return; }
  cpu.push(0x408f8e); f_004086d0(cpu); if cpu.returned_to != 0x408f8e { return; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x408fa8); cpu.call(t); if cpu.returned_to != 0x408fa8 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x408fbb); cpu.call(t); if cpu.returned_to != 0x408fbb { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x408fcd => {
  { let v: u32 = cpu.r32(0x5e18c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x408fe1; continue; }
  pc = 0x408fdc; continue;
 }
 0x408fdc => {
  { let v: u32 = cpu.eax; cpu.w32(0x4ab15cu32, v); }
  pc = 0x408fe1; continue;
 }
 0x408fe1 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4a11d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4a11ccu32); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x40901c; continue; }
  pc = 0x408ff9; continue;
 }
 0x408ff9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x409003); cpu.call(t); if cpu.returned_to != 0x409003 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x409006); cpu.call(t); if cpu.returned_to != 0x409006 { return; }
  { cpu.sub32(cpu.r32(0x5e18c0u32), 0x3u32); }
  if (cpu.sf == cpu.of) { pc = 0x40901c; continue; }
  pc = 0x40900f; continue;
 }
 0x40900f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x42au32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x409019); cpu.call(t); if cpu.returned_to != 0x409019 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x40901c); cpu.call(t); if cpu.returned_to != 0x40901c { return; }
  pc = 0x40901c; continue;
 }
 0x40901c => {
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x427u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = 0x429u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11d4u32); cpu.push(0x409038); cpu.call(t); if cpu.returned_to != 0x409038 { return; }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ebda0u32, v); }
  { let v: u32 = 0x33u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x33u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5de660u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5de5f0u32, v); }
  if cpu.zf { pc = 0x4090c1; continue; }
  pc = 0x40906b; continue;
 }
 0x40906b => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x4ab1b4u32; cpu.push(v); }
  cpu.push(0x40907f); f_00408610(cpu); if cpu.returned_to != 0x40907f { return; }
  { let v: u32 = cpu.r32(0x4a11b8u32); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409099; continue; }
  pc = 0x40908c; continue;
 }
 0x40908c => {
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x3e9u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x409099); cpu.call(t); if cpu.returned_to != 0x409099 { return; }
  pc = 0x409099; continue;
 }
 0x409099 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x4ab1a8u32; cpu.push(v); }
  cpu.push(0x4090ad); f_00408610(cpu); if cpu.returned_to != 0x4090ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4090c1; continue; }
  pc = 0x4090b4; continue;
 }
 0x4090b4 => {
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4090c1); cpu.call(t); if cpu.returned_to != 0x4090c1 { return; }
  pc = 0x4090c1; continue;
 }
 0x4090c1 => {
  { let v: u32 = cpu.r32(0x4a1218u32); cpu.edi = v; }
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de5f0u32; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4090d6); cpu.call(t); if cpu.returned_to != 0x4090d6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4090f1; continue; }
  pc = 0x4090da; continue;
 }
 0x4090da => {
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de660u32; cpu.push(v); }
  { let v = 0x3e9u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4090e9); cpu.call(t); if cpu.returned_to != 0x4090e9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4090f1; continue; }
  pc = 0x4090ed; continue;
 }
 0x4090ed => {
  { let v = 0x5u32; cpu.push(v); }
  pc = 0x4090f3; continue;
 }
 0x4090f1 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x4090f3; continue;
 }
 0x4090f3 => {
  { let v = 0x43bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4090fb); cpu.call(t); if cpu.returned_to != 0x4090fb { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x4090fe); cpu.call(t); if cpu.returned_to != 0x4090fe { return; }
  pc = 0x4090fe; continue;
 }
 0x4090fe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x80u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409190); }
 let mut pc: u32 = 0x409190;
 loop { match pc {
 0x409190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a101cu32); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x20u32; cpu.push(v); }
  { let v = 0x20u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x4091a6); cpu.call(t); if cpu.returned_to != 0x4091a6 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x20u32; cpu.push(v); }
  { let v = 0x20u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  let t = cpu.edi; cpu.push(0x4091b4); cpu.call(t); if cpu.returned_to != 0x4091b4 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5df590u32); cpu.eax = v; }
  { let v = 0x69u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a1264u32); cpu.push(0x4091c4); cpu.call(t); if cpu.returned_to != 0x4091c4 { return; }
  { let v: u32 = cpu.r32(0x4a1018u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4091d2); cpu.call(t); if cpu.returned_to != 0x4091d2 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4091d8); cpu.call(t); if cpu.returned_to != 0x4091d8 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x4a1058u32); cpu.push(0x4091df); cpu.call(t); if cpu.returned_to != 0x4091df { return; }
  { let v: u32 = cpu.r32(0x5df590u32); cpu.ecx = v; }
  { let v = 0x95u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a1264u32); cpu.push(0x4091f1); cpu.call(t); if cpu.returned_to != 0x4091f1 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4091f9); cpu.call(t); if cpu.returned_to != 0x4091f9 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4091ff); cpu.call(t); if cpu.returned_to != 0x4091ff { return; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x4a1058u32); cpu.push(0x409206); cpu.call(t); if cpu.returned_to != 0x409206 { return; }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1003u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x40921b); cpu.call(t); if cpu.returned_to != 0x40921b { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1003u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x409226); cpu.call(t); if cpu.returned_to != 0x409226 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409240); }
 let mut pc: u32 = 0x409240;
 loop { match pc {
 0x409240 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x101bu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x40928c); cpu.call(t); if cpu.returned_to != 0x40928c { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004092a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4092a0); }
 let mut pc: u32 = 0x4092a0;
 loop { match pc {
 0x4092a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x4092b1); cpu.call(t); if cpu.returned_to != 0x4092b1 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4092b9); f_00409190(cpu); if cpu.returned_to != 0x4092b9 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x56d314u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4092f9; continue; }
  pc = 0x4092c6; continue;
 }
 0x4092c6 => {
  cpu.poll(); pc = cpu.r32(0x4092fcu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x4092cd => {
  { let v = 0x220u32; cpu.push(v); }
  { let v = 0x4ab1d0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x4092df); f_00409240(cpu); if cpu.returned_to != 0x4092df { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4092e4 => {
  { let v = 0x220u32; cpu.push(v); }
  { let v = 0x4ab1c0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x4092f6); f_00409240(cpu); if cpu.returned_to != 0x4092f6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x4092f9; continue;
 }
 0x4092f9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409310); }
 let mut pc: u32 = 0x409310;
 loop { match pc {
 0x409310 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409324); cpu.call(t); if cpu.returned_to != 0x409324 { return; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7bu32; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1007u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409373); cpu.call(t); if cpu.returned_to != 0x409373 { return; }
  { let v: u32 = cpu.r32(0x5de6a8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6a8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409390); }
 let mut pc: u32 = 0x409390;
 loop { match pc {
 0x409390 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x5de69cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6a0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6a4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004093b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4093b0); }
 let mut pc: u32 = 0x4093b0;
 loop { match pc {
 0x4093b0 => {
  { let v: u32 = cpu.r32(0x5de6a8u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4093f2; continue; }
  pc = 0x4093bc; continue;
 }
 0x4093bc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a11ccu32); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.ebx = v; }
  pc = 0x4093cf; continue;
 }
 0x4093cf => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1008u32; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x4093e2); cpu.call(t); if cpu.returned_to != 0x4093e2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x4093e5); cpu.call(t); if cpu.returned_to != 0x4093e5 { return; }
  { let v: u32 = cpu.r32(0x5de6a8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4093cf; continue; }
  pc = 0x4093ef; continue;
 }
 0x4093ef => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4093f2; continue;
 }
 0x4093f2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de6a8u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409400); }
 let mut pc: u32 = 0x409400;
 loop { match pc {
 0x409400 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if (cpu.sf == cpu.of) { pc = 0x409476; continue; }
  pc = 0x409409; continue;
 }
 0x409409 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eb440u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x5eb47bu32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = 0x5eb448u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eb48bu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x409476; continue;
 }
 0x409476 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409480); }
 let mut pc: u32 = 0x409480;
 loop { match pc {
 0x409480 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x40948c; continue; }
  pc = 0x409487; continue;
 }
 0x409487 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x40948c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ebda0u32, v); }
  if !cpu.zf { pc = 0x4094bf; continue; }
  pc = 0x4094a2; continue;
 }
 0x4094a2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eb438u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4094b1); f_00409400(cpu); if cpu.returned_to != 0x4094b1 { return; }
  { let v: u32 = cpu.r32(0x5eb438u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5eb438u32, v); }
  pc = 0x4094bf; continue;
 }
 0x4094bf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004094d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4094d0); }
 let mut pc: u32 = 0x4094d0;
 loop { match pc {
 0x4094d0 => {
  { let v: u32 = cpu.r32(0x5de69cu32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x409550; continue; }
  pc = 0x4094dc; continue;
 }
 0x4094dc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4a126cu32); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x4094e9; continue;
 }
 0x4094e9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x409480u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eb438u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5eb440u32, v); }
  cpu.push(0x409502); f_004038f0(cpu); if cpu.returned_to != 0x409502 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409546; continue; }
  pc = 0x409509; continue;
 }
 0x409509 => {
  { let v: u32 = cpu.r32(0x5de6acu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de6a0u32, v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x409540; continue; }
  pc = 0x409518; continue;
 }
 0x409518 => {
  { let v: u32 = cpu.r32(0x5de624u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x113u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x409527); cpu.call(t); if cpu.returned_to != 0x409527 { return; }
  { cpu.sub32(cpu.r32(0x5de6a0u32), cpu.esi); }
  if cpu.zf { pc = 0x409540; continue; }
  pc = 0x40952f; continue;
 }
 0x40952f => {
  { let v: u32 = cpu.r32(0x5de69cu32); cpu.eax = v; }
  pc = 0x409534; continue;
 }
 0x409534 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x409540; continue; }
  pc = 0x409538; continue;
 }
 0x409538 => {
  { cpu.sub32(cpu.r32(0x5de6a0u32), cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x409534; continue; }
  pc = 0x409540; continue;
 }
 0x409540 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5de6acu32, v); }
  pc = 0x409546; continue;
 }
 0x409546 => {
  { cpu.sub32(cpu.r32(0x5de69cu32), cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x4094e9; continue; }
  pc = 0x40954e; continue;
 }
 0x40954e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x409550; continue;
 }
 0x409550 => {
  { let v: u32 = cpu.esi; cpu.w32(0x5de69cu32, v); }
  { let v: u32 = 0xfaceu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409560); }
 let mut pc: u32 = 0x409560;
 loop { match pc {
 0x409560 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5e2440u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5e2440u32, v); }
  { let v: u32 = 0x4ab1e4u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004095e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095e0); }
 let mut pc: u32 = 0x4095e0;
 loop { match pc {
 0x4095e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eb440u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40963f; continue; }
  pc = 0x4095f2; continue;
 }
 0x4095f2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5eb448u32; cpu.ebp = v; }
  pc = 0x4095fb; continue;
 }
 0x4095fb => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409627; continue; }
  pc = 0x40960f; continue;
 }
 0x40960f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x40960f; continue; }
  pc = 0x409627; continue;
 }
 0x409627 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x43u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4bu32); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x4095fb; continue; }
  pc = 0x40963b; continue;
 }
 0x40963b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x40963f; continue;
 }
 0x40963f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409650); }
 let mut pc: u32 = 0x409650;
 loop { match pc {
 0x409650 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x2cu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x409654; continue;
 }
 0x409654 => {
  { cpu.sub8(cpu.r8(0x5df900u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x409665; continue; }
  pc = 0x40965c; continue;
 }
 0x40965c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409654; continue; }
  pc = 0x409664; continue;
 }
 0x409664 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409665 => {
  { let v: u8 = 0x3au8; cpu.w8(0x5df900u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00409670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409670); }
 let mut pc: u32 = 0x409670;
 loop { match pc {
 0x409670 => {
  { let v: u32 = cpu.r32(0x5de5ecu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409695; continue; }
  pc = 0x409679; continue;
 }
 0x409679 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de5ccu32); cpu.esi = v; }
  cpu.push(0x409685); f_004095e0(cpu); if cpu.returned_to != 0x409685 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5ccu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409695 => {
  cpu.push(0x40969a); f_004095e0(cpu); if cpu.returned_to != 0x40969a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5ccu32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de5ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004096b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4096b0); } r_004096b0(cpu, 0x4096b0); }
pub fn f_00409800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409800); } r_004096b0(cpu, 0x409800); }
fn r_004096b0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4096b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xffffffb2u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0xc5u32); }
  { let v = cpu.edi; cpu.push(v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x409f3c; continue; }
  pc = 0x4096cc; continue;
 }
 0x4096cc => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x409f60u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x409f4cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x4096db => {
  { let v: u32 = cpu.r32(0x120u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0xffffff9bu32); }
  if !cpu.zf { pc = 0x409f3c; continue; }
  pc = 0x4096ec; continue;
 }
 0x4096ec => {
  { cpu.and8(cpu.r8(0x14u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x409f3c; continue; }
  pc = 0x4096f6; continue;
 }
 0x4096f6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4ab160u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x40970e => {
  { let v: u32 = cpu.r32(0x5e18c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x409722; continue; }
  pc = 0x40971d; continue;
 }
 0x40971d => {
  { let v: u32 = cpu.eax; cpu.w32(0x4ab15cu32, v); }
  pc = 0x409722; continue;
 }
 0x409722 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x40975f; continue; }
  pc = 0x40972e; continue;
 }
 0x40972e => {
  { let v: u32 = cpu.r32(0x4a11ccu32); cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x40973f); cpu.call(t); if cpu.returned_to != 0x40973f { return; }
  { let v: u32 = cpu.r32(0x4a11d8u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x409748); cpu.call(t); if cpu.returned_to != 0x409748 { return; }
  { cpu.sub32(cpu.r32(0x5e18c0u32), 0x3u32); }
  if (cpu.sf == cpu.of) { pc = 0x409761; continue; }
  pc = 0x409751; continue;
 }
 0x409751 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x42au32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x40975a); cpu.call(t); if cpu.returned_to != 0x40975a { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x40975d); cpu.call(t); if cpu.returned_to != 0x40975d { return; }
  pc = 0x409761; continue;
 }
 0x40975f => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x409761; continue;
 }
 0x409761 => {
  { let v: u32 = cpu.r32(0x4ab15cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x427u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = 0x429u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11d4u32); cpu.push(0x40977d); cpu.call(t); if cpu.returned_to != 0x40977d { return; }
  { let v: u32 = cpu.r32(0x5de698u32); cpu.eax = v; }
  { let v: u32 = 0x33u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u8 = 0x0u8; cpu.w8(0x5de628u32, v); }
  if cpu.zf { pc = 0x4097c1; continue; }
  pc = 0x409795; continue;
 }
 0x409795 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x5de628u32; cpu.push(v); }
  { let v = 0x4ab1b4u32; cpu.push(v); }
  cpu.push(0x4097a9); f_00408610(cpu); if cpu.returned_to != 0x4097a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097c1; continue; }
  pc = 0x4097b0; continue;
 }
 0x4097b0 => {
  { let v = 0x5de628u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11b8u32); cpu.push(0x4097c1); cpu.call(t); if cpu.returned_to != 0x4097c1 { return; }
  pc = 0x4097c1; continue;
 }
 0x4097c1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4097c7); f_004092a0(cpu); if cpu.returned_to != 0x4097c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eb438u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eb440u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x409480u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4097e3); f_004038f0(cpu); if cpu.returned_to != 0x4097e3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x887700aau32); }
  if !cpu.zf { pc = 0x409810; continue; }
  pc = 0x4097ed; continue;
 }
 0x4097ed => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5de694u32, v); }
  cpu.push(0x4097f8); f_004086d0(cpu); if cpu.returned_to != 0x4097f8 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x409800); cpu.call(t); if cpu.returned_to != 0x409800 { return; }
  pc = 0x409800; continue;
 }
 0x409800 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x409d77; continue;
 }
 0x409810 => {
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409820); cpu.call(t); if cpu.returned_to != 0x409820 { return; }
  { cpu.sub32(cpu.r32(0x5de6a4u32), cpu.ebp); }
  if !cpu.zf { pc = 0x409f2a; continue; }
  pc = 0x40982c; continue;
 }
 0x40982c => {
  { let v = 0x5de5d4u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x5de624u32; cpu.push(v); }
  { let v = 0x4094d0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de69cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5de624u32, v); }
  let t = cpu.r32(0x4a10c0u32); cpu.push(0x409850); cpu.call(t); if cpu.returned_to != 0x409850 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6a4u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409867 => {
  cpu.push(0x40986c); f_00409670(cpu); if cpu.returned_to != 0x40986c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099ff; continue; }
  pc = 0x409874; continue;
 }
 0x409874 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409881); f_004093b0(cpu); if cpu.returned_to != 0x409881 { return; }
  { let v: u32 = cpu.r32(0x5eb440u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4ab160u32, v); }
  if cpu.zf { pc = 0x409b9e; continue; }
  pc = 0x40989b; continue;
 }
 0x40989b => {
  { let v: u32 = 0x5eb440u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x5e2240u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5e18e8u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x4098be; continue;
 }
 0x4098ba => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x4098be; continue;
 }
 0x4098be => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x5eb47bu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18e0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2440u32, v); }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = 0x409560u32; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.movs(1, true); }
  cpu.push(0x409947); f_004038c0(cpu); if cpu.returned_to != 0x409947 { return; }
  { let v: u32 = 0x4ab1e8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x40997c); f_00409650(cpu); if cpu.returned_to != 0x40997c { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5e2440u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x40999a; continue; }
  pc = 0x409989; continue;
 }
 0x409989 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5df900u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4099a9; continue;
 }
 0x40999a => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x5df900u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x4099a9; continue;
 }
 0x4099a9 => {
  cpu.push(0x4099ae); f_00409310(cpu); if cpu.returned_to != 0x4099ae { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4bu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4098ba; continue; }
  pc = 0x4099e3; continue;
 }
 0x4099e3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de6a0u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x4099ff => {
  { let v: u32 = cpu.r32(0x5e18e0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b9e; continue; }
  pc = 0x409a0e; continue;
 }
 0x409a0e => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x5e18e0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5e2240u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x409a2a; continue;
 }
 0x409a2a => {
  { let v: u32 = 0x48u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x102du32; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x200u32; cpu.w32(0x74u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5df900u32; cpu.w32(0x70u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409a59); cpu.call(t); if cpu.returned_to != 0x409a59 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409a60); cpu.call(t); if cpu.returned_to != 0x409a60 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2440u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x409560u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x409aa1); f_004038c0(cpu); if cpu.returned_to != 0x409aa1 { return; }
  { let v: u32 = 0x4ab1e8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x5df900u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x409ad6); f_00409650(cpu); if cpu.returned_to != 0x409ad6 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5e2440u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x409b1b; continue; }
  pc = 0x409af1; continue;
 }
 0x409af1 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1006u32; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409b12); cpu.call(t); if cpu.returned_to != 0x409b12 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409b19); cpu.call(t); if cpu.returned_to != 0x409b19 { return; }
  pc = 0x409b4c; continue;
 }
 0x409b1b => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1006u32; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409b43); cpu.call(t); if cpu.returned_to != 0x409b43 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409b4a); cpu.call(t); if cpu.returned_to != 0x409b4a { return; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x409b4c; continue;
 }
 0x409b4c => {
  { let v: u32 = 0x70u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x78u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x102eu32; cpu.push(v); }
  { let v = 0x43du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5df900u32; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409b72); cpu.call(t); if cpu.returned_to != 0x409b72 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409b79); cpu.call(t); if cpu.returned_to != 0x409b79 { return; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4bu32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x409a2a; continue; }
  pc = 0x409b9e; continue;
 }
 0x409b9e => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de6a0u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409bba => {
  { let v: u32 = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = 0xfffffc02u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x409f3c; continue; }
  pc = 0x409bd8; continue;
 }
 0x409bd8 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x40a048u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x40a028u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x409be7 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x429u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x409c03); cpu.call(t); if cpu.returned_to != 0x409c03 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x409f2a; continue; }
  pc = 0x409c0c; continue;
 }
 0x409c0c => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de5ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6acu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4ab160u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x4ab15cu32, v); }
  pc = 0x409cc6; continue;
 }
 0x409c34 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x42au32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x409c50); cpu.call(t); if cpu.returned_to != 0x409c50 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x409f2a; continue; }
  pc = 0x409c59; continue;
 }
 0x409c59 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de5ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6acu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4ab160u32, v); }
  { let v: u32 = 0x3u32; cpu.w32(0x4ab15cu32, v); }
  pc = 0x409cc6; continue;
 }
 0x409c7e => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xf0u32; cpu.push(v); }
  { let v = 0x42bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11d0u32); cpu.push(0x409c9a); cpu.call(t); if cpu.returned_to != 0x409c9a { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x409f2a; continue; }
  pc = 0x409ca3; continue;
 }
 0x409ca3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de5ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6acu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4ab160u32, v); }
  { let v: u32 = 0x4u32; cpu.w32(0x4ab15cu32, v); }
  pc = 0x409cc6; continue;
 }
 0x409cc6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x113u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409cd6); cpu.call(t); if cpu.returned_to != 0x409cd6 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409ce8 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x300u32); }
  if !cpu.zf { pc = 0x409f3c; continue; }
  pc = 0x409cf7; continue;
 }
 0x409cf7 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x32u32; cpu.push(v); }
  { let v = 0x5de628u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a1218u32); cpu.push(0x409d11); cpu.call(t); if cpu.returned_to != 0x409d11 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d19; continue; }
  pc = 0x409d15; continue;
 }
 0x409d15 => {
  { let v = 0x5u32; cpu.push(v); }
  pc = 0x409d1b; continue;
 }
 0x409d19 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x409d1b; continue;
 }
 0x409d1b => {
  { let v = 0x43bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4a11ccu32); cpu.push(0x409d27); cpu.call(t); if cpu.returned_to != 0x409d27 { return; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11d8u32); cpu.push(0x409d2e); cpu.call(t); if cpu.returned_to != 0x409d2e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409d3d => {
  { let v: u32 = cpu.r32(0x5de6a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d4b; continue; }
  pc = 0x409d46; continue;
 }
 0x409d46 => {
  cpu.push(0x409d4b); f_00409390(cpu); if cpu.returned_to != 0x409d4b { return; }
  pc = 0x409d4b; continue;
 }
 0x409d4b => {
  { let v: u32 = 0x0u32; cpu.w32(0x5de694u32, v); }
  cpu.push(0x409d5a); f_004086d0(cpu); if cpu.returned_to != 0x409d5a { return; }
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x409d6a); cpu.call(t); if cpu.returned_to != 0x409d6a { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x409d77; continue;
 }
 0x409d77 => {
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409d7d); cpu.call(t); if cpu.returned_to != 0x409d7d { return; }
  cpu.push(0x409d82); f_00403a00(cpu); if cpu.returned_to != 0x409d82 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409d94 => {
  { let v: u32 = cpu.r32(0x5de6a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409da2; continue; }
  pc = 0x409d9d; continue;
 }
 0x409d9d => {
  cpu.push(0x409da2); f_00409390(cpu); if cpu.returned_to != 0x409da2 { return; }
  pc = 0x409da2; continue;
 }
 0x409da2 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5de694u32, v); }
  pc = 0x409e69; continue;
 }
 0x409db1 => {
  { let v: u32 = cpu.r32(0x4ab160u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x409dcd; continue; }
  pc = 0x409dbb; continue;
 }
 0x409dbb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409dcd => {
  { let v: u32 = cpu.r32(0x5de6a4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x409de1; continue; }
  pc = 0x409dd7; continue;
 }
 0x409dd7 => {
  cpu.push(0x409ddc); f_00409390(cpu); if cpu.returned_to != 0x409ddc { return; }
  { let v: u32 = cpu.r32(0x4ab160u32); cpu.eax = v; }
  pc = 0x409de1; continue;
 }
 0x409de1 => {
  { let v: u32 = cpu.r32(0x5de698u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x409e4a; continue; }
  pc = 0x409deb; continue;
 }
 0x409deb => {
  { let v: u32 = 0xc0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x5de5d8u32; cpu.push(v); }
  cpu.push(0x409dfd); f_0040bce0(cpu); if cpu.returned_to != 0x409dfd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xc0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x98u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4eu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409e17); f_0040bf30(cpu); if cpu.returned_to != 0x409e17 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x98u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x27u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4ab198u32; cpu.push(v); }
  cpu.push(0x409e2e); f_004085f0(cpu); if cpu.returned_to != 0x409e2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x33u32; cpu.push(v); }
  { let v = 0x5de628u32; cpu.push(v); }
  { let v = 0x4ab1b4u32; cpu.push(v); }
  cpu.push(0x409e42); f_004085d0(cpu); if cpu.returned_to != 0x409e42 { return; }
  { let v: u32 = cpu.r32(0x4ab160u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x409e4a; continue;
 }
 0x409e4a => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5e2240u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409e58); f_00403970(cpu); if cpu.returned_to != 0x409e58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409e9d; continue; }
  pc = 0x409e5f; continue;
 }
 0x409e5f => {
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  pc = 0x409e69; continue;
 }
 0x409e69 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x409e79); cpu.call(t); if cpu.returned_to != 0x409e79 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409e8b); cpu.call(t); if cpu.returned_to != 0x409e8b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409e9d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5de628u32; cpu.push(v); }
  { let v = 0x5eb430u32; cpu.push(v); }
  cpu.push(0x409eb2); f_004037c0(cpu); if cpu.returned_to != 0x409eb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ef8; continue; }
  pc = 0x409eb9; continue;
 }
 0x409eb9 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x409ed3); cpu.call(t); if cpu.returned_to != 0x409ed3 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409ee6); cpu.call(t); if cpu.returned_to != 0x409ee6 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409ef8 => {
  cpu.push(0x409efd); f_004086d0(cpu); if cpu.returned_to != 0x409efd { return; }
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de694u32, v); }
  let t = cpu.r32(0x4a11bcu32); cpu.push(0x409f17); cpu.call(t); if cpu.returned_to != 0x409f17 { return; }
  { let v: u32 = cpu.r32(0x5df38cu32); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a126cu32); cpu.push(0x409f2a); cpu.call(t); if cpu.returned_to != 0x409f2a { return; }
  pc = 0x409f2a; continue;
 }
 0x409f2a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409f3c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a090); }
 let mut pc: u32 = 0x40a090;
 loop { match pc {
 0x40a090 => {
  { let v: u32 = cpu.r32(0x5e18b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a0a8; continue; }
  pc = 0x40a09c; continue;
 }
 0x40a09c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e18b4u32, v); }
  pc = 0x40a0a8; continue;
 }
 0x40a0a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0b0); }
 let mut pc: u32 = 0x40a0b0;
 loop { match pc {
 0x40a0b0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x0u32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a0e2; continue; }
  pc = 0x40a0b9; continue;
 }
 0x40a0b9 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40a0d1; continue; }
  pc = 0x40a0bc; continue;
 }
 0x40a0bc => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40a0eb; continue; }
  pc = 0x40a0bf; continue;
 }
 0x40a0bf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5de718u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a0d1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5e18b8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a0e2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b8u32, v); }
  pc = 0x40a0eb; continue;
 }
 0x40a0eb => {
  { let v: u32 = cpu.r32(0x5e18b8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a100); }
 let mut pc: u32 = 0x40a100;
 loop { match pc {
 0x40a100 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x40a15a; continue; }
  pc = 0x40a113; continue;
 }
 0x40a113 => {
  { let v: u32 = cpu.r32(0x5e18b8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5de718u32); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a12c; continue; }
  pc = 0x40a128; continue;
 }
 0x40a128 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40a12c; continue;
 }
 0x40a12c => {
  { let v: u32 = cpu.r32(0x5de714u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x5e18b8u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x40a15a; continue;
 }
 0x40a15a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a170); }
 let mut pc: u32 = 0x40a170;
 loop { match pc {
 0x40a170 => {
  { let v: u32 = cpu.r32(0x5de714u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a18d; continue; }
  pc = 0x40a17a; continue;
 }
 0x40a17a => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a180); f_00402420(cpu); if cpu.returned_to != 0x40a180 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de714u32, v); }
  pc = 0x40a18d; continue;
 }
 0x40a18d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x4ab158u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a19c); f_0048cf60(cpu); if cpu.returned_to != 0x40a19c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a1ef; continue; }
  pc = 0x40a1a5; continue;
 }
 0x40a1a5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a1ab); f_0040d4a0(cpu); if cpu.returned_to != 0x40a1ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de718u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a1b9); f_00402400(cpu); if cpu.returned_to != 0x40a1b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de714u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a1ef; continue; }
  pc = 0x40a1c5; continue;
 }
 0x40a1c5 => {
  { let v: u32 = cpu.r32(0x5de718u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a1d5); f_0048d270(cpu); if cpu.returned_to != 0x40a1d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a1de); f_0048d190(cpu); if cpu.returned_to != 0x40a1de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e18b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a1ef => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a200); }
 let mut pc: u32 = 0x40a200;
 loop { match pc {
 0x40a200 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x40a24a; continue; }
  pc = 0x40a215; continue;
 }
 0x40a215 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6e0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5de6e4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6e8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6fcu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6c4u32, v); }
  pc = 0x40a25b; continue;
 }
 0x40a24a => {
  { let v: u32 = cpu.r32(0x5de6e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5de6ecu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5de6f0u32); cpu.edx = v; }
  pc = 0x40a25b; continue;
 }
 0x40a25b => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a3c3; continue; }
  pc = 0x40a264; continue;
 }
 0x40a264 => {
  { cpu.sub32(cpu.r32(0x5ddba0u32), cpu.ebp); }
  if cpu.zf { pc = 0x40a3b9; continue; }
  pc = 0x40a270; continue;
 }
 0x40a270 => {
  cpu.push(0x40a275); f_0040a820(cpu); if cpu.returned_to != 0x40a275 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6f0u32, v); }
  pc = 0x40a281; continue;
 }
 0x40a281 => {
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5e18b0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edx | 0xffffffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  { let v: u32 = cpu.esi; cpu.w32(0x5de6fcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6c4u32, v); }
  if (cpu.cf || cpu.zf) { pc = 0x40a2cf; continue; }
  pc = 0x40a2a6; continue;
 }
 0x40a2a6 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x40a2a8; continue;
 }
 0x40a2a8 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x40a2c4; continue; }
  pc = 0x40a2ad; continue;
 }
 0x40a2ad => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.cf { pc = 0x40a2c4; continue; }
  pc = 0x40a2b4; continue;
 }
 0x40a2b4 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5de6fcu32, v); }
  pc = 0x40a2c4; continue;
 }
 0x40a2c4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x25u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6c4u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40a2a8; continue; }
  pc = 0x40a2cf; continue;
 }
 0x40a2cf => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x40a344; continue; }
  pc = 0x40a2d3; continue;
 }
 0x40a2d3 => {
  { let v = 0x5de6e0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a2de); f_0040ac50(cpu); if cpu.returned_to != 0x40a2de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a357; continue; }
  pc = 0x40a2e5; continue;
 }
 0x40a2e5 => {
  { let v: u32 = cpu.r32(0x5de6e4u32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x40a31a; continue; }
  pc = 0x40a2ee; continue;
 }
 0x40a2ee => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x40a31a; continue; }
  pc = 0x40a2f3; continue;
 }
 0x40a2f3 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a2fb; continue;
 }
 0x40a2fb => {
  { let v: u32 = cpu.r32(0x5de6ecu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x40a302; continue;
 }
 0x40a302 => {
  cpu.push(0x40a307); f_00407510(cpu); if cpu.returned_to != 0x40a307 { return; }
  { let v: u32 = cpu.r32(0x5de704u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de704u32, v); }
  cpu.poll(); pc = 0x40a281; continue;
 }
 0x40a31a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x5de6e0u32; cpu.push(v); }
  cpu.push(0x40a325); f_0040b2f0(cpu); if cpu.returned_to != 0x40a325 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a330; continue;
 }
 0x40a330 => {
  { cpu.sub32(cpu.eax, 0xffffff96u32); }
  if cpu.zf { pc = 0x40a36d; continue; }
  pc = 0x40a335; continue;
 }
 0x40a335 => {
  { cpu.sub32(cpu.eax, 0xffffff98u32); }
  if cpu.zf { pc = 0x40a37a; continue; }
  pc = 0x40a33a; continue;
 }
 0x40a33a => {
  { cpu.sub32(cpu.eax, 0xffffff97u32); }
  if !cpu.zf { pc = 0x40a38d; continue; }
  pc = 0x40a33f; continue;
 }
 0x40a33f => {
  cpu.poll(); pc = 0x40a281; continue;
 }
 0x40a344 => {
  { let v: u32 = cpu.r32(0x5de6f0u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a357 => {
  { let v: u32 = cpu.r32(0x5de6f0u32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = 0xffffff9bu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a36d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a37a => {
  { let v: u32 = cpu.r32(0x5de6f0u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a38d => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { pc = 0x40a3af; continue; }
  pc = 0x40a395; continue;
 }
 0x40a395 => {
  { let v: u32 = cpu.r32(0x5de6ecu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x40a39c; continue;
 }
 0x40a39c => {
  cpu.push(0x40a3a1); f_00407510(cpu); if cpu.returned_to != 0x40a3a1 { return; }
  { let v: u32 = cpu.r32(0x5de704u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de704u32, v); }
  pc = 0x40a3af; continue;
 }
 0x40a3af => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a3b9 => {
  { let v: u32 = 0xffffff99u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a3c3 => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x40a3d2; continue; }
  pc = 0x40a3c8; continue;
 }
 0x40a3c8 => {
  { let v: u32 = 0xffffff9au32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a3d2 => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a3db; continue;
 }
 0x40a3db => {
  { let v: u32 = cpu.edx ^ 0x4u32; cpu.edx = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f0u32, v); }
  if !cpu.zf { pc = 0x40a400; continue; }
  pc = 0x40a3e9; continue;
 }
 0x40a3e9 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x40a400; continue; }
  pc = 0x40a3ee; continue;
 }
 0x40a3ee => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a3fa; continue;
 }
 0x40a3fa => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x40a302; continue;
 }
 0x40a400 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x5de6e0u32; cpu.push(v); }
  cpu.push(0x40a40b); f_0040b2f0(cpu); if cpu.returned_to != 0x40a40b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a416; continue;
 }
 0x40a416 => {
  { cpu.sub32(cpu.eax, 0xffffff96u32); }
  if !cpu.zf { pc = 0x40a428; continue; }
  pc = 0x40a41b; continue;
 }
 0x40a41b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a428 => {
  { cpu.sub32(cpu.eax, 0xffffff98u32); }
  if !cpu.zf { pc = 0x40a440; continue; }
  pc = 0x40a42d; continue;
 }
 0x40a42d => {
  { let v: u32 = cpu.r32(0x5de6f0u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6f0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a440 => {
  { cpu.sub32(cpu.eax, 0xffffff97u32); }
  if cpu.zf { cpu.poll(); pc = 0x40a281; continue; }
  pc = 0x40a449; continue;
 }
 0x40a449 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40a3af; continue; }
  pc = 0x40a455; continue;
 }
 0x40a455 => {
  { let v: u32 = cpu.r32(0x5de6ecu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x40a39c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a480); }
 let mut pc: u32 = 0x40a480;
 loop { match pc {
 0x40a480 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18a0u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18a4u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18a8u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18acu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b0u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de6bcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5e18a0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5e18b0u32, v); }
  cpu.push(0x40a4c8); f_0040a170(cpu); if cpu.returned_to != 0x40a4c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x4ab1f0u32, v); }
  if cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a4d9; continue;
 }
 0x40a4d9 => {
  { let v: u32 = cpu.r32(0x5de718u32); cpu.edx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5e18a0u32, v); }
  cpu.push(0x40a4f1); f_0040a760(cpu); if cpu.returned_to != 0x40a4f1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a4fc; continue;
 }
 0x40a4fc => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x6468544du32); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a50a; continue;
 }
 0x40a50a => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a516); f_0040a760(cpu); if cpu.returned_to != 0x40a516 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a521; continue;
 }
 0x40a521 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xff00u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0xff0000u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  if cpu.cf { pc = 0x40a73f; continue; }
  pc = 0x40a555; continue;
 }
 0x40a555 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a560); f_0040a760(cpu); if cpu.returned_to != 0x40a560 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a56b; continue;
 }
 0x40a56b => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18a8u32, v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5e18acu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5e18a4u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5bf); f_00402400(cpu); if cpu.returned_to != 0x40a5bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.eax; cpu.w32(0x5e18b0u32, v); }
  if cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a5cf; continue;
 }
 0x40a5cf => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x40a73b; continue; }
  pc = 0x40a5e2; continue;
 }
 0x40a5e2 => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x40a5e7; continue;
 }
 0x40a5e7 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5ed); f_00402400(cpu); if cpu.returned_to != 0x40a5ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a5fb; continue;
 }
 0x40a5fb => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a607); f_0040a760(cpu); if cpu.returned_to != 0x40a607 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a612; continue;
 }
 0x40a612 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x6b72544du32); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a620; continue;
 }
 0x40a620 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a62c); f_0040a760(cpu); if cpu.returned_to != 0x40a62c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a637; continue;
 }
 0x40a637 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xff00u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0xff0000u32; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a674); f_0040a0b0(cpu); if cpu.returned_to != 0x40a674 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a685; continue; }
  pc = 0x40a683; continue;
 }
 0x40a683 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40a685; continue;
 }
 0x40a685 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a69b); f_0040a100(cpu); if cpu.returned_to != 0x40a69b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a6a6; continue;
 }
 0x40a6a6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a6b2; continue;
 }
 0x40a6b2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x21u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a6d0); f_0040a0b0(cpu); if cpu.returned_to != 0x40a6d0 { return; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x40a6fa; continue; }
  pc = 0x40a6ed; continue;
 }
 0x40a6ed => {
  { cpu.sub32(cpu.r32(0x21u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if !cpu.zf { pc = 0x40a6fa; continue; }
  pc = 0x40a6f2; continue;
 }
 0x40a6f2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a721; continue;
 }
 0x40a6fa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a701); f_0040abe0(cpu); if cpu.returned_to != 0x40a701 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a73f; continue; }
  pc = 0x40a708; continue;
 }
 0x40a708 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a71e); f_0040a0b0(cpu); if cpu.returned_to != 0x40a71e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a721; continue;
 }
 0x40a721 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x25u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if cpu.cf { cpu.poll(); pc = 0x40a5e7; continue; }
  pc = 0x40a73b; continue;
 }
 0x40a73b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a73f; continue;
 }
 0x40a73f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.zf { pc = 0x40a74c; continue; }
  pc = 0x40a747; continue;
 }
 0x40a747 => {
  cpu.push(0x40a74c); f_0040a7a0(cpu); if cpu.returned_to != 0x40a74c { return; }
  pc = 0x40a74c; continue;
 }
 0x40a74c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a760); }
 let mut pc: u32 = 0x40a760;
 loop { match pc {
 0x40a760 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a77e); f_0040a100(cpu); if cpu.returned_to != 0x40a77e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a78f; continue; }
  pc = 0x40a785; continue;
 }
 0x40a785 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), cpu.esi); }
  if !cpu.zf { pc = 0x40a78f; continue; }
  pc = 0x40a78b; continue;
 }
 0x40a78b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a78f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7a0); }
 let mut pc: u32 = 0x40a7a0;
 loop { match pc {
 0x40a7a0 => {
  { let v: u32 = cpu.r32(0x5de714u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x40a7be; continue; }
  pc = 0x40a7ab; continue;
 }
 0x40a7ab => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a7b1); f_00402420(cpu); if cpu.returned_to != 0x40a7b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5de714u32, v); }
  pc = 0x40a7be; continue;
 }
 0x40a7be => {
  { let v: u32 = cpu.r32(0x5e18b0u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4ab1f0u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a816; continue; }
  pc = 0x40a7d2; continue;
 }
 0x40a7d2 => {
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x40a803; continue; }
  pc = 0x40a7dd; continue;
 }
 0x40a7dd => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a7df; continue;
 }
 0x40a7df => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a7f6; continue; }
  pc = 0x40a7e7; continue;
 }
 0x40a7e7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a7ed); f_00402420(cpu); if cpu.returned_to != 0x40a7ed { return; }
  { let v: u32 = cpu.r32(0x5e18b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a7f6; continue;
 }
 0x40a7f6 => {
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x25u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x40a7df; continue; }
  pc = 0x40a803; continue;
 }
 0x40a803 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a809); f_00402420(cpu); if cpu.returned_to != 0x40a809 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5e18b0u32, v); }
  pc = 0x40a816; continue;
 }
 0x40a816 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a820); }
 let mut pc: u32 = 0x40a820;
 loop { match pc {
 0x40a820 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5e18b0u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5de6bcu32, v); }
  if (cpu.cf || cpu.zf) { pc = 0x40a918; continue; }
  pc = 0x40a842; continue;
 }
 0x40a842 => {
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a854); f_0040a0b0(cpu); if cpu.returned_to != 0x40a854 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x800u32); }
  { let v: u32 = 0x800u32; cpu.edi = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a868; continue; }
  pc = 0x40a866; continue;
 }
 0x40a866 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40a868; continue;
 }
 0x40a868 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.ecx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a87f); f_0040a100(cpu); if cpu.returned_to != 0x40a87f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a929; continue; }
  pc = 0x40a88a; continue;
 }
 0x40a88a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { pc = 0x40a929; continue; }
  pc = 0x40a896; continue;
 }
 0x40a896 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x21u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a8b5); f_0040a0b0(cpu); if cpu.returned_to != 0x40a8b5 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x40a8df; continue; }
  pc = 0x40a8d2; continue;
 }
 0x40a8d2 => {
  { cpu.sub32(cpu.r32(0x21u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if !cpu.zf { pc = 0x40a8df; continue; }
  pc = 0x40a8d7; continue;
 }
 0x40a8d7 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a907; continue;
 }
 0x40a8df => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a8e6); f_0040abe0(cpu); if cpu.returned_to != 0x40a8e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a929; continue; }
  pc = 0x40a8ed; continue;
 }
 0x40a8ed => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4ab1f0u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a904); f_0040a0b0(cpu); if cpu.returned_to != 0x40a904 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a907; continue;
 }
 0x40a907 => {
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x25u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x40a842; continue; }
  pc = 0x40a918; continue;
 }
 0x40a918 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a929 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0040a940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a940); }
 let mut pc: u32 = 0x40a940;
 loop { match pc {
 0x40a940 => {
  { let v: u32 = cpu.r32(0x5de708u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x40a998; continue; }
  pc = 0x40a953; continue;
 }
 0x40a953 => {
  { cpu.sub32(cpu.r32(0x5de70cu32), cpu.ebp); }
  if !cpu.zf { pc = 0x40a976; continue; }
  pc = 0x40a95b; continue;
 }
 0x40a95b => {
  cpu.push(0x40a960); f_0040a820(cpu); if cpu.returned_to != 0x40a960 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de70cu32, v); }
  pc = 0x40a976; continue;
 }
 0x40a976 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a97d); f_0040a200(cpu); if cpu.returned_to != 0x40a97d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a998; continue; }
  pc = 0x40a984; continue;
 }
 0x40a984 => {
  { cpu.sub32(cpu.r32(0x5de6f0u32), cpu.ebp); }
  if !cpu.zf { pc = 0x40a998; continue; }
  pc = 0x40a98c; continue;
 }
 0x40a98c => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5de708u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de70cu32, v); }
  pc = 0x40a998; continue;
 }
 0x40a998 => {
  { cpu.sub32(cpu.r32(0x5de708u32), cpu.ebp); }
  if cpu.zf { pc = 0x40a9a7; continue; }
  pc = 0x40a9a0; continue;
 }
 0x40a9a0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a9a7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a9e7; continue; }
  pc = 0x40a9b2; continue;
 }
 0x40a9b2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6d4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6c8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5de6ccu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6d0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6d8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6c0u32, v); }
  pc = 0x40a9f8; continue;
 }
 0x40a9e7 => {
  { let v: u32 = cpu.r32(0x5de6ccu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5de6d4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5de6b8u32); cpu.edx = v; }
  pc = 0x40a9f8; continue;
 }
 0x40a9f8 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40ab4e; continue; }
  pc = 0x40aa01; continue;
 }
 0x40aa01 => {
  { cpu.sub32(cpu.r32(0x5ddba0u32), cpu.ebp); }
  if cpu.zf { pc = 0x40ab44; continue; }
  pc = 0x40aa0d; continue;
 }
 0x40aa0d => {
  cpu.push(0x40aa12); f_0040a820(cpu); if cpu.returned_to != 0x40aa12 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5eaac8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5de6b8u32, v); }
  pc = 0x40aa1e; continue;
 }
 0x40aa1e => {
  { let v: u32 = cpu.r32(0x5e18acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5e18b0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edx | 0xffffffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  { let v: u32 = cpu.esi; cpu.w32(0x5de6d8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6c0u32, v); }
  if (cpu.cf || cpu.zf) { pc = 0x40aa6c; continue; }
  pc = 0x40aa43; continue;
 }
 0x40aa43 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x40aa45; continue;
 }
 0x40aa45 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x40aa61; continue; }
  pc = 0x40aa4a; continue;
 }
 0x40aa4a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.cf { pc = 0x40aa61; continue; }
  pc = 0x40aa51; continue;
 }
 0x40aa51 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6f4u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5de6d8u32, v); }
  pc = 0x40aa61; continue;
 }
 0x40aa61 => {
  { let v: u32 = cpu.eax.wrapping_add(0x25u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6c0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40aa45; continue; }
  pc = 0x40aa6c; continue;
 }
 0x40aa6c => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x40aadc; continue; }
  pc = 0x40aa70; continue;
 }
 0x40aa70 => {
  { let v = 0x5de6c8u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40aa7b); f_0040ac50(cpu); if cpu.returned_to != 0x40aa7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40aaef; continue; }
  pc = 0x40aa82; continue;
 }
 0x40aa82 => {
  { let v: u32 = cpu.r32(0x5de6ccu32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x40aab7; continue; }
  pc = 0x40aa8b; continue;
 }
 0x40aa8b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x40aab7; continue; }
  pc = 0x40aa90; continue;
 }
 0x40aa90 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40aa98; continue;
 }
 0x40aa98 => {
  { let v: u32 = cpu.r32(0x5de6d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x40aa9f; continue;
 }
 0x40aa9f => {
  cpu.push(0x40aaa4); f_00407510(cpu); if cpu.returned_to != 0x40aaa4 { return; }
  { let v: u32 = cpu.r32(0x5de704u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de704u32, v); }
  cpu.poll(); pc = 0x40aa1e; continue;
 }
 0x40aab7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x5de6c8u32; cpu.push(v); }
  cpu.push(0x40aac2); f_0040aff0(cpu); if cpu.returned_to != 0x40aac2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40aacd; continue;
 }
 0x40aacd => {
  { cpu.sub32(cpu.eax, 0xffffff98u32); }
  if cpu.zf { pc = 0x40ab05; continue; }
  pc = 0x40aad2; continue;
 }
 0x40aad2 => {
  { cpu.sub32(cpu.eax, 0xffffff97u32); }
  if !cpu.zf { pc = 0x40ab18; continue; }
  pc = 0x40aad7; continue;
 }
 0x40aad7 => {
  cpu.poll(); pc = 0x40aa1e; continue;
 }
 0x40aadc => {
  { let v: u32 = cpu.r32(0x5de6b8u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaef => {
  { let v: u32 = cpu.r32(0x5de6b8u32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = 0xffffff9bu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab05 => {
  { let v: u32 = cpu.r32(0x5de6b8u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab18 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { pc = 0x40ab3a; continue; }
  pc = 0x40ab20; continue;
 }
 0x40ab20 => {
  { let v: u32 = cpu.r32(0x5de6d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x40ab27; continue;
 }
 0x40ab27 => {
  cpu.push(0x40ab2c); f_00407510(cpu); if cpu.returned_to != 0x40ab2c { return; }
  { let v: u32 = cpu.r32(0x5de704u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de704u32, v); }
  pc = 0x40ab3a; continue;
 }
 0x40ab3a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab44 => {
  { let v: u32 = 0xffffff99u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab4e => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x40ab5d; continue; }
  pc = 0x40ab53; continue;
 }
 0x40ab53 => {
  { let v: u32 = 0xffffff9au32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab5d => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40ab66; continue;
 }
 0x40ab66 => {
  { let v: u32 = cpu.edx ^ 0x4u32; cpu.edx = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  { let v: u32 = cpu.edx; cpu.w32(0x5de6b8u32, v); }
  if !cpu.zf { pc = 0x40ab8b; continue; }
  pc = 0x40ab74; continue;
 }
 0x40ab74 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x40ab8b; continue; }
  pc = 0x40ab79; continue;
 }
 0x40ab79 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40ab85; continue;
 }
 0x40ab85 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x40aa9f; continue;
 }
 0x40ab8b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x5de6c8u32; cpu.push(v); }
  cpu.push(0x40ab96); f_0040aff0(cpu); if cpu.returned_to != 0x40ab96 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40aba1; continue;
 }
 0x40aba1 => {
  { cpu.sub32(cpu.eax, 0xffffff98u32); }
  if !cpu.zf { pc = 0x40abb9; continue; }
  pc = 0x40aba6; continue;
 }
 0x40aba6 => {
  { let v: u32 = cpu.r32(0x5de6b8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de6b8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abb9 => {
  { cpu.sub32(cpu.eax, 0xffffff97u32); }
  if cpu.zf { cpu.poll(); pc = 0x40aa1e; continue; }
  pc = 0x40abc2; continue;
 }
 0x40abc2 => {
  { cpu.sub32(cpu.r32(0x5de704u32), cpu.ebp); }
  if cpu.zf { cpu.poll(); pc = 0x40ab3a; continue; }
  pc = 0x40abce; continue;
 }
 0x40abce => {
  { let v: u32 = cpu.r32(0x5de6d4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x40ab27; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
