#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0109ed90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ed90); }
 let mut pc: u32 = 0x109ed90;
 loop { match pc {
 0x109ed90 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ee53; continue; }
  pc = 0x109ed9c; continue;
 }
 0x109ed9c => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xfffffffeu32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x11ee2c8u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x2b11u32; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109ee1c; continue; }
  pc = 0x109ee17; continue;
 }
 0x109ee17 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x109ee25; continue; }
  pc = 0x109ee1c; continue;
 }
 0x109ee1c => {
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ee2c; continue;
 }
 0x109ee25 => {
  { let v: u32 = 0x40u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ee2c; continue;
 }
 0x109ee2c => {
  { let v: u32 = 0x0u32; cpu.w32(0x848u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x84cu32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x850u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x109ee50); f_0109d9f0(cpu); if cpu.returned_to != 0x109ee50 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109ee53; continue;
 }
 0x109ee53 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ee60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ee60); }
 let mut pc: u32 = 0x109ee60;
 loop { match pc {
 0x109ee60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ee69; continue; }
  pc = 0x109ee68; continue;
 }
 0x109ee68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ee69 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ee70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ee70); }
 let mut pc: u32 = 0x109ee70;
 loop { match pc {
 0x109ee70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ee94; continue; }
  pc = 0x109ee78; continue;
 }
 0x109ee78 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ee94; continue;
 }
 0x109ee94 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109eea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109eea0); }
 let mut pc: u32 = 0x109eea0;
 loop { match pc {
 0x109eea0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109eec9; continue; }
  pc = 0x109eeb0; continue;
 }
 0x109eeb0 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x34u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x109eeba; continue; }
  pc = 0x109eeb5; continue;
 }
 0x109eeb5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x38u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x109eec9; continue; }
  pc = 0x109eeba; continue;
 }
 0x109eeba => {
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x109eec6); f_0109d9f0(cpu); if cpu.returned_to != 0x109eec6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109eec9; continue;
 }
 0x109eec9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109eed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109eed0); }
 let mut pc: u32 = 0x109eed0;
 loop { match pc {
 0x109eed0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109eed9; continue; }
  pc = 0x109eed8; continue;
 }
 0x109eed8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109eed9 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109eee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109eee0); }
 let mut pc: u32 = 0x109eee0;
 loop { match pc {
 0x109eee0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109eeef; continue; }
  pc = 0x109eee8; continue;
 }
 0x109eee8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x109eeef; continue;
 }
 0x109eeef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109eef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109eef0); }
 let mut pc: u32 = 0x109eef0;
 loop { match pc {
 0x109eef0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109eef9; continue; }
  pc = 0x109eef8; continue;
 }
 0x109eef8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109eef9 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ef00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ef00); }
 let mut pc: u32 = 0x109ef00;
 loop { match pc {
 0x109ef00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ef1d; continue; }
  pc = 0x109ef0c; continue;
 }
 0x109ef0c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x109ef1d; continue; }
  pc = 0x109ef11; continue;
 }
 0x109ef11 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x109ef1a); f_0109d9f0(cpu); if cpu.returned_to != 0x109ef1a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109ef1d; continue;
 }
 0x109ef1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ef20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ef20); }
 let mut pc: u32 = 0x109ef20;
 loop { match pc {
 0x109ef20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ef29; continue; }
  pc = 0x109ef28; continue;
 }
 0x109ef28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ef29 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ef30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ef30); }
 let mut pc: u32 = 0x109ef30;
 loop { match pc {
 0x109ef30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ef4d; continue; }
  pc = 0x109ef3c; continue;
 }
 0x109ef3c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x44u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x109ef4d; continue; }
  pc = 0x109ef41; continue;
 }
 0x109ef41 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x109ef4a); f_0109d9f0(cpu); if cpu.returned_to != 0x109ef4a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109ef4d; continue;
 }
 0x109ef4d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ef50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ef50); }
 let mut pc: u32 = 0x109ef50;
 loop { match pc {
 0x109ef50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x109ef5b; continue; }
  pc = 0x109ef58; continue;
 }
 0x109ef58 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ef5b => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ef70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ef70); }
 let mut pc: u32 = 0x109ef70;
 loop { match pc {
 0x109ef70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109ef8b; continue; }
  pc = 0x109ef78; continue;
 }
 0x109ef78 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ef8b; continue;
 }
 0x109ef8b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109efb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109efb0); }
 let mut pc: u32 = 0x109efb0;
 loop { match pc {
 0x109efb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109efbf; continue; }
  pc = 0x109efb8; continue;
 }
 0x109efb8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x109efbf; continue;
 }
 0x109efbf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109efc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109efc0); }
 let mut pc: u32 = 0x109efc0;
 loop { match pc {
 0x109efc0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109f032; continue; }
  pc = 0x109efce; continue;
 }
 0x109efce => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x109f032; continue; }
  pc = 0x109efd4; continue;
 }
 0x109efd4 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x109f032; continue; }
  pc = 0x109efe6; continue;
 }
 0x109efe6 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x109f032; continue; }
  pc = 0x109efec; continue;
 }
 0x109efec => {
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x109f032; continue; }
  pc = 0x109f003; continue;
 }
 0x109f003 => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109f028); f_0108fe91(cpu); if cpu.returned_to != 0x109f028 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109f032; continue;
 }
 0x109f032 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f040); }
 let mut pc: u32 = 0x109f040;
 loop { match pc {
 0x109f040 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109f055; continue; }
  pc = 0x109f048; continue;
 }
 0x109f048 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x109f055; continue; }
  pc = 0x109f04e; continue;
 }
 0x109f04e => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x109f055; continue;
 }
 0x109f055 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f0c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f0c0); }
 let mut pc: u32 = 0x109f0c0;
 loop { match pc {
 0x109f0c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109f104; continue; }
  pc = 0x109f0c9; continue;
 }
 0x109f0c9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109f104; continue; }
  pc = 0x109f0d1; continue;
 }
 0x109f0d1 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x109f104; continue; }
  pc = 0x109f0d6; continue;
 }
 0x109f0d6 => {
  { let v: u32 = cpu.r32(0x84cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x109f0f1; continue; }
  pc = 0x109f0e7; continue;
 }
 0x109f0e7 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x84cu32.wrapping_add(cpu.esi)); cpu.push(0x109f0ee); cpu.call(t); if cpu.returned_to != 0x109f0ee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109f0f1; continue;
 }
 0x109f0f1 => {
  { cpu.sub32(cpu.r32(0x850u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109f104; continue; }
  pc = 0x109f0fa; continue;
 }
 0x109f0fa => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x850u32.wrapping_add(cpu.esi)); cpu.push(0x109f101); cpu.call(t); if cpu.returned_to != 0x109f101 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109f104; continue;
 }
 0x109f104 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f110); }
 let mut pc: u32 = 0x109f110;
 loop { match pc {
 0x109f110 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109f119; continue; }
  pc = 0x109f118; continue;
 }
 0x109f118 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f119 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x848u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x848u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f130); }
 let mut pc: u32 = 0x109f130;
 loop { match pc {
 0x109f130 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109f139; continue; }
  pc = 0x109f138; continue;
 }
 0x109f138 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f139 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x84cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x84cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f150); }
 let mut pc: u32 = 0x109f150;
 loop { match pc {
 0x109f150 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109f159; continue; }
  pc = 0x109f158; continue;
 }
 0x109f158 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f159 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x850u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x850u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f1b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f1b0); }
 let mut pc: u32 = 0x109f1b0;
 loop { match pc {
 0x109f1b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109f1eb; continue; }
  pc = 0x109f1c7; continue;
 }
 0x109f1c7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x109f1c9; continue;
 }
 0x109f1c9 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x109f1dd; continue; }
  pc = 0x109f1d4; continue;
 }
 0x109f1d4 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109f1da); f_0109d9f0(cpu); if cpu.returned_to != 0x109f1da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109f1dd; continue;
 }
 0x109f1dd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x894u32); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109f1c9; continue; }
  pc = 0x109f1eb; continue;
 }
 0x109f1eb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f1f0); }
 let mut pc: u32 = 0x109f1f0;
 loop { match pc {
 0x109f1f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f1f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f1f8); }
 let mut pc: u32 = 0x109f1f8;
 loop { match pc {
 0x109f1f8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xf2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi ^ cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi ^ cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109f23f; continue; }
  pc = 0x109f224; continue;
 }
 0x109f224 => {
  cpu.poll(); pc = cpu.r32(0x109f1f8u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x109f22c => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109f23f; continue;
 }
 0x109f233 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  pc = 0x109f23f; continue;
 }
 0x109f23a => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x109f23f; continue;
 }
 0x109f23f => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.ecx))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x11ee2ccu32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 15u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x109f29a; continue; }
  pc = 0x109f281; continue;
 }
 0x109f281 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x109f29a; continue; }
  pc = 0x109f28c; continue;
 }
 0x109f28c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x109f29a; continue; }
  pc = 0x109f297; continue;
 }
 0x109f297 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x109f29a; continue;
 }
 0x109f29a => {
  { let v: u32 = cpu.ebx.wrapping_add(0xffu32); cpu.ebx = v; }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f2b0); }
 let mut pc: u32 = 0x109f2b0;
 loop { match pc {
 0x109f2b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109f32f; continue; }
  pc = 0x109f2c0; continue;
 }
 0x109f2c0 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v: u8 = (cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109f32f; continue; }
  pc = 0x109f2ea; continue;
 }
 0x109f2ea => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x4u32); }
  if cpu.zf { pc = 0x109f32f; continue; }
  pc = 0x109f2f0; continue;
 }
 0x109f2f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x109f32f; continue; }
  pc = 0x109f300; continue;
 }
 0x109f300 => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109f325); f_0108fe91(cpu); if cpu.returned_to != 0x109f325 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109f32f; continue;
 }
 0x109f32f => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f340); }
 let mut pc: u32 = 0x109f340;
 loop { match pc {
 0x109f340 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109f34e; continue; }
  pc = 0x109f348; continue;
 }
 0x109f348 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f34e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffeu32); }
  if cpu.cf { pc = 0x109f37c; continue; }
  pc = 0x109f356; continue;
 }
 0x109f356 => {
  if (cpu.cf || cpu.zf) { pc = 0x109f35f; continue; }
  pc = 0x109f358; continue;
 }
 0x109f358 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109f370; continue; }
  pc = 0x109f35d; continue;
 }
 0x109f35d => {
  pc = 0x109f37c; continue;
 }
 0x109f35f => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f370 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f37c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if !cpu.zf { pc = 0x109f38c; continue; }
  pc = 0x109f386; continue;
 }
 0x109f386 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f38c => {
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f3a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f3a0); }
 let mut pc: u32 = 0x109f3a0;
 loop { match pc {
 0x109f3a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d08u32), 0x0u32); }
  if cpu.zf { pc = 0x109f454; continue; }
  pc = 0x109f3b0; continue;
 }
 0x109f3b0 => {
  { let v = 0x109f3a0u32; cpu.push(v); }
  { let v = 0x109d8b0u32; cpu.push(v); }
  cpu.push(0x109f3bf); f_01013dc7(cpu); if cpu.returned_to != 0x109f3bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x1134c84u32; cpu.push(v); }
  cpu.push(0x109f3d1); f_01013e03(cpu); if cpu.returned_to != 0x109f3d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134d04u32; cpu.push(v); }
  cpu.push(0x109f3e0); f_01013e03(cpu); if cpu.returned_to != 0x109f3e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee36cu32; cpu.push(v); }
  cpu.push(0x109f3ef); f_01013e03(cpu); if cpu.returned_to != 0x109f3ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x11ee358u32; cpu.push(v); }
  cpu.push(0x109f3fe); f_01013e03(cpu); if cpu.returned_to != 0x109f3fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee374u32; cpu.push(v); }
  cpu.push(0x109f40d); f_01013e03(cpu); if cpu.returned_to != 0x109f40d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee378u32; cpu.push(v); }
  cpu.push(0x109f41c); f_01013e03(cpu); if cpu.returned_to != 0x109f41c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee368u32; cpu.push(v); }
  cpu.push(0x109f42b); f_01013e03(cpu); if cpu.returned_to != 0x109f42b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee370u32; cpu.push(v); }
  cpu.push(0x109f43a); f_01013e03(cpu); if cpu.returned_to != 0x109f43a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee364u32; cpu.push(v); }
  cpu.push(0x109f449); f_01013e03(cpu); if cpu.returned_to != 0x109f449 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d08u32, v); }
  pc = 0x109f454; continue;
 }
 0x109f454 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f458); }
 let mut pc: u32 = 0x109f458;
 loop { match pc {
 0x109f458 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d0cu32), 0x0u32); }
  if !cpu.zf { pc = 0x109f480; continue; }
  pc = 0x109f464; continue;
 }
 0x109f464 => {
  { let v = 0x109fa72u32; cpu.push(v); }
  { let v = 0x109f458u32; cpu.push(v); }
  cpu.push(0x109f473); f_01013d5e(cpu); if cpu.returned_to != 0x109f473 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1134d0cu32, v); }
  pc = 0x109f480; continue;
 }
 0x109f480 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f484); }
 let mut pc: u32 = 0x109f484;
 loop { match pc {
 0x109f484 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f48e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f48e); }
 let mut pc: u32 = 0x109f48e;
 loop { match pc {
 0x109f48e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x874u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109f4a6; continue; }
  pc = 0x109f49c; continue;
 }
 0x109f49c => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x874u32.wrapping_add(cpu.esi)); cpu.push(0x109f4a3); cpu.call(t); if cpu.returned_to != 0x109f4a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109f4a6; continue;
 }
 0x109f4a6 => {
  { cpu.sub32(cpu.r32(0x88cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109f4b8; continue; }
  pc = 0x109f4af; continue;
 }
 0x109f4af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f4b5); f_01091071(cpu); if cpu.returned_to != 0x109f4b5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109f4b8; continue;
 }
 0x109f4b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x88cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f4c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f4c4); }
 let mut pc: u32 = 0x109f4c4;
 loop { match pc {
 0x109f4c4 => {
  { let v: u8 = cpu.sbb8(((cpu.edx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x109f4c8, "Daa");
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x9f74001u32); }
  { let v: u32 = cpu.r32(0x400109f5u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edx); cpu.w32(0x400109f5u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x9f5c401u32); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8f0109f6u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x1u8); }
  f_0109f4ec(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f4ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f4ec); }
 let mut pc: u32 = 0x109f4ec;
 loop { match pc {
 0x109f4ec => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x109f4f5; continue;
 }
 0x109f4f5 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x109f523; continue; }
  pc = 0x109f4fd; continue;
 }
 0x109f4fd => {
  { let v: u32 = cpu.r32(0x878u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109f740; continue; }
  pc = 0x109f50d; continue;
 }
 0x109f50d => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x109f4c4u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x109f51a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f520); f_0109f48e(cpu); if cpu.returned_to != 0x109f520 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109f523; continue;
 }
 0x109f523 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109f527 => {
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109f740; continue; }
  pc = 0x109f534; continue;
 }
 0x109f534 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f53a); f_0109f484(cpu); if cpu.returned_to != 0x109f53a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f54b); f_01091251(cpu); if cpu.returned_to != 0x109f54b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f56a); f_01091509(cpu); if cpu.returned_to != 0x109f56a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f577); f_010912d3(cpu); if cpu.returned_to != 0x109f577 { return; }
  pc = 0x109f577; continue;
 }
 0x109f577 => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x109f73b; continue; }
  pc = 0x109f585; continue;
 }
 0x109f585 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f58b); f_01091355(cpu); if cpu.returned_to != 0x109f58b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109f73b; continue;
 }
 0x109f593 => {
  { let v: u32 = cpu.r32(0x884u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109f740; continue; }
  pc = 0x109f5a2; continue;
 }
 0x109f5a2 => {
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x109f5b9; continue; }
  pc = 0x109f5aa; continue;
 }
 0x109f5aa => {
  { let v: u32 = 0x1u32; cpu.w32(0x888u32.wrapping_add(cpu.esi), v); }
  pc = 0x109f740; continue;
 }
 0x109f5b9 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x888u32.wrapping_add(cpu.esi), v); }
  pc = 0x109f740; continue;
 }
 0x109f5c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x87cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x880u32.wrapping_add(cpu.esi), v); }
  pc = 0x109f740; continue;
 }
 0x109f5db => {
  { let v: u32 = cpu.r32(0x880u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffu32); }
  if cpu.zf { pc = 0x109f5fa; continue; }
  pc = 0x109f5e9; continue;
 }
 0x109f5e9 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x880u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109f740; continue; }
  pc = 0x109f5fa; continue;
 }
 0x109f5fa => {
  { let v: u32 = cpu.r32(0x87cu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  pc = 0x109f740; continue;
 }
 0x109f605 => {
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109f740; continue; }
  pc = 0x109f612; continue;
 }
 0x109f612 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x109f63a; continue; }
  pc = 0x109f618; continue;
 }
 0x109f618 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f621); f_010912d3(cpu); if cpu.returned_to != 0x109f621 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10000u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x7a12000u32; cpu.eax = v; }
  pc = 0x109f659; continue;
 }
 0x109f63a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f642); f_010912d3(cpu); if cpu.returned_to != 0x109f642 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as u32; cpu.ebp = v; }
  { let v: u32 = 0x10000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xf424000u32; cpu.eax = v; }
  pc = 0x109f659; continue;
 }
 0x109f659 => {
  { let v = cpu.ebp; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f662); f_01091509(cpu); if cpu.returned_to != 0x109f662 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f66b); f_0109f484(cpu); if cpu.returned_to != 0x109f66b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f679); f_0109f484(cpu); if cpu.returned_to != 0x109f679 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f68a); f_01091251(cpu); if cpu.returned_to != 0x109f68a { return; }
  cpu.poll(); pc = 0x109f577; continue;
 }
 0x109f68f => {
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109f740; continue; }
  pc = 0x109f69c; continue;
 }
 0x109f69c => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f6a2); f_0109f484(cpu); if cpu.returned_to != 0x109f6a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f6b3); f_01091251(cpu); if cpu.returned_to != 0x109f6b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f6c0); f_01091509(cpu); if cpu.returned_to != 0x109f6c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109f6d8; continue; }
  pc = 0x109f6cd; continue;
 }
 0x109f6cd => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x109f6d8; continue; }
  pc = 0x109f6d4; continue;
 }
 0x109f6d4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x109f722; continue;
 }
 0x109f6d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x109f6ee; continue; }
  pc = 0x109f6e2; continue;
 }
 0x109f6e2 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x109f6ee; continue; }
  pc = 0x109f6e9; continue;
 }
 0x109f6e9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x109f722; continue;
 }
 0x109f6ee => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109f709; continue; }
  pc = 0x109f6f8; continue;
 }
 0x109f6f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x109f709; continue; }
  pc = 0x109f703; continue;
 }
 0x109f703 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x109f722; continue;
 }
 0x109f709 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x109f72b; continue; }
  pc = 0x109f713; continue;
 }
 0x109f713 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x109f72b; continue; }
  pc = 0x109f71e; continue;
 }
 0x109f71e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x109f722; continue;
 }
 0x109f722 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f728); f_010912d3(cpu); if cpu.returned_to != 0x109f728 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109f72b; continue;
 }
 0x109f72b => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x109f73b; continue; }
  pc = 0x109f732; continue;
 }
 0x109f732 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109f738); f_01091355(cpu); if cpu.returned_to != 0x109f738 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109f73b; continue;
 }
 0x109f73b => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x109f740; continue;
 }
 0x109f740 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f746); f_0109f484(cpu); if cpu.returned_to != 0x109f746 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x878u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x109f4f5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f759); }
 let mut pc: u32 = 0x109f759;
 loop { match pc {
 0x109f759 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109f765); f_0109f4ec(cpu); if cpu.returned_to != 0x109f765 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109f769(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109f769); }
 let mut pc: u32 = 0x109f769;
 loop { match pc {
 0x109f769 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  pc = 0x109f776; continue;
 }
 0x109f776 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138c48u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109f787); f_0108f6fc(cpu); if cpu.returned_to != 0x109f787 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109f79d; continue; }
  pc = 0x109f78b; continue;
 }
 0x109f78b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.poll(); pc = 0x109f776; continue;
 }
 0x109f79d => {
  { let v: u32 = cpu.r16(0xau32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109f7b5; continue; }
  pc = 0x109f7a6; continue;
 }
 0x109f7a6 => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if !cpu.zf { pc = 0x109f7b5; continue; }
  pc = 0x109f7af; continue;
 }
 0x109f7af => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x109f7fb; continue;
 }
 0x109f7b5 => {
  { let v: u32 = cpu.r16(0xau32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x109f7cd; continue; }
  pc = 0x109f7be; continue;
 }
 0x109f7be => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if !cpu.zf { pc = 0x109f7cd; continue; }
  pc = 0x109f7c7; continue;
 }
 0x109f7c7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x109f7fb; continue;
 }
 0x109f7cd => {
  { let v: u32 = cpu.r16(0xau32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109f7e5; continue; }
  pc = 0x109f7d6; continue;
 }
 0x109f7d6 => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { pc = 0x109f7e5; continue; }
  pc = 0x109f7df; continue;
 }
 0x109f7df => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x109f7fb; continue;
 }
 0x109f7e5 => {
  { let v: u32 = cpu.r16(0xau32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x109f804; continue; }
  pc = 0x109f7ee; continue;
 }
 0x109f7ee => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { pc = 0x109f804; continue; }
  pc = 0x109f7f7; continue;
 }
 0x109f7f7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x109f7fb; continue;
 }
 0x109f7fb => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f801); f_010912d3(cpu); if cpu.returned_to != 0x109f801 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109f804; continue;
 }
 0x109f804 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f80e); f_01091509(cpu); if cpu.returned_to != 0x109f80e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  pc = 0x109f818; continue;
 }
 0x109f818 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138c4du32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109f829); f_0108f6fc(cpu); if cpu.returned_to != 0x109f829 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109f83f; continue; }
  pc = 0x109f82d; continue;
 }
 0x109f82d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  cpu.poll(); pc = 0x109f818; continue;
 }
 0x109f83f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109f84d); f_01091251(cpu); if cpu.returned_to != 0x109f84d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fa72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fa72); }
 let mut pc: u32 = 0x109fa72;
 loop { match pc {
 0x109fa72 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d0cu32), 0x0u32); }
  if cpu.zf { pc = 0x109fa98; continue; }
  pc = 0x109fa7e; continue;
 }
 0x109fa7e => {
  { let v = 0x109fa72u32; cpu.push(v); }
  { let v = 0x109f458u32; cpu.push(v); }
  cpu.push(0x109fa8d); f_01013dc7(cpu); if cpu.returned_to != 0x109fa8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d0cu32, v); }
  pc = 0x109fa98; continue;
 }
 0x109fa98 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109faa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109faa0); }
 let mut pc: u32 = 0x109faa0;
 loop { match pc {
 0x109faa0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d94u32), 0x0u32); }
  if !cpu.zf { pc = 0x109fbb1; continue; }
  pc = 0x109fab0; continue;
 }
 0x109fab0 => {
  { let v = 0x10a3010u32; cpu.push(v); }
  { let v = 0x109faa0u32; cpu.push(v); }
  cpu.push(0x109fabf); f_01013d5e(cpu); if cpu.returned_to != 0x109fabf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x1134d10u32; cpu.push(v); }
  cpu.push(0x109fad1); f_01013dec(cpu); if cpu.returned_to != 0x109fad1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134d90u32; cpu.push(v); }
  cpu.push(0x109fae0); f_01013dec(cpu); if cpu.returned_to != 0x109fae0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee388u32; cpu.push(v); }
  cpu.push(0x109faef); f_01013dec(cpu); if cpu.returned_to != 0x109faef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee38cu32; cpu.push(v); }
  cpu.push(0x109fafe); f_01013dec(cpu); if cpu.returned_to != 0x109fafe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee390u32; cpu.push(v); }
  cpu.push(0x109fb0d); f_01013dec(cpu); if cpu.returned_to != 0x109fb0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee394u32; cpu.push(v); }
  cpu.push(0x109fb1c); f_01013dec(cpu); if cpu.returned_to != 0x109fb1c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a4u32; cpu.push(v); }
  cpu.push(0x109fb2b); f_01013dec(cpu); if cpu.returned_to != 0x109fb2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee398u32; cpu.push(v); }
  cpu.push(0x109fb3a); f_01013dec(cpu); if cpu.returned_to != 0x109fb3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee39cu32; cpu.push(v); }
  cpu.push(0x109fb49); f_01013dec(cpu); if cpu.returned_to != 0x109fb49 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee384u32; cpu.push(v); }
  cpu.push(0x109fb58); f_01013dec(cpu); if cpu.returned_to != 0x109fb58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a0u32; cpu.push(v); }
  cpu.push(0x109fb67); f_01013dec(cpu); if cpu.returned_to != 0x109fb67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3acu32; cpu.push(v); }
  cpu.push(0x109fb76); f_01013dec(cpu); if cpu.returned_to != 0x109fb76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a8u32; cpu.push(v); }
  cpu.push(0x109fb85); f_01013dec(cpu); if cpu.returned_to != 0x109fb85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee37cu32; cpu.push(v); }
  cpu.push(0x109fb94); f_01013dec(cpu); if cpu.returned_to != 0x109fb94 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee380u32; cpu.push(v); }
  cpu.push(0x109fba3); f_01013dec(cpu); if cpu.returned_to != 0x109fba3 { return; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d94u32, v); }
  pc = 0x109fbb1; continue;
 }
 0x109fbb1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fbc0); }
 let mut pc: u32 = 0x109fbc0;
 loop { match pc {
 0x109fbc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x109fbef; continue; }
  pc = 0x109fbd0; continue;
 }
 0x109fbd0 => {
  if (cpu.cf || cpu.zf) { pc = 0x109fc09; continue; }
  pc = 0x109fbd2; continue;
 }
 0x109fbd2 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x109fbe5; continue; }
  pc = 0x109fbd9; continue;
 }
 0x109fbd9 => {
  if (cpu.cf || cpu.zf) { pc = 0x109fc0f; continue; }
  pc = 0x109fbdb; continue;
 }
 0x109fbdb => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x109fc09; continue; }
  pc = 0x109fbe2; continue;
 }
 0x109fbe2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109fbe5 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x109fc0f; continue; }
  pc = 0x109fbec; continue;
 }
 0x109fbec => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109fbef => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x109fc02; continue; }
  pc = 0x109fbf6; continue;
 }
 0x109fbf6 => {
  if (cpu.cf || cpu.zf) { pc = 0x109fc09; continue; }
  pc = 0x109fbf8; continue;
 }
 0x109fbf8 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x109fc09; continue; }
  pc = 0x109fbff; continue;
 }
 0x109fbff => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109fc02 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x109fc15; continue; }
  pc = 0x109fc09; continue;
 }
 0x109fc09 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109fc0f => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109fc15 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fc20); }
 let mut pc: u32 = 0x109fc20;
 loop { match pc {
 0x109fc20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109fc68; continue; }
  pc = 0x109fc31; continue;
 }
 0x109fc31 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109fc51); f_0108fe91(cpu); if cpu.returned_to != 0x109fc51 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x109fc68; continue;
 }
 0x109fc68 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fc70); }
 let mut pc: u32 = 0x109fc70;
 loop { match pc {
 0x109fc70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x109fca6; continue; }
  pc = 0x109fc89; continue;
 }
 0x109fc89 => {
  if (cpu.cf || cpu.zf) { pc = 0x109fcbf; continue; }
  pc = 0x109fc8b; continue;
 }
 0x109fc8b => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x109fc9d; continue; }
  pc = 0x109fc92; continue;
 }
 0x109fc92 => {
  if (cpu.cf || cpu.zf) { pc = 0x109fcc6; continue; }
  pc = 0x109fc94; continue;
 }
 0x109fc94 => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x109fcbf; continue; }
  pc = 0x109fc9b; continue;
 }
 0x109fc9b => {
  pc = 0x109fccd; continue;
 }
 0x109fc9d => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x109fcc6; continue; }
  pc = 0x109fca4; continue;
 }
 0x109fca4 => {
  pc = 0x109fccd; continue;
 }
 0x109fca6 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x109fcb8; continue; }
  pc = 0x109fcad; continue;
 }
 0x109fcad => {
  if (cpu.cf || cpu.zf) { pc = 0x109fcbf; continue; }
  pc = 0x109fcaf; continue;
 }
 0x109fcaf => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x109fcbf; continue; }
  pc = 0x109fcb6; continue;
 }
 0x109fcb6 => {
  pc = 0x109fccd; continue;
 }
 0x109fcb8 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x109fccd; continue; }
  pc = 0x109fcbf; continue;
 }
 0x109fcbf => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x109fccf; continue;
 }
 0x109fcc6 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x109fccf; continue;
 }
 0x109fccd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109fccf; continue;
 }
 0x109fccf => {
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x109fd20; continue; }
  pc = 0x109fce0; continue;
 }
 0x109fce0 => {
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109fd20; continue; }
  pc = 0x109fce9; continue;
 }
 0x109fce9 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109fd09); f_0108fe91(cpu); if cpu.returned_to != 0x109fd09 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x109fd20; continue;
 }
 0x109fd20 => {
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x100u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x100u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x109fd7c; continue; }
  pc = 0x109fd5f; continue;
 }
 0x109fd5f => {
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x100u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x109fd7c; continue;
 }
 0x109fd7c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi))); cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fd90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fd90); }
 let mut pc: u32 = 0x109fd90;
 loop { match pc {
 0x109fd90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109fdd9; continue; }
  pc = 0x109fda2; continue;
 }
 0x109fda2 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109fdc2); f_0108fe91(cpu); if cpu.returned_to != 0x109fdc2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x109fdd9; continue;
 }
 0x109fdd9 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x109fdec; continue; }
  pc = 0x109fde5; continue;
 }
 0x109fde5 => {
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  pc = 0x109fdee; continue;
 }
 0x109fdec => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x109fdee; continue;
 }
 0x109fdee => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  cpu.push(0x109fdff); f_010a6310(cpu); if cpu.returned_to != 0x109fdff { return; }
  { let v: u32 = cpu.r32(0x1a8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109fe47; continue; }
  pc = 0x109fe10; continue;
 }
 0x109fe10 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109fe30); f_0108fe91(cpu); if cpu.returned_to != 0x109fe30 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x109fe47; continue;
 }
 0x109fe47 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fe50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fe50); }
 let mut pc: u32 = 0x109fe50;
 loop { match pc {
 0x109fe50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x109fe5d; continue;
 }
 0x109fe5d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx | cpu.esi; cpu.ecx = v; }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x109fe78; continue; }
  pc = 0x109fe75; continue;
 }
 0x109fe75 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109fe5d; continue; }
  pc = 0x109fe78; continue;
 }
 0x109fe78 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fe80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fe80); }
 let mut pc: u32 = 0x109fe80;
 loop { match pc {
 0x109fe80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109fec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109fec0); }
 let mut pc: u32 = 0x109fec0;
 loop { match pc {
 0x109fec0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x109fed2; continue;
 }
 0x109fed2 => {
  { let v: u32 = 0x1138cecu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109fee2); f_0108f5f5(cpu); if cpu.returned_to != 0x109fee2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109fefc; continue; }
  pc = 0x109fee6; continue;
 }
 0x109fee6 => {
  { let v: u32 = 0x1138cf4u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109fef4); f_0108f5f5(cpu); if cpu.returned_to != 0x109fef4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ffce; continue; }
  pc = 0x109fefc; continue;
 }
 0x109fefc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 24u32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff0000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x1138cfcu32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x109ff40); f_0108f5f5(cpu); if cpu.returned_to != 0x109ff40 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x109fed2; continue; }
  pc = 0x109ff44; continue;
 }
 0x109ff44 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138cecu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109ff55); f_0108f5f5(cpu); if cpu.returned_to != 0x109ff55 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ff67; continue; }
  pc = 0x109ff59; continue;
 }
 0x109ff59 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x109ffd0; continue; }
  pc = 0x109ff61; continue;
 }
 0x109ff61 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ff67 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x109ffce; continue; }
  pc = 0x109ff71; continue;
 }
 0x109ff71 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138cfcu32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x109ff83); f_0108f5f5(cpu); if cpu.returned_to != 0x109ff83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ff93; continue; }
  pc = 0x109ff87; continue;
 }
 0x109ff87 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109ff93; continue; }
  pc = 0x109ff8d; continue;
 }
 0x109ff8d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ff93 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edi & 0xff00u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 24u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff0000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x109ff71; continue; }
  pc = 0x109ffce; continue;
 }
 0x109ffce => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x109ffd0; continue;
 }
 0x109ffd0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0000); }
 let mut pc: u32 = 0x10a0000;
 loop { match pc {
 0x10a0000 => {
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(65153); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0105(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0105); }
 let mut pc: u32 = 0x10a0105;
 loop { match pc {
 0x10a0105 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffc3u32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); cpu.w8(0xffffffc3u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x180u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0113(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0113); }
 let mut pc: u32 = 0x10a0113;
 loop { match pc {
 0x10a0113 => {
  { let v: u32 = cpu.r32(0x1c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0180); }
 let mut pc: u32 = 0x10a0180;
 loop { match pc {
 0x10a0180 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xf0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, 0xc0u32); }
  if cpu.cf { pc = 0x10a01bb; continue; }
  pc = 0x10a01ae; continue;
 }
 0x10a01ae => {
  if (cpu.cf || cpu.zf) { pc = 0x10a01c6; continue; }
  pc = 0x10a01b0; continue;
 }
 0x10a01b0 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x10a01d0; continue; }
  pc = 0x10a01b8; continue;
 }
 0x10a01b8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a01bb => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x10a01e5; continue; }
  pc = 0x10a01c3; continue;
 }
 0x10a01c3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a01c6 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a01d0 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a01e5 => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x10a022f; continue; }
  pc = 0x10a01ea; continue;
 }
 0x10a01ea => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0326; continue; }
  pc = 0x10a01f0; continue;
 }
 0x10a01f0 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x10a021e; continue; }
  pc = 0x10a01f5; continue;
 }
 0x10a01f5 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a029f; continue; }
  pc = 0x10a01fb; continue;
 }
 0x10a01fb => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x10a0212; continue; }
  pc = 0x10a0200; continue;
 }
 0x10a0200 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a02b9; continue; }
  pc = 0x10a0206; continue;
 }
 0x10a0206 => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x10a02c6; continue; }
  pc = 0x10a020f; continue;
 }
 0x10a020f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0212 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x10a02ac; continue; }
  pc = 0x10a021b; continue;
 }
 0x10a021b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a021e => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x10a0227; continue; }
  pc = 0x10a0223; continue;
 }
 0x10a0223 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0278; continue; }
  pc = 0x10a0225; continue;
 }
 0x10a0225 => {
  pc = 0x10a0285; continue;
 }
 0x10a0227 => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x10a0292; continue; }
  pc = 0x10a022c; continue;
 }
 0x10a022c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a022f => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x10a025d; continue; }
  pc = 0x10a0234; continue;
 }
 0x10a0234 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a02ef; continue; }
  pc = 0x10a023a; continue;
 }
 0x10a023a => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x10a0251; continue; }
  pc = 0x10a023f; continue;
 }
 0x10a023f => {
  if (cpu.cf || cpu.zf) { pc = 0x10a030b; continue; }
  pc = 0x10a0245; continue;
 }
 0x10a0245 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x10a0319; continue; }
  pc = 0x10a024e; continue;
 }
 0x10a024e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0251 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x10a02fd; continue; }
  pc = 0x10a025a; continue;
 }
 0x10a025a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a025d => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x10a0270; continue; }
  pc = 0x10a0262; continue;
 }
 0x10a0262 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0333; continue; }
  pc = 0x10a0268; continue;
 }
 0x10a0268 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x10a02e1; continue; }
  pc = 0x10a026d; continue;
 }
 0x10a026d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0270 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10a02d3; continue; }
  pc = 0x10a0275; continue;
 }
 0x10a0275 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0278 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0285 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0292 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a029f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02ac => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02b9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02c6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02d3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02e1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02ef => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a02fd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a030b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0319 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0326 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0333 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0350); }
 let mut pc: u32 = 0x10a0350;
 loop { match pc {
 0x10a0350 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xf0u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0xb0u32); }
  if cpu.zf { pc = 0x10a03a2; continue; }
  pc = 0x10a0392; continue;
 }
 0x10a0392 => {
  { cpu.sub32(cpu.edx, 0xc0u32); }
  if cpu.zf { pc = 0x10a03a2; continue; }
  pc = 0x10a039a; continue;
 }
 0x10a039a => {
  { cpu.sub32(cpu.edx, 0xe0u32); }
  if !cpu.zf { pc = 0x10a03be; continue; }
  pc = 0x10a03a2; continue;
 }
 0x10a03a2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xd4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a03bb); f_010a0180(cpu); if cpu.returned_to != 0x10a03bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a03be; continue;
 }
 0x10a03be => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xb0u32); }
  if !cpu.zf { pc = 0x10a0504; continue; }
  pc = 0x10a03cc; continue;
 }
 0x10a03cc => {
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10a03f7; continue; }
  pc = 0x10a03d3; continue;
 }
 0x10a03d3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2d4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a03f7; continue; }
  pc = 0x10a03eb; continue;
 }
 0x10a03eb => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2d4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a03f7; continue;
 }
 0x10a03f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, 0x6fu32); }
  if cpu.cf { pc = 0x10a0456; continue; }
  pc = 0x10a041e; continue;
 }
 0x10a041e => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0694; continue; }
  pc = 0x10a0424; continue;
 }
 0x10a0424 => {
  { cpu.sub32(cpu.ebp, 0x75u32); }
  if cpu.cf { pc = 0x10a0446; continue; }
  pc = 0x10a0429; continue;
 }
 0x10a0429 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a060d; continue; }
  pc = 0x10a042f; continue;
 }
 0x10a042f => {
  { cpu.sub32(cpu.ebp, 0x76u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a057d; continue; }
  pc = 0x10a0438; continue;
 }
 0x10a0438 => {
  { cpu.sub32(cpu.ebp, 0x77u32); }
  if cpu.zf { pc = 0x10a05bb; continue; }
  pc = 0x10a0441; continue;
 }
 0x10a0441 => {
  pc = 0x10a0504; continue;
 }
 0x10a0446 => {
  { cpu.sub32(cpu.ebp, 0x73u32); }
  if cpu.cf { pc = 0x10a0504; continue; }
  pc = 0x10a044f; continue;
 }
 0x10a044f => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0484; continue; }
  pc = 0x10a0451; continue;
 }
 0x10a0451 => {
  pc = 0x10a05d9; continue;
 }
 0x10a0456 => {
  { cpu.sub32(cpu.ebp, 0x6cu32); }
  if cpu.cf { pc = 0x10a046b; continue; }
  pc = 0x10a045b; continue;
 }
 0x10a045b => {
  if (cpu.cf || cpu.zf) { pc = 0x10a049d; continue; }
  pc = 0x10a045d; continue;
 }
 0x10a045d => {
  { cpu.sub32(cpu.ebp, 0x6du32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a0683; continue; }
  pc = 0x10a0466; continue;
 }
 0x10a0466 => {
  pc = 0x10a06bf; continue;
 }
 0x10a046b => {
  { cpu.sub32(cpu.ebp, 0x6u32); }
  if cpu.cf { pc = 0x10a0504; continue; }
  pc = 0x10a0474; continue;
 }
 0x10a0474 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a04c2; continue; }
  pc = 0x10a0476; continue;
 }
 0x10a0476 => {
  { cpu.sub32(cpu.ebp, 0x7u32); }
  if cpu.zf { pc = 0x10a0550; continue; }
  pc = 0x10a047f; continue;
 }
 0x10a047f => {
  pc = 0x10a0504; continue;
 }
 0x10a0484 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d4u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a0504; continue;
 }
 0x10a049d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a0504; continue; }
  pc = 0x10a04a7; continue;
 }
 0x10a04a7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.push(0x10a04b1); cpu.call(t); if cpu.returned_to != 0x10a04b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d4u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a0504; continue;
 }
 0x10a04c2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a04d5); f_010a0350(cpu); if cpu.returned_to != 0x10a04d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x65u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a04eb); f_010a0350(cpu); if cpu.returned_to != 0x10a04eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x26u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a0501); f_010a0350(cpu); if cpu.returned_to != 0x10a0501 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0504; continue;
 }
 0x10a0504 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x10a0524; continue; }
  pc = 0x10a051b; continue;
 }
 0x10a051b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x60u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a0524; continue;
 }
 0x10a0524 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x90u32); }
  if !cpu.zf { pc = 0x10a0743; continue; }
  pc = 0x10a0545; continue;
 }
 0x10a0545 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x160u32.wrapping_add(cpu.eax))); cpu.w32(0x160u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a0751; continue;
 }
 0x10a0550 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x1b0u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v: u32 = 0x3f01u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a0575; continue; }
  pc = 0x10a0570; continue;
 }
 0x10a0570 => {
  { let v: u32 = 0x7fu32; cpu.edi = v; }
  pc = 0x10a0575; continue;
 }
 0x10a0575 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a0504; continue; }
  pc = 0x10a0579; continue;
 }
 0x10a0579 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.poll(); pc = 0x10a0504; continue;
 }
 0x10a057d => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x6cu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a05a8; continue;
 }
 0x10a05a8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x10a05b1); cpu.call(t); if cpu.returned_to != 0x10a05b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a05bb => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a05c5; continue;
 }
 0x10a05c5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.push(0x10a05cf); cpu.call(t); if cpu.returned_to != 0x10a05cf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a05d9 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a05dd; continue;
 }
 0x10a05dd => {
  { cpu.sub32(cpu.r32(0x84u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10a05ef; continue; }
  pc = 0x10a05e6; continue;
 }
 0x10a05e6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a05dd; continue; }
  pc = 0x10a05ef; continue;
 }
 0x10a05ef => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a05f8; continue;
 }
 0x10a05f8 => {
  { let v: u32 = cpu.edi; cpu.w32(0x84u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x74u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a060d => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a07cb; continue; }
  pc = 0x10a0616; continue;
 }
 0x10a0616 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x10a061e; continue;
 }
 0x10a061e => {
  { cpu.sub32(cpu.r32(0x84u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a062f; continue; }
  pc = 0x10a0627; continue;
 }
 0x10a0627 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a061e; continue; }
  pc = 0x10a062f; continue;
 }
 0x10a062f => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a0638; continue;
 }
 0x10a0638 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10a0658; continue; }
  pc = 0x10a064b; continue;
 }
 0x10a064b => {
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0658 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a0672; continue; }
  pc = 0x10a0665; continue;
 }
 0x10a0665 => {
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0672 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0683 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a068a); f_01093d39(cpu); if cpu.returned_to != 0x10a068a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0694 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a069e; continue;
 }
 0x10a069e => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a06b1; continue; }
  pc = 0x10a06a3; continue;
 }
 0x10a06a3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a06b1 => {
  { let v: u32 = 0x2u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a06bf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a0706; continue; }
  pc = 0x10a06c9; continue;
 }
 0x10a06c9 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a06d3; continue;
 }
 0x10a06d3 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a06d9); f_01094522(cpu); if cpu.returned_to != 0x10a06d9 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a06e6; continue;
 }
 0x10a06e6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a06ee); f_0109467d(cpu); if cpu.returned_to != 0x10a06ee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0706 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a0710; continue;
 }
 0x10a0710 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a071b); f_010a08b0(cpu); if cpu.returned_to != 0x10a071b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a072e); f_01094607(cpu); if cpu.returned_to != 0x10a072e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a0739); f_0109467d(cpu); if cpu.returned_to != 0x10a0739 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a0743 => {
  { cpu.sub32(cpu.edx, 0x80u32); }
  if !cpu.zf { pc = 0x10a0751; continue; }
  pc = 0x10a074b; continue;
 }
 0x10a074b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x160u32.wrapping_add(cpu.eax))); cpu.w32(0x160u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a0751; continue;
 }
 0x10a0751 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xe0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if !cpu.zf { pc = 0x10a0782; continue; }
  pc = 0x10a0774; continue;
 }
 0x10a0774 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x214u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a07cb; continue; }
  pc = 0x10a0782; continue;
 }
 0x10a0782 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10a07b1; continue; }
  pc = 0x10a078f; continue;
 }
 0x10a078f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x1a0u32.wrapping_add(cpu.eax)); cpu.push(0x10a07aa); cpu.call(t); if cpu.returned_to != 0x10a07aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a07cb; continue; }
  pc = 0x10a07b1; continue;
 }
 0x10a07b1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a07c8); f_0109fc70(cpu); if cpu.returned_to != 0x10a07c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a07cb; continue;
 }
 0x10a07cb => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a07e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a07e0); }
 let mut pc: u32 = 0x10a07e0;
 loop { match pc {
 0x10a07e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  pc = 0x10a0800; continue;
 }
 0x10a0800 => {
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0830; continue; }
  pc = 0x10a080b; continue;
 }
 0x10a080b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x10a0820); f_010a0350(cpu); if cpu.returned_to != 0x10a0820 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10a0830; continue;
 }
 0x10a0830 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0800; continue; }
  pc = 0x10a083b; continue;
 }
 0x10a083b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x554u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a0888; continue; }
  pc = 0x10a0851; continue;
 }
 0x10a0851 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a0871); f_0108fe91(cpu); if cpu.returned_to != 0x10a0871 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x10a0888; continue;
 }
 0x10a0888 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a089f; continue; }
  pc = 0x10a088c; continue;
 }
 0x10a088c => {
  cpu.push(0x10a0891); f_01090014(cpu); if cpu.returned_to != 0x10a0891 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a089f; continue; }
  pc = 0x10a0895; continue;
 }
 0x10a0895 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a089c); f_0108ffa7(cpu); if cpu.returned_to != 0x10a089c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10a089f; continue;
 }
 0x10a089f => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a08b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a08b0); }
 let mut pc: u32 = 0x10a08b0;
 loop { match pc {
 0x10a08b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x10a08d0; continue;
 }
 0x10a08d0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x558u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x10a0910; continue; }
  pc = 0x10a08dc; continue;
 }
 0x10a08dc => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a08f6); f_010a0350(cpu); if cpu.returned_to != 0x10a08f6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  {  }
  pc = 0x10a0910; continue;
 }
 0x10a0910 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10a08d0; continue; }
  pc = 0x10a0917; continue;
 }
 0x10a0917 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a0959; continue; }
  pc = 0x10a0922; continue;
 }
 0x10a0922 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a0942); f_0108fe91(cpu); if cpu.returned_to != 0x10a0942 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x10a0959; continue;
 }
 0x10a0959 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0960(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a0960); } r_010a0960(cpu, 0x10a0960); }
