#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010572c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10572c5); }
 let mut pc: u32 = 0x10572c5;
 loop { match pc {
 0x10572c5 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x10572cf); f_010352e0(cpu); if cpu.returned_to != 0x10572cf { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e55u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x10572de); f_010352e0(cpu); if cpu.returned_to != 0x10572de { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e56u32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x10572ed); f_010352e0(cpu); if cpu.returned_to != 0x10572ed { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e57u32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x10572fc); f_010352e0(cpu); if cpu.returned_to != 0x10572fc { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e58u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057302(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057302); }
 let mut pc: u32 = 0x1057302;
 loop { match pc {
 0x1057302 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10c6876u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105730a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105730a); }
 let mut pc: u32 = 0x105730a;
 loop { match pc {
 0x105730a => {
  { cpu.sub16(cpu.r16(0x10e3ce3u32), 0xffffu16); }
  if cpu.zf { pc = 0x105735a; continue; }
  pc = 0x1057314; continue;
 }
 0x1057314 => {
  { let v: u16 = cpu.r16(0x10e3ce3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ce5u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee97du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10c6876u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1057333); f_01057625(cpu); if cpu.returned_to != 0x1057333 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee97du32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10c6876u32, v); }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e3ce3u32)); }
  if !cpu.zf { pc = 0x105735a; continue; }
  pc = 0x1057348; continue;
 }
 0x1057348 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e3ce5u32)); }
  if !cpu.zf { pc = 0x105735a; continue; }
  pc = 0x1057351; continue;
 }
 0x1057351 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  pc = 0x105735a; continue;
 }
 0x105735a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105735b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105735b); }
 let mut pc: u32 = 0x105735b;
 loop { match pc {
 0x105735b => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10574d5; continue; }
  pc = 0x1057368; continue;
 }
 0x1057368 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1057381; continue; }
  pc = 0x1057380; continue;
 }
 0x1057380 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057381 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91cu32), 0x0u8); }
  if cpu.zf { pc = 0x10574d5; continue; }
  pc = 0x10573b1; continue;
 }
 0x10573b1 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x10574d5; continue; }
  pc = 0x10573be; continue;
 }
 0x10573be => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x10574d5; continue; }
  pc = 0x10573cb; continue;
 }
 0x10573cb => {
  { cpu.sub16(cpu.r16(0x10e3ce3u32), 0xffffu16); }
  if !cpu.zf { pc = 0x10573ed; continue; }
  pc = 0x10573d5; continue;
 }
 0x10573d5 => {
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce9u32, v); }
  pc = 0x10573ed; continue;
 }
 0x10573ed => {
  { let v: u32 = cpu.r32(0x11296ccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3cddu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11296d0u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10574d5; continue; }
  pc = 0x105740f; continue;
 }
 0x105740f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10574d5; continue; }
  pc = 0x105741f; continue;
 }
 0x105741f => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r16(0x10e3d19u32); cpu.mul16(v); }
  { let v = cpu.r16(0x111a974u32); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11296d4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x111a970u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3cdbu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11296d8u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10574d5; continue; }
  pc = 0x1057455; continue;
 }
 0x1057455 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10574d5; continue; }
  pc = 0x1057461; continue;
 }
 0x1057461 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r16(0x10e3d17u32); cpu.mul16(v); }
  { let v = cpu.r16(0x111a970u32); cpu.div16(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x70u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x20u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105748c; continue; }
  pc = 0x1057484; continue;
 }
 0x1057484 => {
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  pc = 0x105748c; continue;
 }
 0x105748c => {
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if !cpu.zf { pc = 0x10574a1; continue; }
  pc = 0x1057496; continue;
 }
 0x1057496 => {
  { let v: u32 = cpu.eax & 0xfffcu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfffcu32); cpu.ebx = v; }
  pc = 0x10574a1; continue;
 }
 0x10574a1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce5u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10c6876u32, v); }
  cpu.push(0x10574c1); f_01057a4c(cpu); if cpu.returned_to != 0x10574c1 { return; }
  cpu.push(0x10574c6); f_0103d12a(cpu); if cpu.returned_to != 0x10574c6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10574d5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010574dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10574dd); }
 let mut pc: u32 = 0x10574dd;
 loop { match pc {
 0x10574dd => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x105761d; continue; }
  pc = 0x10574ea; continue;
 }
 0x10574ea => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1057501; continue; }
  pc = 0x1057500; continue;
 }
 0x1057500 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057501 => {
  { cpu.sub8(cpu.r8(0x10c6876u32), 0x0u8); }
  if !cpu.zf { pc = 0x105761d; continue; }
  pc = 0x105750e; continue;
 }
 0x105750e => {
  { cpu.sub16(cpu.r16(0x10e3ce3u32), 0xffffu16); }
  if !cpu.zf { pc = 0x1057530; continue; }
  pc = 0x1057518; continue;
 }
 0x1057518 => {
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce9u32, v); }
  pc = 0x1057530; continue;
 }
 0x1057530 => {
  { let v: u16 = cpu.r16(0x10e690cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e690eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 5u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d17u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d19u32, v); }
  { let v: u32 = 0x34u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3cddu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x11u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x105761d; continue; }
  pc = 0x1057577; continue;
 }
 0x1057577 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105761d; continue; }
  pc = 0x1057587; continue;
 }
 0x1057587 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r16(0x10e3d19u32); cpu.mul16(v); }
  { let v = cpu.r16(0x111a974u32); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x111a970u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3cdbu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x11u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x105761d; continue; }
  pc = 0x10575b6; continue;
 }
 0x10575b6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105761d; continue; }
  pc = 0x10575c2; continue;
 }
 0x10575c2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r16(0x10e3d17u32); cpu.mul16(v); }
  { let v = cpu.r16(0x111a970u32); cpu.div16(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f60u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f64u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296e3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10c6876u32, v); }
  cpu.push(0x1057610); f_01057a4c(cpu); if cpu.returned_to != 0x1057610 { return; }
  cpu.push(0x1057615); f_0103d12a(cpu); if cpu.returned_to != 0x1057615 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105761d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057625(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057625); }
 let mut pc: u32 = 0x1057625;
 loop { match pc {
 0x1057625 => {
  { let v: u16 = cpu.r16(0x10e3ce7u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ce9u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  f_01057641(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057641(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057641); }
 let mut pc: u32 = 0x1057641;
 loop { match pc {
 0x1057641 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105764f; continue; }
  pc = 0x105764c; continue;
 }
 0x105764c => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105764f; continue;
 }
 0x105764f => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1057657; continue; }
  pc = 0x1057654; continue;
 }
 0x1057654 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1057657; continue;
 }
 0x1057657 => {
  { let v: u16 = cpu.r16(0x10e3d17u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(cpu.r16(0x10e4104u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.esi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105766d; continue; }
  pc = 0x105766a; continue;
 }
 0x105766a => {
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105766d; continue;
 }
 0x105766d => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(cpu.r16(0x10e4108u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1057683; continue; }
  pc = 0x1057680; continue;
 }
 0x1057680 => {
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1057683; continue;
 }
 0x1057683 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ce9u32, v); }
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if !cpu.zf { pc = 0x10576a3; continue; }
  pc = 0x105769b; continue;
 }
 0x105769b => {
  { let v: u16 = (cpu.ecx as u16) & 0xfffcu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.edx as u16), 0xfffcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10576a3; continue;
 }
 0x10576a3 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3f60u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3f64u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010576b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10576b2); }
 let mut pc: u32 = 0x10576b2;
 loop { match pc {
 0x10576b2 => {
  { let v: u32 = cpu.r16(0x10e3ce1u32) as u32; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x14u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010576d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10576d9); }
 let mut pc: u32 = 0x10576d9;
 loop { match pc {
 0x10576d9 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef1ffu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef1feu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105771b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105771b); }
 let mut pc: u32 = 0x105771b;
 loop { match pc {
 0x105771b => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x1057725; continue; }
  pc = 0x1057724; continue;
 }
 0x1057724 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057725 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x105772f; continue; }
  pc = 0x105772e; continue;
 }
 0x105772e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105772f => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x1057739; continue; }
  pc = 0x1057738; continue;
 }
 0x1057738 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057739 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1057750; continue; }
  pc = 0x105774f; continue;
 }
 0x105774f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057750 => {
  cpu.push(0x1057755); f_01057a4c(cpu); if cpu.returned_to != 0x1057755 { return; }
  cpu.push(0x105775a); f_0103d12a(cpu); if cpu.returned_to != 0x105775a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91au32), 0x0u8); }
  if !cpu.zf { pc = 0x10577a6; continue; }
  pc = 0x1057773; continue;
 }
 0x1057773 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u16 = cpu.r16(0x10e3d45u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3db5u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10e3db5u32, v); }
  { let v: u16 = cpu.r16(0x10e3d43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3db5u32), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10577a6; continue; }
  pc = 0x10577a0; continue;
 }
 0x10577a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3db5u32, v); }
  pc = 0x10577a6; continue;
 }
 0x10577a6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010577a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10577a7); }
 let mut pc: u32 = 0x10577a7;
 loop { match pc {
 0x10577a7 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x10577b1; continue; }
  pc = 0x10577b0; continue;
 }
 0x10577b0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10577b1 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x10577bb; continue; }
  pc = 0x10577ba; continue;
 }
 0x10577ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10577bb => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x10577c5; continue; }
  pc = 0x10577c4; continue;
 }
 0x10577c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10577c5 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10577dc; continue; }
  pc = 0x10577db; continue;
 }
 0x10577db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10577dc => {
  cpu.push(0x10577e1); f_01057a4c(cpu); if cpu.returned_to != 0x10577e1 { return; }
  cpu.push(0x10577e6); f_0103d12a(cpu); if cpu.returned_to != 0x10577e6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91au32), 0x0u8); }
  if !cpu.zf { pc = 0x105782f; continue; }
  pc = 0x10577ff; continue;
 }
 0x10577ff => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u16 = cpu.r16(0x10e3d45u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3db5u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3db5u32, v); }
  { let v: u16 = cpu.r16(0x10e3d43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3db5u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105782f; continue; }
  pc = 0x1057829; continue;
 }
 0x1057829 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3db5u32, v); }
  pc = 0x105782f; continue;
 }
 0x105782f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057830); }
 let mut pc: u32 = 0x1057830;
 loop { match pc {
 0x1057830 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x105783a; continue; }
  pc = 0x1057839; continue;
 }
 0x1057839 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105783a => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x1057844; continue; }
  pc = 0x1057843; continue;
 }
 0x1057843 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057844 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x105784e; continue; }
  pc = 0x105784d; continue;
 }
 0x105784d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105784e => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1057865; continue; }
  pc = 0x1057864; continue;
 }
 0x1057864 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057865 => {
  cpu.push(0x105786a); f_01057a4c(cpu); if cpu.returned_to != 0x105786a { return; }
  cpu.push(0x105786f); f_0103d12a(cpu); if cpu.returned_to != 0x105786f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee919u32), 0x0u8); }
  if !cpu.zf { pc = 0x10578bb; continue; }
  pc = 0x1057888; continue;
 }
 0x1057888 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee919u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u16 = cpu.r16(0x10e3d45u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3db3u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10e3db3u32, v); }
  { let v: u16 = cpu.r16(0x10e3d43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3db3u32), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10578bb; continue; }
  pc = 0x10578b5; continue;
 }
 0x10578b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3db3u32, v); }
  pc = 0x10578bb; continue;
 }
 0x10578bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010578bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10578bc); }
 let mut pc: u32 = 0x10578bc;
 loop { match pc {
 0x10578bc => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x10578c6; continue; }
  pc = 0x10578c5; continue;
 }
 0x10578c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10578c6 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x10578d0; continue; }
  pc = 0x10578cf; continue;
 }
 0x10578cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10578d0 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x10578da; continue; }
  pc = 0x10578d9; continue;
 }
 0x10578d9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10578da => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10578f1; continue; }
  pc = 0x10578f0; continue;
 }
 0x10578f0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10578f1 => {
  cpu.push(0x10578f6); f_01057a4c(cpu); if cpu.returned_to != 0x10578f6 { return; }
  cpu.push(0x10578fb); f_0103d12a(cpu); if cpu.returned_to != 0x10578fb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee919u32), 0x0u8); }
  if !cpu.zf { pc = 0x1057944; continue; }
  pc = 0x1057914; continue;
 }
 0x1057914 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee919u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u16 = cpu.r16(0x10e3d45u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3db3u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3db3u32, v); }
  { let v: u16 = cpu.r16(0x10e3d43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3db3u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1057944; continue; }
  pc = 0x105793e; continue;
 }
 0x105793e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3db3u32, v); }
  pc = 0x1057944; continue;
 }
 0x1057944 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057945(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057945); }
 let mut pc: u32 = 0x1057945;
 loop { match pc {
 0x1057945 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1057991; continue; }
  pc = 0x105794f; continue;
 }
 0x105794f => {
  cpu.push(0x1057954); f_010576b2(cpu); if cpu.returned_to != 0x1057954 { return; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105796c; continue; }
  pc = 0x105795b; continue;
 }
 0x105795b => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xfdu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x105796c; continue;
 }
 0x105796c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3dcdu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ce1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee92bu32, v); }
  cpu.push(0x1057991); f_01057a4c(cpu); if cpu.returned_to != 0x1057991 { return; }
  pc = 0x1057991; continue;
 }
 0x1057991 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057992(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057992); }
 let mut pc: u32 = 0x1057992;
 loop { match pc {
 0x1057992 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x10579c5; continue; }
  pc = 0x105799c; continue;
 }
 0x105799c => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r16(0x10e3ce1u32); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152d9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x10579bb; continue; }
  pc = 0x10579b5; continue;
 }
 0x10579b5 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x10579fb; continue; }
  pc = 0x10579bb; continue;
 }
 0x10579bb => {
  cpu.push(0x10579c0); f_01057945(cpu); if cpu.returned_to != 0x10579c0 { return; }
  cpu.push(0x10579c5); f_010599d1(cpu); if cpu.returned_to != 0x10579c5 { return; }
  pc = 0x10579c5; continue;
 }
 0x10579c5 => {
  { cpu.sub16(cpu.r16(0x10e3de9u32), 0x0u16); }
  if cpu.zf { pc = 0x10579fa; continue; }
  pc = 0x10579cf; continue;
 }
 0x10579cf => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r16(0x10e3de9u32); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152d9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x10579fa; continue; }
  pc = 0x10579e8; continue;
 }
 0x10579e8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3debu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3de9u32, v); }
  pc = 0x10579fa; continue;
 }
 0x10579fa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10579fb => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x10579bb; continue; }
  pc = 0x1057a03; continue;
 }
 0x1057a03 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x10579bb; continue; }
  pc = 0x1057a0b; continue;
 }
 0x1057a0b => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x800u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10579bb; continue; }
  pc = 0x1057a13; continue;
 }
 0x1057a13 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x800u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10579bb; continue; }
  pc = 0x1057a1b; continue;
 }
 0x1057a1b => {
  cpu.poll(); pc = 0x10579c5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057a1d); }
 let mut pc: u32 = 0x1057a1d;
 loop { match pc {
 0x1057a1d => {
  { cpu.sub16(cpu.r16(0x10e3de9u32), 0x0u16); }
  if cpu.zf { pc = 0x1057a31; continue; }
  pc = 0x1057a27; continue;
 }
 0x1057a27 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3de9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057a31 => {
  { let v: u16 = cpu.r16(0x10e3debu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1057a4b; continue; }
  pc = 0x1057a3c; continue;
 }
 0x1057a3c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3de9u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  pc = 0x1057a4b; continue;
 }
 0x1057a4b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057a4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057a4c); }
 let mut pc: u32 = 0x1057a4c;
 loop { match pc {
 0x1057a4c => {
  { cpu.sub16(cpu.r16(0x10e3de9u32), 0x0u16); }
  if cpu.zf { pc = 0x1057a5b; continue; }
  pc = 0x1057a56; continue;
 }
 0x1057a56 => {
  cpu.push(0x1057a5b); f_01057a1d(cpu); if cpu.returned_to != 0x1057a5b { return; }
  pc = 0x1057a5b; continue;
 }
 0x1057a5b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057a5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057a5c); }
 let mut pc: u32 = 0x1057a5c;
 loop { match pc {
 0x1057a5c => {
  { cpu.sub8(cpu.r8(0x10ee984u32), 0x2u8); }
  if cpu.zf { pc = 0x1057a6e; continue; }
  pc = 0x1057a65; continue;
 }
 0x1057a65 => {
  { cpu.sub8(cpu.r8(0x10ee984u32), 0x0u8); }
  if !cpu.zf { pc = 0x1057a7c; continue; }
  pc = 0x1057a6e; continue;
 }
 0x1057a6e => {
  { cpu.sub16(cpu.r16(0x10e3de9u32), 0x0u16); }
  if !cpu.zf { pc = 0x1057ba4; continue; }
  pc = 0x1057a7c; continue;
 }
 0x1057a7c => {
  { cpu.sub16(cpu.r16(0x10e3ce3u32), 0xffffu16); }
  if !cpu.zf { pc = 0x1057ba3; continue; }
  pc = 0x1057a8a; continue;
 }
 0x1057a8a => {
  { let v: u16 = cpu.r16(0x10e3d17u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e4104u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x10e3db3u32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e3db5u32) as i16 as i32 as u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x11cc99eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1057abf; continue; }
  pc = 0x1057aaf; continue;
 }
 0x1057aaf => {
  { let v: u32 = cpu.r8(0x112970fu32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x112970fu32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.edx = v; }
  pc = 0x1057ad9; continue;
 }
 0x1057abf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x1130f1du32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1129738u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1057ad9; continue;
 }
 0x1057ad9 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1057afa; continue; }
  pc = 0x1057ade; continue;
 }
 0x1057ade => {
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1057aec; continue; }
  pc = 0x1057ae9; continue;
 }
 0x1057ae9 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1057aec; continue;
 }
 0x1057aec => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1057af4; continue; }
  pc = 0x1057af1; continue;
 }
 0x1057af1 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1057af4; continue;
 }
 0x1057af4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f60u32, v); }
  pc = 0x1057afa; continue;
 }
 0x1057afa => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1057b29; continue; }
  pc = 0x1057aff; continue;
 }
 0x1057aff => {
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1057b0d; continue; }
  pc = 0x1057b0a; continue;
 }
 0x1057b0a => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1057b0d; continue;
 }
 0x1057b0d => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(cpu.r16(0x10e4108u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebp as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1057b23; continue; }
  pc = 0x1057b20; continue;
 }
 0x1057b20 => {
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1057b23; continue;
 }
 0x1057b23 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f64u32, v); }
  pc = 0x1057b29; continue;
 }
 0x1057b29 => {
  { let v: u16 = cpu.r16(0x10e3db3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3db5u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if cpu.zf { pc = 0x1057b4b; continue; }
  pc = 0x1057b46; continue;
 }
 0x1057b46 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x1057b4b; continue;
 }
 0x1057b4b => {
  { cpu.sub8(cpu.r8(0x10ee919u32), 0x0u8); }
  if !cpu.zf { pc = 0x1057b70; continue; }
  pc = 0x1057b54; continue;
 }
 0x1057b54 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1057b70; continue; }
  pc = 0x1057b59; continue;
 }
 0x1057b59 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11cc99eu32), 0x0u8); }
  if cpu.zf { pc = 0x1057b68; continue; }
  pc = 0x1057b65; continue;
 }
 0x1057b65 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1057b68; continue;
 }
 0x1057b68 => {
  { cpu.sub16((cpu.ecx as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x1057b70; continue; }
  pc = 0x1057b6e; continue;
 }
 0x1057b6e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1057b70; continue;
 }
 0x1057b70 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3db3u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91au32), 0x0u8); }
  if !cpu.zf { pc = 0x1057b9c; continue; }
  pc = 0x1057b80; continue;
 }
 0x1057b80 => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1057b9c; continue; }
  pc = 0x1057b85; continue;
 }
 0x1057b85 => {
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11cc99eu32), 0x0u8); }
  if cpu.zf { pc = 0x1057b94; continue; }
  pc = 0x1057b91; continue;
 }
 0x1057b91 => {
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1057b94; continue;
 }
 0x1057b94 => {
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x1057b9c; continue; }
  pc = 0x1057b9a; continue;
 }
 0x1057b9a => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x1057b9c; continue;
 }
 0x1057b9c => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3db5u32, v); }
  pc = 0x1057ba3; continue;
 }
 0x1057ba3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057ba4 => {
  { cpu.sub8(cpu.r8(0x10ee92cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1057ca2; continue; }
  pc = 0x1057bb1; continue;
 }
 0x1057bb1 => {
  { cpu.sub8(cpu.r8(0x10c6876u32), 0x0u8); }
  if cpu.zf { pc = 0x1057bc7; continue; }
  pc = 0x1057bba; continue;
 }
 0x1057bba => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if !cpu.zf { pc = 0x1057ca2; continue; }
  pc = 0x1057bc7; continue;
 }
 0x1057bc7 => {
  { let v: u16 = cpu.r16(0x10e3de9u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1057c00; continue; }
  pc = 0x1057bf8; continue;
 }
 0x1057bf8 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1057c00; continue;
 }
 0x1057c00 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3f60u32), 0x0u16); }
  if cpu.zf { pc = 0x1057c24; continue; }
  pc = 0x1057c1e; continue;
 }
 0x1057c1e => {
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1057c6a; continue; }
  pc = 0x1057c24; continue;
 }
 0x1057c24 => {
  { let v: u16 = cpu.r16(0x10e3d17u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x120u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x10e3f60u32)); }
  if cpu.zf { pc = 0x1057c3f; continue; }
  pc = 0x1057c39; continue;
 }
 0x1057c39 => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1057c6a; continue; }
  pc = 0x1057c3f; continue;
 }
 0x1057c3f => {
  { cpu.sub16(cpu.r16(0x10e3f64u32), 0x0u16); }
  if cpu.zf { pc = 0x1057c4f; continue; }
  pc = 0x1057c49; continue;
 }
 0x1057c49 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1057c6a; continue; }
  pc = 0x1057c4f; continue;
 }
 0x1057c4f => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0xb8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x10e3f64u32)); }
  if cpu.zf { pc = 0x1057ca2; continue; }
  pc = 0x1057c64; continue;
 }
 0x1057c64 => {
  { cpu.sub16((cpu.ebx as u16), 0x20u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1057ca2; continue; }
  pc = 0x1057c6a; continue;
 }
 0x1057c6a => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1057c73; continue; }
  pc = 0x1057c70; continue;
 }
 0x1057c70 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1057c73; continue;
 }
 0x1057c73 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1057c7c; continue; }
  pc = 0x1057c79; continue;
 }
 0x1057c79 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1057c7c; continue;
 }
 0x1057c7c => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ce5u32, v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce9u32, v); }
  pc = 0x1057ca2; continue;
 }
 0x1057ca2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057ca3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057ca3); }
 let mut pc: u32 = 0x1057ca3;
 loop { match pc {
 0x1057ca3 => {
  { cpu.sub16(cpu.r16(0x10e3dafu32), 0x0u16); }
  if cpu.zf { pc = 0x1057cdc; continue; }
  pc = 0x1057cad; continue;
 }
 0x1057cad => {
  if cpu.sf { pc = 0x1057cdd; continue; }
  pc = 0x1057caf; continue;
 }
 0x1057caf => {
  cpu.push(0x1057cb4); f_0104f3c9(cpu); if cpu.returned_to != 0x1057cb4 { return; }
  { cpu.sub8(cpu.r8(0x10ee97cu32), 0x0u8); }
  if cpu.zf { pc = 0x1057cc2; continue; }
  pc = 0x1057cbd; continue;
 }
 0x1057cbd => {
  cpu.push(0x1057cc2); f_0104f3e2(cpu); if cpu.returned_to != 0x1057cc2 { return; }
  pc = 0x1057cc2; continue;
 }
 0x1057cc2 => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dafu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1057cdc); cpu.call(t); if cpu.returned_to != 0x1057cdc { return; }
  pc = 0x1057cdc; continue;
 }
 0x1057cdc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057cdd => {
  { let v: u16 = cpu.r16(0x10e3dafu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1057ced); f_01057d05(cpu); if cpu.returned_to != 0x1057ced { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1057d04); cpu.call(t); if cpu.returned_to != 0x1057d04 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057d05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057d05); }
 let mut pc: u32 = 0x1057d05;
 loop { match pc {
 0x1057d05 => {
  { let v: u16 = 0x19u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x10c5974u32); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057d3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057d3c); }
 let mut pc: u32 = 0x1057d3c;
 loop { match pc {
 0x1057d3c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1057d63; continue; }
  pc = 0x1057d56; continue;
 }
 0x1057d56 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1057d63; continue;
 }
 0x1057d63 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057d72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1057d72); }
 let mut pc: u32 = 0x1057d72;
 loop { match pc {
 0x1057d72 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1057d99; continue; }
  pc = 0x1057d8c; continue;
 }
 0x1057d8c => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1057d99; continue;
 }
 0x1057d99 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01057d9a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1057d9a); } r_01057d9a(cpu, 0x1057d9a); }
pub fn f_0105840f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105840f); } r_01057d9a(cpu, 0x105840f); }
fn r_01057d9a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1057d9a => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if cpu.zf { pc = 0x1057dc0; continue; }
  pc = 0x1057da3; continue;
 }
 0x1057da3 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1057dc1; continue; }
  pc = 0x1057dac; continue;
 }
 0x1057dac => {
  { cpu.sub8(cpu.r8(0x1124e87u32), 0x0u8); }
  if !cpu.zf { pc = 0x1057dc0; continue; }
  pc = 0x1057db5; continue;
 }
 0x1057db5 => {
  { cpu.sub8(cpu.r8(0x10ef20fu32), 0xffu8); }
  if !cpu.zf { pc = 0x1057dc0; continue; }
  pc = 0x1057dbe; continue;
 }
 0x1057dbe => {
  pc = 0x1057dc1; continue;
 }
 0x1057dc0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057dc1 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x105867e; continue; }
  pc = 0x1057dce; continue;
 }
 0x1057dce => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcbedu32, v); }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x2u8); }
  if cpu.zf { pc = 0x1057dfe; continue; }
  pc = 0x1057dfd; continue;
 }
 0x1057dfd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057dfe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1057e25; continue; }
  pc = 0x1057e18; continue;
 }
 0x1057e18 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1057e25; continue;
 }
 0x1057e25 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e9du32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11296c4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11296c6u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11296c8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11296cau32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef37du32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.sf { pc = 0x105872b; continue; }
  pc = 0x1057e7b; continue;
 }
 0x1057e7b => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x1057e8a; continue; }
  pc = 0x1057e84; continue;
 }
 0x1057e84 => {
  cpu.push(0x1057e89); f_01058881(cpu); if cpu.returned_to != 0x1057e89 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057e8a => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x1057e99; continue; }
  pc = 0x1057e93; continue;
 }
 0x1057e93 => {
  cpu.push(0x1057e98); f_01016708(cpu); if cpu.returned_to != 0x1057e98 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057e99 => {
  { cpu.pushad(); }
  cpu.push(0x1057e9f); f_01017635(cpu); if cpu.returned_to != 0x1057e9f { return; }
  if !cpu.cf { pc = 0x1057ee9; continue; }
  pc = 0x1057ea1; continue;
 }
 0x1057ea1 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1057ea7); f_010586d4(cpu); if cpu.returned_to != 0x1057ea7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1057eb9; continue; }
  pc = 0x1057eb8; continue;
 }
 0x1057eb8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057eb9 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110eab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  cpu.push(0x1057ed2); f_010863bd(cpu); if cpu.returned_to != 0x1057ed2 { return; }
  cpu.push(0x1057ed7); f_01015c29(cpu); if cpu.returned_to != 0x1057ed7 { return; }
  { let v: u16 = cpu.r16(0x10e3ce3u32).wrapping_sub(0x40u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3ce5u32), 0x20u16); cpu.w16(0x10e3ce5u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057ee9 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x11296c4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11296c6u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1057efd); f_0103357f(cpu); if cpu.returned_to != 0x1057efd { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1057f05; continue; }
  pc = 0x1057f00; continue;
 }
 0x1057f00 => {
  pc = 0x105867e; continue;
 }
 0x1057f05 => {
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if cpu.zf { pc = 0x1057f16; continue; }
  pc = 0x1057f0e; continue;
 }
 0x1057f0e => {
  cpu.push(0x1057f13); f_01058831(cpu); if cpu.returned_to != 0x1057f13 { return; }
  if !cpu.cf { pc = 0x1057f16; continue; }
  pc = 0x1057f15; continue;
 }
 0x1057f15 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1057f16 => {
  { cpu.sub8(cpu.r8(0x10e3d5du32), 0x2u8); }
  if !cpu.zf { pc = 0x1057f31; continue; }
  pc = 0x1057f1f; continue;
 }
 0x1057f1f => {
  cpu.push(0x1057f24); f_01057d3c(cpu); if cpu.returned_to != 0x1057f24 { return; }
  { cpu.pushad(); }
  cpu.push(0x1057f2a); f_0101b70a(cpu); if cpu.returned_to != 0x1057f2a { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105827b; continue; }
  pc = 0x1057f31; continue;
 }
 0x1057f31 => {
  { cpu.sub8(cpu.r8(0x10e3d5du32), 0x5u8); }
  if !cpu.zf { pc = 0x1057f50; continue; }
  pc = 0x1057f3a; continue;
 }
 0x1057f3a => {
  cpu.push(0x1057f3f); f_010576b2(cpu); if cpu.returned_to != 0x1057f3f { return; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1057f50; continue; }
  pc = 0x1057f45; continue;
 }
 0x1057f45 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1057f50; continue; }
  pc = 0x1057f4b; continue;
 }
 0x1057f4b => {
  pc = 0x10583d9; continue;
 }
 0x1057f50 => {
  cpu.push(0x1057f55); f_01068e69(cpu); if cpu.returned_to != 0x1057f55 { return; }
  if !cpu.cf { pc = 0x1057f62; continue; }
  pc = 0x1057f57; continue;
 }
 0x1057f57 => {
  cpu.push(0x1057f5c); f_01068db3(cpu); if cpu.returned_to != 0x1057f5c { return; }
  if cpu.cf { pc = 0x1058032; continue; }
  pc = 0x1057f62; continue;
 }
 0x1057f62 => {
  { let v: u16 = 0xf1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1058032; continue; }
  pc = 0x1057f88; continue;
 }
 0x1057f88 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1058032; continue; }
  pc = 0x1057f92; continue;
 }
 0x1057f92 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x40u8); }
  { cpu.sub8(cpu.r8(0x10ef371u32), 0x0u8); }
  if cpu.zf { pc = 0x1057fb2; continue; }
  pc = 0x1057f9f; continue;
 }
 0x1057f9f => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if !cpu.zf { pc = 0x1057fb2; continue; }
  pc = 0x1057fa9; continue;
 }
 0x1057fa9 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1057fce; continue; }
  pc = 0x1057fb2; continue;
 }
 0x1057fb2 => {
  cpu.push(0x1057fb7); f_01058cae(cpu); if cpu.returned_to != 0x1057fb7 { return; }
  if cpu.cf { pc = 0x10583eb; continue; }
  pc = 0x1057fbd; continue;
 }
 0x1057fbd => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10583eb; continue; }
  pc = 0x1057fce; continue;
 }
 0x1057fce => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1057fe4; continue; }
  pc = 0x1057fd4; continue;
 }
 0x1057fd4 => {
  { let v: u32 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1152e54u32); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x1057ff0; continue; }
  pc = 0x1057fe4; continue;
 }
 0x1057fe4 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x1058032; continue; }
  pc = 0x1057fea; continue;
 }
 0x1057fea => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1057ff5; continue; }
  pc = 0x1057ff0; continue;
 }
 0x1057ff0 => {
  cpu.push(0x1057ff5); f_010594f8(cpu); if cpu.returned_to != 0x1057ff5 { return; }
  pc = 0x1057ff5; continue;
 }
 0x1057ff5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x105800e); f_01059200(cpu); if cpu.returned_to != 0x105800e { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10fcc0eu32), cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1058031; continue; }
  pc = 0x105801d; continue;
 }
 0x105801d => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc0eu32, v); }
  pc = 0x1058031; continue;
 }
 0x1058031 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058032 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10581a2; continue; }
  pc = 0x1058046; continue;
 }
 0x1058046 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1058061; continue; }
  pc = 0x105805c; continue;
 }
 0x105805c => {
  pc = 0x10581a2; continue;
 }
 0x1058061 => {
  { cpu.sub32(cpu.r32(0x110eab8u32), 0x0u32); }
  if cpu.zf { pc = 0x10581a2; continue; }
  pc = 0x105806e; continue;
 }
 0x105806e => {
  cpu.push(0x1058073); f_01057d72(cpu); if cpu.returned_to != 0x1058073 { return; }
  { let v: u16 = 0x40u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0xe8u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x110ed91u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10580c4; continue; }
  pc = 0x10580a0; continue;
 }
 0x10580a0 => {
  { let v: u16 = 0xe0u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0x188u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x110ed91u32, v); }
  pc = 0x10580c4; continue;
 }
 0x10580c4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8bu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10581a2; continue; }
  pc = 0x10580d1; continue;
 }
 0x10580d1 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8du32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10581a2; continue; }
  pc = 0x10580de; continue;
 }
 0x10580de => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed8fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10581a2; continue; }
  pc = 0x10580eb; continue;
 }
 0x10580eb => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed91u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10581a2; continue; }
  pc = 0x10580f8; continue;
 }
 0x10580f8 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1058107; continue; }
  pc = 0x1058102; continue;
 }
 0x1058102 => {
  cpu.push(0x1058107); f_01057945(cpu); if cpu.returned_to != 0x1058107 { return; }
  pc = 0x1058107; continue;
 }
 0x1058107 => {
  { cpu.sub32(cpu.r32(0x110eac0u32), 0x4d2u32); }
  if !cpu.zf { pc = 0x1058124; continue; }
  pc = 0x1058113; continue;
 }
 0x1058113 => {
  cpu.push(0x1058118); f_01002d99(cpu); if cpu.returned_to != 0x1058118 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  pc = 0x1058172; continue;
 }
 0x1058124 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10fcc0eu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x1058144; continue; }
  pc = 0x1058131; continue;
 }
 0x1058131 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1058144; continue; }
  pc = 0x1058137; continue;
 }
 0x1058137 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1058142); f_01059200(cpu); if cpu.returned_to != 0x1058142 { return; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1058144; continue;
 }
 0x1058144 => {
  { let v: u32 = cpu.r32(0x110eac0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1058151); f_010586d4(cpu); if cpu.returned_to != 0x1058151 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc0eu32, v); }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x1058172; continue; }
  pc = 0x105816c; continue;
 }
 0x105816c => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1058173; continue; }
  pc = 0x1058172; continue;
 }
 0x1058172 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058173 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110eab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  cpu.push(0x105818c); f_010863bd(cpu); if cpu.returned_to != 0x105818c { return; }
  cpu.push(0x1058191); f_01015c29(cpu); if cpu.returned_to != 0x1058191 { return; }
  { let v: u16 = cpu.r16(0x10e3ce3u32).wrapping_sub(0x40u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3ce5u32), 0x20u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10581a2 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x10585a4; continue; }
  pc = 0x10581b0; continue;
 }
 0x10581b0 => {
  cpu.push(0x10581b5); f_010576b2(cpu); if cpu.returned_to != 0x10581b5 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1058420; continue; }
  pc = 0x10581bf; continue;
 }
 0x10581bf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10581db; continue; }
  pc = 0x10581c5; continue;
 }
 0x10581c5 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10581db; continue; }
  pc = 0x10581ce; continue;
 }
 0x10581ce => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x105827b; continue; }
  pc = 0x10581db; continue;
 }
 0x10581db => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x105827b; continue; }
  pc = 0x10581e5; continue;
 }
 0x10581e5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1058434; continue; }
  pc = 0x10581ef; continue;
 }
 0x10581ef => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x10582fe; continue; }
  pc = 0x10581f9; continue;
 }
 0x10581f9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x11296c8u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x11296cau32) as u32; cpu.edi = v; }
  cpu.push(0x105820e); f_01019090(cpu); if cpu.returned_to != 0x105820e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x10582fd; continue; }
  pc = 0x1058216; continue;
 }
 0x1058216 => {
  cpu.push(0x105821b); f_01057d3c(cpu); if cpu.returned_to != 0x105821b { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1058247; continue; }
  pc = 0x1058221; continue;
 }
 0x1058221 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1058247; continue; }
  pc = 0x1058232; continue;
 }
 0x1058232 => {
  cpu.push(0x1058237); f_01018baf(cpu); if cpu.returned_to != 0x1058237 { return; }
  if !cpu.cf { pc = 0x1058247; continue; }
  pc = 0x1058239; continue;
 }
 0x1058239 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x10582e5; continue; }
  pc = 0x1058245; continue;
 }
 0x1058245 => {
  pc = 0x1058261; continue;
 }
 0x1058247 => {
  cpu.push(0x105824c); f_01058f0f(cpu); if cpu.returned_to != 0x105824c { return; }
  if !cpu.zf { pc = 0x1058258; continue; }
  pc = 0x105824e; continue;
 }
 0x105824e => {
  { cpu.sub16(cpu.r16(0x10e3db7u32), 0x4u16); }
  if !cpu.zf { pc = 0x105827b; continue; }
  pc = 0x1058258; continue;
 }
 0x1058258 => {
  { cpu.sub8(cpu.r8(0x110d6bau32), 0x2bu8); }
  if !cpu.zf { pc = 0x1058261; continue; }
  pc = 0x1058261; continue;
 }
 0x1058261 => {
  { cpu.pushad(); }
  cpu.push(0x1058267); f_01057d3c(cpu); if cpu.returned_to != 0x1058267 { return; }
  cpu.push(0x105826c); f_0101acb0(cpu); if cpu.returned_to != 0x105826c { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x10582fd; continue; }
  pc = 0x1058273; continue;
 }
 0x1058273 => {
  cpu.push(0x1058278); f_010626b1(cpu); if cpu.returned_to != 0x1058278 { return; }
  if cpu.cf { pc = 0x105827b; continue; }
  pc = 0x105827a; continue;
 }
 0x105827a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105827b => {
  { cpu.sub16(cpu.r16(0x10e3db7u32), 0x0u16); }
  if cpu.zf { pc = 0x10582fd; continue; }
  pc = 0x1058285; continue;
 }
 0x1058285 => {
  { cpu.sub16(cpu.r16(0x10e3db7u32), 0xae4u16); }
  if cpu.zf { pc = 0x10582fd; continue; }
  pc = 0x1058290; continue;
 }
 0x1058290 => {
  cpu.push(0x1058295); f_01018cce(cpu); if cpu.returned_to != 0x1058295 { return; }
  if !cpu.cf { pc = 0x10582a1; continue; }
  pc = 0x1058297; continue;
 }
 0x1058297 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10582a1; continue; }
  pc = 0x105829f; continue;
 }
 0x105829f => {
  pc = 0x10582fd; continue;
 }
 0x10582a1 => {
  { cpu.sub8(cpu.r8(0x10e3d5du32), 0x2u8); }
  if !cpu.zf { pc = 0x10582cb; continue; }
  pc = 0x10582aa; continue;
 }
 0x10582aa => {
  { let v: u8 = 0x1u8; cpu.w8(0x110567cu32, v); }
  cpu.push(0x10582b6); f_01063008(cpu); if cpu.returned_to != 0x10582b6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac3u32; cpu.eax = v; }
  cpu.push(0x10582c8); f_01059dbe(cpu); if cpu.returned_to != 0x10582c8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10582fd; continue;
 }
 0x10582cb => {
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  cpu.push(0x10582d7); f_01063008(cpu); if cpu.returned_to != 0x10582d7 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x10582e2); f_01059dbe(cpu); if cpu.returned_to != 0x10582e2 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10582fd; continue;
 }
 0x10582e5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  cpu.push(0x10582f1); f_01063008(cpu); if cpu.returned_to != 0x10582f1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xacbu32; cpu.eax = v; }
  cpu.push(0x10582fc); f_01059dbe(cpu); if cpu.returned_to != 0x10582fc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10582fd; continue;
 }
 0x10582fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10582fe => {
  cpu.push(0x1058303); f_01057d3c(cpu); if cpu.returned_to != 0x1058303 { return; }
  { cpu.pushad(); }
  cpu.push(0x1058309); f_01018e6e(cpu); if cpu.returned_to != 0x1058309 { return; }
  if cpu.cf { pc = 0x1058341; continue; }
  pc = 0x105830b; continue;
 }
 0x105830b => {
  cpu.push(0x1058310); f_010191d1(cpu); if cpu.returned_to != 0x1058310 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x10f78beu32, v); }
  cpu.push(0x105831c); f_010654da(cpu); if cpu.returned_to != 0x105831c { return; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10f78beu32, v); }
  if !cpu.cf { pc = 0x105833b; continue; }
  pc = 0x105832b; continue;
 }
 0x105832b => {
  { let v: u32 = cpu.edi; cpu.w32(0x11ded58u32, v); }
  { cpu.popad(); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1058398; continue;
 }
 0x105833b => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x105827b; continue;
 }
 0x1058341 => {
  { cpu.popad(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10583bf; continue; }
  pc = 0x1058359; continue;
 }
 0x1058359 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10583bf; continue; }
  pc = 0x1058363; continue;
 }
 0x1058363 => {
  { let v: u32 = cpu.edi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1058398; continue;
 }
 0x1058398 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10ee95eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10ee960u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee967u32, v); }
  cpu.push(0x10583b1); f_010626b1(cpu); if cpu.returned_to != 0x10583b1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee967u32, v); }
  if cpu.cf { cpu.poll(); pc = 0x105827b; continue; }
  pc = 0x10583be; continue;
 }
 0x10583be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10583bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x110567cu32, v); }
  cpu.push(0x10583cc); f_01063008(cpu); if cpu.returned_to != 0x10583cc { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x10583d7); f_01059dbe(cpu); if cpu.returned_to != 0x10583d7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10583d9 => {
  cpu.push(0x10583de); f_0105867f(cpu); if cpu.returned_to != 0x10583de { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.push(0x10583e9); f_01059dbe(cpu); if cpu.returned_to != 0x10583e9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10583eb => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x10582fd; continue; }
  pc = 0x10583f9; continue;
 }
 0x10583f9 => {
  cpu.push(0x10583fe); f_010576b2(cpu); if cpu.returned_to != 0x10583fe { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { cpu.poll(); pc = 0x10582fe; continue; }
  pc = 0x1058408; continue;
 }
 0x1058408 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1058434; continue; }
  pc = 0x105840e; continue;
 }
 0x105840e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1058420; continue; }
  pc = 0x1058414; continue;
 }
 0x105840f => {
  if (cpu.sf == cpu.of) { pc = 0x105848a; continue; }
  pc = 0x1058411; continue;
 }
 0x1058411 => {
  { let v: u32 = cpu.add32(cpu.r32(0xffffff80u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), cpu.esi); cpu.w32(0xffffff80u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  if (cpu.sf == cpu.of) { pc = 0x1058490; continue; }
  pc = 0x1058417; continue;
 }
 0x1058414 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1058420; continue; }
  pc = 0x105841a; continue;
 }
 0x1058417 => {
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), cpu.r8(0xffffffe8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xe8c30000u32, v); }
  cpu.call(0x1058421); return;
 }
 0x105841a => {
  cpu.push(0x105841f); f_010626b1(cpu); if cpu.returned_to != 0x105841f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058420 => {
  cpu.push(0x1058425); f_010576b2(cpu); if cpu.returned_to != 0x1058425 { return; }
  { cpu.sub8(cpu.r8(0x10e3d5du32), 0x6u8); }
  if cpu.zf { pc = 0x1058433; continue; }
  pc = 0x105842e; continue;
 }
 0x105842e => {
  cpu.push(0x1058433); f_01062cbb(cpu); if cpu.returned_to != 0x1058433 { return; }
  pc = 0x1058433; continue;
 }
 0x1058433 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058434 => {
  cpu.push(0x1058439); f_01058cae(cpu); if cpu.returned_to != 0x1058439 { return; }
  if cpu.cf { cpu.poll(); pc = 0x1058433; continue; }
  pc = 0x105843b; continue;
 }
 0x105843b => {
  cpu.push(0x1058440); f_01057d3c(cpu); if cpu.returned_to != 0x1058440 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1058452); f_01018f3f(cpu); if cpu.returned_to != 0x1058452 { return; }
  if !cpu.cf { pc = 0x1058479; continue; }
  pc = 0x1058454; continue;
 }
 0x1058454 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x105846f; continue; }
  pc = 0x105846c; continue;
 }
 0x105846c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10584a3; continue;
 }
 0x105846f => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x4u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1058537; continue;
 }
 0x1058479 => {
  cpu.push(0x105847e); f_0101acb0(cpu); if cpu.returned_to != 0x105847e { return; }
  if !cpu.cf { pc = 0x10584a3; continue; }
  pc = 0x1058480; continue;
 }
 0x1058480 => {
  { let v: u32 = cpu.esi; cpu.w32(0x11b51aeu32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1058494); f_0101acb0(cpu); if cpu.returned_to != 0x1058494 { return; }
  { cpu.sub32(cpu.r32(0x11b51aeu32), cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x10584a3; continue; }
  pc = 0x105849d; continue;
 }
 0x105848a => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x66u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  cpu.push(0x1058494); f_0101acb0(cpu); if cpu.returned_to != 0x1058494 { return; }
  { cpu.sub32(cpu.r32(0x11b51aeu32), cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x10584a3; continue; }
  pc = 0x105849d; continue;
 }
 0x1058490 => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x28u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  cpu.call(0x1058493); return;
 }
 0x105849d => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  pc = 0x10584ea; continue;
 }
 0x10584a3 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x10584d7; continue; }
  pc = 0x10584ac; continue;
 }
 0x10584ac => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x10584c7; continue; }
  pc = 0x10584b5; continue;
 }
 0x10584b5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10584d7; continue;
 }
 0x10584c7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10584d7; continue;
 }
 0x10584d7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x105827b; continue;
 }
 0x10584ea => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x1058531; continue; }
  pc = 0x1058529; continue;
 }
 0x1058529 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1058531; continue;
 }
 0x1058531 => {
  cpu.push(0x1058536); f_010631f2(cpu); if cpu.returned_to != 0x1058536 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058537 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1058557; continue; }
  pc = 0x1058555; continue;
 }
 0x1058555 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1058557; continue;
 }
 0x1058557 => {
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x34u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10585a3); f_010631f2(cpu); if cpu.returned_to != 0x10585a3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10585a4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10585ab); f_01057d3c(cpu); if cpu.returned_to != 0x10585ab { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10585c0; continue; }
  pc = 0x10585b4; continue;
 }
 0x10585b4 => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x105862a; continue; }
  pc = 0x10585be; continue;
 }
 0x10585be => {
  pc = 0x10585c9; continue;
 }
 0x10585c0 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x105862a; continue; }
  pc = 0x10585c9; continue;
 }
 0x10585c9 => {
  cpu.push(0x10585ce); f_01018da6(cpu); if cpu.returned_to != 0x10585ce { return; }
  if !cpu.cf { pc = 0x105862a; continue; }
  pc = 0x10585d0; continue;
 }
 0x10585d0 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x10585e6; continue; }
  pc = 0x10585dc; continue;
 }
 0x10585dc => {
  { let v: u32 = 0x1f4u32; cpu.w32(0x10fcbedu32, v); }
  pc = 0x10585e6; continue;
 }
 0x10585e6 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x105862a; continue; }
  pc = 0x10585f4; continue;
 }
 0x10585f4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110eab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  cpu.push(0x105860d); f_010863bd(cpu); if cpu.returned_to != 0x105860d { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1058613); f_010586d4(cpu); if cpu.returned_to != 0x1058613 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3ce3u32).wrapping_sub(0x40u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3ce5u32), 0x20u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.push(0x1058629); f_01015c29(cpu); if cpu.returned_to != 0x1058629 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105862a => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x105867e; continue; }
  pc = 0x1058633; continue;
 }
 0x1058633 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef37cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1058668; continue; }
  pc = 0x105865b; continue;
 }
 0x105865b => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1058668; continue;
 }
 0x1058668 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e41d4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41d8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41dcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41e0u32, v); }
  pc = 0x105867e; continue;
 }
 0x105867e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105872b => {
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
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11296e2u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112528au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112528eu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x112529bu32), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x1058811; continue; }
  pc = 0x10587ae; continue;
 }
 0x10587ae => {
  { let v: u16 = cpu.r16(0x112529bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112529du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x10587d5; continue; }
  pc = 0x10587c2; continue;
 }
 0x10587c2 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10587d5; continue; }
  pc = 0x10587c9; continue;
 }
 0x10587c9 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if cpu.zf { pc = 0x10587d5; continue; }
  pc = 0x10587d0; continue;
 }
 0x10587d0 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x10587d5; continue;
 }
 0x10587d5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 6u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef37du32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e4048u32, v); }
  { let v: u8 = 0xfcu8; cpu.w8(0x10ef210u32, v); }
  cpu.push(0x1058810); f_01036a75(cpu); if cpu.returned_to != 0x1058810 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058811 => {
  { cpu.sub8(cpu.r8(0x110ed93u32), 0x0u8); }
  if cpu.zf { pc = 0x1058830; continue; }
  pc = 0x105881a; continue;
 }
 0x105881a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ed93u32)); cpu.w8(0x110ed93u32, v); }
  if !cpu.zf { pc = 0x1058830; continue; }
  pc = 0x1058822; continue;
 }
 0x1058822 => {
  { let v: u8 = 0x20u8; cpu.w8(0x112529fu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x110ed93u32, v); }
  pc = 0x1058830; continue;
 }
 0x1058830 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105867f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105867f); }
 let mut pc: u32 = 0x105867f;
 loop { match pc {
 0x105867f => {
  { let v: u8 = 0xdu8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x1058697); f_01059dbe(cpu); if cpu.returned_to != 0x1058697 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x10586a4); f_01036a75(cpu); if cpu.returned_to != 0x10586a4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10586bd); f_01035f7d(cpu); if cpu.returned_to != 0x10586bd { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10586d3; continue; }
  pc = 0x10586c5; continue;
 }
 0x10586c5 => {
  cpu.push(0x10586ca); f_01057945(cpu); if cpu.returned_to != 0x10586ca { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3debu32, v); }
  pc = 0x10586d3; continue;
 }
 0x10586d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010586d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10586d4); }
 let mut pc: u32 = 0x10586d4;
 loop { match pc {
 0x10586d4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x60u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1058705; continue; }
  pc = 0x10586ed; continue;
 }
 0x10586ed => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x190u32); }
  if cpu.zf { pc = 0x1058705; continue; }
  pc = 0x1058701; continue;
 }
 0x1058701 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1058705; continue;
 }
 0x1058705 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce5u32, v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058831(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058831); }
 let mut pc: u32 = 0x1058831;
 loop { match pc {
 0x1058831 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x105885b; continue; }
  pc = 0x105884d; continue;
 }
 0x105884d => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x105885b; continue;
 }
 0x105885b => {
  { let v: u32 = cpu.r32(0x11d50dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d50e0u32); cpu.ebx = v; }
  { let v: u32 = 0x70u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1058871); f_01018ffd(cpu); if cpu.returned_to != 0x1058871 { return; }
  if !cpu.cf { pc = 0x105887b; continue; }
  pc = 0x1058873; continue;
 }
 0x1058873 => {
  cpu.push(0x1058878); f_0101660b(cpu); if cpu.returned_to != 0x1058878 { return; }
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105887b => {
  { cpu.popad(); }
  f_01016950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058881(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058881); }
 let mut pc: u32 = 0x1058881;
 loop { match pc {
 0x1058881 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  cpu.push(0x105888d); f_01057d72(cpu); if cpu.returned_to != 0x105888d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41dcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41e0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058899(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058899); }
 let mut pc: u32 = 0x1058899;
 loop { match pc {
 0x1058899 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x10588ed; continue; }
  pc = 0x10588a2; continue;
 }
 0x10588a2 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if !cpu.zf { pc = 0x10588b8; continue; }
  pc = 0x10588ab; continue;
 }
 0x10588ab => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37cu32, v); }
  cpu.push(0x10588b7); f_010588ee(cpu); if cpu.returned_to != 0x10588b7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10588b8 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x10588bf); f_01057d72(cpu); if cpu.returned_to != 0x10588bf { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x10588ce; continue; }
  pc = 0x10588c7; continue;
 }
 0x10588c7 => {
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10588ce; continue;
 }
 0x10588ce => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x10588de; continue; }
  pc = 0x10588d6; continue;
 }
 0x10588d6 => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10588de; continue;
 }
 0x10588de => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10588ed; continue; }
  pc = 0x10588e2; continue;
 }
 0x10588e2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e41dcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41e0u32, v); }
  pc = 0x10588ed; continue;
 }
 0x10588ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010588ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10588ee); }
 let mut pc: u32 = 0x10588ee;
 loop { match pc {
 0x10588ee => {
  cpu.push(0x10588f3); f_01058b98(cpu); if cpu.returned_to != 0x10588f3 { return; }
  { cpu.sub8(cpu.r8(0x11df16fu32), 0x0u8); }
  if cpu.zf { pc = 0x1058979; continue; }
  pc = 0x10588fc; continue;
 }
 0x10588fc => {
  cpu.push(0x1058901); f_01058cf0(cpu); if cpu.returned_to != 0x1058901 { return; }
  { let v: u8 = 0xfeu8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = cpu.r16(0x11df11eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4048u32, v); }
  { let v: u16 = cpu.r16(0x11df120u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x11df122u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = cpu.r16(0x11df124u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.r32(0x11df126u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e15u32, v); }
  { let v: u32 = cpu.r32(0x11df12au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e19u32, v); }
  { let v: u32 = cpu.r32(0x11df12eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e1du32, v); }
  { let v: u32 = cpu.r32(0x11df132u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e21u32, v); }
  { let v: u32 = cpu.r32(0x11df136u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e25u32, v); }
  { let v: u32 = cpu.r32(0x11df13au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e29u32, v); }
  cpu.push(0x1058979); f_01036a75(cpu); if cpu.returned_to != 0x1058979 { return; }
  pc = 0x1058979; continue;
 }
 0x1058979 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105897a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105897a); }
 let mut pc: u32 = 0x105897a;
 loop { match pc {
 0x105897a => {
  { let v: u32 = 0x11ded5cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11df16fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1058a37; continue; }
  pc = 0x105898d; continue;
 }
 0x105898d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x11757a4u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  cpu.push(0x105899f); f_0105dae1(cpu); if cpu.returned_to != 0x105899f { return; }
  if cpu.cf { pc = 0x10589a6; continue; }
  pc = 0x10589a1; continue;
 }
 0x10589a1 => {
  cpu.push(0x10589a6); f_0105db89(cpu); if cpu.returned_to != 0x10589a6 { return; }
  pc = 0x10589a6; continue;
 }
 0x10589a6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105898d; continue; }
  pc = 0x10589af; continue;
 }
 0x10589af => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  { let v: u32 = 0x11ded5cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11df16fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10589c1; continue;
 }
 0x10589c1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296e6u32, v); }
  cpu.push(0x10589e5); f_0105d97d(cpu); if cpu.returned_to != 0x10589e5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296e6u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10589c1; continue; }
  pc = 0x10589f5; continue;
 }
 0x10589f5 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1058a37; continue; }
  pc = 0x10589fe; continue;
 }
 0x10589fe => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1058a37; continue; }
  pc = 0x1058a13; continue;
 }
 0x1058a13 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1058a25); f_01059200(cpu); if cpu.returned_to != 0x1058a25 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1058a37; continue; }
  pc = 0x1058a2e; continue;
 }
 0x1058a2e => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3de9u32, v); }
  pc = 0x1058a37; continue;
 }
 0x1058a37 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058a38); }
 let mut pc: u32 = 0x1058a38;
 loop { match pc {
 0x1058a38 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x1058aca; continue; }
  pc = 0x1058a45; continue;
 }
 0x1058a45 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x15u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e41d4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e41d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e41dcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e41e0u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1058a6e; continue; }
  pc = 0x1058a6d; continue;
 }
 0x1058a6d => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1058a6e; continue;
 }
 0x1058a6e => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1058a74; continue; }
  pc = 0x1058a72; continue;
 }
 0x1058a72 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x1058a74; continue;
 }
 0x1058a74 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x10e3f64u32)); cpu.edx = v; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x1058ac5); f_010704c2(cpu); if cpu.returned_to != 0x1058ac5 { return; }
  cpu.push(0x1058aca); f_01058acb(cpu); if cpu.returned_to != 0x1058aca { return; }
  pc = 0x1058aca; continue;
 }
 0x1058aca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058acb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058acb); }
 let mut pc: u32 = 0x1058acb;
 loop { match pc {
 0x1058acb => {
  cpu.push(0x1058ad0); f_01058b98(cpu); if cpu.returned_to != 0x1058ad0 { return; }
  { let v: u32 = 0x11ded5cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11df16fu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1058aff; continue; }
  pc = 0x1058adf; continue;
 }
 0x1058adf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1058af6); f_0104ec72(cpu); if cpu.returned_to != 0x1058af6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1058adf; continue; }
  pc = 0x1058aff; continue;
 }
 0x1058aff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058b00); }
 let mut pc: u32 = 0x1058b00;
 loop { match pc {
 0x1058b00 => {
  { cpu.sub8(cpu.r8(0x11c5abau32), 0x0u8); }
  if cpu.zf { pc = 0x1058b97; continue; }
  pc = 0x1058b0d; continue;
 }
 0x1058b0d => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x1058b2c; continue; }
  pc = 0x1058b16; continue;
 }
 0x1058b16 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x10eu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xb6u32; cpu.edx = v; }
  pc = 0x1058b62; continue;
 }
 0x1058b2c => {
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x1e0u32); }
  if cpu.zf { pc = 0x1058b4e; continue; }
  pc = 0x1058b38; continue;
 }
 0x1058b38 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x21cu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x16cu32; cpu.edx = v; }
  pc = 0x1058b62; continue;
 }
 0x1058b4e => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x21cu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x1bcu32; cpu.edx = v; }
  pc = 0x1058b62; continue;
 }
 0x1058b62 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e41d4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41dcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e41d8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e41e0u32, v); }
  cpu.push(0x1058b7e); f_010588ee(cpu); if cpu.returned_to != 0x1058b7e { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1058b90; continue; }
  pc = 0x1058b87; continue;
 }
 0x1058b87 => {
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x1058b97; continue; }
  pc = 0x1058b90; continue;
 }
 0x1058b90 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5abau32, v); }
  pc = 0x1058b97; continue;
 }
 0x1058b97 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058b98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058b98); }
 let mut pc: u32 = 0x1058b98;
 loop { match pc {
 0x1058b98 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e41d4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e41d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e41dcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e41e0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1058bcd; continue; }
  pc = 0x1058bcc; continue;
 }
 0x1058bcc => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1058bcd; continue;
 }
 0x1058bcd => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1058bd3; continue; }
  pc = 0x1058bd1; continue;
 }
 0x1058bd1 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x1058bd3; continue;
 }
 0x1058bd3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4144u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0x11ded5cu32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11df16fu32, v); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1058c00; continue;
 }
 0x1058c00 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c0a; continue;
 }
 0x1058c0a => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c14; continue;
 }
 0x1058c14 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c28; continue;
 }
 0x1058c28 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c2e; continue;
 }
 0x1058c2e => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c37; continue;
 }
 0x1058c37 => {
  { cpu.sub8(cpu.r8(0x10ef371u32), 0x0u8); }
  if !cpu.zf { pc = 0x1058c4d; continue; }
  pc = 0x1058c40; continue;
 }
 0x1058c40 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1058ca2; continue; }
  pc = 0x1058c4d; continue;
 }
 0x1058c4d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1058c77; continue; }
  pc = 0x1058c65; continue;
 }
 0x1058c65 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1058c77; continue; }
  pc = 0x1058c71; continue;
 }
 0x1058c71 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x1058c77; continue;
 }
 0x1058c77 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4138u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1058ca2; continue; }
  pc = 0x1058c7f; continue;
 }
 0x1058c7f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4140u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1058ca2; continue; }
  pc = 0x1058c87; continue;
 }
 0x1058c87 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e413cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1058ca2; continue; }
  pc = 0x1058c8f; continue;
 }
 0x1058c8f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4144u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1058ca2; continue; }
  pc = 0x1058c97; continue;
 }
 0x1058c97 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11df16fu32)); cpu.w8(0x11df16fu32, v); }
  pc = 0x1058ca2; continue;
 }
 0x1058ca2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1058c00; continue; }
  pc = 0x1058cac; continue;
 }
 0x1058cac => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058cae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058cae); }
 let mut pc: u32 = 0x1058cae;
 loop { match pc {
 0x1058cae => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x1058cb9; continue; }
  pc = 0x1058cb7; continue;
 }
 0x1058cb7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058cb9 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x1058cc8; continue; }
  pc = 0x1058cc2; continue;
 }
 0x1058cc2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1058cca; continue; }
  pc = 0x1058cc8; continue;
 }
 0x1058cc8 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058cca => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x32u8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058cd6; continue;
 }
 0x1058cd6 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058cda; continue;
 }
 0x1058cda => {
  { cpu.sub8((cpu.eax as u8), 0x30u8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058cde; continue;
 }
 0x1058cde => {
  { cpu.sub8((cpu.eax as u8), 0x3au8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058ce2; continue;
 }
 0x1058ce2 => {
  { cpu.sub8((cpu.eax as u8), 0x38u8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058ce6; continue;
 }
 0x1058ce6 => {
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  if cpu.zf { pc = 0x1058ced; continue; }
  pc = 0x1058cea; continue;
 }
 0x1058cea => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058ced => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058cf0); }
 let mut pc: u32 = 0x1058cf0;
 loop { match pc {
 0x1058cf0 => {
  { let v: u32 = cpu.r8(0x11df16fu32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x11ded5cu32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11df11eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df122u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df126u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df12au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df12eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df132u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df136u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df13au32, v); }
  pc = 0x1058d4c; continue;
 }
 0x1058d4c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x11df146u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df14au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df14eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df152u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df156u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df15au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df15eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11df162u32, v); }
  pc = 0x1058daa; continue;
 }
 0x1058daa => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1058de2; continue; }
  pc = 0x1058dae; continue;
 }
 0x1058dae => {
  { let v: u32 = cpu.shl32(cpu.r32(0x11df146u32), 1u32); cpu.w32(0x11df146u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df14au32), 1u32); cpu.w32(0x11df14au32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df14eu32), 1u32); cpu.w32(0x11df14eu32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df152u32), 1u32); cpu.w32(0x11df152u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df156u32), 1u32); cpu.w32(0x11df156u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df15au32), 1u32); cpu.w32(0x11df15au32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df15eu32), 1u32); cpu.w32(0x11df15eu32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x11df162u32), 1u32); cpu.w32(0x11df162u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1058daa; continue; }
  pc = 0x1058de2; continue;
 }
 0x1058de2 => {
  { let v: u32 = cpu.r32(0x11df146u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df11eu32) | cpu.eax; cpu.w32(0x11df11eu32, v); }
  { let v: u32 = cpu.r32(0x11df14au32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df122u32) | cpu.eax; cpu.w32(0x11df122u32, v); }
  { let v: u32 = cpu.r32(0x11df14eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df126u32) | cpu.eax; cpu.w32(0x11df126u32, v); }
  { let v: u32 = cpu.r32(0x11df152u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df12au32) | cpu.eax; cpu.w32(0x11df12au32, v); }
  { let v: u32 = cpu.r32(0x11df156u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df12eu32) | cpu.eax; cpu.w32(0x11df12eu32, v); }
  { let v: u32 = cpu.r32(0x11df15au32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df132u32) | cpu.eax; cpu.w32(0x11df132u32, v); }
  { let v: u32 = cpu.r32(0x11df15eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df136u32) | cpu.eax; cpu.w32(0x11df136u32, v); }
  { let v: u32 = cpu.r32(0x11df162u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11df13au32) | cpu.eax; cpu.w32(0x11df13au32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1058d4c; continue; }
  pc = 0x1058e45; continue;
 }
 0x1058e45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058e46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058e46); }
 let mut pc: u32 = 0x1058e46;
 loop { match pc {
 0x1058e46 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x11ded5cu32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11df16fu32, v); }
  { let v: u32 = 0x11757a4u32; cpu.ebp = v; }
  pc = 0x1058e5c; continue;
 }
 0x1058e5c => {
  { let v: u32 = cpu.shr32(cpu.r32(0x11df13au32), 1u32); cpu.w32(0x11df13au32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df136u32), 1u32); cpu.w32(0x11df136u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df132u32), 1u32); cpu.w32(0x11df132u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df12eu32), 1u32); cpu.w32(0x11df12eu32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df12au32), 1u32); cpu.w32(0x11df12au32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df126u32), 1u32); cpu.w32(0x11df126u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df122u32), 1u32); cpu.w32(0x11df122u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x11df11eu32), 1u32); cpu.w32(0x11df11eu32, v); }
  if !cpu.cf { pc = 0x1058ea5; continue; }
  pc = 0x1058e8e; continue;
 }
 0x1058e8e => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1058ea5; continue; }
  pc = 0x1058e94; continue;
 }
 0x1058e94 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x1058ea5; continue; }
  pc = 0x1058e9a; continue;
 }
 0x1058e9a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11df16fu32)); cpu.w8(0x11df16fu32, v); }
  pc = 0x1058ea5; continue;
 }
 0x1058ea5 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x48u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1058e5c; continue; }
  pc = 0x1058eab; continue;
 }
 0x1058eab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058eac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058eac); }
 let mut pc: u32 = 0x1058eac;
 loop { match pc {
 0x1058eac => {
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11c59fcu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1058f0d; continue; }
  pc = 0x1058ebc; continue;
 }
 0x1058ebc => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1058f0d; continue; }
  pc = 0x1058ec6; continue;
 }
 0x1058ec6 => {
  cpu.push(0x1058ecb); f_010576b2(cpu); if cpu.returned_to != 0x1058ecb { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1058f0d; continue; }
  pc = 0x1058ed1; continue;
 }
 0x1058ed1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1058f0d; continue; }
  pc = 0x1058edc; continue;
 }
 0x1058edc => {
  { let v: u32 = 0x0u32; cpu.w32(0x11c59fcu32, v); }
  { let v: u16 = cpu.r16(0x11c5a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11c5a02u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1058f06); f_01066c93(cpu); if cpu.returned_to != 0x1058f06 { return; }
  cpu.push(0x1058f0b); f_01057945(cpu); if cpu.returned_to != 0x1058f0b { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1058f0d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058f0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058f0f); }
 let mut pc: u32 = 0x1058f0f;
 loop { match pc {
 0x1058f0f => {
  cpu.push(0x1058f14); f_01057d3c(cpu); if cpu.returned_to != 0x1058f14 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1058f33); f_01009e97(cpu); if cpu.returned_to != 0x1058f33 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058f36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058f36); }
 let mut pc: u32 = 0x1058f36;
 loop { match pc {
 0x1058f36 => {
  { cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058f92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058f92); }
 let mut pc: u32 = 0x1058f92;
 loop { match pc {
 0x1058f92 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u32 = cpu.r8(0x10ef357u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  cpu.push(0x1058fd4); f_01036a75(cpu); if cpu.returned_to != 0x1058fd4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058feb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1058feb); }
 let mut pc: u32 = 0x1058feb;
 loop { match pc {
 0x1058feb => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e415cu32, v); }
  cpu.push(0x1058ffd); f_0105901d(cpu); if cpu.returned_to != 0x1058ffd { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01058fff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1058fff); } r_01058fff(cpu, 0x1058fff); }
pub fn f_0105901d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105901d); } r_01058fff(cpu, 0x105901d); }
fn r_01058fff(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1058fff => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x105900a); f_010576b2(cpu); if cpu.returned_to != 0x105900a { return; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e415cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1059045; continue;
 }
 0x105901d => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059022; continue;
 }
 0x1059022 => {
  { cpu.sub8((cpu.ecx as u8), 0x19u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059027; continue;
 }
 0x1059027 => {
  { cpu.sub8((cpu.ecx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x105902c; continue;
 }
 0x105902c => {
  { cpu.sub8((cpu.ecx as u8), 0x18u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059031; continue;
 }
 0x1059031 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059036; continue;
 }
 0x1059036 => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x105903b; continue;
 }
 0x105903b => {
  { cpu.sub8((cpu.ecx as u8), 0x24u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059040; continue;
 }
 0x1059040 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x105908d; continue; }
  pc = 0x1059045; continue;
 }
 0x1059045 => {
  { let v: u32 = 0xfffffff9u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff9u32; cpu.edx = v; }
  cpu.push(0x1059054); f_0105911b(cpu); if cpu.returned_to != 0x1059054 { return; }
  if cpu.zf { pc = 0x105908b; continue; }
  pc = 0x1059056; continue;
 }
 0x1059056 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff9u32; cpu.edx = v; }
  cpu.push(0x1059065); f_0105911b(cpu); if cpu.returned_to != 0x1059065 { return; }
  if cpu.zf { pc = 0x105908b; continue; }
  pc = 0x1059067; continue;
 }
 0x1059067 => {
  { let v: u32 = 0xfffffff9u32; cpu.ecx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x1059076); f_0105911b(cpu); if cpu.returned_to != 0x1059076 { return; }
  if cpu.zf { pc = 0x105908b; continue; }
  pc = 0x1059078; continue;
 }
 0x1059078 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x1059087); f_0105911b(cpu); if cpu.returned_to != 0x1059087 { return; }
  if cpu.zf { pc = 0x105908b; continue; }
  pc = 0x1059089; continue;
 }
 0x1059089 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105908b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105908d => {
  { let v: u32 = 0xfffffff1u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff1u32; cpu.edx = v; }
  cpu.push(0x105909c); f_0105911b(cpu); if cpu.returned_to != 0x105909c { return; }
  if cpu.zf { cpu.poll(); pc = 0x105908b; continue; }
  pc = 0x105909e; continue;
 }
 0x105909e => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff1u32; cpu.edx = v; }
  cpu.push(0x10590ad); f_0105911b(cpu); if cpu.returned_to != 0x10590ad { return; }
  if cpu.zf { cpu.poll(); pc = 0x105908b; continue; }
  pc = 0x10590af; continue;
 }
 0x10590af => {
  { let v: u32 = 0xfffffff1u32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x10590be); f_0105911b(cpu); if cpu.returned_to != 0x10590be { return; }
  if cpu.zf { cpu.poll(); pc = 0x105908b; continue; }
  pc = 0x10590c0; continue;
 }
 0x10590c0 => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x10590cf); f_0105911b(cpu); if cpu.returned_to != 0x10590cf { return; }
  if cpu.zf { cpu.poll(); pc = 0x105908b; continue; }
  pc = 0x10590d1; continue;
 }
 0x10590d1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010590d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10590d3); }
 let mut pc: u32 = 0x10590d3;
 loop { match pc {
 0x10590d3 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10590e2); f_0105911b(cpu); if cpu.returned_to != 0x10590e2 { return; }
  if cpu.zf { pc = 0x1059119; continue; }
  pc = 0x10590e4; continue;
 }
 0x10590e4 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10590f3); f_0105911b(cpu); if cpu.returned_to != 0x10590f3 { return; }
  if cpu.zf { pc = 0x1059119; continue; }
  pc = 0x10590f5; continue;
 }
 0x10590f5 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  cpu.push(0x1059104); f_0105911b(cpu); if cpu.returned_to != 0x1059104 { return; }
  if cpu.zf { pc = 0x1059119; continue; }
  pc = 0x1059106; continue;
 }
 0x1059106 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  cpu.push(0x1059115); f_0105911b(cpu); if cpu.returned_to != 0x1059115 { return; }
  if cpu.zf { pc = 0x1059119; continue; }
  pc = 0x1059117; continue;
 }
 0x1059117 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059119 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105911b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105911b); }
 let mut pc: u32 = 0x105911b;
 loop { match pc {
 0x105911b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1059129); f_0104f36d(cpu); if cpu.returned_to != 0x1059129 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e415cu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059141(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059141); }
 let mut pc: u32 = 0x1059141;
 loop { match pc {
 0x1059141 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1059149); f_0104f36d(cpu); if cpu.returned_to != 0x1059149 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x14u8); }
  if cpu.zf { pc = 0x1059172; continue; }
  pc = 0x1059157; continue;
 }
 0x1059157 => {
  { cpu.sub8((cpu.ebx as u8), 0x15u8); }
  if cpu.zf { pc = 0x1059192; continue; }
  pc = 0x105915c; continue;
 }
 0x105915c => {
  { cpu.sub8((cpu.ebx as u8), 0x3cu8); }
  if cpu.zf { pc = 0x10591c0; continue; }
  pc = 0x1059161; continue;
 }
 0x1059161 => {
  { cpu.sub8((cpu.ebx as u8), 0x3du8); }
  if cpu.zf { pc = 0x10591e0; continue; }
  pc = 0x1059166; continue;
 }
 0x1059166 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11296e0u32, v); }
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059172 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11296e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059192 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11296e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fcc47u32), 0x0u8); }
  if !cpu.zf { pc = 0x10591be; continue; }
  pc = 0x10591b9; continue;
 }
 0x10591b9 => {
  cpu.push(0x10591be); f_01035481(cpu); if cpu.returned_to != 0x10591be { return; }
  pc = 0x10591be; continue;
 }
 0x10591be => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10591c0 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11296e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10591e0 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11296e0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059200); }
 let mut pc: u32 = 0x1059200;
 loop { match pc {
 0x1059200 => {
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1059362; continue; }
  pc = 0x105920a; continue;
 }
 0x105920a => {
  { cpu.sub8(cpu.r8(0x10fcdb3u32), 0x0u8); }
  if !cpu.zf { pc = 0x1059375; continue; }
  pc = 0x1059217; continue;
 }
 0x1059217 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d77u32, v); }
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x105923f; continue; }
  pc = 0x105922a; continue;
 }
 0x105922a => {
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d77u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x105923e); f_01057945(cpu); if cpu.returned_to != 0x105923e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105923f; continue;
 }
 0x105923f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce1u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3debu32, v); }
  cpu.push(0x1059250); f_010576b2(cpu); if cpu.returned_to != 0x1059250 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1059362; continue; }
  pc = 0x105925a; continue;
 }
 0x105925a => {
  { cpu.sub8(cpu.r8(0x10ef371u32), 0x0u8); }
  if !cpu.zf { pc = 0x1059274; continue; }
  pc = 0x1059263; continue;
 }
 0x1059263 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1059362; continue; }
  pc = 0x1059274; continue;
 }
 0x1059274 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110d648u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1059362; continue; }
  pc = 0x1059284; continue;
 }
 0x1059284 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41a8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef350u32, v); }
  cpu.push(0x10592a9); f_0105950a(cpu); if cpu.returned_to != 0x10592a9 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x110d6e0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee92bu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3de9u32, v); }
  { let v: u32 = 0x11a1890u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4118u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11a1888u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11a188au32, v); }
  cpu.push(0x10592e4); f_01057a1d(cpu); if cpu.returned_to != 0x10592e4 { return; }
  cpu.push(0x10592e9); f_010576b2(cpu); if cpu.returned_to != 0x10592e9 { return; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1059361; continue; }
  pc = 0x10592f0; continue;
 }
 0x10592f0 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) | 0x2u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1059362; continue; }
  pc = 0x105930f; continue;
 }
 0x105930f => {
  { cpu.sub8(cpu.r8(0x10fd4c0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1059361; continue; }
  pc = 0x1059318; continue;
 }
 0x1059318 => {
  cpu.push(0x105931d); f_01035e73(cpu); if cpu.returned_to != 0x105931d { return; }
  cpu.push(0x1059322); f_010576b2(cpu); if cpu.returned_to != 0x1059322 { return; }
  { let v: u8 = 0xf8u8; cpu.w8(0x10ef210u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4048u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x105934e); f_01036a75(cpu); if cpu.returned_to != 0x105934e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  pc = 0x105935c; continue;
 }
 0x105935c => {
  cpu.push(0x1059361); f_0105d947(cpu); if cpu.returned_to != 0x1059361 { return; }
  pc = 0x1059361; continue;
 }
 0x1059361 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059362 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ce1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3debu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059375 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x1059362; continue; }
  pc = 0x1059391; continue;
 }
 0x1059391 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1059362; continue; }
  pc = 0x1059397; continue;
 }
 0x1059397 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { cpu.poll(); pc = 0x1059362; continue; }
  pc = 0x10593a4; continue;
 }
 0x10593a4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110d648u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41a8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3de9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef350u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x80u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010593f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10593f1); }
 let mut pc: u32 = 0x10593f1;
 loop { match pc {
 0x10593f1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1129891u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x1059405; continue;
 }
 0x1059405 => {
  { let v: u32 = 0x2u32; cpu.w32(0x1129891u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x1059419; continue;
 }
 0x1059419 => {
  { let v: u32 = 0x3u32; cpu.w32(0x1129891u32, v); }
  { let v: u32 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1129837u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x1059434; continue;
 }
 0x1059434 => {
  { let v: u32 = 0x4u32; cpu.w32(0x1129891u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x105945a; continue;
 }
 0x105945a => {
  { let v: u32 = 0x5u32; cpu.w32(0x1129891u32, v); }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x105946e; continue;
 }
 0x105946e => {
  { let v: u32 = 0x6u32; cpu.w32(0x1129891u32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x105947e; continue;
 }
 0x105947e => {
  { let v: u32 = 0x7u32; cpu.w32(0x1129891u32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x3du8); }
  if cpu.zf { pc = 0x10594f1; continue; }
  pc = 0x105948e; continue;
 }
 0x105948e => {
  { let v: u32 = 0x8u32; cpu.w32(0x1129891u32, v); }
  cpu.push(0x105949d); f_010528b1(cpu); if cpu.returned_to != 0x105949d { return; }
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e3f84u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10594b5; continue;
 }
 0x10594b5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10594e5; continue; }
  pc = 0x10594ba; continue;
 }
 0x10594ba => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1129837u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10594e5; continue; }
  pc = 0x10594d4; continue;
 }
 0x10594d4 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1129891u32), 0x100u32); cpu.w32(0x1129891u32, v); }
  { cpu.pushad(); }
  cpu.push(0x10594e4); f_010675fd(cpu); if cpu.returned_to != 0x10594e4 { return; }
  { cpu.popad(); }
  pc = 0x10594e5; continue;
 }
 0x10594e5 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10594b5; continue; }
  pc = 0x10594ea; continue;
 }
 0x10594ea => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  { cpu.popad(); }
  pc = 0x10594f1; continue;
 }
 0x10594f1 => {
  { let v: u32 = cpu.r32(0x1129891u32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010594f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10594f8); }
 let mut pc: u32 = 0x10594f8;
 loop { match pc {
 0x10594f8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x11757b7u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105950a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105950a); }
 let mut pc: u32 = 0x105950a;
 loop { match pc {
 0x105950a => {
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91fu32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1059534; continue; }
  pc = 0x105952c; continue;
 }
 0x105952c => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91fu32, v); }
  pc = 0x1059533; continue;
 }
 0x1059533 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059534 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1059533; continue; }
  pc = 0x105953a; continue;
 }
 0x105953a => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ee91fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105956c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105956c); }
 let mut pc: u32 = 0x105956c;
 loop { match pc {
 0x105956c => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if cpu.zf { pc = 0x1059576; continue; }
  pc = 0x1059575; continue;
 }
 0x1059575 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059576 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x1059580; continue; }
  pc = 0x105957f; continue;
 }
 0x105957f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059580 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1059597; continue; }
  pc = 0x1059596; continue;
 }
 0x1059596 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059597 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10595b0; continue; }
  pc = 0x10595a0; continue;
 }
 0x10595a0 => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x1059871; continue; }
  pc = 0x10595ae; continue;
 }
 0x10595ae => {
  pc = 0x10595bd; continue;
 }
 0x10595b0 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1059871; continue; }
  pc = 0x10595bd; continue;
 }
 0x10595bd => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x11a10b8u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x105982d; continue; }
  pc = 0x10595cf; continue;
 }
 0x10595cf => {
  { cpu.and8(cpu.r8(0x10ee968u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x105982d; continue; }
  pc = 0x10595db; continue;
 }
 0x10595db => {
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1059740; continue; }
  pc = 0x10595e8; continue;
 }
 0x10595e8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cebu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cedu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cddu32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x1130f1du32), 0x0u8); }
  if cpu.zf { pc = 0x1059653; continue; }
  pc = 0x105963e; continue;
 }
 0x105963e => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x1130f1du32), 0x1u8); }
  if cpu.zf { pc = 0x1059653; continue; }
  pc = 0x105964d; continue;
 }
 0x105964d => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1059653; continue;
 }
 0x1059653 => {
  { let v = cpu.r32(0x10e3f60u32); cpu.push(v); }
  { let v = cpu.r32(0x10e3f64u32); cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d03u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d05u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1059672); f_01057641(cpu); if cpu.returned_to != 0x1059672 { return; }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d03u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d05u32, v); }
  { let v: u32 = 0x4u32; cpu.esi = v; }
  { let v: u32 = 0xfffffffcu32; cpu.edi = v; }
  { let v: u32 = 0xfffcu32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if cpu.zf { pc = 0x10596b4; continue; }
  pc = 0x10596a5; continue;
 }
 0x10596a5 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffu32; cpu.ebp = v; }
  pc = 0x10596b4; continue;
 }
 0x10596b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10596e0; continue; }
  pc = 0x10596bb; continue;
 }
 0x10596bb => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10596cb; continue; }
  pc = 0x10596c1; continue;
 }
 0x10596c1 => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10596d3; continue; }
  pc = 0x10596c6; continue;
 }
 0x10596c6 => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10596d3; continue;
 }
 0x10596cb => {
  { cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10596d3; continue; }
  pc = 0x10596d0; continue;
 }
 0x10596d0 => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10596d3; continue;
 }
 0x10596d3 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3f60u32, v); }
  pc = 0x10596e0; continue;
 }
 0x10596e0 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105970a; continue; }
  pc = 0x10596e5; continue;
 }
 0x10596e5 => {
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10596f5; continue; }
  pc = 0x10596eb; continue;
 }
 0x10596eb => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10596fd; continue; }
  pc = 0x10596f0; continue;
 }
 0x10596f0 => {
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10596fd; continue;
 }
 0x10596f5 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.esi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10596fd; continue; }
  pc = 0x10596fa; continue;
 }
 0x10596fa => {
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10596fd; continue;
 }
 0x10596fd => {
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3f64u32, v); }
  pc = 0x105970a; continue;
 }
 0x105970a => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x2u8); }
  if !cpu.zf { pc = 0x105972c; continue; }
  pc = 0x1059726; continue;
 }
 0x1059726 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105972c; continue;
 }
 0x105972c => {
  cpu.push(0x1059731); f_0105f339(cpu); if cpu.returned_to != 0x1059731 { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  cpu.push(0x105973f); f_0103d12a(cpu); if cpu.returned_to != 0x105973f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059740 => {
  cpu.push(0x1059745); f_01057d72(cpu); if cpu.returned_to != 0x1059745 { return; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105978e; continue; }
  pc = 0x105974e; continue;
 }
 0x105974e => {
  { cpu.sub8(cpu.r8(0x10fd2a5u32), 0x0u8); }
  if cpu.zf { pc = 0x1059771; continue; }
  pc = 0x1059757; continue;
 }
 0x1059757 => {
  { cpu.sub16((cpu.eax as u16), 0x280u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x1059761; continue;
 }
 0x1059761 => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x105976f; continue;
 }
 0x105976f => {
  pc = 0x10597bd; continue;
 }
 0x1059771 => {
  { cpu.sub16((cpu.eax as u16), 0x220u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x105977b; continue;
 }
 0x105977b => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x105978c; continue;
 }
 0x105978c => {
  pc = 0x10597bd; continue;
 }
 0x105978e => {
  { cpu.sub8(cpu.r8(0x10fd2a5u32), 0x0u8); }
  if cpu.zf { pc = 0x10597ae; continue; }
  pc = 0x1059797; continue;
 }
 0x1059797 => {
  { cpu.sub16((cpu.eax as u16), 0x140u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x10597a1; continue;
 }
 0x10597a1 => {
  { cpu.sub16((cpu.ebx as u16), 0xc8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x10597ac; continue;
 }
 0x10597ac => {
  pc = 0x10597bd; continue;
 }
 0x10597ae => {
  { cpu.sub16((cpu.eax as u16), 0x110u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x10597b4; continue;
 }
 0x10597b4 => {
  { cpu.sub16((cpu.ebx as u16), 0xb8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105982d; continue; }
  pc = 0x10597bb; continue;
 }
 0x10597bb => {
  pc = 0x10597bd; continue;
 }
 0x10597bd => {
  { cpu.sub8(cpu.r8(0x10ee92cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1059872; continue; }
  pc = 0x10597ca; continue;
 }
 0x10597ca => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee97bu32, v); }
  cpu.push(0x10597d6); f_01057a4c(cpu); if cpu.returned_to != 0x10597d6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef331u32, v); }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d51u32, v); }
  { let v: u16 = 0xa0u16; cpu.w16(0x10e3cebu32, v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d03u32, v); }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d53u32, v); }
  { let v: u16 = 0x64u16; cpu.w16(0x10e3cedu32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d05u32, v); }
  { let v: u16 = 0xa0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105982c); f_0105edc0(cpu); if cpu.returned_to != 0x105982c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105982d => {
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1059858; continue; }
  pc = 0x1059836; continue;
 }
 0x1059836 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1059848); f_0105edc0(cpu); if cpu.returned_to != 0x1059848 { return; }
  { let v: u16 = 0xfc18u16; cpu.w16(0x10e3d51u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97bu32, v); }
  pc = 0x1059858; continue;
 }
 0x1059858 => {
  { cpu.sub8(cpu.r8(0x10ee92cu32), 0x0u8); }
  if cpu.zf { pc = 0x1059871; continue; }
  pc = 0x1059861; continue;
 }
 0x1059861 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee92cu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ce3u32, v); }
  pc = 0x1059871; continue;
 }
 0x1059871 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1059872 => {
  { let v: u16 = cpu.r16(0x10e3d0bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.r16(0x10e3d0du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105988b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105988b); }
 let mut pc: u32 = 0x105988b;
 loop { match pc {
 0x105988b => {
  cpu.push(0x1059890); f_010576b2(cpu); if cpu.returned_to != 0x1059890 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5fu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x10598d6; continue; }
  pc = 0x1059899; continue;
 }
 0x1059899 => {
  let t = cpu.r32(0x1128d04u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x10598a0); cpu.call(t); if cpu.returned_to != 0x10598a0 { return; }
  if cpu.cf { pc = 0x10598d6; continue; }
  pc = 0x10598a2; continue;
 }
 0x10598a2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d0bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d0du32, v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee92cu32, v); }
  pc = 0x10598d6; continue;
 }
 0x10598d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010598d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10598d7); }
 let mut pc: u32 = 0x10598d7;
 loop { match pc {
 0x10598d7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010598d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10598d8); }
 let mut pc: u32 = 0x10598d8;
 loop { match pc {
 0x10598d8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x1059903; continue; }
  pc = 0x10598dd; continue;
 }
 0x10598dd => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1059903; continue; }
  pc = 0x10598e5; continue;
 }
 0x10598e5 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(2, false); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10598fc); f_01059985(cpu); if cpu.returned_to != 0x10598fc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10598d8; continue;
 }
 0x1059903 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105996f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105996f); }
 let mut pc: u32 = 0x105996f;
 loop { match pc {
 0x105996f => {
  { let v: u32 = 0x10e45fcu32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee982u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059985); }
 let mut pc: u32 = 0x1059985;
 loop { match pc {
 0x1059985 => {
  { let v: u32 = 0x10e45fcu32; cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  pc = 0x1059992; continue;
 }
 0x1059992 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x10599b1; continue; }
  pc = 0x1059998; continue;
 }
 0x1059998 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee982u32)); cpu.w8(0x10ee982u32, v); }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10599b1 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059992; continue; }
  pc = 0x10599b6; continue;
 }
 0x10599b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010599b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10599b8); }
 let mut pc: u32 = 0x10599b8;
 loop { match pc {
 0x10599b8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef209u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x10599cf; continue; }
  pc = 0x10599c8; continue;
 }
 0x10599c8 => {
  { let v: u8 = 0x78u8; cpu.w8(0x10ef209u32, v); }
  pc = 0x10599cf; continue;
 }
 0x10599cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010599d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10599d1); }
 let mut pc: u32 = 0x10599d1;
 loop { match pc {
 0x10599d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010599d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10599d2); }
 let mut pc: u32 = 0x10599d2;
 loop { match pc {
 0x10599d2 => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ef306u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4030u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef305u32, v); }
  { let v: u16 = 0x1au16; cpu.w16(0x10e3e37u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3e3bu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3e41u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059a06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059a06); }
 let mut pc: u32 = 0x1059a06;
 loop { match pc {
 0x1059a06 => {
  { cpu.sub16(cpu.r16(0x10e3cfbu32), 0xff0u16); }
  if !cpu.cf { pc = 0x1059a1f; continue; }
  pc = 0x1059a11; continue;
 }
 0x1059a11 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3cfbu32)); cpu.w16(0x10e3cfbu32, v); }
  if !cpu.sf { pc = 0x1059a1f; continue; }
  pc = 0x1059a1a; continue;
 }
 0x1059a1a => {
  cpu.push(0x1059a1f); f_010599d1(cpu); if cpu.returned_to != 0x1059a1f { return; }
  pc = 0x1059a1f; continue;
 }
 0x1059a1f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059a20); }
 let mut pc: u32 = 0x1059a20;
 loop { match pc {
 0x1059a20 => {
  { let v: u32 = 0x118ccc4u32; cpu.edi = v; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059a31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059a31); }
 let mut pc: u32 = 0x1059a31;
 loop { match pc {
 0x1059a31 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1059a60; continue; }
  pc = 0x1059a3b; continue;
 }
 0x1059a3b => {
  cpu.push(0x1059a40); f_01057945(cpu); if cpu.returned_to != 0x1059a40 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.push(0x1059a4c); f_010599d1(cpu); if cpu.returned_to != 0x1059a4c { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.push(0x1059a60); f_01033e70(cpu); if cpu.returned_to != 0x1059a60 { return; }
  pc = 0x1059a60; continue;
 }
 0x1059a60 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059a61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059a61); }
 let mut pc: u32 = 0x1059a61;
 loop { match pc {
 0x1059a61 => {
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffau32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1059a79); f_01059985(cpu); if cpu.returned_to != 0x1059a79 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x302u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1059a89); f_01059985(cpu); if cpu.returned_to != 0x1059a89 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ab6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ab6); }
 let mut pc: u32 = 0x1059ab6;
 loop { match pc {
 0x1059ab6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ab7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ab7); }
 let mut pc: u32 = 0x1059ab7;
 loop { match pc {
 0x1059ab7 => {
  { let v: u32 = 0x10c05fcu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1059acf; continue; }
  pc = 0x1059ac5; continue;
 }
 0x1059ac5 => {
  cpu.push(0x1059aca); f_01072005(cpu); if cpu.returned_to != 0x1059aca { return; }
  { let v: u32 = 0x10c07ccu32; cpu.esi = v; }
  pc = 0x1059acf; continue;
 }
 0x1059acf => {
  cpu.push(0x1059ad4); f_01059ad5(cpu); if cpu.returned_to != 0x1059ad4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ad5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ad5); }
 let mut pc: u32 = 0x1059ad5;
 loop { match pc {
 0x1059ad5 => {
  { let v: u32 = 0x11a10b8u32; cpu.edi = v; }
  pc = 0x1059ada; continue;
 }
 0x1059ada => {
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x1059ada; continue; }
  pc = 0x1059ae1; continue;
 }
 0x1059ae1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ae2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ae2); }
 let mut pc: u32 = 0x1059ae2;
 loop { match pc {
 0x1059ae2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ae3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ae3); }
 let mut pc: u32 = 0x1059ae3;
 loop { match pc {
 0x1059ae3 => {
  { cpu.sub8(cpu.r8(0x110566du32), 0x0u8); }
  if !cpu.zf { pc = 0x1059af1; continue; }
  pc = 0x1059aec; continue;
 }
 0x1059aec => {
  cpu.push(0x1059af1); f_01038d64(cpu); if cpu.returned_to != 0x1059af1 { return; }
  pc = 0x1059af1; continue;
 }
 0x1059af1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11296e3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110566du32, v); }
  { let v: u32 = 0x11a10b8u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1059b15; continue;
 }
 0x1059b15 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1059bb0; continue; }
  pc = 0x1059b21; continue;
 }
 0x1059b21 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x1059b2e; continue; }
  pc = 0x1059b29; continue;
 }
 0x1059b29 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b2e => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ebp as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x1059b3a; continue; }
  pc = 0x1059b35; continue;
 }
 0x1059b35 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b3a => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1059b46; continue; }
  pc = 0x1059b41; continue;
 }
 0x1059b41 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b46 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ebp as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1059b4f; continue; }
  pc = 0x1059b4d; continue;
 }
 0x1059b4d => {
  pc = 0x1059ba3; continue;
 }
 0x1059b4f => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if cpu.zf { pc = 0x1059b71; continue; }
  pc = 0x1059b5a; continue;
 }
 0x1059b5a => {
  { cpu.sub8(cpu.r8(0x10ee97du32), 0x0u8); }
  if !cpu.zf { pc = 0x1059b71; continue; }
  pc = 0x1059b63; continue;
 }
 0x1059b63 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee97du32, v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(0x1059b6e); cpu.call(t); if cpu.returned_to != 0x1059b6e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b71 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x2u8); }
  if cpu.zf { pc = 0x1059b9d; continue; }
  pc = 0x1059b7a; continue;
 }
 0x1059b7a => {
  { cpu.sub8(cpu.r8(0x10ee97eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1059b9d; continue; }
  pc = 0x1059b83; continue;
 }
 0x1059b83 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee97eu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0xbu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1059b97; continue; }
  pc = 0x1059b91; continue;
 }
 0x1059b91 => {
  let t = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.push(0x1059b94); cpu.call(t); if cpu.returned_to != 0x1059b94 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b97 => {
  let t = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(0x1059b9a); cpu.call(t); if cpu.returned_to != 0x1059b9a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059b9d => {
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(0x1059ba0); cpu.call(t); if cpu.returned_to != 0x1059ba0 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1059ba3; continue;
 }
 0x1059ba3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1059b15; continue;
 }
 0x1059bb0 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x1059bbb; continue; }
  pc = 0x1059bb4; continue;
 }
 0x1059bb4 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ee979u32, v); }
  pc = 0x1059bbb; continue;
 }
 0x1059bbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059bbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059bbc); }
 let mut pc: u32 = 0x1059bbc;
 loop { match pc {
 0x1059bbc => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ee979u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059bc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059bc4); }
 let mut pc: u32 = 0x1059bc4;
 loop { match pc {
 0x1059bc4 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1059bdf; continue; }
  pc = 0x1059bcd; continue;
 }
 0x1059bcd => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x1059bdf; continue; }
  pc = 0x1059bd6; continue;
 }
 0x1059bd6 => {
  { cpu.sub8(cpu.r8(0x10c6876u32), 0x0u8); }
  if cpu.zf { pc = 0x1059be6; continue; }
  pc = 0x1059bdf; continue;
 }
 0x1059bdf => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  pc = 0x1059be6; continue;
 }
 0x1059be6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059be7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059be7); }
 let mut pc: u32 = 0x1059be7;
 loop { match pc {
 0x1059be7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110d73du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059bf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059bf4); }
 let mut pc: u32 = 0x1059bf4;
 loop { match pc {
 0x1059bf4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059bf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059bf5); }
 let mut pc: u32 = 0x1059bf5;
 loop { match pc {
 0x1059bf5 => {
  { let v = cpu.r32(0x10e4104u32); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10ee96eu32), 0x0u8); }
  if cpu.zf { pc = 0x1059c04; continue; }
  pc = 0x1059c04; continue;
 }
 0x1059c04 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4104u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059c0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059c0b); }
 let mut pc: u32 = 0x1059c0b;
 loop { match pc {
 0x1059c0b => {
  { let v: u32 = 0x10fd45du32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x8cu32; cpu.ebx = v; }
  pc = 0x1059c1a; continue;
 }
 0x1059c1a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1059c25; continue;
 }
 0x1059c25 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x1059c3b); f_01059c7e(cpu); if cpu.returned_to != 0x1059c3b { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059c25; continue; }
  pc = 0x1059c45; continue;
 }
 0x1059c45 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059c1a; continue; }
  pc = 0x1059c4b; continue;
 }
 0x1059c4b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059c4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059c4c); }
 let mut pc: u32 = 0x1059c4c;
 loop { match pc {
 0x1059c4c => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 6u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059c64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059c64); }
 let mut pc: u32 = 0x1059c64;
 loop { match pc {
 0x1059c64 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x500u32; cpu.eax = v; }
  { let v: u32 = 0x10fcc53u32; cpu.edi = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcc53u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 10u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059c7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059c7e); }
 let mut pc: u32 = 0x1059c7e;
 loop { match pc {
 0x1059c7e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10ef397u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x1059c9d; continue;
 }
 0x1059c9d => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059c9d; continue; }
  pc = 0x1059ca9; continue;
 }
 0x1059ca9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059cb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059cb9); }
 let mut pc: u32 = 0x1059cb9;
 loop { match pc {
 0x1059cb9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10ef397u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x1059cd8; continue;
 }
 0x1059cd8 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1059ce6; continue; }
  pc = 0x1059ce1; continue;
 }
 0x1059ce1 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x37u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1059ce9; continue;
 }
 0x1059ce6 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1059ce9; continue;
 }
 0x1059ce9 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059cd8; continue; }
  pc = 0x1059cee; continue;
 }
 0x1059cee => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059cfe); }
 let mut pc: u32 = 0x1059cfe;
 loop { match pc {
 0x1059cfe => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x10ef397u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = 0xau16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1059d17; continue;
 }
 0x1059d17 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059d17; continue; }
  pc = 0x1059d25; continue;
 }
 0x1059d25 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059d27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059d27); }
 let mut pc: u32 = 0x1059d27;
 loop { match pc {
 0x1059d27 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x10ef397u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1059d40; continue;
 }
 0x1059d40 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1059d50; continue; }
  pc = 0x1059d4b; continue;
 }
 0x1059d4b => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1059d53; continue;
 }
 0x1059d50 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1059d53; continue;
 }
 0x1059d53 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059d40; continue; }
  pc = 0x1059d58; continue;
 }
 0x1059d58 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059d5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059d5a); }
 let mut pc: u32 = 0x1059d5a;
 loop { match pc {
 0x1059d5a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x10ef397u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11df16eu32, v); }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if !cpu.sf { pc = 0x1059d80; continue; }
  pc = 0x1059d76; continue;
 }
 0x1059d76 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11df16eu32, v); }
  pc = 0x1059d80; continue;
 }
 0x1059d80 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x1059d89; continue;
 }
 0x1059d89 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1059d89; continue; }
  pc = 0x1059d95; continue;
 }
 0x1059d95 => {
  { cpu.sub8(cpu.r8(0x11df16eu32), 0x0u8); }
  if cpu.zf { pc = 0x1059da1; continue; }
  pc = 0x1059d9e; continue;
 }
 0x1059d9e => {
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1059da1; continue;
 }
 0x1059da1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059da3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059da3); }
 let mut pc: u32 = 0x1059da3;
 loop { match pc {
 0x1059da3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  { cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1059db7; continue; }
  pc = 0x1059db2; continue;
 }
 0x1059db2 => {
  { let v: u32 = 0xacbu32; cpu.eax = v; }
  pc = 0x1059db7; continue;
 }
 0x1059db7 => {
  cpu.push(0x1059dbc); f_01059dbe(cpu); if cpu.returned_to != 0x1059dbc { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059dbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059dbe); }
 let mut pc: u32 = 0x1059dbe;
 loop { match pc {
 0x1059dbe => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xa3u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1059dfa; continue; }
  pc = 0x1059dcf; continue;
 }
 0x1059dcf => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1059de2; continue;
 }
 0x1059de2 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1059df5; continue; }
  pc = 0x1059de7; continue;
 }
 0x1059de7 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1152ef7u32.wrapping_add(cpu.eax), v); }
  pc = 0x1059df5; continue;
 }
 0x1059df5 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1059de2; continue; }
  pc = 0x1059dfa; continue;
 }
 0x1059dfa => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059dfc); }
 let mut pc: u32 = 0x1059dfc;
 loop { match pc {
 0x1059dfc => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3d63u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112447bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x22u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1059e2c); f_0106f7f8(cpu); if cpu.returned_to != 0x1059e2c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x12u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1059e4c); f_0106f7f8(cpu); if cpu.returned_to != 0x1059e4c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1059ea0; continue; }
  pc = 0x1059e56; continue;
 }
 0x1059e56 => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1059ea0; continue; }
  pc = 0x1059e60; continue;
 }
 0x1059e60 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x32u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1059e7f); f_0106f7f8(cpu); if cpu.returned_to != 0x1059e7f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x42u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1059e9f); f_0106f7f8(cpu); if cpu.returned_to != 0x1059e9f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1059ea0; continue;
 }
 0x1059ea0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059ee0); }
 let mut pc: u32 = 0x1059ee0;
 loop { match pc {
 0x1059ee0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u32 = 0xf0u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110577cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e4bu32, v); }
  { let v: u8 = cpu.r8(0x10ee974u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  cpu.push(0x1059f12); f_01059f8c(cpu); if cpu.returned_to != 0x1059f12 { return; }
  { let v: u32 = 0x1105e2eu32; cpu.esi = v; }
  cpu.push(0x1059f1c); f_0105a2f3(cpu); if cpu.returned_to != 0x1059f1c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059f2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059f2a); }
 let mut pc: u32 = 0x1059f2a;
 loop { match pc {
 0x1059f2a => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u8 = cpu.r8(0x10ee974u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  cpu.push(0x1059f43); f_01059f8c(cpu); if cpu.returned_to != 0x1059f43 { return; }
  { let v: u32 = 0x1105e4du32; cpu.esi = v; }
  cpu.push(0x1059f4d); f_0105a2f3(cpu); if cpu.returned_to != 0x1059f4d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059f5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059f5b); }
 let mut pc: u32 = 0x1059f5b;
 loop { match pc {
 0x1059f5b => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u8 = cpu.r8(0x10ee974u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  cpu.push(0x1059f74); f_01059f8c(cpu); if cpu.returned_to != 0x1059f74 { return; }
  { let v: u32 = 0x1105e50u32; cpu.esi = v; }
  cpu.push(0x1059f7e); f_0105a2f3(cpu); if cpu.returned_to != 0x1059f7e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4ccu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059f8c); }
 let mut pc: u32 = 0x1059f8c;
 loop { match pc {
 0x1059f8c => {
  cpu.push(0x1059f91); f_01033b46(cpu); if cpu.returned_to != 0x1059f91 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x1059f96; continue;
 }
 0x1059f96 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1152d54u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x32u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1059f96; continue; }
  pc = 0x1059fa3; continue;
 }
 0x1059fa3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059fa4); }
 let mut pc: u32 = 0x1059fa4;
 loop { match pc {
 0x1059fa4 => {
  { let v: u32 = cpu.r32(0x10e400cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e4014u32)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d02b3u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1059fc0; continue; }
  pc = 0x1059fb8; continue;
 }
 0x1059fb8 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x11d02b3u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1059fc5; continue; }
  pc = 0x1059fc0; continue;
 }
 0x1059fc0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11d02b3u32, v); }
  pc = 0x1059fc5; continue;
 }
 0x1059fc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01059fc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1059fc6); }
 let mut pc: u32 = 0x1059fc6;
 loop { match pc {
 0x1059fc6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1059fc8; continue;
 }
 0x1059fc8 => {
  { cpu.pushad(); }
  cpu.push(0x1059fce); f_0105a0b6(cpu); if cpu.returned_to != 0x1059fce { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x32u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1059fc8; continue; }
  pc = 0x1059fd5; continue;
 }
 0x1059fd5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd360u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a0b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a0b6); }
 let mut pc: u32 = 0x105a0b6;
 loop { match pc {
 0x105a0b6 => {
  { cpu.sub8(cpu.r8(0x10fd4ccu32), 0x0u8); }
  if !cpu.zf { pc = 0x105a0ce; continue; }
  pc = 0x105a0bf; continue;
 }
 0x105a0bf => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x105a0ce; continue; }
  pc = 0x105a0c3; continue;
 }
 0x105a0c3 => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x0u8); }
  if cpu.sf { pc = 0x105a0ce; continue; }
  pc = 0x105a0cc; continue;
 }
 0x105a0cc => {
  if !cpu.zf { pc = 0x105a117; continue; }
  pc = 0x105a0ce; continue;
 }
 0x105a0ce => {
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x105a0d8; continue;
 }
 0x105a0d8 => {
  { cpu.lods(1, false); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x105a102; continue; }
  pc = 0x105a0dd; continue;
 }
 0x105a0dd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11df178u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11df17cu32, v); }
  cpu.push(0x105a0f1); f_0105a118(cpu); if cpu.returned_to != 0x105a0f1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x11df170u32)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x11df170u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x105a102; continue;
 }
 0x105a102 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e3db9u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x105a0d8; continue; }
  pc = 0x105a10c; continue;
 }
 0x105a10c => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x1152d54u32.wrapping_add(cpu.ebx), v); }
  pc = 0x105a117; continue;
 }
 0x105a117 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a2f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a2f3); }
 let mut pc: u32 = 0x105a2f3;
 loop { match pc {
 0x105a2f3 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x105a301; continue; }
  pc = 0x105a2f8; continue;
 }
 0x105a2f8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x105a2fe); f_0105a0b6(cpu); if cpu.returned_to != 0x105a2fe { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x105a2f3; continue;
 }
 0x105a301 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a302(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a302); }
 let mut pc: u32 = 0x105a302;
 loop { match pc {
 0x105a302 => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x1u8); }
  if cpu.zf { pc = 0x105a316; continue; }
  pc = 0x105a30b; continue;
 }
 0x105a30b => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1152d54u32.wrapping_add(cpu.ebx), v); }
  pc = 0x105a316; continue;
 }
 0x105a316 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a317(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a317); }
 let mut pc: u32 = 0x105a317;
 loop { match pc {
 0x105a317 => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x1u8); }
  if cpu.zf { pc = 0x105a332; continue; }
  pc = 0x105a320; continue;
 }
 0x105a320 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x105a322; continue;
 }
 0x105a322 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x105a332; continue; }
  pc = 0x105a327; continue;
 }
 0x105a327 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1152d54u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x105a322; continue;
 }
 0x105a332 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a333(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a333); }
 let mut pc: u32 = 0x105a333;
 loop { match pc {
 0x105a333 => {
  { cpu.sub8(cpu.r8(0x10fd4cau32), 0x0u8); }
  if !cpu.zf { pc = 0x105a345; continue; }
  pc = 0x105a33c; continue;
 }
 0x105a33c => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x1u8); }
  if cpu.zf { pc = 0x105a394; continue; }
  pc = 0x105a345; continue;
 }
 0x105a345 => {
  { let v: u16 = cpu.r16(0x10e3db9u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e51u32, v); }
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x105a35c; continue;
 }
 0x105a35c => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105a38f; continue; }
  pc = 0x105a361; continue;
 }
 0x105a361 => {
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x10fd4cau32), 0x0u8); }
  if !cpu.zf { pc = 0x105a36f; continue; }
  pc = 0x105a36b; continue;
 }
 0x105a36b => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x105a386; continue; }
  pc = 0x105a36f; continue;
 }
 0x105a36f => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x1152d54u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x105a386; continue; }
  pc = 0x105a37c; continue;
 }
 0x105a37c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef30eu32, v); }
  cpu.push(0x105a386); f_0105a475(cpu); if cpu.returned_to != 0x105a386 { return; }
  pc = 0x105a386; continue;
 }
 0x105a386 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3e51u32)); cpu.w16(0x10e3e51u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x105a35c; continue; }
  pc = 0x105a38f; continue;
 }
 0x105a38f => {
  cpu.push(0x105a394); f_0105a395(cpu); if cpu.returned_to != 0x105a394 { return; }
  pc = 0x105a394; continue;
 }
 0x105a394 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a395(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a395); }
 let mut pc: u32 = 0x105a395;
 loop { match pc {
 0x105a395 => {
  { let v: u32 = 0x118cd68u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3db9u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x105a3a5; continue;
 }
 0x105a3a5 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x105a3b0; continue; }
  pc = 0x105a3aa; continue;
 }
 0x105a3aa => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105a3b0; continue; }
  pc = 0x105a3ae; continue;
 }
 0x105a3ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  pc = 0x105a3b0; continue;
 }
 0x105a3b0 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105a3a5; continue; }
  pc = 0x105a3b5; continue;
 }
 0x105a3b5 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x105a3e1; continue; }
  pc = 0x105a3ba; continue;
 }
 0x105a3ba => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x105a3ca; continue;
 }
 0x105a3ca => {
  { cpu.and32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x105a3d5; continue; }
  pc = 0x105a3d2; continue;
 }
 0x105a3d2 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x105a3ca; continue;
 }
 0x105a3d5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e400cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105a3e1 => {
  { let v: u32 = cpu.r32(0x10e4014u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e400cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a3ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a3ee); }
 let mut pc: u32 = 0x105a3ee;
 loop { match pc {
 0x105a3ee => {
  { cpu.sub8(cpu.r8(0x10ef30fu32), 0x0u8); }
  if cpu.zf { pc = 0x105a474; continue; }
  pc = 0x105a3fb; continue;
 }
 0x105a3fb => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u16 = 0x80u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x20u32; cpu.ebp = v; }
  cpu.push(0x105a418); f_0106fa32(cpu); if cpu.returned_to != 0x105a418 { return; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v: u32 = 0x44u32; cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = 0x7cu32; cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.ebp = v; }
  cpu.push(0x105a436); f_0106fa32(cpu); if cpu.returned_to != 0x105a436 { return; }
  { let v: u32 = cpu.r32(0x10e405cu32); cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e400cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e4014u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 10u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x7cu32; cpu.ebx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = 0x7cu32; cpu.edx = v; }
  { let v: u32 = 0x18u32; cpu.ebp = v; }
  cpu.push(0x105a474); f_0106fa32(cpu); if cpu.returned_to != 0x105a474 { return; }
  pc = 0x105a474; continue;
 }
 0x105a474 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a475(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a475); }
 let mut pc: u32 = 0x105a475;
 loop { match pc {
 0x105a475 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10e4058u32)); cpu.esi = v; }
  { let v: u32 = 0x118cd64u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  pc = 0x105a48e; continue;
 }
 0x105a48e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x105a499; continue; }
  pc = 0x105a493; continue;
 }
 0x105a493 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105a56b; continue;
 }
 0x105a499 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e40c4u32, v); }
  pc = 0x105a4a3; continue;
 }
 0x105a4a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x118cd68u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11cccf0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x105a4c7; continue; }
  pc = 0x105a4c0; continue;
 }
 0x105a4c0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11cccf0u32.wrapping_add(cpu.eax), v); }
  pc = 0x105a4c7; continue;
 }
 0x105a4c7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x105a4d8; continue;
 }
 0x105a4d8 => {
  { cpu.and32(cpu.ebx, 0x3u32); }
  if cpu.zf { pc = 0x105a4e3; continue; }
  pc = 0x105a4e0; continue;
 }
 0x105a4e0 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x105a4d8; continue;
 }
 0x105a4e3 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if cpu.sf { pc = 0x105a52f; continue; }
  pc = 0x105a4ea; continue;
 }
 0x105a4ea => {
  { let v: u32 = cpu.r32(0x10e40c4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3db9u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x118cd68u32; cpu.edi = v; }
  pc = 0x105a500; continue;
 }
 0x105a500 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.esi); }
  if !cpu.zf { pc = 0x105a528; continue; }
  pc = 0x105a504; continue;
 }
 0x105a504 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x118cd68u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e4058u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ef30eu32)); }
  if !cpu.zf { pc = 0x105a52d; continue; }
  pc = 0x105a51f; continue;
 }
 0x105a51f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x105a4a3; continue;
 }
 0x105a528 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105a500; continue; }
  pc = 0x105a52d; continue;
 }
 0x105a52d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x105a52f; continue;
 }
 0x105a52f => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x105a531; continue;
 }
 0x105a531 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e40c4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3db9u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x118cd68u32; cpu.edi = v; }
  pc = 0x105a548; continue;
 }
 0x105a548 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x105a553; continue; }
  pc = 0x105a54c; continue;
 }
 0x105a54c => {
  cpu.push(0x105a551); f_0105a571(cpu); if cpu.returned_to != 0x105a551 { return; }
  cpu.poll(); pc = 0x105a531; continue;
 }
 0x105a553 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105a548; continue; }
  pc = 0x105a558; continue;
 }
 0x105a558 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x105a56b; continue; }
  pc = 0x105a55f; continue;
 }
 0x105a55f => {
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 4u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105a56b; continue; }
  pc = 0x105a568; continue;
 }
 0x105a568 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105a56b; continue;
 }
 0x105a56b => {
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a571(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a571); }
 let mut pc: u32 = 0x105a571;
 loop { match pc {
 0x105a571 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_sub(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e40c4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x105a58b; continue;
 }
 0x105a58b => {
  { cpu.and32(cpu.ebx, 0x3u32); }
  if cpu.zf { pc = 0x105a596; continue; }
  pc = 0x105a593; continue;
 }
 0x105a593 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x105a58b; continue;
 }
 0x105a596 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a5aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a5aa); }
 let mut pc: u32 = 0x105a5aa;
 loop { match pc {
 0x105a5aa => {
  cpu.push(0x105a5af); f_01033a4b(cpu); if cpu.returned_to != 0x105a5af { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e3db9u32) as u32; cpu.edx = v; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x2u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x105a5c5); f_01033a3b(cpu); if cpu.returned_to != 0x105a5c5 { return; }
  { let v: u32 = cpu.r16(0x10e3db9u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.edx = v; }
  cpu.push(0x105a5d7); f_0105e746(cpu); if cpu.returned_to != 0x105a5d7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a73a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a73a); }
 let mut pc: u32 = 0x105a73a;
 loop { match pc {
 0x105a73a => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11298e0u32.wrapping_add(cpu.ebx)); cpu.push(0x105a747); cpu.call(t); if cpu.returned_to != 0x105a747 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a748(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a748); }
 let mut pc: u32 = 0x105a748;
 loop { match pc {
 0x105a748 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  cpu.push(0x105a752); f_0105f215(cpu); if cpu.returned_to != 0x105a752 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105a762); f_0105f215(cpu); if cpu.returned_to != 0x105a762 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x12cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105a76a; continue;
 }
 0x105a76a => {
  cpu.push(0x105a76f); f_0105c651(cpu); if cpu.returned_to != 0x105a76f { return; }
  if cpu.cf { pc = 0x105a7ec; continue; }
  pc = 0x105a771; continue;
 }
 0x105a771 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d47u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a79c); f_0105f215(cpu); if cpu.returned_to != 0x105a79c { return; }
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
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a7dd); f_0105f215(cpu); if cpu.returned_to != 0x105a7dd { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4fu8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105a7ec; continue;
 }
 0x105a7ec => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105a76a; continue; }
  pc = 0x105a7f4; continue;
 }
 0x105a7f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a7f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a7f5); }
 let mut pc: u32 = 0x105a7f5;
 loop { match pc {
 0x105a7f5 => {
  { let v: u32 = 0x105a804u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a804(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a804); }
 let mut pc: u32 = 0x105a804;
 loop { match pc {
 0x105a804 => {
  cpu.push(0x105a809); f_0105f215(cpu); if cpu.returned_to != 0x105a809 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a813); f_0105f215(cpu); if cpu.returned_to != 0x105a813 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105a820); f_0105f215(cpu); if cpu.returned_to != 0x105a820 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a838); f_0105f215(cpu); if cpu.returned_to != 0x105a838 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x10e3ebfu32), 0x76fu16); }
  if !cpu.zf { pc = 0x105a85a; continue; }
  pc = 0x105a853; continue;
 }
 0x105a853 => {
  { let v: u32 = 0x105a8ddu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105a85a; continue;
 }
 0x105a85a => {
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a869(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a869); }
 let mut pc: u32 = 0x105a869;
 loop { match pc {
 0x105a869 => {
  { let v: u32 = 0x105a878u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a878(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a878); }
 let mut pc: u32 = 0x105a878;
 loop { match pc {
 0x105a878 => {
  cpu.push(0x105a87d); f_0105f215(cpu); if cpu.returned_to != 0x105a87d { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a887); f_0105f215(cpu); if cpu.returned_to != 0x105a887 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105a894); f_0105f215(cpu); if cpu.returned_to != 0x105a894 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105a8ac); f_0105f215(cpu); if cpu.returned_to != 0x105a8ac { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x10e3ebfu32), 0x76fu16); }
  if !cpu.zf { pc = 0x105a8ce; continue; }
  pc = 0x105a8c7; continue;
 }
 0x105a8c7 => {
  { let v: u32 = 0x105a8ddu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105a8ce; continue;
 }
 0x105a8ce => {
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105a8dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105a8dd); }
 let mut pc: u32 = 0x105a8dd;
 loop { match pc {
 0x105a8dd => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105a8f8); f_0104f36d(cpu); if cpu.returned_to != 0x105a8f8 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105aa03; continue; }
  pc = 0x105a90a; continue;
 }
 0x105a90a => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105aa03; continue; }
  pc = 0x105a916; continue;
 }
 0x105a916 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x105a92e; continue; }
  pc = 0x105a91e; continue;
 }
 0x105a91e => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x105a92e; continue; }
  pc = 0x105a922; continue;
 }
 0x105a922 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x105a92e; continue; }
  pc = 0x105a926; continue;
 }
 0x105a926 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x105a92e; continue; }
  pc = 0x105a92a; continue;
 }
 0x105a92a => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x105a9a2; continue; }
  pc = 0x105a92e; continue;
 }
 0x105a92e => {
  { cpu.pushad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x105a958); f_01072d00(cpu); if cpu.returned_to != 0x105a958 { return; }
  if cpu.cf { pc = 0x105a97b; continue; }
  pc = 0x105a95a; continue;
 }
 0x105a95a => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105a97a; continue; }
  pc = 0x105a970; continue;
 }
 0x105a970 => {
  cpu.push(0x105a975); f_01012780(cpu); if cpu.returned_to != 0x105a975 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105a97a; continue;
 }
 0x105a97a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105a97b; continue;
 }
 0x105a97b => {
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105a991); f_01072e0c(cpu); if cpu.returned_to != 0x105a991 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { cpu.popad(); }
  pc = 0x105aa02; continue;
 }
 0x105a9a2 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105a9be); f_0101bdfe(cpu); if cpu.returned_to != 0x105a9be { return; }
  if cpu.cf { pc = 0x105a9fb; continue; }
  pc = 0x105a9c0; continue;
 }
 0x105a9c0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1123088u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
  pc = 0x105a9fb; continue;
 }
 0x105a9fb => {
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105aa02; continue;
 }
 0x105aa02 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105aa03 => {
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aa11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aa11); }
 let mut pc: u32 = 0x105aa11;
 loop { match pc {
 0x105aa11 => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105aa34u32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105aa2d); f_0105beaf(cpu); if cpu.returned_to != 0x105aa2d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aa34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aa34); }
 let mut pc: u32 = 0x105aa34;
 loop { match pc {
 0x105aa34 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105aa41); f_0105f215(cpu); if cpu.returned_to != 0x105aa41 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x10u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105aa56); f_0105f215(cpu); if cpu.returned_to != 0x105aa56 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aa75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aa75); }
 let mut pc: u32 = 0x105aa75;
 loop { match pc {
 0x105aa75 => {
  { let v: u32 = 0x105aa84u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aa84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aa84); }
 let mut pc: u32 = 0x105aa84;
 loop { match pc {
 0x105aa84 => {
  cpu.push(0x105aa89); f_0105aa34(cpu); if cpu.returned_to != 0x105aa89 { return; }
  { let v: u8 = 0xf3u8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xbu32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) | 0x30u8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105aac2u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3cu16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105aab1); f_0105f215(cpu); if cpu.returned_to != 0x105aab1 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aac2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aac2); }
 let mut pc: u32 = 0x105aac2;
 loop { match pc {
 0x105aac2 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105aadd); f_0104f36d(cpu); if cpu.returned_to != 0x105aadd { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105ada5; continue; }
  pc = 0x105aaef; continue;
 }
 0x105aaef => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105ada5; continue; }
  pc = 0x105aafb; continue;
 }
 0x105aafb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x105ab13; continue; }
  pc = 0x105ab03; continue;
 }
 0x105ab03 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x105ab13; continue; }
  pc = 0x105ab07; continue;
 }
 0x105ab07 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x105ab13; continue; }
  pc = 0x105ab0b; continue;
 }
 0x105ab0b => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x105ab13; continue; }
  pc = 0x105ab0f; continue;
 }
 0x105ab0f => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x105ab91; continue; }
  pc = 0x105ab13; continue;
 }
 0x105ab13 => {
  { cpu.pushad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105ab3c); f_01072d00(cpu); if cpu.returned_to != 0x105ab3c { return; }
  if cpu.cf { pc = 0x105ab68; continue; }
  pc = 0x105ab3e; continue;
 }
 0x105ab3e => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ab68; continue; }
  pc = 0x105ab47; continue;
 }
 0x105ab47 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105ab67; continue; }
  pc = 0x105ab5d; continue;
 }
 0x105ab5d => {
  cpu.push(0x105ab62); f_01012780(cpu); if cpu.returned_to != 0x105ab62 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105ab67; continue;
 }
 0x105ab67 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105ab68; continue;
 }
 0x105ab68 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  cpu.push(0x105ab7d); f_01072e0c(cpu); if cpu.returned_to != 0x105ab7d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  pc = 0x105ada5; continue;
 }
 0x105ab91 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x166bu16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abb2; continue;
 }
 0x105abb2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x1637u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105abbe; continue;
 }
 0x105abbe => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x616u16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abc6; continue;
 }
 0x105abc6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x162fu16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abce; continue;
 }
 0x105abce => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x16bau16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abd6; continue;
 }
 0x105abd6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x74du16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abde; continue;
 }
 0x105abde => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x14d9u16); }
  if cpu.zf { pc = 0x105ac18; continue; }
  pc = 0x105abe6; continue;
 }
 0x105abe6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x200u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105abee; continue;
 }
 0x105abee => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x14c5u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105abf6; continue;
 }
 0x105abf6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x735u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105abfe; continue;
 }
 0x105abfe => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x1638u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105ac06; continue;
 }
 0x105ac06 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x1640u16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105ac0e; continue;
 }
 0x105ac0e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x163cu16); }
  if cpu.zf { pc = 0x105ac4b; continue; }
  pc = 0x105ac16; continue;
 }
 0x105ac16 => {
  pc = 0x105ac7c; continue;
 }
 0x105ac18 => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  cpu.push(0x105ac26); f_01072d00(cpu); if cpu.returned_to != 0x105ac26 { return; }
  if cpu.cf { pc = 0x105ac7c; continue; }
  pc = 0x105ac28; continue;
 }
 0x105ac28 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105ac48; continue; }
  pc = 0x105ac3e; continue;
 }
 0x105ac3e => {
  cpu.push(0x105ac43); f_01012780(cpu); if cpu.returned_to != 0x105ac43 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105ac48; continue;
 }
 0x105ac48 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105ac7c; continue;
 }
 0x105ac4b => {
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  cpu.push(0x105ac59); f_01072d00(cpu); if cpu.returned_to != 0x105ac59 { return; }
  if cpu.cf { pc = 0x105ac7c; continue; }
  pc = 0x105ac5b; continue;
 }
 0x105ac5b => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105ac7b; continue; }
  pc = 0x105ac71; continue;
 }
 0x105ac71 => {
  cpu.push(0x105ac76); f_01012780(cpu); if cpu.returned_to != 0x105ac76 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105ac7b; continue;
 }
 0x105ac7b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105ac7c; continue;
 }
 0x105ac7c => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105ac88); f_01072e0c(cpu); if cpu.returned_to != 0x105ac88 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { let v: u16 = 0x1d9u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x15u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfbu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x105ad3c; continue; }
  pc = 0x105ad25; continue;
 }
 0x105ad25 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x105ad41; continue; }
  pc = 0x105ad2e; continue;
 }
 0x105ad2e => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x105ad46; continue; }
  pc = 0x105ad37; continue;
 }
 0x105ad37 => {
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  pc = 0x105ad49; continue;
 }
 0x105ad3c => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x105ad49; continue;
 }
 0x105ad41 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x105ad49; continue;
 }
 0x105ad46 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x105ad49; continue;
 }
 0x105ad49 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105ad50); f_0105aa11(cpu); if cpu.returned_to != 0x105ad50 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x105ad90; continue; }
  pc = 0x105ad79; continue;
 }
 0x105ad79 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x105ad95; continue; }
  pc = 0x105ad82; continue;
 }
 0x105ad82 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x105ad9a; continue; }
  pc = 0x105ad8b; continue;
 }
 0x105ad8b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x105ad9d; continue;
 }
 0x105ad90 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x105ad9d; continue;
 }
 0x105ad95 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x105ad9d; continue;
 }
 0x105ad9a => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x105ad9d; continue;
 }
 0x105ad9d => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105ada4); f_0105a7f5(cpu); if cpu.returned_to != 0x105ada4 { return; }
  { cpu.popad(); }
  pc = 0x105ada5; continue;
 }
 0x105ada5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ada6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ada6); }
 let mut pc: u32 = 0x105ada6;
 loop { match pc {
 0x105ada6 => {
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105adb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105adb4); }
 let mut pc: u32 = 0x105adb4;
 loop { match pc {
 0x105adb4 => {
  { let v: u32 = 0x105adc3u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105adc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105adc3); }
 let mut pc: u32 = 0x105adc3;
 loop { match pc {
 0x105adc3 => {
  cpu.push(0x105adc8); f_0105aa34(cpu); if cpu.returned_to != 0x105adc8 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105adefu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105adde); f_0105f215(cpu); if cpu.returned_to != 0x105adde { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105adef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105adef); }
 let mut pc: u32 = 0x105adef;
 loop { match pc {
 0x105adef => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = cpu.r16(0x112308au32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x105ae4f); f_0105f215(cpu); if cpu.returned_to != 0x105ae4f { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105ae5c); f_0105a7f5(cpu); if cpu.returned_to != 0x105ae5c { return; }
  { let v: u32 = cpu.r16(0x112308cu32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x105ae8b; continue; }
  pc = 0x105ae74; continue;
 }
 0x105ae74 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x105ae90; continue; }
  pc = 0x105ae7d; continue;
 }
 0x105ae7d => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x105ae95; continue; }
  pc = 0x105ae86; continue;
 }
 0x105ae86 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x105ae98; continue;
 }
 0x105ae8b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x105ae98; continue;
 }
 0x105ae90 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x105ae98; continue;
 }
 0x105ae95 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x6u32); cpu.eax = v; }
  pc = 0x105ae98; continue;
 }
 0x105ae98 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105ae9f); f_0105a7f5(cpu); if cpu.returned_to != 0x105ae9f { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aea7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aea7); }
 let mut pc: u32 = 0x105aea7;
 loop { match pc {
 0x105aea7 => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105aecau32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105aec3); f_0105beaf(cpu); if cpu.returned_to != 0x105aec3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105aeca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105aeca); }
 let mut pc: u32 = 0x105aeca;
 loop { match pc {
 0x105aeca => {
  cpu.push(0x105aecf); f_0105f215(cpu); if cpu.returned_to != 0x105aecf { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105aed9); f_0105f215(cpu); if cpu.returned_to != 0x105aed9 { return; }
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
  cpu.push(0x105aefa); f_0105f215(cpu); if cpu.returned_to != 0x105aefa { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x105af04; continue; }
  pc = 0x105af02; continue;
 }
 0x105af02 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105af04; continue;
 }
 0x105af04 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105adefu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105af23); f_0105f215(cpu); if cpu.returned_to != 0x105af23 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105af34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105af34); }
 let mut pc: u32 = 0x105af34;
 loop { match pc {
 0x105af34 => {
  { let v: u32 = 0x105af43u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105af43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105af43); }
 let mut pc: u32 = 0x105af43;
 loop { match pc {
 0x105af43 => {
  cpu.push(0x105af48); f_0105aa34(cpu); if cpu.returned_to != 0x105af48 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b734u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105af5e); f_0105f215(cpu); if cpu.returned_to != 0x105af5e { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x105af6e; continue; }
  pc = 0x105af69; continue;
 }
 0x105af69 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  pc = 0x105af6e; continue;
 }
 0x105af6e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105af81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105af81); }
 let mut pc: u32 = 0x105af81;
 loop { match pc {
 0x105af81 => {
  { let v: u32 = 0x105af90u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105af90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105af90); }
 let mut pc: u32 = 0x105af90;
 loop { match pc {
 0x105af90 => {
  cpu.push(0x105af95); f_0105aa34(cpu); if cpu.returned_to != 0x105af95 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b541u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105afab); f_0105f215(cpu); if cpu.returned_to != 0x105afab { return; }
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
pub fn f_0105afc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105afc4); }
 let mut pc: u32 = 0x105afc4;
 loop { match pc {
 0x105afc4 => {
  { let v: u32 = 0x105afd3u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105afd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105afd3); }
 let mut pc: u32 = 0x105afd3;
 loop { match pc {
 0x105afd3 => {
  cpu.push(0x105afd8); f_0105aa34(cpu); if cpu.returned_to != 0x105afd8 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b541u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105afee); f_0105f215(cpu); if cpu.returned_to != 0x105afee { return; }
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
pub fn f_0105b007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b007); }
 let mut pc: u32 = 0x105b007;
 loop { match pc {
 0x105b007 => {
  { cpu.pushad(); }
  pc = 0x105b008; continue;
 }
 0x105b008 => {
  cpu.push(0x105b00d); f_0105c651(cpu); if cpu.returned_to != 0x105b00d { return; }
  if cpu.cf { pc = 0x105b0b4; continue; }
  pc = 0x105b013; continue;
 }
 0x105b013 => {
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
  cpu.push(0x105b03c); f_0105f215(cpu); if cpu.returned_to != 0x105b03c { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b051); f_0105f215(cpu); if cpu.returned_to != 0x105b051 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b176u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11230c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b08f); f_0105f215(cpu); if cpu.returned_to != 0x105b08f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b09c); f_0105f215(cpu); if cpu.returned_to != 0x105b09c { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b0a9); f_0105f215(cpu); if cpu.returned_to != 0x105b0a9 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105b0b4; continue;
 }
 0x105b0b4 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105b008; continue; }
  pc = 0x105b0bc; continue;
 }
 0x105b0bc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b0be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b0be); }
 let mut pc: u32 = 0x105b0be;
 loop { match pc {
 0x105b0be => {
  { cpu.pushad(); }
  pc = 0x105b0bf; continue;
 }
 0x105b0bf => {
  cpu.push(0x105b0c4); f_0105c651(cpu); if cpu.returned_to != 0x105b0c4 { return; }
  if cpu.cf { pc = 0x105b16c; continue; }
  pc = 0x105b0ca; continue;
 }
 0x105b0ca => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x32u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b0f5); f_0105f215(cpu); if cpu.returned_to != 0x105b0f5 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b10a); f_0105f215(cpu); if cpu.returned_to != 0x105b10a { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b176u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11230c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b147); f_0105f215(cpu); if cpu.returned_to != 0x105b147 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b154); f_0105f215(cpu); if cpu.returned_to != 0x105b154 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b161); f_0105f215(cpu); if cpu.returned_to != 0x105b161 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105b16c; continue;
 }
 0x105b16c => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105b0bf; continue; }
  pc = 0x105b174; continue;
 }
 0x105b174 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b176(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b176); }
 let mut pc: u32 = 0x105b176;
 loop { match pc {
 0x105b176 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x16u8; cpu.w8(0x10ef32cu32, v); }
  cpu.push(0x105b1cb); f_0105bf68(cpu); if cpu.returned_to != 0x105b1cb { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b1d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b1d3); }
 let mut pc: u32 = 0x105b1d3;
 loop { match pc {
 0x105b1d3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x105b210); f_0108c1c5(cpu); if cpu.returned_to != 0x105b210 { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ecx as u8), 0x18u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10ef32cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x105b234); f_0105bf93(cpu); if cpu.returned_to != 0x105b234 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b236(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b236); }
 let mut pc: u32 = 0x105b236;
 loop { match pc {
 0x105b236 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x105b23d); f_0105c651(cpu); if cpu.returned_to != 0x105b23d { return; }
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
  cpu.push(0x105b25f); f_0104f36d(cpu); if cpu.returned_to != 0x105b25f { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x105b2a2); f_0105f215(cpu); if cpu.returned_to != 0x105b2a2 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105b2af); f_0105a7f5(cpu); if cpu.returned_to != 0x105b2af { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b2b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b2b7); }
 let mut pc: u32 = 0x105b2b7;
 loop { match pc {
 0x105b2b7 => {
  { let v: u32 = 0x105b2c6u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b2c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b2c6); }
 let mut pc: u32 = 0x105b2c6;
 loop { match pc {
 0x105b2c6 => {
  cpu.push(0x105b2cb); f_0105aa34(cpu); if cpu.returned_to != 0x105b2cb { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x2u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b2fau32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b2e1); f_0105f215(cpu); if cpu.returned_to != 0x105b2e1 { return; }
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
pub fn f_0105b2fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b2fa); }
 let mut pc: u32 = 0x105b2fa;
 loop { match pc {
 0x105b2fa => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b31f); f_01075003(cpu); if cpu.returned_to != 0x105b31f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105b343); f_0104f36d(cpu); if cpu.returned_to != 0x105b343 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105b41a; continue; }
  pc = 0x105b355; continue;
 }
 0x105b355 => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105b41a; continue; }
  pc = 0x105b361; continue;
 }
 0x105b361 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x105b379; continue; }
  pc = 0x105b369; continue;
 }
 0x105b369 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x105b379; continue; }
  pc = 0x105b36d; continue;
 }
 0x105b36d => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x105b379; continue; }
  pc = 0x105b371; continue;
 }
 0x105b371 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x105b379; continue; }
  pc = 0x105b375; continue;
 }
 0x105b375 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x105b3b6; continue; }
  pc = 0x105b379; continue;
 }
 0x105b379 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b3ac); f_01072e0c(cpu); if cpu.returned_to != 0x105b3ac { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  pc = 0x105b419; continue;
 }
 0x105b3b6 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x105b405); f_0105f215(cpu); if cpu.returned_to != 0x105b405 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105b412); f_0105a7f5(cpu); if cpu.returned_to != 0x105b412 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105b419; continue;
 }
 0x105b419 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105b41a => {
  { cpu.popad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b429); }
 let mut pc: u32 = 0x105b429;
 loop { match pc {
 0x105b429 => {
  { cpu.pushad(); }
  cpu.push(0x105b42f); f_0105f215(cpu); if cpu.returned_to != 0x105b42f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b541(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b541); }
 let mut pc: u32 = 0x105b541;
 loop { match pc {
 0x105b541 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b566); f_01075003(cpu); if cpu.returned_to != 0x105b566 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105b58a); f_0104f36d(cpu); if cpu.returned_to != 0x105b58a { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105b725; continue; }
  pc = 0x105b59c; continue;
 }
 0x105b59c => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105b725; continue; }
  pc = 0x105b5a8; continue;
 }
 0x105b5a8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x105b5c0; continue; }
  pc = 0x105b5b0; continue;
 }
 0x105b5b0 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x105b5c0; continue; }
  pc = 0x105b5b4; continue;
 }
 0x105b5b4 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x105b5c0; continue; }
  pc = 0x105b5b8; continue;
 }
 0x105b5b8 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x105b5c0; continue; }
  pc = 0x105b5bc; continue;
 }
 0x105b5bc => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if !cpu.zf { pc = 0x105b636; continue; }
  pc = 0x105b5c0; continue;
 }
 0x105b5c0 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x105b5ec); f_01072d00(cpu); if cpu.returned_to != 0x105b5ec { return; }
  if cpu.cf { pc = 0x105b60f; continue; }
  pc = 0x105b5ee; continue;
 }
 0x105b5ee => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105b60e; continue; }
  pc = 0x105b604; continue;
 }
 0x105b604 => {
  cpu.push(0x105b609); f_01012780(cpu); if cpu.returned_to != 0x105b609 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105b60e; continue;
 }
 0x105b60e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105b60f; continue;
 }
 0x105b60f => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b622); f_01072e0c(cpu); if cpu.returned_to != 0x105b622 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { cpu.popad(); }
  pc = 0x105b724; continue;
 }
 0x105b636 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x105b656); f_01072d00(cpu); if cpu.returned_to != 0x105b656 { return; }
  if cpu.cf { pc = 0x105b67b; continue; }
  pc = 0x105b658; continue;
 }
 0x105b658 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105b67a; continue; }
  pc = 0x105b670; continue;
 }
 0x105b670 => {
  cpu.push(0x105b675); f_01012780(cpu); if cpu.returned_to != 0x105b675 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105b67a; continue;
 }
 0x105b67a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105b67b; continue;
 }
 0x105b67b => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b68e); f_01072e0c(cpu); if cpu.returned_to != 0x105b68e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x105b707; continue; }
  pc = 0x105b6f0; continue;
 }
 0x105b6f0 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x105b70c; continue; }
  pc = 0x105b6f9; continue;
 }
 0x105b6f9 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x105b711; continue; }
  pc = 0x105b702; continue;
 }
 0x105b702 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x105b714; continue;
 }
 0x105b707 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x105b714; continue;
 }
 0x105b70c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x105b714; continue;
 }
 0x105b711 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x105b714; continue;
 }
 0x105b714 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105b71d); f_0105a7f5(cpu); if cpu.returned_to != 0x105b71d { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105b724; continue;
 }
 0x105b724 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105b725 => {
  { cpu.popad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b734(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b734); }
 let mut pc: u32 = 0x105b734;
 loop { match pc {
 0x105b734 => {
  cpu.push(0x105b739); f_01072d00(cpu); if cpu.returned_to != 0x105b739 { return; }
  if cpu.cf { pc = 0x105b76e; continue; }
  pc = 0x105b73b; continue;
 }
 0x105b73b => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x105b76e; continue; }
  pc = 0x105b744; continue;
 }
 0x105b744 => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105b765; continue; }
  pc = 0x105b75b; continue;
 }
 0x105b75b => {
  cpu.push(0x105b760); f_01012780(cpu); if cpu.returned_to != 0x105b760 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105b765; continue;
 }
 0x105b765 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x105b76e; continue;
 }
 0x105b76e => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x32u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b793); f_01075003(cpu); if cpu.returned_to != 0x105b793 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x105b807; continue; }
  pc = 0x105b7f0; continue;
 }
 0x105b7f0 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x105b80c; continue; }
  pc = 0x105b7f9; continue;
 }
 0x105b7f9 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x105b811; continue; }
  pc = 0x105b802; continue;
 }
 0x105b802 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x105b814; continue;
 }
 0x105b807 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x105b814; continue;
 }
 0x105b80c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x105b814; continue;
 }
 0x105b811 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x105b814; continue;
 }
 0x105b814 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x105b81c); f_0105a7f5(cpu); if cpu.returned_to != 0x105b81c { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b824(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b824); }
 let mut pc: u32 = 0x105b824;
 loop { match pc {
 0x105b824 => {
  { let v: u32 = 0x105b833u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b833(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b833); }
 let mut pc: u32 = 0x105b833;
 loop { match pc {
 0x105b833 => {
  cpu.push(0x105b838); f_0105aa34(cpu); if cpu.returned_to != 0x105b838 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub(0x3u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105b87fu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x190u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b84e); f_0105f215(cpu); if cpu.returned_to != 0x105b84e { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b86b); f_0105f215(cpu); if cpu.returned_to != 0x105b86b { return; }
  { let v: u8 = (cpu.eax as u8) & 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105b877); f_0105f215(cpu); if cpu.returned_to != 0x105b877 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105b87f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105b87f); }
 let mut pc: u32 = 0x105b87f;
 loop { match pc {
 0x105b87f => {
  cpu.push(0x105b884); f_01072d00(cpu); if cpu.returned_to != 0x105b884 { return; }
  if cpu.cf { pc = 0x105b8b7; continue; }
  pc = 0x105b886; continue;
 }
 0x105b886 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x105b8b7; continue; }
  pc = 0x105b88f; continue;
 }
 0x105b88f => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x105b8af; continue; }
  pc = 0x105b8a5; continue;
 }
 0x105b8a5 => {
  cpu.push(0x105b8aa); f_01012780(cpu); if cpu.returned_to != 0x105b8aa { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x105b8af; continue;
 }
 0x105b8af => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x105b8b7; continue;
 }
 0x105b8b7 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105b8d2); f_0104f36d(cpu); if cpu.returned_to != 0x105b8d2 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105ba33; continue; }
  pc = 0x105b8e4; continue;
 }
 0x105b8e4 => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105ba33; continue; }
  pc = 0x105b8f0; continue;
 }
 0x105b8f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x105b908; continue; }
  pc = 0x105b8f8; continue;
 }
 0x105b8f8 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x105b908; continue; }
  pc = 0x105b8fc; continue;
 }
 0x105b8fc => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x105b908; continue; }
  pc = 0x105b900; continue;
 }
 0x105b900 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x105b908; continue; }
  pc = 0x105b904; continue;
 }
 0x105b904 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if !cpu.zf { pc = 0x105b947; continue; }
  pc = 0x105b908; continue;
 }
 0x105b908 => {
  { cpu.pushad(); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x28u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b93a); f_01072e0c(cpu); if cpu.returned_to != 0x105b93a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  pc = 0x105ba32; continue;
 }
 0x105b947 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x28u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x105b96c); f_01072e0c(cpu); if cpu.returned_to != 0x105b96c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { let v: u16 = 0x1d9u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x15u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfbu8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x54u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  cpu.push(0x105b9fc); f_0105f215(cpu); if cpu.returned_to != 0x105b9fc { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x105ba09); f_0105aa11(cpu); if cpu.returned_to != 0x105ba09 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x105ba31); f_0105a7f5(cpu); if cpu.returned_to != 0x105ba31 { return; }
  { cpu.popad(); }
  pc = 0x105ba32; continue;
 }
 0x105ba32 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ba33 => {
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ba41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ba41); }
 let mut pc: u32 = 0x105ba41;
 loop { match pc {
 0x105ba41 => {
  { let v: u32 = 0x105ba50u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ba50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ba50); }
 let mut pc: u32 = 0x105ba50;
 loop { match pc {
 0x105ba50 => {
  cpu.push(0x105ba55); f_0105af43(cpu); if cpu.returned_to != 0x105ba55 { return; }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
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
pub fn f_0105ba7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ba7e); }
 let mut pc: u32 = 0x105ba7e;
 loop { match pc {
 0x105ba7e => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105baa1u32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105ba9a); f_0105beaf(cpu); if cpu.returned_to != 0x105ba9a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105baa1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105baa1); }
 let mut pc: u32 = 0x105baa1;
 loop { match pc {
 0x105baa1 => {
  cpu.push(0x105baa6); f_0105f215(cpu); if cpu.returned_to != 0x105baa6 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bab0); f_0105f215(cpu); if cpu.returned_to != 0x105bab0 { return; }
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
  cpu.push(0x105bad1); f_0105f215(cpu); if cpu.returned_to != 0x105bad1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x105badb; continue; }
  pc = 0x105bad9; continue;
 }
 0x105bad9 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105badb; continue;
 }
 0x105badb => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105bb0bu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bafa); f_0105f215(cpu); if cpu.returned_to != 0x105bafa { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bb0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bb0b); }
 let mut pc: u32 = 0x105bb0b;
 loop { match pc {
 0x105bb0b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x9u8; cpu.w8(0x10ef32cu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x105bb60); f_0105bf68(cpu); if cpu.returned_to != 0x105bb60 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bb68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bb68); }
 let mut pc: u32 = 0x105bb68;
 loop { match pc {
 0x105bb68 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef32cu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x105bbbd); f_0105bf68(cpu); if cpu.returned_to != 0x105bbbd { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bbc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bbc5); }
 let mut pc: u32 = 0x105bbc5;
 loop { match pc {
 0x105bbc5 => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105bbe8u32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105bbe1); f_0105beaf(cpu); if cpu.returned_to != 0x105bbe1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bbe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bbe8); }
 let mut pc: u32 = 0x105bbe8;
 loop { match pc {
 0x105bbe8 => {
  cpu.push(0x105bbed); f_0105f215(cpu); if cpu.returned_to != 0x105bbed { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bbf7); f_0105f215(cpu); if cpu.returned_to != 0x105bbf7 { return; }
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
  cpu.push(0x105bc16); f_0105f215(cpu); if cpu.returned_to != 0x105bc16 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x105bc1e; continue; }
  pc = 0x105bc1c; continue;
 }
 0x105bc1c => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105bc1e; continue;
 }
 0x105bc1e => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105bb68u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bc3d); f_0105f215(cpu); if cpu.returned_to != 0x105bc3d { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bc4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bc4e); }
 let mut pc: u32 = 0x105bc4e;
 loop { match pc {
 0x105bc4e => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105bc71u32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105bc6a); f_0105beaf(cpu); if cpu.returned_to != 0x105bc6a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bc71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bc71); }
 let mut pc: u32 = 0x105bc71;
 loop { match pc {
 0x105bc71 => {
  cpu.push(0x105bc76); f_0105f215(cpu); if cpu.returned_to != 0x105bc76 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bc80); f_0105f215(cpu); if cpu.returned_to != 0x105bc80 { return; }
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
  cpu.push(0x105bca1); f_0105f215(cpu); if cpu.returned_to != 0x105bca1 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x105bcab; continue; }
  pc = 0x105bca9; continue;
 }
 0x105bca9 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105bcab; continue;
 }
 0x105bcab => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105bcdbu32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1f4u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bcca); f_0105f215(cpu); if cpu.returned_to != 0x105bcca { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bcdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bcdb); }
 let mut pc: u32 = 0x105bcdb;
 loop { match pc {
 0x105bcdb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x9u8; cpu.w8(0x10ef32cu32, v); }
  cpu.push(0x105bd30); f_0105bf68(cpu); if cpu.returned_to != 0x105bd30 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bd38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bd38); }
 let mut pc: u32 = 0x105bd38;
 loop { match pc {
 0x105bd38 => {
  cpu.push(0x105bd3d); f_0105c651(cpu); if cpu.returned_to != 0x105bd3d { return; }
  if cpu.cf { pc = 0x105bdd0; continue; }
  pc = 0x105bd43; continue;
 }
 0x105bd43 => {
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
  { let v: u16 = 0xdd1u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
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
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x226u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bdc0); f_0105f215(cpu); if cpu.returned_to != 0x105bdc0 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x105bdd0; continue;
 }
 0x105bdd0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bdd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bdd1); }
 let mut pc: u32 = 0x105bdd1;
 loop { match pc {
 0x105bdd1 => {
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u32 = 0x105bdf4u32; cpu.w32(0x10e409cu32, v); }
  cpu.push(0x105bded); f_0105beaf(cpu); if cpu.returned_to != 0x105bded { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bdf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bdf4); }
 let mut pc: u32 = 0x105bdf4;
 loop { match pc {
 0x105bdf4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bdfd); f_0105f215(cpu); if cpu.returned_to != 0x105bdfd { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x1u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105be18); f_0105f215(cpu); if cpu.returned_to != 0x105be18 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.sf == cpu.of) { pc = 0x105be22; continue; }
  pc = 0x105be20; continue;
 }
 0x105be20 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105be22; continue;
 }
 0x105be22 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105be52u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xfau16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105be41); f_0105f215(cpu); if cpu.returned_to != 0x105be41 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105be52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105be52); }
 let mut pc: u32 = 0x105be52;
 loop { match pc {
 0x105be52 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11298c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32cu32, v); }
  cpu.push(0x105bea7); f_0105bf68(cpu); if cpu.returned_to != 0x105bea7 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105beaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105beaf); }
 let mut pc: u32 = 0x105beaf;
 loop { match pc {
 0x105beaf => {
  { cpu.sub8(cpu.r8(0x10ef33au32), 0x0u8); }
  if !cpu.zf { pc = 0x105bf67; continue; }
  pc = 0x105bebc; continue;
 }
 0x105bebc => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  pc = 0x105bec8; continue;
 }
 0x105bec8 => {
  cpu.push(0x105becd); f_0105c651(cpu); if cpu.returned_to != 0x105becd { return; }
  if cpu.cf { pc = 0x105bf67; continue; }
  pc = 0x105bed3; continue;
 }
 0x105bed3 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d47u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3ebfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef342u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1122ff1u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1122ff5u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1122ff9u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x23u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x10e409cu32); cpu.push(0x105bf60); cpu.call(t); if cpu.returned_to != 0x105bf60 { return; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x105bec8; continue; }
  pc = 0x105bf67; continue;
 }
 0x105bf67 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bf68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bf68); }
 let mut pc: u32 = 0x105bf68;
 loop { match pc {
 0x105bf68 => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if (cpu.sf == cpu.of) { f_0105bf93(cpu); return; }
  pc = 0x105bf76; continue;
 }
 0x105bf76 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if !cpu.zf { f_0105bf93(cpu); return; }
  pc = 0x105bf84; continue;
 }
 0x105bf84 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x105bf8e); f_0105f215(cpu); if cpu.returned_to != 0x105bf8e { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  f_0105bf93(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105bf93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105bf93); }
 let mut pc: u32 = 0x105bf93;
 loop { match pc {
 0x105bf93 => {
  { cpu.sub8(cpu.r8(0x10ef33au32), 0x0u8); }
  if !cpu.zf { pc = 0x105c059; continue; }
  pc = 0x105bfa0; continue;
 }
 0x105bfa0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x105bfa4; continue;
 }
 0x105bfa4 => {
  cpu.push(0x105bfa9); f_0105c651(cpu); if cpu.returned_to != 0x105bfa9 { return; }
  if cpu.cf { pc = 0x105c059; continue; }
  pc = 0x105bfaf; continue;
 }
 0x105bfaf => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x105bfc6); f_0105f215(cpu); if cpu.returned_to != 0x105bfc6 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d47u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bfe4); f_0105f215(cpu); if cpu.returned_to != 0x105bfe4 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x8u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105bff9); f_0105f215(cpu); if cpu.returned_to != 0x105bff9 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3ebfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105c030); f_0105f215(cpu); if cpu.returned_to != 0x105c030 { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x2fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105bfa4; continue; }
  pc = 0x105c059; continue;
 }
 0x105c059 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c05a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c05a); }
 let mut pc: u32 = 0x105c05a;
 loop { match pc {
 0x105c05a => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c061(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c061); }
 let mut pc: u32 = 0x105c061;
 loop { match pc {
 0x105c061 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.sf { pc = 0x105c06b; continue; }
  pc = 0x105c067; continue;
 }
 0x105c067 => {
  if cpu.zf { pc = 0x105c06b; continue; }
  pc = 0x105c069; continue;
 }
 0x105c069 => {
  pc = 0x105c06f; continue;
 }
 0x105c06b => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105c06f; continue;
 }
 0x105c06f => {
  { cpu.sub8(cpu.r8(0x10ef33au32), 0x0u8); }
  if !cpu.zf { pc = 0x105c148; continue; }
  pc = 0x105c07c; continue;
 }
 0x105c07c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x105c080; continue;
 }
 0x105c080 => {
  cpu.push(0x105c085); f_0105c651(cpu); if cpu.returned_to != 0x105c085 { return; }
  if cpu.cf { pc = 0x105c148; continue; }
  pc = 0x105c08b; continue;
 }
 0x105c08b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x105c0a3); f_0105f215(cpu); if cpu.returned_to != 0x105c0a3 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d47u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d49u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105c0c9); f_0105f2ec(cpu); if cpu.returned_to != 0x105c0c9 { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105c0db); f_0105f2ec(cpu); if cpu.returned_to != 0x105c0db { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105c0e5); f_0105f215(cpu); if cpu.returned_to != 0x105c0e5 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3ebfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x18u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x38u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105c124); f_0105f2ec(cpu); if cpu.returned_to != 0x105c124 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef32fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105c080; continue; }
  pc = 0x105c148; continue;
 }
 0x105c148 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c149(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c149); }
 let mut pc: u32 = 0x105c149;
 loop { match pc {
 0x105c149 => {
  { let v: u32 = 0x105c158u32; cpu.w32(0x10e409cu32, v); }
  f_0105beaf(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c158); }
 let mut pc: u32 = 0x105c158;
 loop { match pc {
 0x105c158 => {
  cpu.push(0x105c15d); f_0105f215(cpu); if cpu.returned_to != 0x105c15d { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105c167); f_0105f215(cpu); if cpu.returned_to != 0x105c167 { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105c174); f_0105f215(cpu); if cpu.returned_to != 0x105c174 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x707u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0xcu8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x105c18c); f_0105f215(cpu); if cpu.returned_to != 0x105c18c { return; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x105c05au32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x40u16; cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c1b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c1b4); }
 let mut pc: u32 = 0x105c1b4;
 loop { match pc {
 0x105c1b4 => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x105c1ba); cpu.call(t); if cpu.returned_to != 0x105c1ba { return; }
  { let v: u16 = 0x1u16; cpu.w16(0x11298d0u32, v); }
  { let v: u16 = 0x1d7u16; cpu.w16(0x11298d2u32, v); }
  { let v: u16 = 0xff00u16; cpu.w16(0x11298d4u32, v); }
  { let v: u16 = 0x352u16; cpu.w16(0x11298d8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11298dcu32, v); }
  { let v: u32 = 0x1139714u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e41acu32); cpu.ecx = v; }
  pc = 0x105c1f3; continue;
 }
 0x105c1f3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xfc18u16); }
  if !cpu.zf { pc = 0x105c201; continue; }
  pc = 0x105c1fb; continue;
 }
 0x105c1fb => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x2cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105c1f3; continue; }
  pc = 0x105c200; continue;
 }
 0x105c200 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105c201 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11298dcu32)); cpu.w32(0x11298dcu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x12u32.wrapping_add(cpu.ebp))); cpu.w16(0x12u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x105c615; continue; }
  pc = 0x105c212; continue;
 }
 0x105c212 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11298a8u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11298aau32, v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11298acu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x105c236); f_0105c7bf(cpu); if cpu.returned_to != 0x105c236 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x105c53e; continue; }
  pc = 0x105c23d; continue;
 }
 0x105c23d => {
  { let v: u16 = cpu.r16(0x11298c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11298c2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x105c322; continue; }
  pc = 0x105c26a; continue;
 }
 0x105c26a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c277; continue;
 }
 0x105c277 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c284; continue;
 }
 0x105c284 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x105c2ba; continue; }
  pc = 0x105c28a; continue;
 }
 0x105c28a => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x105c2ee; continue; }
  pc = 0x105c290; continue;
 }
 0x105c290 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c29d; continue;
 }
 0x105c29d => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c2aa; continue;
 }
 0x105c2aa => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e4324u32); cpu.push(0x105c2b4); cpu.call(t); if cpu.returned_to != 0x105c2b4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x105c53e; continue;
 }
 0x105c2ba => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c2cc; continue;
 }
 0x105c2cc => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c2de; continue;
 }
 0x105c2de => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e4328u32); cpu.push(0x105c2e8); cpu.call(t); if cpu.returned_to != 0x105c2e8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x105c53e; continue;
 }
 0x105c2ee => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c300; continue;
 }
 0x105c300 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c312; continue;
 }
 0x105c312 => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e4384u32); cpu.push(0x105c31c); cpu.call(t); if cpu.returned_to != 0x105c31c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x105c53e; continue;
 }
 0x105c322 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0xfau16); }
  if cpu.zf { pc = 0x105c4cb; continue; }
  pc = 0x105c32e; continue;
 }
 0x105c32e => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x105c3a1; continue; }
  pc = 0x105c334; continue;
 }
 0x105c334 => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x105c39d; continue; }
  pc = 0x105c33a; continue;
 }
 0x105c33a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x11298c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11298c2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
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
  pc = 0x105c39d; continue;
 }
 0x105c39d => {
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)) ^ 0x8u8; cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x105c3a1; continue;
 }
 0x105c3a1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c3b2; continue;
 }
 0x105c3b2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c3c3; continue;
 }
 0x105c3c3 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x46u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xc8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c3d9; continue;
 }
 0x105c3d9 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c3eb; continue;
 }
 0x105c3eb => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x46u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.edi)); cpu.push(0x105c404); cpu.call(t); if cpu.returned_to != 0x105c404 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x105c48b; continue; }
  pc = 0x105c415; continue;
 }
 0x105c415 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if (cpu.cf || cpu.zf) { pc = 0x105c48b; continue; }
  pc = 0x105c41e; continue;
 }
 0x105c41e => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10c052cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if (cpu.cf || cpu.zf) { pc = 0x105c45b; continue; }
  pc = 0x105c443; continue;
 }
 0x105c443 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x20u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4130u32, v); }
  pc = 0x105c45b; continue;
 }
 0x105c45b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10b00eau32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x105c47f; continue; }
  pc = 0x105c46b; continue;
 }
 0x105c46b => {
  { cpu.pushad(); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = 0x10e4308u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.popad(); }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x105c47f); cpu.call(t); if cpu.returned_to != 0x105c47f { return; }
  pc = 0x105c47f; continue;
 }
 0x105c47f => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  pc = 0x105c4af; continue;
 }
 0x105c48b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10b00eau32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x105c4af; continue; }
  pc = 0x105c49b; continue;
 }
 0x105c49b => {
  { cpu.pushad(); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = 0x10e42ecu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.popad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x105c4af); cpu.call(t); if cpu.returned_to != 0x105c4af { return; }
  pc = 0x105c4af; continue;
 }
 0x105c4af => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x105c53e; continue; }
  pc = 0x105c4ba; continue;
 }
 0x105c4ba => {
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x19u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c4c5; continue;
 }
 0x105c4c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105c53e; continue;
 }
 0x105c4cb => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c4d4; continue;
 }
 0x105c4d4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c4dd; continue;
 }
 0x105c4dd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c4e6; continue;
 }
 0x105c4e6 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105c53e; continue; }
  pc = 0x105c4ef; continue;
 }
 0x105c4ef => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 5u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c505; continue; }
  pc = 0x105c503; continue;
 }
 0x105c503 => {
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x105c505; continue;
 }
 0x105c505 => {
  { let v: u16 = cpu.shl16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xfau16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10b00eau32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x105c536; continue; }
  pc = 0x105c522; continue;
 }
 0x105c522 => {
  { cpu.pushad(); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = 0x10e42ecu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.popad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x105c536); cpu.call(t); if cpu.returned_to != 0x105c536 { return; }
  pc = 0x105c536; continue;
 }
 0x105c536 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105c53e; continue;
 }
 0x105c53e => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11298c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x105c649; continue; }
  pc = 0x105c564; continue;
 }
 0x105c564 => {
  { cpu.sub16(cpu.r16(0x10e3d17u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c649; continue; }
  pc = 0x105c571; continue;
 }
 0x105c571 => {
  { let v: u16 = cpu.r16(0x11298c2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x105c649; continue; }
  pc = 0x105c585; continue;
 }
 0x105c585 => {
  { cpu.sub16(cpu.r16(0x10e3d19u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c649; continue; }
  pc = 0x105c592; continue;
 }
 0x105c592 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.add16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105c5b3; continue; }
  pc = 0x105c59d; continue;
 }
 0x105c59d => {
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x105c615; continue; }
  pc = 0x105c5a9; continue;
 }
 0x105c5a9 => {
  { let v: u8 = cpu.neg8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x105c5b3; continue; }
  pc = 0x105c5b1; continue;
 }
 0x105c5b1 => {
  pc = 0x105c615; continue;
 }
 0x105c5b3 => {
  { let v: u8 = cpu.dec8(cpu.r8(0xeu32.wrapping_add(cpu.ebp))); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x105c5f7; continue; }
  pc = 0x105c5b8; continue;
 }
 0x105c5b8 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x105c5d3; continue; }
  pc = 0x105c5c7; continue;
 }
 0x105c5c7 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x105c5d3; continue; }
  pc = 0x105c5cb; continue;
 }
 0x105c5cb => {
  if cpu.sf { pc = 0x105c5d1; continue; }
  pc = 0x105c5cd; continue;
 }
 0x105c5cd => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105c5d3; continue;
 }
 0x105c5d1 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105c5d3; continue;
 }
 0x105c5d3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x105c5eb; continue; }
  pc = 0x105c5df; continue;
 }
 0x105c5df => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x105c5eb; continue; }
  pc = 0x105c5e3; continue;
 }
 0x105c5e3 => {
  if cpu.sf { pc = 0x105c5e9; continue; }
  pc = 0x105c5e5; continue;
 }
 0x105c5e5 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105c5eb; continue;
 }
 0x105c5e9 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105c5eb; continue;
 }
 0x105c5eb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x105c5f7; continue; }
  pc = 0x105c5f4; continue;
 }
 0x105c5f4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x105c5f7; continue;
 }
 0x105c5f7 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x105c60f; continue; }
  pc = 0x105c5fc; continue;
 }
 0x105c5fc => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0xfu32.wrapping_add(cpu.ebp))); cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfu32.wrapping_add(cpu.ebp)), 0x3fu8); }
  if !cpu.zf { pc = 0x105c60f; continue; }
  pc = 0x105c60b; continue;
 }
 0x105c60b => {
  { let v: u8 = 0x40u8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  pc = 0x105c60f; continue;
 }
 0x105c60f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x105c1fb; continue;
 }
 0x105c615 => {
  { cpu.and8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x105c631; continue; }
  pc = 0x105c61b; continue;
 }
 0x105c61b => {
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
  pc = 0x105c631; continue;
 }
 0x105c631 => {
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x105c634); cpu.call(t); if cpu.returned_to != 0x105c634 { return; }
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x105c60f; continue; }
  pc = 0x105c63b; continue;
 }
 0x105c63b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xfc18u16); }
  if !cpu.zf { cpu.poll(); pc = 0x105c212; continue; }
  pc = 0x105c647; continue;
 }
 0x105c647 => {
  cpu.poll(); pc = 0x105c60f; continue;
 }
 0x105c649 => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x105c60f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c651(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c651); }
 let mut pc: u32 = 0x105c651;
 loop { match pc {
 0x105c651 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1139714u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11b5414u32); cpu.ecx = v; }
  pc = 0x105c65d; continue;
 }
 0x105c65d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xfc18u16); }
  if cpu.zf { pc = 0x105c66c; continue; }
  pc = 0x105c664; continue;
 }
 0x105c664 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x2cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105c65d; continue; }
  pc = 0x105c669; continue;
 }
 0x105c669 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105c66c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c66f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c66f); }
 let mut pc: u32 = 0x105c66f;
 loop { match pc {
 0x105c66f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1139714u32; cpu.edi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x105c67a; continue;
 }
 0x105c67a => {
  { let v: u16 = 0xfc18u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105c67a; continue; }
  pc = 0x105c684; continue;
 }
 0x105c684 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c687(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c687); }
 let mut pc: u32 = 0x105c687;
 loop { match pc {
 0x105c687 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c68a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c68a); }
 let mut pc: u32 = 0x105c68a;
 loop { match pc {
 0x105c68a => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x105c691; continue;
 }
 0x105c691 => {
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x105c698; continue;
 }
 0x105c698 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11298a8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11298aau32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11298acu32, v); }
  cpu.push(0x105c6b7); f_0105c7bf(cpu); if cpu.returned_to != 0x105c6b7 { return; }
  if cpu.cf { pc = 0x105c754; continue; }
  pc = 0x105c6bd; continue;
 }
 0x105c6bd => {
  { let v = cpu.r16(0x11298c0u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x11298c2u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x11298a8u32), 0x40u16); cpu.w16(0x11298a8u32, v); }
  cpu.push(0x105c6d8); f_0105c7bf(cpu); if cpu.returned_to != 0x105c6d8 { return; }
  { let v: u16 = cpu.r16(0x11298c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11298c2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x105c754; continue; }
  pc = 0x105c6eb; continue;
 }
 0x105c6eb => {
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  let t = cpu.r32(0x10e4320u32); cpu.push(0x105c6f6); cpu.call(t); if cpu.returned_to != 0x105c6f6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11298a8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11298aau32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11298acu32, v); }
  cpu.push(0x105c719); f_0105c7bf(cpu); if cpu.returned_to != 0x105c719 { return; }
  if cpu.cf { pc = 0x105c754; continue; }
  pc = 0x105c71b; continue;
 }
 0x105c71b => {
  { let v = cpu.r16(0x11298c0u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x11298c2u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x11298aau32), 0x40u16); cpu.w16(0x11298aau32, v); }
  cpu.push(0x105c736); f_0105c7bf(cpu); if cpu.returned_to != 0x105c736 { return; }
  { let v: u16 = cpu.r16(0x11298c0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11298c2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x105c754; continue; }
  pc = 0x105c749; continue;
 }
 0x105c749 => {
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  let t = cpu.r32(0x10e4320u32); cpu.push(0x105c754); cpu.call(t); if cpu.returned_to != 0x105c754 { return; }
  pc = 0x105c754; continue;
 }
 0x105c754 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105c698; continue; }
  pc = 0x105c764; continue;
 }
 0x105c764 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105c691; continue; }
  pc = 0x105c770; continue;
 }
 0x105c770 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c771(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c771); }
 let mut pc: u32 = 0x105c771;
 loop { match pc {
 0x105c771 => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x98u16); }
  if cpu.zf { pc = 0x105c798; continue; }
  pc = 0x105c77d; continue;
 }
 0x105c77d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11298d0u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11298d0u32, v); }
  { let v: u16 = 0x98u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105c798); f_0105edc0(cpu); if cpu.returned_to != 0x105c798 { return; }
  pc = 0x105c798; continue;
 }
 0x105c798 => {
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x105c7be; continue; }
  pc = 0x105c7a4; continue;
 }
 0x105c7a4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x5cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16(cpu.r16(0x11298d2u32), (cpu.eax as u16)); cpu.w16(0x11298d2u32, v); }
  { let v: u16 = 0x5cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x105c7be); f_0105edc0(cpu); if cpu.returned_to != 0x105c7be { return; }
  pc = 0x105c7be; continue;
 }
 0x105c7be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c7bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c7bf); }
 let mut pc: u32 = 0x105c7bf;
 loop { match pc {
 0x105c7bf => {
  { let v: u32 = 0x11298a8u32; cpu.esi = v; }
  { let v: u32 = 0x11298c0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4038u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11298d8u32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x11298d0u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x112989cu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x11298d2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11298a0u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x11298d4u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11298a4u32, v); }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11298a0u32); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11298a0u32); cpu.eax = v; }
  { let v = cpu.r16(0x100u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11298a4u32); cpu.eax = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.edx, 10u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11298a0u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11298a4u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = 0x100u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x112989cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x105c87d; continue; }
  pc = 0x105c878; continue;
 }
 0x105c878 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x105c87d; continue;
 }
 0x105c87d => {
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x11298a0u32); cpu.eax = v; }
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
pub fn f_0105c89c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105c89c); }
 let mut pc: u32 = 0x105c89c;
 loop { match pc {
 0x105c89c => {
  cpu.push(0x105c8a1); f_0105caef(cpu); if cpu.returned_to != 0x105c8a1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = cpu.r16(0x10e3e9fu32) as u32; cpu.ecx = v; }
  pc = 0x105c8bc; continue;
 }
 0x105c8bc => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4070u32); cpu.push(0x105c8c3); cpu.call(t); if cpu.returned_to != 0x105c8c3 { return; }
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x7u8); }
  if cpu.zf { pc = 0x105c8fa; continue; }
  pc = 0x105c8f0; continue;
 }
 0x105c8f0 => {
  { cpu.sub8((cpu.ebx as u8), 0x33u8); }
  if cpu.zf { pc = 0x105c8fa; continue; }
  pc = 0x105c8f5; continue;
 }
 0x105c8f5 => {
  { cpu.sub8((cpu.ebx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x105c8ff; continue; }
  pc = 0x105c8fa; continue;
 }
 0x105c8fa => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105c8bc; continue; }
  pc = 0x105c8fd; continue;
 }
 0x105c8fd => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105c8ff => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105c902(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105c902); } r_0105c902(cpu, 0x105c902); }
pub fn f_0105c935(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105c935); } r_0105c902(cpu, 0x105c935); }
pub fn f_0105c9ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105c9ad); } r_0105c902(cpu, 0x105c9ad); }
pub fn f_0105ca22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105ca22); } r_0105c902(cpu, 0x105ca22); }
pub fn f_0105ca27(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105ca27); } r_0105c902(cpu, 0x105ca27); }
fn r_0105c902(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x105c902 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e91u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8fu32, v); }
  pc = 0x105ca5b; continue;
 }
 0x105c935 => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11a188cu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x10e3e87u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x105c95d); f_0105cdcc(cpu); if cpu.returned_to != 0x105c95d { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e8fu32, v); }
  cpu.push(0x105c989); f_0104f36d(cpu); if cpu.returned_to != 0x105c989 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x105c9aa; continue; }
  pc = 0x105c99e; continue;
 }
 0x105c99e => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e91u32, v); }
  pc = 0x105ca5b; continue;
 }
 0x105c9aa => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105c9ad => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1172f54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x10e3e87u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x105c9d5); f_0105cdcc(cpu); if cpu.returned_to != 0x105c9d5 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e8fu32, v); }
  cpu.push(0x105ca01); f_0104f36d(cpu); if cpu.returned_to != 0x105ca01 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x105ca1f; continue; }
  pc = 0x105ca16; continue;
 }
 0x105ca16 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e91u32, v); }
  pc = 0x105ca5b; continue;
 }
 0x105ca1f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ca22 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x105ca43; continue;
 }
 0x105ca27 => {
  { cpu.pushad(); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x105ca43; continue;
 }
 0x105ca43 => {
  { let v: u32 = 0x10e3e87u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x105ca4f); f_0105cdcc(cpu); if cpu.returned_to != 0x105ca4f { return; }
  { let v: u32 = 0x10e3e8du32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x105ca5b); f_0105cdcc(cpu); if cpu.returned_to != 0x105ca5b { return; }
  pc = 0x105ca5b; continue;
 }
 0x105ca5b => {
  cpu.push(0x105ca60); f_0105caef(cpu); if cpu.returned_to != 0x105ca60 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = cpu.r16(0x10e3e9fu32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x105ca82; continue; }
  pc = 0x105ca7f; continue;
 }
 0x105ca7f => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ca82 => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4070u32); cpu.push(0x105ca89); cpu.call(t); if cpu.returned_to != 0x105ca89 { return; }
  { let v: u32 = cpu.r16(0x10e3e87u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3e89u32) as u32; cpu.ebx = v; }
  cpu.push(0x105ca9c); f_0105cac4(cpu); if cpu.returned_to != 0x105ca9c { return; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1129904u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10e3e8bu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105cac0; continue; }
  pc = 0x105caba; continue;
 }
 0x105caba => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105ca82; continue; }
  pc = 0x105cabd; continue;
 }
 0x105cabd => {
  { cpu.cf = false; }
  pc = 0x105cac2; continue;
 }
 0x105cac0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  pc = 0x105cac2; continue;
 }
 0x105cac2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105cac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105cac4); }
 let mut pc: u32 = 0x105cac4;
 loop { match pc {
 0x105cac4 => {
  { cpu.sub8(cpu.r8(0x10ef356u32), 0x0u8); }
  if cpu.zf { pc = 0x105caee; continue; }
  pc = 0x105cacd; continue;
 }
 0x105cacd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4e6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x105caed); cpu.call(t); if cpu.returned_to != 0x105caed { return; }
  { cpu.popad(); }
  pc = 0x105caee; continue;
 }
 0x105caee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105caef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105caef); }
 let mut pc: u32 = 0x105caef;
 loop { match pc {
 0x105caef => {
  { cpu.sub16(cpu.r16(0x10e3e8bu32), 0x0u16); }
  if cpu.zf { pc = 0x105cb00; continue; }
  pc = 0x105caf9; continue;
 }
 0x105caf9 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e8bu32)); cpu.w16(0x10e3e8bu32, v); }
  pc = 0x105cb00; continue;
 }
 0x105cb00 => {
  { cpu.sub16(cpu.r16(0x10e3e91u32), 0x0u16); }
  if cpu.zf { pc = 0x105cb11; continue; }
  pc = 0x105cb0a; continue;
 }
 0x105cb0a => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e91u32)); cpu.w16(0x10e3e91u32, v); }
  pc = 0x105cb11; continue;
 }
 0x105cb11 => {
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e87u32), 4u32); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e89u32), 4u32); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e8du32), 4u32); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e8fu32), 4u32); cpu.w16(0x10e3e8fu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea1u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea3u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea5u32, v); }
  { let v: u16 = cpu.r16(0x10e3e8du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e91u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e87u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105cb92; continue; }
  pc = 0x105cb69; continue;
 }
 0x105cb69 => {
  { let a = cpu.r16(0x10e3e87u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x10e3e87u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let a = cpu.r16(0x10e3e89u32); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.w16(0x10e3e89u32, v); } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e8fu32, v); }
  { let a = cpu.r16(0x10e3e8bu32); let b = (cpu.ecx as u16); { let v: u16 = b; cpu.w16(0x10e3e8bu32, v); } { let v: u16 = a; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e91u32, v); }
  pc = 0x105cb92; continue;
 }
 0x105cb92 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e87u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e89u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cbae; continue; }
  pc = 0x105cba2; continue;
 }
 0x105cba2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ea3u32, v); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x105cbae; continue;
 }
 0x105cbae => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e3e8bu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cbc3; continue; }
  pc = 0x105cbb7; continue;
 }
 0x105cbb7 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ea5u32, v); }
  pc = 0x105cbc3; continue;
 }
 0x105cbc3 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105cbfb; continue; }
  pc = 0x105cbc8; continue;
 }
 0x105cbc8 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105cc2f; continue; }
  pc = 0x105cbcd; continue;
 }
 0x105cbcd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ea7u32, v); }
  { let v: u32 = 0x105cc5eu32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105cbfb => {
  { cpu.sub16((cpu.ecx as u16), (cpu.ebx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105cc2f; continue; }
  pc = 0x105cc00; continue;
 }
 0x105cc00 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ea9u32, v); }
  { let v: u32 = 0x105ccd8u32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105cc2f => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u32 = 0x105cd52u32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105cc5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105cc5e); }
 let mut pc: u32 = 0x105cc5e;
 loop { match pc {
 0x105cc5e => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea9u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eabu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e95u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cc9e; continue; }
  pc = 0x105cc90; continue;
 }
 0x105cc90 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x105cc9e; continue;
 }
 0x105cc9e => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105ccb5; continue; }
  pc = 0x105cca7; continue;
 }
 0x105cca7 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105ccb5; continue;
 }
 0x105ccb5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3eabu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ccd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ccd8); }
 let mut pc: u32 = 0x105ccd8;
 loop { match pc {
 0x105ccd8 => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea7u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eabu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cd18; continue; }
  pc = 0x105cd0a; continue;
 }
 0x105cd0a => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105cd18; continue;
 }
 0x105cd18 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cd2f; continue; }
  pc = 0x105cd21; continue;
 }
 0x105cd21 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105cd2f; continue;
 }
 0x105cd2f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3eabu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105cd52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105cd52); }
 let mut pc: u32 = 0x105cd52;
 loop { match pc {
 0x105cd52 => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea7u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea9u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cd92; continue; }
  pc = 0x105cd84; continue;
 }
 0x105cd84 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105cd92; continue;
 }
 0x105cd92 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105cda9; continue; }
  pc = 0x105cd9b; continue;
 }
 0x105cd9b => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x105cda9; continue;
 }
 0x105cda9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3ea9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105cdcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105cdcc); }
 let mut pc: u32 = 0x105cdcc;
 loop { match pc {
 0x105cdcc => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x105cdd6; continue; }
  pc = 0x105cdd4; continue;
 }
 0x105cdd4 => {
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x105cdd6; continue;
 }
 0x105cdd6 => {
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
  { let v: u32 = cpu.r8(0x10f714bu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ce0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ce0c); }
 let mut pc: u32 = 0x105ce0c;
 loop { match pc {
 0x105ce0c => {
  { cpu.pushad(); }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  { let v: u32 = 0x11e0180u32; cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105ce3f; continue; }
  pc = 0x105ce26; continue;
 }
 0x105ce26 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x105ce3f; continue; }
  pc = 0x105ce2a; continue;
 }
 0x105ce2a => {
  { let v: u32 = 0x11e0180u32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1129908u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x112990cu32, v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ce3f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ce42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ce42); }
 let mut pc: u32 = 0x105ce42;
 loop { match pc {
 0x105ce42 => {
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x105ce4c; continue; }
  pc = 0x105ce46; continue;
 }
 0x105ce46 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105ce4c; continue; }
  pc = 0x105ce4a; continue;
 }
 0x105ce4a => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105ce4c; continue;
 }
 0x105ce4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ce4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ce4d); }
 let mut pc: u32 = 0x105ce4d;
 loop { match pc {
 0x105ce4d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1129910u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x105ce7d); f_010018d7(cpu); if cpu.returned_to != 0x105ce7d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1129910u32, v); }
  { cpu.popad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x105d140; continue; }
  pc = 0x105ce98; continue;
 }
 0x105ce98 => {
  { let v: u32 = 0xfffffffeu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffcu32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4202u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105cebc; continue;
 }
 0x105cebc => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105ced7; continue;
 }
 0x105ced7 => {
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x32504d4au32); }
  if !cpu.zf { pc = 0x105d140; continue; }
  pc = 0x105cee4; continue;
 }
 0x105cee4 => {
  { let v: u32 = 0xfffffff8u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4202u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105cf01; continue;
 }
 0x105cf01 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105cf1c; continue;
 }
 0x105cf1c => {
  { let v: u32 = 0xfffffffdu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xffffffe8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x8u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4202u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105cf44; continue;
 }
 0x105cf44 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x105cf81; continue; }
  pc = 0x105cf4c; continue;
 }
 0x105cf4c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105cf63; continue;
 }
 0x105cf63 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x105cfdc; continue; }
  pc = 0x105cf67; continue;
 }
 0x105cf67 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x105cf63; continue;
 }
 0x105cf81 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x105cfa1; continue;
 }
 0x105cfa1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x105cfa9); f_0105ce42(cpu); if cpu.returned_to != 0x105cfa9 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x105cfb3); f_0105ce42(cpu); if cpu.returned_to != 0x105cfb3 { return; }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x105cfbd; continue; }
  pc = 0x105cfb7; continue;
 }
 0x105cfb7 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x105cfdc; continue; }
  pc = 0x105cfbb; continue;
 }
 0x105cfbb => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105cfa1; continue; }
  pc = 0x105cfbd; continue;
 }
 0x105cfbd => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x105cf81; continue; }
  pc = 0x105cfd7; continue;
 }
 0x105cfd7 => {
  pc = 0x105d140; continue;
 }
 0x105cfdc => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105cff7; continue;
 }
 0x105cff7 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3f00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105d012; continue;
 }
 0x105d012 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x105d021; continue; }
  pc = 0x105d018; continue;
 }
 0x105d018 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x105d11e; continue;
 }
 0x105d021 => {
  { let v: u32 = cpu.r16(0xffffffd0u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffd2u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = 0x4200u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.push(0x105d03b); f_0105ce0c(cpu); if cpu.returned_to != 0x105d03b { return; }
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x112990cu32)); }
  if cpu.cf { pc = 0x105d062; continue; }
  pc = 0x105d051; continue;
 }
 0x105d051 => {
  cpu.push(0x105d056); f_0105ce0c(cpu); if cpu.returned_to != 0x105d056 { return; }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105d05c; continue;
 }
 0x105d05c => {
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  pc = 0x105d062; continue;
 }
 0x105d062 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x11df180u32; cpu.ebp = v; }
  pc = 0x105d093; continue;
 }
 0x105d070 => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x105d093; continue; }
  pc = 0x105d074; continue;
 }
 0x105d074 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x112990cu32)); }
  if cpu.cf { pc = 0x105d08d; continue; }
  pc = 0x105d07c; continue;
 }
 0x105d07c => {
  cpu.push(0x105d081); f_0105ce0c(cpu); if cpu.returned_to != 0x105d081 { return; }
  if cpu.cf { pc = 0x105d139; continue; }
  pc = 0x105d087; continue;
 }
 0x105d087 => {
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  pc = 0x105d08d; continue;
 }
 0x105d08d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x105d093; continue;
 }
 0x105d093 => {
  if !cpu.cf { pc = 0x105d0c0; continue; }
  pc = 0x105d095; continue;
 }
 0x105d095 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x112990cu32)); }
  if cpu.cf { pc = 0x105d0ae; continue; }
  pc = 0x105d09d; continue;
 }
 0x105d09d => {
  cpu.push(0x105d0a2); f_0105ce0c(cpu); if cpu.returned_to != 0x105d0a2 { return; }
  if cpu.cf { pc = 0x105d138; continue; }
  pc = 0x105d0a8; continue;
 }
 0x105d0a8 => {
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  pc = 0x105d0ae; continue;
 }
 0x105d0ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x105d070; continue;
 }
 0x105d0c0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x112990cu32)); }
  if cpu.cf { pc = 0x105d0d5; continue; }
  pc = 0x105d0c8; continue;
 }
 0x105d0c8 => {
  cpu.push(0x105d0cd); f_0105ce0c(cpu); if cpu.returned_to != 0x105d0cd { return; }
  if cpu.cf { pc = 0x105d138; continue; }
  pc = 0x105d0cf; continue;
 }
 0x105d0cf => {
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  pc = 0x105d0d5; continue;
 }
 0x105d0d5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x112990cu32)); }
  if cpu.cf { pc = 0x105d0ed; continue; }
  pc = 0x105d0e0; continue;
 }
 0x105d0e0 => {
  cpu.push(0x105d0e5); f_0105ce0c(cpu); if cpu.returned_to != 0x105d0e5 { return; }
  if cpu.cf { pc = 0x105d138; continue; }
  pc = 0x105d0e7; continue;
 }
 0x105d0e7 => {
  { let v: u32 = cpu.r32(0x1129908u32); cpu.esi = v; }
  pc = 0x105d0ed; continue;
 }
 0x105d0ed => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x105d11e; continue; }
  pc = 0x105d0fa; continue;
 }
 0x105d0fa => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x105d100; continue;
 }
 0x105d100 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105d100; continue; }
  pc = 0x105d119; continue;
 }
 0x105d119 => {
  cpu.poll(); pc = 0x105d070; continue;
 }
 0x105d11e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3e00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d138 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x105d139; continue;
 }
 0x105d139 => {
  { let v: u32 = 0xfffffffcu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x105d140; continue;
 }
 0x105d140 => {
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x105d158; continue; }
  pc = 0x105d14b; continue;
 }
 0x105d14b => {
  { let v: u32 = 0x3e00u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1129910u32); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  pc = 0x105d158; continue;
 }
 0x105d158 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d15e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d15e); }
 let mut pc: u32 = 0x105d15e;
 loop { match pc {
 0x105d15e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffd0u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffeu32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x32504d4au32); }
  if !cpu.zf { pc = 0x105d286; continue; }
  pc = 0x105d18c; continue;
 }
 0x105d18c => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffdu32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, 0xffffffe8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x105d1bc; continue; }
  pc = 0x105d1af; continue;
 }
 0x105d1af => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  pc = 0x105d1b2; continue;
 }
 0x105d1b2 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x105d1f2; continue; }
  pc = 0x105d1b6; continue;
 }
 0x105d1b6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x18u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x105d1b2; continue;
 }
 0x105d1bc => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x105d1c9; continue;
 }
 0x105d1c9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x105d1d1); f_0105ce42(cpu); if cpu.returned_to != 0x105d1d1 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x105d1db); f_0105ce42(cpu); if cpu.returned_to != 0x105d1db { return; }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x105d1e5; continue; }
  pc = 0x105d1df; continue;
 }
 0x105d1df => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x105d1f2; continue; }
  pc = 0x105d1e3; continue;
 }
 0x105d1e3 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105d1c9; continue; }
  pc = 0x105d1e5; continue;
 }
 0x105d1e5 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x105d1bc; continue; }
  pc = 0x105d1ed; continue;
 }
 0x105d1ed => {
  pc = 0x105d286; continue;
 }
 0x105d1f2 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x105d206; continue; }
  pc = 0x105d200; continue;
 }
 0x105d200 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x105d271; continue;
 }
 0x105d206 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x11df180u32; cpu.ebp = v; }
  pc = 0x105d22c; continue;
 }
 0x105d222 => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x105d22c; continue; }
  pc = 0x105d226; continue;
 }
 0x105d226 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x105d22c; continue;
 }
 0x105d22c => {
  if !cpu.cf { pc = 0x105d240; continue; }
  pc = 0x105d22e; continue;
 }
 0x105d22e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x105d222; continue;
 }
 0x105d240 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x105d271; continue; }
  pc = 0x105d250; continue;
 }
 0x105d250 => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x105d256; continue;
 }
 0x105d256 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105d256; continue; }
  pc = 0x105d26f; continue;
 }
 0x105d26f => {
  cpu.poll(); pc = 0x105d222; continue;
 }
 0x105d271 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d286 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d28c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d28c); }
 let mut pc: u32 = 0x105d28c;
 loop { match pc {
 0x105d28c => {
  { let v: u32 = 0x117e804u32; cpu.edi = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d29b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d29b); }
 let mut pc: u32 = 0x105d29b;
 loop { match pc {
 0x105d29b => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x105d2d1); f_01018ffd(cpu); if cpu.returned_to != 0x105d2d1 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105d4cc; continue; }
  pc = 0x105d2d8; continue;
 }
 0x105d2d8 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x105d399; continue; }
  pc = 0x105d2e5; continue;
 }
 0x105d2e5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x105d31a); f_01018ffd(cpu); if cpu.returned_to != 0x105d31a { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105d4cc; continue; }
  pc = 0x105d321; continue;
 }
 0x105d321 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x105d356); f_01018ffd(cpu); if cpu.returned_to != 0x105d356 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105d4cc; continue; }
  pc = 0x105d35d; continue;
 }
 0x105d35d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x105d392); f_01018ffd(cpu); if cpu.returned_to != 0x105d392 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105d4cc; continue; }
  pc = 0x105d399; continue;
 }
 0x105d399 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x105d3a1); f_0105dae1(cpu); if cpu.returned_to != 0x105d3a1 { return; }
  if cpu.cf { pc = 0x105d4cc; continue; }
  pc = 0x105d3a7; continue;
 }
 0x105d3a7 => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x105d4cc; continue; }
  pc = 0x105d3af; continue;
 }
 0x105d3af => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x105d3d4; continue; }
  pc = 0x105d3bb; continue;
 }
 0x105d3bb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = 0x112994eu32; cpu.edi = v; }
  { let v: u32 = 0x4b0u32; cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  pc = 0x105d3e9; continue;
 }
 0x105d3d4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = 0x112994eu32; cpu.edi = v; }
  { let v: u32 = 0x4b0u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16) ^ (cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105d3e9; continue;
 }
 0x105d3e9 => {
  { cpu.sub8(cpu.r8(0x112994eu32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x105d483; continue; }
  pc = 0x105d3f6; continue;
 }
 0x105d3f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1129a3eu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1129a40u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x105d41d; continue; }
  pc = 0x105d417; continue;
 }
 0x105d417 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x105d41d; continue;
 }
 0x105d41d => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x10e3e0fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x105d482; continue; }
  pc = 0x105d431; continue;
 }
 0x105d431 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3e11u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x105d482; continue; }
  pc = 0x105d43a; continue;
 }
 0x105d43a => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.r32(0x10e4154u32); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10ef210u32), 0x1u8); }
  if !cpu.zf { pc = 0x105d469; continue; }
  pc = 0x105d44b; continue;
 }
 0x105d44b => {
  cpu.push(0x105d450); f_01018e6e(cpu); if cpu.returned_to != 0x105d450 { return; }
  if !cpu.cf { pc = 0x105d469; continue; }
  pc = 0x105d452; continue;
 }
 0x105d452 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x105d469; continue; }
  pc = 0x105d45e; continue;
 }
 0x105d45e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.popad(); }
  pc = 0x105d483; continue;
 }
 0x105d469 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  cpu.push(0x105d476); f_0105d4df(cpu); if cpu.returned_to != 0x105d476 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105d483; continue; }
  pc = 0x105d479; continue;
 }
 0x105d479 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112994eu32.wrapping_add(cpu.ecx), v); }
  pc = 0x105d4b1; continue;
 }
 0x105d482 => {
  { cpu.popad(); }
  pc = 0x105d483; continue;
 }
 0x105d483 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xf0u8); }
  if !cpu.zf { pc = 0x105d4a6; continue; }
  pc = 0x105d48a; continue;
 }
 0x105d48a => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x105d491); f_0105d4ce(cpu); if cpu.returned_to != 0x105d491 { return; }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if !cpu.zf { pc = 0x105d4a4; continue; }
  pc = 0x105d49d; continue;
 }
 0x105d49d => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  pc = 0x105d4a6; continue;
 }
 0x105d4a4 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x105d4a6; continue;
 }
 0x105d4a6 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x105d3e9; continue; }
  pc = 0x105d4ad; continue;
 }
 0x105d4ad => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x105d4cc; continue;
 }
 0x105d4b1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4140u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e0fu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x10e3e11u32), (cpu.ebx as u16)); cpu.w16(0x10e3e11u32, v); }
  pc = 0x105d4cc; continue;
 }
 0x105d4cc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d4ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d4ce); }
 let mut pc: u32 = 0x105d4ce;
 loop { match pc {
 0x105d4ce => {
  { cpu.pushad(); }
  { let v: u32 = 0x112994eu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d4df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d4df); }
 let mut pc: u32 = 0x105d4df;
 loop { match pc {
 0x105d4df => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105d55a; continue; }
  pc = 0x105d4ea; continue;
 }
 0x105d4ea => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3d19u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105d55a; continue; }
  pc = 0x105d4f3; continue;
 }
 0x105d4f3 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.sf { pc = 0x105d55a; continue; }
  pc = 0x105d4f8; continue;
 }
 0x105d4f8 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  if cpu.sf { pc = 0x105d55a; continue; }
  pc = 0x105d4fd; continue;
 }
 0x105d4fd => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x105d521; continue; }
  pc = 0x105d503; continue;
 }
 0x105d503 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.sf { pc = 0x105d55a; continue; }
  pc = 0x105d508; continue;
 }
 0x105d508 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x7u32); cpu.ebx = v; }
  if cpu.sf { pc = 0x105d55a; continue; }
  pc = 0x105d50d; continue;
 }
 0x105d50d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105d55a; continue; }
  pc = 0x105d516; continue;
 }
 0x105d516 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3d19u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105d55a; continue; }
  pc = 0x105d51f; continue;
 }
 0x105d51f => {
  pc = 0x105d552; continue;
 }
 0x105d521 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d769u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105d536); f_010191d1(cpu); if cpu.returned_to != 0x105d536 { return; }
  if !cpu.cf { pc = 0x105d551; continue; }
  pc = 0x105d538; continue;
 }
 0x105d538 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x1fu16); }
  if !cpu.zf { pc = 0x105d551; continue; }
  pc = 0x105d54e; continue;
 }
 0x105d54e => {
  { cpu.popad(); }
  pc = 0x105d55a; continue;
 }
 0x105d551 => {
  { cpu.popad(); }
  pc = 0x105d552; continue;
 }
 0x105d552 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x105d559); f_01058feb(cpu); if cpu.returned_to != 0x105d559 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d55a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d55e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d55e); }
 let mut pc: u32 = 0x105d55e;
 loop { match pc {
 0x105d55e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x105d5fa; continue; }
  pc = 0x105d575; continue;
 }
 0x105d575 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x105d57d); f_0105dae1(cpu); if cpu.returned_to != 0x105d57d { return; }
  if cpu.cf { pc = 0x105d5fa; continue; }
  pc = 0x105d57f; continue;
 }
 0x105d57f => {
  cpu.push(0x105d584); f_0105d4ce(cpu); if cpu.returned_to != 0x105d584 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x1u32); cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x105d589; continue;
 }
 0x105d589 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x105d5f1; continue; }
  pc = 0x105d58e; continue;
 }
 0x105d58e => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x105d5f1; continue; }
  pc = 0x105d5a1; continue;
 }
 0x105d5a1 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x10e4154u32)); }
  if cpu.zf { pc = 0x105d5f1; continue; }
  pc = 0x105d5b7; continue;
 }
 0x105d5b7 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x105d5f1; continue; }
  pc = 0x105d5c3; continue;
 }
 0x105d5c3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x105d5f1; continue; }
  pc = 0x105d5c9; continue;
 }
 0x105d5c9 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e4154u32); cpu.push(v); }
  cpu.push(0x105d5d5); f_0105d5fc(cpu); if cpu.returned_to != 0x105d5d5 { return; }
  if cpu.cf { pc = 0x105d5ea; continue; }
  pc = 0x105d5d7; continue;
 }
 0x105d5d7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  cpu.push(0x105d5e3); f_01036a75(cpu); if cpu.returned_to != 0x105d5e3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  pc = 0x105d5ea; continue;
 }
 0x105d5ea => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { cpu.popad(); }
  pc = 0x105d5f1; continue;
 }
 0x105d5f1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105d589; continue; }
  pc = 0x105d5fa; continue;
 }
 0x105d5fa => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d5fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d5fc); } r_0105d5fc(cpu, 0x105d5fc); }
pub fn f_0105d776(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d776); } r_0105d5fc(cpu, 0x105d776); }
pub fn f_0105d78a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d78a); } r_0105d5fc(cpu, 0x105d78a); }
pub fn f_0105d7ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d7ba); } r_0105d5fc(cpu, 0x105d7ba); }
pub fn f_0105d83e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d83e); } r_0105d5fc(cpu, 0x105d83e); }
pub fn f_0105d8b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105d8b7); } r_0105d5fc(cpu, 0x105d8b7); }
fn r_0105d5fc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x105d5fc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10ef210u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x105d78a; continue; }
  pc = 0x105d611; continue;
 }
 0x105d611 => {
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x105d630; continue; }
  pc = 0x105d61e; continue;
 }
 0x105d61e => {
  cpu.push(0x105d623); f_01068d2a(cpu); if cpu.returned_to != 0x105d623 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1105860u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x105d636; continue;
 }
 0x105d630 => {
  { let v: u8 = cpu.r8(0x1105839u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x105d636; continue;
 }
 0x105d636 => {
  { let v: u32 = 0x1129f10u32; cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x105d761; continue; }
  pc = 0x105d644; continue;
 }
 0x105d644 => {
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x105d76e; continue; }
  pc = 0x105d64d; continue;
 }
 0x105d64d => {
  { let v: u32 = 0x1129f1eu32; cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x10e3e0fu32), 0xd80u16); }
  if !cpu.zf { pc = 0x105d761; continue; }
  pc = 0x105d661; continue;
 }
 0x105d661 => {
  { let v: u32 = 0x1129f48u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x105d761; continue; }
  pc = 0x105d676; continue;
 }
 0x105d676 => {
  { let v: u32 = 0x1129f56u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x105d6b0; continue; }
  pc = 0x105d681; continue;
 }
 0x105d681 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105d694); f_0104f36d(cpu); if cpu.returned_to != 0x105d694 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x3bu8); }
  if cpu.zf { pc = 0x105d6aa; continue; }
  pc = 0x105d6a2; continue;
 }
 0x105d6a2 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x105d6aa; continue; }
  pc = 0x105d6a7; continue;
 }
 0x105d6a7 => {
  { cpu.popad(); }
  pc = 0x105d6b0; continue;
 }
 0x105d6aa => {
  { cpu.popad(); }
  pc = 0x105d761; continue;
 }
 0x105d6b0 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x105d734; continue; }
  pc = 0x105d6b5; continue;
 }
 0x105d6b5 => {
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x105d734; continue; }
  pc = 0x105d6c1; continue;
 }
 0x105d6c1 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e3e93u32); cpu.push(v); }
  { let v = cpu.r32(0x10e3e95u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4154u32); cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x105d6e6); f_0101ac6c(cpu); if cpu.returned_to != 0x105d6e6 { return; }
  if cpu.cf { pc = 0x105d71f; continue; }
  pc = 0x105d6e8; continue;
 }
 0x105d6e8 => {
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x105d701); f_01018e6e(cpu); if cpu.returned_to != 0x105d701 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  if cpu.cf { pc = 0x105d71f; continue; }
  pc = 0x105d70a; continue;
 }
 0x105d70a => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3e93u32, v); }
  { cpu.popad(); }
  pc = 0x105d734; continue;
 }
 0x105d71f => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3e93u32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x105d734; continue;
 }
 0x105d734 => {
  { cpu.sub8((cpu.ecx as u8), 0xdu8); }
  if !cpu.zf { pc = 0x105d752; continue; }
  pc = 0x105d739; continue;
 }
 0x105d739 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1105860u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { cpu.poll(); pc = 0x105d6c1; continue; }
  pc = 0x105d752; continue;
 }
 0x105d752 => {
  { let v: u32 = 0x1129f3au32; cpu.esi = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x0u8); }
  if cpu.zf { pc = 0x105d761; continue; }
  pc = 0x105d75c; continue;
 }
 0x105d75c => {
  { let v: u32 = 0x1129f2cu32; cpu.esi = v; }
  pc = 0x105d761; continue;
 }
 0x105d761 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x1129f64u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x105d76e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.div32(v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d776 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d78a => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xa3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xacbu16); }
  if cpu.zf { pc = 0x105d7af; continue; }
  pc = 0x105d7a3; continue;
 }
 0x105d7a3 => {
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x105d7af; continue; }
  pc = 0x105d7a9; continue;
 }
 0x105d7a9 => {
  { cpu.sub16((cpu.eax as u16), 0xac3u16); }
  if !cpu.zf { pc = 0x105d7b3; continue; }
  pc = 0x105d7af; continue;
 }
 0x105d7af => {
  { let v: u16 = 0xabbu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105d7b3; continue;
 }
 0x105d7b3 => {
  cpu.push(0x105d7b8); f_01059dbe(cpu); if cpu.returned_to != 0x105d7b8 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d7ba => {
  { let v: u8 = cpu.r8(0x10ef210u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x10e3e0fu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  cpu.push(0x105d80a); f_01036a75(cpu); if cpu.returned_to != 0x105d80a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x105d821); f_01059dbe(cpu); if cpu.returned_to != 0x105d821 { return; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e13u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d83e => {
  { let v: u8 = cpu.r8(0x10ef210u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  cpu.push(0x105d883); f_01036a75(cpu); if cpu.returned_to != 0x105d883 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x105d89a); f_01059dbe(cpu); if cpu.returned_to != 0x105d89a { return; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e13u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d8b7 => {
  { let v: u8 = cpu.r8(0x10ef210u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e0fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e13u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3e11u32) as u32; cpu.eax = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x105d92a; continue; }
  pc = 0x105d8e5; continue;
 }
 0x105d8e5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  cpu.push(0x105d913); f_01036a75(cpu); if cpu.returned_to != 0x105d913 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u32 = cpu.r32(0x10e4048u32); cpu.ebp = v; }
  { let v: u32 = 0xadfu32; cpu.eax = v; }
  cpu.push(0x105d92a); f_01059dbe(cpu); if cpu.returned_to != 0x105d92a { return; }
  pc = 0x105d92a; continue;
 }
 0x105d92a => {
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e13u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d947); }
 let mut pc: u32 = 0x105d947;
 loop { match pc {
 0x105d947 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x105d951; continue; }
  pc = 0x105d950; continue;
 }
 0x105d950 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d951 => {
  { cpu.sub8(cpu.r8(0x112994cu32), 0x0u8); }
  if !cpu.zf { pc = 0x105d965; continue; }
  pc = 0x105d95a; continue;
 }
 0x105d95a => {
  cpu.push(0x105d95f); f_010576b2(cpu); if cpu.returned_to != 0x105d95f { return; }
  cpu.push(0x105d964); f_0105d97d(cpu); if cpu.returned_to != 0x105d964 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d965 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d967); }
 let mut pc: u32 = 0x105d967;
 loop { match pc {
 0x105d967 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x105d97c; continue; }
  pc = 0x105d96d; continue;
 }
 0x105d96d => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x105d975); f_0105dae1(cpu); if cpu.returned_to != 0x105d975 { return; }
  if cpu.cf { pc = 0x105d97c; continue; }
  pc = 0x105d977; continue;
 }
 0x105d977 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x105d97c; continue;
 }
 0x105d97c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105d97d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105d97d); }
 let mut pc: u32 = 0x105d97d;
 loop { match pc {
 0x105d97d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x105da16; continue; }
  pc = 0x105d987; continue;
 }
 0x105d987 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x105da16; continue; }
  pc = 0x105d991; continue;
 }
 0x105d991 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  cpu.push(0x105d9af); f_0106a6aa(cpu); if cpu.returned_to != 0x105d9af { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x105da16; continue; }
  pc = 0x105d9b8; continue;
 }
 0x105d9b8 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x105d9c1); f_0105db17(cpu); if cpu.returned_to != 0x105d9c1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x105da1d; continue; }
  pc = 0x105d9c4; continue;
 }
 0x105d9c4 => {
  { cpu.sub8(cpu.r8(0x11296e6u32), 0x0u8); }
  if !cpu.zf { pc = 0x105d9ec; continue; }
  pc = 0x105d9cd; continue;
 }
 0x105d9cd => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x105d9ec; continue; }
  pc = 0x105d9e3; continue;
 }
 0x105d9e3 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105d9ec => {
  { cpu.sub8(cpu.r8(0x10ef370u32), 0xffu8); }
  if cpu.zf { pc = 0x105da1f; continue; }
  pc = 0x105d9f5; continue;
 }
 0x105d9f5 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf4u32; cpu.eax = v; }
  { let v = cpu.r8(0x10ef370u32); cpu.mul8(v); }
  { let v: u32 = 0x117e804u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x105da18; continue; }
  pc = 0x105da16; continue;
 }
 0x105da16 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105da18 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x105da1d; continue;
 }
 0x105da1d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105da1f => {
  { cpu.sub8(cpu.r8(0x11296e6u32), 0x0u8); }
  if !cpu.zf { pc = 0x105da41; continue; }
  pc = 0x105da28; continue;
 }
 0x105da28 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x105da41; continue; }
  pc = 0x105da31; continue;
 }
 0x105da31 => {
  { let v: u16 = cpu.r16(0x10e3d77u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3ce1u32)); }
  if cpu.zf { cpu.poll(); pc = 0x105da16; continue; }
  pc = 0x105da41; continue;
 }
 0x105da41 => {
  { let v: u32 = 0x117e804u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x105da48; continue;
 }
 0x105da48 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef370u32)); cpu.w8(0x10ef370u32, v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x105da5f; continue; }
  pc = 0x105da53; continue;
 }
 0x105da53 => {
  { let v: u32 = cpu.esi.wrapping_add(0xf4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105da48; continue; }
  pc = 0x105da5d; continue;
 }
 0x105da5d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105da5f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x105daaa; continue; }
  pc = 0x105da70; continue;
 }
 0x105da70 => {
  { cpu.sub8(cpu.r8(0x11296e6u32), 0x0u8); }
  if !cpu.zf { pc = 0x105daaa; continue; }
  pc = 0x105da79; continue;
 }
 0x105da79 => {
  { let v: u16 = cpu.r16(0x10e3d77u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105daaa; continue; }
  pc = 0x105da85; continue;
 }
 0x105da85 => {
  { let v = cpu.r16(0x10e3ce1u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce1u32, v); }
  cpu.push(0x105da98); f_010576b2(cpu); if cpu.returned_to != 0x105da98 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3ce1u32, v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x105daaa; continue; }
  pc = 0x105daa7; continue;
 }
 0x105daa7 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x105daaa; continue;
 }
 0x105daaa => {
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
pub fn f_0105dabc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dabc); }
 let mut pc: u32 = 0x105dabc;
 loop { match pc {
 0x105dabc => {
  cpu.push(0x105dac1); f_0105dae1(cpu); if cpu.returned_to != 0x105dac1 { return; }
  if cpu.cf { pc = 0x105dae0; continue; }
  pc = 0x105dac3; continue;
 }
 0x105dac3 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x105dacb; continue;
 }
 0x105dacb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x105dadc; continue; }
  pc = 0x105dad1; continue;
 }
 0x105dad1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x105dad9); f_0105db89(cpu); if cpu.returned_to != 0x105dad9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x105dadc; continue;
 }
 0x105dadc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x105dacb; continue; }
  pc = 0x105dae0; continue;
 }
 0x105dae0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105dae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dae1); }
 let mut pc: u32 = 0x105dae1;
 loop { match pc {
 0x105dae1 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x105db0b; continue; }
  pc = 0x105dae5; continue;
 }
 0x105dae5 => {
  { let v: u32 = 0x117e804u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x105daec; continue;
 }
 0x105daec => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x105db01; continue; }
  pc = 0x105daf1; continue;
 }
 0x105daf1 => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x105db0d; continue; }
  pc = 0x105dafa; continue;
 }
 0x105dafa => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x105db0d; continue; }
  pc = 0x105db01; continue;
 }
 0x105db01 => {
  { let v: u32 = cpu.esi.wrapping_add(0xf4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105daec; continue; }
  pc = 0x105db0b; continue;
 }
 0x105db0b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105db0d => {
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
pub fn f_0105db17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105db17); }
 let mut pc: u32 = 0x105db17;
 loop { match pc {
 0x105db17 => {
  cpu.push(0x105db1c); f_0105dae1(cpu); if cpu.returned_to != 0x105db1c { return; }
  if !cpu.cf { pc = 0x105db20; continue; }
  pc = 0x105db1e; continue;
 }
 0x105db1e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105db20 => {
  { cpu.sub8(cpu.r8(0x11296e6u32), 0x0u8); }
  if !cpu.zf { pc = 0x105db3f; continue; }
  pc = 0x105db29; continue;
 }
 0x105db29 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x105db43; continue; }
  pc = 0x105db3f; continue;
 }
 0x105db3f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105db43 => {
  cpu.push(0x105db48); f_0105db89(cpu); if cpu.returned_to != 0x105db48 { return; }
  if cpu.cf { pc = 0x105db7a; continue; }
  pc = 0x105db4a; continue;
 }
 0x105db4a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x105db7c; continue; }
  pc = 0x105db4f; continue;
 }
 0x105db4f => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x105db7a; continue; }
  pc = 0x105db62; continue;
 }
 0x105db62 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x112994cu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x105db72); f_01059200(cpu); if cpu.returned_to != 0x105db72 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x112994cu32, v); }
  pc = 0x105db7a; continue;
 }
 0x105db7a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105db7c => {
  cpu.push(0x105db81); f_01057945(cpu); if cpu.returned_to != 0x105db81 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105db89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105db89); }
 let mut pc: u32 = 0x105db89;
 loop { match pc {
 0x105db89 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x105dbe8; continue; }
  pc = 0x105db95; continue;
 }
 0x105db95 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x105dbb3; continue; }
  pc = 0x105db9d; continue;
 }
 0x105db9d => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  if cpu.zf { pc = 0x105dbb3; continue; }
  pc = 0x105dbae; continue;
 }
 0x105dbae => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x105dbb3; continue;
 }
 0x105dbb3 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x105dbbd; continue;
 }
 0x105dbbd => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x105dbc4; continue; }
  pc = 0x105dbc2; continue;
 }
 0x105dbc2 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x105dbc4; continue;
 }
 0x105dbc4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x105dbbd; continue; }
  pc = 0x105dbc9; continue;
 }
 0x105dbc9 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105dbe6; continue; }
  pc = 0x105dbd0; continue;
 }
 0x105dbd0 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x105dbd7; continue; }
  pc = 0x105dbd5; continue;
 }
 0x105dbd5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x105dbd7; continue;
 }
 0x105dbd7 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  pc = 0x105dbe6; continue;
 }
 0x105dbe6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105dbe8 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105dbec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dbec); }
 let mut pc: u32 = 0x105dbec;
 loop { match pc {
 0x105dbec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105dc26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dc26); }
 let mut pc: u32 = 0x105dc26;
 loop { match pc {
 0x105dc26 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c62fbu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c633au32, v); }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  cpu.push(0x105dc3f); f_010019bc(cpu); if cpu.returned_to != 0x105dc3f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c633au32, v); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { f_0105f11c(cpu); return; }
  pc = 0x105dc51; continue;
 }
 0x105dc51 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fbu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x11b519du32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11b51a1u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x105dc6c); f_010132ed(cpu); if cpu.returned_to != 0x105dc6c { return; }
  cpu.push(0x105dc71); f_010129b5(cpu); if cpu.returned_to != 0x105dc71 { return; }
  cpu.push(0x105dc76); f_0101302f(cpu); if cpu.returned_to != 0x105dc76 { return; }
  cpu.push(0x105dc7b); f_0101223e(cpu); if cpu.returned_to != 0x105dc7b { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x105dce0; continue; }
  pc = 0x105dc80; continue;
 }
 0x105dc80 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11b51a1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11b519du32); cpu.ecx = v; }
  cpu.push(0x105dc93); f_0100feb6(cpu); if cpu.returned_to != 0x105dc93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x105dce0; continue; }
  pc = 0x105dc97; continue;
 }
 0x105dc97 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd361u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  cpu.push(0x105dcaf); f_010385ad(cpu); if cpu.returned_to != 0x105dcaf { return; }
  cpu.push(0x105dcb4); f_01072257(cpu); if cpu.returned_to != 0x105dcb4 { return; }
  { let v: u8 = 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x10); }
  cpu.push(0x105dcbf); f_01072682(cpu); if cpu.returned_to != 0x105dcbf { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x3e8u32; cpu.edx = v; }
  cpu.push(0x105dcce); f_0100ff75(cpu); if cpu.returned_to != 0x105dcce { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c62fbu32, v); }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x105dcdb); cpu.call(t); if cpu.returned_to != 0x105dcdb { return; }
  cpu.push(0x105dce0); f_0105ec4c(cpu); if cpu.returned_to != 0x105dce0 { return; }
  pc = 0x105dce0; continue;
 }
 0x105dce0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c62fbu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105dd62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dd62); }
 let mut pc: u32 = 0x105dd62;
 loop { match pc {
 0x105dd62 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e4024u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x105dda1; continue; }
  pc = 0x105dd6c; continue;
 }
 0x105dd6c => {
  { let v: u32 = cpu.r32(0x11cc9a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x11cc99fu32), 0x0u8); }
  if cpu.zf { pc = 0x105dd87; continue; }
  pc = 0x105dd81; continue;
 }
 0x105dd81 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x105dd87); f_0105e3dc(cpu); if cpu.returned_to != 0x105dd87 { return; }
  pc = 0x105dd87; continue;
 }
 0x105dd87 => {
  { let v: u16 = cpu.r16(0x10e3ee7u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ee9u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.dec32(cpu.r32(0x11cc9b6u32)); cpu.w32(0x11cc9b6u32, v); }
  pc = 0x105dda1; continue;
 }
 0x105dda1 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e4024u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11cc99fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11cc9a0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ddbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ddbe); }
 let mut pc: u32 = 0x105ddbe;
 loop { match pc {
 0x105ddbe => {
  { let v: u32 = 0x0u32; cpu.w32(0x11c5b0au32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105ddf7; continue; }
  pc = 0x105ddd1; continue;
 }
 0x105ddd1 => {
  { cpu.sub32(cpu.r32(0x11c5b06u32), 0x0u32); }
  if !cpu.zf { pc = 0x105dde4; continue; }
  pc = 0x105ddda; continue;
 }
 0x105ddda => {
  { let v: u32 = 0xd9u32; cpu.w32(0x11c5b0au32, v); }
  pc = 0x105dde4; continue;
 }
 0x105dde4 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105ddf7; continue; }
  pc = 0x105dded; continue;
 }
 0x105dded => {
  { let v: u32 = cpu.r32(0x11c5b0au32).wrapping_add(0x3ceu32); cpu.w32(0x11c5b0au32, v); }
  pc = 0x105ddf7; continue;
 }
 0x105ddf7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x11c5a26u32, v); }
  cpu.push(0x105de05); f_01059c64(cpu); if cpu.returned_to != 0x105de05 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x10fd362u32, v); }
  { let v: u32 = 0x7cu32; cpu.w32(0x10fd366u32, v); }
  { let v: u32 = 0x2bcu32; cpu.w32(0x10fd2c8u32, v); }
  { let v: u32 = 0xa20u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x110576cu32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11c5b0au32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105de41; continue; }
  pc = 0x105de39; continue;
 }
 0x105de39 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x81u32); cpu.ebx = v; }
  pc = 0x105de42; continue;
 }
 0x105de41 => {
  {  }
  pc = 0x105de42; continue;
 }
 0x105de42 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  if cpu.sf { pc = 0x105de74; continue; }
  pc = 0x105de48; continue;
 }
 0x105de48 => {
  { cpu.sub32(cpu.eax, 0x2cau32); }
  if (cpu.sf != cpu.of) { pc = 0x105de62; continue; }
  pc = 0x105de4f; continue;
 }
 0x105de4f => {
  { let v: u32 = 0x586u32; cpu.w32(0x10fd2c8u32, v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cau32); cpu.eax = v; }
  if cpu.sf { pc = 0x105de74; continue; }
  pc = 0x105de60; continue;
 }
 0x105de60 => {
  pc = 0x105de68; continue;
 }
 0x105de62 => {
  { let v: u32 = cpu.add32(cpu.r32(0x10fd2c8u32), cpu.eax); cpu.w32(0x10fd2c8u32, v); }
  pc = 0x105de68; continue;
 }
 0x105de68 => {
  { let v: u32 = 0x5dcu32; cpu.w32(0x10e405cu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105de74 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105dfae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105dfae); }
 let mut pc: u32 = 0x105dfae;
 loop { match pc {
 0x105dfae => {
  cpu.push(0x105dfb3); f_0105e006(cpu); if cpu.returned_to != 0x105dfb3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4014u32, v); }
  { let v: u32 = cpu.r32(0x10e405cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 10u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4160u32, v); }
  { let v: u32 = cpu.r32(0x10fd2c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 10u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fd2c4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105dfe9; continue; }
  pc = 0x105dfe2; continue;
 }
 0x105dfe2 => {
  cpu.push(0x105dfe7); f_0105e09a(cpu); if cpu.returned_to != 0x105dfe7 { return; }
  pc = 0x105e000; continue;
 }
 0x105dfe9 => {
  { cpu.sub32(cpu.r32(0x11c5b06u32), 0x0u32); }
  if !cpu.zf { pc = 0x105dff9; continue; }
  pc = 0x105dff2; continue;
 }
 0x105dff2 => {
  cpu.push(0x105dff7); f_0105e1fe(cpu); if cpu.returned_to != 0x105dff7 { return; }
  pc = 0x105e000; continue;
 }
 0x105dff9 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x105dfff); f_0105e1fe(cpu); if cpu.returned_to != 0x105dfff { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x105e000; continue;
 }
 0x105e000 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4018u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e006(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e006); }
 let mut pc: u32 = 0x105e006;
 loop { match pc {
 0x105e006 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4098u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f94u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x61b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40a0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x9c50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f8cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4090u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c6334u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40a8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x3310u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4028u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4038u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4058u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x2710u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4068u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e406cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4074u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x640u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11e2408u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x3310u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e099; continue; }
  pc = 0x105e08f; continue;
 }
 0x105e08f => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4188u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000u32); cpu.eax = v; }
  pc = 0x105e099; continue;
 }
 0x105e099 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e09a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e09a); }
 let mut pc: u32 = 0x105e09a;
 loop { match pc {
 0x105e09a => {
  { let v: u32 = cpu.eax; cpu.w32(0x110f8b4u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e0ad; continue; }
  pc = 0x105e0a8; continue;
 }
 0x105e0a8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x429c0u32); cpu.eax = v; }
  pc = 0x105e0ad; continue;
 }
 0x105e0ad => {
  { let v: u32 = cpu.eax.wrapping_add(0x16340u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110f8ccu32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e0c5; continue; }
  pc = 0x105e0c0; continue;
 }
 0x105e0c0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x16340u32); cpu.eax = v; }
  pc = 0x105e0c5; continue;
 }
 0x105e0c5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1111388u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e0d8; continue; }
  pc = 0x105e0d3; continue;
 }
 0x105e0d3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x87c0u32); cpu.eax = v; }
  pc = 0x105e0d8; continue;
 }
 0x105e0d8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2d40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1111380u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e0f0; continue; }
  pc = 0x105e0eb; continue;
 }
 0x105e0eb => {
  { let v: u32 = cpu.eax.wrapping_add(0x2d40u32); cpu.eax = v; }
  pc = 0x105e0f0; continue;
 }
 0x105e0f0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f20u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e108; continue; }
  pc = 0x105e0fe; continue;
 }
 0x105e0fe => {
  { let v: u32 = cpu.eax.wrapping_add(0x32130u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1e10u32); cpu.eax = v; }
  pc = 0x105e108; continue;
 }
 0x105e108 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10b10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119f24u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e125; continue; }
  pc = 0x105e120; continue;
 }
 0x105e120 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e125; continue;
 }
 0x105e125 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f28u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e13d; continue; }
  pc = 0x105e138; continue;
 }
 0x105e138 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e13d; continue;
 }
 0x105e13d => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f2cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e155; continue; }
  pc = 0x105e150; continue;
 }
 0x105e150 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e155; continue;
 }
 0x105e155 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f30u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e16d; continue; }
  pc = 0x105e168; continue;
 }
 0x105e168 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e16d; continue;
 }
 0x105e16d => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f34u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e185; continue; }
  pc = 0x105e180; continue;
 }
 0x105e180 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e185; continue;
 }
 0x105e185 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f38u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e19d; continue; }
  pc = 0x105e198; continue;
 }
 0x105e198 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e19d; continue;
 }
 0x105e19d => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f3cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e1b5; continue; }
  pc = 0x105e1b0; continue;
 }
 0x105e1b0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e1b5; continue;
 }
 0x105e1b5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f40u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e1cd; continue; }
  pc = 0x105e1c8; continue;
 }
 0x105e1c8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e1cd; continue;
 }
 0x105e1cd => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119f44u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e1e5; continue; }
  pc = 0x105e1e0; continue;
 }
 0x105e1e0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x105e1e5; continue;
 }
 0x105e1e5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x110f8acu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x7800u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105e1fd; continue; }
  pc = 0x105e1f8; continue;
 }
 0x105e1f8 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x16800u32); cpu.eax = v; }
  pc = 0x105e1fd; continue;
 }
 0x105e1fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e1fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105e1fe); } r_0105e1fe(cpu, 0x105e1fe); }
pub fn f_0105e490(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105e490); } r_0105e1fe(cpu, 0x105e490); }
fn r_0105e1fe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x105e1fe => {
  { let v: u32 = 0x3625u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c6305u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x11c6305u32.wrapping_add(cpu.bases[3])), 4u32); cpu.w32(0x11c6305u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x105f160; continue; }
  pc = 0x105e21d; continue;
 }
 0x105e21d => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11c5a26u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6309u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105e22f); cpu.push(0x105e234); f_0105e09a(cpu); if cpu.returned_to != 0x105e234 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4178u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105e239); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105e490 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x10b1040u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10b0000u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x105f160; continue; }
  pc = 0x105e4ae; continue;
 }
 0x105e4ae => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ef3u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40f4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6319u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x10b1040u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10b0000u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11c6315u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x1010u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x10b1040u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10b0000u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x10e40f4u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x10b0000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4158u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105e501); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105f160 => {
  cpu.flat_ds(0x105f160); cpu.push(0x105f165); f_0105e3e6(cpu); if cpu.returned_to != 0x105f165 { return; }
  { let v: u32 = cpu.r32(0x11310d0u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { cpu.interrupts_off = true; }
  { let v: u32 = cpu.r32(0x10e4034u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { cpu.interrupts_off = false; }
  cpu.flat_ds(0x105f177); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e23a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e23a); }
 let mut pc: u32 = 0x105e23a;
 loop { match pc {
 0x105e23a => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x11c5a26u32) as u32; cpu.edx = v; }
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x105e252; continue; }
  pc = 0x105e24c; continue;
 }
 0x105e24c => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x105e252; continue; }
  pc = 0x105e250; continue;
 }
 0x105e250 => {
  { cpu.interrupt(0x31); }
  pc = 0x105e252; continue;
 }
 0x105e252 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e290); }
 let mut pc: u32 = 0x105e290;
 loop { match pc {
 0x105e290 => {
  { let v: u16 = cpu.r16(0x11c631du32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11c631fu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.esi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x105e2a9; continue; }
  pc = 0x105e2a3; continue;
 }
 0x105e2a3 => {
  { let v: u16 = 0x502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x105e2a9; continue;
 }
 0x105e2a9 => {
  { let v: u16 = cpu.r16(0x10e3eedu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eebu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.push(0x105e2c2); f_0105e23a(cpu); if cpu.returned_to != 0x105e2c2 { return; }
  { cpu.sub8(cpu.r8(0x10ef33fu32), 0x0u8); }
  if cpu.zf { pc = 0x105e2cb; continue; }
  pc = 0x105e2cb; continue;
 }
 0x105e2cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e2cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e2cc); }
 let mut pc: u32 = 0x105e2cc;
 loop { match pc {
 0x105e2cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e319); }
 let mut pc: u32 = 0x105e319;
 loop { match pc {
 0x105e319 => {
  cpu.push(0x105e31e); f_0105e510(cpu); if cpu.returned_to != 0x105e31e { return; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5a06u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef341u32, v); }
  cpu.push(0x105e333); f_0105de76(cpu); if cpu.returned_to != 0x105e333 { return; }
  cpu.push(0x105e338); f_0105e490(cpu); if cpu.returned_to != 0x105e338 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x105e34b); f_0105ec4c(cpu); if cpu.returned_to != 0x105e34b { return; }
  cpu.push(0x105e350); f_0105e511(cpu); if cpu.returned_to != 0x105e350 { return; }
  cpu.push(0x105e355); f_01033b46(cpu); if cpu.returned_to != 0x105e355 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x112a1e1u32, v); }
  { let v: u32 = cpu.r32(0x11c6309u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6305u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x105e372; continue; }
  pc = 0x105e36d; continue;
 }
 0x105e36d => {
  cpu.push(0x105e372); f_0105e3c7(cpu); if cpu.returned_to != 0x105e372 { return; }
  pc = 0x105e372; continue;
 }
 0x105e372 => {
  { let v: u32 = cpu.r32(0x11c6311u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c630du32); cpu.ecx = v; }
  cpu.push(0x105e383); f_0105e3c7(cpu); if cpu.returned_to != 0x105e383 { return; }
  { let v: u32 = cpu.r32(0x11c6319u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6315u32); cpu.ecx = v; }
  cpu.push(0x105e394); f_0105e3c7(cpu); if cpu.returned_to != 0x105e394 { return; }
  { let v: u32 = 0x100001fu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4034u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  if cpu.sf { pc = 0x105e3b7; continue; }
  pc = 0x105e3a3; continue;
 }
 0x105e3a3 => {
  { let v: u16 = cpu.r16(0x105564cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3f01u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x105e3b7; continue; }
  pc = 0x105e3b2; continue;
 }
 0x105e3b2 => {
  cpu.push(0x105e3b7); f_0105e3c7(cpu); if cpu.returned_to != 0x105e3b7 { return; }
  pc = 0x105e3b7; continue;
 }
 0x105e3b7 => {
  { let v: u32 = 0xa0000u32; cpu.edi = v; }
  { let v: u32 = 0x10000u32; cpu.ecx = v; }
  cpu.push(0x105e3c6); f_0105e3c7(cpu); if cpu.returned_to != 0x105e3c6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e3c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e3c7); }
 let mut pc: u32 = 0x105e3c7;
 loop { match pc {
 0x105e3c7 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x105e3d0); f_01013ba6(cpu); if cpu.returned_to != 0x105e3d0 { return; }
  { let v: u32 = !cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x11cc9beu32), cpu.eax); cpu.w32(0x11cc9beu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e3dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e3dc); }
 let mut pc: u32 = 0x105e3dc;
 loop { match pc {
 0x105e3dc => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x105e3e5); f_01013bd1(cpu); if cpu.returned_to != 0x105e3e5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e3e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e3e6); }
 let mut pc: u32 = 0x105e3e6;
 loop { match pc {
 0x105e3e6 => {
  { cpu.sub8(cpu.r8(0x112a1e0u32), 0x0u8); }
  if !cpu.zf { pc = 0x105e48f; continue; }
  pc = 0x105e3f3; continue;
 }
 0x105e3f3 => {
  { cpu.sub8(cpu.r8(0x112a1e1u32), 0x0u8); }
  if cpu.zf { pc = 0x105e48f; continue; }
  pc = 0x105e400; continue;
 }
 0x105e400 => {
  cpu.push(0x105e405); f_010129b5(cpu); if cpu.returned_to != 0x105e405 { return; }
  { let v: u32 = cpu.r32(0x11c6309u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6305u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x105e41b; continue; }
  pc = 0x105e416; continue;
 }
 0x105e416 => {
  cpu.push(0x105e41b); f_0105e3dc(cpu); if cpu.returned_to != 0x105e41b { return; }
  pc = 0x105e41b; continue;
 }
 0x105e41b => {
  { let v: u32 = cpu.r32(0x11c6311u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c630du32); cpu.ecx = v; }
  cpu.push(0x105e42c); f_0105e3dc(cpu); if cpu.returned_to != 0x105e42c { return; }
  { let v: u32 = cpu.r32(0x11c6319u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6315u32); cpu.ecx = v; }
  cpu.push(0x105e43d); f_0105e3dc(cpu); if cpu.returned_to != 0x105e43d { return; }
  { let v: u32 = 0x100001fu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4034u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  if cpu.sf { pc = 0x105e460; continue; }
  pc = 0x105e44c; continue;
 }
 0x105e44c => {
  { let v: u16 = cpu.r16(0x105564cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3f01u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x105e460; continue; }
  pc = 0x105e45b; continue;
 }
 0x105e45b => {
  cpu.push(0x105e460); f_0105e3dc(cpu); if cpu.returned_to != 0x105e460 { return; }
  pc = 0x105e460; continue;
 }
 0x105e460 => {
  cpu.push(0x105e465); f_01033b21(cpu); if cpu.returned_to != 0x105e465 { return; }
  cpu.push(0x105e46a); f_0105e57d(cpu); if cpu.returned_to != 0x105e46a { return; }
  cpu.push(0x105e46f); f_0105edac(cpu); if cpu.returned_to != 0x105e46f { return; }
  cpu.push(0x105e474); f_01072435(cpu); if cpu.returned_to != 0x105e474 { return; }
  cpu.push(0x105e479); f_010135ef(cpu); if cpu.returned_to != 0x105e479 { return; }
  cpu.push(0x105e47e); f_0105e290(cpu); if cpu.returned_to != 0x105e47e { return; }
  cpu.push(0x105e483); f_01037e00(cpu); if cpu.returned_to != 0x105e483 { return; }
  cpu.push(0x105e488); f_0105e502(cpu); if cpu.returned_to != 0x105e488 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x112a1e0u32, v); }
  pc = 0x105e48f; continue;
 }
 0x105e48f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e502(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e502); }
 let mut pc: u32 = 0x105e502;
 loop { match pc {
 0x105e502 => {
  { let v: u16 = cpu.r16(0x10e3ef3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e510); }
 let mut pc: u32 = 0x105e510;
 loop { match pc {
 0x105e510 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e511(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e511); }
 let mut pc: u32 = 0x105e511;
 loop { match pc {
 0x105e511 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x16); }
  { cpu.and8((cpu.eax as u8), 0xfu8); }
  if !cpu.zf { cpu.poll(); pc = 0x105e511; continue; }
  pc = 0x105e519; continue;
 }
 0x105e519 => {
  { cpu.sub16(cpu.r16(0x11c5a0eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x105e57c; continue; }
  pc = 0x105e523; continue;
 }
 0x105e523 => {
  { let v: u16 = 0x3509u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x11c5a08u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c59f4u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { let v: u8 = 0x9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11c5a0eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11c5a0cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x105e5c7u32; cpu.edx = v; }
  { let v: u32 = 0x2509u32; cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { let v: u8 = 0x9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e40f4u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x10b0000u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10b0000u32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  pc = 0x105e57c; continue;
 }
 0x105e57c => {
  cpu.flat_ds(0x105e57c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e57d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e57d); }
 let mut pc: u32 = 0x105e57d;
 loop { match pc {
 0x105e57d => {
  { cpu.sub16(cpu.r16(0x11c5a0eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x105e5c6; continue; }
  pc = 0x105e587; continue;
 }
 0x105e587 => {
  { let v: u32 = 0x2509u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c59f4u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x11c5a08u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { let v: u8 = 0x9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0x11c5a0eu32.wrapping_add(cpu.bases[3])) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x11c5a0cu32.wrapping_add(cpu.bases[3])) as u32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u16 = 0x0u16; cpu.w16(0x11c5a0cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11c5a0eu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e5c6; continue;
 }
 0x105e5c6 => {
  cpu.flat_ds(0x105e5c6); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e5c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e5c7); }
 let mut pc: u32 = 0x105e5c7;
 loop { match pc {
 0x105e5c7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.df = false; }
  { cpu.interrupts_off = true; }
  { let v: u8 = cpu.port_in(0x60u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xe0u8); }
  if cpu.zf { pc = 0x105e6d9; continue; }
  pc = 0x105e5da; continue;
 }
 0x105e5da => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x1du8); }
  if !cpu.zf { pc = 0x105e5f2; continue; }
  pc = 0x105e5eb; continue;
 }
 0x105e5eb => {
  { let v: u8 = 0x1u8; cpu.w8(0x10b00ebu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e5f2; continue;
 }
 0x105e5f2 => {
  { cpu.sub8((cpu.eax as u8), 0x9du8); }
  if !cpu.zf { pc = 0x105e5fd; continue; }
  pc = 0x105e5f6; continue;
 }
 0x105e5f6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10b00ebu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e5fd; continue;
 }
 0x105e5fd => {
  { cpu.sub8((cpu.eax as u8), 0x38u8); }
  if !cpu.zf { pc = 0x105e608; continue; }
  pc = 0x105e601; continue;
 }
 0x105e601 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10b00eau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e608; continue;
 }
 0x105e608 => {
  { cpu.sub8((cpu.eax as u8), 0xb8u8); }
  if !cpu.zf { pc = 0x105e613; continue; }
  pc = 0x105e60c; continue;
 }
 0x105e60c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10b00eau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e613; continue;
 }
 0x105e613 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x105e61e; continue; }
  pc = 0x105e617; continue;
 }
 0x105e617 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10b00ecu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e61e; continue;
 }
 0x105e61e => {
  { cpu.sub8((cpu.eax as u8), 0xaau8); }
  if !cpu.zf { pc = 0x105e629; continue; }
  pc = 0x105e622; continue;
 }
 0x105e622 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10b00ecu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e629; continue;
 }
 0x105e629 => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if !cpu.zf { pc = 0x105e634; continue; }
  pc = 0x105e62d; continue;
 }
 0x105e62d => {
  { let v: u8 = 0x1u8; cpu.w8(0x10b00edu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e634; continue;
 }
 0x105e634 => {
  { cpu.sub8((cpu.eax as u8), 0xb6u8); }
  if !cpu.zf { pc = 0x105e63f; continue; }
  pc = 0x105e638; continue;
 }
 0x105e638 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10b00edu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e63f; continue;
 }
 0x105e63f => {
  { let v: u8 = cpu.r8(0x10b00edu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e4158u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10b0c3eu32.wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10b00eeu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x105e692; continue; }
  pc = 0x105e661; continue;
 }
 0x105e661 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10b00f2u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10b01a4u32); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x105e692; continue; }
  pc = 0x105e67b; continue;
 }
 0x105e67b => {
  { cpu.lods(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x105e692; continue; }
  pc = 0x105e684; continue;
 }
 0x105e684 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b00f0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e699; continue;
 }
 0x105e692 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10b0c3eu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e699; continue;
 }
 0x105e699 => {
  { cpu.sub8(cpu.r8(0x10b00ebu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x105e6d9; continue; }
  pc = 0x105e6a2; continue;
 }
 0x105e6a2 => {
  { cpu.sub8(cpu.r8(0x10fd40eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e6d9; continue; }
  pc = 0x105e6ab; continue;
 }
 0x105e6ab => {
  { cpu.sub8(cpu.r8(0x10b00f0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1bu8); }
  if !cpu.zf { pc = 0x105e6c7; continue; }
  pc = 0x105e6b4; continue;
 }
 0x105e6b4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd41bu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105e6bb); cpu.push(0x105e6c0); f_0105e6f5(cpu); if cpu.returned_to != 0x105e6c0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b00f0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e6c7; continue;
 }
 0x105e6c7 => {
  { cpu.sub8(cpu.r8(0x10b00f1u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x12u8); }
  if !cpu.zf { pc = 0x105e6d9; continue; }
  pc = 0x105e6d0; continue;
 }
 0x105e6d0 => {
  { cpu.sub8(cpu.r8(0x10c6871u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e6d9; continue; }
  pc = 0x105e6d9; continue;
 }
 0x105e6d9 => {
  cpu.flat_ds(0x105e6d9); cpu.push(0x105e6de); f_0105e6e6(cpu); if cpu.returned_to != 0x105e6de { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x105e6e5); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e6e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e6e6); }
 let mut pc: u32 = 0x105e6e6;
 loop { match pc {
 0x105e6e6 => {
  { let v: u8 = cpu.port_in(0x61u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x61u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x61u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e6f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e6f5); }
 let mut pc: u32 = 0x105e6f5;
 loop { match pc {
 0x105e6f5 => {
  { let v: u8 = cpu.port_in(0x61u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x61u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x61u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  {  }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x7eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e70b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e70b); }
 let mut pc: u32 = 0x105e70b;
 loop { match pc {
 0x105e70b => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.pushad(); }
  cpu.push(0x105e712); f_010018d7(cpu); if cpu.returned_to != 0x105e712 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x105e728; continue;
 }
 0x105e728 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x10e3d8fu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x105e738); f_01001c0f(cpu); if cpu.returned_to != 0x105e738 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3ff0u32, v); }
  cpu.push(0x105e742); f_0105e77c(cpu); if cpu.returned_to != 0x105e742 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e746(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e746); }
 let mut pc: u32 = 0x105e746;
 loop { match pc {
 0x105e746 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3d8fu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x105e757); f_01001dd6(cpu); if cpu.returned_to != 0x105e757 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3ff0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e75e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e75e); }
 let mut pc: u32 = 0x105e75e;
 loop { match pc {
 0x105e75e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3d91u32) as u32; cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5afeu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e771(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e771); }
 let mut pc: u32 = 0x105e771;
 loop { match pc {
 0x105e771 => {
  cpu.push(0x105e776); f_0105e77c(cpu); if cpu.returned_to != 0x105e776 { return; }
  cpu.push(0x105e77b); f_0105e798(cpu); if cpu.returned_to != 0x105e77b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e77c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e77c); }
 let mut pc: u32 = 0x105e77c;
 loop { match pc {
 0x105e77c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3d8fu32) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105e796; continue; }
  pc = 0x105e789; continue;
 }
 0x105e789 => {
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d8fu32, v); }
  pc = 0x105e796; continue;
 }
 0x105e796 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e798); }
 let mut pc: u32 = 0x105e798;
 loop { match pc {
 0x105e798 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d91u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105e7b2; continue; }
  pc = 0x105e7a5; continue;
 }
 0x105e7a5 => {
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d91u32, v); }
  pc = 0x105e7b2; continue;
 }
 0x105e7b2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e7b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e7b6); }
 let mut pc: u32 = 0x105e7b6;
 loop { match pc {
 0x105e7b6 => {
  { let v: u32 = cpu.r32(0x10e3ffcu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x105e7f4; continue; }
  pc = 0x105e7bf; continue;
 }
 0x105e7bf => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e3ffcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x11c5a28u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x105e980u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x2508u32; cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x4295u32; cpu.ecx = v; }
  cpu.flat_ds(0x105e7ef); cpu.push(0x105e7f4); f_0105e821(cpu); if cpu.returned_to != 0x105e7f4 { return; }
  pc = 0x105e7f4; continue;
 }
 0x105e7f4 => {
  cpu.flat_ds(0x105e7f4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e7f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e7f5); }
 let mut pc: u32 = 0x105e7f5;
 loop { match pc {
 0x105e7f5 => {
  { let v: u32 = cpu.r32(0x10e3ffcu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x105e816; continue; }
  pc = 0x105e7ff; continue;
 }
 0x105e7ff => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x11c5a28u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2508u32; cpu.eax = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.flat_ds(0x105e811); cpu.push(0x105e816); f_0105e821(cpu); if cpu.returned_to != 0x105e816 { return; }
  pc = 0x105e816; continue;
 }
 0x105e816 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3ffcu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105e820); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e821(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e821); }
 let mut pc: u32 = 0x105e821;
 loop { match pc {
 0x105e821 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3d99u32, v); }
  { let v: u8 = 0x36u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x105e82e; continue;
 }
 0x105e82e => {
  pc = 0x105e830; continue;
 }
 0x105e830 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x105e836; continue;
 }
 0x105e836 => {
  pc = 0x105e838; continue;
 }
 0x105e838 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e83d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e83d); }
 let mut pc: u32 = 0x105e83d;
 loop { match pc {
 0x105e83d => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fd4c6u32, v); }
  pc = 0x105e842; continue;
 }
 0x105e842 => {
  { cpu.sub32(cpu.r32(0x10fd4c6u32), 0x0u32); }
  if !cpu.sf { cpu.poll(); pc = 0x105e842; continue; }
  pc = 0x105e84b; continue;
 }
 0x105e84b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e84c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e84c); }
 let mut pc: u32 = 0x105e84c;
 loop { match pc {
 0x105e84c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x112a5e2u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.df = false; }
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4cbu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x11c5f4cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e880; continue; }
  pc = 0x105e872; continue;
 }
 0x105e872 => {
  { cpu.sub8(cpu.r8(0x10fd41bu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x105e880; continue; }
  pc = 0x105e87b; continue;
 }
 0x105e87b => {
  cpu.flat_ds(0x105e87b); cpu.push(0x105e880); f_01040a19(cpu); if cpu.returned_to != 0x105e880 { return; }
  pc = 0x105e880; continue;
 }
 0x105e880 => {
  let t = cpu.r32(0x10e4338u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x105e880); cpu.push(0x105e886); cpu.call(t); if cpu.returned_to != 0x105e886 { return; }
  { cpu.sub8(cpu.r8(0x105593eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e8a6; continue; }
  pc = 0x105e88f; continue;
 }
 0x105e88f => {
  { cpu.sub8(cpu.r8(0x10ef352u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e8a6; continue; }
  pc = 0x105e898; continue;
 }
 0x105e898 => {
  { cpu.sub8(cpu.r8(0x1105668u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x105e8a6; continue; }
  pc = 0x105e8a1; continue;
 }
 0x105e8a1 => {
  cpu.flat_ds(0x105e8a1); cpu.push(0x105e8a6); f_010707d8(cpu); if cpu.returned_to != 0x105e8a6 { return; }
  pc = 0x105e8a6; continue;
 }
 0x105e8a6 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x10fd4c6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10fd4c6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee96au32.wrapping_add(cpu.bases[3]))); cpu.w8(0x10ee96au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1119db3u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x1119db3u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1119df4u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x1119df4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10e3f13u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10e3f13u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10fcbfau32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10fcbfau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11c59f0u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x11c59f0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x1105667u32.wrapping_add(cpu.bases[3]))); cpu.w8(0x1105667u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x1105667u32.wrapping_add(cpu.bases[3])) & 0x1fu8; cpu.w8(0x1105667u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x11e242cu32.wrapping_add(cpu.bases[3]))); cpu.w16(0x11e242cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x11e242eu32.wrapping_add(cpu.bases[3]))); cpu.w16(0x11e242eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x11e2430u32.wrapping_add(cpu.bases[3]))); cpu.w16(0x11e2430u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x11e2432u32.wrapping_add(cpu.bases[3]))); cpu.w16(0x11e2432u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x10fcc4fu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x105e908; continue; }
  pc = 0x105e902; continue;
 }
 0x105e902 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x10fcc4fu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10fcc4fu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e908; continue;
 }
 0x105e908 => {
  { cpu.sub8(cpu.r8(0x11c5b05u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x105e93d; continue; }
  pc = 0x105e911; continue;
 }
 0x105e911 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x11c5b03u32.wrapping_add(cpu.bases[3]))); cpu.w16(0x11c5b03u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub16(cpu.r16(0x11c5b03u32.wrapping_add(cpu.bases[3])), 0x46u16); }
  if !cpu.zf { pc = 0x105e93d; continue; }
  pc = 0x105e922; continue;
 }
 0x105e922 => {
  { cpu.sub32(cpu.r32(0x10c6a5cu32.wrapping_add(cpu.bases[3])), 0x8c9fu32); }
  if (cpu.sf == cpu.of) { pc = 0x105e934; continue; }
  pc = 0x105e92e; continue;
 }
 0x105e92e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10c6a5cu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10c6a5cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e934; continue;
 }
 0x105e934 => {
  { let v: u16 = 0x0u16; cpu.w16(0x11c5b03u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e93d; continue;
 }
 0x105e93d => {
  { cpu.sub16(cpu.r16(0x10e3efdu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x105e950; continue; }
  pc = 0x105e947; continue;
 }
 0x105e947 => {
  { cpu.sub8(cpu.r8(0x10ef346u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x105e974; continue; }
  pc = 0x105e950; continue;
 }
 0x105e950 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef345u32.wrapping_add(cpu.bases[3]))); cpu.w8(0x10ef345u32.wrapping_add(cpu.bases[3]), v); }
  if !cpu.zf { pc = 0x105e96b; continue; }
  pc = 0x105e958; continue;
 }
 0x105e958 => {
  { let v: u8 = cpu.r8(0x11c5a2cu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8(cpu.r8(0x11c5a2cu32.wrapping_add(cpu.bases[3])), 0x7u8); cpu.w8(0x11c5a2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef345u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e974; continue;
 }
 0x105e96b => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef344u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x105e974; continue;
 }
 0x105e974 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd4cbu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.interrupts_off = false; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x105e97f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105e980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105e980); }
 let mut pc: u32 = 0x105e980;
 loop { match pc {
 0x105e980 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee96au32.wrapping_add(cpu.bases[3]))); cpu.w8(0x10ee96au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10e3f13u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x10e3f13u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.interrupts_off = true; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5a04u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c59f8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10e3f01u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x10e4034u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1800u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { cpu.interrupts_off = false; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3d95u32.wrapping_add(cpu.bases[3]))); cpu.w16(0x10e3d95u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10e3d99u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x105e9db; continue; }
  pc = 0x105e9d2; continue;
 }
 0x105e9d2 => {
  { let v: u16 = cpu.add16(cpu.r16(0x10e3d97u32.wrapping_add(cpu.bases[3])), (cpu.eax as u16)); cpu.w16(0x10e3d97u32.wrapping_add(cpu.bases[3]), v); }
  if !cpu.cf { pc = 0x105ea12; continue; }
  pc = 0x105e9db; continue;
 }
 0x105e9db => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3d9bu32.wrapping_add(cpu.bases[3]))); cpu.w16(0x10e3d9bu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.interrupts_off = true; }
  { let v: u16 = cpu.r16(0x11c5a04u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x11c59f8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.r32(0x10e3ffcu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x11c5a28u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let a = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x105ea12 => {
  { cpu.interrupts_off = true; }
  { let v: u16 = cpu.r16(0x11c5a04u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x11c59f8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { cpu.interrupts_off = false; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x105ea2b); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ea2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ea2c); }
 let mut pc: u32 = 0x105ea2c;
 loop { match pc {
 0x105ea2c => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10c6877u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea36; continue;
 }
 0x105ea36 => {
  { cpu.sub8(cpu.r8(0x10fd430u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea3f; continue;
 }
 0x105ea3f => {
  { cpu.sub8(cpu.r8(0x10fd431u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea48; continue;
 }
 0x105ea48 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if !cpu.zf { pc = 0x105ea56; continue; }
  pc = 0x105ea56; continue;
 }
 0x105ea56 => {
  { cpu.sub8(cpu.r8(0x10b00ebu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea5f; continue;
 }
 0x105ea5f => {
  { cpu.sub8(cpu.r8(0x10b00f0u32.wrapping_add(cpu.eax)), 0x77u8); }
  if cpu.zf { pc = 0x105eaab; continue; }
  pc = 0x105ea68; continue;
 }
 0x105ea68 => {
  { cpu.sub8(cpu.r8(0x10b00f0u32.wrapping_add(cpu.eax)), 0x68u8); }
  if !cpu.zf { pc = 0x105ea83; continue; }
  pc = 0x105ea71; continue;
 }
 0x105ea71 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea7a; continue;
 }
 0x105ea7a => {
  { let v: u8 = 0x1u8; cpu.w8(0x10c6877u32, v); }
  pc = 0x105eaa8; continue;
 }
 0x105ea83 => {
  { cpu.sub8(cpu.r8(0x11c5f4cu32), 0x0u8); }
  if cpu.zf { pc = 0x105ea95; continue; }
  pc = 0x105ea8c; continue;
 }
 0x105ea8c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x105ea9e; continue; }
  pc = 0x105ea95; continue;
 }
 0x105ea95 => {
  { cpu.sub8(cpu.r8(0x10b00f0u32.wrapping_add(cpu.eax)), 0x71u8); }
  if cpu.zf { pc = 0x105eaa1; continue; }
  pc = 0x105ea9e; continue;
 }
 0x105ea9e => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105eaa1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd430u32, v); }
  pc = 0x105eaa8; continue;
 }
 0x105eaa8 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105eaab => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c13u32, v); }
  { let v: u8 = !cpu.r8(0x10ee96eu32); cpu.w8(0x10ee96eu32, v); }
  { let v: u8 = !cpu.r8(0x10fd372u32); cpu.w8(0x10fd372u32, v); }
  { let v: u8 = !cpu.r8(0x1105e67u32); cpu.w8(0x1105e67u32, v); }
  cpu.poll(); pc = 0x105ea9e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105eac6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105eac6); }
 let mut pc: u32 = 0x105eac6;
 loop { match pc {
 0x105eac6 => {
  cpu.push(0x105eacb); f_0105edff(cpu); if cpu.returned_to != 0x105eacb { return; }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x7u8); }
  if !cpu.zf { cpu.poll(); pc = 0x105eac6; continue; }
  pc = 0x105ead4; continue;
 }
 0x105ead4 => {
  cpu.push(0x105ead9); f_0105edff(cpu); if cpu.returned_to != 0x105ead9 { return; }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x7u8); }
  if !cpu.zf { pc = 0x105eaf0; continue; }
  pc = 0x105eae2; continue;
 }
 0x105eae2 => {
  cpu.push(0x105eae7); f_0105eb1d(cpu); if cpu.returned_to != 0x105eae7 { return; }
  if cpu.cf { pc = 0x105eaf0; continue; }
  pc = 0x105eae9; continue;
 }
 0x105eae9 => {
  cpu.push(0x105eaee); f_0105eb00(cpu); if cpu.returned_to != 0x105eaee { return; }
  if !cpu.cf { cpu.poll(); pc = 0x105ead4; continue; }
  pc = 0x105eaf0; continue;
 }
 0x105eaf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105eaf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105eaf1); }
 let mut pc: u32 = 0x105eaf1;
 loop { match pc {
 0x105eaf1 => {
  cpu.push(0x105eaf6); f_0105eb1d(cpu); if cpu.returned_to != 0x105eaf6 { return; }
  if cpu.cf { pc = 0x105eaff; continue; }
  pc = 0x105eaf8; continue;
 }
 0x105eaf8 => {
  cpu.push(0x105eafd); f_0105eb00(cpu); if cpu.returned_to != 0x105eafd { return; }
  if !cpu.cf { cpu.poll(); pc = 0x105eaf1; continue; }
  pc = 0x105eaff; continue;
 }
 0x105eaff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105eb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105eb00); }
 let mut pc: u32 = 0x105eb00;
 loop { match pc {
 0x105eb00 => {
  { let v: u8 = 0x11u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x16); }
  if cpu.zf { pc = 0x105eb1b; continue; }
  pc = 0x105eb06; continue;
 }
 0x105eb06 => {
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b00f0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105eb1b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105eb66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105eb66); }
 let mut pc: u32 = 0x105eb66;
 loop { match pc {
 0x105eb66 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x3au8); }
  if (cpu.sf != cpu.of) { pc = 0x105eb84; continue; }
  pc = 0x105eb72; continue;
 }
 0x105eb72 => {
  { let v: u8 = cpu.sub8((cpu.ebx as u8), 0x47u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x105ebe9; continue; }
  pc = 0x105eb77; continue;
 }
 0x105eb77 => {
  { cpu.sub8((cpu.ebx as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x105ebe9; continue; }
  pc = 0x105eb7c; continue;
 }
 0x105eb7c => {
  { let v: u8 = cpu.r8(0x11c5f36u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x105eba2; continue;
 }
 0x105eb84 => {
  { cpu.sub8((cpu.ebx as u8), 0x36u8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eb89; continue;
 }
 0x105eb89 => {
  { cpu.sub8((cpu.ebx as u8), 0x2au8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eb8e; continue;
 }
 0x105eb8e => {
  { cpu.sub8((cpu.ebx as u8), 0x1cu8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eb93; continue;
 }
 0x105eb93 => {
  { cpu.sub8((cpu.ebx as u8), 0x1du8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eb98; continue;
 }
 0x105eb98 => {
  { cpu.sub8((cpu.ebx as u8), 0xeu8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eb9d; continue;
 }
 0x105eb9d => {
  { cpu.sub8((cpu.ebx as u8), 0xfu8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105eba2; continue;
 }
 0x105eba2 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x105ebbd; continue; }
  pc = 0x105ebb8; continue;
 }
 0x105ebb8 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x3au8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x105ebd0; continue;
 }
 0x105ebbd => {
  { cpu.sub8(cpu.r8(0x10b00eau32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x105ebd0; continue; }
  pc = 0x105ebc6; continue;
 }
 0x105ebc6 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x74u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x80u16); }
  if (cpu.sf == cpu.of) { pc = 0x105ebe9; continue; }
  pc = 0x105ebd0; continue;
 }
 0x105ebd0 => {
  { let v: u32 = cpu.r8(0x11b5187u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c5bb8u32.wrapping_add(cpu.edi.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x105ebe9; continue; }
  pc = 0x105ebe4; continue;
 }
 0x105ebe4 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x105ebe9; continue;
 }
 0x105ebe9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ebed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ebed); }
 let mut pc: u32 = 0x105ebed;
 loop { match pc {
 0x105ebed => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10b00f0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10b00f0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x105ec0a; continue; }
  pc = 0x105ec08; continue;
 }
 0x105ec08 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ec0a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ec0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ec0c); }
 let mut pc: u32 = 0x105ec0c;
 loop { match pc {
 0x105ec0c => {
  { let v = (cpu.ebx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.edx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  pc = 0x105ec12; continue;
 }
 0x105ec12 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x16); }
  if cpu.zf { pc = 0x105ec1e; continue; }
  pc = 0x105ec18; continue;
 }
 0x105ec18 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x16); }
  cpu.poll(); pc = 0x105ec12; continue;
 }
 0x105ec1e => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ec25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ec25); }
 let mut pc: u32 = 0x105ec25;
 loop { match pc {
 0x105ec25 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x10e3d89u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x10e3d85u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d89u32, v); }
  cpu.push(0x105ec3e); f_0107185e(cpu); if cpu.returned_to != 0x105ec3e { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d89u32, v); }
  cpu.push(0x105ec4a); f_0107185e(cpu); if cpu.returned_to != 0x105ec4a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ec4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ec4c); }
 let mut pc: u32 = 0x105ec4c;
 loop { match pc {
 0x105ec4c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97bu32, v); }
  { cpu.sub8(cpu.r8(0x10ee978u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ecd9; continue; }
  pc = 0x105ec5c; continue;
 }
 0x105ec5c => {
  { let v: u32 = 0x400u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x49u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x3u8; cpu.w8(0x49u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x105ec7c); f_0105f339(cpu); if cpu.returned_to != 0x105ec7c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x400u32; cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x49u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x105edaa; continue; }
  pc = 0x105ec8d; continue;
 }
 0x105ec8d => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ef35au32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ee978u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5a2au32, v); }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  cpu.push(0x105ecaa); f_0105f339(cpu); if cpu.returned_to != 0x105ecaa { return; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x11b4c8eu32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11b4c8du32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x10u8); }
  if (cpu.sf == cpu.of) { pc = 0x105ecd9; continue; }
  pc = 0x105ecbb; continue;
 }
 0x105ecbb => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x10fd52eu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x105ecd9; continue; }
  pc = 0x105ecd4; continue;
 }
 0x105ecd4 => {
  cpu.push(0x105ecd9); f_0104091e(cpu); if cpu.returned_to != 0x105ecd9 { return; }
  pc = 0x105ecd9; continue;
 }
 0x105ecd9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd2a6u32, v); }
  cpu.push(0x105ece5); f_01054417(cpu); if cpu.returned_to != 0x105ece5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef355u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1057156u32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x105ecfd); f_0105f339(cpu); if cpu.returned_to != 0x105ecfd { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x9f8u32; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x105ed0e); f_0105f339(cpu); if cpu.returned_to != 0x105ed0e { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x638u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x105ed1f); f_0105f339(cpu); if cpu.returned_to != 0x105ed1f { return; }
  { let v: u32 = 0x98u32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  cpu.push(0x105ed2e); f_0105edc0(cpu); if cpu.returned_to != 0x105ed2e { return; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x105ed42); f_0105f339(cpu); if cpu.returned_to != 0x105ed42 { return; }
  { let v: u32 = 0x13u32; cpu.eax = v; }
  { let v: u32 = 0x5000u32; cpu.edx = v; }
  cpu.push(0x105ed51); f_0105f339(cpu); if cpu.returned_to != 0x105ed51 { return; }
  { cpu.sub8(cpu.r8(0x11c5a2au32), 0x0u8); }
  if !cpu.zf { pc = 0x105eda1; continue; }
  pc = 0x105ed5a; continue;
 }
 0x105ed5a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x1001ec3u32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.eax = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.push(0x105ed7f); f_0105f339(cpu); if cpu.returned_to != 0x105ed7f { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e41c8u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3f3du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef375u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5a2au32, v); }
  pc = 0x105eda1; continue;
 }
 0x105eda1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd361u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105edaa => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105edac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105edac); }
 let mut pc: u32 = 0x105edac;
 loop { match pc {
 0x105edac => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x105edb8); f_0105f339(cpu); if cpu.returned_to != 0x105edb8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee978u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105edc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105edc0); }
 let mut pc: u32 = 0x105edc0;
 loop { match pc {
 0x105edc0 => {
  { cpu.sub8(cpu.r8(0x10ee978u32), 0x0u8); }
  if cpu.zf { pc = 0x105edfe; continue; }
  pc = 0x105edc9; continue;
 }
 0x105edc9 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3cddu32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x2u8); }
  if !cpu.zf { pc = 0x105edf9; continue; }
  pc = 0x105edf3; continue;
 }
 0x105edf3 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105edf9; continue;
 }
 0x105edf9 => {
  cpu.push(0x105edfe); f_0105f339(cpu); if cpu.returned_to != 0x105edfe { return; }
  pc = 0x105edfe; continue;
 }
 0x105edfe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105edff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105edff); }
 let mut pc: u32 = 0x105edff;
 loop { match pc {
 0x105edff => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x105ee09); f_0105f339(cpu); if cpu.returned_to != 0x105ee09 { return; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x2u8); }
  if !cpu.zf { pc = 0x105ee21; continue; }
  pc = 0x105ee12; continue;
 }
 0x105ee12 => {
  { cpu.sub8(cpu.r8(0x10ef355u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ee21; continue; }
  pc = 0x105ee1b; continue;
 }
 0x105ee1b => {
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105ee21; continue;
 }
 0x105ee21 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef355u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ee42; continue; }
  pc = 0x105ee32; continue;
 }
 0x105ee32 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3cddu32, v); }
  pc = 0x105ee50; continue;
 }
 0x105ee42 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3cddu32, v); }
  pc = 0x105ee50; continue;
 }
 0x105ee50 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ee968u32, v); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x105ee62; continue; }
  pc = 0x105ee5b; continue;
 }
 0x105ee5b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  pc = 0x105ee62; continue;
 }
 0x105ee62 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x105ee6e; continue; }
  pc = 0x105ee67; continue;
 }
 0x105ee67 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97eu32, v); }
  pc = 0x105ee6e; continue;
 }
 0x105ee6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ee6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ee6f); }
 let mut pc: u32 = 0x105ee6f;
 loop { match pc {
 0x105ee6f => {
  { let v: u16 = cpu.r16(0x10e3ef5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ef7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ef9u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3efbu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x2u8); }
  if !cpu.zf { pc = 0x105eea2; continue; }
  pc = 0x105ee93; continue;
 }
 0x105ee93 => {
  { cpu.sub8(cpu.r8(0x10ef355u32), 0x0u8); }
  if !cpu.zf { pc = 0x105eea2; continue; }
  pc = 0x105ee9c; continue;
 }
 0x105ee9c => {
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105eea2; continue;
 }
 0x105eea2 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x105eedf; continue; }
  pc = 0x105eea6; continue;
 }
 0x105eea6 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef355u32), 0x0u8); }
  if !cpu.zf { pc = 0x105eed1; continue; }
  pc = 0x105eeb7; continue;
 }
 0x105eeb7 => {
  cpu.push(0x105eebc); f_0105ef21(cpu); if cpu.returned_to != 0x105eebc { return; }
  cpu.push(0x105eec1); f_0105ef93(cpu); if cpu.returned_to != 0x105eec1 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3cddu32, v); }
  pc = 0x105eedf; continue;
 }
 0x105eed1 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3cddu32, v); }
  pc = 0x105eedf; continue;
 }
 0x105eedf => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ee968u32, v); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x105eef1; continue; }
  pc = 0x105eeea; continue;
 }
 0x105eeea => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  pc = 0x105eef1; continue;
 }
 0x105eef1 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x105eefd; continue; }
  pc = 0x105eef6; continue;
 }
 0x105eef6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97eu32, v); }
  pc = 0x105eefd; continue;
 }
 0x105eefd => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x105ef09; continue; }
  pc = 0x105ef02; continue;
 }
 0x105ef02 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35bu32, v); }
  pc = 0x105ef09; continue;
 }
 0x105ef09 => {
  cpu.push(0x105ef0e); f_0105f02d(cpu); if cpu.returned_to != 0x105ef0e { return; }
  { cpu.sub8(cpu.r8(0x1057156u32), 0x0u8); }
  if cpu.zf { pc = 0x105ef20; continue; }
  pc = 0x105ef17; continue;
 }
 0x105ef17 => {
  { cpu.sub8(cpu.r8(0x10ef375u32), 0x0u8); }
  if cpu.zf { pc = 0x105ef20; continue; }
  pc = 0x105ef20; continue;
 }
 0x105ef20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ef21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ef21); }
 let mut pc: u32 = 0x105ef21;
 loop { match pc {
 0x105ef21 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x105ef2b; continue; }
  pc = 0x105ef2a; continue;
 }
 0x105ef2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ef2b => {
  { let v: u16 = 0x10eu16; cpu.w16(0x10fd562u32, v); }
  { let v: u16 = 0xb6u16; cpu.w16(0x10fd564u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105ef5e; continue; }
  pc = 0x105ef46; continue;
 }
 0x105ef46 => {
  { let v: u16 = 0x21cu16; cpu.w16(0x10fd562u32, v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x24u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10fd564u32, v); }
  pc = 0x105ef5e; continue;
 }
 0x105ef5e => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10fd562u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105ef6e; continue; }
  pc = 0x105ef67; continue;
 }
 0x105ef67 => {
  { let v: u16 = cpu.r16(0x10fd562u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105ef6e; continue;
 }
 0x105ef6e => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10fd564u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105ef7e; continue; }
  pc = 0x105ef77; continue;
 }
 0x105ef77 => {
  { let v: u16 = cpu.r16(0x10fd564u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105ef7e; continue;
 }
 0x105ef7e => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x105ef88; continue; }
  pc = 0x105ef84; continue;
 }
 0x105ef84 => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105ef88; continue;
 }
 0x105ef88 => {
  { cpu.sub16((cpu.edx as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x105ef92; continue; }
  pc = 0x105ef8e; continue;
 }
 0x105ef8e => {
  { let v: u16 = 0x2u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105ef92; continue;
 }
 0x105ef92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105ef93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105ef93); }
 let mut pc: u32 = 0x105ef93;
 loop { match pc {
 0x105ef93 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x105ef9d; continue; }
  pc = 0x105ef9c; continue;
 }
 0x105ef9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105ef9d => {
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if !cpu.zf { pc = 0x105efa7; continue; }
  pc = 0x105efa6; continue;
 }
 0x105efa6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105efa7 => {
  { let v: u16 = 0x2u16; cpu.w16(0x1105671u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x1105673u32, v); }
  { let v: u16 = 0x10eu16; cpu.w16(0x10fd562u32, v); }
  { let v: u16 = 0xb6u16; cpu.w16(0x10fd564u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x105efec; continue; }
  pc = 0x105efd4; continue;
 }
 0x105efd4 => {
  { let v: u16 = 0x21cu16; cpu.w16(0x10fd562u32, v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x24u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10fd564u32, v); }
  pc = 0x105efec; continue;
 }
 0x105efec => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10fd562u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105effc; continue; }
  pc = 0x105eff5; continue;
 }
 0x105eff5 => {
  { let v: u16 = cpu.r16(0x10fd562u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105effc; continue;
 }
 0x105effc => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10fd564u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105f00c; continue; }
  pc = 0x105f005; continue;
 }
 0x105f005 => {
  { let v: u16 = cpu.r16(0x10fd564u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105f00c; continue;
 }
 0x105f00c => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x1105671u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105f01c; continue; }
  pc = 0x105f015; continue;
 }
 0x105f015 => {
  { let v: u16 = cpu.r16(0x1105671u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105f01c; continue;
 }
 0x105f01c => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1105673u32)); }
  if (cpu.sf == cpu.of) { pc = 0x105f02c; continue; }
  pc = 0x105f025; continue;
 }
 0x105f025 => {
  { let v: u16 = cpu.r16(0x1105673u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105f02c; continue;
 }
 0x105f02c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f02d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f02d); }
 let mut pc: u32 = 0x105f02d;
 loop { match pc {
 0x105f02d => {
  { cpu.pushad(); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x105f060; continue; }
  pc = 0x105f033; continue;
 }
 0x105f033 => {
  { cpu.and8(cpu.r8(0x11c5a2bu32), 0x1u8); }
  if cpu.zf { pc = 0x105f0d1; continue; }
  pc = 0x105f040; continue;
 }
 0x105f040 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11c5a22u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11c5a24u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c59f0u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11c5a2bu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105f060 => {
  { cpu.and8(cpu.r8(0x11c5a2bu32), 0x1u8); }
  if !cpu.zf { pc = 0x105f0d1; continue; }
  pc = 0x105f069; continue;
 }
 0x105f069 => {
  { cpu.sub32(cpu.r32(0x11c59f0u32), 0x14u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x105f0b8; continue; }
  pc = 0x105f072; continue;
 }
 0x105f072 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x11c5a22u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105f07e; continue; }
  pc = 0x105f07b; continue;
 }
 0x105f07b => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x105f07e; continue;
 }
 0x105f07e => {
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105f0b8; continue; }
  pc = 0x105f084; continue;
 }
 0x105f084 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x11c5a24u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x105f090; continue; }
  pc = 0x105f08d; continue;
 }
 0x105f08d => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x105f090; continue;
 }
 0x105f090 => {
  { cpu.sub16((cpu.edx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x105f0b8; continue; }
  pc = 0x105f096; continue;
 }
 0x105f096 => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5a00u32, v); }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5a02u32, v); }
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c59fcu32, v); }
  pc = 0x105f0b8; continue;
 }
 0x105f0b8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5a2bu32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x11c5a22u32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x11c5a24u32, v); }
  pc = 0x105f0d1; continue;
 }
 0x105f0d1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f0d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f0d3); }
 let mut pc: u32 = 0x105f0d3;
 loop { match pc {
 0x105f0d3 => {
  { let v: u32 = 0x10f7c08u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4028u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x105f0e9); f_01033bea(cpu); if cpu.returned_to != 0x105f0e9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f0eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f0eb); }
 let mut pc: u32 = 0x105f0eb;
 loop { match pc {
 0x105f0eb => {
  { let v: u32 = 0x10f7c21u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4038u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x105f101); f_01033bea(cpu); if cpu.returned_to != 0x105f101 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4038u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x280u32; cpu.ecx = v; }
  pc = 0x105f10f; continue;
 }
 0x105f10f => {
  { cpu.lods(2, false); }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x105f10f; continue; }
  pc = 0x105f11b; continue;
 }
 0x105f11b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f178); }
 let mut pc: u32 = 0x105f178;
 loop { match pc {
 0x105f178 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5afau32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x112a1e2u32); cpu.edi = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = cpu.r32(0x11c5afau32); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b02u32, v); }
  { let v = cpu.r16(0x10e3cdfu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x10e3ec1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cdfu32, v); }
  cpu.push(0x105f1ad); f_0105f215(cpu); if cpu.returned_to != 0x105f1ad { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ec1u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3cdfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b02u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f1cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f1cc); }
 let mut pc: u32 = 0x105f1cc;
 loop { match pc {
 0x105f1cc => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5afau32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x112a1e2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x110d530u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11c5afau32).wrapping_add(0x10u32); cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.r32(0x11c5afau32) & 0x3ffu32; cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f215(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f215); }
 let mut pc: u32 = 0x105f215;
 loop { match pc {
 0x105f215 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x11c5b02u32), 0x0u8); }
  if !cpu.zf { pc = 0x105f229; continue; }
  pc = 0x105f220; continue;
 }
 0x105f220 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if !cpu.zf { pc = 0x105f229; continue; }
  pc = 0x105f229; continue;
 }
 0x105f229 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5afau32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x112a1e2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd438u32), 0x0u8); }
  if cpu.zf { pc = 0x105f249; continue; }
  pc = 0x105f244; continue;
 }
 0x105f244 => {
  { let v: u32 = cpu.eax | 0x80000000u32; cpu.eax = v; }
  pc = 0x105f249; continue;
 }
 0x105f249 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x100001fu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10ee934u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x110d530u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11c5afau32).wrapping_add(0x10u32); cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.r32(0x11c5afau32) & 0x3ffu32; cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3cdfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cdfu32, v); }
  { cpu.sub8(cpu.r8(0x11c5b02u32), 0x0u8); }
  if !cpu.zf { pc = 0x105f2e9; continue; }
  pc = 0x105f2be; continue;
 }
 0x105f2be => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x1u8); }
  if !cpu.zf { pc = 0x105f2e9; continue; }
  pc = 0x105f2c7; continue;
 }
 0x105f2c7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5afau32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x112a1e2u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11c5afau32).wrapping_add(0x2u32); cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x11c5afau32), 0x3ffu32); cpu.w32(0x11c5afau32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x105f2e9; continue;
 }
 0x105f2e9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f2ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f2ec); }
 let mut pc: u32 = 0x105f2ec;
 loop { match pc {
 0x105f2ec => {
  cpu.push(0x105f2f1); f_0105f215(cpu); if cpu.returned_to != 0x105f2f1 { return; }
  f_0105f2f1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f2f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f2f1); }
 let mut pc: u32 = 0x105f2f1;
 loop { match pc {
 0x105f2f1 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x105f2fa; continue; }
  pc = 0x105f2f8; continue;
 }
 0x105f2f8 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x105f2fa; continue;
 }
 0x105f2fa => {
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if cpu.cf { pc = 0x105f302; continue; }
  pc = 0x105f2ff; continue;
 }
 0x105f2ff => {
  { let a = (cpu.ecx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x105f302; continue;
 }
 0x105f302 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f329(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f329); }
 let mut pc: u32 = 0x105f329;
 loop { match pc {
 0x105f329 => {
  cpu.push(0x105f32e); f_0105f178(cpu); if cpu.returned_to != 0x105f32e { return; }
  cpu.push(0x105f333); f_0105f2f1(cpu); if cpu.returned_to != 0x105f333 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f339(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f339); }
 let mut pc: u32 = 0x105f339;
 loop { match pc {
 0x105f339 => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x105f33f; continue; }
  pc = 0x105f33e; continue;
 }
 0x105f33e => {
  {  }
  pc = 0x105f33f; continue;
 }
 0x105f33f => {
  { let v: u32 = 0x400u32; cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x49u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x49u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.interrupt(0x33); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x400u32; cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x49u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f360); }
 let mut pc: u32 = 0x105f360;
 loop { match pc {
 0x105f360 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11b543fu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x11b5443u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x105f3bau32; cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x11b5445u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x105f391); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f392(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f392); }
 let mut pc: u32 = 0x105f392;
 loop { match pc {
 0x105f392 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x11b5445u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x105f3b1; continue; }
  pc = 0x105f39c; continue;
 }
 0x105f39c => {
  { let v: u32 = cpu.r32(0x11b543fu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x11b5443u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x105f3b1; continue;
 }
 0x105f3b1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b5445u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  cpu.flat_ds(0x105f3b9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f3ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f3ba); }
 let mut pc: u32 = 0x105f3ba;
 loop { match pc {
 0x105f3ba => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x1u32; cpu.w32(0x11cec24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x105f3d2); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f3d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f3d3); }
 let mut pc: u32 = 0x105f3d3;
 loop { match pc {
 0x105f3d3 => {
  { cpu.sub8(cpu.r8(0x11cccddu32), 0x0u8); }
  if !cpu.zf { pc = 0x105f635; continue; }
  pc = 0x105f3e0; continue;
 }
 0x105f3e0 => {
  { cpu.pushad(); }
  { let v = cpu.r16(0x10e4108u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5f4cu32, v); }
  { let v: u8 = cpu.r8(0x10ef352u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x11c633cu32), 0x0u8); }
  if !cpu.zf { pc = 0x105f474; continue; }
  pc = 0x105f3fe; continue;
 }
 0x105f3fe => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x105f408); f_010725e4(cpu); if cpu.returned_to != 0x105f408 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f41a); f_010385ad(cpu); if cpu.returned_to != 0x105f41a { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f422); cpu.call(t); if cpu.returned_to != 0x105f422 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  cpu.push(0x105f42e); f_01072682(cpu); if cpu.returned_to != 0x105f42e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10e3e93u32, v); }
  cpu.push(0x105f43a); f_01071c84(cpu); if cpu.returned_to != 0x105f43a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e93u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f453); f_010385ad(cpu); if cpu.returned_to != 0x105f453 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f45b); cpu.call(t); if cpu.returned_to != 0x105f45b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u16 = 0x140u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x10e4108u32, v); }
  pc = 0x105f474; continue;
 }
 0x105f474 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1105668u32, v); }
  { cpu.sub8(cpu.r8(0x11c62fbu32), 0x0u8); }
  if !cpu.zf { pc = 0x105f60e; continue; }
  pc = 0x105f488; continue;
 }
 0x105f488 => {
  { cpu.sub8(cpu.r8(0x1124468u32), 0x0u8); }
  if cpu.zf { pc = 0x105f620; continue; }
  pc = 0x105f495; continue;
 }
 0x105f495 => {
  { cpu.sub8(cpu.r8(0x112a1e1u32), 0x0u8); }
  if cpu.zf { pc = 0x105f60e; continue; }
  pc = 0x105f4a2; continue;
 }
 0x105f4a2 => {
  cpu.push(0x105f4a7); f_010723fb(cpu); if cpu.returned_to != 0x105f4a7 { return; }
  { let v: u16 = 0x824u16; cpu.w16(0x10e3db7u32, v); }
  cpu.push(0x105f4b5); f_0105f6ca(cpu); if cpu.returned_to != 0x105f4b5 { return; }
  cpu.push(0x105f4ba); f_01026fd9(cpu); if cpu.returned_to != 0x105f4ba { return; }
  cpu.push(0x105f4bf); f_0105f638(cpu); if cpu.returned_to != 0x105f4bf { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  pc = 0x105f4c6; continue;
 }
 0x105f4c6 => {
  cpu.push(0x105f4cb); f_0105edff(cpu); if cpu.returned_to != 0x105f4cb { return; }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x7u8); }
  if !cpu.zf { cpu.poll(); pc = 0x105f4c6; continue; }
  pc = 0x105f4d4; continue;
 }
 0x105f4d4 => {
  cpu.push(0x105f4d9); f_0105edff(cpu); if cpu.returned_to != 0x105f4d9 { return; }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x7u8); }
  if cpu.zf { pc = 0x105f526; continue; }
  pc = 0x105f4e2; continue;
 }
 0x105f4e2 => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x68u16); }
  if cpu.cf { pc = 0x105f50c; continue; }
  pc = 0x105f4f5; continue;
 }
 0x105f4f5 => {
  { cpu.sub16((cpu.eax as u16), 0x8bu16); }
  if !cpu.cf { pc = 0x105f50c; continue; }
  pc = 0x105f4fb; continue;
 }
 0x105f4fb => {
  { cpu.sub16((cpu.ebx as u16), 0x6fu16); }
  if cpu.cf { pc = 0x105f50c; continue; }
  pc = 0x105f501; continue;
 }
 0x105f501 => {
  { cpu.sub16((cpu.ebx as u16), 0x7cu16); }
  if !cpu.cf { pc = 0x105f50c; continue; }
  pc = 0x105f507; continue;
 }
 0x105f507 => {
  pc = 0x105f5cb; continue;
 }
 0x105f50c => {
  { cpu.sub16((cpu.eax as u16), 0xa2u16); }
  if cpu.cf { pc = 0x105f526; continue; }
  pc = 0x105f512; continue;
 }
 0x105f512 => {
  { cpu.sub16((cpu.eax as u16), 0xc6u16); }
  if !cpu.cf { pc = 0x105f526; continue; }
  pc = 0x105f518; continue;
 }
 0x105f518 => {
  { cpu.sub16((cpu.ebx as u16), 0x6fu16); }
  if cpu.cf { pc = 0x105f526; continue; }
  pc = 0x105f51e; continue;
 }
 0x105f51e => {
  { cpu.sub16((cpu.ebx as u16), 0x7cu16); }
  if !cpu.cf { pc = 0x105f526; continue; }
  pc = 0x105f524; continue;
 }
 0x105f524 => {
  pc = 0x105f53b; continue;
 }
 0x105f526 => {
  cpu.push(0x105f52b); f_0105eb1d(cpu); if cpu.returned_to != 0x105f52b { return; }
  if cpu.cf { pc = 0x105f52f; continue; }
  pc = 0x105f52d; continue;
 }
 0x105f52d => {
  if !cpu.cf { cpu.poll(); pc = 0x105f4d4; continue; }
  pc = 0x105f52f; continue;
 }
 0x105f52f => {
  { cpu.sub8((cpu.eax as u8), 0x6fu8); }
  if cpu.zf { pc = 0x105f53b; continue; }
  pc = 0x105f533; continue;
 }
 0x105f533 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x105f5c3; continue; }
  pc = 0x105f53b; continue;
 }
 0x105f53b => {
  cpu.push(0x105f540); f_0102702b(cpu); if cpu.returned_to != 0x105f540 { return; }
  cpu.push(0x105f545); f_0105f638(cpu); if cpu.returned_to != 0x105f545 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f557); f_010385ad(cpu); if cpu.returned_to != 0x105f557 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f55f); cpu.call(t); if cpu.returned_to != 0x105f55f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x105f566; continue;
 }
 0x105f566 => {
  cpu.push(0x105f56b); f_0105edff(cpu); if cpu.returned_to != 0x105f56b { return; }
  { cpu.and8(cpu.r8(0x10ee968u32), 0x7u8); }
  if !cpu.zf { cpu.poll(); pc = 0x105f566; continue; }
  pc = 0x105f574; continue;
 }
 0x105f574 => {
  cpu.push(0x105f579); f_0105f67d(cpu); if cpu.returned_to != 0x105f579 { return; }
  { cpu.sub8(cpu.r8(0x11c5f4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x105f5a8; continue; }
  pc = 0x105f582; continue;
 }
 0x105f582 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c6339u32, v); }
  { cpu.pushad(); }
  cpu.push(0x105f58f); f_01079415(cpu); if cpu.returned_to != 0x105f58f { return; }
  { let v = cpu.r32(0x11cec24u32); cpu.push(v); }
  cpu.push(0x105f59a); f_01001bb7(cpu); if cpu.returned_to != 0x105f59a { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x11cec24u32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c6339u32, v); }
  pc = 0x105f5a8; continue;
 }
 0x105f5a8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b5446u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef352u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { cpu.popad(); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x105f5c3 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { cpu.poll(); pc = 0x105f4d4; continue; }
  pc = 0x105f5cb; continue;
 }
 0x105f5cb => {
  cpu.push(0x105f5d0); f_01027002(cpu); if cpu.returned_to != 0x105f5d0 { return; }
  cpu.push(0x105f5d5); f_0105f638(cpu); if cpu.returned_to != 0x105f5d5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f5e7); f_010385ad(cpu); if cpu.returned_to != 0x105f5e7 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f5ef); cpu.call(t); if cpu.returned_to != 0x105f5ef { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  f_0105f11c(cpu); return;
 }
 0x105f60e => {
  { cpu.sub8(cpu.r8(0x11c5f4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x105f620; continue; }
  pc = 0x105f617; continue;
 }
 0x105f617 => {
  { cpu.sub8(cpu.r8(0x11c633au32), 0x0u8); }
  if cpu.zf { pc = 0x105f620; continue; }
  pc = 0x105f620; continue;
 }
 0x105f620 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b5446u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef352u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { cpu.popad(); }
  pc = 0x105f635; continue;
 }
 0x105f635 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f638); }
 let mut pc: u32 = 0x105f638;
 loop { match pc {
 0x105f638 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f64a); f_010385ad(cpu); if cpu.returned_to != 0x105f64a { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f652); cpu.call(t); if cpu.returned_to != 0x105f652 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x105f65f); cpu.call(t); if cpu.returned_to != 0x105f65f { return; }
  cpu.push(0x105f664); f_01027065(cpu); if cpu.returned_to != 0x105f664 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f66c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f66c); }
 let mut pc: u32 = 0x105f66c;
 loop { match pc {
 0x105f66c => {
  cpu.push(0x105f671); f_01026fd9(cpu); if cpu.returned_to != 0x105f671 { return; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x105f677); cpu.call(t); if cpu.returned_to != 0x105f677 { return; }
  cpu.push(0x105f67c); f_01027065(cpu); if cpu.returned_to != 0x105f67c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f67d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f67d); }
 let mut pc: u32 = 0x105f67d;
 loop { match pc {
 0x105f67d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f68f); f_010385ad(cpu); if cpu.returned_to != 0x105f68f { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f697); cpu.call(t); if cpu.returned_to != 0x105f697 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u16 = cpu.r16(0x11c5abcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = cpu.r16(0x11c5abeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = cpu.r16(0x11c5ac0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d61u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f6ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x105f6ca); }
 let mut pc: u32 = 0x105f6ca;
 loop { match pc {
 0x105f6ca => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x105f6dc); f_010385ad(cpu); if cpu.returned_to != 0x105f6dc { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x105f6e4); cpu.call(t); if cpu.returned_to != 0x105f6e4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u16 = cpu.r16(0x10e3db7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5abcu32, v); }
  { let v: u16 = cpu.r16(0x10e3d5fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5abeu32, v); }
  { let v: u16 = cpu.r16(0x10e3d61u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11c5ac0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d61u32, v); }
  { let v: u16 = 0x824u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f732(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105f732); } r_0105f732(cpu, 0x105f732); }
pub fn f_0105f741(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105f741); } r_0105f732(cpu, 0x105f741); }
fn r_0105f732(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x105f732 => {
  { cpu.pushad(); }
  { let v: u32 = 0x10ef352u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11b51b9u32), 0x0u8); }
  pc = 0x105f7a2; continue;
 }
 0x105f741 => {
  if !cpu.zf { pc = 0x105f7a2; continue; }
  pc = 0x105f743; continue;
 }
 0x105f743 => {
  { let v: u32 = 0x112a09cu32; cpu.ebp = v; }
  pc = 0x105f748; continue;
 }
 0x105f748 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x1121365u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c59cau32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1121365u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x105f770); f_0102d611(cpu); if cpu.returned_to != 0x105f770 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x1121365u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x10e412cu32), cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x105f7a2; continue; }
  pc = 0x105f78c; continue;
 }
 0x105f78c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x105f748; continue; }
  pc = 0x105f795; continue;
 }
 0x105f795 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { cpu.poll(); pc = 0x105f748; continue; }
  pc = 0x105f79d; continue;
 }
 0x105f79d => {
  pc = 0x105f842; continue;
 }
 0x105f7a2 => {
  { let v: u32 = cpu.r32(0x11c6340u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6348u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x105f7be; continue; }
  pc = 0x105f7b3; continue;
 }
 0x105f7b3 => {
  cpu.push(0x105f7b8); f_0105f844(cpu); if cpu.returned_to != 0x105f7b8 { return; }
  if cpu.cf { pc = 0x105f842; continue; }
  pc = 0x105f7be; continue;
 }
 0x105f7be => {
  { let v: u32 = cpu.r32(0x11c6344u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6348u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x105f7d6; continue; }
  pc = 0x105f7cf; continue;
 }
 0x105f7cf => {
  cpu.push(0x105f7d4); f_0105f844(cpu); if cpu.returned_to != 0x105f7d4 { return; }
  if cpu.cf { pc = 0x105f842; continue; }
  pc = 0x105f7d6; continue;
 }
 0x105f7d6 => {
  { let v: u32 = cpu.r32(0x11c6309u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6305u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x105f7ee; continue; }
  pc = 0x105f7e7; continue;
 }
 0x105f7e7 => {
  cpu.push(0x105f7ec); f_0105f844(cpu); if cpu.returned_to != 0x105f7ec { return; }
  if cpu.cf { pc = 0x105f842; continue; }
  pc = 0x105f7ee; continue;
 }
 0x105f7ee => {
  { let v: u32 = cpu.r32(0x11c6311u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c630du32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x105f806; continue; }
  pc = 0x105f7ff; continue;
 }
 0x105f7ff => {
  cpu.push(0x105f804); f_0105f844(cpu); if cpu.returned_to != 0x105f804 { return; }
  if cpu.cf { pc = 0x105f842; continue; }
  pc = 0x105f806; continue;
 }
 0x105f806 => {
  { let v: u32 = cpu.r32(0x11c6319u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11c6315u32); cpu.ecx = v; }
  cpu.push(0x105f817); f_0105f844(cpu); if cpu.returned_to != 0x105f817 { return; }
  if cpu.cf { pc = 0x105f842; continue; }
  pc = 0x105f819; continue;
 }
 0x105f819 => {
  { let v: u32 = 0x100001fu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4034u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x105f82d; continue; }
  pc = 0x105f829; continue;
 }
 0x105f829 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  if cpu.sf { pc = 0x105f842; continue; }
  pc = 0x105f82d; continue;
 }
 0x105f82d => {
  { let v: u16 = cpu.r16(0x105564cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f01u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x105f842; continue; }
  pc = 0x105f83d; continue;
 }
 0x105f83d => {
  cpu.push(0x105f842); f_0105f844(cpu); if cpu.returned_to != 0x105f842 { return; }
  pc = 0x105f842; continue;
 }
 0x105f842 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0105f844(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x105f844); } r_0105f844(cpu, 0x105f844); }
pub fn f_01060000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060000); } r_0105f844(cpu, 0x1060000); }
pub fn f_01060105(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060105); } r_0105f844(cpu, 0x1060105); }
pub fn f_01060246(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060246); } r_0105f844(cpu, 0x1060246); }
pub fn f_01060264(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060264); } r_0105f844(cpu, 0x1060264); }
pub fn f_01060601(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060601); } r_0105f844(cpu, 0x1060601); }
pub fn f_01060606(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060606); } r_0105f844(cpu, 0x1060606); }
pub fn f_0106060c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106060c); } r_0105f844(cpu, 0x106060c); }
pub fn f_01060620(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060620); } r_0105f844(cpu, 0x1060620); }
pub fn f_010606fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10606fa); } r_0105f844(cpu, 0x10606fa); }
pub fn f_01060c0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1060c0c); } r_0105f844(cpu, 0x1060c0c); }
fn r_0105f844(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x105f844 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 12u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f858; continue;
 }
 0x105f858 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f86e; continue; }
  pc = 0x105f861; continue;
 }
 0x105f861 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f86e; continue; }
  pc = 0x105f869; continue;
 }
 0x105f869 => {
  pc = 0x1061f2d; continue;
 }
 0x105f86e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f87f; continue;
 }
 0x105f87f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f895; continue; }
  pc = 0x105f888; continue;
 }
 0x105f888 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f895; continue; }
  pc = 0x105f890; continue;
 }
 0x105f890 => {
  pc = 0x1061f2d; continue;
 }
 0x105f895 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f8a6; continue;
 }
 0x105f8a6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f8bc; continue; }
  pc = 0x105f8af; continue;
 }
 0x105f8af => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f8bc; continue; }
  pc = 0x105f8b7; continue;
 }
 0x105f8b7 => {
  pc = 0x1061f2d; continue;
 }
 0x105f8bc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f8cd; continue;
 }
 0x105f8cd => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f8e3; continue; }
  pc = 0x105f8d6; continue;
 }
 0x105f8d6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f8e3; continue; }
  pc = 0x105f8de; continue;
 }
 0x105f8de => {
  pc = 0x1061f2d; continue;
 }
 0x105f8e3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f8f4; continue;
 }
 0x105f8f4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f90a; continue; }
  pc = 0x105f8fd; continue;
 }
 0x105f8fd => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f90a; continue; }
  pc = 0x105f905; continue;
 }
 0x105f905 => {
  pc = 0x1061f2d; continue;
 }
 0x105f90a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f91b; continue;
 }
 0x105f91b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f931; continue; }
  pc = 0x105f924; continue;
 }
 0x105f924 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f931; continue; }
  pc = 0x105f92c; continue;
 }
 0x105f92c => {
  pc = 0x1061f2d; continue;
 }
 0x105f931 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f942; continue;
 }
 0x105f942 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f958; continue; }
  pc = 0x105f94b; continue;
 }
 0x105f94b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f958; continue; }
  pc = 0x105f953; continue;
 }
 0x105f953 => {
  pc = 0x1061f2d; continue;
 }
 0x105f958 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f969; continue;
 }
 0x105f969 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f97f; continue; }
  pc = 0x105f972; continue;
 }
 0x105f972 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f97f; continue; }
  pc = 0x105f97a; continue;
 }
 0x105f97a => {
  pc = 0x1061f2d; continue;
 }
 0x105f97f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f990; continue;
 }
 0x105f990 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f9a6; continue; }
  pc = 0x105f999; continue;
 }
 0x105f999 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f9a6; continue; }
  pc = 0x105f9a1; continue;
 }
 0x105f9a1 => {
  pc = 0x1061f2d; continue;
 }
 0x105f9a6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f9b7; continue;
 }
 0x105f9b7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f9cd; continue; }
  pc = 0x105f9c0; continue;
 }
 0x105f9c0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f9cd; continue; }
  pc = 0x105f9c8; continue;
 }
 0x105f9c8 => {
  pc = 0x1061f2d; continue;
 }
 0x105f9cd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105f9de; continue;
 }
 0x105f9de => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105f9f4; continue; }
  pc = 0x105f9e7; continue;
 }
 0x105f9e7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105f9f4; continue; }
  pc = 0x105f9ef; continue;
 }
 0x105f9ef => {
  pc = 0x1061f2d; continue;
 }
 0x105f9f4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fa05; continue;
 }
 0x105fa05 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fa1b; continue; }
  pc = 0x105fa0e; continue;
 }
 0x105fa0e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fa1b; continue; }
  pc = 0x105fa16; continue;
 }
 0x105fa16 => {
  pc = 0x1061f2d; continue;
 }
 0x105fa1b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fa2c; continue;
 }
 0x105fa2c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fa42; continue; }
  pc = 0x105fa35; continue;
 }
 0x105fa35 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fa42; continue; }
  pc = 0x105fa3d; continue;
 }
 0x105fa3d => {
  pc = 0x1061f2d; continue;
 }
 0x105fa42 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fa53; continue;
 }
 0x105fa53 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fa69; continue; }
  pc = 0x105fa5c; continue;
 }
 0x105fa5c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fa69; continue; }
  pc = 0x105fa64; continue;
 }
 0x105fa64 => {
  pc = 0x1061f2d; continue;
 }
 0x105fa69 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fa7a; continue;
 }
 0x105fa7a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fa90; continue; }
  pc = 0x105fa83; continue;
 }
 0x105fa83 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fa90; continue; }
  pc = 0x105fa8b; continue;
 }
 0x105fa8b => {
  pc = 0x1061f2d; continue;
 }
 0x105fa90 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105faa1; continue;
 }
 0x105faa1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fab7; continue; }
  pc = 0x105faaa; continue;
 }
 0x105faaa => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fab7; continue; }
  pc = 0x105fab2; continue;
 }
 0x105fab2 => {
  pc = 0x1061f2d; continue;
 }
 0x105fab7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fac8; continue;
 }
 0x105fac8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fade; continue; }
  pc = 0x105fad1; continue;
 }
 0x105fad1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fade; continue; }
  pc = 0x105fad9; continue;
 }
 0x105fad9 => {
  pc = 0x1061f2d; continue;
 }
 0x105fade => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105faef; continue;
 }
 0x105faef => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fb05; continue; }
  pc = 0x105faf8; continue;
 }
 0x105faf8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fb05; continue; }
  pc = 0x105fb00; continue;
 }
 0x105fb00 => {
  pc = 0x1061f2d; continue;
 }
 0x105fb05 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fb16; continue;
 }
 0x105fb16 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fb2c; continue; }
  pc = 0x105fb1f; continue;
 }
 0x105fb1f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fb2c; continue; }
  pc = 0x105fb27; continue;
 }
 0x105fb27 => {
  pc = 0x1061f2d; continue;
 }
 0x105fb2c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fb3d; continue;
 }
 0x105fb3d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fb53; continue; }
  pc = 0x105fb46; continue;
 }
 0x105fb46 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fb53; continue; }
  pc = 0x105fb4e; continue;
 }
 0x105fb4e => {
  pc = 0x1061f2d; continue;
 }
 0x105fb53 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fb64; continue;
 }
 0x105fb64 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fb7a; continue; }
  pc = 0x105fb6d; continue;
 }
 0x105fb6d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fb7a; continue; }
  pc = 0x105fb75; continue;
 }
 0x105fb75 => {
  pc = 0x1061f2d; continue;
 }
 0x105fb7a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fb8b; continue;
 }
 0x105fb8b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fba1; continue; }
  pc = 0x105fb94; continue;
 }
 0x105fb94 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fba1; continue; }
  pc = 0x105fb9c; continue;
 }
 0x105fb9c => {
  pc = 0x1061f2d; continue;
 }
 0x105fba1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fbb2; continue;
 }
 0x105fbb2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fbc8; continue; }
  pc = 0x105fbbb; continue;
 }
 0x105fbbb => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fbc8; continue; }
  pc = 0x105fbc3; continue;
 }
 0x105fbc3 => {
  pc = 0x1061f2d; continue;
 }
 0x105fbc8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fbd9; continue;
 }
 0x105fbd9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fbef; continue; }
  pc = 0x105fbe2; continue;
 }
 0x105fbe2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fbef; continue; }
  pc = 0x105fbea; continue;
 }
 0x105fbea => {
  pc = 0x1061f2d; continue;
 }
 0x105fbef => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fc00; continue;
 }
 0x105fc00 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fc16; continue; }
  pc = 0x105fc09; continue;
 }
 0x105fc09 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fc16; continue; }
  pc = 0x105fc11; continue;
 }
 0x105fc11 => {
  pc = 0x1061f2d; continue;
 }
 0x105fc16 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fc27; continue;
 }
 0x105fc27 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fc3d; continue; }
  pc = 0x105fc30; continue;
 }
 0x105fc30 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fc3d; continue; }
  pc = 0x105fc38; continue;
 }
 0x105fc38 => {
  pc = 0x1061f2d; continue;
 }
 0x105fc3d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fc4e; continue;
 }
 0x105fc4e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fc64; continue; }
  pc = 0x105fc57; continue;
 }
 0x105fc57 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fc64; continue; }
  pc = 0x105fc5f; continue;
 }
 0x105fc5f => {
  pc = 0x1061f2d; continue;
 }
 0x105fc64 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fc75; continue;
 }
 0x105fc75 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fc8b; continue; }
  pc = 0x105fc7e; continue;
 }
 0x105fc7e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fc8b; continue; }
  pc = 0x105fc86; continue;
 }
 0x105fc86 => {
  pc = 0x1061f2d; continue;
 }
 0x105fc8b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fc9c; continue;
 }
 0x105fc9c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fcb2; continue; }
  pc = 0x105fca5; continue;
 }
 0x105fca5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fcb2; continue; }
  pc = 0x105fcad; continue;
 }
 0x105fcad => {
  pc = 0x1061f2d; continue;
 }
 0x105fcb2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fcc3; continue;
 }
 0x105fcc3 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fcd9; continue; }
  pc = 0x105fccc; continue;
 }
 0x105fccc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fcd9; continue; }
  pc = 0x105fcd4; continue;
 }
 0x105fcd4 => {
  pc = 0x1061f2d; continue;
 }
 0x105fcd9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fcea; continue;
 }
 0x105fcea => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fd00; continue; }
  pc = 0x105fcf3; continue;
 }
 0x105fcf3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fd00; continue; }
  pc = 0x105fcfb; continue;
 }
 0x105fcfb => {
  pc = 0x1061f2d; continue;
 }
 0x105fd00 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fd11; continue;
 }
 0x105fd11 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fd27; continue; }
  pc = 0x105fd1a; continue;
 }
 0x105fd1a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fd27; continue; }
  pc = 0x105fd22; continue;
 }
 0x105fd22 => {
  pc = 0x1061f2d; continue;
 }
 0x105fd27 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fd38; continue;
 }
 0x105fd38 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fd4e; continue; }
  pc = 0x105fd41; continue;
 }
 0x105fd41 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fd4e; continue; }
  pc = 0x105fd49; continue;
 }
 0x105fd49 => {
  pc = 0x1061f2d; continue;
 }
 0x105fd4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fd5f; continue;
 }
 0x105fd5f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fd75; continue; }
  pc = 0x105fd68; continue;
 }
 0x105fd68 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fd75; continue; }
  pc = 0x105fd70; continue;
 }
 0x105fd70 => {
  pc = 0x1061f2d; continue;
 }
 0x105fd75 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fd86; continue;
 }
 0x105fd86 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fd9c; continue; }
  pc = 0x105fd8f; continue;
 }
 0x105fd8f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fd9c; continue; }
  pc = 0x105fd97; continue;
 }
 0x105fd97 => {
  pc = 0x1061f2d; continue;
 }
 0x105fd9c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fdad; continue;
 }
 0x105fdad => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fdc3; continue; }
  pc = 0x105fdb6; continue;
 }
 0x105fdb6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fdc3; continue; }
  pc = 0x105fdbe; continue;
 }
 0x105fdbe => {
  pc = 0x1061f2d; continue;
 }
 0x105fdc3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fdd4; continue;
 }
 0x105fdd4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fdea; continue; }
  pc = 0x105fddd; continue;
 }
 0x105fddd => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fdea; continue; }
  pc = 0x105fde5; continue;
 }
 0x105fde5 => {
  pc = 0x1061f2d; continue;
 }
 0x105fdea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fdfb; continue;
 }
 0x105fdfb => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fe11; continue; }
  pc = 0x105fe04; continue;
 }
 0x105fe04 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fe11; continue; }
  pc = 0x105fe0c; continue;
 }
 0x105fe0c => {
  pc = 0x1061f2d; continue;
 }
 0x105fe11 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fe22; continue;
 }
 0x105fe22 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fe38; continue; }
  pc = 0x105fe2b; continue;
 }
 0x105fe2b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fe38; continue; }
  pc = 0x105fe33; continue;
 }
 0x105fe33 => {
  pc = 0x1061f2d; continue;
 }
 0x105fe38 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fe49; continue;
 }
 0x105fe49 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fe5f; continue; }
  pc = 0x105fe52; continue;
 }
 0x105fe52 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fe5f; continue; }
  pc = 0x105fe5a; continue;
 }
 0x105fe5a => {
  pc = 0x1061f2d; continue;
 }
 0x105fe5f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fe70; continue;
 }
 0x105fe70 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fe86; continue; }
  pc = 0x105fe79; continue;
 }
 0x105fe79 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fe86; continue; }
  pc = 0x105fe81; continue;
 }
 0x105fe81 => {
  pc = 0x1061f2d; continue;
 }
 0x105fe86 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fe97; continue;
 }
 0x105fe97 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fead; continue; }
  pc = 0x105fea0; continue;
 }
 0x105fea0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fead; continue; }
  pc = 0x105fea8; continue;
 }
 0x105fea8 => {
  pc = 0x1061f2d; continue;
 }
 0x105fead => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105febe; continue;
 }
 0x105febe => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fed4; continue; }
  pc = 0x105fec7; continue;
 }
 0x105fec7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fed4; continue; }
  pc = 0x105fecf; continue;
 }
 0x105fecf => {
  pc = 0x1061f2d; continue;
 }
 0x105fed4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fee5; continue;
 }
 0x105fee5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105fefb; continue; }
  pc = 0x105feee; continue;
 }
 0x105feee => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105fefb; continue; }
  pc = 0x105fef6; continue;
 }
 0x105fef6 => {
  pc = 0x1061f2d; continue;
 }
 0x105fefb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ff0c; continue;
 }
 0x105ff0c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ff22; continue; }
  pc = 0x105ff15; continue;
 }
 0x105ff15 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ff22; continue; }
  pc = 0x105ff1d; continue;
 }
 0x105ff1d => {
  pc = 0x1061f2d; continue;
 }
 0x105ff22 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ff33; continue;
 }
 0x105ff33 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ff49; continue; }
  pc = 0x105ff3c; continue;
 }
 0x105ff3c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ff49; continue; }
  pc = 0x105ff44; continue;
 }
 0x105ff44 => {
  pc = 0x1061f2d; continue;
 }
 0x105ff49 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ff5a; continue;
 }
 0x105ff5a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ff70; continue; }
  pc = 0x105ff63; continue;
 }
 0x105ff63 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ff70; continue; }
  pc = 0x105ff6b; continue;
 }
 0x105ff6b => {
  pc = 0x1061f2d; continue;
 }
 0x105ff70 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ff81; continue;
 }
 0x105ff81 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ff97; continue; }
  pc = 0x105ff8a; continue;
 }
 0x105ff8a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ff97; continue; }
  pc = 0x105ff92; continue;
 }
 0x105ff92 => {
  pc = 0x1061f2d; continue;
 }
 0x105ff97 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ffa8; continue;
 }
 0x105ffa8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ffbe; continue; }
  pc = 0x105ffb1; continue;
 }
 0x105ffb1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ffbe; continue; }
  pc = 0x105ffb9; continue;
 }
 0x105ffb9 => {
  pc = 0x1061f2d; continue;
 }
 0x105ffbe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105ffcf; continue;
 }
 0x105ffcf => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x105ffe5; continue; }
  pc = 0x105ffd8; continue;
 }
 0x105ffd8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x105ffe5; continue; }
  pc = 0x105ffe0; continue;
 }
 0x105ffe0 => {
  pc = 0x1061f2d; continue;
 }
 0x105ffe5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x105fff6; continue;
 }
 0x105fff6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106000c; continue; }
  pc = 0x105ffff; continue;
 }
 0x105ffff => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106000c; continue; }
  pc = 0x1060007; continue;
 }
 0x1060000 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10ee96au32); cpu.eax = v; }
  if !cpu.cf { pc = 0x106000c; continue; }
  pc = 0x1060007; continue;
 }
 0x1060007 => {
  pc = 0x1061f2d; continue;
 }
 0x106000c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106001d; continue;
 }
 0x106001d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060033; continue; }
  pc = 0x1060026; continue;
 }
 0x1060026 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060033; continue; }
  pc = 0x106002e; continue;
 }
 0x106002e => {
  pc = 0x1061f2d; continue;
 }
 0x1060033 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060044; continue;
 }
 0x1060044 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106005a; continue; }
  pc = 0x106004d; continue;
 }
 0x106004d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106005a; continue; }
  pc = 0x1060055; continue;
 }
 0x1060055 => {
  pc = 0x1061f2d; continue;
 }
 0x106005a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106006b; continue;
 }
 0x106006b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060081; continue; }
  pc = 0x1060074; continue;
 }
 0x1060074 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060081; continue; }
  pc = 0x106007c; continue;
 }
 0x106007c => {
  pc = 0x1061f2d; continue;
 }
 0x1060081 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060092; continue;
 }
 0x1060092 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10600a8; continue; }
  pc = 0x106009b; continue;
 }
 0x106009b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10600a8; continue; }
  pc = 0x10600a3; continue;
 }
 0x10600a3 => {
  pc = 0x1061f2d; continue;
 }
 0x10600a8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10600b9; continue;
 }
 0x10600b9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10600cf; continue; }
  pc = 0x10600c2; continue;
 }
 0x10600c2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10600cf; continue; }
  pc = 0x10600ca; continue;
 }
 0x10600ca => {
  pc = 0x1061f2d; continue;
 }
 0x10600cf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10600e0; continue;
 }
 0x10600e0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10600f6; continue; }
  pc = 0x10600e9; continue;
 }
 0x10600e9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10600f6; continue; }
  pc = 0x10600f1; continue;
 }
 0x10600f1 => {
  pc = 0x1061f2d; continue;
 }
 0x10600f6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060107; continue;
 }
 0x1060105 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1060107; continue;
 }
 0x1060107 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106011d; continue; }
  pc = 0x1060110; continue;
 }
 0x1060110 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106011d; continue; }
  pc = 0x1060118; continue;
 }
 0x1060118 => {
  pc = 0x1061f2d; continue;
 }
 0x106011d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106012e; continue;
 }
 0x106012e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060144; continue; }
  pc = 0x1060137; continue;
 }
 0x1060137 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060144; continue; }
  pc = 0x106013f; continue;
 }
 0x106013f => {
  pc = 0x1061f2d; continue;
 }
 0x1060144 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060155; continue;
 }
 0x1060155 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106016b; continue; }
  pc = 0x106015e; continue;
 }
 0x106015e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106016b; continue; }
  pc = 0x1060166; continue;
 }
 0x1060166 => {
  pc = 0x1061f2d; continue;
 }
 0x106016b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106017c; continue;
 }
 0x106017c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060192; continue; }
  pc = 0x1060185; continue;
 }
 0x1060185 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060192; continue; }
  pc = 0x106018d; continue;
 }
 0x106018d => {
  pc = 0x1061f2d; continue;
 }
 0x1060192 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10601a3; continue;
 }
 0x10601a3 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10601b9; continue; }
  pc = 0x10601ac; continue;
 }
 0x10601ac => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10601b9; continue; }
  pc = 0x10601b4; continue;
 }
 0x10601b4 => {
  pc = 0x1061f2d; continue;
 }
 0x10601b9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10601ca; continue;
 }
 0x10601ca => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10601e0; continue; }
  pc = 0x10601d3; continue;
 }
 0x10601d3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10601e0; continue; }
  pc = 0x10601db; continue;
 }
 0x10601db => {
  pc = 0x1061f2d; continue;
 }
 0x10601e0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10601f1; continue;
 }
 0x10601f1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060207; continue; }
  pc = 0x10601fa; continue;
 }
 0x10601fa => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060207; continue; }
  pc = 0x1060202; continue;
 }
 0x1060202 => {
  pc = 0x1061f2d; continue;
 }
 0x1060207 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060218; continue;
 }
 0x1060218 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106022e; continue; }
  pc = 0x1060221; continue;
 }
 0x1060221 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106022e; continue; }
  pc = 0x1060229; continue;
 }
 0x1060229 => {
  pc = 0x1061f2d; continue;
 }
 0x106022e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106023f; continue;
 }
 0x106023f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  pc = 0x1060246; continue;
 }
 0x1060246 => {
  if cpu.zf { pc = 0x1060255; continue; }
  pc = 0x1060248; continue;
 }
 0x1060248 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060255; continue; }
  pc = 0x1060250; continue;
 }
 0x1060250 => {
  pc = 0x1061f2d; continue;
 }
 0x1060255 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060266; continue;
 }
 0x1060264 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1060266; continue;
 }
 0x1060266 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106027c; continue; }
  pc = 0x106026f; continue;
 }
 0x106026f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106027c; continue; }
  pc = 0x1060277; continue;
 }
 0x1060277 => {
  pc = 0x1061f2d; continue;
 }
 0x106027c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106028d; continue;
 }
 0x106028d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10602a3; continue; }
  pc = 0x1060296; continue;
 }
 0x1060296 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10602a3; continue; }
  pc = 0x106029e; continue;
 }
 0x106029e => {
  pc = 0x1061f2d; continue;
 }
 0x10602a3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10602b4; continue;
 }
 0x10602b4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10602ca; continue; }
  pc = 0x10602bd; continue;
 }
 0x10602bd => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10602ca; continue; }
  pc = 0x10602c5; continue;
 }
 0x10602c5 => {
  pc = 0x1061f2d; continue;
 }
 0x10602ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10602db; continue;
 }
 0x10602db => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10602f1; continue; }
  pc = 0x10602e4; continue;
 }
 0x10602e4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10602f1; continue; }
  pc = 0x10602ec; continue;
 }
 0x10602ec => {
  pc = 0x1061f2d; continue;
 }
 0x10602f1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060302; continue;
 }
 0x1060302 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060318; continue; }
  pc = 0x106030b; continue;
 }
 0x106030b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060318; continue; }
  pc = 0x1060313; continue;
 }
 0x1060313 => {
  pc = 0x1061f2d; continue;
 }
 0x1060318 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060329; continue;
 }
 0x1060329 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106033f; continue; }
  pc = 0x1060332; continue;
 }
 0x1060332 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106033f; continue; }
  pc = 0x106033a; continue;
 }
 0x106033a => {
  pc = 0x1061f2d; continue;
 }
 0x106033f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060350; continue;
 }
 0x1060350 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060366; continue; }
  pc = 0x1060359; continue;
 }
 0x1060359 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060366; continue; }
  pc = 0x1060361; continue;
 }
 0x1060361 => {
  pc = 0x1061f2d; continue;
 }
 0x1060366 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060377; continue;
 }
 0x1060377 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106038d; continue; }
  pc = 0x1060380; continue;
 }
 0x1060380 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106038d; continue; }
  pc = 0x1060388; continue;
 }
 0x1060388 => {
  pc = 0x1061f2d; continue;
 }
 0x106038d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106039e; continue;
 }
 0x106039e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10603b4; continue; }
  pc = 0x10603a7; continue;
 }
 0x10603a7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10603b4; continue; }
  pc = 0x10603af; continue;
 }
 0x10603af => {
  pc = 0x1061f2d; continue;
 }
 0x10603b4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10603c5; continue;
 }
 0x10603c5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10603db; continue; }
  pc = 0x10603ce; continue;
 }
 0x10603ce => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10603db; continue; }
  pc = 0x10603d6; continue;
 }
 0x10603d6 => {
  pc = 0x1061f2d; continue;
 }
 0x10603db => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10603ec; continue;
 }
 0x10603ec => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060402; continue; }
  pc = 0x10603f5; continue;
 }
 0x10603f5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060402; continue; }
  pc = 0x10603fd; continue;
 }
 0x10603fd => {
  pc = 0x1061f2d; continue;
 }
 0x1060402 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060413; continue;
 }
 0x1060413 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060429; continue; }
  pc = 0x106041c; continue;
 }
 0x106041c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060429; continue; }
  pc = 0x1060424; continue;
 }
 0x1060424 => {
  pc = 0x1061f2d; continue;
 }
 0x1060429 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106043a; continue;
 }
 0x106043a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060450; continue; }
  pc = 0x1060443; continue;
 }
 0x1060443 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060450; continue; }
  pc = 0x106044b; continue;
 }
 0x106044b => {
  pc = 0x1061f2d; continue;
 }
 0x1060450 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060461; continue;
 }
 0x1060461 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060477; continue; }
  pc = 0x106046a; continue;
 }
 0x106046a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060477; continue; }
  pc = 0x1060472; continue;
 }
 0x1060472 => {
  pc = 0x1061f2d; continue;
 }
 0x1060477 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060488; continue;
 }
 0x1060488 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106049e; continue; }
  pc = 0x1060491; continue;
 }
 0x1060491 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106049e; continue; }
  pc = 0x1060499; continue;
 }
 0x1060499 => {
  pc = 0x1061f2d; continue;
 }
 0x106049e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10604af; continue;
 }
 0x10604af => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10604c5; continue; }
  pc = 0x10604b8; continue;
 }
 0x10604b8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10604c5; continue; }
  pc = 0x10604c0; continue;
 }
 0x10604c0 => {
  pc = 0x1061f2d; continue;
 }
 0x10604c5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10604d6; continue;
 }
 0x10604d6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10604ec; continue; }
  pc = 0x10604df; continue;
 }
 0x10604df => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10604ec; continue; }
  pc = 0x10604e7; continue;
 }
 0x10604e7 => {
  pc = 0x1061f2d; continue;
 }
 0x10604ec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10604fd; continue;
 }
 0x10604fd => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060513; continue; }
  pc = 0x1060506; continue;
 }
 0x1060506 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060513; continue; }
  pc = 0x106050e; continue;
 }
 0x106050e => {
  pc = 0x1061f2d; continue;
 }
 0x1060513 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060524; continue;
 }
 0x1060524 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106053a; continue; }
  pc = 0x106052d; continue;
 }
 0x106052d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106053a; continue; }
  pc = 0x1060535; continue;
 }
 0x1060535 => {
  pc = 0x1061f2d; continue;
 }
 0x106053a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106054b; continue;
 }
 0x106054b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060561; continue; }
  pc = 0x1060554; continue;
 }
 0x1060554 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060561; continue; }
  pc = 0x106055c; continue;
 }
 0x106055c => {
  pc = 0x1061f2d; continue;
 }
 0x1060561 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060572; continue;
 }
 0x1060572 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060588; continue; }
  pc = 0x106057b; continue;
 }
 0x106057b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060588; continue; }
  pc = 0x1060583; continue;
 }
 0x1060583 => {
  pc = 0x1061f2d; continue;
 }
 0x1060588 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060599; continue;
 }
 0x1060599 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10605af; continue; }
  pc = 0x10605a2; continue;
 }
 0x10605a2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10605af; continue; }
  pc = 0x10605aa; continue;
 }
 0x10605aa => {
  pc = 0x1061f2d; continue;
 }
 0x10605af => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10605c0; continue;
 }
 0x10605c0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10605d6; continue; }
  pc = 0x10605c9; continue;
 }
 0x10605c9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10605d6; continue; }
  pc = 0x10605d1; continue;
 }
 0x10605d1 => {
  pc = 0x1061f2d; continue;
 }
 0x10605d6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10605e7; continue;
 }
 0x10605e7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10605fd; continue; }
  pc = 0x10605f0; continue;
 }
 0x10605f0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10605fd; continue; }
  pc = 0x10605f8; continue;
 }
 0x10605f8 => {
  pc = 0x1061f2d; continue;
 }
 0x10605fd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1060601; continue;
 }
 0x1060601 => {
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106060e; continue;
 }
 0x1060606 => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xfu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x80000019u32, v); }
  { cpu.sub32(cpu.eax, 0x11c62f9u32); }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3au32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10ee96au32); cpu.eax = v; }
  if !cpu.cf { pc = 0x1060624; continue; }
  pc = 0x106061f; continue;
 }
 0x106060c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x106060e; continue;
 }
 0x106060e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060624; continue; }
  pc = 0x1060617; continue;
 }
 0x1060617 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060624; continue; }
  pc = 0x106061f; continue;
 }
 0x106061f => {
  pc = 0x1061f2d; continue;
 }
 0x1060620 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1060624; continue;
 }
 0x1060624 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060635; continue;
 }
 0x1060635 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106064b; continue; }
  pc = 0x106063e; continue;
 }
 0x106063e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106064b; continue; }
  pc = 0x1060646; continue;
 }
 0x1060646 => {
  pc = 0x1061f2d; continue;
 }
 0x106064b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106065c; continue;
 }
 0x106065c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060672; continue; }
  pc = 0x1060665; continue;
 }
 0x1060665 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060672; continue; }
  pc = 0x106066d; continue;
 }
 0x106066d => {
  pc = 0x1061f2d; continue;
 }
 0x1060672 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060683; continue;
 }
 0x1060683 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060699; continue; }
  pc = 0x106068c; continue;
 }
 0x106068c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060699; continue; }
  pc = 0x1060694; continue;
 }
 0x1060694 => {
  pc = 0x1061f2d; continue;
 }
 0x1060699 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10606aa; continue;
 }
 0x10606aa => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10606c0; continue; }
  pc = 0x10606b3; continue;
 }
 0x10606b3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10606c0; continue; }
  pc = 0x10606bb; continue;
 }
 0x10606bb => {
  pc = 0x1061f2d; continue;
 }
 0x10606c0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10606d1; continue;
 }
 0x10606d1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10606e7; continue; }
  pc = 0x10606da; continue;
 }
 0x10606da => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10606e7; continue; }
  pc = 0x10606e2; continue;
 }
 0x10606e2 => {
  pc = 0x1061f2d; continue;
 }
 0x10606e7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10606f8; continue;
 }
 0x10606f8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106070e; continue; }
  pc = 0x1060701; continue;
 }
 0x10606fa => {
  { cpu.cf = true; }
  cpu.unimplemented(0x10606fb, "Bound");
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3au32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10ee96au32); cpu.eax = v; }
  if !cpu.cf { pc = 0x106070e; continue; }
  pc = 0x1060709; continue;
 }
 0x1060701 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106070e; continue; }
  pc = 0x1060709; continue;
 }
 0x1060709 => {
  pc = 0x1061f2d; continue;
 }
 0x106070e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106071f; continue;
 }
 0x106071f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060735; continue; }
  pc = 0x1060728; continue;
 }
 0x1060728 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060735; continue; }
  pc = 0x1060730; continue;
 }
 0x1060730 => {
  pc = 0x1061f2d; continue;
 }
 0x1060735 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060746; continue;
 }
 0x1060746 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106075c; continue; }
  pc = 0x106074f; continue;
 }
 0x106074f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106075c; continue; }
  pc = 0x1060757; continue;
 }
 0x1060757 => {
  pc = 0x1061f2d; continue;
 }
 0x106075c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106076d; continue;
 }
 0x106076d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060783; continue; }
  pc = 0x1060776; continue;
 }
 0x1060776 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060783; continue; }
  pc = 0x106077e; continue;
 }
 0x106077e => {
  pc = 0x1061f2d; continue;
 }
 0x1060783 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060794; continue;
 }
 0x1060794 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10607aa; continue; }
  pc = 0x106079d; continue;
 }
 0x106079d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10607aa; continue; }
  pc = 0x10607a5; continue;
 }
 0x10607a5 => {
  pc = 0x1061f2d; continue;
 }
 0x10607aa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10607bb; continue;
 }
 0x10607bb => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10607d1; continue; }
  pc = 0x10607c4; continue;
 }
 0x10607c4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10607d1; continue; }
  pc = 0x10607cc; continue;
 }
 0x10607cc => {
  pc = 0x1061f2d; continue;
 }
 0x10607d1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10607e2; continue;
 }
 0x10607e2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10607f8; continue; }
  pc = 0x10607eb; continue;
 }
 0x10607eb => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10607f8; continue; }
  pc = 0x10607f3; continue;
 }
 0x10607f3 => {
  pc = 0x1061f2d; continue;
 }
 0x10607f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060809; continue;
 }
 0x1060809 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106081f; continue; }
  pc = 0x1060812; continue;
 }
 0x1060812 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106081f; continue; }
  pc = 0x106081a; continue;
 }
 0x106081a => {
  pc = 0x1061f2d; continue;
 }
 0x106081f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060830; continue;
 }
 0x1060830 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060846; continue; }
  pc = 0x1060839; continue;
 }
 0x1060839 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060846; continue; }
  pc = 0x1060841; continue;
 }
 0x1060841 => {
  pc = 0x1061f2d; continue;
 }
 0x1060846 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060857; continue;
 }
 0x1060857 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106086d; continue; }
  pc = 0x1060860; continue;
 }
 0x1060860 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106086d; continue; }
  pc = 0x1060868; continue;
 }
 0x1060868 => {
  pc = 0x1061f2d; continue;
 }
 0x106086d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106087e; continue;
 }
 0x106087e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060894; continue; }
  pc = 0x1060887; continue;
 }
 0x1060887 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060894; continue; }
  pc = 0x106088f; continue;
 }
 0x106088f => {
  pc = 0x1061f2d; continue;
 }
 0x1060894 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10608a5; continue;
 }
 0x10608a5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10608bb; continue; }
  pc = 0x10608ae; continue;
 }
 0x10608ae => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10608bb; continue; }
  pc = 0x10608b6; continue;
 }
 0x10608b6 => {
  pc = 0x1061f2d; continue;
 }
 0x10608bb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10608cc; continue;
 }
 0x10608cc => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10608e2; continue; }
  pc = 0x10608d5; continue;
 }
 0x10608d5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10608e2; continue; }
  pc = 0x10608dd; continue;
 }
 0x10608dd => {
  pc = 0x1061f2d; continue;
 }
 0x10608e2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10608f3; continue;
 }
 0x10608f3 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060909; continue; }
  pc = 0x10608fc; continue;
 }
 0x10608fc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060909; continue; }
  pc = 0x1060904; continue;
 }
 0x1060904 => {
  pc = 0x1061f2d; continue;
 }
 0x1060909 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106091a; continue;
 }
 0x106091a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060930; continue; }
  pc = 0x1060923; continue;
 }
 0x1060923 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060930; continue; }
  pc = 0x106092b; continue;
 }
 0x106092b => {
  pc = 0x1061f2d; continue;
 }
 0x1060930 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060941; continue;
 }
 0x1060941 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060957; continue; }
  pc = 0x106094a; continue;
 }
 0x106094a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060957; continue; }
  pc = 0x1060952; continue;
 }
 0x1060952 => {
  pc = 0x1061f2d; continue;
 }
 0x1060957 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060968; continue;
 }
 0x1060968 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106097e; continue; }
  pc = 0x1060971; continue;
 }
 0x1060971 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106097e; continue; }
  pc = 0x1060979; continue;
 }
 0x1060979 => {
  pc = 0x1061f2d; continue;
 }
 0x106097e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106098f; continue;
 }
 0x106098f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10609a5; continue; }
  pc = 0x1060998; continue;
 }
 0x1060998 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10609a5; continue; }
  pc = 0x10609a0; continue;
 }
 0x10609a0 => {
  pc = 0x1061f2d; continue;
 }
 0x10609a5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10609b6; continue;
 }
 0x10609b6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10609cc; continue; }
  pc = 0x10609bf; continue;
 }
 0x10609bf => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10609cc; continue; }
  pc = 0x10609c7; continue;
 }
 0x10609c7 => {
  pc = 0x1061f2d; continue;
 }
 0x10609cc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10609dd; continue;
 }
 0x10609dd => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10609f3; continue; }
  pc = 0x10609e6; continue;
 }
 0x10609e6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10609f3; continue; }
  pc = 0x10609ee; continue;
 }
 0x10609ee => {
  pc = 0x1061f2d; continue;
 }
 0x10609f3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060a04; continue;
 }
 0x1060a04 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060a1a; continue; }
  pc = 0x1060a0d; continue;
 }
 0x1060a0d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060a1a; continue; }
  pc = 0x1060a15; continue;
 }
 0x1060a15 => {
  pc = 0x1061f2d; continue;
 }
 0x1060a1a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060a2b; continue;
 }
 0x1060a2b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060a41; continue; }
  pc = 0x1060a34; continue;
 }
 0x1060a34 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060a41; continue; }
  pc = 0x1060a3c; continue;
 }
 0x1060a3c => {
  pc = 0x1061f2d; continue;
 }
 0x1060a41 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060a52; continue;
 }
 0x1060a52 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060a68; continue; }
  pc = 0x1060a5b; continue;
 }
 0x1060a5b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060a68; continue; }
  pc = 0x1060a63; continue;
 }
 0x1060a63 => {
  pc = 0x1061f2d; continue;
 }
 0x1060a68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060a79; continue;
 }
 0x1060a79 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060a8f; continue; }
  pc = 0x1060a82; continue;
 }
 0x1060a82 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060a8f; continue; }
  pc = 0x1060a8a; continue;
 }
 0x1060a8a => {
  pc = 0x1061f2d; continue;
 }
 0x1060a8f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060aa0; continue;
 }
 0x1060aa0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060ab6; continue; }
  pc = 0x1060aa9; continue;
 }
 0x1060aa9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060ab6; continue; }
  pc = 0x1060ab1; continue;
 }
 0x1060ab1 => {
  pc = 0x1061f2d; continue;
 }
 0x1060ab6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060ac7; continue;
 }
 0x1060ac7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060add; continue; }
  pc = 0x1060ad0; continue;
 }
 0x1060ad0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060add; continue; }
  pc = 0x1060ad8; continue;
 }
 0x1060ad8 => {
  pc = 0x1061f2d; continue;
 }
 0x1060add => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060aee; continue;
 }
 0x1060aee => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060b04; continue; }
  pc = 0x1060af7; continue;
 }
 0x1060af7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060b04; continue; }
  pc = 0x1060aff; continue;
 }
 0x1060aff => {
  pc = 0x1061f2d; continue;
 }
 0x1060b04 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060b15; continue;
 }
 0x1060b15 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060b2b; continue; }
  pc = 0x1060b1e; continue;
 }
 0x1060b1e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060b2b; continue; }
  pc = 0x1060b26; continue;
 }
 0x1060b26 => {
  pc = 0x1061f2d; continue;
 }
 0x1060b2b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060b3c; continue;
 }
 0x1060b3c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060b52; continue; }
  pc = 0x1060b45; continue;
 }
 0x1060b45 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060b52; continue; }
  pc = 0x1060b4d; continue;
 }
 0x1060b4d => {
  pc = 0x1061f2d; continue;
 }
 0x1060b52 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060b63; continue;
 }
 0x1060b63 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060b79; continue; }
  pc = 0x1060b6c; continue;
 }
 0x1060b6c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060b79; continue; }
  pc = 0x1060b74; continue;
 }
 0x1060b74 => {
  pc = 0x1061f2d; continue;
 }
 0x1060b79 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060b8a; continue;
 }
 0x1060b8a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060ba0; continue; }
  pc = 0x1060b93; continue;
 }
 0x1060b93 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060ba0; continue; }
  pc = 0x1060b9b; continue;
 }
 0x1060b9b => {
  pc = 0x1061f2d; continue;
 }
 0x1060ba0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060bb1; continue;
 }
 0x1060bb1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060bc7; continue; }
  pc = 0x1060bba; continue;
 }
 0x1060bba => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060bc7; continue; }
  pc = 0x1060bc2; continue;
 }
 0x1060bc2 => {
  pc = 0x1061f2d; continue;
 }
 0x1060bc7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060bd8; continue;
 }
 0x1060bd8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060bee; continue; }
  pc = 0x1060be1; continue;
 }
 0x1060be1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060bee; continue; }
  pc = 0x1060be9; continue;
 }
 0x1060be9 => {
  pc = 0x1061f2d; continue;
 }
 0x1060bee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060bff; continue;
 }
 0x1060bff => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060c15; continue; }
  pc = 0x1060c08; continue;
 }
 0x1060c08 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060c15; continue; }
  pc = 0x1060c10; continue;
 }
 0x1060c0c => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0x5u32.wrapping_add(cpu.ebx)), cpu.esi); cpu.w32(0x5u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1060c10; continue;
 }
 0x1060c10 => {
  pc = 0x1061f2d; continue;
 }
 0x1060c15 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060c26; continue;
 }
 0x1060c26 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060c3c; continue; }
  pc = 0x1060c2f; continue;
 }
 0x1060c2f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060c3c; continue; }
  pc = 0x1060c37; continue;
 }
 0x1060c37 => {
  pc = 0x1061f2d; continue;
 }
 0x1060c3c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060c4d; continue;
 }
 0x1060c4d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060c63; continue; }
  pc = 0x1060c56; continue;
 }
 0x1060c56 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060c63; continue; }
  pc = 0x1060c5e; continue;
 }
 0x1060c5e => {
  pc = 0x1061f2d; continue;
 }
 0x1060c63 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060c74; continue;
 }
 0x1060c74 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060c8a; continue; }
  pc = 0x1060c7d; continue;
 }
 0x1060c7d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060c8a; continue; }
  pc = 0x1060c85; continue;
 }
 0x1060c85 => {
  pc = 0x1061f2d; continue;
 }
 0x1060c8a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060c9b; continue;
 }
 0x1060c9b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060cb1; continue; }
  pc = 0x1060ca4; continue;
 }
 0x1060ca4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060cb1; continue; }
  pc = 0x1060cac; continue;
 }
 0x1060cac => {
  pc = 0x1061f2d; continue;
 }
 0x1060cb1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060cc2; continue;
 }
 0x1060cc2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060cd8; continue; }
  pc = 0x1060ccb; continue;
 }
 0x1060ccb => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060cd8; continue; }
  pc = 0x1060cd3; continue;
 }
 0x1060cd3 => {
  pc = 0x1061f2d; continue;
 }
 0x1060cd8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060ce9; continue;
 }
 0x1060ce9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060cff; continue; }
  pc = 0x1060cf2; continue;
 }
 0x1060cf2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060cff; continue; }
  pc = 0x1060cfa; continue;
 }
 0x1060cfa => {
  pc = 0x1061f2d; continue;
 }
 0x1060cff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060d10; continue;
 }
 0x1060d10 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060d26; continue; }
  pc = 0x1060d19; continue;
 }
 0x1060d19 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060d26; continue; }
  pc = 0x1060d21; continue;
 }
 0x1060d21 => {
  pc = 0x1061f2d; continue;
 }
 0x1060d26 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060d37; continue;
 }
 0x1060d37 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060d4d; continue; }
  pc = 0x1060d40; continue;
 }
 0x1060d40 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060d4d; continue; }
  pc = 0x1060d48; continue;
 }
 0x1060d48 => {
  pc = 0x1061f2d; continue;
 }
 0x1060d4d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060d5e; continue;
 }
 0x1060d5e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060d74; continue; }
  pc = 0x1060d67; continue;
 }
 0x1060d67 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060d74; continue; }
  pc = 0x1060d6f; continue;
 }
 0x1060d6f => {
  pc = 0x1061f2d; continue;
 }
 0x1060d74 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060d85; continue;
 }
 0x1060d85 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060d9b; continue; }
  pc = 0x1060d8e; continue;
 }
 0x1060d8e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060d9b; continue; }
  pc = 0x1060d96; continue;
 }
 0x1060d96 => {
  pc = 0x1061f2d; continue;
 }
 0x1060d9b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060dac; continue;
 }
 0x1060dac => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060dc2; continue; }
  pc = 0x1060db5; continue;
 }
 0x1060db5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060dc2; continue; }
  pc = 0x1060dbd; continue;
 }
 0x1060dbd => {
  pc = 0x1061f2d; continue;
 }
 0x1060dc2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060dd3; continue;
 }
 0x1060dd3 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060de9; continue; }
  pc = 0x1060ddc; continue;
 }
 0x1060ddc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060de9; continue; }
  pc = 0x1060de4; continue;
 }
 0x1060de4 => {
  pc = 0x1061f2d; continue;
 }
 0x1060de9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060dfa; continue;
 }
 0x1060dfa => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060e10; continue; }
  pc = 0x1060e03; continue;
 }
 0x1060e03 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060e10; continue; }
  pc = 0x1060e0b; continue;
 }
 0x1060e0b => {
  pc = 0x1061f2d; continue;
 }
 0x1060e10 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060e21; continue;
 }
 0x1060e21 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060e37; continue; }
  pc = 0x1060e2a; continue;
 }
 0x1060e2a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060e37; continue; }
  pc = 0x1060e32; continue;
 }
 0x1060e32 => {
  pc = 0x1061f2d; continue;
 }
 0x1060e37 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060e48; continue;
 }
 0x1060e48 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060e5e; continue; }
  pc = 0x1060e51; continue;
 }
 0x1060e51 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060e5e; continue; }
  pc = 0x1060e59; continue;
 }
 0x1060e59 => {
  pc = 0x1061f2d; continue;
 }
 0x1060e5e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060e6f; continue;
 }
 0x1060e6f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060e85; continue; }
  pc = 0x1060e78; continue;
 }
 0x1060e78 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060e85; continue; }
  pc = 0x1060e80; continue;
 }
 0x1060e80 => {
  pc = 0x1061f2d; continue;
 }
 0x1060e85 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060e96; continue;
 }
 0x1060e96 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060eac; continue; }
  pc = 0x1060e9f; continue;
 }
 0x1060e9f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060eac; continue; }
  pc = 0x1060ea7; continue;
 }
 0x1060ea7 => {
  pc = 0x1061f2d; continue;
 }
 0x1060eac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060ebd; continue;
 }
 0x1060ebd => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060ed3; continue; }
  pc = 0x1060ec6; continue;
 }
 0x1060ec6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060ed3; continue; }
  pc = 0x1060ece; continue;
 }
 0x1060ece => {
  pc = 0x1061f2d; continue;
 }
 0x1060ed3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060ee4; continue;
 }
 0x1060ee4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060efa; continue; }
  pc = 0x1060eed; continue;
 }
 0x1060eed => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060efa; continue; }
  pc = 0x1060ef5; continue;
 }
 0x1060ef5 => {
  pc = 0x1061f2d; continue;
 }
 0x1060efa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060f0b; continue;
 }
 0x1060f0b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060f21; continue; }
  pc = 0x1060f14; continue;
 }
 0x1060f14 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060f21; continue; }
  pc = 0x1060f1c; continue;
 }
 0x1060f1c => {
  pc = 0x1061f2d; continue;
 }
 0x1060f21 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060f32; continue;
 }
 0x1060f32 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060f48; continue; }
  pc = 0x1060f3b; continue;
 }
 0x1060f3b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060f48; continue; }
  pc = 0x1060f43; continue;
 }
 0x1060f43 => {
  pc = 0x1061f2d; continue;
 }
 0x1060f48 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060f59; continue;
 }
 0x1060f59 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060f6f; continue; }
  pc = 0x1060f62; continue;
 }
 0x1060f62 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060f6f; continue; }
  pc = 0x1060f6a; continue;
 }
 0x1060f6a => {
  pc = 0x1061f2d; continue;
 }
 0x1060f6f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060f80; continue;
 }
 0x1060f80 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060f96; continue; }
  pc = 0x1060f89; continue;
 }
 0x1060f89 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060f96; continue; }
  pc = 0x1060f91; continue;
 }
 0x1060f91 => {
  pc = 0x1061f2d; continue;
 }
 0x1060f96 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060fa7; continue;
 }
 0x1060fa7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060fbd; continue; }
  pc = 0x1060fb0; continue;
 }
 0x1060fb0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060fbd; continue; }
  pc = 0x1060fb8; continue;
 }
 0x1060fb8 => {
  pc = 0x1061f2d; continue;
 }
 0x1060fbd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060fce; continue;
 }
 0x1060fce => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1060fe4; continue; }
  pc = 0x1060fd7; continue;
 }
 0x1060fd7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1060fe4; continue; }
  pc = 0x1060fdf; continue;
 }
 0x1060fdf => {
  pc = 0x1061f2d; continue;
 }
 0x1060fe4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1060ff5; continue;
 }
 0x1060ff5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106100b; continue; }
  pc = 0x1060ffe; continue;
 }
 0x1060ffe => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106100b; continue; }
  pc = 0x1061006; continue;
 }
 0x1061006 => {
  pc = 0x1061f2d; continue;
 }
 0x106100b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106101c; continue;
 }
 0x106101c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061032; continue; }
  pc = 0x1061025; continue;
 }
 0x1061025 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061032; continue; }
  pc = 0x106102d; continue;
 }
 0x106102d => {
  pc = 0x1061f2d; continue;
 }
 0x1061032 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061043; continue;
 }
 0x1061043 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061059; continue; }
  pc = 0x106104c; continue;
 }
 0x106104c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061059; continue; }
  pc = 0x1061054; continue;
 }
 0x1061054 => {
  pc = 0x1061f2d; continue;
 }
 0x1061059 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106106a; continue;
 }
 0x106106a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061080; continue; }
  pc = 0x1061073; continue;
 }
 0x1061073 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061080; continue; }
  pc = 0x106107b; continue;
 }
 0x106107b => {
  pc = 0x1061f2d; continue;
 }
 0x1061080 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061091; continue;
 }
 0x1061091 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10610a7; continue; }
  pc = 0x106109a; continue;
 }
 0x106109a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10610a7; continue; }
  pc = 0x10610a2; continue;
 }
 0x10610a2 => {
  pc = 0x1061f2d; continue;
 }
 0x10610a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10610b8; continue;
 }
 0x10610b8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10610ce; continue; }
  pc = 0x10610c1; continue;
 }
 0x10610c1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10610ce; continue; }
  pc = 0x10610c9; continue;
 }
 0x10610c9 => {
  pc = 0x1061f2d; continue;
 }
 0x10610ce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10610df; continue;
 }
 0x10610df => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10610f5; continue; }
  pc = 0x10610e8; continue;
 }
 0x10610e8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10610f5; continue; }
  pc = 0x10610f0; continue;
 }
 0x10610f0 => {
  pc = 0x1061f2d; continue;
 }
 0x10610f5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061106; continue;
 }
 0x1061106 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106111c; continue; }
  pc = 0x106110f; continue;
 }
 0x106110f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106111c; continue; }
  pc = 0x1061117; continue;
 }
 0x1061117 => {
  pc = 0x1061f2d; continue;
 }
 0x106111c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106112d; continue;
 }
 0x106112d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061143; continue; }
  pc = 0x1061136; continue;
 }
 0x1061136 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061143; continue; }
  pc = 0x106113e; continue;
 }
 0x106113e => {
  pc = 0x1061f2d; continue;
 }
 0x1061143 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061154; continue;
 }
 0x1061154 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106116a; continue; }
  pc = 0x106115d; continue;
 }
 0x106115d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106116a; continue; }
  pc = 0x1061165; continue;
 }
 0x1061165 => {
  pc = 0x1061f2d; continue;
 }
 0x106116a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106117b; continue;
 }
 0x106117b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061191; continue; }
  pc = 0x1061184; continue;
 }
 0x1061184 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061191; continue; }
  pc = 0x106118c; continue;
 }
 0x106118c => {
  pc = 0x1061f2d; continue;
 }
 0x1061191 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10611a2; continue;
 }
 0x10611a2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10611b8; continue; }
  pc = 0x10611ab; continue;
 }
 0x10611ab => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10611b8; continue; }
  pc = 0x10611b3; continue;
 }
 0x10611b3 => {
  pc = 0x1061f2d; continue;
 }
 0x10611b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10611c9; continue;
 }
 0x10611c9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10611df; continue; }
  pc = 0x10611d2; continue;
 }
 0x10611d2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10611df; continue; }
  pc = 0x10611da; continue;
 }
 0x10611da => {
  pc = 0x1061f2d; continue;
 }
 0x10611df => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10611f0; continue;
 }
 0x10611f0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061206; continue; }
  pc = 0x10611f9; continue;
 }
 0x10611f9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061206; continue; }
  pc = 0x1061201; continue;
 }
 0x1061201 => {
  pc = 0x1061f2d; continue;
 }
 0x1061206 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061217; continue;
 }
 0x1061217 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106122d; continue; }
  pc = 0x1061220; continue;
 }
 0x1061220 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106122d; continue; }
  pc = 0x1061228; continue;
 }
 0x1061228 => {
  pc = 0x1061f2d; continue;
 }
 0x106122d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106123e; continue;
 }
 0x106123e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061254; continue; }
  pc = 0x1061247; continue;
 }
 0x1061247 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061254; continue; }
  pc = 0x106124f; continue;
 }
 0x106124f => {
  pc = 0x1061f2d; continue;
 }
 0x1061254 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061265; continue;
 }
 0x1061265 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106127b; continue; }
  pc = 0x106126e; continue;
 }
 0x106126e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106127b; continue; }
  pc = 0x1061276; continue;
 }
 0x1061276 => {
  pc = 0x1061f2d; continue;
 }
 0x106127b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106128c; continue;
 }
 0x106128c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10612a2; continue; }
  pc = 0x1061295; continue;
 }
 0x1061295 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10612a2; continue; }
  pc = 0x106129d; continue;
 }
 0x106129d => {
  pc = 0x1061f2d; continue;
 }
 0x10612a2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10612b3; continue;
 }
 0x10612b3 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10612c9; continue; }
  pc = 0x10612bc; continue;
 }
 0x10612bc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10612c9; continue; }
  pc = 0x10612c4; continue;
 }
 0x10612c4 => {
  pc = 0x1061f2d; continue;
 }
 0x10612c9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10612da; continue;
 }
 0x10612da => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10612f0; continue; }
  pc = 0x10612e3; continue;
 }
 0x10612e3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10612f0; continue; }
  pc = 0x10612eb; continue;
 }
 0x10612eb => {
  pc = 0x1061f2d; continue;
 }
 0x10612f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061301; continue;
 }
 0x1061301 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061317; continue; }
  pc = 0x106130a; continue;
 }
 0x106130a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061317; continue; }
  pc = 0x1061312; continue;
 }
 0x1061312 => {
  pc = 0x1061f2d; continue;
 }
 0x1061317 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061328; continue;
 }
 0x1061328 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106133e; continue; }
  pc = 0x1061331; continue;
 }
 0x1061331 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106133e; continue; }
  pc = 0x1061339; continue;
 }
 0x1061339 => {
  pc = 0x1061f2d; continue;
 }
 0x106133e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106134f; continue;
 }
 0x106134f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061365; continue; }
  pc = 0x1061358; continue;
 }
 0x1061358 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061365; continue; }
  pc = 0x1061360; continue;
 }
 0x1061360 => {
  pc = 0x1061f2d; continue;
 }
 0x1061365 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061376; continue;
 }
 0x1061376 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106138c; continue; }
  pc = 0x106137f; continue;
 }
 0x106137f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106138c; continue; }
  pc = 0x1061387; continue;
 }
 0x1061387 => {
  pc = 0x1061f2d; continue;
 }
 0x106138c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106139d; continue;
 }
 0x106139d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10613b3; continue; }
  pc = 0x10613a6; continue;
 }
 0x10613a6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10613b3; continue; }
  pc = 0x10613ae; continue;
 }
 0x10613ae => {
  pc = 0x1061f2d; continue;
 }
 0x10613b3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10613c4; continue;
 }
 0x10613c4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10613da; continue; }
  pc = 0x10613cd; continue;
 }
 0x10613cd => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10613da; continue; }
  pc = 0x10613d5; continue;
 }
 0x10613d5 => {
  pc = 0x1061f2d; continue;
 }
 0x10613da => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10613eb; continue;
 }
 0x10613eb => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061401; continue; }
  pc = 0x10613f4; continue;
 }
 0x10613f4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061401; continue; }
  pc = 0x10613fc; continue;
 }
 0x10613fc => {
  pc = 0x1061f2d; continue;
 }
 0x1061401 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061412; continue;
 }
 0x1061412 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061428; continue; }
  pc = 0x106141b; continue;
 }
 0x106141b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061428; continue; }
  pc = 0x1061423; continue;
 }
 0x1061423 => {
  pc = 0x1061f2d; continue;
 }
 0x1061428 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061439; continue;
 }
 0x1061439 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106144f; continue; }
  pc = 0x1061442; continue;
 }
 0x1061442 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106144f; continue; }
  pc = 0x106144a; continue;
 }
 0x106144a => {
  pc = 0x1061f2d; continue;
 }
 0x106144f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061460; continue;
 }
 0x1061460 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061476; continue; }
  pc = 0x1061469; continue;
 }
 0x1061469 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061476; continue; }
  pc = 0x1061471; continue;
 }
 0x1061471 => {
  pc = 0x1061f2d; continue;
 }
 0x1061476 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061487; continue;
 }
 0x1061487 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106149d; continue; }
  pc = 0x1061490; continue;
 }
 0x1061490 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106149d; continue; }
  pc = 0x1061498; continue;
 }
 0x1061498 => {
  pc = 0x1061f2d; continue;
 }
 0x106149d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10614ae; continue;
 }
 0x10614ae => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10614c4; continue; }
  pc = 0x10614b7; continue;
 }
 0x10614b7 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10614c4; continue; }
  pc = 0x10614bf; continue;
 }
 0x10614bf => {
  pc = 0x1061f2d; continue;
 }
 0x10614c4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10614d5; continue;
 }
 0x10614d5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10614eb; continue; }
  pc = 0x10614de; continue;
 }
 0x10614de => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10614eb; continue; }
  pc = 0x10614e6; continue;
 }
 0x10614e6 => {
  pc = 0x1061f2d; continue;
 }
 0x10614eb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10614fc; continue;
 }
 0x10614fc => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061512; continue; }
  pc = 0x1061505; continue;
 }
 0x1061505 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061512; continue; }
  pc = 0x106150d; continue;
 }
 0x106150d => {
  pc = 0x1061f2d; continue;
 }
 0x1061512 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061523; continue;
 }
 0x1061523 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061539; continue; }
  pc = 0x106152c; continue;
 }
 0x106152c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061539; continue; }
  pc = 0x1061534; continue;
 }
 0x1061534 => {
  pc = 0x1061f2d; continue;
 }
 0x1061539 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106154a; continue;
 }
 0x106154a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061560; continue; }
  pc = 0x1061553; continue;
 }
 0x1061553 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061560; continue; }
  pc = 0x106155b; continue;
 }
 0x106155b => {
  pc = 0x1061f2d; continue;
 }
 0x1061560 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061571; continue;
 }
 0x1061571 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061587; continue; }
  pc = 0x106157a; continue;
 }
 0x106157a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061587; continue; }
  pc = 0x1061582; continue;
 }
 0x1061582 => {
  pc = 0x1061f2d; continue;
 }
 0x1061587 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061598; continue;
 }
 0x1061598 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10615ae; continue; }
  pc = 0x10615a1; continue;
 }
 0x10615a1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10615ae; continue; }
  pc = 0x10615a9; continue;
 }
 0x10615a9 => {
  pc = 0x1061f2d; continue;
 }
 0x10615ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10615bf; continue;
 }
 0x10615bf => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10615d5; continue; }
  pc = 0x10615c8; continue;
 }
 0x10615c8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10615d5; continue; }
  pc = 0x10615d0; continue;
 }
 0x10615d0 => {
  pc = 0x1061f2d; continue;
 }
 0x10615d5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10615e6; continue;
 }
 0x10615e6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10615fc; continue; }
  pc = 0x10615ef; continue;
 }
 0x10615ef => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10615fc; continue; }
  pc = 0x10615f7; continue;
 }
 0x10615f7 => {
  pc = 0x1061f2d; continue;
 }
 0x10615fc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106160d; continue;
 }
 0x106160d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061623; continue; }
  pc = 0x1061616; continue;
 }
 0x1061616 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061623; continue; }
  pc = 0x106161e; continue;
 }
 0x106161e => {
  pc = 0x1061f2d; continue;
 }
 0x1061623 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061634; continue;
 }
 0x1061634 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106164a; continue; }
  pc = 0x106163d; continue;
 }
 0x106163d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106164a; continue; }
  pc = 0x1061645; continue;
 }
 0x1061645 => {
  pc = 0x1061f2d; continue;
 }
 0x106164a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106165b; continue;
 }
 0x106165b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061671; continue; }
  pc = 0x1061664; continue;
 }
 0x1061664 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061671; continue; }
  pc = 0x106166c; continue;
 }
 0x106166c => {
  pc = 0x1061f2d; continue;
 }
 0x1061671 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061682; continue;
 }
 0x1061682 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061698; continue; }
  pc = 0x106168b; continue;
 }
 0x106168b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061698; continue; }
  pc = 0x1061693; continue;
 }
 0x1061693 => {
  pc = 0x1061f2d; continue;
 }
 0x1061698 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10616a9; continue;
 }
 0x10616a9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10616bf; continue; }
  pc = 0x10616b2; continue;
 }
 0x10616b2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10616bf; continue; }
  pc = 0x10616ba; continue;
 }
 0x10616ba => {
  pc = 0x1061f2d; continue;
 }
 0x10616bf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10616d0; continue;
 }
 0x10616d0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10616e6; continue; }
  pc = 0x10616d9; continue;
 }
 0x10616d9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10616e6; continue; }
  pc = 0x10616e1; continue;
 }
 0x10616e1 => {
  pc = 0x1061f2d; continue;
 }
 0x10616e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10616f7; continue;
 }
 0x10616f7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106170d; continue; }
  pc = 0x1061700; continue;
 }
 0x1061700 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106170d; continue; }
  pc = 0x1061708; continue;
 }
 0x1061708 => {
  pc = 0x1061f2d; continue;
 }
 0x106170d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106171e; continue;
 }
 0x106171e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061734; continue; }
  pc = 0x1061727; continue;
 }
 0x1061727 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061734; continue; }
  pc = 0x106172f; continue;
 }
 0x106172f => {
  pc = 0x1061f2d; continue;
 }
 0x1061734 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061745; continue;
 }
 0x1061745 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106175b; continue; }
  pc = 0x106174e; continue;
 }
 0x106174e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106175b; continue; }
  pc = 0x1061756; continue;
 }
 0x1061756 => {
  pc = 0x1061f2d; continue;
 }
 0x106175b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106176c; continue;
 }
 0x106176c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061782; continue; }
  pc = 0x1061775; continue;
 }
 0x1061775 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061782; continue; }
  pc = 0x106177d; continue;
 }
 0x106177d => {
  pc = 0x1061f2d; continue;
 }
 0x1061782 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061793; continue;
 }
 0x1061793 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10617a9; continue; }
  pc = 0x106179c; continue;
 }
 0x106179c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10617a9; continue; }
  pc = 0x10617a4; continue;
 }
 0x10617a4 => {
  pc = 0x1061f2d; continue;
 }
 0x10617a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10617ba; continue;
 }
 0x10617ba => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10617d0; continue; }
  pc = 0x10617c3; continue;
 }
 0x10617c3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10617d0; continue; }
  pc = 0x10617cb; continue;
 }
 0x10617cb => {
  pc = 0x1061f2d; continue;
 }
 0x10617d0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10617e1; continue;
 }
 0x10617e1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10617f7; continue; }
  pc = 0x10617ea; continue;
 }
 0x10617ea => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10617f7; continue; }
  pc = 0x10617f2; continue;
 }
 0x10617f2 => {
  pc = 0x1061f2d; continue;
 }
 0x10617f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061808; continue;
 }
 0x1061808 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106181e; continue; }
  pc = 0x1061811; continue;
 }
 0x1061811 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106181e; continue; }
  pc = 0x1061819; continue;
 }
 0x1061819 => {
  pc = 0x1061f2d; continue;
 }
 0x106181e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106182f; continue;
 }
 0x106182f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061845; continue; }
  pc = 0x1061838; continue;
 }
 0x1061838 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061845; continue; }
  pc = 0x1061840; continue;
 }
 0x1061840 => {
  pc = 0x1061f2d; continue;
 }
 0x1061845 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061856; continue;
 }
 0x1061856 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106186c; continue; }
  pc = 0x106185f; continue;
 }
 0x106185f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106186c; continue; }
  pc = 0x1061867; continue;
 }
 0x1061867 => {
  pc = 0x1061f2d; continue;
 }
 0x106186c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106187d; continue;
 }
 0x106187d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061893; continue; }
  pc = 0x1061886; continue;
 }
 0x1061886 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061893; continue; }
  pc = 0x106188e; continue;
 }
 0x106188e => {
  pc = 0x1061f2d; continue;
 }
 0x1061893 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10618a4; continue;
 }
 0x10618a4 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10618ba; continue; }
  pc = 0x10618ad; continue;
 }
 0x10618ad => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10618ba; continue; }
  pc = 0x10618b5; continue;
 }
 0x10618b5 => {
  pc = 0x1061f2d; continue;
 }
 0x10618ba => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10618cb; continue;
 }
 0x10618cb => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10618e1; continue; }
  pc = 0x10618d4; continue;
 }
 0x10618d4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10618e1; continue; }
  pc = 0x10618dc; continue;
 }
 0x10618dc => {
  pc = 0x1061f2d; continue;
 }
 0x10618e1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10618f2; continue;
 }
 0x10618f2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061908; continue; }
  pc = 0x10618fb; continue;
 }
 0x10618fb => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061908; continue; }
  pc = 0x1061903; continue;
 }
 0x1061903 => {
  pc = 0x1061f2d; continue;
 }
 0x1061908 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061919; continue;
 }
 0x1061919 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106192f; continue; }
  pc = 0x1061922; continue;
 }
 0x1061922 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106192f; continue; }
  pc = 0x106192a; continue;
 }
 0x106192a => {
  pc = 0x1061f2d; continue;
 }
 0x106192f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061940; continue;
 }
 0x1061940 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061956; continue; }
  pc = 0x1061949; continue;
 }
 0x1061949 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061956; continue; }
  pc = 0x1061951; continue;
 }
 0x1061951 => {
  pc = 0x1061f2d; continue;
 }
 0x1061956 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061967; continue;
 }
 0x1061967 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x106197d; continue; }
  pc = 0x1061970; continue;
 }
 0x1061970 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x106197d; continue; }
  pc = 0x1061978; continue;
 }
 0x1061978 => {
  pc = 0x1061f2d; continue;
 }
 0x106197d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x106198e; continue;
 }
 0x106198e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10619a4; continue; }
  pc = 0x1061997; continue;
 }
 0x1061997 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10619a4; continue; }
  pc = 0x106199f; continue;
 }
 0x106199f => {
  pc = 0x1061f2d; continue;
 }
 0x10619a4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10619b5; continue;
 }
 0x10619b5 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10619cb; continue; }
  pc = 0x10619be; continue;
 }
 0x10619be => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10619cb; continue; }
  pc = 0x10619c6; continue;
 }
 0x10619c6 => {
  pc = 0x1061f2d; continue;
 }
 0x10619cb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x10619dc; continue;
 }
 0x10619dc => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10619f2; continue; }
  pc = 0x10619e5; continue;
 }
 0x10619e5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x10619f2; continue; }
  pc = 0x10619ed; continue;
 }
 0x10619ed => {
  pc = 0x1061f2d; continue;
 }
 0x10619f2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061a03; continue;
 }
 0x1061a03 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061a19; continue; }
  pc = 0x1061a0c; continue;
 }
 0x1061a0c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061a19; continue; }
  pc = 0x1061a14; continue;
 }
 0x1061a14 => {
  pc = 0x1061f2d; continue;
 }
 0x1061a19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061a2a; continue;
 }
 0x1061a2a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061a40; continue; }
  pc = 0x1061a33; continue;
 }
 0x1061a33 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061a40; continue; }
  pc = 0x1061a3b; continue;
 }
 0x1061a3b => {
  pc = 0x1061f2d; continue;
 }
 0x1061a40 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061a51; continue;
 }
 0x1061a51 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061a67; continue; }
  pc = 0x1061a5a; continue;
 }
 0x1061a5a => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061a67; continue; }
  pc = 0x1061a62; continue;
 }
 0x1061a62 => {
  pc = 0x1061f2d; continue;
 }
 0x1061a67 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061a78; continue;
 }
 0x1061a78 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061a8e; continue; }
  pc = 0x1061a81; continue;
 }
 0x1061a81 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061a8e; continue; }
  pc = 0x1061a89; continue;
 }
 0x1061a89 => {
  pc = 0x1061f2d; continue;
 }
 0x1061a8e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061a9f; continue;
 }
 0x1061a9f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061ab5; continue; }
  pc = 0x1061aa8; continue;
 }
 0x1061aa8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061ab5; continue; }
  pc = 0x1061ab0; continue;
 }
 0x1061ab0 => {
  pc = 0x1061f2d; continue;
 }
 0x1061ab5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061ac6; continue;
 }
 0x1061ac6 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061adc; continue; }
  pc = 0x1061acf; continue;
 }
 0x1061acf => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061adc; continue; }
  pc = 0x1061ad7; continue;
 }
 0x1061ad7 => {
  pc = 0x1061f2d; continue;
 }
 0x1061adc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061aed; continue;
 }
 0x1061aed => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061b03; continue; }
  pc = 0x1061af6; continue;
 }
 0x1061af6 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061b03; continue; }
  pc = 0x1061afe; continue;
 }
 0x1061afe => {
  pc = 0x1061f2d; continue;
 }
 0x1061b03 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061b14; continue;
 }
 0x1061b14 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061b2a; continue; }
  pc = 0x1061b1d; continue;
 }
 0x1061b1d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061b2a; continue; }
  pc = 0x1061b25; continue;
 }
 0x1061b25 => {
  pc = 0x1061f2d; continue;
 }
 0x1061b2a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061b3b; continue;
 }
 0x1061b3b => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061b51; continue; }
  pc = 0x1061b44; continue;
 }
 0x1061b44 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061b51; continue; }
  pc = 0x1061b4c; continue;
 }
 0x1061b4c => {
  pc = 0x1061f2d; continue;
 }
 0x1061b51 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061b62; continue;
 }
 0x1061b62 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061b78; continue; }
  pc = 0x1061b6b; continue;
 }
 0x1061b6b => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061b78; continue; }
  pc = 0x1061b73; continue;
 }
 0x1061b73 => {
  pc = 0x1061f2d; continue;
 }
 0x1061b78 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061b89; continue;
 }
 0x1061b89 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061b9f; continue; }
  pc = 0x1061b92; continue;
 }
 0x1061b92 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061b9f; continue; }
  pc = 0x1061b9a; continue;
 }
 0x1061b9a => {
  pc = 0x1061f2d; continue;
 }
 0x1061b9f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061bb0; continue;
 }
 0x1061bb0 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061bc6; continue; }
  pc = 0x1061bb9; continue;
 }
 0x1061bb9 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061bc6; continue; }
  pc = 0x1061bc1; continue;
 }
 0x1061bc1 => {
  pc = 0x1061f2d; continue;
 }
 0x1061bc6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061bd7; continue;
 }
 0x1061bd7 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061bed; continue; }
  pc = 0x1061be0; continue;
 }
 0x1061be0 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061bed; continue; }
  pc = 0x1061be8; continue;
 }
 0x1061be8 => {
  pc = 0x1061f2d; continue;
 }
 0x1061bed => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061bfe; continue;
 }
 0x1061bfe => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061c14; continue; }
  pc = 0x1061c07; continue;
 }
 0x1061c07 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061c14; continue; }
  pc = 0x1061c0f; continue;
 }
 0x1061c0f => {
  pc = 0x1061f2d; continue;
 }
 0x1061c14 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061c25; continue;
 }
 0x1061c25 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061c3b; continue; }
  pc = 0x1061c2e; continue;
 }
 0x1061c2e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061c3b; continue; }
  pc = 0x1061c36; continue;
 }
 0x1061c36 => {
  pc = 0x1061f2d; continue;
 }
 0x1061c3b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061c4c; continue;
 }
 0x1061c4c => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061c62; continue; }
  pc = 0x1061c55; continue;
 }
 0x1061c55 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061c62; continue; }
  pc = 0x1061c5d; continue;
 }
 0x1061c5d => {
  pc = 0x1061f2d; continue;
 }
 0x1061c62 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061c73; continue;
 }
 0x1061c73 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061c89; continue; }
  pc = 0x1061c7c; continue;
 }
 0x1061c7c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061c89; continue; }
  pc = 0x1061c84; continue;
 }
 0x1061c84 => {
  pc = 0x1061f2d; continue;
 }
 0x1061c89 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061c9a; continue;
 }
 0x1061c9a => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061cb0; continue; }
  pc = 0x1061ca3; continue;
 }
 0x1061ca3 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061cb0; continue; }
  pc = 0x1061cab; continue;
 }
 0x1061cab => {
  pc = 0x1061f2d; continue;
 }
 0x1061cb0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061cc1; continue;
 }
 0x1061cc1 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061cd7; continue; }
  pc = 0x1061cca; continue;
 }
 0x1061cca => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061cd7; continue; }
  pc = 0x1061cd2; continue;
 }
 0x1061cd2 => {
  pc = 0x1061f2d; continue;
 }
 0x1061cd7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061ce8; continue;
 }
 0x1061ce8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061cfe; continue; }
  pc = 0x1061cf1; continue;
 }
 0x1061cf1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061cfe; continue; }
  pc = 0x1061cf9; continue;
 }
 0x1061cf9 => {
  pc = 0x1061f2d; continue;
 }
 0x1061cfe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061d0f; continue;
 }
 0x1061d0f => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061d25; continue; }
  pc = 0x1061d18; continue;
 }
 0x1061d18 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061d25; continue; }
  pc = 0x1061d20; continue;
 }
 0x1061d20 => {
  pc = 0x1061f2d; continue;
 }
 0x1061d25 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061d36; continue;
 }
 0x1061d36 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061d4c; continue; }
  pc = 0x1061d3f; continue;
 }
 0x1061d3f => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061d4c; continue; }
  pc = 0x1061d47; continue;
 }
 0x1061d47 => {
  pc = 0x1061f2d; continue;
 }
 0x1061d4c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061d5d; continue;
 }
 0x1061d5d => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061d73; continue; }
  pc = 0x1061d66; continue;
 }
 0x1061d66 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061d73; continue; }
  pc = 0x1061d6e; continue;
 }
 0x1061d6e => {
  pc = 0x1061f2d; continue;
 }
 0x1061d73 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061d84; continue;
 }
 0x1061d84 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061d9a; continue; }
  pc = 0x1061d8d; continue;
 }
 0x1061d8d => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061d9a; continue; }
  pc = 0x1061d95; continue;
 }
 0x1061d95 => {
  pc = 0x1061f2d; continue;
 }
 0x1061d9a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061dab; continue;
 }
 0x1061dab => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061dc1; continue; }
  pc = 0x1061db4; continue;
 }
 0x1061db4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061dc1; continue; }
  pc = 0x1061dbc; continue;
 }
 0x1061dbc => {
  pc = 0x1061f2d; continue;
 }
 0x1061dc1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061dd2; continue;
 }
 0x1061dd2 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061de8; continue; }
  pc = 0x1061ddb; continue;
 }
 0x1061ddb => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061de8; continue; }
  pc = 0x1061de3; continue;
 }
 0x1061de3 => {
  pc = 0x1061f2d; continue;
 }
 0x1061de8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061df9; continue;
 }
 0x1061df9 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061e0f; continue; }
  pc = 0x1061e02; continue;
 }
 0x1061e02 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061e0f; continue; }
  pc = 0x1061e0a; continue;
 }
 0x1061e0a => {
  pc = 0x1061f2d; continue;
 }
 0x1061e0f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061e20; continue;
 }
 0x1061e20 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061e36; continue; }
  pc = 0x1061e29; continue;
 }
 0x1061e29 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061e36; continue; }
  pc = 0x1061e31; continue;
 }
 0x1061e31 => {
  pc = 0x1061f2d; continue;
 }
 0x1061e36 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061e47; continue;
 }
 0x1061e47 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061e5d; continue; }
  pc = 0x1061e50; continue;
 }
 0x1061e50 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061e5d; continue; }
  pc = 0x1061e58; continue;
 }
 0x1061e58 => {
  pc = 0x1061f2d; continue;
 }
 0x1061e5d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061e6e; continue;
 }
 0x1061e6e => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061e84; continue; }
  pc = 0x1061e77; continue;
 }
 0x1061e77 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061e84; continue; }
  pc = 0x1061e7f; continue;
 }
 0x1061e7f => {
  pc = 0x1061f2d; continue;
 }
 0x1061e84 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061e95; continue;
 }
 0x1061e95 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061eab; continue; }
  pc = 0x1061e9e; continue;
 }
 0x1061e9e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061eab; continue; }
  pc = 0x1061ea6; continue;
 }
 0x1061ea6 => {
  pc = 0x1061f2d; continue;
 }
 0x1061eab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061eb8; continue;
 }
 0x1061eb8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061ecb; continue; }
  pc = 0x1061ec1; continue;
 }
 0x1061ec1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061ecb; continue; }
  pc = 0x1061ec9; continue;
 }
 0x1061ec9 => {
  pc = 0x1061f2d; continue;
 }
 0x1061ecb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061ed8; continue;
 }
 0x1061ed8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061eeb; continue; }
  pc = 0x1061ee1; continue;
 }
 0x1061ee1 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061eeb; continue; }
  pc = 0x1061ee9; continue;
 }
 0x1061ee9 => {
  pc = 0x1061f2d; continue;
 }
 0x1061eeb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061ef8; continue;
 }
 0x1061ef8 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061f0b; continue; }
  pc = 0x1061f01; continue;
 }
 0x1061f01 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061f0b; continue; }
  pc = 0x1061f09; continue;
 }
 0x1061f09 => {
  pc = 0x1061f2d; continue;
 }
 0x1061f0b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1061f2b; continue; }
  pc = 0x1061f18; continue;
 }
 0x1061f18 => {
  { cpu.sub8(cpu.r8(0x11c62f9u32), 0x0u8); }
  if cpu.zf { pc = 0x1061f2b; continue; }
  pc = 0x1061f21; continue;
 }
 0x1061f21 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { pc = 0x1061f2b; continue; }
  pc = 0x1061f29; continue;
 }
 0x1061f29 => {
  pc = 0x1061f2d; continue;
 }
 0x1061f2b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1061f2d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01060224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1060224); }
 let mut pc: u32 = 0x1060224;
 loop { match pc {
 0x1060224 => {
  cpu.call(0x6790337); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01061f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1061f30); }
 let mut pc: u32 = 0x1061f30;
 loop { match pc {
 0x1061f30 => {
  f_01095419(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01061f35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1061f35); }
 let mut pc: u32 = 0x1061f35;
 loop { match pc {
 0x1061f35 => {
  { cpu.sub8(cpu.r8(0x1105c5du32), 0x0u8); }
  if cpu.zf { pc = 0x1061f54; continue; }
  pc = 0x1061f3e; continue;
 }
 0x1061f3e => {
  { let v: u8 = 0x0u8; cpu.w8(0x1105c5du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6348u32, v); }
  cpu.push(0x1061f54); f_010953fc(cpu); if cpu.returned_to != 0x1061f54 { return; }
  pc = 0x1061f54; continue;
 }
 0x1061f54 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01061f55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1061f55); }
 let mut pc: u32 = 0x1061f55;
 loop { match pc {
 0x1061f55 => {
  { cpu.pushad(); }
  cpu.push(0x1061f5b); f_01084d90(cpu); if cpu.returned_to != 0x1061f5b { return; }
  pc = 0x1061f5b; continue;
 }
 0x1061f5b => {
  { let v = 0x1105999u32; cpu.push(v); }
  { let v = 0x16u32; cpu.push(v); }
  { let v = 0x11059efu32; cpu.push(v); }
  cpu.push(0x1061f6f); f_010957ff(cpu); if cpu.returned_to != 0x1061f6f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1061fb2; continue; }
  pc = 0x1061f76; continue;
 }
 0x1061f76 => {
  { let v: u32 = 0x11059efu32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x112639eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1061fa8; continue; }
  pc = 0x1061f87; continue;
 }
 0x1061f87 => {
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1061fa8; continue; }
  pc = 0x1061f93; continue;
 }
 0x1061f93 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.edi)), 0x3u16); }
  if cpu.zf { pc = 0x1061fe6; continue; }
  pc = 0x1061f9a; continue;
 }
 0x1061f9a => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.edi)), 0x2u16); }
  if cpu.zf { pc = 0x1061fbc; continue; }
  pc = 0x1061fa1; continue;
 }
 0x1061fa1 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.edi)), 0x5u16); }
  if cpu.zf { pc = 0x1061fbe; continue; }
  pc = 0x1061fa8; continue;
 }
 0x1061fa8 => {
  cpu.push(0x1061fad); f_010956b1(cpu); if cpu.returned_to != 0x1061fad { return; }
  cpu.poll(); pc = 0x1061f5b; continue;
 }
 0x1061fb2 => {
  cpu.push(0x1061fb7); f_010956b1(cpu); if cpu.returned_to != 0x1061fb7 { return; }
  pc = 0x10620c5; continue;
 }
 0x1061fbc => {
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x1061fbe => {
  { let v: u32 = 0x11059efu32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105c70u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11e242cu32.wrapping_add(cpu.eax.wrapping_mul(2)), v); }
  { cpu.pushad(); }
  cpu.push(0x1061fe3); f_01084e8b(cpu); if cpu.returned_to != 0x1061fe3 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x1061fe6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1105c62u32); cpu.ebx = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x1061fa8; continue; }
  pc = 0x1061ff7; continue;
 }
 0x1061ff7 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1105999u32); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x11059efu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x106200e); f_01084ed2(cpu); if cpu.returned_to != 0x106200e { return; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  { cpu.popad(); }
  if !cpu.zf { cpu.poll(); pc = 0x1061fa8; continue; }
  pc = 0x1062019; continue;
 }
 0x1062019 => {
  { let v: u8 = (cpu.eax as u8) & 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1105c91u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x106203e; continue; }
  pc = 0x1062027; continue;
 }
 0x1062027 => {
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if cpu.zf { pc = 0x1062059; continue; }
  pc = 0x106202d; continue;
 }
 0x106202d => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x1062074; continue; }
  pc = 0x1062033; continue;
 }
 0x1062033 => {
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if cpu.zf { pc = 0x106208f; continue; }
  pc = 0x1062039; continue;
 }
 0x1062039 => {
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x106203e => {
  { let v: u32 = cpu.r32(0x1124eacu32); cpu.edi = v; }
  { let v: u32 = 0x11dc39cu32; cpu.ebx = v; }
  cpu.push(0x106204e); f_010620aa(cpu); if cpu.returned_to != 0x106204e { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1124eacu32, v); }
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x1062059 => {
  { let v: u32 = cpu.r32(0x1124eb0u32); cpu.edi = v; }
  { let v: u32 = 0x11dc91cu32; cpu.ebx = v; }
  cpu.push(0x1062069); f_010620aa(cpu); if cpu.returned_to != 0x1062069 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1124eb0u32, v); }
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x1062074 => {
  { let v: u32 = cpu.r32(0x1124eb4u32); cpu.edi = v; }
  { let v: u32 = 0x11dce9cu32; cpu.ebx = v; }
  cpu.push(0x1062084); f_010620aa(cpu); if cpu.returned_to != 0x1062084 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1124eb4u32, v); }
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x106208f => {
  { let v: u32 = cpu.r32(0x1124eb8u32); cpu.edi = v; }
  { let v: u32 = 0x11dd41cu32; cpu.ebx = v; }
  cpu.push(0x106209f); f_010620aa(cpu); if cpu.returned_to != 0x106209f { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1124eb8u32, v); }
  cpu.poll(); pc = 0x1061fa8; continue;
 }
 0x10620c5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010620aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10620aa); }
 let mut pc: u32 = 0x10620aa;
 loop { match pc {
 0x10620aa => {
  { let v: u32 = cpu.ecx & 0x3ffu32; cpu.ecx = v; }
  pc = 0x10620b0; continue;
 }
 0x10620b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3ffu32; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10620b0; continue; }
  pc = 0x10620c4; continue;
 }
 0x10620c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010620c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10620c7); }
 let mut pc: u32 = 0x10620c7;
 loop { match pc {
 0x10620c7 => {
  { cpu.pushad(); }
  cpu.push(0x10620cd); f_01061f55(cpu); if cpu.returned_to != 0x10620cd { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.edx = v; }
  cpu.push(0x10620d7); f_010849b3(cpu); if cpu.returned_to != 0x10620d7 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010620dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10620dc); }
 let mut pc: u32 = 0x10620dc;
 loop { match pc {
 0x10620dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062539(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062539); }
 let mut pc: u32 = 0x1062539;
 loop { match pc {
 0x1062539 => {
  { let v: u32 = 0x11059c3u32; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1105c62u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v = 0x11u32; cpu.push(v); }
  { let v = 0x16u32; cpu.push(v); }
  { let v = 0x11059d9u32; cpu.push(v); }
  { let v = 0x16u32; cpu.push(v); }
  { let v = 0x11059c3u32; cpu.push(v); }
  cpu.push(0x1062569); f_01095419(cpu); if cpu.returned_to != 0x1062569 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106256d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106256d); }
 let mut pc: u32 = 0x106256d;
 loop { match pc {
 0x106256d => {
  { let v: u32 = cpu.r32(0x1105c62u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062573(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062573); }
 let mut pc: u32 = 0x1062573;
 loop { match pc {
 0x1062573 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1105c6cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106257c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106257c); }
 let mut pc: u32 = 0x106257c;
 loop { match pc {
 0x106257c => {
  { let v: u32 = 0x1052u32; cpu.ebx = v; }
  cpu.push(0x1062586); f_0105dce8(cpu); if cpu.returned_to != 0x1062586 { return; }
  { let v: u32 = 0x10f7c14u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  cpu.push(0x1062596); f_0100144a(cpu); if cpu.returned_to != 0x1062596 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  cpu.push(0x10625a1); f_01033bea(cpu); if cpu.returned_to != 0x10625a1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x10e68f0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xfdu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10625df; continue; }
  pc = 0x10625b9; continue;
 }
 0x10625b9 => {
  cpu.push(0x10625be); f_010626a6(cpu); if cpu.returned_to != 0x10625be { return; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1e0u32); cpu.esi = v; }
  cpu.push(0x10625c9); f_01062644(cpu); if cpu.returned_to != 0x10625c9 { return; }
  cpu.push(0x10625ce); f_0106262f(cpu); if cpu.returned_to != 0x10625ce { return; }
  cpu.push(0x10625d3); f_0105dd62(cpu); if cpu.returned_to != 0x10625d3 { return; }
  cpu.push(0x10625d8); f_0103e104(cpu); if cpu.returned_to != 0x10625d8 { return; }
  let t = cpu.r32(0x10e430cu32); cpu.push(0x10625de); cpu.call(t); if cpu.returned_to != 0x10625de { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10625df => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10625e5); f_0103e791(cpu); if cpu.returned_to != 0x10625e5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xfcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106260f; continue; }
  pc = 0x10625f1; continue;
 }
 0x10625f1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x10c09ccu32; cpu.edi = v; }
  { let v: u16 = 0x14u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10625fb; continue;
 }
 0x10625fb => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x22u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10625fb; continue; }
  pc = 0x106260c; continue;
 }
 0x106260c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = 0x10625b9; continue;
 }
 0x106260f => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xfbu8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106261f; continue; }
  pc = 0x106261a; continue;
 }
 0x106261a => {
  cpu.push(0x106261f); f_01062651(cpu); if cpu.returned_to != 0x106261f { return; }
  pc = 0x106261f; continue;
 }
 0x106261f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x2a8u32; cpu.ecx = v; }
  { let v: u32 = 0x10c09ccu32; cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = 0x10625b9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106262f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106262f); }
 let mut pc: u32 = 0x106262f;
 loop { match pc {
 0x106262f => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xf7u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1062642; continue; }
  pc = 0x1062635; continue;
 }
 0x1062635 => {
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.edi = v; }
  { let v: u32 = 0x3300u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x1062642; continue;
 }
 0x1062642 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062644(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062644); }
 let mut pc: u32 = 0x1062644;
 loop { match pc {
 0x1062644 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062645(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062645); }
 let mut pc: u32 = 0x1062645;
 loop { match pc {
 0x1062645 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xf8u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1062650; continue; }
  pc = 0x106264a; continue;
 }
 0x106264a => {
  { let v: u32 = cpu.add32(cpu.esi, 0x230u32); cpu.esi = v; }
  pc = 0x1062650; continue;
 }
 0x1062650 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062651(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062651); }
 let mut pc: u32 = 0x1062651;
 loop { match pc {
 0x1062651 => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xfau8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1062661; continue; }
  pc = 0x106265c; continue;
 }
 0x106265c => {
  cpu.push(0x1062661); f_01062696(cpu); if cpu.returned_to != 0x1062661 { return; }
  pc = 0x1062661; continue;
 }
 0x1062661 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xf9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1062687; continue; }
  pc = 0x1062666; continue;
 }
 0x1062666 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = 0x116aa58u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x1062673; continue;
 }
 0x1062673 => {
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1062673; continue; }
  pc = 0x1062685; continue;
 }
 0x1062685 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1062687 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xac8u32; cpu.ecx = v; }
  { let v: u32 = 0x116aa58u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01062696(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1062696); }
 let mut pc: u32 = 0x1062696;
 loop { match pc {
 0x1062696 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1a00u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