pub fn f_010a0a00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a0a00); } r_010a0960(cpu, 0x10a0a00); }
pub fn f_010a0a20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a0a20); } r_010a0960(cpu, 0x10a0a20); }
fn r_010a0960(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a0960 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x294u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0992; continue; }
  pc = 0x10a097f; continue;
 }
 0x10a097f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a098f); f_010a0350(cpu); if cpu.returned_to != 0x10a098f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0992; continue;
 }
 0x10a0992 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xd4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a09b9; continue; }
  pc = 0x10a09a6; continue;
 }
 0x10a09a6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a09b6); f_010a0350(cpu); if cpu.returned_to != 0x10a09b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a09b9; continue;
 }
 0x10a09b9 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a09e5; continue; }
  pc = 0x10a09cd; continue;
 }
 0x10a09cd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a09e2); f_010a0350(cpu); if cpu.returned_to != 0x10a09e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a09e5; continue;
 }
 0x10a09e5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x214u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0a0c; continue; }
  pc = 0x10a09f9; continue;
 }
 0x10a09f9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x6bu32; cpu.push(v); }
  pc = 0x10a0a00; continue;
 }
 0x10a0a00 => {
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0a09); f_010a0350(cpu); if cpu.returned_to != 0x10a0a09 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0a0c; continue;
 }
 0x10a0a0c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0a33; continue; }
  pc = 0x10a0a20; continue;
 }
 0x10a0a20 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x6fu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0a30); f_010a0350(cpu); if cpu.returned_to != 0x10a0a30 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0a33; continue;
 }
 0x10a0a33 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x254u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0a5a; continue; }
  pc = 0x10a0a47; continue;
 }
 0x10a0a47 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0a57); f_010a0350(cpu); if cpu.returned_to != 0x10a0a57 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0a5a; continue;
 }
 0x10a0a5a => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x354u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0a81; continue; }
  pc = 0x10a0a6e; continue;
 }
 0x10a0a6e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0a7e); f_010a0350(cpu); if cpu.returned_to != 0x10a0a7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0a81; continue;
 }
 0x10a0a81 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x394u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0aa8; continue; }
  pc = 0x10a0a95; continue;
 }
 0x10a0a95 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0aa5); f_010a0350(cpu); if cpu.returned_to != 0x10a0aa5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0aa8; continue;
 }
 0x10a0aa8 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3d4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0acf; continue; }
  pc = 0x10a0abc; continue;
 }
 0x10a0abc => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0acc); f_010a0350(cpu); if cpu.returned_to != 0x10a0acc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0acf; continue;
 }
 0x10a0acf => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x414u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0af6; continue; }
  pc = 0x10a0ae3; continue;
 }
 0x10a0ae3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0af3); f_010a0350(cpu); if cpu.returned_to != 0x10a0af3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0af6; continue;
 }
 0x10a0af6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x454u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0b1d; continue; }
  pc = 0x10a0b0a; continue;
 }
 0x10a0b0a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0b1a); f_010a0350(cpu); if cpu.returned_to != 0x10a0b1a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0b1d; continue;
 }
 0x10a0b1d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0b44; continue; }
  pc = 0x10a0b31; continue;
 }
 0x10a0b31 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x5bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0b41); f_010a0350(cpu); if cpu.returned_to != 0x10a0b41 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0b44; continue;
 }
 0x10a0b44 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0b6b; continue; }
  pc = 0x10a0b58; continue;
 }
 0x10a0b58 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x5du32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0b68); f_010a0350(cpu); if cpu.returned_to != 0x10a0b68 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10a0b6b; continue;
 }
 0x10a0b6b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0b93; continue; }
  pc = 0x10a0b7f; continue;
 }
 0x10a0b7f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16) | 0xb0u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a0b90); f_010a0350(cpu); if cpu.returned_to != 0x10a0b90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  pc = 0x10a0b93; continue;
 }
 0x10a0b93 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0ba0); }
 let mut pc: u32 = 0x10a0ba0;
 loop { match pc {
 0x10a0ba0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  pc = 0x10a0bb0; continue;
 }
 0x10a0bb0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x94u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a0bb0; continue; }
  pc = 0x10a0bbf; continue;
 }
 0x10a0bbf => {
  { let v: u32 = 0x480u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0xd4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a0bd4); f_0108f030(cpu); if cpu.returned_to != 0x10a0bd4 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x10a0bd9; continue;
 }
 0x10a0bd9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0bd9; continue; }
  pc = 0x10a0bea; continue;
 }
 0x10a0bea => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x10a0c00; continue;
 }
 0x10a0c00 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x554u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0c00; continue; }
  pc = 0x10a0c11; continue;
 }
 0x10a0c11 => {
  { let v: u32 = 0x0u32; cpu.w32(0x554u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x60u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x68u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x64u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x7a1200u32; cpu.w32(0x70u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.ecx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0c60); }
 let mut pc: u32 = 0x10a0c60;
 loop { match pc {
 0x10a0c60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a0c6b); f_010a0ba0(cpu); if cpu.returned_to != 0x10a0c6b { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0c80); }
 let mut pc: u32 = 0x10a0c80;
 loop { match pc {
 0x10a0c80 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  pc = 0x10a0c90; continue;
 }
 0x10a0c90 => {
  { let v: u32 = cpu.r32(0x394u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a0cb0; continue; }
  pc = 0x10a0c9b; continue;
 }
 0x10a0c9b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a0cab); f_010a0350(cpu); if cpu.returned_to != 0x10a0cab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x10a0cb0; continue;
 }
 0x10a0cb0 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a0c90; continue; }
  pc = 0x10a0cb9; continue;
 }
 0x10a0cb9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a0cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a0cc0); }
 let mut pc: u32 = 0x10a0cc0;
 loop { match pc {
 0x10a0cc0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10a1474; continue; }
  pc = 0x10a0cd4; continue;
 }
 0x10a0cd4 => {
  { cpu.sub32(cpu.r32(0x1134d90u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a1474; continue; }
  pc = 0x10a0ce1; continue;
 }
 0x10a0ce1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee394u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1134d90u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee394u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee388u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a1420; continue; }
  pc = 0x10a0d0e; continue;
 }
 0x10a0d0e => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x10a13f9; continue; }
  pc = 0x10a0d1d; continue;
 }
 0x10a0d1d => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11ee3a4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a0d37; continue;
 }
 0x10a0d37 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a1333; continue; }
  pc = 0x10a0d48; continue;
 }
 0x10a0d48 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x554u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a0df5; continue; }
  pc = 0x10a0d5c; continue;
 }
 0x10a0d5c => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11ee38cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10a0d6b; continue;
 }
 0x10a0d6b => {
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x558u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a0ddf; continue; }
  pc = 0x10a0d87; continue;
 }
 0x10a0d87 => {
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x658u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10a0ddf; continue; }
  pc = 0x10a0d99; continue;
 }
 0x10a0d99 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a0db8); f_010a0350(cpu); if cpu.returned_to != 0x10a0db8 { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x558u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x554u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x554u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if cpu.zf { pc = 0x10a0df5; continue; }
  pc = 0x10a0ddf; continue;
 }
 0x10a0ddf => {
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee38cu32, v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a0d6b; continue; }
  pc = 0x10a0df5; continue;
 }
 0x10a0df5 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a12a4; continue; }
  pc = 0x10a0e09; continue;
 }
 0x10a0e09 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee3a0u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.cf { pc = 0x10a1282; continue; }
  pc = 0x10a0e26; continue;
 }
 0x10a0e26 => {
  { cpu.sub32(cpu.r32(0x11ee3a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a1282; continue; }
  pc = 0x10a0e33; continue;
 }
 0x10a0e33 => {
  { let v: u32 = cpu.r32(0x11ee3a0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xf7u32); }
  if cpu.cf { pc = 0x10a0e51; continue; }
  pc = 0x10a0e3f; continue;
 }
 0x10a0e3f => {
  if (cpu.cf || cpu.zf) { pc = 0x10a103d; continue; }
  pc = 0x10a0e45; continue;
 }
 0x10a0e45 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { pc = 0x10a0e61; continue; }
  pc = 0x10a0e4c; continue;
 }
 0x10a0e4c => {
  pc = 0x10a10d6; continue;
 }
 0x10a0e51 => {
  { cpu.sub32(cpu.eax, 0xf0u32); }
  if cpu.zf { pc = 0x10a103d; continue; }
  pc = 0x10a0e5c; continue;
 }
 0x10a0e5c => {
  pc = 0x10a10d6; continue;
 }
 0x10a0e61 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee3acu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10a0e79); f_0109fe50(cpu); if cpu.returned_to != 0x10a0e79 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee3a8u32, v); }
  { let v: u32 = cpu.r32(0x11ee3acu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x51u32); }
  if cpu.cf { pc = 0x10a0e9f; continue; }
  pc = 0x10a0e8b; continue;
 }
 0x10a0e8b => {
  if (cpu.cf || cpu.zf) { pc = 0x10a0f44; continue; }
  pc = 0x10a0e91; continue;
 }
 0x10a0e91 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if cpu.zf { pc = 0x10a0f87; continue; }
  pc = 0x10a0e9a; continue;
 }
 0x10a0e9a => {
  pc = 0x10a102a; continue;
 }
 0x10a0e9f => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x10a102a; continue; }
  pc = 0x10a0ea8; continue;
 }
 0x10a0ea8 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x11ee3a4u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a0ec8; continue; }
  pc = 0x10a0ebf; continue;
 }
 0x10a0ebf => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10a0f05; continue; }
  pc = 0x10a0ec8; continue;
 }
 0x10a0ec8 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a102a; continue; }
  pc = 0x10a0ef6; continue;
 }
 0x10a0ef6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x10a1024; continue;
 }
 0x10a0f05 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a0f0e); f_01092c75(cpu); if cpu.returned_to != 0x10a0f0e { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a102a; continue; }
  pc = 0x10a0f28; continue;
 }
 0x10a0f28 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.push(0x10a0f2e); cpu.call(t); if cpu.returned_to != 0x10a0f2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee3a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a0f44 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.eax)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11ee39cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee3a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a0f87 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x64u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee39cu32, v); }
  { let v: u32 = 0xf42400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ee2e0u32); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x11ee39cu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee398u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10a0fe2; continue; }
  pc = 0x10a0fce; continue;
 }
 0x10a0fce => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11ee39cu32, v); }
  { let v: u8 = cpu.r8(0x11ee39cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x10a0fea; continue;
 }
 0x10a0fe2 => {
  { let v: u8 = cpu.r8(0x11ee39cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x10a0fea; continue;
 }
 0x10a0fea => {
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a102a; continue; }
  pc = 0x10a1019; continue;
 }
 0x10a1019 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a1024; continue;
 }
 0x10a1024 => {
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x10a1027); cpu.call(t); if cpu.returned_to != 0x10a1027 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a102a; continue;
 }
 0x10a102a => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee3a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a103d => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = 0x11ee37cu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee37cu32, v); }
  cpu.push(0x10a1055); f_0109fe50(cpu); if cpu.returned_to != 0x10a1055 { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee3a8u32, v); }
  { let v: u32 = cpu.r32(0x11ee37cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee3a8u32, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  cpu.push(0x10a1085); f_0109fc20(cpu); if cpu.returned_to != 0x10a1085 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebp, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a1097; continue; }
  pc = 0x10a1090; continue;
 }
 0x10a1090 => {
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  pc = 0x10a1099; continue;
 }
 0x10a1097 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x10a1099; continue;
 }
 0x10a1099 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  cpu.push(0x10a10a8); f_010a6310(cpu); if cpu.returned_to != 0x10a10a8 { return; }
  { let v: u32 = cpu.r32(0x1a8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10a10bb); f_0109fc20(cpu); if cpu.returned_to != 0x10a10bb { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee3a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a10d6 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee380u32, v); }
  { let v: u32 = cpu.r32(0x11ee3a0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee384u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ee3a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x2u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx & 0xf0u32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11ee3a0u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a1120); f_010a0350(cpu); if cpu.returned_to != 0x10a1120 { return; }
  { let v: u32 = cpu.r32(0x11ee3a0u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.zf { pc = 0x10a11ae; continue; }
  pc = 0x10a1135; continue;
 }
 0x10a1135 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee380u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0e09; continue; }
  pc = 0x10a114b; continue;
 }
 0x10a114b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x10a1177; continue; }
  pc = 0x10a115a; continue;
 }
 0x10a115a => {
  if (cpu.cf || cpu.zf) { pc = 0x10a1190; continue; }
  pc = 0x10a115c; continue;
 }
 0x10a115c => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x10a116e; continue; }
  pc = 0x10a1163; continue;
 }
 0x10a1163 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a1197; continue; }
  pc = 0x10a1165; continue;
 }
 0x10a1165 => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x10a1190; continue; }
  pc = 0x10a116c; continue;
 }
 0x10a116c => {
  pc = 0x10a119e; continue;
 }
 0x10a116e => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x10a1197; continue; }
  pc = 0x10a1175; continue;
 }
 0x10a1175 => {
  pc = 0x10a119e; continue;
 }
 0x10a1177 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x10a1189; continue; }
  pc = 0x10a117e; continue;
 }
 0x10a117e => {
  if (cpu.cf || cpu.zf) { pc = 0x10a1190; continue; }
  pc = 0x10a1180; continue;
 }
 0x10a1180 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x10a1190; continue; }
  pc = 0x10a1187; continue;
 }
 0x10a1187 => {
  pc = 0x10a119e; continue;
 }
 0x10a1189 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x10a119e; continue; }
  pc = 0x10a1190; continue;
 }
 0x10a1190 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x10a11a0; continue;
 }
 0x10a1197 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x10a11a0; continue;
 }
 0x10a119e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a11a0; continue;
 }
 0x10a11a0 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a11ae => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee38cu32, v); }
  pc = 0x10a11be; continue;
 }
 0x10a11be => {
  { cpu.sub32(cpu.r32(0x558u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffffffffu32); }
  if cpu.zf { pc = 0x10a11df; continue; }
  pc = 0x10a11c8; continue;
 }
 0x10a11c8 => {
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee38cu32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a11be; continue; }
  pc = 0x10a11df; continue;
 }
 0x10a11df => {
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if !cpu.zf { pc = 0x10a1225; continue; }
  pc = 0x10a11ea; continue;
 }
 0x10a11ea => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138d04u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x10a120b); f_01092c75(cpu); if cpu.returned_to != 0x10a120b { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1134d90u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a1474; continue;
 }
 0x10a1225 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x554u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ee384u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x554u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x558u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5d8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a1267); f_0109fe50(cpu); if cpu.returned_to != 0x10a1267 { return; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x658u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x10a0e09; continue;
 }
 0x10a1282 => {
  { cpu.sub32(cpu.r32(0x11ee3a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a12a4; continue; }
  pc = 0x10a128b; continue;
 }
 0x10a128b => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a12a4; continue;
 }
 0x10a12a4 => {
  { cpu.sub32(cpu.r32(0x11ee3a4u32), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0d37; continue; }
  pc = 0x10a12b1; continue;
 }
 0x10a12b1 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a0d37; continue; }
  pc = 0x10a12cc; continue;
 }
 0x10a12cc => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x10a12ef; continue; }
  pc = 0x10a12e1; continue;
 }
 0x10a12e1 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a12ef; continue;
 }
 0x10a12ef => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x10a0d37; continue; }
  pc = 0x10a12fe; continue;
 }
 0x10a12fe => {
  { let v = 0x11ee390u32; cpu.push(v); }
  { let v = 0x11ee38cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a130e); f_01093c31(cpu); if cpu.returned_to != 0x10a130e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee390u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ee38cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x10a132b); cpu.call(t); if cpu.returned_to != 0x10a132b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.poll(); pc = 0x10a0d37; continue;
 }
 0x10a1333 => {
  { cpu.sub32(cpu.r32(0x11ee3a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a13f9; continue; }
  pc = 0x10a1340; continue;
 }
 0x10a1340 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a13a4; continue; }
  pc = 0x10a1348; continue;
 }
 0x10a1348 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a1355; continue;
 }
 0x10a1355 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x10a1390; continue; }
  pc = 0x10a1364; continue;
 }
 0x10a1364 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a137d; continue; }
  pc = 0x10a1375; continue;
 }
 0x10a1375 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a1383; continue;
 }
 0x10a137d => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a1383; continue;
 }
 0x10a1383 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x10a1355; continue; }
  pc = 0x10a1390; continue;
 }
 0x10a1390 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x34u32.wrapping_add(cpu.eax)), 0x7u8); }
  if !cpu.zf { pc = 0x10a13a4; continue; }
  pc = 0x10a139b; continue;
 }
 0x10a139b => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a13a1); f_010a0c80(cpu); if cpu.returned_to != 0x10a13a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10a13a4; continue;
 }
 0x10a13a4 => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a13f9; continue; }
  pc = 0x10a13b1; continue;
 }
 0x10a13b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a13be; continue;
 }
 0x10a13be => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10a13f9; continue; }
  pc = 0x10a13cd; continue;
 }
 0x10a13cd => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a13e6; continue; }
  pc = 0x10a13de; continue;
 }
 0x10a13de => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a13ec; continue;
 }
 0x10a13e6 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a13ec; continue;
 }
 0x10a13ec => {
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x10a13be; continue; }
  pc = 0x10a13f9; continue;
 }
 0x10a13f9 => {
  { let v: u32 = cpu.r32(0x11ee394u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11ee388u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x718u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee394u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11ee388u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10a0d0e; continue; }
  pc = 0x10a1420; continue;
 }
 0x10a1420 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a146c; continue; }
  pc = 0x10a142d; continue;
 }
 0x10a142d => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a1451); f_0108fe91(cpu); if cpu.returned_to != 0x10a1451 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.eax), v); }
  pc = 0x10a146c; continue;
 }
 0x10a146c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1134d90u32, v); }
  pc = 0x10a1474; continue;
 }
 0x10a1474 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1480); }
 let mut pc: u32 = 0x10a1480;
 loop { match pc {
 0x10a1480 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10a151e; continue; }
  pc = 0x10a14a8; continue;
 }
 0x10a14a8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a14cc; continue; }
  pc = 0x10a14c5; continue;
 }
 0x10a14c5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a14cc; continue;
 }
 0x10a14cc => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a14df; continue; }
  pc = 0x10a14d6; continue;
 }
 0x10a14d6 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a14df; continue;
 }
 0x10a14df => {
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a14f2; continue; }
  pc = 0x10a14e9; continue;
 }
 0x10a14e9 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a14f2; continue;
 }
 0x10a14f2 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a1505; continue; }
  pc = 0x10a14fc; continue;
 }
 0x10a14fc => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a1505; continue;
 }
 0x10a1505 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a1507; continue;
 }
 0x10a1507 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a1516; continue; }
  pc = 0x10a150e; continue;
 }
 0x10a150e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x10a1516; continue;
 }
 0x10a1516 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a1507; continue; }
  pc = 0x10a151e; continue;
 }
 0x10a151e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a1544); f_0108fe91(cpu); if cpu.returned_to != 0x10a1544 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1550); }
 let mut pc: u32 = 0x10a1550;
 loop { match pc {
 0x10a1550 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a157e; continue; }
  pc = 0x10a1560; continue;
 }
 0x10a1560 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a1562; continue;
 }
 0x10a1562 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x10a156e); f_01092d4f(cpu); if cpu.returned_to != 0x10a156e { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x718u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a1562; continue; }
  pc = 0x10a157e; continue;
 }
 0x10a157e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a1587); f_0109092e(cpu); if cpu.returned_to != 0x10a1587 { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x718u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a159b); f_01013c8c(cpu); if cpu.returned_to != 0x10a159b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a15a9); f_01013c8c(cpu); if cpu.returned_to != 0x10a15a9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a15b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a15b0); }
 let mut pc: u32 = 0x10a15b0;
 loop { match pc {
 0x10a15b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  cpu.push(0x10a15bb); f_0109faa0(cpu); if cpu.returned_to != 0x10a15bb { return; }
  { let v = 0x1d4u32; cpu.push(v); }
  cpu.push(0x10a15c5); f_01013c6b(cpu); if cpu.returned_to != 0x10a15c5 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a15e6; continue; }
  pc = 0x10a15ce; continue;
 }
 0x10a15ce => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138d24u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  pc = 0x10a1c4b; continue;
 }
 0x10a15e6 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x10a1622; continue; }
  pc = 0x10a15f6; continue;
 }
 0x10a15f6 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138d4cu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a1616); f_01013c8c(cpu); if cpu.returned_to != 0x10a1616 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1622 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1634); f_0108fe91(cpu); if cpu.returned_to != 0x10a1634 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  if cpu.zf { pc = 0x10a16a5; continue; }
  pc = 0x10a167f; continue;
 }
 0x10a167f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10a16a5; continue; }
  pc = 0x10a1684; continue;
 }
 0x10a1684 => {
  cpu.push(0x10a1689); f_0108cfa7(cpu); if cpu.returned_to != 0x10a1689 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a16a5; continue; }
  pc = 0x10a168d; continue;
 }
 0x10a168d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10a16a5; continue; }
  pc = 0x10a1692; continue;
 }
 0x10a1692 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a16a5); f_01098c3e(cpu); if cpu.returned_to != 0x10a16a5 { return; }
  pc = 0x10a16a5; continue;
 }
 0x10a16a5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.esi); cpu.edx = v; }
  if cpu.zf { pc = 0x10a171a; continue; }
  pc = 0x10a16be; continue;
 }
 0x10a16be => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x10a171a; continue; }
  pc = 0x10a16c3; continue;
 }
 0x10a16c3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1134d10u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a16d2; continue;
 }
 0x10a16d2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a16ea; continue; }
  pc = 0x10a16da; continue;
 }
 0x10a16da => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a16d2; continue; }
  pc = 0x10a16ea; continue;
 }
 0x10a16ea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a16ff; continue;
 }
 0x10a16ff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a1717; continue; }
  pc = 0x10a1707; continue;
 }
 0x10a1707 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a16ff; continue; }
  pc = 0x10a1717; continue;
 }
 0x10a1717 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x10a1724; continue;
 }
 0x10a171a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a1724; continue;
 }
 0x10a1724 => {
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x11ee298u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x10a173e); f_0108f030(cpu); if cpu.returned_to != 0x10a173e { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a177a; continue; }
  pc = 0x10a1746; continue;
 }
 0x10a1746 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a175d); f_010a1480(cpu); if cpu.returned_to != 0x10a175d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a177a; continue; }
  pc = 0x10a1764; continue;
 }
 0x10a1764 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x10a177a; continue;
 }
 0x10a177a => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10a17c7; continue; }
  pc = 0x10a1781; continue;
 }
 0x10a1781 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a178a); f_010909fd(cpu); if cpu.returned_to != 0x10a178a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a17c7; continue; }
  pc = 0x10a1795; continue;
 }
 0x10a1795 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10a17aa); f_010a1480(cpu); if cpu.returned_to != 0x10a17aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a17c7; continue; }
  pc = 0x10a17b1; continue;
 }
 0x10a17b1 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  pc = 0x10a17c7; continue;
 }
 0x10a17c7 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a186f; continue; }
  pc = 0x10a17d3; continue;
 }
 0x10a17d3 => {
  { cpu.sub32(cpu.r32(0x11ee2f8u32), 0x1u32); }
  if !cpu.zf { pc = 0x10a186f; continue; }
  pc = 0x10a17e0; continue;
 }
 0x10a17e0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x10a1857; continue;
 }
 0x10a17ea => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10a1819; continue; }
  pc = 0x10a180b; continue;
 }
 0x10a180b => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x10a1819; continue;
 }
 0x10a1819 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1824); f_010a1480(cpu); if cpu.returned_to != 0x10a1824 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a1843; continue; }
  pc = 0x10a182b; continue;
 }
 0x10a182b => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x10a186f; continue;
 }
 0x10a1843 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x18u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a1857; continue;
 }
 0x10a1857 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a17ea; continue; }
  pc = 0x10a186f; continue;
 }
 0x10a186f => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10a189e; continue; }
  pc = 0x10a1876; continue;
 }
 0x10a1876 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138d64u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10a1892); f_01013c8c(cpu); if cpu.returned_to != 0x10a1892 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a189e => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a18c0); f_0108fe91(cpu); if cpu.returned_to != 0x10a18c0 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x501u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a18e2); f_0108fe91(cpu); if cpu.returned_to != 0x10a18e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x10a1935; continue; }
  pc = 0x10a18ed; continue;
 }
 0x10a18ed => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138d84u32; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a1911); f_0108fe91(cpu); if cpu.returned_to != 0x10a1911 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a1929); f_01013c8c(cpu); if cpu.returned_to != 0x10a1929 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1935 => {
  { let v: u32 = cpu.r32(0x11ee2e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x718u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1949); f_01013c6b(cpu); if cpu.returned_to != 0x10a1949 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1998; continue; }
  pc = 0x10a1953; continue;
 }
 0x10a1953 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138db0u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1974); f_0108fe91(cpu); if cpu.returned_to != 0x10a1974 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a198c); f_01013c8c(cpu); if cpu.returned_to != 0x10a198c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1998 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a19c1; continue; }
  pc = 0x10a19a1; continue;
 }
 0x10a19a1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a19a3; continue;
 }
 0x10a19a3 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x718u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a19a3; continue; }
  pc = 0x10a19c1; continue;
 }
 0x10a19c1 => {
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ee2e0u32); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a1a20; continue;
 }
 0x10a1a20 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x9cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xdcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15cu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a1a20; continue; }
  pc = 0x10a1a45; continue;
 }
 0x10a1a45 => {
  { let v = 0x10a0cc0u32; cpu.push(v); }
  cpu.push(0x10a1a4f); f_01090372(cpu); if cpu.returned_to != 0x10a1a4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a1ab7; continue; }
  pc = 0x10a1a5a; continue;
 }
 0x10a1a5a => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138dd8u32; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1a7f); f_0108fe91(cpu); if cpu.returned_to != 0x10a1a7f { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x718u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a1a9d); f_01013c8c(cpu); if cpu.returned_to != 0x10a1a9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1d4u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1aab); f_01013c8c(cpu); if cpu.returned_to != 0x10a1aab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1ab7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1abe); f_0109045d(cpu); if cpu.returned_to != 0x10a1abe { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x10a1550u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  {  }
  pc = 0x10a1ae0; continue;
 }
 0x10a1ae0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16) | 0xb0u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1af2); f_0109fc70(cpu); if cpu.returned_to != 0x10a1af2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b04); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b04 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b16); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b16 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x70u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b24); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b24 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b32); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee2e8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b44); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b44 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b52); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b60); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b60 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b6e); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = 0x5bu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b7c); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b7c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5du32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b8a); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1b98); f_0109fc70(cpu); if cpu.returned_to != 0x10a1b98 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x65u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1ba6); f_0109fc70(cpu); if cpu.returned_to != 0x10a1ba6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1bb4); f_0109fc70(cpu); if cpu.returned_to != 0x10a1bb4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee2f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a1bc7); f_0109fc70(cpu); if cpu.returned_to != 0x10a1bc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a1c0b; continue; }
  pc = 0x10a1bd3; continue;
 }
 0x10a1bd3 => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a1bf4); f_0108fe91(cpu); if cpu.returned_to != 0x10a1bf4 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a1c0b; continue;
 }
 0x10a1c0b => {
  { cpu.and16((cpu.edi as u16), 0x3u16); }
  if !cpu.zf { pc = 0x10a1c20; continue; }
  pc = 0x10a1c12; continue;
 }
 0x10a1c12 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a1c19); f_0108ffa7(cpu); if cpu.returned_to != 0x10a1c19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10a1c20; continue;
 }
 0x10a1c20 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a1ae0; continue; }
  pc = 0x10a1c2a; continue;
 }
 0x10a1c2a => {
  { let v: u32 = cpu.r32(0x11ee2e0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a1c3a); f_010905c6(cpu); if cpu.returned_to != 0x10a1c3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1c46); f_01090790(cpu); if cpu.returned_to != 0x10a1c46 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10a1c4b; continue;
 }
 0x10a1c4b => {
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1c60); }
 let mut pc: u32 = 0x10a1c60;
 loop { match pc {
 0x10a1c60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1c6e); f_01014423(cpu); if cpu.returned_to != 0x10a1c6e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1c8e; continue; }
  pc = 0x10a1c77; continue;
 }
 0x10a1c77 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138df0u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1c8e => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1c94); f_010143d7(cpu); if cpu.returned_to != 0x10a1c94 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a1c9e); f_01090ae2(cpu); if cpu.returned_to != 0x10a1c9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x110de9cu32); cpu.push(0x10a1cab); cpu.call(t); if cpu.returned_to != 0x10a1cab { return; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10a1cb4; continue; }
  pc = 0x10a1caf; continue;
 }
 0x10a1caf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1cb4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1cbf); f_010a15b0(cpu); if cpu.returned_to != 0x10a1cbf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1cd1; continue; }
  pc = 0x10a1cc8; continue;
 }
 0x10a1cc8 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a1cce); f_01090bd5(cpu); if cpu.returned_to != 0x10a1cce { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10a1cd1; continue;
 }
 0x10a1cd1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1ce0); }
 let mut pc: u32 = 0x10a1ce0;
 loop { match pc {
 0x10a1ce0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a1cf1); f_01090ae2(cpu); if cpu.returned_to != 0x10a1cf1 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a1d19; continue; }
  pc = 0x10a1cfa; continue;
 }
 0x10a1cfa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1d05); f_010a15b0(cpu); if cpu.returned_to != 0x10a1d05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1d17; continue; }
  pc = 0x10a1d0e; continue;
 }
 0x10a1d0e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a1d14); f_01090bd5(cpu); if cpu.returned_to != 0x10a1d14 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10a1d17; continue;
 }
 0x10a1d17 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10a1d19; continue;
 }
 0x10a1d19 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1d20); }
 let mut pc: u32 = 0x10a1d20;
 loop { match pc {
 0x10a1d20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x1138e08u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1d37); f_01090019(cpu); if cpu.returned_to != 0x10a1d37 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1d57; continue; }
  pc = 0x10a1d3e; continue;
 }
 0x10a1d3e => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138e10u32; cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  pc = 0x10a1d8c; continue;
 }
 0x10a1d57 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a1d6c); f_01092863(cpu); if cpu.returned_to != 0x10a1d6c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1d8a; continue; }
  pc = 0x10a1d7c; continue;
 }
 0x10a1d7c => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1d8a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a1d8c; continue;
 }
 0x10a1d8c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1da0); }
 let mut pc: u32 = 0x10a1da0;
 loop { match pc {
 0x10a1da0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a1dac); f_01090bd5(cpu); if cpu.returned_to != 0x10a1dac { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1db0); }
 let mut pc: u32 = 0x10a1db0;
 loop { match pc {
 0x10a1db0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x112u32); }
  if cpu.cf { pc = 0x10a1df4; continue; }
  pc = 0x10a1dc2; continue;
 }
 0x10a1dc2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0xbau32); cpu.ebx = v; }
  { let v: u32 = 0x1138e30u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a1dd4); f_0108ddc3(cpu); if cpu.returned_to != 0x10a1dd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1dde; continue; }
  pc = 0x10a1dd8; continue;
 }
 0x10a1dd8 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1dde => {
  { let v: u32 = 0x1138e44u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a1dea); f_0108ddc3(cpu); if cpu.returned_to != 0x10a1dea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1df4; continue; }
  pc = 0x10a1dee; continue;
 }
 0x10a1dee => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1df4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x10a1e49; continue; }
  pc = 0x10a1e0d; continue;
 }
 0x10a1e0d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10a1e49; continue; }
  pc = 0x10a1e12; continue;
 }
 0x10a1e12 => {
  { let v: u32 = 0x1138e60u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a1e1e); f_0108ddc3(cpu); if cpu.returned_to != 0x10a1e1e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1e33; continue; }
  pc = 0x10a1e22; continue;
 }
 0x10a1e22 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x5000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a1e49; continue; }
  pc = 0x10a1e2d; continue;
 }
 0x10a1e2d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1e33 => {
  { let v: u32 = 0x1138e64u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a1e3f); f_0108ddc3(cpu); if cpu.returned_to != 0x10a1e3f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1e49; continue; }
  pc = 0x10a1e43; continue;
 }
 0x10a1e43 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1e49 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1e50); }
 let mut pc: u32 = 0x10a1e50;
 loop { match pc {
 0x10a1e50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x1134d10u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a1e5c; continue;
 }
 0x10a1e5c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a1e74; continue; }
  pc = 0x10a1e64; continue;
 }
 0x10a1e64 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a1e5c; continue; }
  pc = 0x10a1e74; continue;
 }
 0x10a1e74 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a1ec5; continue; }
  pc = 0x10a1e86; continue;
 }
 0x10a1e86 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1134d10u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if !cpu.zf { pc = 0x10a1eb2; continue; }
  pc = 0x10a1e93; continue;
 }
 0x10a1e93 => {
  { let v: u32 = 0x1134d10u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.zf { pc = 0x10a1ec5; continue; }
  pc = 0x10a1ea7; continue;
 }
 0x10a1ea7 => {
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1134d10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1eb2 => {
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x10a1ec2; continue; }
  pc = 0x10a1eb7; continue;
 }
 0x10a1eb7 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1134d10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1ec2 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10a1e86; continue; }
  pc = 0x10a1ec5; continue;
 }
 0x10a1ec5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1ed0); }
 let mut pc: u32 = 0x10a1ed0;
 loop { match pc {
 0x10a1ed0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x10a1edb); f_0108fc12(cpu); if cpu.returned_to != 0x10a1edb { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a1efb; continue; }
  pc = 0x10a1ee4; continue;
 }
 0x10a1ee4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x10a1ee7; continue;
 }
 0x10a1ee7 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x10a1efb; continue; }
  pc = 0x10a1eed; continue;
 }
 0x10a1eed => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x718u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a1ee7; continue; }
  pc = 0x10a1efb; continue;
 }
 0x10a1efb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x10a1f1c; continue; }
  pc = 0x10a1f00; continue;
 }
 0x10a1f00 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138e6cu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  cpu.push(0x10a1f17); f_0108fc17(cpu); if cpu.returned_to != 0x10a1f17 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1f1c => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x718u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10a1f34); f_010a0ba0(cpu); if cpu.returned_to != 0x10a1f34 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10a1f43); f_0108fc17(cpu); if cpu.returned_to != 0x10a1f43 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1f50); }
 let mut pc: u32 = 0x10a1f50;
 loop { match pc {
 0x10a1f50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a1f69; continue; }
  pc = 0x10a1f59; continue;
 }
 0x10a1f59 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a1f5f); f_01092c75(cpu); if cpu.returned_to != 0x10a1f5f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a1f69; continue;
 }
 0x10a1f69 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a1f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a1f70); }
 let mut pc: u32 = 0x10a1f70;
 loop { match pc {
 0x10a1f70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10a1f85; continue; }
  pc = 0x10a1f7e; continue;
 }
 0x10a1f7e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a234a; continue;
 }
 0x10a1f85 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10a1f9b); f_0109fec0(cpu); if cpu.returned_to != 0x10a1f9b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a1fba; continue; }
  pc = 0x10a1fa2; continue;
 }
 0x10a1fa2 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138e88u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a1fba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx & 0xff00u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 24u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff000000u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 24u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10a20aa; continue; }
  pc = 0x10a2017; continue;
 }
 0x10a2017 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138ea0u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a2028); f_0108f5f5(cpu); if cpu.returned_to != 0x10a2028 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2033; continue; }
  pc = 0x10a202c; continue;
 }
 0x10a202c => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a2033; continue;
 }
 0x10a2033 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138ea8u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a2044); f_0108f5f5(cpu); if cpu.returned_to != 0x10a2044 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a204f; continue; }
  pc = 0x10a2048; continue;
 }
 0x10a2048 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x10a204f; continue;
 }
 0x10a204f => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138eb0u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a2060); f_0108f5f5(cpu); if cpu.returned_to != 0x10a2060 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a206b; continue; }
  pc = 0x10a2064; continue;
 }
 0x10a2064 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a206b; continue;
 }
 0x10a206b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff0000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { cpu.poll(); pc = 0x10a2017; continue; }
  pc = 0x10a20aa; continue;
 }
 0x10a20aa => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10a20ce; continue; }
  pc = 0x10a20b4; continue;
 }
 0x10a20b4 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138e88u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a20ce => {
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a2100); f_010a0ba0(cpu); if cpu.returned_to != 0x10a2100 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x48u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x4cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x11ee2e8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a2345; continue; }
  pc = 0x10a2157; continue;
 }
 0x10a2157 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a2160); f_0109fe80(cpu); if cpu.returned_to != 0x10a2160 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a2174; continue; }
  pc = 0x10a216d; continue;
 }
 0x10a216d => {
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  pc = 0x10a2188; continue;
 }
 0x10a2174 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a2182); f_0109fe80(cpu); if cpu.returned_to != 0x10a2182 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10a2188; continue;
 }
 0x10a2188 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x11ee3b0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x10a2199); f_010a6310(cpu); if cpu.returned_to != 0x10a2199 { return; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x11ee3b0u32; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a2251; continue; }
  pc = 0x10a21b2; continue;
 }
 0x10a21b2 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10a21b6; continue;
 }
 0x10a21b6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x10a2251; continue; }
  pc = 0x10a21c4; continue;
 }
 0x10a21c4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.push(0x10a21ec); cpu.call(t); if cpu.returned_to != 0x10a21ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a21fd; continue; }
  pc = 0x10a21f6; continue;
 }
 0x10a21f6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x10a21b6; continue;
 }
 0x10a21fd => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x11ee3b0u32); cpu.eax = v; }
  pc = 0x10a2219; continue;
 }
 0x10a220d => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a2219; continue;
 }
 0x10a2219 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x10a220d; continue; }
  pc = 0x10a2223; continue;
 }
 0x10a2223 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a2244; continue; }
  pc = 0x10a2234; continue;
 }
 0x10a2234 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a21b6; continue;
 }
 0x10a2244 => {
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a21b6; continue;
 }
 0x10a2251 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10a2345; continue; }
  pc = 0x10a225c; continue;
 }
 0x10a225c => {
  cpu.push(0x10a2261); f_01090014(cpu); if cpu.returned_to != 0x10a2261 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a2282; continue; }
  pc = 0x10a2265; continue;
 }
 0x10a2265 => {
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138eb8u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2282 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1a8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a22d1; continue; }
  pc = 0x10a229a; continue;
 }
 0x10a229a => {
  { let v: u16 = cpu.r16(0x1a8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a22ba); f_0108fe91(cpu); if cpu.returned_to != 0x10a22ba { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.esi), v); }
  pc = 0x10a22d1; continue;
 }
 0x10a22d1 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  cpu.push(0x10a22eb); f_010a6310(cpu); if cpu.returned_to != 0x10a22eb { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = 0x503u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a2303); f_0108fe91(cpu); if cpu.returned_to != 0x10a2303 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x10a2345; continue; }
  pc = 0x10a2316; continue;
 }
 0x10a2316 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1138eccu32; cpu.push(v); }
  { let v = 0x11ee198u32; cpu.push(v); }
  cpu.push(0x10a2336); f_0108dece(cpu); if cpu.returned_to != 0x10a2336 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2345 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a234a; continue;
 }
 0x10a234a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2360); }
 let mut pc: u32 = 0x10a2360;
 loop { match pc {
 0x10a2360 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a2391; continue; }
  pc = 0x10a2369; continue;
 }
 0x10a2369 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x10a2391; continue; }
  pc = 0x10a236f; continue;
 }
 0x10a236f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a2375); f_01092c75(cpu); if cpu.returned_to != 0x10a2375 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a237e); f_010a0ba0(cpu); if cpu.returned_to != 0x10a237e { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a2391; continue;
 }
 0x10a2391 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a23a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a23a0); }
 let mut pc: u32 = 0x10a23a0;
 loop { match pc {
 0x10a23a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a245a; continue; }
  pc = 0x10a23b2; continue;
 }
 0x10a23b2 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x4u32); }
  if !cpu.zf { pc = 0x10a245a; continue; }
  pc = 0x10a23bc; continue;
 }
 0x10a23bc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  cpu.push(0x10a23ce); f_010a07e0(cpu); if cpu.returned_to != 0x10a23ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a23e0; continue;
 }
 0x10a23e0 => {
  { let v: u32 = cpu.r32(0x454u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a2403; continue; }
  pc = 0x10a23f1; continue;
 }
 0x10a23f1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a2400); f_0109fc70(cpu); if cpu.returned_to != 0x10a2400 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a2403; continue;
 }
 0x10a2403 => {
  { cpu.sub32(cpu.r32(0x1d4u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a2414; continue; }
  pc = 0x10a240c; continue;
 }
 0x10a240c => {
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  pc = 0x10a2414; continue;
 }
 0x10a2414 => {
  { cpu.sub32(cpu.r32(0x254u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a242f; continue; }
  pc = 0x10a241d; continue;
 }
 0x10a241d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a242c); f_0109fc70(cpu); if cpu.returned_to != 0x10a242c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a242f; continue;
 }
 0x10a242f => {
  { cpu.sub32(cpu.r32(0x194u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a2450; continue; }
  pc = 0x10a2438; continue;
 }
 0x10a2438 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a2440); f_01094607(cpu); if cpu.returned_to != 0x10a2440 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  {  }
  pc = 0x10a2450; continue;
 }
 0x10a2450 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a23e0; continue; }
  pc = 0x10a245a; continue;
 }
 0x10a245a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2470); }
 let mut pc: u32 = 0x10a2470;
 loop { match pc {
 0x10a2470 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a24f7; continue; }
  pc = 0x10a2482; continue;
 }
 0x10a2482 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebp)), 0x8u32); }
  if !cpu.zf { pc = 0x10a24f7; continue; }
  pc = 0x10a248c; continue;
 }
 0x10a248c => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a24a0; continue;
 }
 0x10a24a0 => {
  { cpu.sub32(cpu.r32(0x194u32.wrapping_add(cpu.edi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a24d0; continue; }
  pc = 0x10a24a9; continue;
 }
 0x10a24a9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a24b2); f_01094522(cpu); if cpu.returned_to != 0x10a24b2 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a24bd; continue; }
  pc = 0x10a24bb; continue;
 }
 0x10a24bb => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10a24bd; continue;
 }
 0x10a24bd => {
  { let v: u32 = cpu.eax; cpu.w32(0x94u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  {  }
  pc = 0x10a24d0; continue;
 }
 0x10a24d0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a24a0; continue; }
  pc = 0x10a24d9; continue;
 }
 0x10a24d9 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  pc = 0x10a24e0; continue;
 }
 0x10a24e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a24e7); f_010a0960(cpu); if cpu.returned_to != 0x10a24e7 { return; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a24e0; continue; }
  pc = 0x10a24f0; continue;
 }
 0x10a24f0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a24f7; continue;
 }
 0x10a24f7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2500); }
 let mut pc: u32 = 0x10a2500;
 loop { match pc {
 0x10a2500 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a2534; continue; }
  pc = 0x10a2509; continue;
 }
 0x10a2509 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a2534; continue; }
  pc = 0x10a2511; continue;
 }
 0x10a2511 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x10a2534; continue; }
  pc = 0x10a2516; continue;
 }
 0x10a2516 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a251c); f_01092c75(cpu); if cpu.returned_to != 0x10a251c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a2534; continue; }
  pc = 0x10a252d; continue;
 }
 0x10a252d => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.push(0x10a2531); cpu.call(t); if cpu.returned_to != 0x10a2531 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10a2534; continue;
 }
 0x10a2534 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2540); }
 let mut pc: u32 = 0x10a2540;
 loop { match pc {
 0x10a2540 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a254f; continue; }
  pc = 0x10a2548; continue;
 }
 0x10a2548 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10a254f; continue;
 }
 0x10a254f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2550); }
 let mut pc: u32 = 0x10a2550;
 loop { match pc {
 0x10a2550 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a25b7; continue; }
  pc = 0x10a255d; continue;
 }
 0x10a255d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10a25b2; continue; }
  pc = 0x10a2573; continue;
 }
 0x10a2573 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10a257c; continue; }
  pc = 0x10a2577; continue;
 }
 0x10a2577 => {
  { let v: u32 = cpu.esi; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a25b2; continue;
 }
 0x10a257c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a25b2; continue;
 }
 0x10a25b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a25b7; continue;
 }
 0x10a25b7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a25c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a25c0); }
 let mut pc: u32 = 0x10a25c0;
 loop { match pc {
 0x10a25c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a263d; continue; }
  pc = 0x10a25ce; continue;
 }
 0x10a25ce => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x10a25ec; continue; }
  pc = 0x10a25e4; continue;
 }
 0x10a25e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a25ec => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10a25f5; continue; }
  pc = 0x10a25f0; continue;
 }
 0x10a25f0 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a262b; continue;
 }
 0x10a25f5 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a262b; continue;
 }
 0x10a262b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a2631); f_010a0c80(cpu); if cpu.returned_to != 0x10a2631 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a263d; continue;
 }
 0x10a263d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2640); }
 let mut pc: u32 = 0x10a2640;
 loop { match pc {
 0x10a2640 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2649; continue; }
  pc = 0x10a2648; continue;
 }
 0x10a2648 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2649 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2690); }
 let mut pc: u32 = 0x10a2690;
 loop { match pc {
 0x10a2690 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1b0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10a26d7; continue; }
  pc = 0x10a26a3; continue;
 }
 0x10a26a3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a26d4; continue; }
  pc = 0x10a26ba; continue;
 }
 0x10a26ba => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x4u32); }
  if !cpu.zf { pc = 0x10a26c9; continue; }
  pc = 0x10a26c0; continue;
 }
 0x10a26c0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a26c6); f_010a0c80(cpu); if cpu.returned_to != 0x10a26c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10a26c9; continue;
 }
 0x10a26c9 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x718u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10a26ba; continue; }
  pc = 0x10a26d4; continue;
 }
 0x10a26d4 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a26d7; continue;
 }
 0x10a26d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a26e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a26e0); }
 let mut pc: u32 = 0x10a26e0;
 loop { match pc {
 0x10a26e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2950); }
 let mut pc: u32 = 0x10a2950;
 loop { match pc {
 0x10a2950 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a29ba; continue; }
  pc = 0x10a295b; continue;
 }
 0x10a295b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ee2ecu32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a299b; continue; }
  pc = 0x10a2979; continue;
 }
 0x10a2979 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10a2990; continue; }
  pc = 0x10a2985; continue;
 }
 0x10a2985 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x64u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x10a2990; continue; }
  pc = 0x10a298d; continue;
 }
 0x10a298d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a2990; continue;
 }
 0x10a2990 => {
  { let v: u32 = cpu.r32(0x11ee2ecu32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a2979; continue; }
  pc = 0x10a299b; continue;
 }
 0x10a299b => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10a29a1; continue; }
  pc = 0x10a299f; continue;
 }
 0x10a299f => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a29a1; continue;
 }
 0x10a29a1 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a29ab; continue; }
  pc = 0x10a29a9; continue;
 }
 0x10a29a9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a29ab; continue;
 }
 0x10a29ab => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a29b5; continue; }
  pc = 0x10a29b3; continue;
 }
 0x10a29b3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a29b5; continue;
 }
 0x10a29b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a29ba; continue;
 }
 0x10a29ba => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a29c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a29c0); }
 let mut pc: u32 = 0x10a29c0;
 loop { match pc {
 0x10a29c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a2a48; continue; }
  pc = 0x10a29d3; continue;
 }
 0x10a29d3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a2a48; continue; }
  pc = 0x10a29de; continue;
 }
 0x10a29de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a2a00; continue; }
  pc = 0x10a29ed; continue;
 }
 0x10a29ed => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x10a29ef; continue;
 }
 0x10a29ef => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.zf { pc = 0x10a2a00; continue; }
  pc = 0x10a29f8; continue;
 }
 0x10a29f8 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a29ef; continue; }
  pc = 0x10a2a00; continue;
 }
 0x10a2a00 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x10a2a48; continue; }
  pc = 0x10a2a04; continue;
 }
 0x10a2a04 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee2f0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10a2a48; continue; }
  pc = 0x10a2a32; continue;
 }
 0x10a2a32 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.edx = v; }
  pc = 0x10a2a37; continue;
 }
 0x10a2a37 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2a37; continue; }
  pc = 0x10a2a48; continue;
 }
 0x10a2a48 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2a50); }
 let mut pc: u32 = 0x10a2a50;
 loop { match pc {
 0x10a2a50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2a59; continue; }
  pc = 0x10a2a58; continue;
 }
 0x10a2a58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2a59 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2a70); }
 let mut pc: u32 = 0x10a2a70;
 loop { match pc {
 0x10a2a70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2a79; continue; }
  pc = 0x10a2a78; continue;
 }
 0x10a2a78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2a79 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2a90); }
 let mut pc: u32 = 0x10a2a90;
 loop { match pc {
 0x10a2a90 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2a99; continue; }
  pc = 0x10a2a98; continue;
 }
 0x10a2a98 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2a99 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2ad0); }
 let mut pc: u32 = 0x10a2ad0;
 loop { match pc {
 0x10a2ad0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1a0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1a0u32.wrapping_add(cpu.edx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2af0); }
 let mut pc: u32 = 0x10a2af0;
 loop { match pc {
 0x10a2af0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1a4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1a4u32.wrapping_add(cpu.edx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2b60); }
 let mut pc: u32 = 0x10a2b60;
 loop { match pc {
 0x10a2b60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10a2b88; continue;
 }
 0x10a2b88 => {
  { cpu.sub32(cpu.esi, 0x9u32); }
  if cpu.zf { pc = 0x10a2ba7; continue; }
  pc = 0x10a2b8d; continue;
 }
 0x10a2b8d => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x10a2ba7; continue; }
  pc = 0x10a2b94; continue;
 }
 0x10a2b94 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { pc = 0x10a2ba7; continue; }
  pc = 0x10a2b99; continue;
 }
 0x10a2b99 => {
  { let v: u32 = cpu.r32(0x160u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a2ba7; continue; }
  pc = 0x10a2ba3; continue;
 }
 0x10a2ba3 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10a2ba7; continue;
 }
 0x10a2ba7 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a2b88; continue; }
  pc = 0x10a2bb0; continue;
 }
 0x10a2bb0 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a2be7; continue; }
  pc = 0x10a2bb5; continue;
 }
 0x10a2bb5 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10a2bc6; continue;
 }
 0x10a2bc6 => {
  { cpu.sub32(cpu.esi, 0x9u32); }
  if cpu.zf { pc = 0x10a2bde; continue; }
  pc = 0x10a2bcb; continue;
 }
 0x10a2bcb => {
  { cpu.sub32(cpu.edx, cpu.r32(0x20u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a2bde; continue; }
  pc = 0x10a2bd0; continue;
 }
 0x10a2bd0 => {
  { let v: u32 = cpu.r32(0x160u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a2bde; continue; }
  pc = 0x10a2bda; continue;
 }
 0x10a2bda => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10a2bde; continue;
 }
 0x10a2bde => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a2bc6; continue; }
  pc = 0x10a2be7; continue;
 }
 0x10a2be7 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a2bfe; continue; }
  pc = 0x10a2bec; continue;
 }
 0x10a2bec => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a2cf8; continue;
 }
 0x10a2bfe => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a2c11); f_0109fc70(cpu); if cpu.returned_to != 0x10a2c11 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a2cbe; continue; }
  pc = 0x10a2c2e; continue;
 }
 0x10a2c2e => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a2c37; continue;
 }
 0x10a2c37 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x1u32); }
  if cpu.zf { pc = 0x10a2c9b; continue; }
  pc = 0x10a2c3d; continue;
 }
 0x10a2c3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a2c50; continue;
 }
 0x10a2c50 => {
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a2c90; continue; }
  pc = 0x10a2c5b; continue;
 }
 0x10a2c5b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x94u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x10a2c90; continue; }
  pc = 0x10a2c66; continue;
 }
 0x10a2c66 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a2c7c); f_010a0350(cpu); if cpu.returned_to != 0x10a2c7c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  {  }
  pc = 0x10a2c90; continue;
 }
 0x10a2c90 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2c50; continue; }
  pc = 0x10a2c9b; continue;
 }
 0x10a2c9b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x718u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x718u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2c37; continue; }
  pc = 0x10a2cbe; continue;
 }
 0x10a2cbe => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x120u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xe0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xa0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a2cf8; continue;
 }
 0x10a2cf8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2d00); }
 let mut pc: u32 = 0x10a2d00;
 loop { match pc {
 0x10a2d00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x10a2e20; continue; }
  pc = 0x10a2d28; continue;
 }
 0x10a2d28 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x120u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0xe0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x10a2d53); f_0109fc70(cpu); if cpu.returned_to != 0x10a2d53 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a2de6; continue; }
  pc = 0x10a2d6c; continue;
 }
 0x10a2d6c => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a2d75; continue;
 }
 0x10a2d75 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x1u32); }
  if cpu.zf { pc = 0x10a2dc7; continue; }
  pc = 0x10a2d7b; continue;
 }
 0x10a2d7b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x10a2d80; continue;
 }
 0x10a2d80 => {
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a2dc0; continue; }
  pc = 0x10a2d8b; continue;
 }
 0x10a2d8b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x94u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x10a2dc0; continue; }
  pc = 0x10a2d9a; continue;
 }
 0x10a2d9a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a2db0); f_010a0350(cpu); if cpu.returned_to != 0x10a2db0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10a2dc0; continue;
 }
 0x10a2dc0 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2d80; continue; }
  pc = 0x10a2dc7; continue;
 }
 0x10a2dc7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x718u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x718u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2d75; continue; }
  pc = 0x10a2de6; continue;
 }
 0x10a2de6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a2e19; continue; }
  pc = 0x10a2e01; continue;
 }
 0x10a2e01 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x10a2e19; continue; }
  pc = 0x10a2e0b; continue;
 }
 0x10a2e0b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a2e16); f_010a0960(cpu); if cpu.returned_to != 0x10a2e16 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10a2e19; continue;
 }
 0x10a2e19 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x14u32.wrapping_add(cpu.eax))); cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a2e20; continue;
 }
 0x10a2e20 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2e30); }
 let mut pc: u32 = 0x10a2e30;
 loop { match pc {
 0x10a2e30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a2e63; continue; }
  pc = 0x10a2e3c; continue;
 }
 0x10a2e3c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x90u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x10a2e63; continue; }
  pc = 0x10a2e5b; continue;
 }
 0x10a2e5b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5cu32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x10a2e63; continue; }
  pc = 0x10a2e60; continue;
 }
 0x10a2e60 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5cu32.wrapping_add(cpu.edx), v); }
  pc = 0x10a2e63; continue;
 }
 0x10a2e63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2e70); }
 let mut pc: u32 = 0x10a2e70;
 loop { match pc {
 0x10a2e70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10a2e7b; continue; }
  pc = 0x10a2e78; continue;
 }
 0x10a2e78 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a2e7b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3010); }
 let mut pc: u32 = 0x10a3010;
 loop { match pc {
 0x10a3010 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d94u32), 0x0u32); }
  if cpu.zf { pc = 0x10a310f; continue; }
  pc = 0x10a3020; continue;
 }
 0x10a3020 => {
  { let v = 0x10a3010u32; cpu.push(v); }
  { let v = 0x109faa0u32; cpu.push(v); }
  cpu.push(0x10a302f); f_01013dc7(cpu); if cpu.returned_to != 0x10a302f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x1134d10u32; cpu.push(v); }
  cpu.push(0x10a3041); f_01013e03(cpu); if cpu.returned_to != 0x10a3041 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134d90u32; cpu.push(v); }
  cpu.push(0x10a3050); f_01013e03(cpu); if cpu.returned_to != 0x10a3050 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee388u32; cpu.push(v); }
  cpu.push(0x10a305f); f_01013e03(cpu); if cpu.returned_to != 0x10a305f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee38cu32; cpu.push(v); }
  cpu.push(0x10a306e); f_01013e03(cpu); if cpu.returned_to != 0x10a306e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee394u32; cpu.push(v); }
  cpu.push(0x10a307d); f_01013e03(cpu); if cpu.returned_to != 0x10a307d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a4u32; cpu.push(v); }
  cpu.push(0x10a308c); f_01013e03(cpu); if cpu.returned_to != 0x10a308c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee398u32; cpu.push(v); }
  cpu.push(0x10a309b); f_01013e03(cpu); if cpu.returned_to != 0x10a309b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee39cu32; cpu.push(v); }
  cpu.push(0x10a30aa); f_01013e03(cpu); if cpu.returned_to != 0x10a30aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee384u32; cpu.push(v); }
  cpu.push(0x10a30b9); f_01013e03(cpu); if cpu.returned_to != 0x10a30b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a0u32; cpu.push(v); }
  cpu.push(0x10a30c8); f_01013e03(cpu); if cpu.returned_to != 0x10a30c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3acu32; cpu.push(v); }
  cpu.push(0x10a30d7); f_01013e03(cpu); if cpu.returned_to != 0x10a30d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee3a8u32; cpu.push(v); }
  cpu.push(0x10a30e6); f_01013e03(cpu); if cpu.returned_to != 0x10a30e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee37cu32; cpu.push(v); }
  cpu.push(0x10a30f5); f_01013e03(cpu); if cpu.returned_to != 0x10a30f5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee380u32; cpu.push(v); }
  cpu.push(0x10a3104); f_01013e03(cpu); if cpu.returned_to != 0x10a3104 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d94u32, v); }
  pc = 0x10a310f; continue;
 }
 0x10a310f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3113(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a3113); } r_010a3113(cpu, 0x10a3113); }
pub fn f_010a3313(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a3313); } r_010a3113(cpu, 0x10a3313); }
fn r_010a3113(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a3113 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1e000000u32).wrapping_add((cpu.ebx as u8)); cpu.w8(0x1e000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x41000000u32).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x41000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10a319d; continue; }
  pc = 0x10a319d; continue;
 }
 0x10a319d => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.pf() { pc = 0x10a31a1; continue; }
  pc = 0x10a31a1; continue;
 }
 0x10a31a1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x92000000u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x92000000u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xa4000000u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0xa4000000u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb9000000u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb9000000u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x15000001u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x15000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x37000001u32).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x37000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xd2000001u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xd2000001u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10a320f, "Outsb");
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xba000002u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0xba000002u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xdc000004u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0xdc000004u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5740000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(5); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (cpu.sf == cpu.of) { pc = 0x10a325b; continue; }
  pc = 0x10a3255; continue;
 }
 0x10a3255 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x10a325f; continue; }
  pc = 0x10a3259; continue;
 }
 0x10a3259 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a325b; continue;
 }
 0x10a325b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a325f; continue;
 }
 0x10a325f => {
  { let v: u8 = 0x7u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xa9000008u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb8000009u32).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb8000009u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4000000bu32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x4000000bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | 0xe910000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10a328f, "Outsd");
  cpu.unimplemented(0x10a3290, "Sldt");
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x13u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17200000u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x2300001bu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x2300001bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1ede0000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xe3000024u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xe3000024u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ebp ^ cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a331b; continue;
 }
 0x10a3313 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134da4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a333b; continue; }
  pc = 0x10a331f; continue;
 }
 0x10a331b => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x10a333b; continue; }
  pc = 0x10a331f; continue;
 }
 0x10a331f => {
  { let v = 0x10a3c78u32; cpu.push(v); }
  { let v = 0x10a3313u32; cpu.push(v); }
  cpu.push(0x10a332e); f_01013d5e(cpu); if cpu.returned_to != 0x10a332e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1134da4u32, v); }
  pc = 0x10a333b; continue;
 }
 0x10a333b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a333f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a333f); }
 let mut pc: u32 = 0x10a333f;
 loop { match pc {
 0x10a333f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.cf { pc = 0x10a337a; continue; }
  pc = 0x10a336d; continue;
 }
 0x10a336d => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3385; continue; }
  pc = 0x10a336f; continue;
 }
 0x10a336f => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x10a3390; continue; }
  pc = 0x10a3377; continue;
 }
 0x10a3377 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a337a => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x10a33a6; continue; }
  pc = 0x10a3382; continue;
 }
 0x10a3382 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3385 => {
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3390 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a33a6 => {
  { cpu.sub32(cpu.edx, 0x5bu32); }
  if cpu.cf { pc = 0x10a33f4; continue; }
  pc = 0x10a33ab; continue;
 }
 0x10a33ab => {
  if (cpu.cf || cpu.zf) { pc = 0x10a34cc; continue; }
  pc = 0x10a33b1; continue;
 }
 0x10a33b1 => {
  { cpu.sub32(cpu.edx, 0x70u32); }
  if cpu.cf { pc = 0x10a33df; continue; }
  pc = 0x10a33b6; continue;
 }
 0x10a33b6 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3453; continue; }
  pc = 0x10a33bc; continue;
 }
 0x10a33bc => {
  { cpu.sub32(cpu.edx, 0x73u32); }
  if cpu.cf { pc = 0x10a33d3; continue; }
  pc = 0x10a33c1; continue;
 }
 0x10a33c1 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a346d; continue; }
  pc = 0x10a33c7; continue;
 }
 0x10a33c7 => {
  { cpu.sub32(cpu.edx, 0x77u32); }
  if cpu.zf { pc = 0x10a347a; continue; }
  pc = 0x10a33d0; continue;
 }
 0x10a33d0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a33d3 => {
  { cpu.sub32(cpu.edx, 0x72u32); }
  if cpu.zf { pc = 0x10a3460; continue; }
  pc = 0x10a33dc; continue;
 }
 0x10a33dc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a33df => {
  { cpu.sub32(cpu.edx, 0x6eu32); }
  if cpu.cf { pc = 0x10a33e8; continue; }
  pc = 0x10a33e4; continue;
 }
 0x10a33e4 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3439; continue; }
  pc = 0x10a33e6; continue;
 }
 0x10a33e6 => {
  pc = 0x10a3446; continue;
 }
 0x10a33e8 => {
  { cpu.sub32(cpu.edx, 0x5du32); }
  if cpu.zf { pc = 0x10a34d9; continue; }
  pc = 0x10a33f1; continue;
 }
 0x10a33f1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a33f4 => {
  { cpu.sub32(cpu.edx, 0x7u32); }
  if cpu.cf { pc = 0x10a3422; continue; }
  pc = 0x10a33f9; continue;
 }
 0x10a33f9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3495; continue; }
  pc = 0x10a33ff; continue;
 }
 0x10a33ff => {
  { cpu.sub32(cpu.edx, 0xbu32); }
  if cpu.cf { pc = 0x10a3416; continue; }
  pc = 0x10a3404; continue;
 }
 0x10a3404 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a34b1; continue; }
  pc = 0x10a340a; continue;
 }
 0x10a340a => {
  { cpu.sub32(cpu.edx, 0x40u32); }
  if cpu.zf { pc = 0x10a34bf; continue; }
  pc = 0x10a3413; continue;
 }
 0x10a3413 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3416 => {
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.zf { pc = 0x10a34a3; continue; }
  pc = 0x10a341f; continue;
 }
 0x10a341f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3422 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.cf { pc = 0x10a34f1; continue; }
  pc = 0x10a342b; continue;
 }
 0x10a342b => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3487; continue; }
  pc = 0x10a342d; continue;
 }
 0x10a342d => {
  { cpu.sub32(cpu.edx, 0x6u32); }
  if cpu.zf { pc = 0x10a34e6; continue; }
  pc = 0x10a3436; continue;
 }
 0x10a3436 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3439 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xc0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3446 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x100u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3453 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x180u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3460 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x1c0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a346d => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x200u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a347a => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x240u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3487 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3495 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34a3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34b1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34bf => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x380u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34cc => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x3c0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34d9 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x400u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a34e6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  pc = 0x10a34f1; continue;
 }
 0x10a34f1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a34f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a34f4); }
 let mut pc: u32 = 0x10a34f4;
 loop { match pc {
 0x10a34f4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a3510; continue;
 }
 0x10a34fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a3518; continue; }
  pc = 0x10a3503; continue;
 }
 0x10a3503 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x10a350c; continue; }
  pc = 0x10a3507; continue;
 }
 0x10a3507 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a351a; continue; }
  pc = 0x10a350c; continue;
 }
 0x10a350c => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  pc = 0x10a3510; continue;
 }
 0x10a3510 => {
  { cpu.sub32(cpu.edx, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a34fc; continue; }
  pc = 0x10a3518; continue;
 }
 0x10a3518 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a351a; continue;
 }
 0x10a351a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a351d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a351d); } r_010a351d(cpu, 0x10a351d); }
pub fn f_010a3589(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a3589); } r_010a351d(cpu, 0x10a3589); }
fn r_010a351d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a351d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2d4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x354u32.wrapping_add(cpu.edi))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x618u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a356a); f_0109157f(cpu); if cpu.returned_to != 0x10a356a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x314u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a3580); f_010915f5(cpu); if cpu.returned_to != 0x10a3580 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x10a3583; continue;
 }
 0x10a3583 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3589 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x598u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a35e5; continue; }
  pc = 0x10a35d9; continue;
 }
 0x10a35d9 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1fffu32); cpu.eax = v; }
  pc = 0x10a35fb; continue;
 }
 0x10a35e5 => {
  if (cpu.sf == cpu.of) { pc = 0x10a35f7; continue; }
  pc = 0x10a35e7; continue;
 }
 0x10a35e7 => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x10a35fb; continue;
 }
 0x10a35f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a35fb; continue;
 }
 0x10a35fb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10a3611; continue; }
  pc = 0x10a360f; continue;
 }
 0x10a360f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a3611; continue;
 }
 0x10a3611 => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a361b; continue; }
  pc = 0x10a3616; continue;
 }
 0x10a3616 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x10a361b; continue;
 }
 0x10a361b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10a3113u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10a3113u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10a3113u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 13u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 13u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0x9u32); }
  if cpu.zf { pc = 0x10a366c; continue; }
  pc = 0x10a3657; continue;
 }
 0x10a3657 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10a3673; continue;
 }
 0x10a366c => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a3673; continue;
 }
 0x10a3673 => {
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a3688); f_01091509(cpu); if cpu.returned_to != 0x10a3688 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.poll(); pc = 0x10a3583; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3693(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3693); }
 let mut pc: u32 = 0x10a3693;
 loop { match pc {
 0x10a3693 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x10a36ad); f_010a34f4(cpu); if cpu.returned_to != 0x10a36ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a36b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a36b8); }
 let mut pc: u32 = 0x10a36b8;
 loop { match pc {
 0x10a36b8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1b4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x10a36f5; continue; }
  pc = 0x10a36e5; continue;
 }
 0x10a36e5 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.zf { pc = 0x10a36f5; continue; }
  pc = 0x10a36ed; continue;
 }
 0x10a36ed => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if !cpu.zf { pc = 0x10a370c; continue; }
  pc = 0x10a36f5; continue;
 }
 0x10a36f5 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a3709); f_010a333f(cpu); if cpu.returned_to != 0x10a3709 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a370c; continue;
 }
 0x10a370c => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if !cpu.zf { pc = 0x10a3720; continue; }
  pc = 0x10a3714; continue;
 }
 0x10a3714 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10a3720; continue; }
  pc = 0x10a371b; continue;
 }
 0x10a371b => {
  { let v: u32 = 0x80u32; cpu.esi = v; }
  pc = 0x10a3720; continue;
 }
 0x10a3720 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.cf { pc = 0x10a3756; continue; }
  pc = 0x10a3731; continue;
 }
 0x10a3731 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3779; continue; }
  pc = 0x10a3733; continue;
 }
 0x10a3733 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.cf { pc = 0x10a3aa3; continue; }
  pc = 0x10a373f; continue;
 }
 0x10a373f => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3876; continue; }
  pc = 0x10a3745; continue;
 }
 0x10a3745 => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x10a3893; continue; }
  pc = 0x10a3751; continue;
 }
 0x10a3751 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a3756 => {
  { cpu.sub32(cpu.esi, 0x80u32); }
  if cpu.cf { pc = 0x10a3aa3; continue; }
  pc = 0x10a3762; continue;
 }
 0x10a3762 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3a4e; continue; }
  pc = 0x10a3768; continue;
 }
 0x10a3768 => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.zf { pc = 0x10a38d5; continue; }
  pc = 0x10a3774; continue;
 }
 0x10a3774 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a3779 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7u32); }
  if cpu.cf { pc = 0x10a37a0; continue; }
  pc = 0x10a3782; continue;
 }
 0x10a3782 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a37aa; continue; }
  pc = 0x10a3784; continue;
 }
 0x10a3784 => {
  { cpu.sub32(cpu.esi, 0xau32); }
  if cpu.cf { pc = 0x10a3aa3; continue; }
  pc = 0x10a378d; continue;
 }
 0x10a378d => {
  { cpu.sub32(cpu.esi, 0xbu32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a37aa; continue; }
  pc = 0x10a3792; continue;
 }
 0x10a3792 => {
  { cpu.sub32(cpu.esi, 0x7bu32); }
  if cpu.zf { pc = 0x10a382e; continue; }
  pc = 0x10a379b; continue;
 }
 0x10a379b => {
  pc = 0x10a3aa3; continue;
 }
 0x10a37a0 => {
  { cpu.sub32(cpu.esi, 0x6u32); }
  if cpu.zf { pc = 0x10a37ec; continue; }
  pc = 0x10a37a5; continue;
 }
 0x10a37a5 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a37aa => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a37df; continue;
 }
 0x10a37ae => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a37de; continue; }
  pc = 0x10a37c1; continue;
 }
 0x10a37c1 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a37cd); f_010916e1(cpu); if cpu.returned_to != 0x10a37cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10a37de; continue; }
  pc = 0x10a37d5; continue;
 }
 0x10a37d5 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a37de); f_010a351d(cpu); if cpu.returned_to != 0x10a37de { return; }
  pc = 0x10a37de; continue;
 }
 0x10a37de => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a37df; continue;
 }
 0x10a37df => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a37ae; continue; }
  pc = 0x10a37e7; continue;
 }
 0x10a37e7 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a37ec => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a3821; continue;
 }
 0x10a37f0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a3820; continue; }
  pc = 0x10a3803; continue;
 }
 0x10a3803 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a380f); f_010916e1(cpu); if cpu.returned_to != 0x10a380f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10a3820; continue; }
  pc = 0x10a3817; continue;
 }
 0x10a3817 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a3820); f_010a3589(cpu); if cpu.returned_to != 0x10a3820 { return; }
  pc = 0x10a3820; continue;
 }
 0x10a3820 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a3821; continue;
 }
 0x10a3821 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a37f0; continue; }
  pc = 0x10a3829; continue;
 }
 0x10a3829 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a382e => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a3869; continue;
 }
 0x10a3832 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a3868; continue; }
  pc = 0x10a3845; continue;
 }
 0x10a3845 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a3851); f_010916e1(cpu); if cpu.returned_to != 0x10a3851 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10a3868; continue; }
  pc = 0x10a3859; continue;
 }
 0x10a3859 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a3865); f_0109149c(cpu); if cpu.returned_to != 0x10a3865 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10a3868; continue;
 }
 0x10a3868 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a3869; continue;
 }
 0x10a3869 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3832; continue; }
  pc = 0x10a3871; continue;
 }
 0x10a3871 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a3876 => {
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x10a3888); f_010a34f4(cpu); if cpu.returned_to != 0x10a3888 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a3aa3; continue;
 }
 0x10a3893 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a38c8; continue;
 }
 0x10a3897 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a38c7; continue; }
  pc = 0x10a38aa; continue;
 }
 0x10a38aa => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a38b6); f_010916e1(cpu); if cpu.returned_to != 0x10a38b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10a38c7; continue; }
  pc = 0x10a38be; continue;
 }
 0x10a38be => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a38c7); f_010a3589(cpu); if cpu.returned_to != 0x10a38c7 { return; }
  pc = 0x10a38c7; continue;
 }
 0x10a38c7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a38c8; continue;
 }
 0x10a38c8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3897; continue; }
  pc = 0x10a38d0; continue;
 }
 0x10a38d0 => {
  pc = 0x10a3aa3; continue;
 }
 0x10a38d5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x9u32); }
  if !cpu.zf { pc = 0x10a38f3; continue; }
  pc = 0x10a38dc; continue;
 }
 0x10a38dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  cpu.push(0x10a38ed); f_010a34f4(cpu); if cpu.returned_to != 0x10a38ed { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a38f3; continue;
 }
 0x10a38f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x494u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x10a3aa3; continue; }
  pc = 0x10a3905; continue;
 }
 0x10a3905 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a3921; continue;
 }
 0x10a3909 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a3918); f_010916e1(cpu); if cpu.returned_to != 0x10a3918 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10a3929; continue; }
  pc = 0x10a3920; continue;
 }
 0x10a3920 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a3921; continue;
 }
 0x10a3921 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3909; continue; }
  pc = 0x10a3929; continue;
 }
 0x10a3929 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x10a396a; continue; }
  pc = 0x10a3931; continue;
 }
 0x10a3931 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a3950; continue;
 }
 0x10a393a => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { pc = 0x10a394f; continue; }
  pc = 0x10a394b; continue;
 }
 0x10a394b => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x10a394f; continue;
 }
 0x10a394f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a3950; continue;
 }
 0x10a3950 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a393a; continue; }
  pc = 0x10a3958; continue;
 }
 0x10a3958 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a3967); f_0109149c(cpu); if cpu.returned_to != 0x10a3967 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10a396a; continue;
 }
 0x10a396a => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x518u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x558u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d8u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x618u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x698u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x698u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x658u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a39dd); f_010910de(cpu); if cpu.returned_to != 0x10a39dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a39fe); f_010912d3(cpu); if cpu.returned_to != 0x10a39fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a3a22); f_01091251(cpu); if cpu.returned_to != 0x10a3a22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a3a2e); f_010a3589(cpu); if cpu.returned_to != 0x10a3a2e { return; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a3a37); f_010a351d(cpu); if cpu.returned_to != 0x10a3a37 { return; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a3a44); f_01091355(cpu); if cpu.returned_to != 0x10a3a44 { return; }
  pc = 0x10a3a44; continue;
 }
 0x10a3a44 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a3aa5; continue;
 }
 0x10a3a4e => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x9u32); }
  if cpu.zf { pc = 0x10a3aa3; continue; }
  pc = 0x10a3a55; continue;
 }
 0x10a3a55 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a3a9b; continue;
 }
 0x10a3a59 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a3a9a; continue; }
  pc = 0x10a3a6c; continue;
 }
 0x10a3a6c => {
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x10a3a9a; continue; }
  pc = 0x10a3a78; continue;
 }
 0x10a3a78 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a3a84); f_010916e1(cpu); if cpu.returned_to != 0x10a3a84 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10a3a9a; continue; }
  pc = 0x10a3a8c; continue;
 }
 0x10a3a8c => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a3a98); f_0109149c(cpu); if cpu.returned_to != 0x10a3a98 { return; }
  cpu.poll(); pc = 0x10a3a44; continue;
 }
 0x10a3a9a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10a3a9b; continue;
 }
 0x10a3a9b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x514u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3a59; continue; }
  pc = 0x10a3aa3; continue;
 }
 0x10a3aa3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a3aa5; continue;
 }
 0x10a3aa5 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3aac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3aac); }
 let mut pc: u32 = 0x10a3aac;
 loop { match pc {
 0x10a3aac => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  cpu.push(0x10a3ab8); f_010a3313(cpu); if cpu.returned_to != 0x10a3ab8 { return; }
  { let v = 0x69cu32; cpu.push(v); }
  cpu.push(0x10a3ac2); f_01013c6b(cpu); if cpu.returned_to != 0x10a3ac2 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a3ae4; continue; }
  pc = 0x10a3acd; continue;
 }
 0x10a3acd => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138f34u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3ae4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x10a36b8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a3b08); f_01094163(cpu); if cpu.returned_to != 0x10a3b08 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x10a3693u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a3b1d); f_01094250(cpu); if cpu.returned_to != 0x10a3b1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1b4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10a3b88; continue;
 }
 0x10a3b31 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x40u32; cpu.w32(0x94u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x2d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x40u32; cpu.w32(0x314u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x354u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x11ee2f4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10a3b88; continue;
 }
 0x10a3b88 => {
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3b31; continue; }
  pc = 0x10a3b8d; continue;
 }
 0x10a3b8d => {
  { let v: u32 = 0x0u32; cpu.w32(0x514u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a3bcd; continue;
 }
 0x10a3b99 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a3ba2); f_01090e8d(cpu); if cpu.returned_to != 0x10a3ba2 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x10a3be3; continue; }
  pc = 0x10a3bc4; continue;
 }
 0x10a3bc4 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a3bcd; continue;
 }
 0x10a3bcd => {
  { cpu.sub32(cpu.edi, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a3bd9; continue; }
  pc = 0x10a3bd2; continue;
 }
 0x10a3bd2 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x10a3bdb; continue;
 }
 0x10a3bd9 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10a3bdb; continue;
 }
 0x10a3bdb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a3b99; continue; }
  pc = 0x10a3be3; continue;
 }
 0x10a3be3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10a3bf5; continue;
 }
 0x10a3be7 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10a3bf5; continue;
 }
 0x10a3bf5 => {
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3be7; continue; }
  pc = 0x10a3bfa; continue;
 }
 0x10a3bfa => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10a3c0c; continue;
 }
 0x10a3bfe => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10a3c0c; continue;
 }
 0x10a3c0c => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3bfe; continue; }
  pc = 0x10a3c14; continue;
 }
 0x10a3c14 => {
  { let v: u32 = 0x0u32; cpu.w32(0x698u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3c78); }
 let mut pc: u32 = 0x10a3c78;
 loop { match pc {
 0x10a3c78 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134da4u32), 0x0u32); }
  if cpu.zf { pc = 0x10a3c9e; continue; }
  pc = 0x10a3c84; continue;
 }
 0x10a3c84 => {
  { let v = 0x10a3c78u32; cpu.push(v); }
  { let v = 0x10a3313u32; cpu.push(v); }
  cpu.push(0x10a3c93); f_01013dc7(cpu); if cpu.returned_to != 0x10a3c93 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134da4u32, v); }
  pc = 0x10a3c9e; continue;
 }
 0x10a3c9e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3ca2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3ca2); }
 let mut pc: u32 = 0x10a3ca2;
 loop { match pc {
 0x10a3ca2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10a3cf1; continue; }
  pc = 0x10a3cac; continue;
 }
 0x10a3cac => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x10a3cf1; continue; }
  pc = 0x10a3cb1; continue;
 }
 0x10a3cb1 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3cf1; continue; }
  pc = 0x10a3cc6; continue;
 }
 0x10a3cc6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3cd5; continue; }
  pc = 0x10a3cd0; continue;
 }
 0x10a3cd0 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x10a3cd5; continue;
 }
 0x10a3cd5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x10a3cf5; continue; }
  pc = 0x10a3cdb; continue;
 }
 0x10a3cdb => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3cd5; continue; }
  pc = 0x10a3cee; continue;
 }
 0x10a3cee => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3cf1; continue;
 }
 0x10a3cf1 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a3d40; continue;
 }
 0x10a3cf5 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x10a3d23; continue; }
  pc = 0x10a3d05; continue;
 }
 0x10a3d05 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3d38; continue;
 }
 0x10a3d23 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x10a3d38; continue;
 }
 0x10a3d38 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x10a3d40; continue;
 }
 0x10a3d40 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10a3d44); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3d4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3d4a); }
 let mut pc: u32 = 0x10a3d4a;
 loop { match pc {
 0x10a3d45 => {
  pc = 0x10a3e50; continue;
 }
 0x10a3d4a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x10a3d45; continue; }
  pc = 0x10a3d54; continue;
 }
 0x10a3d54 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10a3d45; continue; }
  pc = 0x10a3d5f; continue;
 }
 0x10a3d5f => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x10a3d8c; continue; }
  pc = 0x10a3d6d; continue;
 }
 0x10a3d6d => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x10a3d75; continue; }
  pc = 0x10a3d72; continue;
 }
 0x10a3d72 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3d75; continue;
 }
 0x10a3d75 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3e08; continue;
 }
 0x10a3d8c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10a3da3; continue; }
  pc = 0x10a3d95; continue;
 }
 0x10a3d95 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3e08; continue; }
  pc = 0x10a3d9a; continue;
 }
 0x10a3d9a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10a3e08; continue; }
  pc = 0x10a3da1; continue;
 }
 0x10a3da1 => {
  pc = 0x10a3db2; continue;
 }
 0x10a3da3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10a3e08; continue; }
  pc = 0x10a3daa; continue;
 }
 0x10a3daa => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3e08; continue; }
  pc = 0x10a3db2; continue;
 }
 0x10a3db2 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x10a3de7; continue; }
  pc = 0x10a3dc2; continue;
 }
 0x10a3dc2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3dcf; continue; }
  pc = 0x10a3dcd; continue;
 }
 0x10a3dcd => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x10a3dcf; continue;
 }
 0x10a3dcf => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x10a3dd3; continue;
 }
 0x10a3dd3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10a3e08; continue; }
  pc = 0x10a3dda; continue;
 }
 0x10a3dda => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a3de7; continue; }
  pc = 0x10a3ddf; continue;
 }
 0x10a3ddf => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10a3dd3; continue; }
  pc = 0x10a3de7; continue;
 }
 0x10a3de7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10a3df1; continue; }
  pc = 0x10a3dee; continue;
 }
 0x10a3dee => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x10a3df1; continue;
 }
 0x10a3df1 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10a3e08; continue; }
  pc = 0x10a3df5; continue;
 }
 0x10a3df5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10a3e08; continue; }
  pc = 0x10a3dfc; continue;
 }
 0x10a3dfc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10a3e08; continue; }
  pc = 0x10a3e03; continue;
 }
 0x10a3e03 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x10a3df1; continue;
 }
 0x10a3e08 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10a3e27; continue; }
  pc = 0x10a3e17; continue;
 }
 0x10a3e17 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x10a3e23; continue; }
  pc = 0x10a3e20; continue;
 }
 0x10a3e20 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3e23; continue;
 }
 0x10a3e23 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x10a3e38; continue;
 }
 0x10a3e27 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3e38; continue;
 }
 0x10a3e38 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x10a3e48; continue; }
  pc = 0x10a3e40; continue;
 }
 0x10a3e40 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x10a3e48; continue; }
  pc = 0x10a3e45; continue;
 }
 0x10a3e45 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3e48; continue;
 }
 0x10a3e48 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x10a3e50; continue; }
  pc = 0x10a3e4d; continue;
 }
 0x10a3e4d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a3e50; continue;
 }
 0x10a3e50 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10a3e54); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3e55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3e55); }
 let mut pc: u32 = 0x10a3e55;
 loop { match pc {
 0x10a3e55 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133c20u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x10a3e6b; continue; }
  pc = 0x10a3e62; continue;
 }
 0x10a3e62 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133c20u32, v); }
  pc = 0x10a3e6b; continue;
 }
 0x10a3e6b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x11ec614u32)); }
  if !cpu.zf { pc = 0x10a3e7b; continue; }
  pc = 0x10a3e73; continue;
 }
 0x10a3e73 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11ec614u32, v); }
  pc = 0x10a3e7b; continue;
 }
 0x10a3e7b => {
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10a3e8e; continue; }
  pc = 0x10a3e85; continue;
 }
 0x10a3e85 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133c1cu32, v); }
  pc = 0x10a3e8e; continue;
 }
 0x10a3e8e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a3e9b; continue; }
  pc = 0x10a3e98; continue;
 }
 0x10a3e98 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a3e9b; continue;
 }
 0x10a3e9b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a3ea2; continue; }
  pc = 0x10a3e9f; continue;
 }
 0x10a3e9f => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a3ea2; continue;
 }
 0x10a3ea2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3ea6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a3ea6); } r_010a3ea6(cpu, 0x10a3ea6); }
pub fn f_010a40c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a40c5); } r_010a3ea6(cpu, 0x10a40c5); }
pub fn f_010a4196(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4196); } r_010a3ea6(cpu, 0x10a4196); }
fn r_010a3ea6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a3ea6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.edx = v; }
  pc = 0x10a3eb1; continue;
 }
 0x10a3eb1 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a4190; continue; }
  pc = 0x10a3eb9; continue;
 }
 0x10a3eb9 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x10a3efd; continue; }
  pc = 0x10a3ec7; continue;
 }
 0x10a3ec7 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x10a3ed4); f_010a3e55(cpu); if cpu.returned_to != 0x10a3ed4 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10a3eed; continue; }
  pc = 0x10a3edd; continue;
 }
 0x10a3edd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x10a3eb1; continue;
 }
 0x10a3eed => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x10a3eb1; continue;
 }
 0x10a3efd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x10a3eb1; continue;
 }
 0x10a40c5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x10a40d9); f_010a3ea6(cpu); if cpu.returned_to != 0x10a40d9 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10a40f4; continue; }
  pc = 0x10a40e4; continue;
 }
 0x10a40e4 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a40f4; continue;
 }
 0x10a40f4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a4111; continue; }
  pc = 0x10a40f8; continue;
 }
 0x10a40f8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a418d; continue;
 }
 0x10a4111 => {
  { cpu.and16(cpu.r16(0x1105eb6u32), 0xfff0u16); }
  if cpu.zf { pc = 0x10a4126; continue; }
  pc = 0x10a411c; continue;
 }
 0x10a411c => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x1105eb4u32, v); }
  pc = 0x10a4126; continue;
 }
 0x10a4126 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a418b; continue; }
  pc = 0x10a412f; continue;
 }
 0x10a412f => {
  { let v: u32 = cpu.r32(0x1105eb4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10a418b; continue; }
  pc = 0x10a4149; continue;
 }
 0x10a4149 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x10a418b; continue; }
  pc = 0x10a416d; continue;
 }
 0x10a416d => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a418d; continue;
 }
 0x10a418b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a418d; continue;
 }
 0x10a418d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10a4190; continue;
 }
 0x10a4190 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4196 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134f94u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a41a9; continue; }
  pc = 0x10a41a5; continue;
 }
 0x10a41a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x10a418d; continue;
 }
 0x10a41a9 => {
  { cpu.sub32(cpu.r32(0x1133e50u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a41a5; continue; }
  pc = 0x10a41b2; continue;
 }
 0x10a41b2 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a41b9); f_010a4392(cpu); if cpu.returned_to != 0x10a41b9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x10a418d; continue; }
  pc = 0x10a41bd; continue;
 }
 0x10a41bd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10a41de; continue; }
  pc = 0x10a41c9; continue;
 }
 0x10a41c9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10a41d1); f_010a40c5(cpu); if cpu.returned_to != 0x10a41d1 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a41f9; continue; }
  pc = 0x10a41d7; continue;
 }
 0x10a41d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a41f9; continue;
 }
 0x10a41de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x80004800u32; cpu.eax = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a41f9; continue;
 }
 0x10a41f9 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x10a41a5; continue; }
  pc = 0x10a41fd; continue;
 }
 0x10a41fd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10a41a5; continue; }
  pc = 0x10a420a; continue;
 }
 0x10a420a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x10a41a5; continue; }
  pc = 0x10a4212; continue;
 }
 0x10a4212 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a421b); f_010a4001(cpu); if cpu.returned_to != 0x10a421b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10a423c); f_01094b7d(cpu); if cpu.returned_to != 0x10a423c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10a418d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3f02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3f02); }
 let mut pc: u32 = 0x10a3f02;
 loop { match pc {
 0x10a3f02 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x10a3f10); f_010a3ea6(cpu); if cpu.returned_to != 0x10a3f10 { return; }
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.edx = v; }
  pc = 0x10a3ff1; continue;
 }
 0x10a3f1b => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3ff9; continue; }
  pc = 0x10a3f23; continue;
 }
 0x10a3f23 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x10a3fee; continue; }
  pc = 0x10a3f2e; continue;
 }
 0x10a3f2e => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x10a3fee; continue; }
  pc = 0x10a3f38; continue;
 }
 0x10a3f38 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a3f3f); f_010a3e55(cpu); if cpu.returned_to != 0x10a3f3f { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10a3ff9; continue; }
  pc = 0x10a3f4c; continue;
 }
 0x10a3f4c => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10a3f90; continue; }
  pc = 0x10a3f80; continue;
 }
 0x10a3f80 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a3f90; continue;
 }
 0x10a3f90 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a3ffb; continue; }
  pc = 0x10a3f94; continue;
 }
 0x10a3f94 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10a3fab); f_010a4001(cpu); if cpu.returned_to != 0x10a3fab { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x10a3fe7; continue; }
  pc = 0x10a3fc0; continue;
 }
 0x10a3fc0 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16) | 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10a3fe5); f_01094b7d(cpu); if cpu.returned_to != 0x10a3fe5 { return; }
  pc = 0x10a3fea; continue;
 }
 0x10a3fe7 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a3fea; continue;
 }
 0x10a3fea => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10a3ffb; continue;
 }
 0x10a3fee => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x10a3ff1; continue;
 }
 0x10a3ff1 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3f1b; continue; }
  pc = 0x10a3ff9; continue;
 }
 0x10a3ff9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a3ffb; continue;
 }
 0x10a3ffb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4001(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4001); }
 let mut pc: u32 = 0x10a4001;
 loop { match pc {
 0x10a4001 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a4017; continue;
 }
 0x10a400e => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x10a401b; continue; }
  pc = 0x10a4012; continue;
 }
 0x10a4012 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10a4017; continue;
 }
 0x10a4017 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a400e; continue; }
  pc = 0x10a401b; continue;
 }
 0x10a401b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a402a; continue; }
  pc = 0x10a4025; continue;
 }
 0x10a4025 => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a4030; continue;
 }
 0x10a402a => {
  { let v: u32 = cpu.edx; cpu.w32(0x1133c1cu32, v); }
  pc = 0x10a4030; continue;
 }
 0x10a4030 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a4037; continue; }
  pc = 0x10a4034; continue;
 }
 0x10a4034 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a4037; continue;
 }
 0x10a4037 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4075(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4075); }
 let mut pc: u32 = 0x10a4075;
 loop { match pc {
 0x10a4075 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a40c0; continue; }
  pc = 0x10a4081; continue;
 }
 0x10a4081 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a40b3; continue; }
  pc = 0x10a4097; continue;
 }
 0x10a4097 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a40b3; continue; }
  pc = 0x10a409c; continue;
 }
 0x10a409c => {
  { cpu.sub8(cpu.r8(0x1133e80u32), 0x0u8); }
  if !cpu.zf { pc = 0x10a40b3; continue; }
  pc = 0x10a40a5; continue;
 }
 0x10a40a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x10a40aa, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133e50u32, v); }
  pc = 0x10a40b3; continue;
 }
 0x10a40b3 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x1133e50u32)); }
  if !cpu.zf { pc = 0x10a40c0; continue; }
  pc = 0x10a40bb; continue;
 }
 0x10a40bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a40c0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4246(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4246); }
 let mut pc: u32 = 0x10a4246;
 loop { match pc {
 0x10a4246 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10a4261; continue; }
  pc = 0x10a4258; continue;
 }
 0x10a4258 => {
  { cpu.sub8(cpu.r8(0x1133e7fu32), 0x0u8); }
  if cpu.zf { pc = 0x10a426d; continue; }
  pc = 0x10a4261; continue;
 }
 0x10a4261 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x10a427a; continue; }
  pc = 0x10a426d; continue;
 }
 0x10a426d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10a4275); f_010a4196(cpu); if cpu.returned_to != 0x10a4275 { return; }
  pc = 0x10a4389; continue;
 }
 0x10a427a => {
  { cpu.sub32(cpu.r32(0x1134f94u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a428a; continue; }
  pc = 0x10a4283; continue;
 }
 0x10a4283 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a4389; continue;
 }
 0x10a428a => {
  { cpu.sub32(cpu.r32(0x1133e50u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a4293; continue;
 }
 0x10a4293 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a429a); f_010a4392(cpu); if cpu.returned_to != 0x10a429a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a4389; continue; }
  pc = 0x10a42a2; continue;
 }
 0x10a42a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a42ca; continue; }
  pc = 0x10a42ae; continue;
 }
 0x10a42ae => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a42ca; continue; }
  pc = 0x10a42b3; continue;
 }
 0x10a42b3 => {
  { cpu.sub8(cpu.r8(0x1133e80u32), 0x0u8); }
  if !cpu.zf { pc = 0x10a42ca; continue; }
  pc = 0x10a42bc; continue;
 }
 0x10a42bc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x10a42c1, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133e50u32, v); }
  pc = 0x10a42ca; continue;
 }
 0x10a42ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133e50u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x10a42de; continue; }
  pc = 0x10a42d9; continue;
 }
 0x10a42d9 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x10a42de; continue;
 }
 0x10a42de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a42e5); f_010ad5a1(cpu); if cpu.returned_to != 0x10a42e5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a42ea; continue;
 }
 0x10a42ea => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a42f1; continue;
 }
 0x10a42f1 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a42f5; continue;
 }
 0x10a42f5 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a4301; continue;
 }
 0x10a4301 => {
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a431e; continue;
 }
 0x10a430c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a4322; continue; }
  pc = 0x10a4313; continue;
 }
 0x10a4313 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a431b; continue; }
  pc = 0x10a4317; continue;
 }
 0x10a4317 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x10a4322; continue; }
  pc = 0x10a431b; continue;
 }
 0x10a431b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x10a431e; continue;
 }
 0x10a431e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a430c; continue; }
  pc = 0x10a4322; continue;
 }
 0x10a4322 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a434c; continue; }
  pc = 0x10a4326; continue;
 }
 0x10a4326 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10a434c; continue; }
  pc = 0x10a4331; continue;
 }
 0x10a4331 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a436b; continue;
 }
 0x10a434c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x10a4283; continue; }
  pc = 0x10a4356; continue;
 }
 0x10a4356 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10a4364); f_010a4001(cpu); if cpu.returned_to != 0x10a4364 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a436b; continue;
 }
 0x10a436b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10a4384); f_01094b7d(cpu); if cpu.returned_to != 0x10a4384 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a4389; continue;
 }
 0x10a4389 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4392(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4392); }
 let mut pc: u32 = 0x10a4392;
 loop { match pc {
 0x10a4392 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10a43a9; continue; }
  pc = 0x10a43a3; continue;
 }
 0x10a43a3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a43a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10a43be; continue; }
  pc = 0x10a43b5; continue;
 }
 0x10a43b5 => {
  { cpu.sub8(cpu.r8(0x1133e7fu32), 0x0u8); }
  if cpu.zf { pc = 0x10a43ca; continue; }
  pc = 0x10a43be; continue;
 }
 0x10a43be => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x10a43cf; continue; }
  pc = 0x10a43ca; continue;
 }
 0x10a43ca => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x10a43d6; continue;
 }
 0x10a43cf => {
  cpu.push(0x10a43d4); f_010a4075(cpu); if cpu.returned_to != 0x10a43d4 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x10a43d6; continue;
 }
 0x10a43d6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x10a43a3; continue; }
  pc = 0x10a43df; continue;
 }
 0x10a43df => {
  { let v: u32 = cpu.r32(0x1134f98u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x10a43ee; continue; }
  pc = 0x10a43e9; continue;
 }
 0x10a43e9 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10a43ee; continue;
 }
 0x10a43ee => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x10a43a3; continue; }
  pc = 0x10a43fa; continue;
 }
 0x10a43fa => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a440f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a440f); }
 let mut pc: u32 = 0x10a440f;
 loop { match pc {
 0x10a440f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4412); }
 let mut pc: u32 = 0x10a4412;
 loop { match pc {
 0x10a4412 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a4455; continue; }
  pc = 0x10a4426; continue;
 }
 0x10a4426 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10a444e; continue; }
  pc = 0x10a442a; continue;
 }
 0x10a442a => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if !cpu.cf { pc = 0x10a443e; continue; }
  pc = 0x10a4434; continue;
 }
 0x10a4434 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a448d; continue;
 }
 0x10a443e => {
  if (cpu.cf || cpu.zf) { pc = 0x10a4447; continue; }
  pc = 0x10a4440; continue;
 }
 0x10a4440 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x10a447f; continue;
 }
 0x10a4447 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x10a447f; continue;
 }
 0x10a444e => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x10a447f; continue;
 }
 0x10a4455 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if cpu.cf { pc = 0x10a4469; continue; }
  pc = 0x10a445f; continue;
 }
 0x10a445f => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a448d; continue;
 }
 0x10a4469 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if !cpu.cf { pc = 0x10a447a; continue; }
  pc = 0x10a4473; continue;
 }
 0x10a4473 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x10a447f; continue;
 }
 0x10a447a => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x10a447f; continue;
 }
 0x10a447f => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x10a448a); f_010a64b5(cpu); if cpu.returned_to != 0x10a448a { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x10a448d; continue;
 }
 0x10a448d => {
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a44a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a44a4); }
 let mut pc: u32 = 0x10a44a4;
 loop { match pc {
 0x10a44a4 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_010a44a6(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a44a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a44a6); }
 let mut pc: u32 = 0x10a44a6;
 loop { match pc {
 0x10a44a6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x1134da8u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (cpu.cf || cpu.zf) { pc = 0x10a44d5; continue; }
  pc = 0x10a44bd; continue;
 }
 0x10a44bd => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10a44cf; continue; }
  pc = 0x10a44c1; continue;
 }
 0x10a44c1 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a44cf); f_0109987c(cpu); if cpu.returned_to != 0x10a44cf { return; }
  pc = 0x10a44cf; continue;
 }
 0x10a44cf => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a44d5 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x1134db0u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a44ea; continue; }
  pc = 0x10a44e6; continue;
 }
 0x10a44e6 => {
  { cpu.fpush(0.0); }
  pc = 0x10a44fe; continue;
 }
 0x10a44ea => {
  { cpu.fpush(std::f64::consts::LOG2_E); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let v = 2f64.powf(cpu.st(0)) - 1.0; cpu.set_st(0, v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fscale(); }
  pc = 0x10a44fe; continue;
 }
 0x10a44fe => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4506(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4506); }
 let mut pc: u32 = 0x10a4506;
 loop { match pc {
 0x10a4506 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10a450f); f_010a44a4(cpu); if cpu.returned_to != 0x10a450f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4514(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4514); }
 let mut pc: u32 = 0x10a4514;
 loop { match pc {
 0x10a4514 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x10a451c; continue;
 }
 0x10a451c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x10a45ae; continue; }
  pc = 0x10a4528; continue;
 }
 0x10a4528 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x10a4537; continue; }
  pc = 0x10a4534; continue;
 }
 0x10a4534 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4537 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1139394u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10a4546; continue; }
  pc = 0x10a4543; continue;
 }
 0x10a4543 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4546 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10a45b8; continue; }
  pc = 0x10a4551; continue;
 }
 0x10a4551 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x10a45b8; continue; }
  pc = 0x10a4558; continue;
 }
 0x10a4558 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10a4597; continue; }
  pc = 0x10a4580; continue;
 }
 0x10a4580 => {
  { let v = cpu.st(0) * cpu.rf32(0x11393a4u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x11393a4u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4597 => {
  { let v = cpu.st(0) * cpu.rf32(0x11393a8u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x11393a8u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a45ae => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a45bb; continue; }
  pc = 0x10a45b8; continue;
 }
 0x10a45b8 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a45bb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10a45b8; continue; }
  pc = 0x10a45c6; continue;
 }
 0x10a45c6 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10a45fc; continue; }
  pc = 0x10a45eb; continue;
 }
 0x10a45eb => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x10a4624; continue; }
  pc = 0x10a45f2; continue;
 }
 0x10a45f2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x10a4624; continue; }
  pc = 0x10a45fa; continue;
 }
 0x10a45fa => {
  pc = 0x10a4604; continue;
 }
 0x10a45fc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x10a4624; continue; }
  pc = 0x10a4604; continue;
 }
 0x10a4604 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x11393acu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x10a451c; continue;
 }
 0x10a4624 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a462b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a462b); } r_010a462b(cpu, 0x10a462b); }
pub fn f_010a4635(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4635); } r_010a462b(cpu, 0x10a4635); }
pub fn f_010a463b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a463b); } r_010a462b(cpu, 0x10a463b); }
pub fn f_010a4640(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4640); } r_010a462b(cpu, 0x10a4640); }
pub fn f_010a4646(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4646); } r_010a462b(cpu, 0x10a4646); }
pub fn f_010a464b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a464b); } r_010a462b(cpu, 0x10a464b); }
pub fn f_010a4651(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4651); } r_010a462b(cpu, 0x10a4651); }
pub fn f_010a4657(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4657); } r_010a462b(cpu, 0x10a4657); }
pub fn f_010a465d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a465d); } r_010a462b(cpu, 0x10a465d); }
pub fn f_010a4663(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4663); } r_010a462b(cpu, 0x10a4663); }
pub fn f_010a467f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a467f); } r_010a462b(cpu, 0x10a467f); }
pub fn f_010a4684(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4684); } r_010a462b(cpu, 0x10a4684); }
pub fn f_010a469a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a469a); } r_010a462b(cpu, 0x10a469a); }
pub fn f_010a469f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a469f); } r_010a462b(cpu, 0x10a469f); }
pub fn f_010a46bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a46bb); } r_010a462b(cpu, 0x10a46bb); }
pub fn f_010a46cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a46cb); } r_010a462b(cpu, 0x10a46cb); }
pub fn f_010a46df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a46df); } r_010a462b(cpu, 0x10a46df); }
pub fn f_010a46ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a46ef); } r_010a462b(cpu, 0x10a46ef); }
pub fn f_010a470f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a470f); } r_010a462b(cpu, 0x10a470f); }
pub fn f_010a4714(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4714); } r_010a462b(cpu, 0x10a4714); }
pub fn f_010a472e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a472e); } r_010a462b(cpu, 0x10a472e); }
pub fn f_010a4733(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4733); } r_010a462b(cpu, 0x10a4733); }
pub fn f_010a4753(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4753); } r_010a462b(cpu, 0x10a4753); }
pub fn f_010a4767(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4767); } r_010a462b(cpu, 0x10a4767); }
pub fn f_010a477f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a477f); } r_010a462b(cpu, 0x10a477f); }
pub fn f_010a4793(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4793); } r_010a462b(cpu, 0x10a4793); }
pub fn f_010a47b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a47b3); } r_010a462b(cpu, 0x10a47b3); }
pub fn f_010a47b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a47b8); } r_010a462b(cpu, 0x10a47b8); }
pub fn f_010a47d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a47d2); } r_010a462b(cpu, 0x10a47d2); }
pub fn f_010a47d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a47d7); } r_010a462b(cpu, 0x10a47d7); }
pub fn f_010a47f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a47f7); } r_010a462b(cpu, 0x10a47f7); }
pub fn f_010a480b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a480b); } r_010a462b(cpu, 0x10a480b); }
pub fn f_010a4823(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4823); } r_010a462b(cpu, 0x10a4823); }
pub fn f_010a4837(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4837); } r_010a462b(cpu, 0x10a4837); }
pub fn f_010a4857(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4857); } r_010a462b(cpu, 0x10a4857); }
pub fn f_010a485c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a485c); } r_010a462b(cpu, 0x10a485c); }
pub fn f_010a4876(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4876); } r_010a462b(cpu, 0x10a4876); }
pub fn f_010a487b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a487b); } r_010a462b(cpu, 0x10a487b); }
pub fn f_010a489b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a489b); } r_010a462b(cpu, 0x10a489b); }
pub fn f_010a48af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a48af); } r_010a462b(cpu, 0x10a48af); }
pub fn f_010a48c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a48c7); } r_010a462b(cpu, 0x10a48c7); }
pub fn f_010a48db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a48db); } r_010a462b(cpu, 0x10a48db); }
pub fn f_010a48fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a48fb); } r_010a462b(cpu, 0x10a48fb); }
pub fn f_010a4900(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4900); } r_010a462b(cpu, 0x10a4900); }
pub fn f_010a491a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a491a); } r_010a462b(cpu, 0x10a491a); }
pub fn f_010a491f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a491f); } r_010a462b(cpu, 0x10a491f); }
pub fn f_010a493f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a493f); } r_010a462b(cpu, 0x10a493f); }
pub fn f_010a4953(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4953); } r_010a462b(cpu, 0x10a4953); }
pub fn f_010a496b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a496b); } r_010a462b(cpu, 0x10a496b); }
pub fn f_010a497f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a497f); } r_010a462b(cpu, 0x10a497f); }
pub fn f_010a499f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a499f); } r_010a462b(cpu, 0x10a499f); }
pub fn f_010a49a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a49a4); } r_010a462b(cpu, 0x10a49a4); }
pub fn f_010a49be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a49be); } r_010a462b(cpu, 0x10a49be); }
pub fn f_010a49c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a49c3); } r_010a462b(cpu, 0x10a49c3); }
pub fn f_010a49e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a49e3); } r_010a462b(cpu, 0x10a49e3); }
pub fn f_010a49f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a49f7); } r_010a462b(cpu, 0x10a49f7); }
pub fn f_010a4a0f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a0f); } r_010a462b(cpu, 0x10a4a0f); }
pub fn f_010a4a23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a23); } r_010a462b(cpu, 0x10a4a23); }
pub fn f_010a4a43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a43); } r_010a462b(cpu, 0x10a4a43); }
pub fn f_010a4a48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a48); } r_010a462b(cpu, 0x10a4a48); }
pub fn f_010a4a62(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a62); } r_010a462b(cpu, 0x10a4a62); }
pub fn f_010a4a67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a67); } r_010a462b(cpu, 0x10a4a67); }
pub fn f_010a4a87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a87); } r_010a462b(cpu, 0x10a4a87); }
pub fn f_010a4a9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4a9b); } r_010a462b(cpu, 0x10a4a9b); }
pub fn f_010a4ab3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a4ab3); } r_010a462b(cpu, 0x10a4ab3); }
fn r_010a462b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a462b => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x11393b0u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10a4635 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a463b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4640; continue;
 }
 0x10a4640 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4646 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a464b; continue;
 }
 0x10a464b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4651 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4657 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a465d => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4663 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4675); f_010a4514(cpu); if cpu.returned_to != 0x10a4675 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a467f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4684; continue;
 }
 0x10a4684 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4690); f_010a4514(cpu); if cpu.returned_to != 0x10a4690 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a469a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a469f; continue;
 }
 0x10a469f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a46b3); f_010a4514(cpu); if cpu.returned_to != 0x10a46b3 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a46bb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a46c7); f_010a4514(cpu); if cpu.returned_to != 0x10a46c7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a46cb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a46d7); f_010a4514(cpu); if cpu.returned_to != 0x10a46d7 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a46df => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a46eb); f_010a4514(cpu); if cpu.returned_to != 0x10a46eb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a46ef => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4703); f_010a4514(cpu); if cpu.returned_to != 0x10a4703 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a470f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4714; continue;
 }
 0x10a4714 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4722); f_010a4514(cpu); if cpu.returned_to != 0x10a4722 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a472e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4733; continue;
 }
 0x10a4733 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4749); f_010a4514(cpu); if cpu.returned_to != 0x10a4749 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4753 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4761); f_010a4514(cpu); if cpu.returned_to != 0x10a4761 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4767 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4775); f_010a4514(cpu); if cpu.returned_to != 0x10a4775 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a477f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a478d); f_010a4514(cpu); if cpu.returned_to != 0x10a478d { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4793 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a47a7); f_010a4514(cpu); if cpu.returned_to != 0x10a47a7 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a47b3 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a47b8; continue;
 }
 0x10a47b8 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a47c6); f_010a4514(cpu); if cpu.returned_to != 0x10a47c6 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a47d2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a47d7; continue;
 }
 0x10a47d7 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a47ed); f_010a4514(cpu); if cpu.returned_to != 0x10a47ed { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a47f7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4805); f_010a4514(cpu); if cpu.returned_to != 0x10a4805 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a480b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4819); f_010a4514(cpu); if cpu.returned_to != 0x10a4819 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4823 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4831); f_010a4514(cpu); if cpu.returned_to != 0x10a4831 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4837 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a484b); f_010a4514(cpu); if cpu.returned_to != 0x10a484b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4857 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a485c; continue;
 }
 0x10a485c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a486a); f_010a4514(cpu); if cpu.returned_to != 0x10a486a { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4876 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a487b; continue;
 }
 0x10a487b => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4891); f_010a4514(cpu); if cpu.returned_to != 0x10a4891 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a489b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a48a9); f_010a4514(cpu); if cpu.returned_to != 0x10a48a9 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a48af => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a48bd); f_010a4514(cpu); if cpu.returned_to != 0x10a48bd { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a48c7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a48d5); f_010a4514(cpu); if cpu.returned_to != 0x10a48d5 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a48db => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a48ef); f_010a4514(cpu); if cpu.returned_to != 0x10a48ef { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a48fb => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4900; continue;
 }
 0x10a4900 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a490e); f_010a4514(cpu); if cpu.returned_to != 0x10a490e { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a491a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a491f; continue;
 }
 0x10a491f => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4935); f_010a4514(cpu); if cpu.returned_to != 0x10a4935 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a493f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a494d); f_010a4514(cpu); if cpu.returned_to != 0x10a494d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4953 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4961); f_010a4514(cpu); if cpu.returned_to != 0x10a4961 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a496b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4979); f_010a4514(cpu); if cpu.returned_to != 0x10a4979 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a497f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4993); f_010a4514(cpu); if cpu.returned_to != 0x10a4993 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a499f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a49a4; continue;
 }
 0x10a49a4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a49b2); f_010a4514(cpu); if cpu.returned_to != 0x10a49b2 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a49be => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a49c3; continue;
 }
 0x10a49c3 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a49d9); f_010a4514(cpu); if cpu.returned_to != 0x10a49d9 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a49e3 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a49f1); f_010a4514(cpu); if cpu.returned_to != 0x10a49f1 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a49f7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a05); f_010a4514(cpu); if cpu.returned_to != 0x10a4a05 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a0f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a1d); f_010a4514(cpu); if cpu.returned_to != 0x10a4a1d { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a23 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a37); f_010a4514(cpu); if cpu.returned_to != 0x10a4a37 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a43 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4a48; continue;
 }
 0x10a4a48 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a56); f_010a4514(cpu); if cpu.returned_to != 0x10a4a56 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a62 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10a4a67; continue;
 }
 0x10a4a67 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a7d); f_010a4514(cpu); if cpu.returned_to != 0x10a4a7d { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a87 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4a95); f_010a4514(cpu); if cpu.returned_to != 0x10a4a95 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4a9b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4aa9); f_010a4514(cpu); if cpu.returned_to != 0x10a4aa9 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4ab3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4ac1); f_010a4514(cpu); if cpu.returned_to != 0x10a4ac1 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4ac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4ac7); }
 let mut pc: u32 = 0x10a4ac7;
 loop { match pc {
 0x10a4ac7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4ad6); f_010a4514(cpu); if cpu.returned_to != 0x10a4ad6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4ada(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4ada); }
 let mut pc: u32 = 0x10a4ada;
 loop { match pc {
 0x10a4ada => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10a4ae9); f_010a4514(cpu); if cpu.returned_to != 0x10a4ae9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4b4c); }
 let mut pc: u32 = 0x10a4b4c;
 loop { match pc {
 0x10a4b4c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x10a4b90; continue; }
  pc = 0x10a4b5d; continue;
 }
 0x10a4b5d => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x10a4b73; continue; }
  pc = 0x10a4b66; continue;
 }
 0x10a4b66 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10a4b6f); f_010a4ac7(cpu); if cpu.returned_to != 0x10a4b6f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x10a4b73 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10a4b84); f_010a4ac7(cpu); if cpu.returned_to != 0x10a4b84 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x10a4b90 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4b98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4b98); }
 let mut pc: u32 = 0x10a4b98;
 loop { match pc {
 0x10a4b98 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x10a4bdc; continue; }
  pc = 0x10a4ba9; continue;
 }
 0x10a4ba9 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x10a4bbf; continue; }
  pc = 0x10a4bb2; continue;
 }
 0x10a4bb2 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10a4bbb); f_010a4ada(cpu); if cpu.returned_to != 0x10a4bbb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x10a4bbf => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10a4bd0); f_010a4ada(cpu); if cpu.returned_to != 0x10a4bd0 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x10a4bdc => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4c30); }
 let mut pc: u32 = 0x10a4c30;
 loop { match pc {
 0x10a4c30 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x10a4c42); f_010999ce(cpu); if cpu.returned_to != 0x10a4c42 { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x10a4c75; continue; }
  pc = 0x10a4c46; continue;
 }
 0x10a4c46 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.rcl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10a4c75; continue; }
  pc = 0x10a4c68; continue;
 }
 0x10a4c68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10a4c73); f_0109995d(cpu); if cpu.returned_to != 0x10a4c73 { return; }
  pc = 0x10a4c9c; continue;
 }
 0x10a4c75 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a4c68; continue; }
  pc = 0x10a4c8c; continue;
 }
 0x10a4c8c => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10a4c9a; continue; }
  pc = 0x10a4c90; continue;
 }
 0x10a4c90 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x10a4c9a); f_01099923(cpu); if cpu.returned_to != 0x10a4c9a { return; }
  pc = 0x10a4c9a; continue;
 }
 0x10a4c9a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a4c9c; continue;
 }
 0x10a4c9c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4ca4); }
 let mut pc: u32 = 0x10a4ca4;
 loop { match pc {
 0x10a4ca4 => {
  { let v: u32 = 0x11ee5b4u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4caa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4caa); }
 let mut pc: u32 = 0x10a4caa;
 loop { match pc {
 0x10a4caa => {
  { let v: u32 = 0x11ee5b8u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4cb0); }
 let mut pc: u32 = 0x10a4cb0;
 loop { match pc {
 0x10a4cb0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef370u32, v); }
  { let v: u32 = 0x11ef568u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ef570u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ef524u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xf00u32; cpu.push(v); }
  cpu.push(0x10a4ce1); f_010956f2(cpu); if cpu.returned_to != 0x10a4ce1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4cf4; continue; }
  pc = 0x10a4ce8; continue;
 }
 0x10a4ce8 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4cf4 => {
  { let v: u32 = 0x11ef52cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ef56cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ef520u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xf00u32; cpu.push(v); }
  cpu.push(0x10a4d10); f_010956f2(cpu); if cpu.returned_to != 0x10a4d10 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4d23; continue; }
  pc = 0x10a4d17; continue;
 }
 0x10a4d17 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4d23 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11ef528u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef36cu32, v); }
  { let v: u32 = cpu.r32(0x11ef524u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef378u32, v); }
  { let v: u32 = cpu.r32(0x11ef520u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef374u32, v); }
  cpu.push(0x10a4d4e); f_010a5297(cpu); if cpu.returned_to != 0x10a4d4e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4d5e; continue; }
  pc = 0x10a4d52; continue;
 }
 0x10a4d52 => {
  { let v: u32 = 0x2u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4d5e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10a4d65); f_0109d6d2(cpu); if cpu.returned_to != 0x10a4d65 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a4d73); f_010a60df(cpu); if cpu.returned_to != 0x10a4d73 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a4d8a); f_010a63f4(cpu); if cpu.returned_to != 0x10a4d8a { return; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a4d92); f_0108e46c(cpu); if cpu.returned_to != 0x10a4d92 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a4da8); f_010a63f4(cpu); if cpu.returned_to != 0x10a4da8 { return; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.push(0x10a4db3); f_0108e46c(cpu); if cpu.returned_to != 0x10a4db3 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a4dc9); f_010a63f4(cpu); if cpu.returned_to != 0x10a4dc9 { return; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4dd2); f_010a5ab0(cpu); if cpu.returned_to != 0x10a4dd2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4de5; continue; }
  pc = 0x10a4dd9; continue;
 }
 0x10a4dd9 => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4de5 => {
  cpu.push(0x10a4dea); f_010a5d0b(cpu); if cpu.returned_to != 0x10a4dea { return; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4df3); f_010a5be8(cpu); if cpu.returned_to != 0x10a4df3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4e06; continue; }
  pc = 0x10a4dfa; continue;
 }
 0x10a4dfa => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4e06 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11ef35cu32; cpu.edx = v; }
  { let v: u32 = 0x11ef34cu32; cpu.eax = v; }
  cpu.push(0x10a4e1a); f_0108ef05(cpu); if cpu.returned_to != 0x10a4e1a { return; }
  { let v: u32 = 0x11ef34cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4e25); f_010a50f5(cpu); if cpu.returned_to != 0x10a4e25 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4e3d; continue;
 }
 0x10a4e3d => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a4e4d; continue; }
  pc = 0x10a4e43; continue;
 }
 0x10a4e43 => {
  pc = 0x10a4e7f; continue;
 }
 0x10a4e45 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a4e3d; continue;
 }
 0x10a4e4d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11eecf4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11eecf6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11eed38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0xc0u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a4e45; continue;
 }
 0x10a4e7f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4e86; continue;
 }
 0x10a4e86 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a4e96; continue; }
  pc = 0x10a4e8c; continue;
 }
 0x10a4e8c => {
  pc = 0x10a4ee6; continue;
 }
 0x10a4e8e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a4e86; continue;
 }
 0x10a4e96 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ee6dcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ee6deu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11ee720u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xau32); }
  if !cpu.cf { pc = 0x10a4ed2; continue; }
  pc = 0x10a4ec5; continue;
 }
 0x10a4ec5 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u8 = 0xa1u8; cpu.w8(0x11ee6e0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a4edd; continue;
 }
 0x10a4ed2 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u8 = 0xa3u8; cpu.w8(0x11ee6e0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a4edd; continue;
 }
 0x10a4edd => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0xc0u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a4e8e; continue;
 }
 0x10a4ee6 => {
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x11ee6ccu32; cpu.eax = v; }
  cpu.push(0x10a4efa); f_0108f030(cpu); if cpu.returned_to != 0x10a4efa { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f01; continue;
 }
 0x10a4f01 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4f0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4f0a); }
 let mut pc: u32 = 0x10a4f0a;
 loop { match pc {
 0x10a4f0a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  cpu.push(0x10a4f1a); f_010a5e9d(cpu); if cpu.returned_to != 0x10a4f1a { return; }
  { let v: u32 = cpu.r32(0x11ef520u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef374u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11ef574u32, v); }
  { let v: u32 = 0x11ef35cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4f38); f_010a5be8(cpu); if cpu.returned_to != 0x10a4f38 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4f48; continue; }
  pc = 0x10a4f3f; continue;
 }
 0x10a4f3f => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f6b; continue;
 }
 0x10a4f48 => {
  { let v = cpu.r32(0x11ef568u32); cpu.push(v); }
  cpu.push(0x10a4f53); f_01095774(cpu); if cpu.returned_to != 0x10a4f53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x11ef52cu32); cpu.push(v); }
  cpu.push(0x10a4f61); f_01095774(cpu); if cpu.returned_to != 0x10a4f61 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4f6b; continue;
 }
 0x10a4f6b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4f74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4f74); }
 let mut pc: u32 = 0x10a4f74;
 loop { match pc {
 0x10a4f74 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ef370u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4f90); }
 let mut pc: u32 = 0x10a4f90;
 loop { match pc {
 0x10a4f90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ef530u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4fac); }
 let mut pc: u32 = 0x10a4fac;
 loop { match pc {
 0x10a4fac => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  pc = 0x10a4fb7; continue;
 }
 0x10a4fb7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a4fc5; continue;
 }
 0x10a4fc5 => {
  { let v: u32 = cpu.r32(0x11ef370u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a4fdd; continue; }
  pc = 0x10a4fd0; continue;
 }
 0x10a4fd0 => {
  pc = 0x10a5072; continue;
 }
 0x10a4fd5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a4fc5; continue;
 }
 0x10a4fdd => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a5004); f_010a621e(cpu); if cpu.returned_to != 0x10a5004 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a506d; continue; }
  pc = 0x10a5008; continue;
 }
 0x10a5008 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5029); f_0108ef05(cpu); if cpu.returned_to != 0x10a5029 { return; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a5050); f_0108ef05(cpu); if cpu.returned_to != 0x10a5050 { return; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a506d); f_0108ef05(cpu); if cpu.returned_to != 0x10a506d { return; }
  pc = 0x10a506d; continue;
 }
 0x10a506d => {
  cpu.poll(); pc = 0x10a4fd5; continue;
 }
 0x10a5072 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4fb7; continue; }
  pc = 0x10a507c; continue;
 }
 0x10a507c => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5083; continue;
 }
 0x10a5083 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef370u32)); }
  if cpu.cf { pc = 0x10a5098; continue; }
  pc = 0x10a508e; continue;
 }
 0x10a508e => {
  pc = 0x10a50c2; continue;
 }
 0x10a5090 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5083; continue;
 }
 0x10a5098 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11ef35cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a50b4); f_010a621e(cpu); if cpu.returned_to != 0x10a50b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a50c0; continue; }
  pc = 0x10a50b8; continue;
 }
 0x10a50b8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef530u32, v); }
  pc = 0x10a50c0; continue;
 }
 0x10a50c0 => {
  cpu.poll(); pc = 0x10a5090; continue;
 }
 0x10a50c2 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a50c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a50c8); }
 let mut pc: u32 = 0x10a50c8;
 loop { match pc {
 0x10a50c8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11ef35cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a50e5); f_0108ef05(cpu); if cpu.returned_to != 0x10a50e5 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a50f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a50f5); }
 let mut pc: u32 = 0x10a50f5;
 loop { match pc {
 0x10a50f5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ef370u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a512d; continue; }
  pc = 0x10a5109; continue;
 }
 0x10a5109 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  cpu.push(0x10a511b); f_0108ef05(cpu); if cpu.returned_to != 0x10a511b { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ef370u32)); cpu.w32(0x11ef370u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a51b5; continue;
 }
 0x10a512d => {
  { let v: u32 = cpu.r32(0x11ef370u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef36cu32)); }
  if !cpu.zf { pc = 0x10a5146; continue; }
  pc = 0x10a513a; continue;
 }
 0x10a513a => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a51b5; continue;
 }
 0x10a5146 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a514d; continue;
 }
 0x10a514d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef370u32)); }
  if cpu.cf { pc = 0x10a5162; continue; }
  pc = 0x10a5158; continue;
 }
 0x10a5158 => {
  pc = 0x10a518b; continue;
 }
 0x10a515a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a514d; continue;
 }
 0x10a5162 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a517c); f_010a621e(cpu); if cpu.returned_to != 0x10a517c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5189; continue; }
  pc = 0x10a5180; continue;
 }
 0x10a5180 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a51b5; continue;
 }
 0x10a5189 => {
  cpu.poll(); pc = 0x10a515a; continue;
 }
 0x10a518b => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef370u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a51a8); f_0108ef05(cpu); if cpu.returned_to != 0x10a51a8 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ef370u32)); cpu.w32(0x11ef370u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a51b5; continue;
 }
 0x10a51b5 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a51be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a51be); }
 let mut pc: u32 = 0x10a51be;
 loop { match pc {
 0x10a51be => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef370u32)); }
  if cpu.cf { pc = 0x10a51dd; continue; }
  pc = 0x10a51d4; continue;
 }
 0x10a51d4 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a521d; continue;
 }
 0x10a51dd => {
  { let v: u32 = cpu.r32(0x11ef370u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a5210); f_0108ef05(cpu); if cpu.returned_to != 0x10a5210 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ef370u32)); cpu.w32(0x11ef370u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a521d; continue;
 }
 0x10a521d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5226); }
 let mut pc: u32 = 0x10a5226;
 loop { match pc {
 0x10a5226 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ef370u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a5243; continue; }
  pc = 0x10a523a; continue;
 }
 0x10a523a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a528e; continue;
 }
 0x10a5243 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a524a; continue;
 }
 0x10a524a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef370u32)); }
  if cpu.cf { pc = 0x10a525f; continue; }
  pc = 0x10a5255; continue;
 }
 0x10a5255 => {
  pc = 0x10a5287; continue;
 }
 0x10a5257 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a524a; continue;
 }
 0x10a525f => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a5279); f_010a621e(cpu); if cpu.returned_to != 0x10a5279 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5285; continue; }
  pc = 0x10a527d; continue;
 }
 0x10a527d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a528e; continue;
 }
 0x10a5285 => {
  cpu.poll(); pc = 0x10a5257; continue;
 }
 0x10a5287 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a528e; continue;
 }
 0x10a528e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5297(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5297); }
 let mut pc: u32 = 0x10a5297;
 loop { match pc {
 0x10a5297 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a52b3); f_0108f030(cpu); if cpu.returned_to != 0x10a52b3 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a52c2); f_0108f030(cpu); if cpu.returned_to != 0x10a52c2 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a52d1); f_0108f030(cpu); if cpu.returned_to != 0x10a52d1 { return; }
  { let v: u8 = 0x7fu8; cpu.w8(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffc1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  cpu.push(0x10a52eb); f_0108ef05(cpu); if cpu.returned_to != 0x10a52eb { return; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a532a); f_010a619b(cpu); if cpu.returned_to != 0x10a532a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5348); f_010a6120(cpu); if cpu.returned_to != 0x10a5348 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x10a535d; continue; }
  pc = 0x10a5354; continue;
 }
 0x10a5354 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5364; continue;
 }
 0x10a535d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5364; continue;
 }
 0x10a5364 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a536d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a536d); }
 let mut pc: u32 = 0x10a536d;
 loop { match pc {
 0x10a536d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a538d; continue;
 }
 0x10a538d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a539d; continue; }
  pc = 0x10a5393; continue;
 }
 0x10a5393 => {
  pc = 0x10a53cf; continue;
 }
 0x10a5395 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a538d; continue;
 }
 0x10a539d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x10a53cd; continue; }
  pc = 0x10a53b3; continue;
 }
 0x10a53b3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x10a53cd; continue; }
  pc = 0x10a53be; continue;
 }
 0x10a53be => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a53cd; continue;
 }
 0x10a53cd => {
  cpu.poll(); pc = 0x10a5395; continue;
 }
 0x10a53cf => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a53de; continue; }
  pc = 0x10a53d5; continue;
 }
 0x10a53d5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a542f; continue;
 }
 0x10a53de => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a53fc); f_0108ef05(cpu); if cpu.returned_to != 0x10a53fc { return; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a5416; continue; }
  pc = 0x10a5402; continue;
 }
 0x10a5402 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a5416); f_0108ef05(cpu); if cpu.returned_to != 0x10a5416 { return; }
  pc = 0x10a5416; continue;
 }
 0x10a5416 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ee6dcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10a5428); f_010a57b6(cpu); if cpu.returned_to != 0x10a5428 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a542f; continue;
 }
 0x10a542f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5438(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5438); }
 let mut pc: u32 = 0x10a5438;
 loop { match pc {
 0x10a5438 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5458; continue;
 }
 0x10a5458 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a5468; continue; }
  pc = 0x10a545e; continue;
 }
 0x10a545e => {
  pc = 0x10a549a; continue;
 }
 0x10a5460 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5458; continue;
 }
 0x10a5468 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x10a5498; continue; }
  pc = 0x10a547e; continue;
 }
 0x10a547e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x10a5498; continue; }
  pc = 0x10a5489; continue;
 }
 0x10a5489 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5498; continue;
 }
 0x10a5498 => {
  cpu.poll(); pc = 0x10a5460; continue;
 }
 0x10a549a => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a54a9; continue; }
  pc = 0x10a54a0; continue;
 }
 0x10a54a0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a54eb; continue;
 }
 0x10a54a9 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a54c7); f_0108ef05(cpu); if cpu.returned_to != 0x10a54c7 { return; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ee6dcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a54eb; continue;
 }
 0x10a54eb => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a54f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a54f4); }
 let mut pc: u32 = 0x10a54f4;
 loop { match pc {
 0x10a54f4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11ef370u32)); }
  if cpu.cf { pc = 0x10a5510; continue; }
  pc = 0x10a550a; continue;
 }
 0x10a550a => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x11u32); }
  if !cpu.zf { pc = 0x10a5512; continue; }
  pc = 0x10a5510; continue;
 }
 0x10a5510 => {
  pc = 0x10a551e; continue;
 }
 0x10a5512 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a566b; continue;
 }
 0x10a551e => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5525; continue;
 }
 0x10a5525 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a5535; continue; }
  pc = 0x10a552b; continue;
 }
 0x10a552b => {
  pc = 0x10a5542; continue;
 }
 0x10a552d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5525; continue;
 }
 0x10a5535 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11eecf5u32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a552d; continue; }
  pc = 0x10a5542; continue;
 }
 0x10a5542 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10a5581; continue; }
  pc = 0x10a5548; continue;
 }
 0x10a5548 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a554f; continue;
 }
 0x10a554f => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a555f; continue; }
  pc = 0x10a5555; continue;
 }
 0x10a5555 => {
  pc = 0x10a556f; continue;
 }
 0x10a5557 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a554f; continue;
 }
 0x10a555f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11eed38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10a5557; continue; }
  pc = 0x10a556f; continue;
 }
 0x10a556f => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10a5581; continue; }
  pc = 0x10a5575; continue;
 }
 0x10a5575 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a566b; continue;
 }
 0x10a5581 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.edx = v; }
  { let v: u32 = 0x11eecf4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ef524u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ef378u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef574u32, v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x10a55d0); f_0108f030(cpu); if cpu.returned_to != 0x10a55d0 { return; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x11u32); }
  if !cpu.zf { pc = 0x10a55df; continue; }
  pc = 0x10a55d6; continue;
 }
 0x10a55d6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xa2u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a55e6; continue;
 }
 0x10a55df => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xa0u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a55e6; continue;
 }
 0x10a55e6 => {
  { let v: u8 = cpu.r8(0x11ef30fu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x35u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x11u32); }
  if cpu.zf { pc = 0x10a562f; continue; }
  pc = 0x10a5612; continue;
 }
 0x10a5612 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x11ee5bcu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  cpu.push(0x10a562f); f_0108ef05(cpu); if cpu.returned_to != 0x10a562f { return; }
  pc = 0x10a562f; continue;
 }
 0x10a562f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11ef528u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ef528u32)); cpu.w32(0x11ef528u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a5661); f_010a589f(cpu); if cpu.returned_to != 0x10a5661 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a566b; continue;
 }
 0x10a566b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5674); }
 let mut pc: u32 = 0x10a5674;
 loop { match pc {
 0x10a5674 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5686; continue;
 }
 0x10a5686 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a5696; continue; }
  pc = 0x10a568c; continue;
 }
 0x10a568c => {
  pc = 0x10a56a3; continue;
 }
 0x10a568e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5686; continue;
 }
 0x10a5696 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11eecf5u32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a568e; continue; }
  pc = 0x10a56a3; continue;
 }
 0x10a56a3 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10a56e2; continue; }
  pc = 0x10a56a9; continue;
 }
 0x10a56a9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a56b0; continue;
 }
 0x10a56b0 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a56c0; continue; }
  pc = 0x10a56b6; continue;
 }
 0x10a56b6 => {
  pc = 0x10a56d0; continue;
 }
 0x10a56b8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a56b0; continue;
 }
 0x10a56c0 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11eed38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10a56b8; continue; }
  pc = 0x10a56d0; continue;
 }
 0x10a56d0 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10a56e2; continue; }
  pc = 0x10a56d6; continue;
 }
 0x10a56d6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a57ad; continue;
 }
 0x10a56e2 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = 0x11eecf4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ef524u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ef378u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef574u32, v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x10a5731); f_0108f030(cpu); if cpu.returned_to != 0x10a5731 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xa0u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11ef30fu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x35u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  cpu.push(0x10a5771); f_0108ef05(cpu); if cpu.returned_to != 0x10a5771 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11ef528u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ef528u32)); cpu.w32(0x11ef528u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a57a3); f_010a589f(cpu); if cpu.returned_to != 0x10a57a3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a57ad; continue;
 }
 0x10a57ad => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a57b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a57b6); }
 let mut pc: u32 = 0x10a57b6;
 loop { match pc {
 0x10a57b6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a57c8; continue;
 }
 0x10a57c8 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a57d8; continue; }
  pc = 0x10a57ce; continue;
 }
 0x10a57ce => {
  pc = 0x10a57e5; continue;
 }
 0x10a57d0 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a57c8; continue;
 }
 0x10a57d8 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11ee6ddu32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a57d0; continue; }
  pc = 0x10a57e5; continue;
 }
 0x10a57e5 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10a57f7; continue; }
  pc = 0x10a57eb; continue;
 }
 0x10a57eb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5896; continue;
 }
 0x10a57f7 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = 0x11ee6dcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xa000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ef520u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ef374u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef574u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x10a584f); f_0108f030(cpu); if cpu.returned_to != 0x10a584f { return; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x80u16; cpu.w16(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x35u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a588c); f_010a59da(cpu); if cpu.returned_to != 0x10a588c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5896; continue;
 }
 0x10a5896 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a589f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a589f); }
 let mut pc: u32 = 0x10a589f;
 loop { match pc {
 0x10a589f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ef378u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11ef378u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a58d5); f_0108ef05(cpu); if cpu.returned_to != 0x10a58d5 { return; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10a5919; continue; }
  pc = 0x10a58db; continue;
 }
 0x10a58db => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a58e9); f_0108ef05(cpu); if cpu.returned_to != 0x10a58e9 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10a590d; continue; }
  pc = 0x10a58f3; continue;
 }
 0x10a58f3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.push(0x10a590d); f_0108ef05(cpu); if cpu.returned_to != 0x10a590d { return; }
  pc = 0x10a590d; continue;
 }
 0x10a590d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a593a; continue;
 }
 0x10a5919 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10a593a; continue; }
  pc = 0x10a5923; continue;
 }
 0x10a5923 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a593a); f_0108ef05(cpu); if cpu.returned_to != 0x10a593a { return; }
  pc = 0x10a593a; continue;
 }
 0x10a593a => {
  { let v: u16 = cpu.r16(0x11ef574u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11ef570u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a596a); f_0108f030(cpu); if cpu.returned_to != 0x10a596a { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5979); f_0108f030(cpu); if cpu.returned_to != 0x10a5979 { return; }
  { let v: u16 = cpu.r16(0x11ef570u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11ef574u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a59b3); f_010a619b(cpu); if cpu.returned_to != 0x10a59b3 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a59d1); f_010a6120(cpu); if cpu.returned_to != 0x10a59d1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a59da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a59da); }
 let mut pc: u32 = 0x10a59da;
 loop { match pc {
 0x10a59da => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a5a10); f_0108ef05(cpu); if cpu.returned_to != 0x10a5a10 { return; }
  { let v: u16 = cpu.r16(0x11ef574u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a5a40); f_0108f030(cpu); if cpu.returned_to != 0x10a5a40 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5a4f); f_0108f030(cpu); if cpu.returned_to != 0x10a5a4f { return; }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11ef574u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a5a89); f_010a619b(cpu); if cpu.returned_to != 0x10a5a89 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5aa7); f_010a6120(cpu); if cpu.returned_to != 0x10a5aa7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5ab0); }
 let mut pc: u32 = 0x10a5ab0;
 loop { match pc {
 0x10a5ab0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5ac6); f_0108e30e(cpu); if cpu.returned_to != 0x10a5ac6 { return; }
  pc = 0x10a5ac6; continue;
 }
 0x10a5ac6 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5ace); f_0108e46c(cpu); if cpu.returned_to != 0x10a5ace { return; }
  { cpu.sub32(cpu.eax, 0xfu32); }
  if !cpu.cf { pc = 0x10a5ae2; continue; }
  pc = 0x10a5ad3; continue;
 }
 0x10a5ad3 => {
  { let v: u32 = 0x1138f60u32; cpu.edx = v; }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5ae0); f_0108ef8d(cpu); if cpu.returned_to != 0x10a5ae0 { return; }
  cpu.poll(); pc = 0x10a5ac6; continue;
 }
 0x10a5ae2 => {
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a5af3); f_0108f030(cpu); if cpu.returned_to != 0x10a5af3 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5b02); f_0108f030(cpu); if cpu.returned_to != 0x10a5b02 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffff70u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5b14); f_0108f030(cpu); if cpu.returned_to != 0x10a5b14 { return; }
  { let v: u8 = 0xb0u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffa1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffff8au32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5b2a); f_0108e30e(cpu); if cpu.returned_to != 0x10a5b2a { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff70u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  cpu.push(0x10a5b3f); f_0108ef05(cpu); if cpu.returned_to != 0x10a5b3f { return; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a5b7e); f_010a619b(cpu); if cpu.returned_to != 0x10a5b7e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5b9c); f_010a6120(cpu); if cpu.returned_to != 0x10a5b9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a5b9f; continue;
 }
 0x10a5b9f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a5b9f; continue; }
  pc = 0x10a5bb1; continue;
 }
 0x10a5bb1 => {
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.edx = v; }
  { let v: u32 = 0x11ef30cu32; cpu.eax = v; }
  cpu.push(0x10a5bc6); f_0108ef05(cpu); if cpu.returned_to != 0x10a5bc6 { return; }
  { cpu.sub8(cpu.r8(0x11ef33du32), 0x0u8); }
  if cpu.zf { pc = 0x10a5bd8; continue; }
  pc = 0x10a5bcf; continue;
 }
 0x10a5bcf => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5bdf; continue;
 }
 0x10a5bd8 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5bdf; continue;
 }
 0x10a5bdf => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5be8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5be8); }
 let mut pc: u32 = 0x10a5be8;
 loop { match pc {
 0x10a5be8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5bfe); f_0108e30e(cpu); if cpu.returned_to != 0x10a5bfe { return; }
  pc = 0x10a5bfe; continue;
 }
 0x10a5bfe => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5c06); f_0108e46c(cpu); if cpu.returned_to != 0x10a5c06 { return; }
  { cpu.sub32(cpu.eax, 0xfu32); }
  if !cpu.cf { pc = 0x10a5c1a; continue; }
  pc = 0x10a5c0b; continue;
 }
 0x10a5c0b => {
  { let v: u32 = 0x1138f60u32; cpu.edx = v; }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5c18); f_0108ef8d(cpu); if cpu.returned_to != 0x10a5c18 { return; }
  cpu.poll(); pc = 0x10a5bfe; continue;
 }
 0x10a5c1a => {
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a5c2b); f_0108f030(cpu); if cpu.returned_to != 0x10a5c2b { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5c3a); f_0108f030(cpu); if cpu.returned_to != 0x10a5c3a { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffff70u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5c4c); f_0108f030(cpu); if cpu.returned_to != 0x10a5c4c { return; }
  { let v: u8 = 0xb1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffa1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffff8au32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5c62); f_0108e30e(cpu); if cpu.returned_to != 0x10a5c62 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff70u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  cpu.push(0x10a5c77); f_0108ef05(cpu); if cpu.returned_to != 0x10a5c77 { return; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a5cb6); f_010a619b(cpu); if cpu.returned_to != 0x10a5cb6 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5cd4); f_010a6120(cpu); if cpu.returned_to != 0x10a5cd4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a5cd7; continue;
 }
 0x10a5cd7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a5cd7; continue; }
  pc = 0x10a5ce9; continue;
 }
 0x10a5ce9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10a5cfb; continue; }
  pc = 0x10a5cf2; continue;
 }
 0x10a5cf2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5d02; continue;
 }
 0x10a5cfb => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5d02; continue;
 }
 0x10a5d02 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5d0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5d0b); }
 let mut pc: u32 = 0x10a5d0b;
 loop { match pc {
 0x10a5d0b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a5d27); f_0108f030(cpu); if cpu.returned_to != 0x10a5d27 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5d36); f_0108f030(cpu); if cpu.returned_to != 0x10a5d36 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5d45); f_0108f030(cpu); if cpu.returned_to != 0x10a5d45 { return; }
  { let v: u8 = 0xb3u8; cpu.w8(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffbdu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1a4u16; cpu.w16(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffff92u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x40u16; cpu.w16(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef326u32; cpu.edx = v; }
  { let v: u32 = 0xffffff96u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5d70); f_0108e30e(cpu); if cpu.returned_to != 0x10a5d70 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  cpu.push(0x10a5d82); f_0108ef05(cpu); if cpu.returned_to != 0x10a5d82 { return; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a5dc1); f_010a619b(cpu); if cpu.returned_to != 0x10a5dc1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5ddf); f_010a6120(cpu); if cpu.returned_to != 0x10a5ddf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a5de2; continue;
 }
 0x10a5de2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10a5de2; continue; }
  pc = 0x10a5df4; continue;
 }
 0x10a5df4 => {
  { let v: u32 = 0x1a4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ef374u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x40u32); cpu.edx = v; }
  { let v: u32 = 0x11ef37cu32; cpu.eax = v; }
  cpu.push(0x10a5e0c); f_0108ef05(cpu); if cpu.returned_to != 0x10a5e0c { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0x11ef37cu32; cpu.edx = v; }
  { let v: u32 = 0x11ef35cu32; cpu.eax = v; }
  cpu.push(0x10a5e20); f_0108ef05(cpu); if cpu.returned_to != 0x10a5e20 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5e27; continue;
 }
 0x10a5e27 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x6u32); }
  if cpu.cf { pc = 0x10a5e37; continue; }
  pc = 0x10a5e2d; continue;
 }
 0x10a5e2d => {
  pc = 0x10a5e4f; continue;
 }
 0x10a5e2f => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5e27; continue;
 }
 0x10a5e37 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11ef35cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10a5e4d; continue; }
  pc = 0x10a5e43; continue;
 }
 0x10a5e43 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11ef35cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10a5e4d; continue;
 }
 0x10a5e4d => {
  cpu.poll(); pc = 0x10a5e2f; continue;
 }
 0x10a5e4f => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5e56; continue;
 }
 0x10a5e56 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xfu32); }
  if cpu.cf { pc = 0x10a5e66; continue; }
  pc = 0x10a5e5c; continue;
 }
 0x10a5e5c => {
  pc = 0x10a5e72; continue;
 }
 0x10a5e5e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5e56; continue;
 }
 0x10a5e66 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x11ef35cu32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a5e5e; continue;
 }
 0x10a5e72 => {
  { let v: u32 = 0x11ef35cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a5e7d); f_010a5ab0(cpu); if cpu.returned_to != 0x10a5e7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a5e8d; continue; }
  pc = 0x10a5e84; continue;
 }
 0x10a5e84 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5e94; continue;
 }
 0x10a5e8d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5e94; continue;
 }
 0x10a5e94 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5e9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5e9d); }
 let mut pc: u32 = 0x10a5e9d;
 loop { match pc {
 0x10a5e9d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a5eaf; continue;
 }
 0x10a5eaf => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10a5ec2; continue; }
  pc = 0x10a5eb5; continue;
 }
 0x10a5eb5 => {
  pc = 0x10a5fa3; continue;
 }
 0x10a5eba => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a5eaf; continue;
 }
 0x10a5ec2 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10a5edf; continue; }
  pc = 0x10a5ecf; continue;
 }
 0x10a5ecf => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee720u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x10a5ee4; continue; }
  pc = 0x10a5edf; continue;
 }
 0x10a5edf => {
  pc = 0x10a5f9e; continue;
 }
 0x10a5ee4 => {
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11ef534u32; cpu.eax = v; }
  cpu.push(0x10a5ef5); f_0108f030(cpu); if cpu.returned_to != 0x10a5ef5 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5f04); f_0108f030(cpu); if cpu.returned_to != 0x10a5f04 { return; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a5f13); f_0108f030(cpu); if cpu.returned_to != 0x10a5f13 { return; }
  { let v: u8 = 0x35u8; cpu.w8(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffbdu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11ef56cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffff92u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4eu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11ee6deu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef524u32); cpu.eax = v; }
  cpu.push(0x10a5f46); f_0108ef05(cpu); if cpu.returned_to != 0x10a5f46 { return; }
  { let v: u16 = cpu.r16(0x11ef570u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ef556u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11ef544u32, v); }
  { let v: u32 = 0x100u32; cpu.w32(0x11ef550u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x5cu16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a5f7d); f_010a619b(cpu); if cpu.returned_to != 0x10a5f7d { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11ef534u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10a5f9b); f_010a6120(cpu); if cpu.returned_to != 0x10a5f9b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10a5f9e; continue;
 }
 0x10a5f9e => {
  cpu.poll(); pc = 0x10a5eba; continue;
 }
 0x10a5fa3 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5fb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5fb3); }
 let mut pc: u32 = 0x10a5fb3;
 loop { match pc {
 0x10a5fb3 => {
  { cpu.sub32(cpu.r32(0x1133e70u32), 0x0u32); }
  if cpu.zf { pc = 0x10a5fc2; continue; }
  pc = 0x10a5fbc; continue;
 }
 0x10a5fbc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5fc2 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5fca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5fca); }
 let mut pc: u32 = 0x10a5fca;
 loop { match pc {
 0x10a5fca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x10a5ff7; continue; }
  pc = 0x10a5fdb; continue;
 }
 0x10a5fdb => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5ff7; continue; }
  pc = 0x10a5fdf; continue;
 }
 0x10a5fdf => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a6006; continue;
 }
 0x10a5ff7 => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x10a6006; continue;
 }
 0x10a6006 => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xe10u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = 0x16eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x10a606d; continue; }
  pc = 0x10a6065; continue;
 }
 0x10a6065 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x10a606d; continue;
 }
 0x10a606d => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x10a6078); f_010aaf5a(cpu); if cpu.returned_to != 0x10a6078 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x10a607d; continue;
 }
 0x10a607d => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x10a6086; continue; }
  pc = 0x10a6081; continue;
 }
 0x10a6081 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x10a607d; continue;
 }
 0x10a6086 => {
  { let v: u32 = 0x113927cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x10a609c); f_010aaf5a(cpu); if cpu.returned_to != 0x10a609c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a60a5; continue; }
  pc = 0x10a60a0; continue;
 }
 0x10a60a0 => {
  { let v: u32 = 0x1139296u32; cpu.esi = v; }
  pc = 0x10a60a5; continue;
 }
 0x10a60a5 => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x10a60bd; continue; }
  pc = 0x10a60bc; continue;
 }
 0x10a60bc => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a60bd; continue;
 }
 0x10a60bd => {
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a60df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a60df); }
 let mut pc: u32 = 0x10a60df;
 loop { match pc {
 0x10a60df => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x10a60fa); f_010a5fca(cpu); if cpu.returned_to != 0x10a60fa { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a60fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a60fd); }
 let mut pc: u32 = 0x10a60fd;
 loop { match pc {
 0x10a60fd => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x11ef578u32; cpu.edx = v; }
  cpu.push(0x10a6108); f_010a60df(cpu); if cpu.returned_to != 0x10a6108 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a610a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a610a); }
 let mut pc: u32 = 0x10a610a;
 loop { match pc {
 0x10a610a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10a610d; continue;
 }
 0x10a610d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a611d; continue; }
  pc = 0x10a6116; continue;
 }
 0x10a6116 => {
  cpu.push(0x10a611b); f_0108de27(cpu); if cpu.returned_to != 0x10a611b { return; }
  cpu.poll(); pc = 0x10a610d; continue;
 }
 0x10a611d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6120); }
 let mut pc: u32 = 0x10a6120;
 loop { match pc {
 0x10a6120 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.interrupt(0x31); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x10a6195); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a619b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a619b); }
 let mut pc: u32 = 0x10a619b;
 loop { match pc {
 0x10a619b => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a61a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a61a0); }
 let mut pc: u32 = 0x10a61a0;
 loop { match pc {
 0x10a61a0 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x504u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x10a61ca; continue; }
  pc = 0x10a61be; continue;
 }
 0x10a61be => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a61ca; continue;
 }
 0x10a61ca => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a621e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a621e); }
 let mut pc: u32 = 0x10a621e;
 loop { match pc {
 0x10a621e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x10a6237; continue; }
  pc = 0x10a6232; continue;
 }
 0x10a6232 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x10a6237; continue;
 }
 0x10a6237 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a623c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a623c); }
 let mut pc: u32 = 0x10a623c;
 loop { match pc {
 0x10a623c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x1133e90u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a6307; continue; }
  pc = 0x10a6251; continue;
 }
 0x10a6251 => {
  { let v = cpu.r16(0x1133e85u32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133e81u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10a6267; continue;
 }
 0x10a6267 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x10a627f; continue; }
  pc = 0x10a626c; continue;
 }
 0x10a626c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x10a627a; continue; }
  pc = 0x10a6276; continue;
 }
 0x10a6276 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x10a626c; continue;
 }
 0x10a627a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x10a6267; continue;
 }
 0x10a627f => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a6288; continue; }
  pc = 0x10a6283; continue;
 }
 0x10a6283 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6288; continue;
 }
 0x10a6288 => {
  cpu.push(0x10a628d); f_01094a9a(cpu); if cpu.returned_to != 0x10a628d { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6307; continue; }
  pc = 0x10a6299; continue;
 }
 0x10a6299 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10a62a8); f_01094a9a(cpu); if cpu.returned_to != 0x10a62a8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6300; continue; }
  pc = 0x10a62ac; continue;
 }
 0x10a62ac => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133e90u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10a62bb; continue;
 }
 0x10a62bb => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x10a62db; continue; }
  pc = 0x10a62c1; continue;
 }
 0x10a62c1 => {
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x10a62ca; continue;
 }
 0x10a62ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a62ca; continue; }
  pc = 0x10a62d5; continue;
 }
 0x10a62d5 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a62bb; continue;
 }
 0x10a62db => {
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133e94u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x10a62fe); f_0108f030(cpu); if cpu.returned_to != 0x10a62fe { return; }
  pc = 0x10a6307; continue;
 }
 0x10a6300 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a6307); f_01094b7d(cpu); if cpu.returned_to != 0x10a6307 { return; }
  pc = 0x10a6307; continue;
 }
 0x10a6307 => {
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
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
pub fn f_010a6310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6310); }
 let mut pc: u32 = 0x10a6310;
 loop { match pc {
 0x10a6310 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x10a6359; continue; }
  pc = 0x10a6319; continue;
 }
 0x10a6319 => {
  if !cpu.cf { pc = 0x10a6343; continue; }
  pc = 0x10a631b; continue;
 }
 0x10a631b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x10a6343; continue; }
  pc = 0x10a6322; continue;
 }
 0x10a6322 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.df = true; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6343 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  pc = 0x10a6359; continue;
 }
 0x10a6359 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a635d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a635d); }
 let mut pc: u32 = 0x10a635d;
 loop { match pc {
 0x10a635d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6370); }
 let mut pc: u32 = 0x10a6370;
 loop { match pc {
 0x10a6370 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10a635du32; cpu.ecx = v; }
  cpu.push(0x10a637e); f_01098c9e(cpu); if cpu.returned_to != 0x10a637e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6385(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6385); } r_010a6385(cpu, 0x10a6385); }
pub fn f_010a640f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a640f); } r_010a6385(cpu, 0x10a640f); }
pub fn f_010a6421(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6421); } r_010a6385(cpu, 0x10a6421); }
pub fn f_010a6446(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6446); } r_010a6385(cpu, 0x10a6446); }
pub fn f_010a64ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a64ab); } r_010a6385(cpu, 0x10a64ab); }
pub fn f_010a64b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a64b5); } r_010a6385(cpu, 0x10a64b5); }
fn r_010a6385(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a6385 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a638b, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x10a6390, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x10a6411; continue; }
  pc = 0x10a63a0; continue;
 }
 0x10a63a0 => {
  if cpu.cf { pc = 0x10a6415; continue; }
  pc = 0x10a63a2; continue;
 }
 0x10a63a2 => {
  if cpu.zf { pc = 0x10a6419; continue; }
  pc = 0x10a63a4; continue;
 }
 0x10a63a4 => {
  if (cpu.cf || cpu.zf) { cpu.call(0x10a641d); return; }
  pc = 0x10a63a6; continue;
 }
 0x10a63a6 => {
  if cpu.sf { pc = 0x10a6421; continue; }
  pc = 0x10a63a8; continue;
 }
 0x10a63a8 => {
  if cpu.pf() { f_010a63aa(cpu); return; }
  f_010a63aa(cpu); return;
 }
 0x10a640f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10a6415; continue;
 }
 0x10a6411 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a6415; continue;
 }
 0x10a6415 => {
  { let v: u32 = 0x1138f64u32; cpu.eax = v; }
  cpu.push(0x10a641f); f_0109aa8f(cpu); if cpu.returned_to != 0x10a641f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6419 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if cpu.of { pc = 0x10a6463; continue; }
  cpu.call(0x10a641d); return;
 }
 0x10a6421 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a6427, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x10a642c, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x10a64ad; continue; }
  pc = 0x10a643c; continue;
 }
 0x10a643c => {
  if cpu.cf { pc = 0x10a64b1; continue; }
  pc = 0x10a643e; continue;
 }
 0x10a643e => {
  if cpu.zf { pc = 0x10a64b5; continue; }
  pc = 0x10a6440; continue;
 }
 0x10a6440 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a64b9; continue; }
  pc = 0x10a6442; continue;
 }
 0x10a6442 => {
  if cpu.sf { pc = 0x10a64bd; continue; }
  pc = 0x10a6444; continue;
 }
 0x10a6444 => {
  if cpu.pf() { pc = 0x10a6446; continue; }
  pc = 0x10a6446; continue;
 }
 0x10a6446 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a645c; continue;
 }
 0x10a645c => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10a6421u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a645c; continue; }
  pc = 0x10a647c; continue;
 }
 0x10a6463 => {
  { let v: u8 = (cpu.eax as u8) & 0x31u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.ebx >> 8) as u8), (cpu.ecx & 0xff)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x24548b13u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10a6421u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a645c; continue; }
  pc = 0x10a647c; continue;
 }
 0x10a647c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a647c; continue; }
  pc = 0x10a6486; continue;
 }
 0x10a6486 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a64ab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x10a64b3); f_010a64b5(cpu); if cpu.returned_to != 0x10a64b3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a64ad => {
  { let v: u32 = cpu.shr32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a64b1; continue;
 }
 0x10a64b1 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a64b5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x10a64b9; continue;
 }
 0x10a64b9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  pc = 0x10a64bd; continue;
 }
 0x10a64bd => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x10a64e1; continue; }
  pc = 0x10a64da; continue;
 }
 0x10a64da => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a64e1 => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x10a64ed; continue; }
  pc = 0x10a64e6; continue;
 }
 0x10a64e6 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a64ed => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10a64f9; continue; }
  pc = 0x10a64f2; continue;
 }
 0x10a64f2 => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a64f9 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x10a6505; continue; }
  pc = 0x10a64fe; continue;
 }
 0x10a64fe => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a6505 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x10a6511; continue; }
  pc = 0x10a650a; continue;
 }
 0x10a650a => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a6511 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x10a651b; continue; }
  pc = 0x10a6516; continue;
 }
 0x10a6516 => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x10a651b; continue;
 }
 0x10a651b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1134dbcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x10a653f; continue; }
  pc = 0x10a6532; continue;
 }
 0x10a6532 => {
  { let v = cpu.rf64(0x11392b0u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x10a6581; continue;
 }
 0x10a653f => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x10a654e; continue; }
  pc = 0x10a6544; continue;
 }
 0x10a6544 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a6581; continue;
 }
 0x10a654e => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x10a6562; continue; }
  pc = 0x10a6553; continue;
 }
 0x10a6553 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a6581; continue;
 }
 0x10a6562 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x10a6576; continue; }
  pc = 0x10a6567; continue;
 }
 0x10a6567 => {
  { let v: u32 = cpu.r32(0x11392b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11392b4u32); cpu.eax = v; }
  pc = 0x10a657e; continue;
 }
 0x10a6576 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x10a657e; continue;
 }
 0x10a657e => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a6581; continue;
 }
 0x10a6581 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a6589); f_010ad6ba(cpu); if cpu.returned_to != 0x10a6589 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a63aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a63aa); }
 let mut pc: u32 = 0x10a63aa;
 loop { match pc {
 0x10a63aa => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a63c0; continue;
 }
 0x10a63c0 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10a6385u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a63c0; continue; }
  pc = 0x10a63e0; continue;
 }
 0x10a63e0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a63e0; continue; }
  pc = 0x10a63ea; continue;
 }
 0x10a63ea => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a63f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a63f4); }
 let mut pc: u32 = 0x10a63f4;
 loop { match pc {
 0x10a63f4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x10a6406; continue; }
  pc = 0x10a63fc; continue;
 }
 0x10a63fc => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a6406; continue; }
  pc = 0x10a6400; continue;
 }
 0x10a6400 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10a6406; continue;
 }
 0x10a6406 => {
  cpu.push(0x10a640b); f_010a63aa(cpu); if cpu.returned_to != 0x10a640b { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6490); }
 let mut pc: u32 = 0x10a6490;
 loop { match pc {
 0x10a6490 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x10a64a2; continue; }
  pc = 0x10a6498; continue;
 }
 0x10a6498 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a64a2; continue; }
  pc = 0x10a649c; continue;
 }
 0x10a649c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10a64a2; continue;
 }
 0x10a64a2 => {
  cpu.push(0x10a64a7); f_010a6446(cpu); if cpu.returned_to != 0x10a64a7 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6590); }
 let mut pc: u32 = 0x10a6590;
 loop { match pc {
 0x10a6590 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let a = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let a = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
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
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a65a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a65a9); }
 let mut pc: u32 = 0x10a65a9;
 loop { match pc {
 0x10a65a9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a65cc; continue; }
  pc = 0x10a65b9; continue;
 }
 0x10a65b9 => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a65cc; continue; }
  pc = 0x10a65be; continue;
 }
 0x10a65be => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x10a65cb); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a65cc => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x10a65d5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a65d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a65d6); }
 let mut pc: u32 = 0x10a65d6;
 loop { match pc {
 0x10a65d6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a65f3; continue; }
  pc = 0x10a65e5; continue;
 }
 0x10a65e5 => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a65f3; continue; }
  pc = 0x10a65ea; continue;
 }
 0x10a65ea => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x10a65fd; continue;
 }
 0x10a65f3 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10a65fd; continue;
 }
 0x10a65fd => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6639(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6639); }
 let mut pc: u32 = 0x10a6639;
 loop { match pc {
 0x10a6639 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1133c79u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1133c79u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1133c38u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1133c79u32, v); }
  pc = 0x10a669f; continue;
 }
 0x10a665d => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x10a6667); f_01094a9a(cpu); if cpu.returned_to != 0x10a6667 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a668c; continue; }
  pc = 0x10a666d; continue;
 }
 0x10a666d => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x10a6677); f_01094a9a(cpu); if cpu.returned_to != 0x10a6677 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a668c; continue; }
  pc = 0x10a667d; continue;
 }
 0x10a667d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x1138ff4u32; cpu.eax = v; }
  cpu.push(0x10a668c); f_0109aa8f(cpu); if cpu.returned_to != 0x10a668c { return; }
  pc = 0x10a668c; continue;
 }
 0x10a668c => {
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee150u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a669f; continue;
 }
 0x10a669f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a665d; continue; }
  pc = 0x10a66a6; continue;
 }
 0x10a66a6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x11eca18u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66b0); }
 let mut pc: u32 = 0x10a66b0;
 loop { match pc {
 0x109a1df => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x109a1e0; continue;
 }
 0x109a1e0 => {
  { let v: u32 = cpu.r32(0x11eca18u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109a1fb; continue; }
  pc = 0x109a1ea; continue;
 }
 0x109a1ea => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x109a1f3); f_01094b7d(cpu); if cpu.returned_to != 0x109a1f3 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x11eca18u32, v); }
  cpu.poll(); pc = 0x109a1e0; continue;
 }
 0x109a1fb => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a66b0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10a66b7); f_010a66c1(cpu); if cpu.returned_to != 0x10a66b7 { return; }
  cpu.poll(); pc = 0x109a1df; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66c1); }
 let mut pc: u32 = 0x10a66c1;
 loop { match pc {
 0x10a66c1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1133c38u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.eax = v; }
  pc = 0x10a670e; continue;
 }
 0x10a66e3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x10a6706; continue; }
  pc = 0x10a66f3; continue;
 }
 0x10a66f3 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x10a6706; continue; }
  pc = 0x10a66f9; continue;
 }
 0x10a66f9 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x10a670c; continue; }
  pc = 0x10a66fd; continue;
 }
 0x10a66fd => {
  { cpu.sub32(cpu.eax, 0x1133cbau32); }
  if !cpu.cf { pc = 0x10a6706; continue; }
  pc = 0x10a6704; continue;
 }
 0x10a6704 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a6706; continue;
 }
 0x10a6706 => {
  cpu.push(0x10a670b); f_0108f1f5(cpu); if cpu.returned_to != 0x10a670b { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10a670c; continue;
 }
 0x10a670c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10a670e; continue;
 }
 0x10a670e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a66e3; continue; }
  pc = 0x10a6712; continue;
 }
 0x10a6712 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6719(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6719); }
 let mut pc: u32 = 0x10a6719;
 loop { match pc {
 0x10a6719 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x10a671a; continue;
 }
 0x10a671a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x10a6726; continue; }
  pc = 0x10a6722; continue;
 }
 0x10a6722 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x10a671a; continue;
 }
 0x10a6726 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6728); }
 let mut pc: u32 = 0x10a6728;
 loop { match pc {
 0x10a6728 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(8); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10a673f; continue; }
  pc = 0x10a6739; continue;
 }
 0x10a6739 => {
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.edi = v; }
  pc = 0x10a673f; continue;
 }
 0x10a673f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a675b; continue; }
  pc = 0x10a6745; continue;
 }
 0x10a6745 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10a6756; continue;
 }
 0x10a6749 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x10a6750); f_0108e46c(cpu); if cpu.returned_to != 0x10a6750 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10a6756; continue;
 }
 0x10a6756 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6749; continue; }
  pc = 0x10a675b; continue;
 }
 0x10a675b => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10a676e; continue; }
  pc = 0x10a6762; continue;
 }
 0x10a6762 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x10a6769); f_0108e46c(cpu); if cpu.returned_to != 0x10a6769 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10a676e; continue;
 }
 0x10a676e => {
  { let v: u32 = cpu.r32(0x1134f98u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x1134f98u32, v); }
  cpu.push(0x10a678a); f_01094a9a(cpu); if cpu.returned_to != 0x10a678a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a67bf; continue; }
  pc = 0x10a678e; continue;
 }
 0x10a678e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a6795); f_01094a9a(cpu); if cpu.returned_to != 0x10a6795 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a67bf; continue; }
  pc = 0x10a6799; continue;
 }
 0x10a6799 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x10a67a3); f_01099923(cpu); if cpu.returned_to != 0x10a67a3 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x10a67ad); f_01099946(cpu); if cpu.returned_to != 0x10a67ad { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1134f98u32, v); }
  pc = 0x10a67b5; continue;
 }
 0x10a67b5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10a685a; continue;
 }
 0x10a67bf => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1134f98u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a67f0; continue; }
  pc = 0x10a67dc; continue;
 }
 0x10a67dc => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x10a67eb; continue;
 }
 0x10a67e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.push(0x10a67e7); f_010a6719(cpu); if cpu.returned_to != 0x10a67e7 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  pc = 0x10a67eb; continue;
 }
 0x10a67eb => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a67e0; continue; }
  pc = 0x10a67f0; continue;
 }
 0x10a67f0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10a6803; continue; }
  pc = 0x10a67fa; continue;
 }
 0x10a67fa => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.push(0x10a6803); f_0108e30e(cpu); if cpu.returned_to != 0x10a6803 { return; }
  pc = 0x10a6803; continue;
 }
 0x10a6803 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a6824; continue; }
  pc = 0x10a680a; continue;
 }
 0x10a680a => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x10a681e; continue;
 }
 0x10a680f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a6814; continue; }
  pc = 0x10a6813; continue;
 }
 0x10a6813 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10a6814; continue;
 }
 0x10a6814 => {
  cpu.push(0x10a6819); f_0108e46c(cpu); if cpu.returned_to != 0x10a6819 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x10a681e; continue;
 }
 0x10a681e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a680f; continue; }
  pc = 0x10a6824; continue;
 }
 0x10a6824 => {
  { cpu.sub32(cpu.edx, 0x7eu32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a684c; continue; }
  pc = 0x10a6829; continue;
 }
 0x10a6829 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x10a6833); f_01099923(cpu); if cpu.returned_to != 0x10a6833 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x10a683d); f_01099946(cpu); if cpu.returned_to != 0x10a683d { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10a6847); f_01094b7d(cpu); if cpu.returned_to != 0x10a6847 { return; }
  cpu.poll(); pc = 0x10a67b5; continue;
 }
 0x10a684c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x10a685a; continue;
 }
 0x10a685a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6860); }
 let mut pc: u32 = 0x10a6860;
 loop { match pc {
 0x10a6860 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10a686f; continue; }
  pc = 0x10a686e; continue;
 }
 0x10a686e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10a686f; continue;
 }
 0x10a686f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a6897; continue; }
  pc = 0x10a6874; continue;
 }
 0x10a6874 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a6897; continue; }
  pc = 0x10a687c; continue;
 }
 0x10a687c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.push(0x10a6888); f_010a6719(cpu); if cpu.returned_to != 0x10a6888 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10a6897; continue; }
  pc = 0x10a688f; continue;
 }
 0x10a688f => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10a687c; continue;
 }
 0x10a6897 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a68a1; continue; }
  pc = 0x10a689b; continue;
 }
 0x10a689b => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a68a1 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a68b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a68b3); }
 let mut pc: u32 = 0x10a68b3;
 loop { match pc {
 0x10a68b3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e1cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1134e20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e2eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1134e2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1134e32u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e38u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1134e22u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1134e26u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x1134e40u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x1134e3cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1134e42u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1134e1cu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x10a692d; continue; }
  pc = 0x10a6927; continue;
 }
 0x10a6927 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10a692f; continue;
 }
 0x10a692d => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a692f; continue;
 }
 0x10a692f => {
  { cpu.cf = false; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1133e78u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x4bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.r16(0x1134e40u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x1134e3cu32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v = cpu.r16(0x1134e42u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x0u32; cpu.w32(0x1133e78u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x10a696e; continue; }
  pc = 0x10a696a; continue;
 }
 0x10a696a => {
  { let v: u8 = 0x4du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x10a696e; continue;
 }
 0x10a696e => {
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  cpu.flat_ds(0x10a6973); cpu.push(0x10a6978); f_01099959(cpu); if cpu.returned_to != 0x10a6978 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x10a6980); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6981(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6981); }
 let mut pc: u32 = 0x10a6981;
 loop { match pc {
 0x10a6981 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6990; continue; }
  pc = 0x10a698b; continue;
 }
 0x10a698b => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6990 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x93u32); }
  if cpu.cf { pc = 0x10a69a3; continue; }
  pc = 0x10a699e; continue;
 }
 0x10a699e => {
  { let v: u32 = 0x92u32; cpu.edx = v; }
  pc = 0x10a69a3; continue;
 }
 0x10a69a3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a69aa); f_0109b1c2(cpu); if cpu.returned_to != 0x10a69aa { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a69b7); f_0109b48f(cpu); if cpu.returned_to != 0x10a69b7 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a69c0); f_0109b200(cpu); if cpu.returned_to != 0x10a69c0 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a69cd); f_0109b200(cpu); if cpu.returned_to != 0x10a69cd { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a69d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a69d3); }
 let mut pc: u32 = 0x10a69d3;
 loop { match pc {
 0x10a69d3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10a6a14; continue; }
  pc = 0x10a69e5; continue;
 }
 0x10a69e5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x10a6a14; continue; }
  pc = 0x10a69ef; continue;
 }
 0x10a69ef => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6a04; continue; }
  pc = 0x10a69f3; continue;
 }
 0x10a69f3 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a6a04; continue;
 }
 0x10a6a04 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a6a10); f_0109b231(cpu); if cpu.returned_to != 0x10a6a10 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x10a6a1f; continue;
 }
 0x10a6a14 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6a1f; continue; }
  pc = 0x10a6a18; continue;
 }
 0x10a6a18 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10a6a1f; continue;
 }
 0x10a6a1f => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10a6a25; continue;
 }
 0x10a6a25 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10a6a27; continue;
 }
 0x10a6a27 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a6a2e); f_0109b562(cpu); if cpu.returned_to != 0x10a6a2e { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6a6f; continue; }
  pc = 0x10a6a34; continue;
 }
 0x10a6a34 => {
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x10a6a3e; continue; }
  pc = 0x10a6a39; continue;
 }
 0x10a6a39 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a6a27; continue;
 }
 0x10a6a3e => {
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x10a6a5a; continue; }
  pc = 0x10a6a47; continue;
 }
 0x10a6a47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10a6a5c; continue;
 }
 0x10a6a5a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6a5c; continue;
 }
 0x10a6a5c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x10a6a69; continue; }
  pc = 0x10a6a64; continue;
 }
 0x10a6a64 => {
  { cpu.sub32(cpu.edx, 0x2fu32); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6a27; continue; }
  pc = 0x10a6a69; continue;
 }
 0x10a6a69 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10a6a25; continue;
 }
 0x10a6a6f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x10a6a7d); f_010a6981(cpu); if cpu.returned_to != 0x10a6a7d { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10a6a85; continue; }
  pc = 0x10a6a83; continue;
 }
 0x10a6a83 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x10a6a85; continue;
 }
 0x10a6a85 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x10a6a96); f_010a6981(cpu); if cpu.returned_to != 0x10a6a96 { return; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10a6aa5); f_010a6981(cpu); if cpu.returned_to != 0x10a6aa5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6aae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6aae); }
 let mut pc: u32 = 0x10a6aae;
 loop { match pc {
 0x10a6aae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6aaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6aaf); }
 let mut pc: u32 = 0x10a6aaf;
 loop { match pc {
 0x10a6aaf => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x10a6ab9; continue; }
  pc = 0x10a6ab4; continue;
 }
 0x10a6ab4 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x10a6ac2; continue; }
  pc = 0x10a6ab9; continue;
 }
 0x10a6ab9 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x10a6ac0; continue; }
  pc = 0x10a6abe; continue;
 }
 0x10a6abe => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a6ac0; continue;
 }
 0x10a6ac0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x10a6ac2; continue;
 }
 0x10a6ac2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6ac3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6ac3); }
 let mut pc: u32 = 0x10a6ac3;
 loop { match pc {
 0x10a6ac3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a6ae7; continue; }
  pc = 0x10a6ad8; continue;
 }
 0x10a6ad8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10a6ae7; continue; }
  pc = 0x10a6ade; continue;
 }
 0x10a6ade => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10a6ae7; continue;
 }
 0x10a6ae7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6ba5; continue; }
  pc = 0x10a6af3; continue;
 }
 0x10a6af3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10a6ba5; continue; }
  pc = 0x10a6afc; continue;
 }
 0x10a6afc => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10a6b07); f_0109b562(cpu); if cpu.returned_to != 0x10a6b07 { return; }
  cpu.push(0x10a6b0c); f_010a6aaf(cpu); if cpu.returned_to != 0x10a6b0c { return; }
  cpu.push(0x10a6b11); f_0109b5ba(cpu); if cpu.returned_to != 0x10a6b11 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a6b16; continue;
 }
 0x10a6b16 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a6b2e; continue; }
  pc = 0x10a6b1e; continue;
 }
 0x10a6b1e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6b16; continue; }
  pc = 0x10a6b2e; continue;
 }
 0x10a6b2e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x10a6b4c; continue; }
  pc = 0x10a6b38; continue;
 }
 0x10a6b38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10a6b4e; continue;
 }
 0x10a6b4c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6b4e; continue;
 }
 0x10a6b4e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6b6b; continue; }
  pc = 0x10a6b5a; continue;
 }
 0x10a6b5a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10a6b6b; continue;
 }
 0x10a6b6b => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6afc; continue; }
  pc = 0x10a6b73; continue;
 }
 0x10a6b73 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10a6b82; continue; }
  pc = 0x10a6b7a; continue;
 }
 0x10a6b7a => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a6b82; continue;
 }
 0x10a6b82 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x10a6b8c); f_010aa719(cpu); if cpu.returned_to != 0x10a6b8c { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10a6b9e; continue; }
  pc = 0x10a6b9b; continue;
 }
 0x10a6b9b => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10a6ba5; continue;
 }
 0x10a6b9e => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a6ba5; continue;
 }
 0x10a6ba5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10a6bb4; continue; }
  pc = 0x10a6bac; continue;
 }
 0x10a6bac => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a6bb4; continue;
 }
 0x10a6bb4 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a6c5c; continue; }
  pc = 0x10a6bbf; continue;
 }
 0x10a6bbf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10a6bca); f_0109b562(cpu); if cpu.returned_to != 0x10a6bca { return; }
  cpu.push(0x10a6bcf); f_010a6aaf(cpu); if cpu.returned_to != 0x10a6bcf { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10a6be2; continue; }
  pc = 0x10a6bd7; continue;
 }
 0x10a6bd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10a6be2; continue; }
  pc = 0x10a6be0; continue;
 }
 0x10a6be0 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x10a6be2; continue;
 }
 0x10a6be2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10a6bed); f_0109b562(cpu); if cpu.returned_to != 0x10a6bed { return; }
  cpu.push(0x10a6bf2); f_010a6aaf(cpu); if cpu.returned_to != 0x10a6bf2 { return; }
  cpu.push(0x10a6bf7); f_0109b5ba(cpu); if cpu.returned_to != 0x10a6bf7 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a6bfc; continue;
 }
 0x10a6bfc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a6c14; continue; }
  pc = 0x10a6c04; continue;
 }
 0x10a6c04 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6bfc; continue; }
  pc = 0x10a6c14; continue;
 }
 0x10a6c14 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6c30; continue; }
  pc = 0x10a6c1e; continue;
 }
 0x10a6c1e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10a6c30; continue;
 }
 0x10a6c30 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a6c50; continue; }
  pc = 0x10a6c3d; continue;
 }
 0x10a6c3d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10a6c52; continue;
 }
 0x10a6c50 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6c52; continue;
 }
 0x10a6c52 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6be2; continue; }
  pc = 0x10a6c5a; continue;
 }
 0x10a6c5a => {
  pc = 0x10a6c6b; continue;
 }
 0x10a6c5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x10a6c6b; continue; }
  pc = 0x10a6c69; continue;
 }
 0x10a6c69 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x10a6c6b; continue;
 }
 0x10a6c6b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6ca1; continue; }
  pc = 0x10a6c73; continue;
 }
 0x10a6c73 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10a6ca1; continue; }
  pc = 0x10a6c78; continue;
 }
 0x10a6c78 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x10a6c8a; continue; }
  pc = 0x10a6c85; continue;
 }
 0x10a6c85 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10a6c8a; continue;
 }
 0x10a6c8a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10a6ca1; continue; }
  pc = 0x10a6c94; continue;
 }
 0x10a6c94 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10a6c8a; continue;
 }
 0x10a6ca1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6cae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6cae); }
 let mut pc: u32 = 0x10a6cae;
 loop { match pc {
 0x10a6cae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x1134f98u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a6ccf; continue; }
  pc = 0x10a6cca; continue;
 }
 0x10a6cca => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a6cd6; continue; }
  pc = 0x10a6ccf; continue;
 }
 0x10a6ccf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6cd8; continue;
 }
 0x10a6cd6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6cd8; continue;
 }
 0x10a6cd8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6d11; continue; }
  pc = 0x10a6cdc; continue;
 }
 0x10a6cdc => {
  cpu.push(0x10a6ce1); f_01099ba6(cpu); if cpu.returned_to != 0x10a6ce1 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10a6d11; continue; }
  pc = 0x10a6ce6; continue;
 }
 0x10a6ce6 => {
  { let v: u32 = 0x108ee87u32; cpu.edx = v; }
  { let v: u32 = 0x108ee7cu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x108ee7cu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10a6d11; continue;
 }
 0x10a6d11 => {
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
pub fn f_010a6d17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6d17); }
 let mut pc: u32 = 0x10a6d17;
 loop { match pc {
 0x10a6d17 => {
  { cpu.sub8(cpu.r8(0x1134e4cu32), 0x0u8); }
  if cpu.zf { pc = 0x10a6d3a; continue; }
  pc = 0x10a6d20; continue;
 }
 0x10a6d20 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x10a6d28, "Int3");
  pc = 0x10a6d31; continue;
 }
 0x10a6d31 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6d3a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6d3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6d3d); }
 let mut pc: u32 = 0x10a6d3d;
 loop { match pc {
 0x10a6d3d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133c15u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a6d8e; continue; }
  pc = 0x10a6d6b; continue;
 }
 0x10a6d6b => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x1139024u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (cpu.cf || cpu.zf) { pc = 0x10a6d8e; continue; }
  pc = 0x10a6d87; continue;
 }
 0x10a6d87 => {
  { let v: u8 = cpu.r8(0x1133f74u32) | 0x1u8; cpu.w8(0x1133f74u32, v); }
  pc = 0x10a6d8e; continue;
 }
 0x10a6d8e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6da0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6da0); } r_010a6da0(cpu, 0x10a6da0); }
pub fn f_010a6f00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f00); } r_010a6da0(cpu, 0x10a6f00); }
pub fn f_010a6f0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f0b); } r_010a6da0(cpu, 0x10a6f0b); }
pub fn f_010a6f18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f18); } r_010a6da0(cpu, 0x10a6f18); }
pub fn f_010a6f2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f2b); } r_010a6da0(cpu, 0x10a6f2b); }
pub fn f_010a6f40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f40); } r_010a6da0(cpu, 0x10a6f40); }
pub fn f_010a6f53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f53); } r_010a6da0(cpu, 0x10a6f53); }
pub fn f_010a6f68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f68); } r_010a6da0(cpu, 0x10a6f68); }
pub fn f_010a6f7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a6f7b); } r_010a6da0(cpu, 0x10a6f7b); }
fn r_010a6da0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a6da0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x10a6f90; continue; }
  pc = 0x10a6db9; continue;
 }
 0x10a6db9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x10a6f90; continue; }
  pc = 0x10a6dc5; continue;
 }
 0x10a6dc5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7fffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x10a6f90; continue; }
  pc = 0x10a6dd9; continue;
 }
 0x10a6dd9 => {
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10a6f90; continue; }
  pc = 0x10a6de4; continue;
 }
 0x10a6de4 => {
  { cpu.sub32(cpu.ecx, 0x7fffu32); }
  if cpu.zf { pc = 0x10a6f90; continue; }
  pc = 0x10a6df0; continue;
 }
 0x10a6df0 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x10a6f90; continue; }
  pc = 0x10a6dfb; continue;
 }
 0x10a6dfb => {
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.fpu_control; cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x33fu32; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xf3ffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b);  }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10a6e30; continue; }
  pc = 0x10a6e2d; continue;
 }
 0x10a6e2d => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10a6e30; continue;
 }
 0x10a6e30 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x10a6e3b); f_010a462b(cpu); if cpu.returned_to != 0x10a6e3b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.rf32(0x11394f4u32); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x11394f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.rf64(0x11394b0u32); cpu.fpush(v); }
  { let v = cpu.st(2) + cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x10a6e74); f_010a462b(cpu); if cpu.returned_to != 0x10a6e74 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x113952eu32); cpu.fpush(v); }
  { let v = cpu.rf80(0x1139524u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x113951au32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x1139510u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(4) * cpu.st(0); cpu.set_st(4, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1139506u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(4) + cpu.st(0); cpu.set_st(4, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.rf80(0x11394fcu32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v = cpu.st(5) * cpu.st(0); cpu.set_st(5, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(5) * cpu.st(0); cpu.set_st(5, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v = cpu.st(2) + cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1139538u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.poll(); pc = cpu.r32(0x11394c0u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x10a6f00 => {
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f0b => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f18 => {
  { let v = cpu.rf80(0x11394e0u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f2b => {
  { let v = cpu.rf80(0x11394e0u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f40 => {
  { let v = cpu.rf80(0x11394eau32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f53 => {
  { let v = cpu.rf80(0x11394eau32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f68 => {
  { let v = cpu.rf80(0x11394eau32); cpu.fpush(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f7b => {
  { let v = cpu.rf80(0x11394eau32); cpu.fpush(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x18u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a6f90 => {
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(1).atan2(cpu.st(0)); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6fa0); }
 let mut pc: u32 = 0x10a6fa0;
 loop { match pc {
 0x10a6fa0 => {
  { let v = 0xb8u32; cpu.push(v); }
  cpu.push(0x10a6faa); f_0108ee90(cpu); if cpu.returned_to != 0x10a6faa { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(164); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a6fc4); f_0109a2af(cpu); if cpu.returned_to != 0x10a6fc4 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a6fcf; continue;
 }
 0x10a6fcf => {
  cpu.push(0x10a6fd4); f_010a4ca4(cpu); if cpu.returned_to != 0x10a6fd4 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10a6fe7; continue; }
  pc = 0x10a6fd9; continue;
 }
 0x10a6fd9 => {
  cpu.push(0x10a6fde); f_010a4ca4(cpu); if cpu.returned_to != 0x10a6fde { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a6fe7; continue;
 }
 0x10a6fe7 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a6ff3; continue;
 }
 0x10a6ff3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a6ffc; continue;
 }
 0x10a6ffc => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a7009; continue;
 }
 0x10a7009 => {
  { let v: u32 = 0x113902cu32; cpu.eax = v; }
  cpu.push(0x10a7013); f_0108cfa7(cpu); if cpu.returned_to != 0x10a7013 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a701d; continue;
 }
 0x10a701d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a7024); f_0108e46c(cpu); if cpu.returned_to != 0x10a7024 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a7033; continue;
 }
 0x10a7033 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a703c; continue;
 }
 0x10a703c => {
  { let v: u32 = 0x3bu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a7048); f_010ada04(cpu); if cpu.returned_to != 0x10a7048 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7058; continue; }
  pc = 0x10a704e; continue;
 }
 0x10a704e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a7055); f_0108e46c(cpu); if cpu.returned_to != 0x10a7055 { return; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x10a7058; continue;
 }
 0x10a7058 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x6eu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x10a707f; continue; }
  pc = 0x10a7064; continue;
 }
 0x10a7064 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x10a706e); f_01099923(cpu); if cpu.returned_to != 0x10a706e { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x10a7078); f_01099946(cpu); if cpu.returned_to != 0x10a7078 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10a70e6; continue;
 }
 0x10a707f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10a708b); f_0108ef05(cpu); if cpu.returned_to != 0x10a708b { return; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x7eu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x10a709e; continue; }
  pc = 0x10a709a; continue;
 }
 0x10a709a => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a709e; continue;
 }
 0x10a709e => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x10a70ab); f_0108ef05(cpu); if cpu.returned_to != 0x10a70ab { return; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10a70b9); f_0109a2af(cpu); if cpu.returned_to != 0x10a70b9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a70c0; continue;
 }
 0x10a70c0 => {
  cpu.push(0x10a70c5); f_010a4ca4(cpu); if cpu.returned_to != 0x10a70c5 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10a70d4; continue; }
  pc = 0x10a70ca; continue;
 }
 0x10a70ca => {
  cpu.push(0x10a70cf); f_010a4ca4(cpu); if cpu.returned_to != 0x10a70cf { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a70d4; continue;
 }
 0x10a70d4 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3bu32); }
  if !cpu.zf { pc = 0x10a70e4; continue; }
  pc = 0x10a70dc; continue;
 }
 0x10a70dc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.poll(); pc = 0x10a7033; continue;
 }
 0x10a70e4 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a70e6; continue;
 }
 0x10a70e6 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a70f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a70f0); }
 let mut pc: u32 = 0x10a70f0;
 loop { match pc {
 0x10a70f0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  if cpu.cf { pc = 0x10a712f; continue; }
  pc = 0x10a7112; continue;
 }
 0x10a7112 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a713c; continue; }
  pc = 0x10a7114; continue;
 }
 0x10a7114 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x10a7125; continue; }
  pc = 0x10a7119; continue;
 }
 0x10a7119 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a7151; continue; }
  pc = 0x10a711b; continue;
 }
 0x10a711b => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x10a713c; continue; }
  pc = 0x10a7120; continue;
 }
 0x10a7120 => {
  pc = 0x10a71ce; continue;
 }
 0x10a7125 => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x10a714c; continue; }
  pc = 0x10a712a; continue;
 }
 0x10a712a => {
  pc = 0x10a71ce; continue;
 }
 0x10a712f => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x10a71ce; continue; }
  pc = 0x10a7138; continue;
 }
 0x10a7138 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a714c; continue; }
  pc = 0x10a713a; continue;
 }
 0x10a713a => {
  pc = 0x10a7151; continue;
 }
 0x10a713c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x10a7149; continue; }
  pc = 0x10a7142; continue;
 }
 0x10a7142 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10a7149; continue;
 }
 0x10a7149 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10a714c; continue;
 }
 0x10a714c => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x10a7151; continue;
 }
 0x10a7151 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a7172; continue; }
  pc = 0x10a716b; continue;
 }
 0x10a716b => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10a7172; continue;
 }
 0x10a7172 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x5fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = 0xffu32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a719a); f_010adb95(cpu); if cpu.returned_to != 0x10a719a { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x10a71a7; continue; }
  pc = 0x10a71a3; continue;
 }
 0x10a71a3 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x10a71a7; continue;
 }
 0x10a71a7 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x10a71ae; continue;
 }
 0x10a71ae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x10a71ba; continue; }
  pc = 0x10a71b7; continue;
 }
 0x10a71b7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a71ae; continue;
 }
 0x10a71ba => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a71c3); f_010a71da(cpu); if cpu.returned_to != 0x10a71c3 { return; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10a71ce; continue;
 }
 0x10a71ce => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a71da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a71da); }
 let mut pc: u32 = 0x10a71da;
 loop { match pc {
 0x10a71da => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x10a7237; continue; }
  pc = 0x10a71eb; continue;
 }
 0x10a71eb => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10a7201; continue; }
  pc = 0x10a71f0; continue;
 }
 0x10a71f0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x10a7201; continue; }
  pc = 0x10a71f9; continue;
 }
 0x10a71f9 => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x10a7201; continue; }
  pc = 0x10a71fe; continue;
 }
 0x10a71fe => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10a71eb; continue;
 }
 0x10a7201 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x10a7217; continue; }
  pc = 0x10a7212; continue;
 }
 0x10a7212 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x10a7223; continue; }
  pc = 0x10a7217; continue;
 }
 0x10a7217 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x10a7223; continue; }
  pc = 0x10a7220; continue;
 }
 0x10a7220 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10a7217; continue;
 }
 0x10a7223 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x10a722d; continue; }
  pc = 0x10a722c; continue;
 }
 0x10a722c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a722d; continue;
 }
 0x10a722d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a722d; continue; }
  pc = 0x10a7237; continue;
 }
 0x10a7237 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7239(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7239); }
 let mut pc: u32 = 0x10a7239;
 loop { match pc {
 0x10a7239 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x10a7243); f_010addcf(cpu); if cpu.returned_to != 0x10a7243 { return; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7248(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7248); } r_010a7248(cpu, 0x10a7248); }
pub fn f_010a724c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a724c); } r_010a7248(cpu, 0x10a724c); }
pub fn f_010a7250(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7250); } r_010a7248(cpu, 0x10a7250); }
pub fn f_010a7252(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7252); } r_010a7248(cpu, 0x10a7252); }
pub fn f_010a7254(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7254); } r_010a7248(cpu, 0x10a7254); }
pub fn f_010a7256(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7256); } r_010a7248(cpu, 0x10a7256); }
pub fn f_010a72a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72a2); } r_010a7248(cpu, 0x10a72a2); }
pub fn f_010a72a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72a4); } r_010a7248(cpu, 0x10a72a4); }
pub fn f_010a72a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72a8); } r_010a7248(cpu, 0x10a72a8); }
pub fn f_010a72aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72aa); } r_010a7248(cpu, 0x10a72aa); }
fn r_010a7248(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a7248 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a724c; continue;
 }
 0x10a724c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a7250; continue;
 }
 0x10a7250 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a7252; continue;
 }
 0x10a7252 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a7254; continue;
 }
 0x10a7254 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_sub((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x46000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10a72b6); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x10a7256 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_sub((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x46000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10a72b6); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x10a72a2 => {
  { cpu.df = false; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10a72b6); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x10a72a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a72a8; continue;
 }
 0x10a72a8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a72aa; continue;
 }
 0x10a72aa => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10a72b6); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a72f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72f2); } r_010a72f2(cpu, 0x10a72f2); }
pub fn f_010a72f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a72f6); } r_010a72f2(cpu, 0x10a72f6); }
pub fn f_010a7300(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7300); } r_010a72f2(cpu, 0x10a7300); }
pub fn f_010a7360(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7360); } r_010a72f2(cpu, 0x10a7360); }
pub fn f_010a73c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a73c0); } r_010a72f2(cpu, 0x10a73c0); }
pub fn f_010a74c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a74c0); } r_010a72f2(cpu, 0x10a74c0); }
fn r_010a72f2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a72f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10a72f6; continue;
 }
 0x10a72f6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let a = cpu.ebx; let b = cpu.ebx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x10a7300; continue;
 }
 0x10a7300 => {
  { let v: u8 = 0x78u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xbe010a78u32; cpu.eax = v; }
  if cpu.sf { pc = 0x10a7315; continue; }
  pc = 0x10a730b; continue;
 }
 0x10a730b => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.eax); cpu.esp = v; }
  if cpu.sf { pc = 0x10a7319; continue; }
  pc = 0x10a730f; continue;
 }
 0x10a730f => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.esi); cpu.w32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x10a7315; continue;
 }
 0x10a7315 => {
  if cpu.sf { pc = 0x10a7321; continue; }
  pc = 0x10a7317; continue;
 }
 0x10a7317 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  pc = 0x10a7319; continue;
 }
 0x10a7319 => {
  if cpu.sf { pc = 0x10a7325; continue; }
  pc = 0x10a731b; continue;
 }
 0x10a731b => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ebx); cpu.esi = v; }
  if cpu.sf { pc = 0x10a7329; continue; }
  pc = 0x10a731f; continue;
 }
 0x10a731f => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.esp); cpu.esp = v; }
  pc = 0x10a7321; continue;
 }
 0x10a7321 => {
  if cpu.sf { pc = 0x10a732d; continue; }
  pc = 0x10a7323; continue;
 }
 0x10a7323 => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.ebp); cpu.esp = v; }
  pc = 0x10a7325; continue;
 }
 0x10a7325 => {
  if cpu.sf { pc = 0x10a7331; continue; }
  pc = 0x10a7327; continue;
 }
 0x10a7327 => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.esi); cpu.esp = v; }
  pc = 0x10a7329; continue;
 }
 0x10a7329 => {
  if cpu.sf { pc = 0x10a7335; continue; }
  pc = 0x10a732b; continue;
 }
 0x10a732b => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.edi); cpu.esp = v; }
  pc = 0x10a732d; continue;
 }
 0x10a732d => {
  if cpu.sf { pc = 0x10a7339; continue; }
  pc = 0x10a732f; continue;
 }
 0x10a732f => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.esi); cpu.w32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x10a7333; continue;
 }
 0x10a7331 => {
  if !cpu.sf { pc = 0x10a733d; continue; }
  pc = 0x10a7333; continue;
 }
 0x10a7333 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a7335; continue;
 }
 0x10a7335 => {
  if !cpu.sf { pc = 0x10a7341; continue; }
  pc = 0x10a7337; continue;
 }
 0x10a7337 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a7339; continue;
 }
 0x10a7339 => {
  if !cpu.sf { pc = 0x10a7345; continue; }
  pc = 0x10a733b; continue;
 }
 0x10a733b => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a733d; continue;
 }
 0x10a733d => {
  if !cpu.sf { pc = 0x10a7349; continue; }
  pc = 0x10a733f; continue;
 }
 0x10a733f => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a7341; continue;
 }
 0x10a7341 => {
  if !cpu.sf { pc = 0x10a734d; continue; }
  pc = 0x10a7343; continue;
 }
 0x10a7343 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a7345; continue;
 }
 0x10a7345 => {
  if !cpu.sf { pc = 0x10a7351; continue; }
  pc = 0x10a7347; continue;
 }
 0x10a7347 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7349; continue;
 }
 0x10a7349 => {
  if !cpu.sf { pc = 0x10a7355; continue; }
  pc = 0x10a734b; continue;
 }
 0x10a734b => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x79u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x10a73cb; continue; }
  pc = 0x10a7352; continue;
 }
 0x10a734d => {
  if !cpu.sf { pc = 0x10a7359; continue; }
  pc = 0x10a734f; continue;
 }
 0x10a734f => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.esi); cpu.w32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x10a7353; continue;
 }
 0x10a7351 => {
  if !cpu.sf { pc = 0x10a735d; continue; }
  pc = 0x10a7353; continue;
 }
 0x10a7352 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a7355; continue;
 }
 0x10a7353 => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.edi)).wrapping_add(cpu.edx); cpu.w32(0x79u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a7358, "Bound");
  pc = 0x10a735b; continue;
 }
 0x10a7355 => {
  if !cpu.sf { pc = 0x10a7361; continue; }
  pc = 0x10a7357; continue;
 }
 0x10a7357 => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esp); cpu.w32(0x79u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xau32.wrapping_add(cpu.ecx)), 0x1u32); cpu.edi = v; }
  pc = 0x10a7360; continue;
 }
 0x10a7359 => {
  if !cpu.sf { pc = 0x10a7365; continue; }
  pc = 0x10a735b; continue;
 }
 0x10a735b => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebp); cpu.w32(0x79u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a7360; continue;
 }
 0x10a735d => {
  if !cpu.sf { pc = 0x10a7369; continue; }
  pc = 0x10a735f; continue;
 }
 0x10a735f => {
  { let v: u32 = cpu.add32(cpu.r32(0xb8010a78u32.wrapping_add(cpu.edx)), cpu.esi); cpu.w32(0xb8010a78u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7365; continue;
 }
 0x10a7360 => {
  { let v: u8 = 0x78u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xbe010a78u32; cpu.eax = v; }
  pc = 0x10a7369; continue;
 }
 0x10a7361 => {
  if cpu.sf { pc = 0x10a736d; continue; }
  pc = 0x10a7363; continue;
 }
 0x10a7363 => {
  { let v: u32 = cpu.add32(cpu.r32(0xbe010a78u32.wrapping_add(cpu.eax)), cpu.edi); cpu.w32(0xbe010a78u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a7369; continue;
 }
 0x10a7365 => {
  if cpu.sf { pc = 0x10a7371; continue; }
  pc = 0x10a7367; continue;
 }
 0x10a7367 => {
  { let v: u32 = cpu.add32(cpu.r32(0xc4010a78u32.wrapping_add(cpu.esi)), cpu.edi); cpu.w32(0xc4010a78u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a736d; continue;
 }
 0x10a7369 => {
  if cpu.sf { pc = 0x10a7375; continue; }
  pc = 0x10a736b; continue;
 }
 0x10a736b => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.eax); cpu.esp = v; }
  pc = 0x10a736d; continue;
 }
 0x10a736d => {
  if cpu.sf { pc = 0x10a7379; continue; }
  pc = 0x10a736f; continue;
 }
 0x10a736f => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  pc = 0x10a7371; continue;
 }
 0x10a7371 => {
  if cpu.sf { pc = 0x10a737d; continue; }
  pc = 0x10a7373; continue;
 }
 0x10a7373 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x10a7375; continue;
 }
 0x10a7375 => {
  if cpu.sf { pc = 0x10a7381; continue; }
  pc = 0x10a7377; continue;
 }
 0x10a7377 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  pc = 0x10a7379; continue;
 }
 0x10a7379 => {
  if cpu.sf { pc = 0x10a7385; continue; }
  pc = 0x10a737b; continue;
 }
 0x10a737b => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ebx); cpu.esi = v; }
  pc = 0x10a737d; continue;
 }
 0x10a737d => {
  if cpu.sf { pc = 0x10a7389; continue; }
  pc = 0x10a737f; continue;
 }
 0x10a737f => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.esp); cpu.esp = v; }
  pc = 0x10a7381; continue;
 }
 0x10a7381 => {
  if cpu.sf { pc = 0x10a738d; continue; }
  pc = 0x10a7383; continue;
 }
 0x10a7383 => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.ebp); cpu.esp = v; }
  pc = 0x10a7385; continue;
 }
 0x10a7385 => {
  if cpu.sf { pc = 0x10a7391; continue; }
  pc = 0x10a7387; continue;
 }
 0x10a7387 => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.esi); cpu.esp = v; }
  pc = 0x10a7389; continue;
 }
 0x10a7389 => {
  if cpu.sf { pc = 0x10a7395; continue; }
  pc = 0x10a738b; continue;
 }
 0x10a738b => {
  { let v: u32 = cpu.add32(cpu.esp, cpu.edi); cpu.esp = v; }
  pc = 0x10a738d; continue;
 }
 0x10a738d => {
  if cpu.sf { pc = 0x10a7399; continue; }
  pc = 0x10a738f; continue;
 }
 0x10a738f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x10a7395; continue;
 }
 0x10a7391 => {
  if !cpu.sf { pc = 0x10a739d; continue; }
  pc = 0x10a7393; continue;
 }
 0x10a7393 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a7395; continue;
 }
 0x10a7395 => {
  if !cpu.sf { pc = 0x10a73a1; continue; }
  pc = 0x10a7397; continue;
 }
 0x10a7397 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a7399; continue;
 }
 0x10a7399 => {
  if !cpu.sf { pc = 0x10a73a5; continue; }
  pc = 0x10a739b; continue;
 }
 0x10a739b => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a739d; continue;
 }
 0x10a739d => {
  if !cpu.sf { pc = 0x10a73a9; continue; }
  pc = 0x10a739f; continue;
 }
 0x10a739f => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a73a1; continue;
 }
 0x10a73a1 => {
  if !cpu.sf { pc = 0x10a73ad; continue; }
  pc = 0x10a73a3; continue;
 }
 0x10a73a3 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a73a5; continue;
 }
 0x10a73a5 => {
  if !cpu.sf { pc = 0x10a73b1; continue; }
  pc = 0x10a73a7; continue;
 }
 0x10a73a7 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a73a9; continue;
 }
 0x10a73a9 => {
  if !cpu.sf { pc = 0x10a73b5; continue; }
  pc = 0x10a73ab; continue;
 }
 0x10a73ab => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x79u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  pc = 0x10a73b1; continue;
 }
 0x10a73ad => {
  if !cpu.sf { pc = 0x10a73b9; continue; }
  pc = 0x10a73af; continue;
 }
 0x10a73af => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ecx); cpu.w32(0xau32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x10a73b3; continue;
 }
 0x10a73b1 => {
  if !cpu.sf { pc = 0x10a73bd; continue; }
  pc = 0x10a73b3; continue;
 }
 0x10a73b3 => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.edi)).wrapping_add(cpu.edx); cpu.w32(0x79u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a73b8, "Bound");
  pc = 0x10a73bb; continue;
 }
 0x10a73b5 => {
  if !cpu.sf { pc = 0x10a73c1; continue; }
  pc = 0x10a73b7; continue;
 }
 0x10a73b7 => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esp); cpu.w32(0x79u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xau32.wrapping_add(cpu.ecx)), 0x1u32); cpu.edi = v; }
  pc = 0x10a73c0; continue;
 }
 0x10a73b9 => {
  if !cpu.sf { pc = 0x10a73c5; continue; }
  pc = 0x10a73bb; continue;
 }
 0x10a73bb => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebp); cpu.w32(0x79u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a73c0; continue;
 }
 0x10a73bd => {
  if !cpu.sf { pc = 0x10a73c9; continue; }
  pc = 0x10a73bf; continue;
 }
 0x10a73bf => {
  { let v: u32 = cpu.add32(cpu.r32(0xa3010a79u32.wrapping_add(cpu.ecx)), cpu.esi); cpu.w32(0xa3010a79u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a73c5; continue;
 }
 0x10a73c0 => {
  { let v: u8 = 0x79u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x7f010a7au32, v); }
  pc = 0x10a73c9; continue;
 }
 0x10a73c1 => {
  if !cpu.sf { pc = 0x10a73cd; continue; }
  pc = 0x10a73c3; continue;
 }
 0x10a73c3 => {
  { let v: u32 = cpu.add32(cpu.r32(0x7f010a7au32.wrapping_add(cpu.ebx)), cpu.esp); cpu.w32(0x7f010a7au32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a73c9; continue;
 }
 0x10a73c5 => {
  if cpu.pf() { pc = 0x10a73d1; continue; }
  pc = 0x10a73c7; continue;
 }
 0x10a73c7 => {
  { let v: u32 = cpu.r32(0x7bu32.wrapping_add(cpu.edi)).wrapping_add(cpu.edi); cpu.w32(0x7bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.of { pc = 0x10a744a; continue; }
  pc = 0x10a73ce; continue;
 }
 0x10a73c9 => {
  if !cpu.pf() { pc = 0x10a73d5; continue; }
  pc = 0x10a73cb; continue;
 }
 0x10a73cb => {
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esi); cpu.w32(0x7cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a73ce; continue;
 }
 0x10a73cd => {
  if (cpu.sf != cpu.of) { pc = 0x10a73d9; continue; }
  pc = 0x10a73cf; continue;
 }
 0x10a73ce => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi | cpu.r32(0xau32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x10a73d3; continue;
 }
 0x10a73cf => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a73d1; continue;
 }
 0x10a73d1 => {
  if (cpu.sf == cpu.of) { pc = 0x10a73dd; continue; }
  pc = 0x10a73d3; continue;
 }
 0x10a73d3 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.esp); cpu.w32(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x10a73d7; continue;
 }
 0x10a73d5 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a73e1; continue; }
  pc = 0x10a73d7; continue;
 }
 0x10a73d7 => {
  { let v: u32 = cpu.r32(0x7fu32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebp); cpu.w32(0x7fu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a73dc, "Insb");
  pc = 0x10a73dd; continue;
 }
 0x10a73d9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a73e5; continue; }
  pc = 0x10a73db; continue;
 }
 0x10a73db => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.ebp); cpu.w32(0xau32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  pc = 0x10a73e1; continue;
 }
 0x10a73dd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.ecx)), 1u32); cpu.w8(0xau32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a73e3; continue;
 }
 0x10a73e1 => {
  if !cpu.sf { pc = 0x10a73ed; continue; }
  pc = 0x10a73e3; continue;
 }
 0x10a73e3 => {
  { let v: u32 = cpu.r32(0x7b9e010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x7b9e010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  pc = 0x10a73ed; continue;
 }
 0x10a73e5 => {
  if !cpu.sf { pc = 0x10a73f1; continue; }
  pc = 0x10a73e7; continue;
 }
 0x10a73e7 => {
  { let v: u32 = cpu.add32(cpu.r32(0xac010a7bu32.wrapping_add(cpu.esi)), cpu.ebx); cpu.w32(0xac010a7bu32.wrapping_add(cpu.esi), v); }
  pc = 0x10a73ed; continue;
 }
 0x10a73ed => {
  if !cpu.sf { pc = 0x10a73f9; continue; }
  pc = 0x10a73ef; continue;
 }
 0x10a73ef => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a73f1; continue;
 }
 0x10a73f1 => {
  if (cpu.sf == cpu.of) { pc = 0x10a73fd; continue; }
  pc = 0x10a73f3; continue;
 }
 0x10a73f3 => {
  { let v: u32 = cpu.r32(0x7f8b010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x7f8b010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  pc = 0x10a73fd; continue;
 }
 0x10a73f9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a7405; continue; }
  pc = 0x10a73fb; continue;
 }
 0x10a73fb => {
  { let v: u32 = cpu.r32(0x79ef010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x79ef010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.fstatus(); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7407; continue;
 }
 0x10a73fd => {
  if !cpu.sf { pc = 0x10a7409; continue; }
  pc = 0x10a73ff; continue;
 }
 0x10a73ff => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  if !cpu.sf { pc = 0x10a740d; continue; }
  pc = 0x10a7403; continue;
 }
 0x10a7403 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  pc = 0x10a7405; continue;
 }
 0x10a7405 => {
  if cpu.pf() { pc = 0x10a7411; continue; }
  pc = 0x10a7407; continue;
 }
 0x10a7407 => {
  { let v: u32 = cpu.add32(cpu.r32(0xab010a7bu32.wrapping_add(cpu.ebp)), cpu.edi); cpu.w32(0xab010a7bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10a740d; continue;
 }
 0x10a7409 => {
  if !cpu.pf() { pc = 0x10a7415; continue; }
  pc = 0x10a740b; continue;
 }
 0x10a740b => {
  { let v: u32 = cpu.add32(cpu.r32(0x51010a7cu32.wrapping_add(cpu.ebx)), cpu.ebp); cpu.w32(0x51010a7cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a7411; continue;
 }
 0x10a740d => {
  if (cpu.sf != cpu.of) { pc = 0x10a7419; continue; }
  pc = 0x10a740f; continue;
 }
 0x10a740f => {
  { let v: u32 = cpu.r32(0x7du32.wrapping_add(cpu.ecx)).wrapping_add(cpu.edx); cpu.w32(0x7du32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xac010a7eu32, v); }
  pc = 0x10a7419; continue;
 }
 0x10a7411 => {
  if (cpu.sf == cpu.of) { pc = 0x10a741d; continue; }
  pc = 0x10a7413; continue;
 }
 0x10a7413 => {
  { let v: u32 = cpu.add32(cpu.r32(0xac010a7eu32.wrapping_add(cpu.edx)), cpu.esp); cpu.w32(0xac010a7eu32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7419; continue;
 }
 0x10a7415 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a7421; continue; }
  pc = 0x10a7417; continue;
 }
 0x10a7417 => {
  { let v: u32 = cpu.r32(0x80a8010au32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x80a8010au32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi | cpu.r32(0xau32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x10a7423; continue;
 }
 0x10a7419 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a7425; continue; }
  pc = 0x10a741b; continue;
 }
 0x10a741b => {
  { let v: u32 = cpu.add32(cpu.r32(0xb010a80u32.wrapping_add(cpu.eax)), cpu.ebp); cpu.w32(0xb010a80u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a7421; continue;
 }
 0x10a741d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi | cpu.r32(0xau32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x10a7423; continue;
 }
 0x10a7421 => {
  if cpu.pf() { pc = 0x10a742d; continue; }
  pc = 0x10a7423; continue;
 }
 0x10a7423 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x10a7425; continue;
 }
 0x10a7425 => {
  if cpu.pf() { pc = 0x10a7431; continue; }
  pc = 0x10a7427; continue;
 }
 0x10a7427 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if !cpu.pf() { pc = 0x10a7435; continue; }
  pc = 0x10a742b; continue;
 }
 0x10a742b => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.esp); cpu.edi = v; }
  pc = 0x10a742d; continue;
 }
 0x10a742d => {
  if (cpu.sf != cpu.of) { pc = 0x10a7439; continue; }
  pc = 0x10a742f; continue;
 }
 0x10a742f => {
  { let v: u32 = cpu.r32(0x7du32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esi); cpu.w32(0x7du32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a7434, "Aad");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(2687); return;
 }
 0x10a7431 => {
  if (cpu.sf == cpu.of) { pc = 0x10a743d; continue; }
  pc = 0x10a7433; continue;
 }
 0x10a7433 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  pc = 0x10a7435; continue;
 }
 0x10a7435 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a7441; continue; }
  pc = 0x10a7437; continue;
 }
 0x10a7437 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  pc = 0x10a7439; continue;
 }
 0x10a7439 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a7445; continue; }
  pc = 0x10a743b; continue;
 }
 0x10a743b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x10a743d; continue;
 }
 0x10a743d => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7440, "Daa");
  pc = 0x10a7441; continue;
 }
 0x10a7441 => {
  if cpu.pf() { pc = 0x10a744d; continue; }
  pc = 0x10a7443; continue;
 }
 0x10a7443 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  pc = 0x10a7445; continue;
 }
 0x10a7445 => {
  if cpu.pf() { pc = 0x10a7451; continue; }
  pc = 0x10a7447; continue;
 }
 0x10a7447 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if !cpu.pf() { pc = 0x10a7455; continue; }
  pc = 0x10a744b; continue;
 }
 0x10a744a => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  pc = 0x10a744d; continue;
 }
 0x10a744b => {
  { let v: u32 = cpu.r32(0x7d91010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x7d91010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0xe8010a7e); return;
 }
 0x10a744d => {
  if !cpu.sf { pc = 0x10a7459; continue; }
  pc = 0x10a744f; continue;
 }
 0x10a744f => {
  { let v: u32 = cpu.add32(cpu.r32(0xea010a7du32.wrapping_add(cpu.ecx)), cpu.edx); cpu.w32(0xea010a7du32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a7455; continue;
 }
 0x10a7451 => {
  if (cpu.sf == cpu.of) { pc = 0x10a745d; continue; }
  pc = 0x10a7453; continue;
 }
 0x10a7453 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebp); cpu.edx = v; }
  pc = 0x10a7455; continue;
 }
 0x10a7455 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a7461; continue; }
  pc = 0x10a7457; continue;
 }
 0x10a7457 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  pc = 0x10a7459; continue;
 }
 0x10a7459 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a7465; continue; }
  pc = 0x10a745b; continue;
 }
 0x10a745b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.esp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10a745d; continue;
 }
 0x10a745d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa7a4601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7463; continue;
 }
 0x10a7461 => {
  if cpu.pf() { pc = 0x10a746d; continue; }
  pc = 0x10a7463; continue;
 }
 0x10a7463 => {
  { let v: u32 = cpu.add32(cpu.r32(0x14010a7bu32), cpu.ebx); cpu.w32(0x14010a7bu32, v); }
  if (cpu.sf != cpu.of) { pc = 0x10a7475; continue; }
  pc = 0x10a746b; continue;
 }
 0x10a7465 => {
  if !cpu.pf() { pc = 0x10a7471; continue; }
  pc = 0x10a7467; continue;
 }
 0x10a7467 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xb4010a7eu32); cpu.eax = v; }
  pc = 0x10a7471; continue;
 }
 0x10a746b => {
  { let v: u32 = cpu.add32(cpu.r32(0xb4010a7eu32), cpu.ebx); cpu.w32(0xb4010a7eu32, v); }
  pc = 0x10a7471; continue;
 }
 0x10a746d => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a7479; continue; }
  pc = 0x10a746f; continue;
 }
 0x10a746f => {
  { let v: u32 = cpu.r32(0x79ac010au32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.esi); cpu.w32(0x79ac010au32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8125010au32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x8125010au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.pf() { pc = 0x10a748d; continue; }
  pc = 0x10a7483; continue;
 }
 0x10a7471 => {
  if (cpu.sf == cpu.of) { pc = 0x10a747d; continue; }
  pc = 0x10a7473; continue;
 }
 0x10a7473 => {
  { let v: u32 = cpu.r32(0x8009010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebp); cpu.w32(0x8009010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x65010a81u32); cpu.eax = v; }
  pc = 0x10a7481; continue;
 }
 0x10a7475 => {
  if !cpu.sf { pc = 0x10a7481; continue; }
  pc = 0x10a7477; continue;
 }
 0x10a7477 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10a7479; continue;
 }
 0x10a7479 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x65010a81u32); cpu.eax = v; }
  pc = 0x10a7481; continue;
 }
 0x10a747d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa7a6501u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7483; continue;
 }
 0x10a7481 => {
  if cpu.pf() { pc = 0x10a748d; continue; }
  pc = 0x10a7483; continue;
 }
 0x10a7483 => {
  { let v: u32 = cpu.r32(0x7bu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.ebx); cpu.w32(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.r32(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.edi = v; }
  { cpu.lods(1, false); }
  pc = 0x10a748d; continue;
 }
 0x10a748d => {
  if !cpu.sf { pc = 0x10a7499; continue; }
  pc = 0x10a748f; continue;
 }
 0x10a748f => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x10a749d; continue; }
  pc = 0x10a7493; continue;
 }
 0x10a7493 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a74a1; continue; }
  pc = 0x10a7497; continue;
 }
 0x10a7497 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a7499; continue;
 }
 0x10a7499 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x10a749d; continue;
 }
 0x10a749d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa7a8401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a74a3; continue;
 }
 0x10a74a1 => {
  if cpu.pf() { pc = 0x10a74ad; continue; }
  pc = 0x10a74a3; continue;
 }
 0x10a74a3 => {
  { let v: u32 = cpu.r32(0x7bu32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esi); cpu.w32(0x7bu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  if (cpu.sf != cpu.of) { pc = 0x10a74b5; continue; }
  pc = 0x10a74ab; continue;
 }
 0x10a74ab => {
  { let v: u32 = cpu.add32(cpu.r32(0xfa010a7eu32.wrapping_add(cpu.edi)), cpu.esi); cpu.w32(0xfa010a7eu32.wrapping_add(cpu.edi), v); }
  if (cpu.sf == cpu.of) { pc = 0x10a74bd; continue; }
  pc = 0x10a74b3; continue;
 }
 0x10a74ad => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a74b9; continue; }
  pc = 0x10a74af; continue;
 }
 0x10a74af => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x10a74bd; continue; }
  pc = 0x10a74b3; continue;
 }
 0x10a74b3 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.ebx); cpu.w32(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x10a74b7; continue;
 }
 0x10a74b5 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a74c1; continue; }
  pc = 0x10a74b7; continue;
 }
 0x10a74b7 => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0xffffff80u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[4])) | 0xa81ba01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[4]), v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.ebx); cpu.w32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa886d01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xeb010a89u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeb010a89u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa82a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa888201u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.esp); cpu.w32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.port_in(0x81u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8188010au32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x81e4010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x8188010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x81f6010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81f601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0xffffff89u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7544, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8813010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xd1010a8au32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0xd1010a8au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d010a82u32).wrapping_add(cpu.ebx); cpu.w32(0x4d010a82u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x8188010au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0xffffff89u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10a7578); return;
 }
 0x10a74b9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[4])) | 0xa81ba01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[4]), v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.ebx); cpu.w32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa886d01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xeb010a89u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeb010a89u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa82a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa888201u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.esp); cpu.w32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.port_in(0x81u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8188010au32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x81e4010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x8188010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x81f6010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81f601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0xffffff89u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7544, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8813010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xd1010a8au32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0xd1010a8au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d010a82u32).wrapping_add(cpu.ebx); cpu.w32(0x4d010a82u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x8188010au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0xffffff89u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10a7578); return;
 }
 0x10a74bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81ba01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.ebx); cpu.w32(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa886d01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xeb010a89u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeb010a89u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa82a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa888201u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.esp); cpu.w32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.port_in(0x81u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8188010au32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x81e4010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x8188010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x81f6010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81f601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0xffffff89u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7544, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8813010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xd1010a8au32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0xd1010a8au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d010a82u32).wrapping_add(cpu.ebx); cpu.w32(0x4d010a82u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x8188010au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0xffffff89u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10a7578); return;
 }
 0x10a74c0 => {
  { let v: u32 = 0x5c010a81u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a74d0, "Insd");
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xeb010a89u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeb010a89u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa82a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa888201u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.esp); cpu.w32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.port_in(0x81u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8188010au32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x81e4010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x8188010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x81f6010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81f601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0xffffff89u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7544, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8813010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xd1010a8au32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0xd1010a8au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d010a82u32).wrapping_add(cpu.ebx); cpu.w32(0x4d010a82u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x8188010au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0xffffff89u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10a7578); return;
 }
 0x10a74c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa825c01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa886d01u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xeb010a89u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeb010a89u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa82a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa888201u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.esp); cpu.w32(0x89d1010au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.port_in(0x81u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8188010au32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x81e4010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x8188010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x81f6010au32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0xa81f601u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0xffffff89u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x10a7544, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8813010au32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xd1010a8au32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0xd1010a8au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d010a82u32).wrapping_add(cpu.ebx); cpu.w32(0x4d010a82u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x8188010au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffff89u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0xffffff89u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10a7578); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a75c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a75c0); }
 let mut pc: u32 = 0x10a75c0;
 loop { match pc {
 0x10a75c0 => {
  { cpu.interrupts_off = false; }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10a75d8); cpu.push(0x10a75dd); f_010a7654(cpu); if cpu.returned_to != 0x10a75dd { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x10a75e0); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7630); }
 let mut pc: u32 = 0x10a7630;
 loop { match pc {
 0x10a7630 => {
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1cu32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  cpu.flat_ds(0x10a7647); cpu.push(0x10a764c); f_010a7654(cpu); if cpu.returned_to != 0x10a764c { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.flat_ds(0x10a7652); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7654(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7654); } r_010a7654(cpu, 0x10a7654); }
pub fn f_010a82d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a82d4); } r_010a7654(cpu, 0x10a82d4); }
pub fn f_010a8355(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a8355); } r_010a7654(cpu, 0x10a8355); }
pub fn f_010a8570(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a8570); } r_010a7654(cpu, 0x10a8570); }
pub fn f_010a867a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a867a); } r_010a7654(cpu, 0x10a867a); }
pub fn f_010a8792(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a8792); } r_010a7654(cpu, 0x10a8792); }
pub fn f_010a881b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a881b); } r_010a7654(cpu, 0x10a881b); }
pub fn f_010a8a24(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a8a24); } r_010a7654(cpu, 0x10a8a24); }
pub fn f_010a8bd9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a8bd9); } r_010a7654(cpu, 0x10a8bd9); }
fn r_010a7654(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a7654 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x10a76e0; continue; }
  pc = 0x10a766e; continue;
 }
 0x10a766e => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xf8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if !cpu.zf { pc = 0x10a7787; continue; }
  pc = 0x10a767c; continue;
 }
 0x10a767c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xc0u8); }
  if !cpu.cf { pc = 0x10a773d; continue; }
  pc = 0x10a768d; continue;
 }
 0x10a768d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x1807u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x10a7300u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.flat_ds(0x10a769e); cpu.push(0x10a76a6); cpu.call(t); if cpu.returned_to != 0x10a76a6 { return; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x38u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10a76b5); cpu.push(0x10a76ba); f_0108ee7c(cpu); if cpu.returned_to != 0x10a76ba { return; }
  { let v: u32 = 0x1134e50u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = cpu.r32(0x10a73c0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a76da => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10a766e; continue; }
  pc = 0x10a76e0; continue;
 }
 0x10a76e0 => {
  if cpu.zf { pc = 0x10a7706; continue; }
  pc = 0x10a76e2; continue;
 }
 0x10a76e2 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x10a7724; continue; }
  pc = 0x10a76e6; continue;
 }
 0x10a76e6 => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x10a7709; continue; }
  pc = 0x10a76ea; continue;
 }
 0x10a76ea => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x10a771b; continue; }
  pc = 0x10a76ee; continue;
 }
 0x10a76ee => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x10a7712; continue; }
  pc = 0x10a76f2; continue;
 }
 0x10a76f2 => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x10a772d; continue; }
  pc = 0x10a76f6; continue;
 }
 0x10a76f6 => {
  { cpu.sub8((cpu.eax as u8), 0x65u8); }
  if cpu.zf { pc = 0x10a7735; continue; }
  pc = 0x10a76fa; continue;
 }
 0x10a76fa => {
  { cpu.sub8((cpu.eax as u8), 0x66u8); }
  if cpu.zf { pc = 0x10a7706; continue; }
  pc = 0x10a76fe; continue;
 }
 0x10a76fe => {
  { cpu.sub8((cpu.eax as u8), 0x67u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a766e; continue; }
  pc = 0x10a7706; continue;
 }
 0x10a7706 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a7709 => {
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a7712 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a771b => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a7724 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a772d => {
  { let v: u16 = (cpu.segments[4] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a7735 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a76da; continue;
 }
 0x10a773d => {
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10a7740); cpu.push(0x10a7745); f_0108ee7c(cpu); if cpu.returned_to != 0x10a7745 { return; }
  { let v: u32 = 0x1134e50u32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x38u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) | (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0xf801u16); }
  if !cpu.zf { pc = 0x10a7770; continue; }
  pc = 0x10a776c; continue;
 }
 0x10a776c => {
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x10a7770; continue;
 }
 0x10a7770 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = cpu.r32(0x10a74c0u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x10a7787 => {
  cpu.flat_ds(0x10a7787); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a77f6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x10a72f2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a780d; continue;
 }
 0x10a780d => {
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7848 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x10a72f2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a79ac => {
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a79b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a79b7); cpu.push(0x10a79bc); f_010a95a2(cpu); if cpu.returned_to != 0x10a79bc { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a79c7); cpu.push(0x10a79cc); f_010a8d78(cpu); if cpu.returned_to != 0x10a79cc { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a79d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a79d6); cpu.push(0x10a79db); f_010a95a2(cpu); if cpu.returned_to != 0x10a79db { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a79e6); cpu.push(0x10a79eb); f_010a91ac(cpu); if cpu.returned_to != 0x10a79eb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a79ef => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a79f5); cpu.push(0x10a79fa); f_010a95a2(cpu); if cpu.returned_to != 0x10a79fa { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a01); cpu.push(0x10a7a06); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7a06 { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a7a0b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a11); cpu.push(0x10a7a16); f_010a95a2(cpu); if cpu.returned_to != 0x10a7a16 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a1d); cpu.push(0x10a7a22); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7a22 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a7a27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a2d); cpu.push(0x10a7a32); f_010a95a2(cpu); if cpu.returned_to != 0x10a7a32 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7a3d); cpu.push(0x10a7a42); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7a42 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7a46 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a4c); cpu.push(0x10a7a51); f_010a95a2(cpu); if cpu.returned_to != 0x10a7a51 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7a5c); cpu.push(0x10a7a61); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7a61 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7a65 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a6b); cpu.push(0x10a7a70); f_010a95a2(cpu); if cpu.returned_to != 0x10a7a70 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7a7b); cpu.push(0x10a7a80); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7a80 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7a84 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7a8a); cpu.push(0x10a7a8f); f_010a95a2(cpu); if cpu.returned_to != 0x10a7a8f { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7a9a); cpu.push(0x10a7a9f); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7a9f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7aa3 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a7ad3); cpu.push(0x10a7ad8); f_010a95a2(cpu); if cpu.returned_to != 0x10a7ad8 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7add => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7ae1); cpu.push(0x10a7ae6); f_010a945c(cpu); if cpu.returned_to != 0x10a7ae6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7aee => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7af2); cpu.push(0x10a7af7); f_010a945c(cpu); if cpu.returned_to != 0x10a7af7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a7aff => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.edx as u16); cpu.set_segment(0, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7b1d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x300u16); }
  if !cpu.zf { pc = 0x10a7b3c; continue; }
  pc = 0x10a7b30; continue;
 }
 0x10a7b30 => {
  { let v: u32 = 0x10a7654u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a7b58; continue;
 }
 0x10a7b3c => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if !cpu.zf { pc = 0x10a7b4e; continue; }
  pc = 0x10a7b42; continue;
 }
 0x10a7b42 => {
  { let v: u32 = 0x10a77c4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a7b58; continue;
 }
 0x10a7b4e => {
  { let v: u32 = 0x10a7788u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10a7b58; continue;
 }
 0x10a7b58 => {
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7b5d => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7b71 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7b7f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7b85); cpu.push(0x10a7b8a); f_010a9396(cpu); if cpu.returned_to != 0x10a7b8a { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7b95); cpu.push(0x10a7b9a); f_010a8d78(cpu); if cpu.returned_to != 0x10a7b9a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7b9e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7ba4); cpu.push(0x10a7ba9); f_010a9396(cpu); if cpu.returned_to != 0x10a7ba9 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7bb4); cpu.push(0x10a7bb9); f_010a91ac(cpu); if cpu.returned_to != 0x10a7bb9 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7bbd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7bc3); cpu.push(0x10a7bc8); f_010a9396(cpu); if cpu.returned_to != 0x10a7bc8 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7bcf); cpu.push(0x10a7bd4); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7bd4 { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a7bd9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7bdf); cpu.push(0x10a7be4); f_010a9396(cpu); if cpu.returned_to != 0x10a7be4 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7beb); cpu.push(0x10a7bf0); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7bf0 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a7bf5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7bfb); cpu.push(0x10a7c00); f_010a9396(cpu); if cpu.returned_to != 0x10a7c00 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7c0b); cpu.push(0x10a7c10); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7c10 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7c14 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7c1a); cpu.push(0x10a7c1f); f_010a9396(cpu); if cpu.returned_to != 0x10a7c1f { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7c2a); cpu.push(0x10a7c2f); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7c2f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7c33 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7c39); cpu.push(0x10a7c3e); f_010a9396(cpu); if cpu.returned_to != 0x10a7c3e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7c49); cpu.push(0x10a7c4e); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7c4e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7c52 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7c58); cpu.push(0x10a7c5d); f_010a9396(cpu); if cpu.returned_to != 0x10a7c5d { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7c68); cpu.push(0x10a7c6d); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7c6d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7c71 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a7ca1); cpu.push(0x10a7ca6); f_010a9396(cpu); if cpu.returned_to != 0x10a7ca6 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7cab => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a7cc5); cpu.push(0x10a7cca); f_010a9667(cpu); if cpu.returned_to != 0x10a7cca { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7cce); cpu.push(0x10a7cd3); f_010a9327(cpu); if cpu.returned_to != 0x10a7cd3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7ce7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a7cf5); cpu.push(0x10a7cfa); f_010a9667(cpu); if cpu.returned_to != 0x10a7cfa { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7cfe); cpu.push(0x10a7d03); f_010a9327(cpu); if cpu.returned_to != 0x10a7d03 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a7d0b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d15); cpu.push(0x10a7d1a); f_010a93dc(cpu); if cpu.returned_to != 0x10a7d1a { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d25); cpu.push(0x10a7d2a); f_010a8d78(cpu); if cpu.returned_to != 0x10a7d2a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7d2e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d38); cpu.push(0x10a7d3d); f_010a93dc(cpu); if cpu.returned_to != 0x10a7d3d { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d48); cpu.push(0x10a7d4d); f_010a91ac(cpu); if cpu.returned_to != 0x10a7d4d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7d51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d5b); cpu.push(0x10a7d60); f_010a93dc(cpu); if cpu.returned_to != 0x10a7d60 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7d67); cpu.push(0x10a7d6c); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7d6c { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a7d71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d7b); cpu.push(0x10a7d80); f_010a93dc(cpu); if cpu.returned_to != 0x10a7d80 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7d87); cpu.push(0x10a7d8c); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7d8c { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a7d91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7d9b); cpu.push(0x10a7da0); f_010a93dc(cpu); if cpu.returned_to != 0x10a7da0 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7dab); cpu.push(0x10a7db0); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7db0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7db4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7dbe); cpu.push(0x10a7dc3); f_010a93dc(cpu); if cpu.returned_to != 0x10a7dc3 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7dce); cpu.push(0x10a7dd3); f_010a8d6a(cpu); if cpu.returned_to != 0x10a7dd3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7dd7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7de1); cpu.push(0x10a7de6); f_010a93dc(cpu); if cpu.returned_to != 0x10a7de6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7df1); cpu.push(0x10a7df6); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7df6 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7dfa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x10a7e04); cpu.push(0x10a7e09); f_010a93dc(cpu); if cpu.returned_to != 0x10a7e09 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7e14); cpu.push(0x10a7e19); f_010a8fd7(cpu); if cpu.returned_to != 0x10a7e19 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7e1d => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7e64 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7e98); cpu.push(0x10a7e9d); f_010a93dc(cpu); if cpu.returned_to != 0x10a7e9d { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7ea2 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7ea6); cpu.push(0x10a7eab); f_010a94db(cpu); if cpu.returned_to != 0x10a7eab { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7eb7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a7ed5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a7ed9); cpu.push(0x10a7ede); f_010a94db(cpu); if cpu.returned_to != 0x10a7ede { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a7eea => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.edx as u16); cpu.set_segment(0, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3800u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 11u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x10a7f18; continue;
 }
 0x10a7f18 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10a7f18; continue; }
  pc = 0x10a7f2a; continue;
 }
 0x10a7f2a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7f31 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x10a7f43; continue;
 }
 0x10a7f43 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10a7f43; continue; }
  pc = 0x10a7f55; continue;
 }
 0x10a7f55 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10a8847; continue;
 }
 0x10a7f5c => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a7f6a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7f72); cpu.push(0x10a7f77); f_010a9396(cpu); if cpu.returned_to != 0x10a7f77 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7f82); cpu.push(0x10a7f87); f_010a8d78(cpu); if cpu.returned_to != 0x10a7f87 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7f8b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7f93); cpu.push(0x10a7f98); f_010a9396(cpu); if cpu.returned_to != 0x10a7f98 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a7fa3); cpu.push(0x10a7fa8); f_010a91ac(cpu); if cpu.returned_to != 0x10a7fa8 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a7fac => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7fb4); cpu.push(0x10a7fb9); f_010a9396(cpu); if cpu.returned_to != 0x10a7fb9 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7fc0); cpu.push(0x10a7fc5); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7fc5 { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a7fca => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7fd2); cpu.push(0x10a7fd7); f_010a9396(cpu); if cpu.returned_to != 0x10a7fd7 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7fde); cpu.push(0x10a7fe3); f_010a8f3e(cpu); if cpu.returned_to != 0x10a7fe3 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a7fe8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a7ff0); cpu.push(0x10a7ff5); f_010a9396(cpu); if cpu.returned_to != 0x10a7ff5 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8000); cpu.push(0x10a8005); f_010a8d6a(cpu); if cpu.returned_to != 0x10a8005 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8009 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a8011); cpu.push(0x10a8016); f_010a9396(cpu); if cpu.returned_to != 0x10a8016 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8021); cpu.push(0x10a8026); f_010a8d6a(cpu); if cpu.returned_to != 0x10a8026 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a802a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a8032); cpu.push(0x10a8037); f_010a9396(cpu); if cpu.returned_to != 0x10a8037 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8042); cpu.push(0x10a8047); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8047 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a804b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a8053); cpu.push(0x10a8058); f_010a9396(cpu); if cpu.returned_to != 0x10a8058 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8063); cpu.push(0x10a8068); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8068 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a806c => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a809e); cpu.push(0x10a80a3); f_010a9396(cpu); if cpu.returned_to != 0x10a80a3 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a80a8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a80c2); cpu.push(0x10a80c7); f_010a9667(cpu); if cpu.returned_to != 0x10a80c7 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a80cb); cpu.push(0x10a80d0); f_010a9327(cpu); if cpu.returned_to != 0x10a80d0 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x10a80db; continue; }
  pc = 0x10a80d7; continue;
 }
 0x10a80d7 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10a80db; continue;
 }
 0x10a80db => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a80f0 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a80fe); cpu.push(0x10a8103); f_010a9667(cpu); if cpu.returned_to != 0x10a8103 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8107); cpu.push(0x10a810c); f_010a9327(cpu); if cpu.returned_to != 0x10a810c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x10a8117; continue; }
  pc = 0x10a8113; continue;
 }
 0x10a8113 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10a8117; continue;
 }
 0x10a8117 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a8120 => {
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8125 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8155); cpu.push(0x10a815a); f_010a93a7(cpu); if cpu.returned_to != 0x10a815a { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a815f => {
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a8164 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a8172); cpu.push(0x10a8177); f_010a9667(cpu); if cpu.returned_to != 0x10a8177 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a817b); cpu.push(0x10a8180); f_010a932c(cpu); if cpu.returned_to != 0x10a8180 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a8188 => {
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a818d => {
  cpu.poll(); pc = cpu.r32(0x10a8195u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a81b5 => {
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a81ba => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a81c6); cpu.push(0x10a81cb); f_010a8d78(cpu); if cpu.returned_to != 0x10a81cb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a81cf => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a81db); cpu.push(0x10a81e0); f_010a91ac(cpu); if cpu.returned_to != 0x10a81e0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a81e4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a81ec); cpu.push(0x10a81f1); f_010a8f3e(cpu); if cpu.returned_to != 0x10a81f1 { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a81f6 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a81fe); cpu.push(0x10a8203); f_010a8f3e(cpu); if cpu.returned_to != 0x10a8203 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a8208 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8214); cpu.push(0x10a8219); f_010a8d6a(cpu); if cpu.returned_to != 0x10a8219 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a821d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8229); cpu.push(0x10a822e); f_010a8d6a(cpu); if cpu.returned_to != 0x10a822e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8232 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a823e); cpu.push(0x10a8243); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8243 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8247 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8253); cpu.push(0x10a8258); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8258 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a825c => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a82a4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a82cc => {
  cpu.poll(); pc = cpu.r32(0x10a82d4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a82d4 => {
  cpu.unimplemented(0x10a82d4, "Hlt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8188010au32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x8188010au32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x830e010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10a82e8, "Aas");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a82f4; continue;
 }
 0x10a82da => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x1u32); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x10a82e8, "Aas");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a82f4; continue;
 }
 0x10a82f4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8301 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a830e => {
  { let v: u32 = cpu.r32(0x10a7248u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10a724cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a7250u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x10a8335); cpu.push(0x10a833a); f_010a8f3e(cpu); if cpu.returned_to != 0x10a833a { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a833f => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8343); cpu.push(0x10a8348); f_010a9aee(cpu); if cpu.returned_to != 0x10a8348 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a834d => {
  cpu.poll(); pc = cpu.r32(0x10a8355u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a8355 => {
  if !cpu.zf { cpu.poll(); pc = 0x10a82da; continue; }
  pc = 0x10a8357; continue;
 }
 0x10a8357 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xfd010a83u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.ecx as u8)); }
  { let v: u32 = cpu.r32(0xcc010a84u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0xcc010a84u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.ecx as u8)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), cpu.ecx); }
  { let v: u32 = cpu.r32(0x2e010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x2e010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8375 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a83b8 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xcd1b8afeu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xd49a784bu32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a83fd => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x5c17f0bcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb8aa3b29u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8442 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xc900daa2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8487 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xfbcff799u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x9a209a84u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffdu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a84cc => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xd1cf79acu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb17217f7u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffeu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8511 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a4u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8568 => {
  cpu.poll(); pc = cpu.r32(0x10a8570u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a8570 => {
  {  }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), cpu.ecx); }
  { let v: u32 = cpu.r32(0xe4010a85u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.ebx); cpu.w32(0xe4010a85u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), cpu.ecx); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0xffffff8cu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cmps(4, 0); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = (cpu.ecx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u8 = a; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.r32(0xffffff86u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.edx); cpu.w32(0xffffff86u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a8590; continue;
 }
 0x10a8590 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8594); cpu.push(0x10a8599); f_010aa114(cpu); if cpu.returned_to != 0x10a8599 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a859d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a85db); cpu.push(0x10a85e0); f_010a9bf5(cpu); if cpu.returned_to != 0x10a85e0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a85e4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a85e8); cpu.push(0x10a85ed); f_010aa0de(cpu); if cpu.returned_to != 0x10a85ed { return; }
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a861f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) & 0x46u8; cpu.w8(0xffffffc7u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x10a862a, "Aas");
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8630 => {
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8651 => {
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8672 => {
  cpu.poll(); pc = cpu.r32(0x10a867au32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a867a => {
  cpu.flat_ds(0x10a867a); cpu.push(cpu.segments[1] as u32); cpu.push(0x10a8681); cpu.call(0xc4010a86); if cpu.returned_to != 0x10a8681 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xffffff87u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esi); cpu.w32(0xffffff87u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10a861f; continue; }
  pc = 0x10a8698; continue;
 }
 0x10a8698 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a869a; continue;
 }
 0x10a869a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a86a2); cpu.push(0x10a86a7); f_010a9a0d(cpu); if cpu.returned_to != 0x10a86a7 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x10a72f6u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a86c4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8702); cpu.push(0x10a8707); f_010a9bdc(cpu); if cpu.returned_to != 0x10a8707 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a870b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a870f); cpu.push(0x10a8714); f_010aa2e8(cpu); if cpu.returned_to != 0x10a8714 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8718 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8728); cpu.push(0x10a872d); f_010aa0c7(cpu); if cpu.returned_to != 0x10a872d { return; }
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8766); cpu.push(0x10a876b); f_010aa0d1(cpu); if cpu.returned_to != 0x10a876b { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8770 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8774); cpu.push(0x10a8779); f_010aa0c7(cpu); if cpu.returned_to != 0x10a8779 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a877d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x10a8781); cpu.push(0x10a8786); f_010aa0d1(cpu); if cpu.returned_to != 0x10a8786 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a878a => {
  cpu.poll(); pc = cpu.r32(0x10a8792u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a8792 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x87b2010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a87b2; continue;
 }
 0x10a87b2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8809); cpu.push(0x10a880e); f_010a8f3e(cpu); if cpu.returned_to != 0x10a880e { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a8813 => {
  cpu.poll(); pc = cpu.r32(0x10a881bu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a881b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x8847010au32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a883b; continue;
 }
 0x10a883b => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8847 => {
  { let v: u16 = 0x33fu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x10a7654u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a886d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8879); cpu.push(0x10a887e); f_010a8d78(cpu); if cpu.returned_to != 0x10a887e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8882 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a888e); cpu.push(0x10a8893); f_010a91ac(cpu); if cpu.returned_to != 0x10a8893 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8897 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a88a3); cpu.push(0x10a88a8); f_010a8d6a(cpu); if cpu.returned_to != 0x10a88a8 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a88ac => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a88b8); cpu.push(0x10a88bd); f_010a8d6a(cpu); if cpu.returned_to != 0x10a88bd { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a88c1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a88cd); cpu.push(0x10a88d2); f_010a8fd7(cpu); if cpu.returned_to != 0x10a88d2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a88d6 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a88e2); cpu.push(0x10a88e7); f_010a8fd7(cpu); if cpu.returned_to != 0x10a88e7 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a88eb => {
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a890a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8936 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a893e); cpu.push(0x10a8943); f_010a8f3e(cpu); if cpu.returned_to != 0x10a8943 { return; }
  cpu.poll(); pc = 0x10a7848; continue;
 }
 0x10a8948 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8950); cpu.push(0x10a8955); f_010a8f3e(cpu); if cpu.returned_to != 0x10a8955 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a895a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a780d; continue;
 }
 0x10a8986 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8992); cpu.push(0x10a8997); f_010a8d78(cpu); if cpu.returned_to != 0x10a8997 { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a89d1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a89dd); cpu.push(0x10a89e2); f_010a91ac(cpu); if cpu.returned_to != 0x10a89e2 { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8a1c => {
  cpu.poll(); pc = cpu.r32(0x10a8a24u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a8a24 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8a44010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8188010au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a8a44; continue;
 }
 0x10a8a44 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8a9b); cpu.push(0x10a8aa0); f_010a8f3e(cpu); if cpu.returned_to != 0x10a8aa0 { return; }
  cpu.poll(); pc = 0x10a77f6; continue;
 }
 0x10a8aa5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8ab1); cpu.push(0x10a8ab6); f_010a8d6a(cpu); if cpu.returned_to != 0x10a8ab6 { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8af0 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8afc); cpu.push(0x10a8b01); f_010a8d6a(cpu); if cpu.returned_to != 0x10a8b01 { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8b3b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8b47); cpu.push(0x10a8b4c); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8b4c { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8b86 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x10a8b92); cpu.push(0x10a8b97); f_010a8fd7(cpu); if cpu.returned_to != 0x10a8b97 { return; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8bd1 => {
  cpu.poll(); pc = cpu.r32(0x10a8bd9u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x10a8bd9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa818801u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x88010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x88010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa818801u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x8d010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), cpu.ecx); cpu.w32(0x8d010a81u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esp; cpu.push(v); }
  cpu.unimplemented(0x10a8bfb, "Das");
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x2eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x10a8c39); cpu.push(0x10a8c3e); f_010a97f4(cpu); if cpu.returned_to != 0x10a8c3e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8bf9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72aau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x10a8c39); cpu.push(0x10a8c3e); f_010a97f4(cpu); if cpu.returned_to != 0x10a8c3e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8c42 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10a7254u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10a7256u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x10a72a2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x8000u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) | 0x3fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x7fffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.flat_ds(0x10a8c9d); cpu.push(0x10a8ca2); f_010a9327(cpu); if cpu.returned_to != 0x10a8ca2 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8ca7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x10a8caf); cpu.push(0x10a8cb4); f_010a9a0d(cpu); if cpu.returned_to != 0x10a8cb4 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x10a72f6u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x10a8cd1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a8cdf); cpu.push(0x10a8ce4); f_010a9667(cpu); if cpu.returned_to != 0x10a8ce4 { return; }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8ce9 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10a7252u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x10a8cfb); cpu.push(0x10a8d00); f_010a9327(cpu); if cpu.returned_to != 0x10a8d00 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x10a8d28; continue; }
  pc = 0x10a8d0d; continue;
 }
 0x10a8d0d => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10a8d19; continue; }
  pc = 0x10a8d11; continue;
 }
 0x10a8d11 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10a8d26; continue;
 }
 0x10a8d19 => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10a8d26; continue;
 }
 0x10a8d26 => {
  pc = 0x10a8d4e; continue;
 }
 0x10a8d28 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10a8d4e; continue; }
  pc = 0x10a8d3e; continue;
 }
 0x10a8d3e => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10a8d4a; continue; }
  pc = 0x10a8d42; continue;
 }
 0x10a8d42 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10a8d4e; continue;
 }
 0x10a8d4a => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10a8d4e; continue;
 }
 0x10a8d4e => {
  { let v: u16 = (cpu.edx as u16) | (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 0x10a8d5a => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10a7654; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7788(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7788); }
 let mut pc: u32 = 0x10a7788;
 loop { match pc {
 0x10a7788 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 25u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x10a77b6; continue; }
  pc = 0x10a7799; continue;
 }
 0x10a7799 => {
  if !cpu.zf { pc = 0x10a77a2; continue; }
  pc = 0x10a779b; continue;
 }
 0x10a779b => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x10a77a2; continue; }
  pc = 0x10a77a0; continue;
 }
 0x10a77a0 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a77a2; continue;
 }
 0x10a77a2 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x100u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x10a77b6; continue; }
  pc = 0x10a77ab; continue;
 }
 0x10a77ab => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a77b6; continue;
 }
 0x10a77b6 => {
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_010a7654(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a77c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a77c4); }
 let mut pc: u32 = 0x10a77c4;
 loop { match pc {
 0x10a77c4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xfffff800u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 22u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x10a77ef; continue; }
  pc = 0x10a77d4; continue;
 }
 0x10a77d4 => {
  if !cpu.zf { pc = 0x10a77d8; continue; }
  pc = 0x10a77d6; continue;
 }
 0x10a77d6 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x10a77d8; continue;
 }
 0x10a77d8 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x800u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x10a77ef; continue; }
  pc = 0x10a77e4; continue;
 }
 0x10a77e4 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a77ef; continue;
 }
 0x10a77ef => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_010a7654(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7864); }
 let mut pc: u32 = 0x10a7864;
 loop { match pc {
 0x10a7864 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10a78b1; continue; }
  pc = 0x10a786e; continue;
 }
 0x10a786e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { cpu.df = false; }
  { cpu.movs(4, true); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x1133e89u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10a788b); cpu.push(0x10a7891); cpu.call(t); if cpu.returned_to != 0x10a7891 { return; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { cpu.df = false; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10a78b1; continue;
 }
 0x10a78b1 => {
  cpu.flat_ds(0x10a78b1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78b2); }
 let mut pc: u32 = 0x10a78b2;
 loop { match pc {
 0x10a78b2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78b8); }
 let mut pc: u32 = 0x10a78b8;
 loop { match pc {
 0x10a78b8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78be); }
 let mut pc: u32 = 0x10a78be;
 loop { match pc {
 0x10a78be => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78c4); }
 let mut pc: u32 = 0x10a78c4;
 loop { match pc {
 0x10a78c4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78ca); }
 let mut pc: u32 = 0x10a78ca;
 loop { match pc {
 0x10a78ca => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78d2); }
 let mut pc: u32 = 0x10a78d2;
 loop { match pc {
 0x10a78d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78d8); }
 let mut pc: u32 = 0x10a78d8;
 loop { match pc {
 0x10a78d8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a78de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a78de); }
 let mut pc: u32 = 0x10a78de;
 loop { match pc {
 0x10a78de => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
