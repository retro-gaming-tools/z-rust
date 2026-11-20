#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0108fb18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fb18); } r_0108fb18(cpu, 0x108fb18); }
pub fn f_0108fc1c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fc1c); } r_0108fb18(cpu, 0x108fc1c); }
pub fn f_0108fe91(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fe91); } r_0108fb18(cpu, 0x108fe91); }
pub fn f_01090372(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090372); } r_0108fb18(cpu, 0x1090372); }
pub fn f_0109045d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109045d); } r_0108fb18(cpu, 0x109045d); }
pub fn f_010906b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10906b2); } r_0108fb18(cpu, 0x10906b2); }
pub fn f_010909fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10909fd); } r_0108fb18(cpu, 0x10909fd); }
pub fn f_01090ae2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090ae2); } r_0108fb18(cpu, 0x1090ae2); }
pub fn f_01090d33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090d33); } r_0108fb18(cpu, 0x1090d33); }
pub fn f_01091802(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1091802); } r_0108fb18(cpu, 0x1091802); }
pub fn f_010918ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10918ab); } r_0108fb18(cpu, 0x10918ab); }
pub fn f_01091bd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1091bd0); } r_0108fb18(cpu, 0x1091bd0); }
pub fn f_01092176(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092176); } r_0108fb18(cpu, 0x1092176); }
pub fn f_01092350(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092350); } r_0108fb18(cpu, 0x1092350); }
pub fn f_01092863(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092863); } r_0108fb18(cpu, 0x1092863); }
pub fn f_01093340(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093340); } r_0108fb18(cpu, 0x1093340); }
pub fn f_01093509(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093509); } r_0108fb18(cpu, 0x1093509); }
pub fn f_01093daf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093daf); } r_0108fb18(cpu, 0x1093daf); }
pub fn f_01093f89(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093f89); } r_0108fb18(cpu, 0x1093f89); }
pub fn f_01094163(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1094163); } r_0108fb18(cpu, 0x1094163); }
pub fn f_01094250(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1094250); } r_0108fb18(cpu, 0x1094250); }
pub fn f_01094522(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1094522); } r_0108fb18(cpu, 0x1094522); }
pub fn f_010946ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10946ff); } r_0108fb18(cpu, 0x10946ff); }
fn r_0108fb18(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108fb18 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fb58; continue; }
  pc = 0x108fb31; continue;
 }
 0x108fb31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108fb3f; continue; }
  pc = 0x108fb36; continue;
 }
 0x108fb36 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fb58; continue; }
  pc = 0x108fb3f; continue;
 }
 0x108fb3f => {
  cpu.push(0x108fb44); f_0109cb12(cpu); if cpu.returned_to != 0x108fb44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fb58; continue; }
  pc = 0x108fb48; continue;
 }
 0x108fb48 => {
  cpu.push(0x108fb4d); f_0108f863(cpu); if cpu.returned_to != 0x108fb4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fb58; continue; }
  pc = 0x108fb51; continue;
 }
 0x108fb51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fb5a; continue;
 }
 0x108fb58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fb5a; continue;
 }
 0x108fb5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fb7b; continue; }
  pc = 0x108fb5e; continue;
 }
 0x108fb5e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137b69u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fb78); f_01095199(cpu); if cpu.returned_to != 0x108fb78 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x108fb7b; continue;
 }
 0x108fb7b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108fb8a); f_0109d194(cpu); if cpu.returned_to != 0x108fb8a { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fb9c; continue;
 }
 0x108fb9c => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x108fbb2; continue; }
  pc = 0x108fba5; continue;
 }
 0x108fba5 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fbb2; continue;
 }
 0x108fbb2 => {
  cpu.push(0x108fbb7); f_0109cb12(cpu); if cpu.returned_to != 0x108fbb7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fbbf; continue;
 }
 0x108fbbf => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x108fbd7; continue;
 }
 0x108fbc3 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fbd3); f_01095199(cpu); if cpu.returned_to != 0x108fbd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108fbd7; continue;
 }
 0x108fbd7 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x108fbc3; continue; }
  pc = 0x108fbdc; continue;
 }
 0x108fbdc => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x108fbf8; continue;
 }
 0x108fbe3 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108fbf4); f_01095199(cpu); if cpu.returned_to != 0x108fbf4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108fbf8; continue;
 }
 0x108fbf8 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x108fbe3; continue; }
  pc = 0x108fc00; continue;
 }
 0x108fc00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  pc = 0x108fc06; continue;
 }
 0x108fc06 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x108ff93; continue;
 }
 0x108fc1c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fc5c; continue; }
  pc = 0x108fc35; continue;
 }
 0x108fc35 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108fc43; continue; }
  pc = 0x108fc3a; continue;
 }
 0x108fc3a => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fc5c; continue; }
  pc = 0x108fc43; continue;
 }
 0x108fc43 => {
  cpu.push(0x108fc48); f_0109cb12(cpu); if cpu.returned_to != 0x108fc48 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fc5c; continue; }
  pc = 0x108fc4c; continue;
 }
 0x108fc4c => {
  cpu.push(0x108fc51); f_0108f863(cpu); if cpu.returned_to != 0x108fc51 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fc5c; continue; }
  pc = 0x108fc55; continue;
 }
 0x108fc55 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fc5e; continue;
 }
 0x108fc5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fc5e; continue;
 }
 0x108fc5e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fc7b; continue; }
  pc = 0x108fc62; continue;
 }
 0x108fc62 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137b91u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108fc78); f_01095199(cpu); if cpu.returned_to != 0x108fc78 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x108fc7b; continue;
 }
 0x108fc7b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fc85); f_0109c68d(cpu); if cpu.returned_to != 0x108fc85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fc97; continue;
 }
 0x108fc97 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x108fcad; continue; }
  pc = 0x108fca0; continue;
 }
 0x108fca0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fcad; continue;
 }
 0x108fcad => {
  cpu.push(0x108fcb2); f_0109cb12(cpu); if cpu.returned_to != 0x108fcb2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108fcba; continue;
 }
 0x108fcba => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x108fcd3; continue;
 }
 0x108fcbe => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108fccf); f_01095199(cpu); if cpu.returned_to != 0x108fccf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108fcd3; continue;
 }
 0x108fcd3 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x108fcbe; continue; }
  pc = 0x108fcd8; continue;
 }
 0x108fcd8 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x108fcf4; continue;
 }
 0x108fcdf => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108fcf0); f_01095199(cpu); if cpu.returned_to != 0x108fcf0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108fcf4; continue;
 }
 0x108fcf4 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x108fcdf; continue; }
  pc = 0x108fcfc; continue;
 }
 0x108fcfc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x108fe91 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fed1; continue; }
  pc = 0x108feaa; continue;
 }
 0x108feaa => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108feb8; continue; }
  pc = 0x108feaf; continue;
 }
 0x108feaf => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fed1; continue; }
  pc = 0x108feb8; continue;
 }
 0x108feb8 => {
  cpu.push(0x108febd); f_0109cb12(cpu); if cpu.returned_to != 0x108febd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fed1; continue; }
  pc = 0x108fec1; continue;
 }
 0x108fec1 => {
  cpu.push(0x108fec6); f_0108f863(cpu); if cpu.returned_to != 0x108fec6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fed1; continue; }
  pc = 0x108feca; continue;
 }
 0x108feca => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fed3; continue;
 }
 0x108fed1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fed3; continue;
 }
 0x108fed3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108feff; continue; }
  pc = 0x108fed7; continue;
 }
 0x108fed7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1137c11u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108fefc); f_01095199(cpu); if cpu.returned_to != 0x108fefc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x108feff; continue;
 }
 0x108feff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108ff18); f_0109c9fb(cpu); if cpu.returned_to != 0x108ff18 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108ff2a; continue;
 }
 0x108ff2a => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x108ff3c; continue; }
  pc = 0x108ff33; continue;
 }
 0x108ff33 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108ff3c; continue;
 }
 0x108ff3c => {
  cpu.push(0x108ff41); f_0109cb12(cpu); if cpu.returned_to != 0x108ff41 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108ff9b; continue; }
  pc = 0x108ff45; continue;
 }
 0x108ff45 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x108ff5e; continue;
 }
 0x108ff49 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108ff5a); f_01095199(cpu); if cpu.returned_to != 0x108ff5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108ff5e; continue;
 }
 0x108ff5e => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x108ff49; continue; }
  pc = 0x108ff63; continue;
 }
 0x108ff63 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x108ff7e; continue;
 }
 0x108ff6a => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108ff7a); f_01095199(cpu); if cpu.returned_to != 0x108ff7a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108ff7e; continue;
 }
 0x108ff7e => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x108ff6a; continue; }
  pc = 0x108ff86; continue;
 }
 0x108ff86 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x108ff93; continue;
 }
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090372 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10903b2; continue; }
  pc = 0x109038b; continue;
 }
 0x109038b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090399; continue; }
  pc = 0x1090390; continue;
 }
 0x1090390 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10903b2; continue; }
  pc = 0x1090399; continue;
 }
 0x1090399 => {
  cpu.push(0x109039e); f_0109cb12(cpu); if cpu.returned_to != 0x109039e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10903b2; continue; }
  pc = 0x10903a2; continue;
 }
 0x10903a2 => {
  cpu.push(0x10903a7); f_0108f863(cpu); if cpu.returned_to != 0x10903a7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10903b2; continue; }
  pc = 0x10903ab; continue;
 }
 0x10903ab => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10903b4; continue;
 }
 0x10903b2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10903b4; continue;
 }
 0x10903b4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10903d1; continue; }
  pc = 0x10903b8; continue;
 }
 0x10903b8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137cbcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10903ce); f_01095199(cpu); if cpu.returned_to != 0x10903ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10903d1; continue;
 }
 0x10903d1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10903db); f_0109cb1e(cpu); if cpu.returned_to != 0x10903db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10903ed; continue;
 }
 0x10903ed => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090403; continue; }
  pc = 0x10903f6; continue;
 }
 0x10903f6 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090403; continue;
 }
 0x1090403 => {
  cpu.push(0x1090408); f_0109cb12(cpu); if cpu.returned_to != 0x1090408 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090410; continue;
 }
 0x1090410 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090429; continue;
 }
 0x1090414 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090425); f_01095199(cpu); if cpu.returned_to != 0x1090425 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090429; continue;
 }
 0x1090429 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090414; continue; }
  pc = 0x109042e; continue;
 }
 0x109042e => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109044a; continue;
 }
 0x1090435 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090446); f_01095199(cpu); if cpu.returned_to != 0x1090446 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109044a; continue;
 }
 0x109044a => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090435; continue; }
  pc = 0x1090452; continue;
 }
 0x1090452 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x109045d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109049d; continue; }
  pc = 0x1090476; continue;
 }
 0x1090476 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090484; continue; }
  pc = 0x109047b; continue;
 }
 0x109047b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109049d; continue; }
  pc = 0x1090484; continue;
 }
 0x1090484 => {
  cpu.push(0x1090489); f_0109cb12(cpu); if cpu.returned_to != 0x1090489 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109049d; continue; }
  pc = 0x109048d; continue;
 }
 0x109048d => {
  cpu.push(0x1090492); f_0108f863(cpu); if cpu.returned_to != 0x1090492 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109049d; continue; }
  pc = 0x1090496; continue;
 }
 0x1090496 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109049f; continue;
 }
 0x109049d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109049f; continue;
 }
 0x109049f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10904c0; continue; }
  pc = 0x10904a3; continue;
 }
 0x10904a3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137cd6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10904bd); f_01095199(cpu); if cpu.returned_to != 0x10904bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10904c0; continue;
 }
 0x10904c0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10904cf); f_0109cb65(cpu); if cpu.returned_to != 0x10904cf { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10904e1; continue;
 }
 0x10904e1 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10904f7; continue; }
  pc = 0x10904ea; continue;
 }
 0x10904ea => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10904f7; continue;
 }
 0x10904f7 => {
  cpu.push(0x10904fc); f_0109cb12(cpu); if cpu.returned_to != 0x10904fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090504; continue;
 }
 0x1090504 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109051c; continue;
 }
 0x1090508 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090518); f_01095199(cpu); if cpu.returned_to != 0x1090518 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109051c; continue;
 }
 0x109051c => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090508; continue; }
  pc = 0x1090521; continue;
 }
 0x1090521 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109053d; continue;
 }
 0x1090528 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090539); f_01095199(cpu); if cpu.returned_to != 0x1090539 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109053d; continue;
 }
 0x109053d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090528; continue; }
  pc = 0x1090545; continue;
 }
 0x1090545 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  cpu.poll(); pc = 0x108fc06; continue;
 }
 0x10906b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10906f2; continue; }
  pc = 0x10906cb; continue;
 }
 0x10906cb => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10906d9; continue; }
  pc = 0x10906d0; continue;
 }
 0x10906d0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10906f2; continue; }
  pc = 0x10906d9; continue;
 }
 0x10906d9 => {
  cpu.push(0x10906de); f_0109cb12(cpu); if cpu.returned_to != 0x10906de { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10906f2; continue; }
  pc = 0x10906e2; continue;
 }
 0x10906e2 => {
  cpu.push(0x10906e7); f_0108f863(cpu); if cpu.returned_to != 0x10906e7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10906f2; continue; }
  pc = 0x10906eb; continue;
 }
 0x10906eb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10906f4; continue;
 }
 0x10906f2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10906f4; continue;
 }
 0x10906f4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109070c; continue; }
  pc = 0x10906f8; continue;
 }
 0x10906f8 => {
  { let v = 0x1137d4cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090709); f_01095199(cpu); if cpu.returned_to != 0x1090709 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x109070c; continue;
 }
 0x109070c => {
  cpu.push(0x1090711); f_0109cd22(cpu); if cpu.returned_to != 0x1090711 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090720; continue;
 }
 0x1090720 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090736; continue; }
  pc = 0x1090729; continue;
 }
 0x1090729 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090736; continue;
 }
 0x1090736 => {
  cpu.push(0x109073b); f_0109cb12(cpu); if cpu.returned_to != 0x109073b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090743; continue;
 }
 0x1090743 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109075c; continue;
 }
 0x1090747 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090758); f_01095199(cpu); if cpu.returned_to != 0x1090758 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109075c; continue;
 }
 0x109075c => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090747; continue; }
  pc = 0x1090761; continue;
 }
 0x1090761 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109077d; continue;
 }
 0x1090768 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090779); f_01095199(cpu); if cpu.returned_to != 0x1090779 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109077d; continue;
 }
 0x109077d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090768; continue; }
  pc = 0x1090785; continue;
 }
 0x1090785 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  pc = 0x1091245; continue;
 }
 0x10909fd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090a3d; continue; }
  pc = 0x1090a16; continue;
 }
 0x1090a16 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090a24; continue; }
  pc = 0x1090a1b; continue;
 }
 0x1090a1b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090a3d; continue; }
  pc = 0x1090a24; continue;
 }
 0x1090a24 => {
  cpu.push(0x1090a29); f_0109cb12(cpu); if cpu.returned_to != 0x1090a29 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090a3d; continue; }
  pc = 0x1090a2d; continue;
 }
 0x1090a2d => {
  cpu.push(0x1090a32); f_0108f863(cpu); if cpu.returned_to != 0x1090a32 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090a3d; continue; }
  pc = 0x1090a36; continue;
 }
 0x1090a36 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090a3f; continue;
 }
 0x1090a3d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090a3f; continue;
 }
 0x1090a3f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090a5c; continue; }
  pc = 0x1090a43; continue;
 }
 0x1090a43 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137df5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090a59); f_01095199(cpu); if cpu.returned_to != 0x1090a59 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090a5c; continue;
 }
 0x1090a5c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090a66); f_0109d47e(cpu); if cpu.returned_to != 0x1090a66 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090a78; continue;
 }
 0x1090a78 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090a8e; continue; }
  pc = 0x1090a81; continue;
 }
 0x1090a81 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090a8e; continue;
 }
 0x1090a8e => {
  cpu.push(0x1090a93); f_0109cb12(cpu); if cpu.returned_to != 0x1090a93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090a9b; continue;
 }
 0x1090a9b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090ab4; continue;
 }
 0x1090a9f => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090ab0); f_01095199(cpu); if cpu.returned_to != 0x1090ab0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090ab4; continue;
 }
 0x1090ab4 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090a9f; continue; }
  pc = 0x1090ab9; continue;
 }
 0x1090ab9 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090ad5; continue;
 }
 0x1090ac0 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090ad1); f_01095199(cpu); if cpu.returned_to != 0x1090ad1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090ad5; continue;
 }
 0x1090ad5 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090ac0; continue; }
  pc = 0x1090add; continue;
 }
 0x1090add => {
  cpu.poll(); pc = 0x108fcfc; continue;
 }
 0x1090ae2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090b22; continue; }
  pc = 0x1090afb; continue;
 }
 0x1090afb => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090b09; continue; }
  pc = 0x1090b00; continue;
 }
 0x1090b00 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090b22; continue; }
  pc = 0x1090b09; continue;
 }
 0x1090b09 => {
  cpu.push(0x1090b0e); f_0109cb12(cpu); if cpu.returned_to != 0x1090b0e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090b22; continue; }
  pc = 0x1090b12; continue;
 }
 0x1090b12 => {
  cpu.push(0x1090b17); f_0108f863(cpu); if cpu.returned_to != 0x1090b17 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090b22; continue; }
  pc = 0x1090b1b; continue;
 }
 0x1090b1b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090b24; continue;
 }
 0x1090b22 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090b24; continue;
 }
 0x1090b24 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090b45; continue; }
  pc = 0x1090b28; continue;
 }
 0x1090b28 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137e13u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090b42); f_01095199(cpu); if cpu.returned_to != 0x1090b42 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1090b45; continue;
 }
 0x1090b45 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090b54); f_0109d1ad(cpu); if cpu.returned_to != 0x1090b54 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090b66; continue;
 }
 0x1090b66 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090b7c; continue; }
  pc = 0x1090b6f; continue;
 }
 0x1090b6f => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090b7c; continue;
 }
 0x1090b7c => {
  cpu.push(0x1090b81); f_0109cb12(cpu); if cpu.returned_to != 0x1090b81 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090b89; continue;
 }
 0x1090b89 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090ba1; continue;
 }
 0x1090b8d => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090b9d); f_01095199(cpu); if cpu.returned_to != 0x1090b9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090ba1; continue;
 }
 0x1090ba1 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090b8d; continue; }
  pc = 0x1090ba6; continue;
 }
 0x1090ba6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090bc2; continue;
 }
 0x1090bad => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090bbe); f_01095199(cpu); if cpu.returned_to != 0x1090bbe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090bc2; continue;
 }
 0x1090bc2 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090bad; continue; }
  pc = 0x1090bca; continue;
 }
 0x1090bca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  cpu.poll(); pc = 0x108fc06; continue;
 }
 0x1090d22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1090d33 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090d73; continue; }
  pc = 0x1090d4c; continue;
 }
 0x1090d4c => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090d5a; continue; }
  pc = 0x1090d51; continue;
 }
 0x1090d51 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090d73; continue; }
  pc = 0x1090d5a; continue;
 }
 0x1090d5a => {
  cpu.push(0x1090d5f); f_0109cb12(cpu); if cpu.returned_to != 0x1090d5f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090d73; continue; }
  pc = 0x1090d63; continue;
 }
 0x1090d63 => {
  cpu.push(0x1090d68); f_0108f863(cpu); if cpu.returned_to != 0x1090d68 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090d73; continue; }
  pc = 0x1090d6c; continue;
 }
 0x1090d6c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090d75; continue;
 }
 0x1090d73 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090d75; continue;
 }
 0x1090d75 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090d96; continue; }
  pc = 0x1090d79; continue;
 }
 0x1090d79 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137e67u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090d93); f_01095199(cpu); if cpu.returned_to != 0x1090d93 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1090d96; continue;
 }
 0x1090d96 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090da5); f_0109eb90(cpu); if cpu.returned_to != 0x1090da5 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090db7; continue;
 }
 0x1090db7 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090dcd; continue; }
  pc = 0x1090dc0; continue;
 }
 0x1090dc0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090dcd; continue;
 }
 0x1090dcd => {
  cpu.push(0x1090dd2); f_0109cb12(cpu); if cpu.returned_to != 0x1090dd2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090dda; continue;
 }
 0x1090dda => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090df2; continue;
 }
 0x1090dde => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090dee); f_01095199(cpu); if cpu.returned_to != 0x1090dee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090df2; continue;
 }
 0x1090df2 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090dde; continue; }
  pc = 0x1090df7; continue;
 }
 0x1090df7 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090e13; continue;
 }
 0x1090dfe => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090e0f); f_01095199(cpu); if cpu.returned_to != 0x1090e0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090e13; continue;
 }
 0x1090e13 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090dfe; continue; }
  pc = 0x1090e1b; continue;
 }
 0x1090e1b => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1091066 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  pc = 0x1091245; continue;
 }
 0x1091245 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1091802 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1091808; continue;
 }
 0x1091808 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091825; continue; }
  pc = 0x109180c; continue;
 }
 0x109180c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138098u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1091822); f_01095199(cpu); if cpu.returned_to != 0x1091822 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1091825; continue;
 }
 0x1091825 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109182f); f_0109eed0(cpu); if cpu.returned_to != 0x109182f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091841; continue;
 }
 0x1091841 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1091857; continue; }
  pc = 0x109184a; continue;
 }
 0x109184a => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091857; continue;
 }
 0x1091857 => {
  cpu.push(0x109185c); f_0109cb12(cpu); if cpu.returned_to != 0x109185c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091864; continue;
 }
 0x1091864 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109187d; continue;
 }
 0x1091868 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1091879); f_01095199(cpu); if cpu.returned_to != 0x1091879 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109187d; continue;
 }
 0x109187d => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1091868; continue; }
  pc = 0x1091882; continue;
 }
 0x1091882 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109189e; continue;
 }
 0x1091889 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109189a); f_01095199(cpu); if cpu.returned_to != 0x109189a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109189e; continue;
 }
 0x109189e => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1091889; continue; }
  pc = 0x10918a6; continue;
 }
 0x10918a6 => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 0x10918ab => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10918eb; continue; }
  pc = 0x10918c4; continue;
 }
 0x10918c4 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10918d2; continue; }
  pc = 0x10918c9; continue;
 }
 0x10918c9 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10918eb; continue; }
  pc = 0x10918d2; continue;
 }
 0x10918d2 => {
  cpu.push(0x10918d7); f_0109cb12(cpu); if cpu.returned_to != 0x10918d7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10918eb; continue; }
  pc = 0x10918db; continue;
 }
 0x10918db => {
  cpu.push(0x10918e0); f_0108f863(cpu); if cpu.returned_to != 0x10918e0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10918eb; continue; }
  pc = 0x10918e4; continue;
 }
 0x10918e4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10918ed; continue;
 }
 0x10918eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10918ed; continue;
 }
 0x10918ed => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109190a; continue; }
  pc = 0x10918f1; continue;
 }
 0x10918f1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11380b8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1091907); f_01095199(cpu); if cpu.returned_to != 0x1091907 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109190a; continue;
 }
 0x109190a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091914); f_0109eef0(cpu); if cpu.returned_to != 0x1091914 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091926; continue;
 }
 0x1091926 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x109193c; continue; }
  pc = 0x109192f; continue;
 }
 0x109192f => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109193c; continue;
 }
 0x109193c => {
  cpu.push(0x1091941); f_0109cb12(cpu); if cpu.returned_to != 0x1091941 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091949; continue;
 }
 0x1091949 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1091962; continue;
 }
 0x109194d => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109195e); f_01095199(cpu); if cpu.returned_to != 0x109195e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091962; continue;
 }
 0x1091962 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x109194d; continue; }
  pc = 0x1091967; continue;
 }
 0x1091967 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1091983; continue;
 }
 0x109196e => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109197f); f_01095199(cpu); if cpu.returned_to != 0x109197f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091983; continue;
 }
 0x1091983 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109196e; continue; }
  pc = 0x109198b; continue;
 }
 0x109198b => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 0x1091bd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091c10; continue; }
  pc = 0x1091be9; continue;
 }
 0x1091be9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091bf7; continue; }
  pc = 0x1091bee; continue;
 }
 0x1091bee => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091c10; continue; }
  pc = 0x1091bf7; continue;
 }
 0x1091bf7 => {
  cpu.push(0x1091bfc); f_0109cb12(cpu); if cpu.returned_to != 0x1091bfc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091c10; continue; }
  pc = 0x1091c00; continue;
 }
 0x1091c00 => {
  cpu.push(0x1091c05); f_0108f863(cpu); if cpu.returned_to != 0x1091c05 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091c10; continue; }
  pc = 0x1091c09; continue;
 }
 0x1091c09 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091c12; continue;
 }
 0x1091c10 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091c12; continue;
 }
 0x1091c12 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091c2f; continue; }
  pc = 0x1091c16; continue;
 }
 0x1091c16 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x113812cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1091c2c); f_01095199(cpu); if cpu.returned_to != 0x1091c2c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1091c2f; continue;
 }
 0x1091c2f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091c39); f_0109f1f0(cpu); if cpu.returned_to != 0x1091c39 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091c4b; continue;
 }
 0x1091c4b => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1091c61; continue; }
  pc = 0x1091c54; continue;
 }
 0x1091c54 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091c61; continue;
 }
 0x1091c61 => {
  cpu.push(0x1091c66); f_0109cb12(cpu); if cpu.returned_to != 0x1091c66 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091c6e; continue;
 }
 0x1091c6e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1091c87; continue;
 }
 0x1091c72 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1091c83); f_01095199(cpu); if cpu.returned_to != 0x1091c83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091c87; continue;
 }
 0x1091c87 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1091c72; continue; }
  pc = 0x1091c8c; continue;
 }
 0x1091c8c => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1091ca8; continue;
 }
 0x1091c93 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1091ca4); f_01095199(cpu); if cpu.returned_to != 0x1091ca4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091ca8; continue;
 }
 0x1091ca8 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1091c93; continue; }
  pc = 0x1091cb0; continue;
 }
 0x1091cb0 => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 0x1092176 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10921b6; continue; }
  pc = 0x109218f; continue;
 }
 0x109218f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109219d; continue; }
  pc = 0x1092194; continue;
 }
 0x1092194 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10921b6; continue; }
  pc = 0x109219d; continue;
 }
 0x109219d => {
  cpu.push(0x10921a2); f_0109cb12(cpu); if cpu.returned_to != 0x10921a2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10921b6; continue; }
  pc = 0x10921a6; continue;
 }
 0x10921a6 => {
  cpu.push(0x10921ab); f_0108f863(cpu); if cpu.returned_to != 0x10921ab { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10921b6; continue; }
  pc = 0x10921af; continue;
 }
 0x10921af => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10921b8; continue;
 }
 0x10921b6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10921b8; continue;
 }
 0x10921b8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10921d9; continue; }
  pc = 0x10921bc; continue;
 }
 0x10921bc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113822au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10921d6); f_01095199(cpu); if cpu.returned_to != 0x10921d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10921d9; continue;
 }
 0x10921d9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10921e8); f_0109f110(cpu); if cpu.returned_to != 0x10921e8 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10921fa; continue;
 }
 0x10921fa => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1092210; continue; }
  pc = 0x1092203; continue;
 }
 0x1092203 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092210; continue;
 }
 0x1092210 => {
  cpu.push(0x1092215); f_0109cb12(cpu); if cpu.returned_to != 0x1092215 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109221d; continue;
 }
 0x109221d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092235; continue;
 }
 0x1092221 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092231); f_01095199(cpu); if cpu.returned_to != 0x1092231 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092235; continue;
 }
 0x1092235 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092221; continue; }
  pc = 0x109223a; continue;
 }
 0x109223a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092256; continue;
 }
 0x1092241 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1092252); f_01095199(cpu); if cpu.returned_to != 0x1092252 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092256; continue;
 }
 0x1092256 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092241; continue; }
  pc = 0x109225e; continue;
 }
 0x109225e => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1092350 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092390; continue; }
  pc = 0x1092369; continue;
 }
 0x1092369 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092377; continue; }
  pc = 0x109236e; continue;
 }
 0x109236e => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092390; continue; }
  pc = 0x1092377; continue;
 }
 0x1092377 => {
  cpu.push(0x109237c); f_0109cb12(cpu); if cpu.returned_to != 0x109237c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092390; continue; }
  pc = 0x1092380; continue;
 }
 0x1092380 => {
  cpu.push(0x1092385); f_0108f863(cpu); if cpu.returned_to != 0x1092385 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092390; continue; }
  pc = 0x1092389; continue;
 }
 0x1092389 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092392; continue;
 }
 0x1092390 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092392; continue;
 }
 0x1092392 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10923b3; continue; }
  pc = 0x1092396; continue;
 }
 0x1092396 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138276u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10923b0); f_01095199(cpu); if cpu.returned_to != 0x10923b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10923b3; continue;
 }
 0x10923b3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10923c2); f_0109f150(cpu); if cpu.returned_to != 0x10923c2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10923d4; continue;
 }
 0x10923d4 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10923ea; continue; }
  pc = 0x10923dd; continue;
 }
 0x10923dd => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10923ea; continue;
 }
 0x10923ea => {
  cpu.push(0x10923ef); f_0109cb12(cpu); if cpu.returned_to != 0x10923ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10923f7; continue;
 }
 0x10923f7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109240f; continue;
 }
 0x10923fb => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109240b); f_01095199(cpu); if cpu.returned_to != 0x109240b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109240f; continue;
 }
 0x109240f => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10923fb; continue; }
  pc = 0x1092414; continue;
 }
 0x1092414 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092430; continue;
 }
 0x109241b => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109242c); f_01095199(cpu); if cpu.returned_to != 0x109242c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092430; continue;
 }
 0x1092430 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109241b; continue; }
  pc = 0x1092438; continue;
 }
 0x1092438 => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1092863 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10928a3; continue; }
  pc = 0x109287c; continue;
 }
 0x109287c => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109288a; continue; }
  pc = 0x1092881; continue;
 }
 0x1092881 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10928a3; continue; }
  pc = 0x109288a; continue;
 }
 0x109288a => {
  cpu.push(0x109288f); f_0109cb12(cpu); if cpu.returned_to != 0x109288f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10928a3; continue; }
  pc = 0x1092893; continue;
 }
 0x1092893 => {
  cpu.push(0x1092898); f_0108f863(cpu); if cpu.returned_to != 0x1092898 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10928a3; continue; }
  pc = 0x109289c; continue;
 }
 0x109289c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10928a5; continue;
 }
 0x10928a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10928a5; continue;
 }
 0x10928a5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10928c6; continue; }
  pc = 0x10928a9; continue;
 }
 0x10928a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138341u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10928c3); f_01095199(cpu); if cpu.returned_to != 0x10928c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10928c6; continue;
 }
 0x10928c6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10928d5); f_010a1c60(cpu); if cpu.returned_to != 0x10928d5 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10928e7; continue;
 }
 0x10928e7 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10928fd; continue; }
  pc = 0x10928f0; continue;
 }
 0x10928f0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10928fd; continue;
 }
 0x10928fd => {
  cpu.push(0x1092902); f_0109cb12(cpu); if cpu.returned_to != 0x1092902 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109290a; continue;
 }
 0x109290a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092922; continue;
 }
 0x109290e => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109291e); f_01095199(cpu); if cpu.returned_to != 0x109291e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092922; continue;
 }
 0x1092922 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x109290e; continue; }
  pc = 0x1092927; continue;
 }
 0x1092927 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092943; continue;
 }
 0x109292e => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109293f); f_01095199(cpu); if cpu.returned_to != 0x109293f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092943; continue;
 }
 0x1092943 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109292e; continue; }
  pc = 0x109294b; continue;
 }
 0x109294b => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1093340 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093380; continue; }
  pc = 0x1093359; continue;
 }
 0x1093359 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093367; continue; }
  pc = 0x109335e; continue;
 }
 0x109335e => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093380; continue; }
  pc = 0x1093367; continue;
 }
 0x1093367 => {
  cpu.push(0x109336c); f_0109cb12(cpu); if cpu.returned_to != 0x109336c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093380; continue; }
  pc = 0x1093370; continue;
 }
 0x1093370 => {
  cpu.push(0x1093375); f_0108f863(cpu); if cpu.returned_to != 0x1093375 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093380; continue; }
  pc = 0x1093379; continue;
 }
 0x1093379 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093382; continue;
 }
 0x1093380 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093382; continue;
 }
 0x1093382 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109339f; continue; }
  pc = 0x1093386; continue;
 }
 0x1093386 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138559u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109339c); f_01095199(cpu); if cpu.returned_to != 0x109339c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109339f; continue;
 }
 0x109339f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10933a9); f_010a26e0(cpu); if cpu.returned_to != 0x10933a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10933bb; continue;
 }
 0x10933bb => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10933d1; continue; }
  pc = 0x10933c4; continue;
 }
 0x10933c4 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10933d1; continue;
 }
 0x10933d1 => {
  cpu.push(0x10933d6); f_0109cb12(cpu); if cpu.returned_to != 0x10933d6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10933de; continue;
 }
 0x10933de => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10933f7; continue;
 }
 0x10933e2 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10933f3); f_01095199(cpu); if cpu.returned_to != 0x10933f3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10933f7; continue;
 }
 0x10933f7 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10933e2; continue; }
  pc = 0x10933fc; continue;
 }
 0x10933fc => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1093418; continue;
 }
 0x1093403 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093414); f_01095199(cpu); if cpu.returned_to != 0x1093414 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093418; continue;
 }
 0x1093418 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1093403; continue; }
  pc = 0x1093420; continue;
 }
 0x1093420 => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 0x10934fc => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109350d); f_01095199(cpu); if cpu.returned_to != 0x109350d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10934fc; continue; }
  pc = 0x1093519; continue;
 }
 0x1093509 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x3b4208c4u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x3b4208c4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x11ec5e4u32); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x10934fc; continue; }
  pc = 0x1093519; continue;
 }
 0x1093519 => {
  cpu.poll(); pc = 0x1091066; continue;
 }
 0x1093daf => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093def; continue; }
  pc = 0x1093dc8; continue;
 }
 0x1093dc8 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093dd6; continue; }
  pc = 0x1093dcd; continue;
 }
 0x1093dcd => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093def; continue; }
  pc = 0x1093dd6; continue;
 }
 0x1093dd6 => {
  cpu.push(0x1093ddb); f_0109cb12(cpu); if cpu.returned_to != 0x1093ddb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093def; continue; }
  pc = 0x1093ddf; continue;
 }
 0x1093ddf => {
  cpu.push(0x1093de4); f_0108f863(cpu); if cpu.returned_to != 0x1093de4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093def; continue; }
  pc = 0x1093de8; continue;
 }
 0x1093de8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093df1; continue;
 }
 0x1093def => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093df1; continue;
 }
 0x1093df1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093e12; continue; }
  pc = 0x1093df5; continue;
 }
 0x1093df5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138712u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093e0f); f_01095199(cpu); if cpu.returned_to != 0x1093e0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1093e12; continue;
 }
 0x1093e12 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093e21); f_010a2a50(cpu); if cpu.returned_to != 0x1093e21 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093e33; continue;
 }
 0x1093e33 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1093e49; continue; }
  pc = 0x1093e3c; continue;
 }
 0x1093e3c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093e49; continue;
 }
 0x1093e49 => {
  cpu.push(0x1093e4e); f_0109cb12(cpu); if cpu.returned_to != 0x1093e4e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093e56; continue;
 }
 0x1093e56 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1093e6e; continue;
 }
 0x1093e5a => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093e6a); f_01095199(cpu); if cpu.returned_to != 0x1093e6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093e6e; continue;
 }
 0x1093e6e => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1093e5a; continue; }
  pc = 0x1093e73; continue;
 }
 0x1093e73 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1093e8f; continue;
 }
 0x1093e7a => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1093e8b); f_01095199(cpu); if cpu.returned_to != 0x1093e8b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093e8f; continue;
 }
 0x1093e8f => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1093e7a; continue; }
  pc = 0x1093e97; continue;
 }
 0x1093e97 => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1093f89 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093fc9; continue; }
  pc = 0x1093fa2; continue;
 }
 0x1093fa2 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093fb0; continue; }
  pc = 0x1093fa7; continue;
 }
 0x1093fa7 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093fc9; continue; }
  pc = 0x1093fb0; continue;
 }
 0x1093fb0 => {
  cpu.push(0x1093fb5); f_0109cb12(cpu); if cpu.returned_to != 0x1093fb5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093fc9; continue; }
  pc = 0x1093fb9; continue;
 }
 0x1093fb9 => {
  cpu.push(0x1093fbe); f_0108f863(cpu); if cpu.returned_to != 0x1093fbe { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093fc9; continue; }
  pc = 0x1093fc2; continue;
 }
 0x1093fc2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093fcb; continue;
 }
 0x1093fc9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093fcb; continue;
 }
 0x1093fcb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093fec; continue; }
  pc = 0x1093fcf; continue;
 }
 0x1093fcf => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138765u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093fe9); f_01095199(cpu); if cpu.returned_to != 0x1093fe9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1093fec; continue;
 }
 0x1093fec => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093ffb); f_010a2a90(cpu); if cpu.returned_to != 0x1093ffb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109400d; continue;
 }
 0x109400d => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1094023; continue; }
  pc = 0x1094016; continue;
 }
 0x1094016 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1094023; continue;
 }
 0x1094023 => {
  cpu.push(0x1094028); f_0109cb12(cpu); if cpu.returned_to != 0x1094028 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1094030; continue;
 }
 0x1094030 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1094048; continue;
 }
 0x1094034 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1094044); f_01095199(cpu); if cpu.returned_to != 0x1094044 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1094048; continue;
 }
 0x1094048 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1094034; continue; }
  pc = 0x109404d; continue;
 }
 0x109404d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1094069; continue;
 }
 0x1094054 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1094065); f_01095199(cpu); if cpu.returned_to != 0x1094065 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1094069; continue;
 }
 0x1094069 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1094054; continue; }
  pc = 0x1094071; continue;
 }
 0x1094071 => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1094163 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10941a3; continue; }
  pc = 0x109417c; continue;
 }
 0x109417c => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109418a; continue; }
  pc = 0x1094181; continue;
 }
 0x1094181 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10941a3; continue; }
  pc = 0x109418a; continue;
 }
 0x109418a => {
  cpu.push(0x109418f); f_0109cb12(cpu); if cpu.returned_to != 0x109418f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10941a3; continue; }
  pc = 0x1094193; continue;
 }
 0x1094193 => {
  cpu.push(0x1094198); f_0108f863(cpu); if cpu.returned_to != 0x1094198 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10941a3; continue; }
  pc = 0x109419c; continue;
 }
 0x109419c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10941a5; continue;
 }
 0x10941a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10941a5; continue;
 }
 0x10941a5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10941c6; continue; }
  pc = 0x10941a9; continue;
 }
 0x10941a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x11387b7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10941c3); f_01095199(cpu); if cpu.returned_to != 0x10941c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10941c6; continue;
 }
 0x10941c6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10941d5); f_010a2ad0(cpu); if cpu.returned_to != 0x10941d5 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10941e7; continue;
 }
 0x10941e7 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10941fd; continue; }
  pc = 0x10941f0; continue;
 }
 0x10941f0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10941fd; continue;
 }
 0x10941fd => {
  cpu.push(0x1094202); f_0109cb12(cpu); if cpu.returned_to != 0x1094202 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109420a; continue;
 }
 0x109420a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1094222; continue;
 }
 0x109420e => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109421e); f_01095199(cpu); if cpu.returned_to != 0x109421e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1094222; continue;
 }
 0x1094222 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x109420e; continue; }
  pc = 0x1094227; continue;
 }
 0x1094227 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1094243; continue;
 }
 0x109422e => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109423f); f_01095199(cpu); if cpu.returned_to != 0x109423f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1094243; continue;
 }
 0x1094243 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109422e; continue; }
  pc = 0x109424b; continue;
 }
 0x109424b => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1094250 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1094290; continue; }
  pc = 0x1094269; continue;
 }
 0x1094269 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1094277; continue; }
  pc = 0x109426e; continue;
 }
 0x109426e => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1094290; continue; }
  pc = 0x1094277; continue;
 }
 0x1094277 => {
  cpu.push(0x109427c); f_0109cb12(cpu); if cpu.returned_to != 0x109427c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1094290; continue; }
  pc = 0x1094280; continue;
 }
 0x1094280 => {
  cpu.push(0x1094285); f_0108f863(cpu); if cpu.returned_to != 0x1094285 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094290; continue; }
  pc = 0x1094289; continue;
 }
 0x1094289 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1094292; continue;
 }
 0x1094290 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1094292; continue;
 }
 0x1094292 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10942b3; continue; }
  pc = 0x1094296; continue;
 }
 0x1094296 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x11387dfu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10942b0); f_01095199(cpu); if cpu.returned_to != 0x10942b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10942b3; continue;
 }
 0x10942b3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10942c2); f_010a2af0(cpu); if cpu.returned_to != 0x10942c2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10942d4; continue;
 }
 0x10942d4 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10942ea; continue; }
  pc = 0x10942dd; continue;
 }
 0x10942dd => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10942ea; continue;
 }
 0x10942ea => {
  cpu.push(0x10942ef); f_0109cb12(cpu); if cpu.returned_to != 0x10942ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10942f7; continue;
 }
 0x10942f7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109430f; continue;
 }
 0x10942fb => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109430b); f_01095199(cpu); if cpu.returned_to != 0x109430b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109430f; continue;
 }
 0x109430f => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10942fb; continue; }
  pc = 0x1094314; continue;
 }
 0x1094314 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1094330; continue;
 }
 0x109431b => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109432c); f_01095199(cpu); if cpu.returned_to != 0x109432c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1094330; continue;
 }
 0x1094330 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109431b; continue; }
  pc = 0x1094338; continue;
 }
 0x1094338 => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 0x1094522 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1094562; continue; }
  pc = 0x109453b; continue;
 }
 0x109453b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1094549; continue; }
  pc = 0x1094540; continue;
 }
 0x1094540 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1094562; continue; }
  pc = 0x1094549; continue;
 }
 0x1094549 => {
  cpu.push(0x109454e); f_0109cb12(cpu); if cpu.returned_to != 0x109454e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1094562; continue; }
  pc = 0x1094552; continue;
 }
 0x1094552 => {
  cpu.push(0x1094557); f_0108f863(cpu); if cpu.returned_to != 0x1094557 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094562; continue; }
  pc = 0x109455b; continue;
 }
 0x109455b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1094564; continue;
 }
 0x1094562 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1094564; continue;
 }
 0x1094564 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094581; continue; }
  pc = 0x1094568; continue;
 }
 0x1094568 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138874u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109457e); f_01095199(cpu); if cpu.returned_to != 0x109457e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1094581; continue;
 }
 0x1094581 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109458b); f_010a2b60(cpu); if cpu.returned_to != 0x109458b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109459d; continue;
 }
 0x109459d => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10945b3; continue; }
  pc = 0x10945a6; continue;
 }
 0x10945a6 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10945b3; continue;
 }
 0x10945b3 => {
  cpu.push(0x10945b8); f_0109cb12(cpu); if cpu.returned_to != 0x10945b8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10945c0; continue;
 }
 0x10945c0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10945d9; continue;
 }
 0x10945c4 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10945d5); f_01095199(cpu); if cpu.returned_to != 0x10945d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10945d9; continue;
 }
 0x10945d9 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10945c4; continue; }
  pc = 0x10945de; continue;
 }
 0x10945de => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10945fa; continue;
 }
 0x10945e5 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10945f6); f_01095199(cpu); if cpu.returned_to != 0x10945f6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10945fa; continue;
 }
 0x10945fa => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10945e5; continue; }
  pc = 0x1094602; continue;
 }
 0x1094602 => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 0x10946ff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109473f; continue; }
  pc = 0x1094718; continue;
 }
 0x1094718 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1094726; continue; }
  pc = 0x109471d; continue;
 }
 0x109471d => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109473f; continue; }
  pc = 0x1094726; continue;
 }
 0x1094726 => {
  cpu.push(0x109472b); f_0109cb12(cpu); if cpu.returned_to != 0x109472b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109473f; continue; }
  pc = 0x109472f; continue;
 }
 0x109472f => {
  cpu.push(0x1094734); f_0108f863(cpu); if cpu.returned_to != 0x1094734 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109473f; continue; }
  pc = 0x1094738; continue;
 }
 0x1094738 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1094741; continue;
 }
 0x109473f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1094741; continue;
 }
 0x1094741 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094762; continue; }
  pc = 0x1094745; continue;
 }
 0x1094745 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x11388d0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109475f); f_01095199(cpu); if cpu.returned_to != 0x109475f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1094762; continue;
 }
 0x1094762 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1094771); f_010a2e70(cpu); if cpu.returned_to != 0x1094771 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1094783; continue;
 }
 0x1094783 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1094799; continue; }
  pc = 0x109478c; continue;
 }
 0x109478c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1094799; continue;
 }
 0x1094799 => {
  cpu.push(0x109479e); f_0109cb12(cpu); if cpu.returned_to != 0x109479e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10947a6; continue;
 }
 0x10947a6 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10947be; continue;
 }
 0x10947aa => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10947ba); f_01095199(cpu); if cpu.returned_to != 0x10947ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10947be; continue;
 }
 0x10947be => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10947aa; continue; }
  pc = 0x10947c3; continue;
 }
 0x10947c3 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10947df; continue;
 }
 0x10947ca => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10947db); f_01095199(cpu); if cpu.returned_to != 0x10947db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10947df; continue;
 }
 0x10947df => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10947ca; continue; }
  pc = 0x10947e7; continue;
 }
 0x10947e7 => {
  cpu.poll(); pc = 0x108fc00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fc12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108fc12); }
 let mut pc: u32 = 0x108fc12;
 loop { match pc {
 0x108fc12 => {
  f_0109c35c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fc17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108fc17); }
 let mut pc: u32 = 0x108fc17;
 loop { match pc {
 0x108fc17 => {
  f_0109c369(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fd07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fd07); } r_0108fd07(cpu, 0x108fd07); }
pub fn f_0108fe0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fe0a); } r_0108fd07(cpu, 0x108fe0a); }
pub fn f_0108ffa7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ffa7); } r_0108fd07(cpu, 0x108ffa7); }
pub fn f_0108ffff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ffff); } r_0108fd07(cpu, 0x108ffff); }
pub fn f_01090000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090000); } r_0108fd07(cpu, 0x1090000); }
pub fn f_01090550(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090550); } r_0108fd07(cpu, 0x1090550); }
pub fn f_010905c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10905c6); } r_0108fd07(cpu, 0x10905c6); }
pub fn f_0109063c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109063c); } r_0108fd07(cpu, 0x109063c); }
pub fn f_01090790(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090790); } r_0108fd07(cpu, 0x1090790); }
pub fn f_0109092e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109092e); } r_0108fd07(cpu, 0x109092e); }
pub fn f_01090bd5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090bd5); } r_0108fd07(cpu, 0x1090bd5); }
pub fn f_01090e20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090e20); } r_0108fd07(cpu, 0x1090e20); }
pub fn f_010913c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10913c2); } r_0108fd07(cpu, 0x10913c2); }
pub fn f_0109201b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109201b); } r_0108fd07(cpu, 0x109201b); }
pub fn f_01092950(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092950); } r_0108fd07(cpu, 0x1092950); }
pub fn f_01092ce2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092ce2); } r_0108fd07(cpu, 0x1092ce2); }
pub fn f_01092ec0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092ec0); } r_0108fd07(cpu, 0x1092ec0); }
pub fn f_01093603(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093603); } r_0108fd07(cpu, 0x1093603); }
pub fn f_01094607(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1094607); } r_0108fd07(cpu, 0x1094607); }
fn r_0108fd07(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108fd07 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fd46; continue; }
  pc = 0x108fd1f; continue;
 }
 0x108fd1f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108fd2d; continue; }
  pc = 0x108fd24; continue;
 }
 0x108fd24 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fd46; continue; }
  pc = 0x108fd2d; continue;
 }
 0x108fd2d => {
  cpu.push(0x108fd32); f_0109cb12(cpu); if cpu.returned_to != 0x108fd32 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fd46; continue; }
  pc = 0x108fd36; continue;
 }
 0x108fd36 => {
  cpu.push(0x108fd3b); f_0108f863(cpu); if cpu.returned_to != 0x108fd3b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fd46; continue; }
  pc = 0x108fd3f; continue;
 }
 0x108fd3f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fd48; continue;
 }
 0x108fd46 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fd48; continue;
 }
 0x108fd48 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fd69; continue; }
  pc = 0x108fd4c; continue;
 }
 0x108fd4c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137bb9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fd66); f_01095199(cpu); if cpu.returned_to != 0x108fd66 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x108fd69; continue;
 }
 0x108fd69 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108fd78); f_0109c6b9(cpu); if cpu.returned_to != 0x108fd78 { return; }
  pc = 0x108fd78; continue;
 }
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe0a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fe49; continue; }
  pc = 0x108fe22; continue;
 }
 0x108fe22 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108fe30; continue; }
  pc = 0x108fe27; continue;
 }
 0x108fe27 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fe49; continue; }
  pc = 0x108fe30; continue;
 }
 0x108fe30 => {
  cpu.push(0x108fe35); f_0109cb12(cpu); if cpu.returned_to != 0x108fe35 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fe49; continue; }
  pc = 0x108fe39; continue;
 }
 0x108fe39 => {
  cpu.push(0x108fe3e); f_0108f863(cpu); if cpu.returned_to != 0x108fe3e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fe49; continue; }
  pc = 0x108fe42; continue;
 }
 0x108fe42 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fe4b; continue;
 }
 0x108fe49 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fe4b; continue;
 }
 0x108fe4b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fe68; continue; }
  pc = 0x108fe4f; continue;
 }
 0x108fe4f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137bf6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108fe65); f_01095199(cpu); if cpu.returned_to != 0x108fe65 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x108fe68; continue;
 }
 0x108fe68 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fe72); f_0109c94b(cpu); if cpu.returned_to != 0x108fe72 { return; }
  pc = 0x108fe72; continue;
 }
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108ffa7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108ffe6; continue; }
  pc = 0x108ffbf; continue;
 }
 0x108ffbf => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108ffcd; continue; }
  pc = 0x108ffc4; continue;
 }
 0x108ffc4 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108ffe6; continue; }
  pc = 0x108ffcd; continue;
 }
 0x108ffcd => {
  cpu.push(0x108ffd2); f_0109cb12(cpu); if cpu.returned_to != 0x108ffd2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108ffe6; continue; }
  pc = 0x108ffd6; continue;
 }
 0x108ffd6 => {
  cpu.push(0x108ffdb); f_0108f863(cpu); if cpu.returned_to != 0x108ffdb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108ffe6; continue; }
  pc = 0x108ffdf; continue;
 }
 0x108ffdf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108ffe8; continue;
 }
 0x108ffe6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108ffe8; continue;
 }
 0x108ffe8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090005; continue; }
  pc = 0x108ffec; continue;
 }
 0x108ffec => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137c37u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090002); f_01095199(cpu); if cpu.returned_to != 0x1090002 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090005; continue;
 }
 0x108ffff => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1090000; continue;
 }
 0x1090000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090005; continue;
 }
 0x1090005 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109000f); f_0109cad5(cpu); if cpu.returned_to != 0x109000f { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1090550 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109058f; continue; }
  pc = 0x1090568; continue;
 }
 0x1090568 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090576; continue; }
  pc = 0x109056d; continue;
 }
 0x109056d => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109058f; continue; }
  pc = 0x1090576; continue;
 }
 0x1090576 => {
  cpu.push(0x109057b); f_0109cb12(cpu); if cpu.returned_to != 0x109057b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109058f; continue; }
  pc = 0x109057f; continue;
 }
 0x109057f => {
  cpu.push(0x1090584); f_0108f863(cpu); if cpu.returned_to != 0x1090584 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109058f; continue; }
  pc = 0x1090588; continue;
 }
 0x1090588 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090591; continue;
 }
 0x109058f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090591; continue;
 }
 0x1090591 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10905b2; continue; }
  pc = 0x1090595; continue;
 }
 0x1090595 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137cf1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10905af); f_01095199(cpu); if cpu.returned_to != 0x10905af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10905b2; continue;
 }
 0x10905b2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10905c1); f_0109cc80(cpu); if cpu.returned_to != 0x10905c1 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 0x10905c6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090605; continue; }
  pc = 0x10905de; continue;
 }
 0x10905de => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10905ec; continue; }
  pc = 0x10905e3; continue;
 }
 0x10905e3 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090605; continue; }
  pc = 0x10905ec; continue;
 }
 0x10905ec => {
  cpu.push(0x10905f1); f_0109cb12(cpu); if cpu.returned_to != 0x10905f1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090605; continue; }
  pc = 0x10905f5; continue;
 }
 0x10905f5 => {
  cpu.push(0x10905fa); f_0108f863(cpu); if cpu.returned_to != 0x10905fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090605; continue; }
  pc = 0x10905fe; continue;
 }
 0x10905fe => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090607; continue;
 }
 0x1090605 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090607; continue;
 }
 0x1090607 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090628; continue; }
  pc = 0x109060b; continue;
 }
 0x109060b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137d0eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090625); f_01095199(cpu); if cpu.returned_to != 0x1090625 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1090628; continue;
 }
 0x1090628 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090637); f_0109ccb0(cpu); if cpu.returned_to != 0x1090637 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 0x109063c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109067b; continue; }
  pc = 0x1090654; continue;
 }
 0x1090654 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090662; continue; }
  pc = 0x1090659; continue;
 }
 0x1090659 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109067b; continue; }
  pc = 0x1090662; continue;
 }
 0x1090662 => {
  cpu.push(0x1090667); f_0109cb12(cpu); if cpu.returned_to != 0x1090667 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109067b; continue; }
  pc = 0x109066b; continue;
 }
 0x109066b => {
  cpu.push(0x1090670); f_0108f863(cpu); if cpu.returned_to != 0x1090670 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109067b; continue; }
  pc = 0x1090674; continue;
 }
 0x1090674 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109067d; continue;
 }
 0x109067b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109067d; continue;
 }
 0x109067d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109069e; continue; }
  pc = 0x1090681; continue;
 }
 0x1090681 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137d2eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109069b); f_01095199(cpu); if cpu.returned_to != 0x109069b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109069e; continue;
 }
 0x109069e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10906ad); f_0109cce0(cpu); if cpu.returned_to != 0x10906ad { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 0x1090790 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10907cf; continue; }
  pc = 0x10907a8; continue;
 }
 0x10907a8 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10907b6; continue; }
  pc = 0x10907ad; continue;
 }
 0x10907ad => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10907cf; continue; }
  pc = 0x10907b6; continue;
 }
 0x10907b6 => {
  cpu.push(0x10907bb); f_0109cb12(cpu); if cpu.returned_to != 0x10907bb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10907cf; continue; }
  pc = 0x10907bf; continue;
 }
 0x10907bf => {
  cpu.push(0x10907c4); f_0108f863(cpu); if cpu.returned_to != 0x10907c4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10907cf; continue; }
  pc = 0x10907c8; continue;
 }
 0x10907c8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10907d1; continue;
 }
 0x10907cf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10907d1; continue;
 }
 0x10907d1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10907ee; continue; }
  pc = 0x10907d5; continue;
 }
 0x10907d5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137d65u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10907eb); f_01095199(cpu); if cpu.returned_to != 0x10907eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10907ee; continue;
 }
 0x10907ee => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10907f8); f_0109cbd8(cpu); if cpu.returned_to != 0x10907f8 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x109092e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109096d; continue; }
  pc = 0x1090946; continue;
 }
 0x1090946 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090954; continue; }
  pc = 0x109094b; continue;
 }
 0x109094b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109096d; continue; }
  pc = 0x1090954; continue;
 }
 0x1090954 => {
  cpu.push(0x1090959); f_0109cb12(cpu); if cpu.returned_to != 0x1090959 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109096d; continue; }
  pc = 0x109095d; continue;
 }
 0x109095d => {
  cpu.push(0x1090962); f_0108f863(cpu); if cpu.returned_to != 0x1090962 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109096d; continue; }
  pc = 0x1090966; continue;
 }
 0x1090966 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109096f; continue;
 }
 0x109096d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109096f; continue;
 }
 0x109096f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109098c; continue; }
  pc = 0x1090973; continue;
 }
 0x1090973 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137dbdu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090989); f_01095199(cpu); if cpu.returned_to != 0x1090989 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109098c; continue;
 }
 0x109098c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090996); f_0109cb8d(cpu); if cpu.returned_to != 0x1090996 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1090bd5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090c14; continue; }
  pc = 0x1090bed; continue;
 }
 0x1090bed => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090bfb; continue; }
  pc = 0x1090bf2; continue;
 }
 0x1090bf2 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090c14; continue; }
  pc = 0x1090bfb; continue;
 }
 0x1090bfb => {
  cpu.push(0x1090c00); f_0109cb12(cpu); if cpu.returned_to != 0x1090c00 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090c14; continue; }
  pc = 0x1090c04; continue;
 }
 0x1090c04 => {
  cpu.push(0x1090c09); f_0108f863(cpu); if cpu.returned_to != 0x1090c09 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090c14; continue; }
  pc = 0x1090c0d; continue;
 }
 0x1090c0d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090c16; continue;
 }
 0x1090c14 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090c16; continue;
 }
 0x1090c16 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090c33; continue; }
  pc = 0x1090c1a; continue;
 }
 0x1090c1a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137e30u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090c30); f_01095199(cpu); if cpu.returned_to != 0x1090c30 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090c33; continue;
 }
 0x1090c33 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090c3d); f_0109d3db(cpu); if cpu.returned_to != 0x1090c3d { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1090e20 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090e5f; continue; }
  pc = 0x1090e38; continue;
 }
 0x1090e38 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090e46; continue; }
  pc = 0x1090e3d; continue;
 }
 0x1090e3d => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090e5f; continue; }
  pc = 0x1090e46; continue;
 }
 0x1090e46 => {
  cpu.push(0x1090e4b); f_0109cb12(cpu); if cpu.returned_to != 0x1090e4b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090e5f; continue; }
  pc = 0x1090e4f; continue;
 }
 0x1090e4f => {
  cpu.push(0x1090e54); f_0108f863(cpu); if cpu.returned_to != 0x1090e54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090e5f; continue; }
  pc = 0x1090e58; continue;
 }
 0x1090e58 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090e61; continue;
 }
 0x1090e5f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090e61; continue;
 }
 0x1090e61 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090e7e; continue; }
  pc = 0x1090e65; continue;
 }
 0x1090e65 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137e8du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090e7b); f_01095199(cpu); if cpu.returned_to != 0x1090e7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090e7e; continue;
 }
 0x1090e7e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090e88); f_0109ecd0(cpu); if cpu.returned_to != 0x1090e88 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x10913c2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091401; continue; }
  pc = 0x10913da; continue;
 }
 0x10913da => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10913e8; continue; }
  pc = 0x10913df; continue;
 }
 0x10913df => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091401; continue; }
  pc = 0x10913e8; continue;
 }
 0x10913e8 => {
  cpu.push(0x10913ed); f_0109cb12(cpu); if cpu.returned_to != 0x10913ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091401; continue; }
  pc = 0x10913f1; continue;
 }
 0x10913f1 => {
  cpu.push(0x10913f6); f_0108f863(cpu); if cpu.returned_to != 0x10913f6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091401; continue; }
  pc = 0x10913fa; continue;
 }
 0x10913fa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091403; continue;
 }
 0x1091401 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091403; continue;
 }
 0x1091403 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091420; continue; }
  pc = 0x1091407; continue;
 }
 0x1091407 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137fb1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109141d); f_01095199(cpu); if cpu.returned_to != 0x109141d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1091420; continue;
 }
 0x1091420 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109142a); f_0109f040(cpu); if cpu.returned_to != 0x109142a { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x109201b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109205a; continue; }
  pc = 0x1092033; continue;
 }
 0x1092033 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092041; continue; }
  pc = 0x1092038; continue;
 }
 0x1092038 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109205a; continue; }
  pc = 0x1092041; continue;
 }
 0x1092041 => {
  cpu.push(0x1092046); f_0109cb12(cpu); if cpu.returned_to != 0x1092046 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109205a; continue; }
  pc = 0x109204a; continue;
 }
 0x109204a => {
  cpu.push(0x109204f); f_0108f863(cpu); if cpu.returned_to != 0x109204f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109205a; continue; }
  pc = 0x1092053; continue;
 }
 0x1092053 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109205c; continue;
 }
 0x109205a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109205c; continue;
 }
 0x109205c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109207d; continue; }
  pc = 0x1092060; continue;
 }
 0x1092060 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x11381edu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109207a); f_01095199(cpu); if cpu.returned_to != 0x109207a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109207d; continue;
 }
 0x109207d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109208c); f_0109ef70(cpu); if cpu.returned_to != 0x109208c { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 0x1092950 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109298f; continue; }
  pc = 0x1092968; continue;
 }
 0x1092968 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092976; continue; }
  pc = 0x109296d; continue;
 }
 0x109296d => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109298f; continue; }
  pc = 0x1092976; continue;
 }
 0x1092976 => {
  cpu.push(0x109297b); f_0109cb12(cpu); if cpu.returned_to != 0x109297b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109298f; continue; }
  pc = 0x109297f; continue;
 }
 0x109297f => {
  cpu.push(0x1092984); f_0108f863(cpu); if cpu.returned_to != 0x1092984 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109298f; continue; }
  pc = 0x1092988; continue;
 }
 0x1092988 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092991; continue;
 }
 0x109298f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092991; continue;
 }
 0x1092991 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10929ae; continue; }
  pc = 0x1092995; continue;
 }
 0x1092995 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138367u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10929ab); f_01095199(cpu); if cpu.returned_to != 0x10929ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10929ae; continue;
 }
 0x10929ae => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10929b8); f_010a1da0(cpu); if cpu.returned_to != 0x10929b8 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1092ce2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092d21; continue; }
  pc = 0x1092cfa; continue;
 }
 0x1092cfa => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092d08; continue; }
  pc = 0x1092cff; continue;
 }
 0x1092cff => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092d21; continue; }
  pc = 0x1092d08; continue;
 }
 0x1092d08 => {
  cpu.push(0x1092d0d); f_0109cb12(cpu); if cpu.returned_to != 0x1092d0d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092d21; continue; }
  pc = 0x1092d11; continue;
 }
 0x1092d11 => {
  cpu.push(0x1092d16); f_0108f863(cpu); if cpu.returned_to != 0x1092d16 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092d21; continue; }
  pc = 0x1092d1a; continue;
 }
 0x1092d1a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092d23; continue;
 }
 0x1092d21 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092d23; continue;
 }
 0x1092d23 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092d40; continue; }
  pc = 0x1092d27; continue;
 }
 0x1092d27 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138422u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092d3d); f_01095199(cpu); if cpu.returned_to != 0x1092d3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092d40; continue;
 }
 0x1092d40 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092d4a); f_010a2470(cpu); if cpu.returned_to != 0x1092d4a { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1092ec0 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092eff; continue; }
  pc = 0x1092ed8; continue;
 }
 0x1092ed8 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092ee6; continue; }
  pc = 0x1092edd; continue;
 }
 0x1092edd => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092eff; continue; }
  pc = 0x1092ee6; continue;
 }
 0x1092ee6 => {
  cpu.push(0x1092eeb); f_0109cb12(cpu); if cpu.returned_to != 0x1092eeb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092eff; continue; }
  pc = 0x1092eef; continue;
 }
 0x1092eef => {
  cpu.push(0x1092ef4); f_0108f863(cpu); if cpu.returned_to != 0x1092ef4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092eff; continue; }
  pc = 0x1092ef8; continue;
 }
 0x1092ef8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092f01; continue;
 }
 0x1092eff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092f01; continue;
 }
 0x1092f01 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092f22; continue; }
  pc = 0x1092f05; continue;
 }
 0x1092f05 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113849eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092f1f); f_01095199(cpu); if cpu.returned_to != 0x1092f1f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1092f22; continue;
 }
 0x1092f22 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1092f31); f_010a2540(cpu); if cpu.returned_to != 0x1092f31 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 0x1093603 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093642; continue; }
  pc = 0x109361b; continue;
 }
 0x109361b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093629; continue; }
  pc = 0x1093620; continue;
 }
 0x1093620 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093642; continue; }
  pc = 0x1093629; continue;
 }
 0x1093629 => {
  cpu.push(0x109362e); f_0109cb12(cpu); if cpu.returned_to != 0x109362e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093642; continue; }
  pc = 0x1093632; continue;
 }
 0x1093632 => {
  cpu.push(0x1093637); f_0108f863(cpu); if cpu.returned_to != 0x1093637 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093642; continue; }
  pc = 0x109363b; continue;
 }
 0x109363b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093644; continue;
 }
 0x1093642 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093644; continue;
 }
 0x1093644 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093661; continue; }
  pc = 0x1093648; continue;
 }
 0x1093648 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11385bfu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109365e); f_01095199(cpu); if cpu.returned_to != 0x109365e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1093661; continue;
 }
 0x1093661 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109366b); f_010a1e50(cpu); if cpu.returned_to != 0x109366b { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 0x1094607 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1094646; continue; }
  pc = 0x109461f; continue;
 }
 0x109461f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109462d; continue; }
  pc = 0x1094624; continue;
 }
 0x1094624 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1094646; continue; }
  pc = 0x109462d; continue;
 }
 0x109462d => {
  cpu.push(0x1094632); f_0109cb12(cpu); if cpu.returned_to != 0x1094632 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1094646; continue; }
  pc = 0x1094636; continue;
 }
 0x1094636 => {
  cpu.push(0x109463b); f_0108f863(cpu); if cpu.returned_to != 0x109463b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094646; continue; }
  pc = 0x109463f; continue;
 }
 0x109463f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1094648; continue;
 }
 0x1094646 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1094648; continue;
 }
 0x1094648 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094669; continue; }
  pc = 0x109464c; continue;
 }
 0x109464c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113888cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1094666); f_01095199(cpu); if cpu.returned_to != 0x1094666 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1094669; continue;
 }
 0x1094669 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1094678); f_010a2d00(cpu); if cpu.returned_to != 0x1094678 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fe7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108fe7e); }
 let mut pc: u32 = 0x108fe7e;
 loop { match pc {
 0x108fe7e => {
  pc = 0x109c990; continue;
 }
 0x109c990 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.interrupts_off = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fe83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108fe83); }
 let mut pc: u32 = 0x108fe83;
 loop { match pc {
 0x108fe83 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108fe8d); f_0109c994(cpu); if cpu.returned_to != 0x108fe8d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01090014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1090014); }
 let mut pc: u32 = 0x1090014;
 loop { match pc {
 0x1090014 => {
  f_0109cb12(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01090019(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090019); } r_01090019(cpu, 0x1090019); }
pub fn f_01090108(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1090108); } r_01090019(cpu, 0x1090108); }
pub fn f_010901a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10901a0); } r_01090019(cpu, 0x10901a0); }
fn r_01090019(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1090019 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x109005d; continue; }
  pc = 0x1090036; continue;
 }
 0x1090036 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090044; continue; }
  pc = 0x109003b; continue;
 }
 0x109003b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x109005d; continue; }
  pc = 0x1090044; continue;
 }
 0x1090044 => {
  cpu.flat_ds(0x1090044); cpu.push(0x1090049); f_0109cb12(cpu); if cpu.returned_to != 0x1090049 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109005d; continue; }
  pc = 0x109004d; continue;
 }
 0x109004d => {
  cpu.flat_ds(0x109004d); cpu.push(0x1090052); f_0108f863(cpu); if cpu.returned_to != 0x1090052 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109005d; continue; }
  pc = 0x1090056; continue;
 }
 0x1090056 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109005f; continue;
 }
 0x109005d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109005f; continue;
 }
 0x109005f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109007c; continue; }
  pc = 0x1090063; continue;
 }
 0x1090063 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137c46u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1090074); cpu.push(0x1090079); f_01095199(cpu); if cpu.returned_to != 0x1090079 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109007c; continue;
 }
 0x109007c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.flat_ds(0x1090082); cpu.push(0x1090087); f_0109cecc(cpu); if cpu.returned_to != 0x1090087 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10902e7; continue; }
  pc = 0x1090094; continue;
 }
 0x1090094 => {
  { cpu.sub32(cpu.r32(0x11ec5d4u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x1090366; continue; }
  pc = 0x10900a1; continue;
 }
 0x10900a1 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x10900b7; continue; }
  pc = 0x10900aa; continue;
 }
 0x10900aa => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x1090366; continue; }
  pc = 0x10900b7; continue;
 }
 0x10900b7 => {
  cpu.flat_ds(0x10900b7); cpu.push(0x10900bc); f_0109cb12(cpu); if cpu.returned_to != 0x10900bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090366; continue; }
  pc = 0x10900c4; continue;
 }
 0x10900c4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10900dc; continue;
 }
 0x10900c8 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10900d3); cpu.push(0x10900d8); f_01095199(cpu); if cpu.returned_to != 0x10900d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10900dc; continue;
 }
 0x10900dc => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10900c8; continue; }
  pc = 0x10900e1; continue;
 }
 0x10900e1 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10900fd; continue;
 }
 0x10900e8 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x10900f4); cpu.push(0x10900f9); f_01095199(cpu); if cpu.returned_to != 0x10900f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10900fd; continue;
 }
 0x10900fd => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x10900e8; continue; }
  pc = 0x1090105; continue;
 }
 0x1090105 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137c61u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x1090118); cpu.push(0x109011d); f_01095199(cpu); if cpu.returned_to != 0x109011d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090138; continue;
 }
 0x1090108 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x68u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.edx as u8)); cpu.w8(0x68u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  if (cpu.sf != cpu.of) { pc = 0x1090123; continue; }
  pc = 0x1090110; continue;
 }
 0x1090110 => {
  { let v: u32 = cpu.r32(0x1ec5d00du32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x1ec5d00du32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.edx); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  if (cpu.sf != cpu.of) { pc = 0x109016b; continue; }
  pc = 0x109011b; continue;
 }
 0x109011b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090138; continue;
 }
 0x1090123 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x68u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x13u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esp); cpu.w32(0x11ec5d0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109012f); cpu.push(0x1090134); f_01095199(cpu); if cpu.returned_to != 0x1090134 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090138; continue;
 }
 0x1090124 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109012f); cpu.push(0x1090134); f_01095199(cpu); if cpu.returned_to != 0x1090134 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090138; continue;
 }
 0x1090138 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090124; continue; }
  pc = 0x109013d; continue;
 }
 0x109013d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090159; continue;
 }
 0x1090144 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x1090150); cpu.push(0x1090155); f_01095199(cpu); if cpu.returned_to != 0x1090155 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090159; continue;
 }
 0x1090159 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x1090144; continue; }
  pc = 0x1090161; continue;
 }
 0x1090161 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137c6eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109016e); cpu.push(0x1090173); f_01095199(cpu); if cpu.returned_to != 0x1090173 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109018e; continue;
 }
 0x109016b => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.edx); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109018e; continue;
 }
 0x109017a => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1090185); cpu.push(0x109018a); f_01095199(cpu); if cpu.returned_to != 0x109018a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109018e; continue;
 }
 0x109018e => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x109017a; continue; }
  pc = 0x1090193; continue;
 }
 0x1090193 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10901af; continue;
 }
 0x109019a => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x10901a6); cpu.push(0x10901ab); f_01095199(cpu); if cpu.returned_to != 0x10901ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10901af; continue;
 }
 0x10901a0 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x10901a6); cpu.push(0x10901ab); f_01095199(cpu); if cpu.returned_to != 0x10901ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10901af; continue;
 }
 0x10901af => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x109019a; continue; }
  pc = 0x10901b7; continue;
 }
 0x10901b7 => {
  { let v: u32 = cpu.r16(0x100u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137c7bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x10901cb); cpu.push(0x10901d0); f_01095199(cpu); if cpu.returned_to != 0x10901d0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10901eb; continue;
 }
 0x10901d7 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10901e2); cpu.push(0x10901e7); f_01095199(cpu); if cpu.returned_to != 0x10901e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10901eb; continue;
 }
 0x10901eb => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10901d7; continue; }
  pc = 0x10901f0; continue;
 }
 0x10901f0 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109020c; continue;
 }
 0x10901f7 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x1090203); cpu.push(0x1090208); f_01095199(cpu); if cpu.returned_to != 0x1090208 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109020c; continue;
 }
 0x109020c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x10901f7; continue; }
  pc = 0x1090214; continue;
 }
 0x1090214 => {
  { let v: u32 = cpu.r16(0x102u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137c88u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x1090228); cpu.push(0x109022d); f_01095199(cpu); if cpu.returned_to != 0x109022d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090248; continue;
 }
 0x1090234 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109023f); cpu.push(0x1090244); f_01095199(cpu); if cpu.returned_to != 0x1090244 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090248; continue;
 }
 0x1090248 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090234; continue; }
  pc = 0x109024d; continue;
 }
 0x109024d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090269; continue;
 }
 0x1090254 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x1090260); cpu.push(0x1090265); f_01095199(cpu); if cpu.returned_to != 0x1090265 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090269; continue;
 }
 0x1090269 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x1090254; continue; }
  pc = 0x1090271; continue;
 }
 0x1090271 => {
  { let v: u32 = cpu.r16(0x104u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137c95u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x1090285); cpu.push(0x109028a); f_01095199(cpu); if cpu.returned_to != 0x109028a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10902a5; continue;
 }
 0x1090291 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109029c); cpu.push(0x10902a1); f_01095199(cpu); if cpu.returned_to != 0x10902a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10902a5; continue;
 }
 0x10902a5 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090291; continue; }
  pc = 0x10902aa; continue;
 }
 0x10902aa => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10902c6; continue;
 }
 0x10902b1 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x10902bd); cpu.push(0x10902c2); f_01095199(cpu); if cpu.returned_to != 0x10902c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10902c6; continue;
 }
 0x10902c6 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x10902b1; continue; }
  pc = 0x10902ce; continue;
 }
 0x10902ce => {
  { let v: u32 = cpu.r16(0x106u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137ca2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x109035e; continue;
 }
 0x10902e7 => {
  { cpu.sub32(cpu.r32(0x11ec5d4u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x1090366; continue; }
  pc = 0x10902f4; continue;
 }
 0x10902f4 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x1090306; continue; }
  pc = 0x10902fd; continue;
 }
 0x10902fd => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x1090366; continue; }
  pc = 0x1090306; continue;
 }
 0x1090306 => {
  cpu.flat_ds(0x1090306); cpu.push(0x109030b); f_0109cb12(cpu); if cpu.returned_to != 0x109030b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090366; continue; }
  pc = 0x109030f; continue;
 }
 0x109030f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090328; continue;
 }
 0x1090313 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x109031f); cpu.push(0x1090324); f_01095199(cpu); if cpu.returned_to != 0x1090324 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090328; continue;
 }
 0x1090328 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090313; continue; }
  pc = 0x109032d; continue;
 }
 0x109032d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090349; continue;
 }
 0x1090334 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x1090340); cpu.push(0x1090345); f_01095199(cpu); if cpu.returned_to != 0x1090345 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090349; continue;
 }
 0x1090349 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.cf { cpu.poll(); pc = 0x1090334; continue; }
  pc = 0x1090351; continue;
 }
 0x1090351 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x109035e; continue;
 }
 0x109035e => {
  cpu.flat_ds(0x109035e); cpu.push(0x1090363); f_01095199(cpu); if cpu.returned_to != 0x1090363 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x1090366; continue;
 }
 0x1090366 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x11ec5e4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x1090371); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01090c42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1090c42); }
 let mut pc: u32 = 0x1090c42;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090c42 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090c82; continue; }
  pc = 0x1090c5b; continue;
 }
 0x1090c5b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090c69; continue; }
  pc = 0x1090c60; continue;
 }
 0x1090c60 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090c82; continue; }
  pc = 0x1090c69; continue;
 }
 0x1090c69 => {
  cpu.push(0x1090c6e); f_0109cb12(cpu); if cpu.returned_to != 0x1090c6e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090c82; continue; }
  pc = 0x1090c72; continue;
 }
 0x1090c72 => {
  cpu.push(0x1090c77); f_0108f863(cpu); if cpu.returned_to != 0x1090c77 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090c82; continue; }
  pc = 0x1090c7b; continue;
 }
 0x1090c7b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090c84; continue;
 }
 0x1090c82 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090c84; continue;
 }
 0x1090c84 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090ca1; continue; }
  pc = 0x1090c88; continue;
 }
 0x1090c88 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137e4cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090c9e); f_01095199(cpu); if cpu.returned_to != 0x1090c9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090ca1; continue;
 }
 0x1090ca1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090cab); f_0109ec50(cpu); if cpu.returned_to != 0x1090cab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090cbd; continue;
 }
 0x1090cbd => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090cd3; continue; }
  pc = 0x1090cc6; continue;
 }
 0x1090cc6 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090cd3; continue;
 }
 0x1090cd3 => {
  cpu.push(0x1090cd8); f_0109cb12(cpu); if cpu.returned_to != 0x1090cd8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090ce0; continue;
 }
 0x1090ce0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090cf9; continue;
 }
 0x1090ce4 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090cf5); f_01095199(cpu); if cpu.returned_to != 0x1090cf5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090cf9; continue;
 }
 0x1090cf9 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090ce4; continue; }
  pc = 0x1090cfe; continue;
 }
 0x1090cfe => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090d1a; continue;
 }
 0x1090d05 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090d16); f_01095199(cpu); if cpu.returned_to != 0x1090d16 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090d1a; continue;
 }
 0x1090d1a => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090d05; continue; }
  pc = 0x1090d22; continue;
 }
 0x1090d22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01090e8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1090e8d); }
 let mut pc: u32 = 0x1090e8d;
 loop { match pc {
 0x108fcfc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1090e8d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1090ecd; continue; }
  pc = 0x1090ea6; continue;
 }
 0x1090ea6 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090eb4; continue; }
  pc = 0x1090eab; continue;
 }
 0x1090eab => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1090ecd; continue; }
  pc = 0x1090eb4; continue;
 }
 0x1090eb4 => {
  cpu.push(0x1090eb9); f_0109cb12(cpu); if cpu.returned_to != 0x1090eb9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1090ecd; continue; }
  pc = 0x1090ebd; continue;
 }
 0x1090ebd => {
  cpu.push(0x1090ec2); f_0108f863(cpu); if cpu.returned_to != 0x1090ec2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090ecd; continue; }
  pc = 0x1090ec6; continue;
 }
 0x1090ec6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1090ecf; continue;
 }
 0x1090ecd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1090ecf; continue;
 }
 0x1090ecf => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090eec; continue; }
  pc = 0x1090ed3; continue;
 }
 0x1090ed3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137eadu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1090ee9); f_01095199(cpu); if cpu.returned_to != 0x1090ee9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1090eec; continue;
 }
 0x1090eec => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1090ef6); f_0109ed10(cpu); if cpu.returned_to != 0x1090ef6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090f08; continue;
 }
 0x1090f08 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1090f1e; continue; }
  pc = 0x1090f11; continue;
 }
 0x1090f11 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090f1e; continue;
 }
 0x1090f1e => {
  cpu.push(0x1090f23); f_0109cb12(cpu); if cpu.returned_to != 0x1090f23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1090f2b; continue;
 }
 0x1090f2b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1090f44; continue;
 }
 0x1090f2f => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090f40); f_01095199(cpu); if cpu.returned_to != 0x1090f40 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090f44; continue;
 }
 0x1090f44 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1090f2f; continue; }
  pc = 0x1090f49; continue;
 }
 0x1090f49 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1090f65; continue;
 }
 0x1090f50 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1090f61); f_01095199(cpu); if cpu.returned_to != 0x1090f61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1090f65; continue;
 }
 0x1090f65 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1090f50; continue; }
  pc = 0x1090f6d; continue;
 }
 0x1090f6d => {
  cpu.poll(); pc = 0x108fcfc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091071(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091071); }
 let mut pc: u32 = 0x1091071;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091071 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10910b0; continue; }
  pc = 0x1091089; continue;
 }
 0x1091089 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091097; continue; }
  pc = 0x109108e; continue;
 }
 0x109108e => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10910b0; continue; }
  pc = 0x1091097; continue;
 }
 0x1091097 => {
  cpu.push(0x109109c); f_0109cb12(cpu); if cpu.returned_to != 0x109109c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10910b0; continue; }
  pc = 0x10910a0; continue;
 }
 0x10910a0 => {
  cpu.push(0x10910a5); f_0108f863(cpu); if cpu.returned_to != 0x10910a5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10910b0; continue; }
  pc = 0x10910a9; continue;
 }
 0x10910a9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10910b2; continue;
 }
 0x10910b0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10910b2; continue;
 }
 0x10910b2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10910cf; continue; }
  pc = 0x10910b6; continue;
 }
 0x10910b6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137ef7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10910cc); f_01095199(cpu); if cpu.returned_to != 0x10910cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10910cf; continue;
 }
 0x10910cf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10910d9); f_0109ed80(cpu); if cpu.returned_to != 0x10910d9 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010910de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10910de); }
 let mut pc: u32 = 0x10910de;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10910de => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109111d; continue; }
  pc = 0x10910f6; continue;
 }
 0x10910f6 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091104; continue; }
  pc = 0x10910fb; continue;
 }
 0x10910fb => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109111d; continue; }
  pc = 0x1091104; continue;
 }
 0x1091104 => {
  cpu.push(0x1091109); f_0109cb12(cpu); if cpu.returned_to != 0x1091109 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109111d; continue; }
  pc = 0x109110d; continue;
 }
 0x109110d => {
  cpu.push(0x1091112); f_0108f863(cpu); if cpu.returned_to != 0x1091112 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109111d; continue; }
  pc = 0x1091116; continue;
 }
 0x1091116 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109111f; continue;
 }
 0x109111d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109111f; continue;
 }
 0x109111f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109113c; continue; }
  pc = 0x1091123; continue;
 }
 0x1091123 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137f18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1091139); f_01095199(cpu); if cpu.returned_to != 0x1091139 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109113c; continue;
 }
 0x109113c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091146); f_0109ed90(cpu); if cpu.returned_to != 0x1091146 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091251(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091251); }
 let mut pc: u32 = 0x1091251;
 loop { match pc {
 0x108fdfd => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091251 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091291; continue; }
  pc = 0x109126a; continue;
 }
 0x109126a => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091278; continue; }
  pc = 0x109126f; continue;
 }
 0x109126f => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091291; continue; }
  pc = 0x1091278; continue;
 }
 0x1091278 => {
  cpu.push(0x109127d); f_0109cb12(cpu); if cpu.returned_to != 0x109127d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091291; continue; }
  pc = 0x1091281; continue;
 }
 0x1091281 => {
  cpu.push(0x1091286); f_0108f863(cpu); if cpu.returned_to != 0x1091286 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091291; continue; }
  pc = 0x109128a; continue;
 }
 0x109128a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091293; continue;
 }
 0x1091291 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091293; continue;
 }
 0x1091293 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10912ba; continue; }
  pc = 0x1091297; continue;
 }
 0x1091297 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137f52u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10912b7); f_01095199(cpu); if cpu.returned_to != 0x10912b7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10912ba; continue;
 }
 0x10912ba => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10912ce); f_0109ee70(cpu); if cpu.returned_to != 0x10912ce { return; }
  cpu.poll(); pc = 0x108fdfd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010912d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10912d3); }
 let mut pc: u32 = 0x10912d3;
 loop { match pc {
 0x108fdfd => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10912d3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091313; continue; }
  pc = 0x10912ec; continue;
 }
 0x10912ec => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10912fa; continue; }
  pc = 0x10912f1; continue;
 }
 0x10912f1 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091313; continue; }
  pc = 0x10912fa; continue;
 }
 0x10912fa => {
  cpu.push(0x10912ff); f_0109cb12(cpu); if cpu.returned_to != 0x10912ff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091313; continue; }
  pc = 0x1091303; continue;
 }
 0x1091303 => {
  cpu.push(0x1091308); f_0108f863(cpu); if cpu.returned_to != 0x1091308 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091313; continue; }
  pc = 0x109130c; continue;
 }
 0x109130c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091315; continue;
 }
 0x1091313 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091315; continue;
 }
 0x1091315 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109133c; continue; }
  pc = 0x1091319; continue;
 }
 0x1091319 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137f78u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1091339); f_01095199(cpu); if cpu.returned_to != 0x1091339 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x109133c; continue;
 }
 0x109133c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1091350); f_0109eea0(cpu); if cpu.returned_to != 0x1091350 { return; }
  cpu.poll(); pc = 0x108fdfd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091355(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091355); }
 let mut pc: u32 = 0x1091355;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091355 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091394; continue; }
  pc = 0x109136d; continue;
 }
 0x109136d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109137b; continue; }
  pc = 0x1091372; continue;
 }
 0x1091372 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091394; continue; }
  pc = 0x109137b; continue;
 }
 0x109137b => {
  cpu.push(0x1091380); f_0109cb12(cpu); if cpu.returned_to != 0x1091380 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091394; continue; }
  pc = 0x1091384; continue;
 }
 0x1091384 => {
  cpu.push(0x1091389); f_0108f863(cpu); if cpu.returned_to != 0x1091389 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091394; continue; }
  pc = 0x109138d; continue;
 }
 0x109138d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091396; continue;
 }
 0x1091394 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091396; continue;
 }
 0x1091396 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10913b3; continue; }
  pc = 0x109139a; continue;
 }
 0x109139a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137f99u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10913b0); f_01095199(cpu); if cpu.returned_to != 0x10913b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10913b3; continue;
 }
 0x10913b3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10913bd); f_0109efc0(cpu); if cpu.returned_to != 0x10913bd { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0109149c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109149c); }
 let mut pc: u32 = 0x109149c;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109149c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10914db; continue; }
  pc = 0x10914b4; continue;
 }
 0x10914b4 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10914c2; continue; }
  pc = 0x10914b9; continue;
 }
 0x10914b9 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10914db; continue; }
  pc = 0x10914c2; continue;
 }
 0x10914c2 => {
  cpu.push(0x10914c7); f_0109cb12(cpu); if cpu.returned_to != 0x10914c7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10914db; continue; }
  pc = 0x10914cb; continue;
 }
 0x10914cb => {
  cpu.push(0x10914d0); f_0108f863(cpu); if cpu.returned_to != 0x10914d0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10914db; continue; }
  pc = 0x10914d4; continue;
 }
 0x10914d4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10914dd; continue;
 }
 0x10914db => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10914dd; continue;
 }
 0x10914dd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10914fa; continue; }
  pc = 0x10914e1; continue;
 }
 0x10914e1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1137fe1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10914f7); f_01095199(cpu); if cpu.returned_to != 0x10914f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10914fa; continue;
 }
 0x10914fa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091504); f_0109f0c0(cpu); if cpu.returned_to != 0x1091504 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091509(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091509); }
 let mut pc: u32 = 0x1091509;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091509 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091548; continue; }
  pc = 0x1091521; continue;
 }
 0x1091521 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109152f; continue; }
  pc = 0x1091526; continue;
 }
 0x1091526 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091548; continue; }
  pc = 0x109152f; continue;
 }
 0x109152f => {
  cpu.push(0x1091534); f_0109cb12(cpu); if cpu.returned_to != 0x1091534 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091548; continue; }
  pc = 0x1091538; continue;
 }
 0x1091538 => {
  cpu.push(0x109153d); f_0108f863(cpu); if cpu.returned_to != 0x109153d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091548; continue; }
  pc = 0x1091541; continue;
 }
 0x1091541 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109154a; continue;
 }
 0x1091548 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109154a; continue;
 }
 0x109154a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109156b; continue; }
  pc = 0x109154e; continue;
 }
 0x109154e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1137ff7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091568); f_01095199(cpu); if cpu.returned_to != 0x1091568 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109156b; continue;
 }
 0x109156b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109157a); f_0109eee0(cpu); if cpu.returned_to != 0x109157a { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0109157f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109157f); }
 let mut pc: u32 = 0x109157f;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109157f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10915be; continue; }
  pc = 0x1091597; continue;
 }
 0x1091597 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10915a5; continue; }
  pc = 0x109159c; continue;
 }
 0x109159c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10915be; continue; }
  pc = 0x10915a5; continue;
 }
 0x10915a5 => {
  cpu.push(0x10915aa); f_0109cb12(cpu); if cpu.returned_to != 0x10915aa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10915be; continue; }
  pc = 0x10915ae; continue;
 }
 0x10915ae => {
  cpu.push(0x10915b3); f_0108f863(cpu); if cpu.returned_to != 0x10915b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10915be; continue; }
  pc = 0x10915b7; continue;
 }
 0x10915b7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10915c0; continue;
 }
 0x10915be => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10915c0; continue;
 }
 0x10915c0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10915e1; continue; }
  pc = 0x10915c4; continue;
 }
 0x10915c4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113801eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10915de); f_01095199(cpu); if cpu.returned_to != 0x10915de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10915e1; continue;
 }
 0x10915e1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10915f0); f_0109ef00(cpu); if cpu.returned_to != 0x10915f0 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010915f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10915f5); }
 let mut pc: u32 = 0x10915f5;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10915f5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091634; continue; }
  pc = 0x109160d; continue;
 }
 0x109160d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109161b; continue; }
  pc = 0x1091612; continue;
 }
 0x1091612 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091634; continue; }
  pc = 0x109161b; continue;
 }
 0x109161b => {
  cpu.push(0x1091620); f_0109cb12(cpu); if cpu.returned_to != 0x1091620 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091634; continue; }
  pc = 0x1091624; continue;
 }
 0x1091624 => {
  cpu.push(0x1091629); f_0108f863(cpu); if cpu.returned_to != 0x1091629 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091634; continue; }
  pc = 0x109162d; continue;
 }
 0x109162d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091636; continue;
 }
 0x1091634 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091636; continue;
 }
 0x1091636 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091657; continue; }
  pc = 0x109163a; continue;
 }
 0x109163a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113803eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091654); f_01095199(cpu); if cpu.returned_to != 0x1091654 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1091657; continue;
 }
 0x1091657 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1091666); f_0109ef30(cpu); if cpu.returned_to != 0x1091666 { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0109166b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109166b); }
 let mut pc: u32 = 0x109166b;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109166b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10916aa; continue; }
  pc = 0x1091683; continue;
 }
 0x1091683 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091691; continue; }
  pc = 0x1091688; continue;
 }
 0x1091688 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10916aa; continue; }
  pc = 0x1091691; continue;
 }
 0x1091691 => {
  cpu.push(0x1091696); f_0109cb12(cpu); if cpu.returned_to != 0x1091696 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10916aa; continue; }
  pc = 0x109169a; continue;
 }
 0x109169a => {
  cpu.push(0x109169f); f_0108f863(cpu); if cpu.returned_to != 0x109169f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10916aa; continue; }
  pc = 0x10916a3; continue;
 }
 0x10916a3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10916ac; continue;
 }
 0x10916aa => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10916ac; continue;
 }
 0x10916ac => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10916cd; continue; }
  pc = 0x10916b0; continue;
 }
 0x10916b0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113805bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10916ca); f_01095199(cpu); if cpu.returned_to != 0x10916ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10916cd; continue;
 }
 0x10916cd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10916dc); f_0109efb0(cpu); if cpu.returned_to != 0x10916dc { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010916e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10916e1); }
 let mut pc: u32 = 0x10916e1;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090452 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x10916e1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091721; continue; }
  pc = 0x10916fa; continue;
 }
 0x10916fa => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091708; continue; }
  pc = 0x10916ff; continue;
 }
 0x10916ff => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091721; continue; }
  pc = 0x1091708; continue;
 }
 0x1091708 => {
  cpu.push(0x109170d); f_0109cb12(cpu); if cpu.returned_to != 0x109170d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091721; continue; }
  pc = 0x1091711; continue;
 }
 0x1091711 => {
  cpu.push(0x1091716); f_0108f863(cpu); if cpu.returned_to != 0x1091716 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091721; continue; }
  pc = 0x109171a; continue;
 }
 0x109171a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091723; continue;
 }
 0x1091721 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091723; continue;
 }
 0x1091723 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091740; continue; }
  pc = 0x1091727; continue;
 }
 0x1091727 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x113807fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109173d); f_01095199(cpu); if cpu.returned_to != 0x109173d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1091740; continue;
 }
 0x1091740 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109174a); f_0109ee60(cpu); if cpu.returned_to != 0x109174a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109175c; continue;
 }
 0x109175c => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1091772; continue; }
  pc = 0x1091765; continue;
 }
 0x1091765 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091772; continue;
 }
 0x1091772 => {
  cpu.push(0x1091777); f_0109cb12(cpu); if cpu.returned_to != 0x1091777 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109177f; continue;
 }
 0x109177f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1091798; continue;
 }
 0x1091783 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1091794); f_01095199(cpu); if cpu.returned_to != 0x1091794 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091798; continue;
 }
 0x1091798 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1091783; continue; }
  pc = 0x109179d; continue;
 }
 0x109179d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10917b9; continue;
 }
 0x10917a4 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10917b5); f_01095199(cpu); if cpu.returned_to != 0x10917b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10917b9; continue;
 }
 0x10917b9 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10917a4; continue; }
  pc = 0x10917c1; continue;
 }
 0x10917c1 => {
  cpu.poll(); pc = 0x1090452; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091b5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091b5a); }
 let mut pc: u32 = 0x1091b5a;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091b5a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091b99; continue; }
  pc = 0x1091b72; continue;
 }
 0x1091b72 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091b80; continue; }
  pc = 0x1091b77; continue;
 }
 0x1091b77 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091b99; continue; }
  pc = 0x1091b80; continue;
 }
 0x1091b80 => {
  cpu.push(0x1091b85); f_0109cb12(cpu); if cpu.returned_to != 0x1091b85 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091b99; continue; }
  pc = 0x1091b89; continue;
 }
 0x1091b89 => {
  cpu.push(0x1091b8e); f_0108f863(cpu); if cpu.returned_to != 0x1091b8e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091b99; continue; }
  pc = 0x1091b92; continue;
 }
 0x1091b92 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091b9b; continue;
 }
 0x1091b99 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091b9b; continue;
 }
 0x1091b9b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091bbc; continue; }
  pc = 0x1091b9f; continue;
 }
 0x1091b9f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138104u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091bb9); f_01095199(cpu); if cpu.returned_to != 0x1091bb9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1091bbc; continue;
 }
 0x1091bbc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1091bcb); f_0109f1b0(cpu); if cpu.returned_to != 0x1091bcb { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091ea7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091ea7); }
 let mut pc: u32 = 0x1091ea7;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090d22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1091ea7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091ee7; continue; }
  pc = 0x1091ec0; continue;
 }
 0x1091ec0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091ece; continue; }
  pc = 0x1091ec5; continue;
 }
 0x1091ec5 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091ee7; continue; }
  pc = 0x1091ece; continue;
 }
 0x1091ece => {
  cpu.push(0x1091ed3); f_0109cb12(cpu); if cpu.returned_to != 0x1091ed3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091ee7; continue; }
  pc = 0x1091ed7; continue;
 }
 0x1091ed7 => {
  cpu.push(0x1091edc); f_0108f863(cpu); if cpu.returned_to != 0x1091edc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091ee7; continue; }
  pc = 0x1091ee0; continue;
 }
 0x1091ee0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091ee9; continue;
 }
 0x1091ee7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091ee9; continue;
 }
 0x1091ee9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091f06; continue; }
  pc = 0x1091eed; continue;
 }
 0x1091eed => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11381a5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1091f03); f_01095199(cpu); if cpu.returned_to != 0x1091f03 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1091f06; continue;
 }
 0x1091f06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1091f10); f_0109f340(cpu); if cpu.returned_to != 0x1091f10 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091f22; continue;
 }
 0x1091f22 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1091f38; continue; }
  pc = 0x1091f2b; continue;
 }
 0x1091f2b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091f38; continue;
 }
 0x1091f38 => {
  cpu.push(0x1091f3d); f_0109cb12(cpu); if cpu.returned_to != 0x1091f3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1091f45; continue;
 }
 0x1091f45 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1091f5e; continue;
 }
 0x1091f49 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1091f5a); f_01095199(cpu); if cpu.returned_to != 0x1091f5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091f5e; continue;
 }
 0x1091f5e => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1091f49; continue; }
  pc = 0x1091f63; continue;
 }
 0x1091f63 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1091f7f; continue;
 }
 0x1091f6a => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1091f7b); f_01095199(cpu); if cpu.returned_to != 0x1091f7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1091f7f; continue;
 }
 0x1091f7f => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1091f6a; continue; }
  pc = 0x1091f87; continue;
 }
 0x1091f87 => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01091f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1091f8c); }
 let mut pc: u32 = 0x1091f8c;
 loop { match pc {
 0x108fe00 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1091f8c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1091fcc; continue; }
  pc = 0x1091fa5; continue;
 }
 0x1091fa5 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1091fb3; continue; }
  pc = 0x1091faa; continue;
 }
 0x1091faa => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1091fcc; continue; }
  pc = 0x1091fb3; continue;
 }
 0x1091fb3 => {
  cpu.push(0x1091fb8); f_0109cb12(cpu); if cpu.returned_to != 0x1091fb8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1091fcc; continue; }
  pc = 0x1091fbc; continue;
 }
 0x1091fbc => {
  cpu.push(0x1091fc1); f_0108f863(cpu); if cpu.returned_to != 0x1091fc1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091fcc; continue; }
  pc = 0x1091fc5; continue;
 }
 0x1091fc5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1091fce; continue;
 }
 0x1091fcc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1091fce; continue;
 }
 0x1091fce => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1091ffa; continue; }
  pc = 0x1091fd2; continue;
 }
 0x1091fd2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x11381c4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1091ff7); f_01095199(cpu); if cpu.returned_to != 0x1091ff7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x1091ffa; continue;
 }
 0x1091ffa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092013); f_0109f2b0(cpu); if cpu.returned_to != 0x1092013 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.poll(); pc = 0x108fe00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092091(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092091); }
 let mut pc: u32 = 0x1092091;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090452 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1092091 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10920d1; continue; }
  pc = 0x10920aa; continue;
 }
 0x10920aa => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10920b8; continue; }
  pc = 0x10920af; continue;
 }
 0x10920af => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10920d1; continue; }
  pc = 0x10920b8; continue;
 }
 0x10920b8 => {
  cpu.push(0x10920bd); f_0109cb12(cpu); if cpu.returned_to != 0x10920bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10920d1; continue; }
  pc = 0x10920c1; continue;
 }
 0x10920c1 => {
  cpu.push(0x10920c6); f_0108f863(cpu); if cpu.returned_to != 0x10920c6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10920d1; continue; }
  pc = 0x10920ca; continue;
 }
 0x10920ca => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10920d3; continue;
 }
 0x10920d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10920d3; continue;
 }
 0x10920d3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10920f0; continue; }
  pc = 0x10920d7; continue;
 }
 0x10920d7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x113820fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10920ed); f_01095199(cpu); if cpu.returned_to != 0x10920ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10920f0; continue;
 }
 0x10920f0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10920fa); f_0109ef50(cpu); if cpu.returned_to != 0x10920fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109210c; continue;
 }
 0x109210c => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1092122; continue; }
  pc = 0x1092115; continue;
 }
 0x1092115 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092122; continue;
 }
 0x1092122 => {
  cpu.push(0x1092127); f_0109cb12(cpu); if cpu.returned_to != 0x1092127 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109212f; continue;
 }
 0x109212f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092148; continue;
 }
 0x1092133 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1092144); f_01095199(cpu); if cpu.returned_to != 0x1092144 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092148; continue;
 }
 0x1092148 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092133; continue; }
  pc = 0x109214d; continue;
 }
 0x109214d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092169; continue;
 }
 0x1092154 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1092165); f_01095199(cpu); if cpu.returned_to != 0x1092165 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092169; continue;
 }
 0x1092169 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092154; continue; }
  pc = 0x1092171; continue;
 }
 0x1092171 => {
  cpu.poll(); pc = 0x1090452; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092263(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092263); }
 let mut pc: u32 = 0x1092263;
 loop { match pc {
 0x108fc06 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x108ff93; continue;
 }
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090bca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  cpu.poll(); pc = 0x108fc06; continue;
 }
 0x1092263 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10922a3; continue; }
  pc = 0x109227c; continue;
 }
 0x109227c => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x109228a; continue; }
  pc = 0x1092281; continue;
 }
 0x1092281 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10922a3; continue; }
  pc = 0x109228a; continue;
 }
 0x109228a => {
  cpu.push(0x109228f); f_0109cb12(cpu); if cpu.returned_to != 0x109228f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10922a3; continue; }
  pc = 0x1092293; continue;
 }
 0x1092293 => {
  cpu.push(0x1092298); f_0108f863(cpu); if cpu.returned_to != 0x1092298 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10922a3; continue; }
  pc = 0x109229c; continue;
 }
 0x109229c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10922a5; continue;
 }
 0x10922a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10922a5; continue;
 }
 0x10922a5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10922c6; continue; }
  pc = 0x10922a9; continue;
 }
 0x10922a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138250u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10922c3); f_01095199(cpu); if cpu.returned_to != 0x10922c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10922c6; continue;
 }
 0x10922c6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10922d5); f_0109f130(cpu); if cpu.returned_to != 0x10922d5 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10922e7; continue;
 }
 0x10922e7 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10922fd; continue; }
  pc = 0x10922f0; continue;
 }
 0x10922f0 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10922fd; continue;
 }
 0x10922fd => {
  cpu.push(0x1092302); f_0109cb12(cpu); if cpu.returned_to != 0x1092302 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109230a; continue;
 }
 0x109230a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092322; continue;
 }
 0x109230e => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109231e); f_01095199(cpu); if cpu.returned_to != 0x109231e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092322; continue;
 }
 0x1092322 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x109230e; continue; }
  pc = 0x1092327; continue;
 }
 0x1092327 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092343; continue;
 }
 0x109232e => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109233f); f_01095199(cpu); if cpu.returned_to != 0x109233f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092343; continue;
 }
 0x1092343 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x109232e; continue; }
  pc = 0x109234b; continue;
 }
 0x109234b => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0109277e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109277e); }
 let mut pc: u32 = 0x109277e;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090d22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x109277e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10927be; continue; }
  pc = 0x1092797; continue;
 }
 0x1092797 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10927a5; continue; }
  pc = 0x109279c; continue;
 }
 0x109279c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10927be; continue; }
  pc = 0x10927a5; continue;
 }
 0x10927a5 => {
  cpu.push(0x10927aa); f_0109cb12(cpu); if cpu.returned_to != 0x10927aa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10927be; continue; }
  pc = 0x10927ae; continue;
 }
 0x10927ae => {
  cpu.push(0x10927b3); f_0108f863(cpu); if cpu.returned_to != 0x10927b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10927be; continue; }
  pc = 0x10927b7; continue;
 }
 0x10927b7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10927c0; continue;
 }
 0x10927be => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10927c0; continue;
 }
 0x10927c0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10927dd; continue; }
  pc = 0x10927c4; continue;
 }
 0x10927c4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138326u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10927da); f_01095199(cpu); if cpu.returned_to != 0x10927da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10927dd; continue;
 }
 0x10927dd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10927e7); f_010a1d20(cpu); if cpu.returned_to != 0x10927e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10927f9; continue;
 }
 0x10927f9 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x109280f; continue; }
  pc = 0x1092802; continue;
 }
 0x1092802 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109280f; continue;
 }
 0x109280f => {
  cpu.push(0x1092814); f_0109cb12(cpu); if cpu.returned_to != 0x1092814 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x109281c; continue;
 }
 0x109281c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092835; continue;
 }
 0x1092820 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1092831); f_01095199(cpu); if cpu.returned_to != 0x1092831 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092835; continue;
 }
 0x1092835 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092820; continue; }
  pc = 0x109283a; continue;
 }
 0x109283a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092856; continue;
 }
 0x1092841 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1092852); f_01095199(cpu); if cpu.returned_to != 0x1092852 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092856; continue;
 }
 0x1092856 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092841; continue; }
  pc = 0x109285e; continue;
 }
 0x109285e => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010929bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10929bd); }
 let mut pc: u32 = 0x10929bd;
 loop { match pc {
 0x108fcfc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x10929bd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10929fd; continue; }
  pc = 0x10929d6; continue;
 }
 0x10929d6 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10929e4; continue; }
  pc = 0x10929db; continue;
 }
 0x10929db => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10929fd; continue; }
  pc = 0x10929e4; continue;
 }
 0x10929e4 => {
  cpu.push(0x10929e9); f_0109cb12(cpu); if cpu.returned_to != 0x10929e9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10929fd; continue; }
  pc = 0x10929ed; continue;
 }
 0x10929ed => {
  cpu.push(0x10929f2); f_0108f863(cpu); if cpu.returned_to != 0x10929f2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10929fd; continue; }
  pc = 0x10929f6; continue;
 }
 0x10929f6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10929ff; continue;
 }
 0x10929fd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10929ff; continue;
 }
 0x10929ff => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092a1c; continue; }
  pc = 0x1092a03; continue;
 }
 0x1092a03 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138387u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092a19); f_01095199(cpu); if cpu.returned_to != 0x1092a19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092a1c; continue;
 }
 0x1092a1c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092a26); f_010a1ed0(cpu); if cpu.returned_to != 0x1092a26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092a38; continue;
 }
 0x1092a38 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1092a4e; continue; }
  pc = 0x1092a41; continue;
 }
 0x1092a41 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092a4e; continue;
 }
 0x1092a4e => {
  cpu.push(0x1092a53); f_0109cb12(cpu); if cpu.returned_to != 0x1092a53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092a5b; continue;
 }
 0x1092a5b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092a74; continue;
 }
 0x1092a5f => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1092a70); f_01095199(cpu); if cpu.returned_to != 0x1092a70 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092a74; continue;
 }
 0x1092a74 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092a5f; continue; }
  pc = 0x1092a79; continue;
 }
 0x1092a79 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092a95; continue;
 }
 0x1092a80 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1092a91); f_01095199(cpu); if cpu.returned_to != 0x1092a91 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092a95; continue;
 }
 0x1092a95 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092a80; continue; }
  pc = 0x1092a9d; continue;
 }
 0x1092a9d => {
  cpu.poll(); pc = 0x108fcfc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092aa2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092aa2); }
 let mut pc: u32 = 0x1092aa2;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092aa2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092ae1; continue; }
  pc = 0x1092aba; continue;
 }
 0x1092aba => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092ac8; continue; }
  pc = 0x1092abf; continue;
 }
 0x1092abf => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092ae1; continue; }
  pc = 0x1092ac8; continue;
 }
 0x1092ac8 => {
  cpu.push(0x1092acd); f_0109cb12(cpu); if cpu.returned_to != 0x1092acd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092ae1; continue; }
  pc = 0x1092ad1; continue;
 }
 0x1092ad1 => {
  cpu.push(0x1092ad6); f_0108f863(cpu); if cpu.returned_to != 0x1092ad6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092ae1; continue; }
  pc = 0x1092ada; continue;
 }
 0x1092ada => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092ae3; continue;
 }
 0x1092ae1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092ae3; continue;
 }
 0x1092ae3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092b00; continue; }
  pc = 0x1092ae7; continue;
 }
 0x1092ae7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11383abu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092afd); f_01095199(cpu); if cpu.returned_to != 0x1092afd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092b00; continue;
 }
 0x1092b00 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092b0a); f_010a1f50(cpu); if cpu.returned_to != 0x1092b0a { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092b0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092b0f); }
 let mut pc: u32 = 0x1092b0f;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109123f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1092b0f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092b4f; continue; }
  pc = 0x1092b28; continue;
 }
 0x1092b28 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092b36; continue; }
  pc = 0x1092b2d; continue;
 }
 0x1092b2d => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092b4f; continue; }
  pc = 0x1092b36; continue;
 }
 0x1092b36 => {
  cpu.push(0x1092b3b); f_0109cb12(cpu); if cpu.returned_to != 0x1092b3b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092b4f; continue; }
  pc = 0x1092b3f; continue;
 }
 0x1092b3f => {
  cpu.push(0x1092b44); f_0108f863(cpu); if cpu.returned_to != 0x1092b44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092b4f; continue; }
  pc = 0x1092b48; continue;
 }
 0x1092b48 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092b51; continue;
 }
 0x1092b4f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092b51; continue;
 }
 0x1092b51 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092b78; continue; }
  pc = 0x1092b55; continue;
 }
 0x1092b55 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11383ceu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1092b75); f_01095199(cpu); if cpu.returned_to != 0x1092b75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x1092b78; continue;
 }
 0x1092b78 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1092b8c); f_010a1f70(cpu); if cpu.returned_to != 0x1092b8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092b9e; continue;
 }
 0x1092b9e => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1092bb4; continue; }
  pc = 0x1092ba7; continue;
 }
 0x1092ba7 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092bb4; continue;
 }
 0x1092bb4 => {
  cpu.push(0x1092bb9); f_0109cb12(cpu); if cpu.returned_to != 0x1092bb9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092bc1; continue;
 }
 0x1092bc1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092bda; continue;
 }
 0x1092bc5 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1092bd6); f_01095199(cpu); if cpu.returned_to != 0x1092bd6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092bda; continue;
 }
 0x1092bda => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092bc5; continue; }
  pc = 0x1092bdf; continue;
 }
 0x1092bdf => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1092bfb; continue;
 }
 0x1092be6 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092bf7); f_01095199(cpu); if cpu.returned_to != 0x1092bf7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092bfb; continue;
 }
 0x1092bfb => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092be6; continue; }
  pc = 0x1092c03; continue;
 }
 0x1092c03 => {
  cpu.poll(); pc = 0x109123f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092c08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092c08); }
 let mut pc: u32 = 0x1092c08;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092c08 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092c47; continue; }
  pc = 0x1092c20; continue;
 }
 0x1092c20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092c2e; continue; }
  pc = 0x1092c25; continue;
 }
 0x1092c25 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092c47; continue; }
  pc = 0x1092c2e; continue;
 }
 0x1092c2e => {
  cpu.push(0x1092c33); f_0109cb12(cpu); if cpu.returned_to != 0x1092c33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092c47; continue; }
  pc = 0x1092c37; continue;
 }
 0x1092c37 => {
  cpu.push(0x1092c3c); f_0108f863(cpu); if cpu.returned_to != 0x1092c3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092c47; continue; }
  pc = 0x1092c40; continue;
 }
 0x1092c40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092c49; continue;
 }
 0x1092c47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092c49; continue;
 }
 0x1092c49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092c66; continue; }
  pc = 0x1092c4d; continue;
 }
 0x1092c4d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11383efu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092c63); f_01095199(cpu); if cpu.returned_to != 0x1092c63 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092c66; continue;
 }
 0x1092c66 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092c70); f_010a2360(cpu); if cpu.returned_to != 0x1092c70 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092c75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092c75); }
 let mut pc: u32 = 0x1092c75;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092c75 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092cb4; continue; }
  pc = 0x1092c8d; continue;
 }
 0x1092c8d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092c9b; continue; }
  pc = 0x1092c92; continue;
 }
 0x1092c92 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092cb4; continue; }
  pc = 0x1092c9b; continue;
 }
 0x1092c9b => {
  cpu.push(0x1092ca0); f_0109cb12(cpu); if cpu.returned_to != 0x1092ca0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092cb4; continue; }
  pc = 0x1092ca4; continue;
 }
 0x1092ca4 => {
  cpu.push(0x1092ca9); f_0108f863(cpu); if cpu.returned_to != 0x1092ca9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092cb4; continue; }
  pc = 0x1092cad; continue;
 }
 0x1092cad => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092cb6; continue;
 }
 0x1092cb4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092cb6; continue;
 }
 0x1092cb6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092cd3; continue; }
  pc = 0x1092cba; continue;
 }
 0x1092cba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1138409u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092cd0); f_01095199(cpu); if cpu.returned_to != 0x1092cd0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092cd3; continue;
 }
 0x1092cd3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092cdd); f_010a23a0(cpu); if cpu.returned_to != 0x1092cdd { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092d4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092d4f); }
 let mut pc: u32 = 0x1092d4f;
 loop { match pc {
 0x108fe72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092d4f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092d8e; continue; }
  pc = 0x1092d67; continue;
 }
 0x1092d67 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092d75; continue; }
  pc = 0x1092d6c; continue;
 }
 0x1092d6c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092d8e; continue; }
  pc = 0x1092d75; continue;
 }
 0x1092d75 => {
  cpu.push(0x1092d7a); f_0109cb12(cpu); if cpu.returned_to != 0x1092d7a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092d8e; continue; }
  pc = 0x1092d7e; continue;
 }
 0x1092d7e => {
  cpu.push(0x1092d83); f_0108f863(cpu); if cpu.returned_to != 0x1092d83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092d8e; continue; }
  pc = 0x1092d87; continue;
 }
 0x1092d87 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092d90; continue;
 }
 0x1092d8e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092d90; continue;
 }
 0x1092d90 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092dad; continue; }
  pc = 0x1092d94; continue;
 }
 0x1092d94 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x113843du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092daa); f_01095199(cpu); if cpu.returned_to != 0x1092daa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092dad; continue;
 }
 0x1092dad => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092db7); f_010a2500(cpu); if cpu.returned_to != 0x1092db7 { return; }
  cpu.poll(); pc = 0x108fe72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01092dbc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1092dbc); } r_01092dbc(cpu, 0x1092dbc); }
pub fn f_01093c31(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1093c31); } r_01092dbc(cpu, 0x1093c31); }
pub fn f_0109467d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109467d); } r_01092dbc(cpu, 0x109467d); }
fn r_01092dbc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108fdfd => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108fe00; continue;
 }
 0x108fe00 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092013 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.poll(); pc = 0x108fe00; continue;
 }
 0x1092dbc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092dfc; continue; }
  pc = 0x1092dd5; continue;
 }
 0x1092dd5 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092de3; continue; }
  pc = 0x1092dda; continue;
 }
 0x1092dda => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092dfc; continue; }
  pc = 0x1092de3; continue;
 }
 0x1092de3 => {
  cpu.push(0x1092de8); f_0109cb12(cpu); if cpu.returned_to != 0x1092de8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092dfc; continue; }
  pc = 0x1092dec; continue;
 }
 0x1092dec => {
  cpu.push(0x1092df1); f_0108f863(cpu); if cpu.returned_to != 0x1092df1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092dfc; continue; }
  pc = 0x1092df5; continue;
 }
 0x1092df5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092dfe; continue;
 }
 0x1092dfc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092dfe; continue;
 }
 0x1092dfe => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092e25; continue; }
  pc = 0x1092e02; continue;
 }
 0x1092e02 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1138455u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1092e22); f_01095199(cpu); if cpu.returned_to != 0x1092e22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x1092e25; continue;
 }
 0x1092e25 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1092e39); f_010a2550(cpu); if cpu.returned_to != 0x1092e39 { return; }
  cpu.poll(); pc = 0x108fdfd; continue;
 }
 0x1093c31 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093c79; continue; }
  pc = 0x1093c52; continue;
 }
 0x1093c52 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093c60; continue; }
  pc = 0x1093c57; continue;
 }
 0x1093c57 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093c79; continue; }
  pc = 0x1093c60; continue;
 }
 0x1093c60 => {
  cpu.push(0x1093c65); f_0109cb12(cpu); if cpu.returned_to != 0x1093c65 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093c79; continue; }
  pc = 0x1093c69; continue;
 }
 0x1093c69 => {
  cpu.push(0x1093c6e); f_0108f863(cpu); if cpu.returned_to != 0x1093c6e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093c79; continue; }
  pc = 0x1093c72; continue;
 }
 0x1093c72 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093c7b; continue;
 }
 0x1093c79 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093c7b; continue;
 }
 0x1093c7b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093c9a; continue; }
  pc = 0x1093c7f; continue;
 }
 0x1093c7f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11386c0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1093c97); f_01095199(cpu); if cpu.returned_to != 0x1093c97 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x1093c9a; continue;
 }
 0x1093c9a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093ca6); f_010a2950(cpu); if cpu.returned_to != 0x1093ca6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108fe00; continue; }
  pc = 0x1093cb6; continue;
 }
 0x1093cb6 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1093ccc; continue; }
  pc = 0x1093cbf; continue;
 }
 0x1093cbf => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108fe00; continue; }
  pc = 0x1093ccc; continue;
 }
 0x1093ccc => {
  cpu.push(0x1093cd1); f_0109cb12(cpu); if cpu.returned_to != 0x1093cd1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108fe00; continue; }
  pc = 0x1093cd9; continue;
 }
 0x1093cd9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1093cf2; continue;
 }
 0x1093cdd => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093cee); f_01095199(cpu); if cpu.returned_to != 0x1093cee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093cf2; continue;
 }
 0x1093cf2 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1093cdd; continue; }
  pc = 0x1093cf7; continue;
 }
 0x1093cf7 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1093d13; continue;
 }
 0x1093cfe => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1093d0f); f_01095199(cpu); if cpu.returned_to != 0x1093d0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093d13; continue;
 }
 0x1093d13 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1093cfe; continue; }
  pc = 0x1093d1b; continue;
 }
 0x1093d1b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11386e7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1093d34); f_01095199(cpu); if cpu.returned_to != 0x1093d34 { return; }
  cpu.poll(); pc = 0x1092013; continue;
 }
 0x109467d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10946bd; continue; }
  pc = 0x1094696; continue;
 }
 0x1094696 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10946a4; continue; }
  pc = 0x109469b; continue;
 }
 0x109469b => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10946bd; continue; }
  pc = 0x10946a4; continue;
 }
 0x10946a4 => {
  cpu.push(0x10946a9); f_0109cb12(cpu); if cpu.returned_to != 0x10946a9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10946bd; continue; }
  pc = 0x10946ad; continue;
 }
 0x10946ad => {
  cpu.push(0x10946b2); f_0108f863(cpu); if cpu.returned_to != 0x10946b2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10946bd; continue; }
  pc = 0x10946b6; continue;
 }
 0x10946b6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10946bf; continue;
 }
 0x10946bd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10946bf; continue;
 }
 0x10946bf => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10946e6; continue; }
  pc = 0x10946c3; continue;
 }
 0x10946c3 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11388aau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10946e3); f_01095199(cpu); if cpu.returned_to != 0x10946e3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10946e6; continue;
 }
 0x10946e6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10946fa); f_010a2e30(cpu); if cpu.returned_to != 0x10946fa { return; }
  cpu.poll(); pc = 0x108fdfd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092e3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092e3e); }
 let mut pc: u32 = 0x1092e3e;
 loop { match pc {
 0x108fdfd => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1092e3e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092e7e; continue; }
  pc = 0x1092e57; continue;
 }
 0x1092e57 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092e65; continue; }
  pc = 0x1092e5c; continue;
 }
 0x1092e5c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092e7e; continue; }
  pc = 0x1092e65; continue;
 }
 0x1092e65 => {
  cpu.push(0x1092e6a); f_0109cb12(cpu); if cpu.returned_to != 0x1092e6a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092e7e; continue; }
  pc = 0x1092e6e; continue;
 }
 0x1092e6e => {
  cpu.push(0x1092e73); f_0108f863(cpu); if cpu.returned_to != 0x1092e73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092e7e; continue; }
  pc = 0x1092e77; continue;
 }
 0x1092e77 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092e80; continue;
 }
 0x1092e7e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092e80; continue;
 }
 0x1092e80 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092ea7; continue; }
  pc = 0x1092e84; continue;
 }
 0x1092e84 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1138479u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1092ea4); f_01095199(cpu); if cpu.returned_to != 0x1092ea4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x1092ea7; continue;
 }
 0x1092ea7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1092ebb); f_010a25c0(cpu); if cpu.returned_to != 0x1092ebb { return; }
  cpu.poll(); pc = 0x108fdfd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01092f36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1092f36); }
 let mut pc: u32 = 0x1092f36;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090452 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137cafu32; cpu.push(v); }
  pc = 0x1090d28; continue;
 }
 0x1090d28 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x1092f36 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1092f76; continue; }
  pc = 0x1092f4f; continue;
 }
 0x1092f4f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1092f5d; continue; }
  pc = 0x1092f54; continue;
 }
 0x1092f54 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1092f76; continue; }
  pc = 0x1092f5d; continue;
 }
 0x1092f5d => {
  cpu.push(0x1092f62); f_0109cb12(cpu); if cpu.returned_to != 0x1092f62 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1092f76; continue; }
  pc = 0x1092f66; continue;
 }
 0x1092f66 => {
  cpu.push(0x1092f6b); f_0108f863(cpu); if cpu.returned_to != 0x1092f6b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092f76; continue; }
  pc = 0x1092f6f; continue;
 }
 0x1092f6f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1092f78; continue;
 }
 0x1092f76 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1092f78; continue;
 }
 0x1092f78 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1092f95; continue; }
  pc = 0x1092f7c; continue;
 }
 0x1092f7c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11384c4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1092f92); f_01095199(cpu); if cpu.returned_to != 0x1092f92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1092f95; continue;
 }
 0x1092f95 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1092f9f); f_010a2640(cpu); if cpu.returned_to != 0x1092f9f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092fb1; continue;
 }
 0x1092fb1 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1092fc7; continue; }
  pc = 0x1092fba; continue;
 }
 0x1092fba => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092fc7; continue;
 }
 0x1092fc7 => {
  cpu.push(0x1092fcc); f_0109cb12(cpu); if cpu.returned_to != 0x1092fcc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1092fd4; continue;
 }
 0x1092fd4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1092fed; continue;
 }
 0x1092fd8 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1092fe9); f_01095199(cpu); if cpu.returned_to != 0x1092fe9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1092fed; continue;
 }
 0x1092fed => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1092fd8; continue; }
  pc = 0x1092ff2; continue;
 }
 0x1092ff2 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109300e; continue;
 }
 0x1092ff9 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109300a); f_01095199(cpu); if cpu.returned_to != 0x109300a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109300e; continue;
 }
 0x109300e => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1092ff9; continue; }
  pc = 0x1093016; continue;
 }
 0x1093016 => {
  cpu.poll(); pc = 0x1090452; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010932ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10932ca); }
 let mut pc: u32 = 0x10932ca;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10932ca => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093309; continue; }
  pc = 0x10932e2; continue;
 }
 0x10932e2 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10932f0; continue; }
  pc = 0x10932e7; continue;
 }
 0x10932e7 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093309; continue; }
  pc = 0x10932f0; continue;
 }
 0x10932f0 => {
  cpu.push(0x10932f5); f_0109cb12(cpu); if cpu.returned_to != 0x10932f5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093309; continue; }
  pc = 0x10932f9; continue;
 }
 0x10932f9 => {
  cpu.push(0x10932fe); f_0108f863(cpu); if cpu.returned_to != 0x10932fe { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093309; continue; }
  pc = 0x1093302; continue;
 }
 0x1093302 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109330b; continue;
 }
 0x1093309 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109330b; continue;
 }
 0x109330b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109332c; continue; }
  pc = 0x109330f; continue;
 }
 0x109330f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1138533u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093329); f_01095199(cpu); if cpu.returned_to != 0x1093329 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109332c; continue;
 }
 0x109332c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109333b); f_010a2690(cpu); if cpu.returned_to != 0x109333b { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0109351e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109351e); }
 let mut pc: u32 = 0x109351e;
 loop { match pc {
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090d22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.poll(); pc = 0x108ff93; continue;
 }
 0x109351e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109355e; continue; }
  pc = 0x1093537; continue;
 }
 0x1093537 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093545; continue; }
  pc = 0x109353c; continue;
 }
 0x109353c => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109355e; continue; }
  pc = 0x1093545; continue;
 }
 0x1093545 => {
  cpu.push(0x109354a); f_0109cb12(cpu); if cpu.returned_to != 0x109354a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109355e; continue; }
  pc = 0x109354e; continue;
 }
 0x109354e => {
  cpu.push(0x1093553); f_0108f863(cpu); if cpu.returned_to != 0x1093553 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109355e; continue; }
  pc = 0x1093557; continue;
 }
 0x1093557 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093560; continue;
 }
 0x109355e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093560; continue;
 }
 0x1093560 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109357d; continue; }
  pc = 0x1093564; continue;
 }
 0x1093564 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x11385a4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109357a); f_01095199(cpu); if cpu.returned_to != 0x109357a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109357d; continue;
 }
 0x109357d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093587); f_010a1db0(cpu); if cpu.returned_to != 0x1093587 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093599; continue;
 }
 0x1093599 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x10935af; continue; }
  pc = 0x10935a2; continue;
 }
 0x10935a2 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10935af; continue;
 }
 0x10935af => {
  cpu.push(0x10935b4); f_0109cb12(cpu); if cpu.returned_to != 0x10935b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x10935bc; continue;
 }
 0x10935bc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10935d5; continue;
 }
 0x10935c0 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10935d1); f_01095199(cpu); if cpu.returned_to != 0x10935d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10935d5; continue;
 }
 0x10935d5 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x10935c0; continue; }
  pc = 0x10935da; continue;
 }
 0x10935da => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10935f6; continue;
 }
 0x10935e1 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10935f2); f_01095199(cpu); if cpu.returned_to != 0x10935f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10935f6; continue;
 }
 0x10935f6 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10935e1; continue; }
  pc = 0x10935fe; continue;
 }
 0x10935fe => {
  cpu.poll(); pc = 0x1090d22; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01093d39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1093d39); }
 let mut pc: u32 = 0x1093d39;
 loop { match pc {
 0x108fd78 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108fe75; continue;
 }
 0x108fe75 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1093d39 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093d78; continue; }
  pc = 0x1093d51; continue;
 }
 0x1093d51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093d5f; continue; }
  pc = 0x1093d56; continue;
 }
 0x1093d56 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093d78; continue; }
  pc = 0x1093d5f; continue;
 }
 0x1093d5f => {
  cpu.push(0x1093d64); f_0109cb12(cpu); if cpu.returned_to != 0x1093d64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093d78; continue; }
  pc = 0x1093d68; continue;
 }
 0x1093d68 => {
  cpu.push(0x1093d6d); f_0108f863(cpu); if cpu.returned_to != 0x1093d6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093d78; continue; }
  pc = 0x1093d71; continue;
 }
 0x1093d71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093d7a; continue;
 }
 0x1093d78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093d7a; continue;
 }
 0x1093d7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093d9b; continue; }
  pc = 0x1093d7e; continue;
 }
 0x1093d7e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x11386f7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093d98); f_01095199(cpu); if cpu.returned_to != 0x1093d98 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1093d9b; continue;
 }
 0x1093d9b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093daa); f_010a29c0(cpu); if cpu.returned_to != 0x1093daa { return; }
  cpu.poll(); pc = 0x108fd78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01093e9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1093e9c); }
 let mut pc: u32 = 0x1093e9c;
 loop { match pc {
 0x108fc06 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x108ff93; continue;
 }
 0x108ff93 => {
  cpu.push(0x108ff98); f_01095199(cpu); if cpu.returned_to != 0x108ff98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x108ff9b; continue;
 }
 0x108ff9b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1090bca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1137baau32; cpu.push(v); }
  cpu.poll(); pc = 0x108fc06; continue;
 }
 0x1093e9c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x1093edc; continue; }
  pc = 0x1093eb5; continue;
 }
 0x1093eb5 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1093ec3; continue; }
  pc = 0x1093eba; continue;
 }
 0x1093eba => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x1093edc; continue; }
  pc = 0x1093ec3; continue;
 }
 0x1093ec3 => {
  cpu.push(0x1093ec8); f_0109cb12(cpu); if cpu.returned_to != 0x1093ec8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1093edc; continue; }
  pc = 0x1093ecc; continue;
 }
 0x1093ecc => {
  cpu.push(0x1093ed1); f_0108f863(cpu); if cpu.returned_to != 0x1093ed1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093edc; continue; }
  pc = 0x1093ed5; continue;
 }
 0x1093ed5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1093ede; continue;
 }
 0x1093edc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1093ede; continue;
 }
 0x1093ede => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1093eff; continue; }
  pc = 0x1093ee2; continue;
 }
 0x1093ee2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x113873bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093efc); f_01095199(cpu); if cpu.returned_to != 0x1093efc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1093eff; continue;
 }
 0x1093eff => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1093f0e); f_010a2a70(cpu); if cpu.returned_to != 0x1093f0e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093f20; continue;
 }
 0x1093f20 => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x1093f36; continue; }
  pc = 0x1093f29; continue;
 }
 0x1093f29 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093f36; continue;
 }
 0x1093f36 => {
  cpu.push(0x1093f3b); f_0109cb12(cpu); if cpu.returned_to != 0x1093f3b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x108ff9b; continue; }
  pc = 0x1093f43; continue;
 }
 0x1093f43 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1093f5b; continue;
 }
 0x1093f47 => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1093f57); f_01095199(cpu); if cpu.returned_to != 0x1093f57 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093f5b; continue;
 }
 0x1093f5b => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x1093f47; continue; }
  pc = 0x1093f60; continue;
 }
 0x1093f60 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x1093f7c; continue;
 }
 0x1093f67 => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1093f78); f_01095199(cpu); if cpu.returned_to != 0x1093f78 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1093f7c; continue;
 }
 0x1093f7c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1093f67; continue; }
  pc = 0x1093f84; continue;
 }
 0x1093f84 => {
  cpu.poll(); pc = 0x1090bca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094a70); }
 let mut pc: u32 = 0x1094a70;
 loop { match pc {
 0x1094a70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133c18u32), 0x0u32); }
  if cpu.zf { pc = 0x1094a96; continue; }
  pc = 0x1094a7c; continue;
 }
 0x1094a7c => {
  { let v = 0x1094a70u32; cpu.push(v); }
  { let v = 0x108f74fu32; cpu.push(v); }
  cpu.push(0x1094a8b); f_01013dc7(cpu); if cpu.returned_to != 0x1094a8b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133c18u32, v); }
  pc = 0x1094a96; continue;
 }
 0x1094a96 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094a9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094a9a); }
 let mut pc: u32 = 0x1094a9a;
 loop { match pc {
 0x1094a9a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094ab3; continue; }
  pc = 0x1094aae; continue;
 }
 0x1094aae => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1094aba; continue; }
  pc = 0x1094ab3; continue;
 }
 0x1094ab3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1094b6e; continue;
 }
 0x1094aba => {
  { let v: u32 = 0x1133c1cu32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x1094acd; continue;
 }
 0x1094acd => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x1094ad9; continue; }
  pc = 0x1094ad4; continue;
 }
 0x1094ad4 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x1094ad9; continue;
 }
 0x1094ad9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133c24u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x1094af9; continue; }
  pc = 0x1094ae1; continue;
 }
 0x1094ae1 => {
  { let v: u32 = cpu.r32(0x1133c20u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1094b31; continue; }
  pc = 0x1094aeb; continue;
 }
 0x1094aeb => {
  { let v: u32 = cpu.ecx; cpu.w32(0x1133c24u32, v); }
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.ecx = v; }
  pc = 0x1094b31; continue;
 }
 0x1094af9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x1094aeb; continue;
 }
 0x1094afd => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133c20u32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1094b16); f_010a3ca2(cpu); if cpu.returned_to != 0x1094b16 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1094b64; continue; }
  pc = 0x1094b1c; continue;
 }
 0x1094b1c => {
  { let v: u32 = cpu.r32(0x1133c24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x1094b2e; continue; }
  pc = 0x1094b29; continue;
 }
 0x1094b29 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133c24u32, v); }
  pc = 0x1094b2e; continue;
 }
 0x1094b2e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x1094b31; continue;
 }
 0x1094b31 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x1094afd; continue; }
  pc = 0x1094b35; continue;
 }
 0x1094b35 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x1094b46; continue; }
  pc = 0x1094b3b; continue;
 }
 0x1094b3b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1094b42); f_010a4246(cpu); if cpu.returned_to != 0x1094b42 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1094b5b; continue; }
  pc = 0x1094b46; continue;
 }
 0x1094b46 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1094b4d); f_010a440f(cpu); if cpu.returned_to != 0x1094b4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094b64; continue; }
  pc = 0x1094b51; continue;
 }
 0x1094b51 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x1094acd; continue;
 }
 0x1094b5b => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x1094acd; continue;
 }
 0x1094b64 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x11ee5b0u32, v); }
  pc = 0x1094b6e; continue;
 }
 0x1094b6e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
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
pub fn f_01094b7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094b7d); }
 let mut pc: u32 = 0x1094b7d;
 loop { match pc {
 0x1094b7d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1094bf9; continue; }
  pc = 0x1094b8c; continue;
 }
 0x1094b8c => {
  { let v: u32 = cpu.r32(0x11ec614u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1094b9f; continue; }
  pc = 0x1094b96; continue;
 }
 0x1094b96 => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1094b9f; continue; }
  pc = 0x1094b9a; continue;
 }
 0x1094b9a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x1094bb9; continue; }
  pc = 0x1094b9f; continue;
 }
 0x1094b9f => {
  { let v: u32 = cpu.r32(0x1133c1cu32); cpu.ecx = v; }
  pc = 0x1094bb2; continue;
 }
 0x1094ba7 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1094baf; continue; }
  pc = 0x1094bab; continue;
 }
 0x1094bab => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x1094bb9; continue; }
  pc = 0x1094baf; continue;
 }
 0x1094baf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x1094bb2; continue;
 }
 0x1094bb2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x1094ba7; continue; }
  pc = 0x1094bb9; continue;
 }
 0x1094bb9 => {
  { let v: u32 = 0x1133c1cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1094bcf); f_010a3d4a(cpu); if cpu.returned_to != 0x1094bcf { return; }
  { let v: u32 = cpu.r32(0x1133c20u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11ec614u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x1094bf1; continue; }
  pc = 0x1094bdf; continue;
 }
 0x1094bdf => {
  { let v: u32 = cpu.r32(0x1133c24u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x1094bf1; continue; }
  pc = 0x1094beb; continue;
 }
 0x1094beb => {
  { let v: u32 = cpu.edx; cpu.w32(0x1133c24u32, v); }
  pc = 0x1094bf1; continue;
 }
 0x1094bf1 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11ee5b0u32, v); }
  pc = 0x1094bf9; continue;
 }
 0x1094bf9 => {
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
pub fn f_01094bff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094bff); }
 let mut pc: u32 = 0x1094bff;
 loop { match pc {
 0x1094bff => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1094c39; continue; }
  pc = 0x1094c28; continue;
 }
 0x1094c28 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1094c32); f_0109995d(cpu); if cpu.returned_to != 0x1094c32 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1094c3c; continue;
 }
 0x1094c39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1094c3c; continue;
 }
 0x1094c3c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094c50); }
 let mut pc: u32 = 0x1094c50;
 loop { match pc {
 0x1094c50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x1094cc4; continue; }
  pc = 0x1094c58; continue;
 }
 0x1094c58 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x1094cc9; continue; }
  pc = 0x1094c60; continue;
 }
 0x1094c60 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1094cc4; continue; }
  pc = 0x1094c70; continue;
 }
 0x1094c70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x1094cc9; continue; }
  pc = 0x1094c7a; continue;
 }
 0x1094c7a => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1094cc4; continue; }
  pc = 0x1094c8a; continue;
 }
 0x1094c8a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x1094cc9; continue; }
  pc = 0x1094c94; continue;
 }
 0x1094c94 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1094cc4; continue; }
  pc = 0x1094ca4; continue;
 }
 0x1094ca4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x1094cc9; continue; }
  pc = 0x1094cae; continue;
 }
 0x1094cae => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1094c58; continue; }
  pc = 0x1094cc4; continue;
 }
 0x1094cc4 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1094cc9 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1094cea; continue; }
  pc = 0x1094ccd; continue;
 }
 0x1094ccd => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x1094cc4; continue; }
  pc = 0x1094cd1; continue;
 }
 0x1094cd1 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if !cpu.zf { pc = 0x1094cea; continue; }
  pc = 0x1094cd5; continue;
 }
 0x1094cd5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x1094cc4; continue; }
  pc = 0x1094cda; continue;
 }
 0x1094cda => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1094cea; continue; }
  pc = 0x1094ce4; continue;
 }
 0x1094ce4 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x1094cc4; continue; }
  pc = 0x1094ce8; continue;
 }
 0x1094ce8 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  pc = 0x1094cea; continue;
 }
 0x1094cea => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094cf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094cf2); }
 let mut pc: u32 = 0x1094cf2;
 loop { match pc {
 0x1094cf2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if !cpu.zf { pc = 0x1094d21; continue; }
  pc = 0x1094d07; continue;
 }
 0x1094d07 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1094d09; continue;
 }
 0x1094d09 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1094d19); f_010a4412(cpu); if cpu.returned_to != 0x1094d19 { return; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1094e1a; continue;
 }
 0x1094d21 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x1094d6c; continue; }
  pc = 0x1094d31; continue;
 }
 0x1094d31 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if !cpu.zf { cpu.poll(); pc = 0x1094d09; continue; }
  pc = 0x1094d36; continue;
 }
 0x1094d36 => {
  { let v: u16 = 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { cpu.fprem(); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.pf() { pc = 0x1094d63; continue; }
  pc = 0x1094d52; continue;
 }
 0x1094d52 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1094d63; continue; }
  pc = 0x1094d61; continue;
 }
 0x1094d61 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1094d63; continue;
 }
 0x1094d63 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xfffffff9u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x1094d74; continue;
 }
 0x1094d6c => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if cpu.zf { pc = 0x1094d74; continue; }
  pc = 0x1094d6f; continue;
 }
 0x1094d6f => {
  pc = 0x1094dfb; continue;
 }
 0x1094d74 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if !cpu.cf { pc = 0x1094dfb; continue; }
  pc = 0x1094d8c; continue;
 }
 0x1094d8c => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1094da6; continue; }
  pc = 0x1094d9b; continue;
 }
 0x1094d9b => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1094da4); f_01094e25(cpu); if cpu.returned_to != 0x1094da4 { return; }
  pc = 0x1094dd1; continue;
 }
 0x1094da6 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1094dfb; continue; }
  pc = 0x1094daa; continue;
 }
 0x1094daa => {
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1094dfb; continue; }
  pc = 0x1094db0; continue;
 }
 0x1094db0 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1094db8); f_01094e25(cpu); if cpu.returned_to != 0x1094db8 { return; }
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x1133f74u32), 0x1u8); }
  if !cpu.zf { pc = 0x1094dc7; continue; }
  pc = 0x1094dc3; continue;
 }
 0x1094dc3 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1094dd1; continue;
 }
 0x1094dc7 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x1094dd1); f_010a462b(cpu); if cpu.returned_to != 0x1094dd1 { return; }
  pc = 0x1094dd1; continue;
 }
 0x1094dd1 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1094df7; continue; }
  pc = 0x1094de3; continue;
 }
 0x1094de3 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if !cpu.zf { pc = 0x1094df7; continue; }
  pc = 0x1094df0; continue;
 }
 0x1094df0 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1094d09; continue;
 }
 0x1094df7 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1094e1a; continue;
 }
 0x1094dfb => {
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1094e0c); f_010a44a6(cpu); if cpu.returned_to != 0x1094e0c { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1094df0; continue; }
  pc = 0x1094e10; continue;
 }
 0x1094e10 => {
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if !cpu.cf { pc = 0x1094e18; continue; }
  pc = 0x1094e16; continue;
 }
 0x1094e16 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1094e18; continue;
 }
 0x1094e18 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1094e1a; continue;
 }
 0x1094e1a => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  {  }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094e25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094e25); }
 let mut pc: u32 = 0x1094e25;
 loop { match pc {
 0x1094e25 => {
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x1094e2e; continue; }
  pc = 0x1094e2a; continue;
 }
 0x1094e2a => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  cpu.poll(); pc = 0x1094e25; continue;
 }
 0x1094e2e => {
  if !cpu.cf { pc = 0x1094e43; continue; }
  pc = 0x1094e30; continue;
 }
 0x1094e30 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  pc = 0x1094e32; continue;
 }
 0x1094e32 => {
  if cpu.zf { pc = 0x1094e3f; continue; }
  pc = 0x1094e34; continue;
 }
 0x1094e34 => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x1094e3d; continue; }
  pc = 0x1094e3b; continue;
 }
 0x1094e3b => {
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  pc = 0x1094e3d; continue;
 }
 0x1094e3d => {
  cpu.poll(); pc = 0x1094e32; continue;
 }
 0x1094e3f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x1094e47; continue;
 }
 0x1094e43 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  pc = 0x1094e47; continue;
 }
 0x1094e47 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094e48); }
 let mut pc: u32 = 0x1094e48;
 loop { match pc {
 0x1094e48 => {
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x1094e55); f_01094cf2(cpu); if cpu.returned_to != 0x1094e55 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01094e58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094e58); }
 let mut pc: u32 = 0x1094e58;
 loop { match pc {
 0x1094e58 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x1094e7d; continue; }
  pc = 0x1094e65; continue;
 }
 0x1094e65 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1094e6f); f_01099923(cpu); if cpu.returned_to != 0x1094e6f { return; }
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x1094e73; continue;
 }
 0x1094e73 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1094ef6; continue;
 }
 0x1094e7d => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x1094e8a; continue; }
  pc = 0x1094e83; continue;
 }
 0x1094e83 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1094e8a); f_0109afc8(cpu); if cpu.returned_to != 0x1094e8a { return; }
  pc = 0x1094e8a; continue;
 }
 0x1094e8a => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x1094ec7; continue; }
  pc = 0x1094e94; continue;
 }
 0x1094e94 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1094ec7; continue; }
  pc = 0x1094ea0; continue;
 }
 0x1094ea0 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if !cpu.zf { pc = 0x1094ec7; continue; }
  pc = 0x1094ebc; continue;
 }
 0x1094ebc => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1094ec3); f_0109aec3(cpu); if cpu.returned_to != 0x1094ec3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x1094e73; continue; }
  pc = 0x1094ec7; continue;
 }
 0x1094ec7 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.edi); }
  if !cpu.zf { pc = 0x1094ee7; continue; }
  pc = 0x1094ee2; continue;
 }
 0x1094ee2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x1094ef2; continue; }
  pc = 0x1094ee7; continue;
 }
 0x1094ee7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1094eee); f_0109aec3(cpu); if cpu.returned_to != 0x1094eee { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x1094e73; continue; }
  pc = 0x1094ef2; continue;
 }
 0x1094ef2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1094ef6; continue;
 }
 0x1094ef6 => {
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
pub fn f_01094efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1094efc); }
 let mut pc: u32 = 0x1094efc;
 loop { match pc {
 0x1094efc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x2u8); }
  if !cpu.zf { pc = 0x1094f21; continue; }
  pc = 0x1094f0c; continue;
 }
 0x1094f0c => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1094f16); f_01099923(cpu); if cpu.returned_to != 0x1094f16 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) | 0x20u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10950b5; continue;
 }
 0x1094f21 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1094f2f; continue; }
  pc = 0x1094f28; continue;
 }
 0x1094f28 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10950b5; continue;
 }
 0x1094f2f => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1094f3c; continue; }
  pc = 0x1094f35; continue;
 }
 0x1094f35 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x1094f3c); f_0109afc8(cpu); if cpu.returned_to != 0x1094f3c { return; }
  pc = 0x1094f3c; continue;
 }
 0x1094f3c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax & 0x30u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0xcfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x1095037; continue; }
  pc = 0x1094f5e; continue;
 }
 0x1094f5e => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x1094f62; continue;
 }
 0x1094f62 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1094faa; continue; }
  pc = 0x1094f68; continue;
 }
 0x1094f68 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); }
  if cpu.cf { pc = 0x1094faa; continue; }
  pc = 0x1094f71; continue;
 }
 0x1094f71 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xfeu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1094f7e; continue; }
  pc = 0x1094f7c; continue;
 }
 0x1094f7c => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1094f7e; continue;
 }
 0x1094f7e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1094f8a); f_010a4c30(cpu); if cpu.returned_to != 0x1094f8a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1094fa4; continue; }
  pc = 0x1094f91; continue;
 }
 0x1094f91 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109500b; continue; }
  pc = 0x1094f99; continue;
 }
 0x1094f99 => {
  cpu.push(0x1094f9e); f_010a4ca4(cpu); if cpu.returned_to != 0x1094f9e { return; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1094fa4; continue;
 }
 0x1094fa4 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x109500b; continue;
 }
 0x1094faa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x1094fbc; continue; }
  pc = 0x1094fba; continue;
 }
 0x1094fba => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x1094fbc; continue;
 }
 0x1094fbc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x10u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x1095004; continue; }
  pc = 0x1094ffe; continue;
 }
 0x1094ffe => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x109500b; continue; }
  pc = 0x1095004; continue;
 }
 0x1095004 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x109500b); f_0109aec3(cpu); if cpu.returned_to != 0x109500b { return; }
  pc = 0x109500b; continue;
 }
 0x109500b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x1095092; continue; }
  pc = 0x109502b; continue;
 }
 0x109502b => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x1094f62; continue; }
  pc = 0x1095035; continue;
 }
 0x1095035 => {
  pc = 0x1095092; continue;
 }
 0x1095037 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x1095052; continue; }
  pc = 0x1095041; continue;
 }
 0x1095041 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095052; continue;
 }
 0x1095052 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x1095066); f_01094e58(cpu); if cpu.returned_to != 0x1095066 { return; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x30u8); }
  if !cpu.zf { pc = 0x1095079; continue; }
  pc = 0x109506c; continue;
 }
 0x109506c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x1095052; continue; }
  pc = 0x1095079; continue;
 }
 0x1095079 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1095092; continue; }
  pc = 0x109507d; continue;
 }
 0x109507d => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xfau8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1095092); f_0109aec3(cpu); if cpu.returned_to != 0x1095092 { return; }
  pc = 0x1095092; continue;
 }
 0x1095092 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x109509e; continue; }
  pc = 0x1095098; continue;
 }
 0x1095098 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x109509e; continue;
 }
 0x109509e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx | cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ecx; cpu.div32(v); }
  pc = 0x10950b5; continue;
 }
 0x10950b5 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010950f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10950f8); }
 let mut pc: u32 = 0x10950f8;
 loop { match pc {
 0x10950f8 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.flat_ds(0x1095103); cpu.push(0x1095108); f_0108ee7c(cpu); if cpu.returned_to != 0x1095108 { return; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esp; cpu.w32(0x1133c28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x1133c2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1133e60u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133c34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x11ec618u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.eax; cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x11eca18u32; cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.cf { pc = 0x109515f; continue; }
  pc = 0x1095157; continue;
 }
 0x1095157 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109515f; continue; }
  pc = 0x109515b; continue;
 }
 0x109515b => {
  { let v: u8 = 0x34u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x109516a; continue;
 }
 0x109515f => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1095168; continue; }
  pc = 0x1095163; continue;
 }
 0x1095163 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x109516a; continue;
 }
 0x1095168 => {
  { let v: u8 = 0x34u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x109516a; continue;
 }
 0x109516a => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.edi as u16) as u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  let t = cpu.r32(0x1133c2eu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x1095176); cpu.push(cpu.segments[1] as u32); cpu.push(0x109517c); cpu.call(t); if cpu.returned_to != 0x109517c { return; }
  { let at = 0x1133c28u32.wrapping_add(cpu.bases[3]); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esp = v; cpu.set_segment(2, selector); }
  { let v: u32 = cpu.r32(0x1133c34u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x1095198); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01095199(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095199); }
 let mut pc: u32 = 0x1095199;
 loop { match pc {
 0x1095199 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10951b4); f_01096bcc(cpu); if cpu.returned_to != 0x10951b4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010951ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10951ba); }
 let mut pc: u32 = 0x10951ba;
 loop { match pc {
 0x10951ba => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010951c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10951c2); }
 let mut pc: u32 = 0x10951c2;
 loop { match pc {
 0x10951c2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010951cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10951cc); }
 let mut pc: u32 = 0x10951cc;
 loop { match pc {
 0x10951cc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10951f1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010951f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10951f4); } r_010951f4(cpu, 0x10951f4); }
pub fn f_01095204(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1095204); } r_010951f4(cpu, 0x1095204); }
pub fn f_0109523f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109523f); } r_010951f4(cpu, 0x109523f); }
pub fn f_010952e5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10952e5); } r_010951f4(cpu, 0x10952e5); }
pub fn f_010952e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10952e7); } r_010951f4(cpu, 0x10952e7); }
pub fn f_010952f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10952f3); } r_010951f4(cpu, 0x10952f3); }
fn r_010951f4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10951f4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  pc = 0x1095214; continue;
 }
 0x1095204 => {
  cpu.unimplemented(0x1095204, "Aas");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.port_out(0x52u16, 4, cpu.eax as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.port_in(0x52u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1095214; continue;
 }
 0x1095214 => {
  { let v = 0x4545u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1095222); f_0109582b(cpu); if cpu.returned_to != 0x1095222 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10952ff; continue; }
  pc = 0x1095232; continue;
 }
 0x1095232 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x1095204u32.wrapping_add(cpu.eax)); continue;
 }
 0x109523f => {
  { let v: u32 = 0x4u32; cpu.w32(0x11eca4cu32, v); }
  { let v: u32 = 0x1095f2cu32; cpu.w32(0x11eca48u32, v); }
  { let v: u32 = 0x109615bu32; cpu.w32(0x11eca40u32, v); }
  { let v: u32 = 0x1095da5u32; cpu.w32(0x11eca44u32, v); }
  { let v: u32 = 0x1095a9eu32; cpu.w32(0x11eca5cu32, v); }
  { let v: u32 = 0x1095a82u32; cpu.w32(0x11eca3cu32, v); }
  { let v: u32 = 0x1095a3au32; cpu.w32(0x11eca2cu32, v); }
  { let v: u32 = 0x1096358u32; cpu.w32(0x11eca30u32, v); }
  { let v: u32 = 0x1095abau32; cpu.w32(0x11eca38u32, v); }
  { let v: u32 = 0x1095c03u32; cpu.w32(0x11eca58u32, v); }
  { let v: u32 = 0x1095cccu32; cpu.w32(0x11eca54u32, v); }
  { let v: u32 = 0x1095d34u32; cpu.w32(0x11eca50u32, v); }
  { let v: u32 = 0x1095bd6u32; cpu.w32(0x11eca60u32, v); }
  { let v: u32 = 0x1095e70u32; cpu.w32(0x11eca34u32, v); }
  { let v: u32 = 0x11eca1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10952d6); f_010956ce(cpu); if cpu.returned_to != 0x10952d6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10952e5 => {
  pc = 0x10952ff; continue;
 }
 0x10952e7 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10952f3 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10952ff => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1095308); f_010a4cb0(cpu); if cpu.returned_to != 0x1095308 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.cf { pc = 0x109532d; continue; }
  pc = 0x1095314; continue;
 }
 0x1095314 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10953da; continue; }
  pc = 0x109531e; continue;
 }
 0x109531e => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if cpu.zf { pc = 0x10953e3; continue; }
  pc = 0x1095328; continue;
 }
 0x1095328 => {
  pc = 0x10953ec; continue;
 }
 0x109532d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x10953ec; continue; }
  pc = 0x1095337; continue;
 }
 0x1095337 => {
  { let v: u32 = 0x5u32; cpu.w32(0x11eca4cu32, v); }
  { let v: u32 = 0x10a54f4u32; cpu.w32(0x11eca48u32, v); }
  { let v: u32 = 0x10a5674u32; cpu.w32(0x11eca40u32, v); }
  { let v: u32 = 0x10a536du32; cpu.w32(0x11eca44u32, v); }
  { let v: u32 = 0x10a4f90u32; cpu.w32(0x11eca5cu32, v); }
  { let v: u32 = 0x10a4f74u32; cpu.w32(0x11eca3cu32, v); }
  { let v: u32 = 0x10a4f0au32; cpu.w32(0x11eca2cu32, v); }
  { let v: u32 = 0x10a57b6u32; cpu.w32(0x11eca30u32, v); }
  { let v: u32 = 0x10a4facu32; cpu.w32(0x11eca38u32, v); }
  { let v: u32 = 0x10a50f5u32; cpu.w32(0x11eca58u32, v); }
  { let v: u32 = 0x10a51beu32; cpu.w32(0x11eca54u32, v); }
  { let v: u32 = 0x10a5226u32; cpu.w32(0x11eca50u32, v); }
  { let v: u32 = 0x10a50c8u32; cpu.w32(0x11eca60u32, v); }
  { let v: u32 = 0x10a5438u32; cpu.w32(0x11eca34u32, v); }
  { let v: u32 = 0x11eca1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10953ce); f_010956ce(cpu); if cpu.returned_to != 0x10953ce { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10953da => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10953e3 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10953ec => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10953f3; continue;
 }
 0x10953f3 => {
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
pub fn f_010953fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10953fc); }
 let mut pc: u32 = 0x10953fc;
 loop { match pc {
 0x10953fc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x11eca2cu32); cpu.push(0x109540d); cpu.call(t); if cpu.returned_to != 0x109540d { return; }
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
pub fn f_01095419(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095419); }
 let mut pc: u32 = 0x1095419;
 loop { match pc {
 0x1095419 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca48u32); cpu.push(0x109543e); cpu.call(t); if cpu.returned_to != 0x109543e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
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
pub fn f_0109544d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109544d); }
 let mut pc: u32 = 0x109544d;
 loop { match pc {
 0x109544d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca40u32); cpu.push(0x1095472); cpu.call(t); if cpu.returned_to != 0x1095472 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
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
pub fn f_01095481(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095481); }
 let mut pc: u32 = 0x1095481;
 loop { match pc {
 0x1095481 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca44u32); cpu.push(0x10954a2); cpu.call(t); if cpu.returned_to != 0x10954a2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
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
pub fn f_010954b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10954b1); }
 let mut pc: u32 = 0x10954b1;
 loop { match pc {
 0x10954b1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x11eca5cu32); cpu.push(0x10954c2); cpu.call(t); if cpu.returned_to != 0x10954c2 { return; }
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
pub fn f_010954ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10954ce); }
 let mut pc: u32 = 0x10954ce;
 loop { match pc {
 0x10954ce => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x11eca3cu32); cpu.push(0x10954df); cpu.call(t); if cpu.returned_to != 0x10954df { return; }
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
pub fn f_010954eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10954eb); }
 let mut pc: u32 = 0x10954eb;
 loop { match pc {
 0x10954eb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  pc = 0x10954f6; continue;
 }
 0x10954f6 => {
  cpu.push(0x10954fb); f_010956b1(cpu); if cpu.returned_to != 0x10954fb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10954f6; continue; }
  pc = 0x10954ff; continue;
 }
 0x10954ff => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2u32; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109551b; continue;
 }
 0x109551b => {
  cpu.push(0x1095520); f_010a5fb3(cpu); if cpu.returned_to != 0x1095520 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109552c; continue; }
  pc = 0x1095524; continue;
 }
 0x1095524 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if cpu.cf { pc = 0x1095531; continue; }
  pc = 0x109552c; continue;
 }
 0x109552c => {
  pc = 0x10955b1; continue;
 }
 0x1095531 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1095538); f_0109d6d2(cpu); if cpu.returned_to != 0x1095538 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1095546); f_010a60df(cpu); if cpu.returned_to != 0x1095546 { return; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1095576; continue; }
  pc = 0x109554e; continue;
 }
 0x109554e => {
  { let v: u32 = 0x11389a8u32; cpu.eax = v; }
  cpu.push(0x1095558); f_010a610a(cpu); if cpu.returned_to != 0x1095558 { return; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x11u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = 0x11eca1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1095573); f_01095419(cpu); if cpu.returned_to != 0x1095573 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x1095576; continue;
 }
 0x1095576 => {
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1095587); f_01095481(cpu); if cpu.returned_to != 0x1095587 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10955ac; continue; }
  pc = 0x109558e; continue;
 }
 0x109558e => {
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if !cpu.zf { pc = 0x10955aa; continue; }
  pc = 0x1095594; continue;
 }
 0x1095594 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109559d); f_01095630(cpu); if cpu.returned_to != 0x109559d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10955aa; continue; }
  pc = 0x10955a4; continue;
 }
 0x10955a4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10955aa; continue;
 }
 0x10955aa => {
  cpu.poll(); pc = 0x1095576; continue;
 }
 0x10955ac => {
  cpu.poll(); pc = 0x109551b; continue;
 }
 0x10955b1 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x109560a; continue; }
  pc = 0x10955b9; continue;
 }
 0x10955b9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10955c0); f_0109d6d2(cpu); if cpu.returned_to != 0x10955c0 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10955ce); f_010a60df(cpu); if cpu.returned_to != 0x10955ce { return; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1095604; continue; }
  pc = 0x10955d6; continue;
 }
 0x10955d6 => {
  { let v: u32 = 0x11389a8u32; cpu.eax = v; }
  cpu.push(0x10955e0); f_010a610a(cpu); if cpu.returned_to != 0x10955e0 { return; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x11u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = 0x11eca1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10955fb); f_01095419(cpu); if cpu.returned_to != 0x10955fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095604; continue;
 }
 0x1095604 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10955b9; continue; }
  pc = 0x109560a; continue;
 }
 0x109560a => {
  cpu.push(0x109560f); f_0109569c(cpu); if cpu.returned_to != 0x109560f { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x1095620; continue; }
  pc = 0x1095617; continue;
 }
 0x1095617 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095627; continue;
 }
 0x1095620 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095627; continue;
 }
 0x1095627 => {
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
pub fn f_01095630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095630); }
 let mut pc: u32 = 0x1095630;
 loop { match pc {
 0x1095630 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca58u32); cpu.push(0x1095645); cpu.call(t); if cpu.returned_to != 0x1095645 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_01095654(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095654); }
 let mut pc: u32 = 0x1095654;
 loop { match pc {
 0x1095654 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca50u32); cpu.push(0x1095669); cpu.call(t); if cpu.returned_to != 0x1095669 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_0109569c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109569c); }
 let mut pc: u32 = 0x109569c;
 loop { match pc {
 0x109569c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  let t = cpu.r32(0x11eca38u32); cpu.push(0x10956ad); cpu.call(t); if cpu.returned_to != 0x10956ad { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010956b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10956b1); }
 let mut pc: u32 = 0x10956b1;
 loop { match pc {
 0x10956b1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x11eca30u32); cpu.push(0x10956c2); cpu.call(t); if cpu.returned_to != 0x10956c2 { return; }
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
pub fn f_010956ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10956ce); }
 let mut pc: u32 = 0x10956ce;
 loop { match pc {
 0x10956ce => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca60u32); cpu.push(0x10956e3); cpu.call(t); if cpu.returned_to != 0x10956e3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_010956f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10956f2); }
 let mut pc: u32 = 0x10956f2;
 loop { match pc {
 0x10956f2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x3cu32); cpu.esp = v; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x109570c); f_0108f030(cpu); if cpu.returned_to != 0x109570c { return; }
  { let v: u16 = 0x100u16; cpu.w16(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x1095734); f_010a6120(cpu); if cpu.returned_to != 0x1095734 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_01095774(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095774); }
 let mut pc: u32 = 0x1095774;
 loop { match pc {
 0x1095774 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x109578e); f_0108f030(cpu); if cpu.returned_to != 0x109578e { return; }
  { let v: u16 = 0x101u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10957ae); f_010a6120(cpu); if cpu.returned_to != 0x10957ae { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
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
pub fn f_010957c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10957c1); }
 let mut pc: u32 = 0x10957c1;
 loop { match pc {
 0x10957c1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11eca4cu32); cpu.eax = v; }
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
pub fn f_010957dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10957dd); }
 let mut pc: u32 = 0x10957dd;
 loop { match pc {
 0x10957dd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10957f6); f_0108ef05(cpu); if cpu.returned_to != 0x10957f6 { return; }
  cpu.push(0x10957fb); f_010956b1(cpu); if cpu.returned_to != 0x10957fb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010957ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10957ff); }
 let mut pc: u32 = 0x10957ff;
 loop { match pc {
 0x10957ff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11eca34u32); cpu.push(0x109581c); cpu.call(t); if cpu.returned_to != 0x109581c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_0109582b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109582b); } r_0109582b(cpu, 0x109582b); }
pub fn f_0109589c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109589c); } r_0109582b(cpu, 0x109589c); }
pub fn f_010958cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10958cb); } r_0109582b(cpu, 0x10958cb); }
fn r_0109582b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x109582b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  cpu.flat_ds(0x1095836); cpu.push(0x109583b); f_01096505(cpu); if cpu.returned_to != 0x109583b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109584b; continue; }
  pc = 0x109583f; continue;
 }
 0x109583f => {
  { let v: u32 = 0x2u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095a31; continue;
 }
 0x109584b => {
  { let v: u32 = 0x0u32; cpu.w32(0x11edaa8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edb08u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb08u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1095877); cpu.push(0x109587c); f_010965a3(cpu); if cpu.returned_to != 0x109587c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109588f; continue; }
  pc = 0x1095883; continue;
 }
 0x1095883 => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095a31; continue;
 }
 0x109588f => {
  { let v: u32 = 0x11edafcu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11edb04u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11edab8u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1018u32; cpu.push(v); }
  cpu.flat_ds(0x10958a6); cpu.push(0x10958ab); f_010956f2(cpu); if cpu.returned_to != 0x10958ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10958be; continue; }
  pc = 0x10958b2; continue;
 }
 0x109589c => {
  { let v: u32 = 0x50011edau32; cpu.eax = v; }
  { let v = 0x1018u32; cpu.push(v); }
  cpu.flat_ds(0x10958a6); cpu.push(0x10958ab); f_010956f2(cpu); if cpu.returned_to != 0x10958ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10958be; continue; }
  pc = 0x10958b2; continue;
 }
 0x10958b2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095a31; continue;
 }
 0x10958be => {
  { let v: u32 = 0x11edac0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11edb00u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11edab4u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1018u32; cpu.push(v); }
  cpu.flat_ds(0x10958d5); cpu.push(0x10958da); f_010956f2(cpu); if cpu.returned_to != 0x10958da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10958ed; continue; }
  pc = 0x10958e1; continue;
 }
 0x10958cb => {
  { let v: u8 = 0xdau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0x68u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), cpu.edx); cpu.w32(0x68u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x10958d5); cpu.push(0x10958da); f_010956f2(cpu); if cpu.returned_to != 0x10958da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10958ed; continue; }
  pc = 0x10958e1; continue;
 }
 0x10958e1 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095a31; continue;
 }
 0x10958ed => {
  { let v: u32 = cpu.r32(0x11edab8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edab0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x11edab4u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edaacu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x11eda94u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1095907); cpu.push(0x109590c); f_01096a66(cpu); if cpu.returned_to != 0x109590c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x11eda94u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1095915); cpu.push(0x109591a); f_01096b02(cpu); if cpu.returned_to != 0x109591a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = 0x11eda84u32; cpu.eax = v; }
  cpu.flat_ds(0x109592c); cpu.push(0x1095931); f_0108ef05(cpu); if cpu.returned_to != 0x1095931 { return; }
  { let v: u32 = 0x11eda84u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x1095937); cpu.push(0x109593c); f_01095c03(cpu); if cpu.returned_to != 0x109593c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11edabcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edaa4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095966; continue;
 }
 0x1095966 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x1095976; continue; }
  pc = 0x109596c; continue;
 }
 0x109596c => {
  pc = 0x10959a8; continue;
 }
 0x109596e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095966; continue;
 }
 0x1095976 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ed304u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ed306u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11ed356u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0xceu32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x109596e; continue;
 }
 0x10959a8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10959af; continue;
 }
 0x10959af => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10959bf; continue; }
  pc = 0x10959b5; continue;
 }
 0x10959b5 => {
  pc = 0x10959f1; continue;
 }
 0x10959b7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10959af; continue;
 }
 0x10959bf => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ecb84u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ecb86u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11ecbd6u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0xceu32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10959b7; continue;
 }
 0x10959f1 => {
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x11ecb7eu32; cpu.eax = v; }
  cpu.flat_ds(0x1095a00); cpu.push(0x1095a05); f_0108f030(cpu); if cpu.returned_to != 0x1095a05 { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x11ecb78u32; cpu.eax = v; }
  cpu.flat_ds(0x1095a14); cpu.push(0x1095a19); f_0108f030(cpu); if cpu.returned_to != 0x1095a19 { return; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x11ecb74u32; cpu.eax = v; }
  cpu.flat_ds(0x1095a25); cpu.push(0x1095a2a); f_0108f030(cpu); if cpu.returned_to != 0x1095a2a { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095a31; continue;
 }
 0x1095a31 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x1095a39); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01095a3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095a3a); }
 let mut pc: u32 = 0x1095a3a;
 loop { match pc {
 0x1095a3a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1095a53); f_01096638(cpu); if cpu.returned_to != 0x1095a53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x11edafcu32); cpu.push(v); }
  cpu.push(0x1095a61); f_01095774(cpu); if cpu.returned_to != 0x1095a61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x11edac0u32); cpu.push(v); }
  cpu.push(0x1095a6f); f_01095774(cpu); if cpu.returned_to != 0x1095a6f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_01095a82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095a82); }
 let mut pc: u32 = 0x1095a82;
 loop { match pc {
 0x1095a82 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11edaa8u32); cpu.eax = v; }
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
pub fn f_01095a9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095a9e); }
 let mut pc: u32 = 0x1095a9e;
 loop { match pc {
 0x1095a9e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11edac4u32); cpu.eax = v; }
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
pub fn f_01095aba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095aba); }
 let mut pc: u32 = 0x1095aba;
 loop { match pc {
 0x1095aba => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  pc = 0x1095ac5; continue;
 }
 0x1095ac5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095ad3; continue;
 }
 0x1095ad3 => {
  { let v: u32 = cpu.r32(0x11edaa8u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1095aeb; continue; }
  pc = 0x1095ade; continue;
 }
 0x1095ade => {
  pc = 0x1095b80; continue;
 }
 0x1095ae3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095ad3; continue;
 }
 0x1095aeb => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095b12); f_010a621e(cpu); if cpu.returned_to != 0x1095b12 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1095b7b; continue; }
  pc = 0x1095b16; continue;
 }
 0x1095b16 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1095b37); f_0108ef05(cpu); if cpu.returned_to != 0x1095b37 { return; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095b5e); f_0108ef05(cpu); if cpu.returned_to != 0x1095b5e { return; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095b7b); f_0108ef05(cpu); if cpu.returned_to != 0x1095b7b { return; }
  pc = 0x1095b7b; continue;
 }
 0x1095b7b => {
  cpu.poll(); pc = 0x1095ae3; continue;
 }
 0x1095b80 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1095ac5; continue; }
  pc = 0x1095b8a; continue;
 }
 0x1095b8a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095b91; continue;
 }
 0x1095b91 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa8u32)); }
  if cpu.cf { pc = 0x1095ba6; continue; }
  pc = 0x1095b9c; continue;
 }
 0x1095b9c => {
  pc = 0x1095bd0; continue;
 }
 0x1095b9e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095b91; continue;
 }
 0x1095ba6 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095bc2); f_010a621e(cpu); if cpu.returned_to != 0x1095bc2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1095bce; continue; }
  pc = 0x1095bc6; continue;
 }
 0x1095bc6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edac4u32, v); }
  pc = 0x1095bce; continue;
 }
 0x1095bce => {
  cpu.poll(); pc = 0x1095b9e; continue;
 }
 0x1095bd0 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01095bd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095bd6); }
 let mut pc: u32 = 0x1095bd6;
 loop { match pc {
 0x1095bd6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1095bf3); f_0108ef05(cpu); if cpu.returned_to != 0x1095bf3 { return; }
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
pub fn f_01095c03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095c03); }
 let mut pc: u32 = 0x1095c03;
 loop { match pc {
 0x1095c03 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11edaa8u32), 0x0u32); }
  if !cpu.zf { pc = 0x1095c3b; continue; }
  pc = 0x1095c17; continue;
 }
 0x1095c17 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  cpu.push(0x1095c29); f_0108ef05(cpu); if cpu.returned_to != 0x1095c29 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11edaa8u32)); cpu.w32(0x11edaa8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095cc3; continue;
 }
 0x1095c3b => {
  { let v: u32 = cpu.r32(0x11edaa8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa4u32)); }
  if !cpu.zf { pc = 0x1095c54; continue; }
  pc = 0x1095c48; continue;
 }
 0x1095c48 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095cc3; continue;
 }
 0x1095c54 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095c5b; continue;
 }
 0x1095c5b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa8u32)); }
  if cpu.cf { pc = 0x1095c70; continue; }
  pc = 0x1095c66; continue;
 }
 0x1095c66 => {
  pc = 0x1095c99; continue;
 }
 0x1095c68 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095c5b; continue;
 }
 0x1095c70 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095c8a); f_010a621e(cpu); if cpu.returned_to != 0x1095c8a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1095c97; continue; }
  pc = 0x1095c8e; continue;
 }
 0x1095c8e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095cc3; continue;
 }
 0x1095c97 => {
  cpu.poll(); pc = 0x1095c68; continue;
 }
 0x1095c99 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edaa8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095cb6); f_0108ef05(cpu); if cpu.returned_to != 0x1095cb6 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11edaa8u32)); cpu.w32(0x11edaa8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095cc3; continue;
 }
 0x1095cc3 => {
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
pub fn f_01095ccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095ccc); }
 let mut pc: u32 = 0x1095ccc;
 loop { match pc {
 0x1095ccc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa8u32)); }
  if cpu.cf { pc = 0x1095ceb; continue; }
  pc = 0x1095ce2; continue;
 }
 0x1095ce2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d2b; continue;
 }
 0x1095ceb => {
  { let v: u32 = cpu.r32(0x11edaa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = 0x11eca64u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095d1e); f_0108ef05(cpu); if cpu.returned_to != 0x1095d1e { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11edaa8u32)); cpu.w32(0x11edaa8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d2b; continue;
 }
 0x1095d2b => {
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
pub fn f_01095d34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095d34); }
 let mut pc: u32 = 0x1095d34;
 loop { match pc {
 0x1095d34 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11edaa8u32), 0x0u32); }
  if !cpu.zf { pc = 0x1095d51; continue; }
  pc = 0x1095d48; continue;
 }
 0x1095d48 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d9c; continue;
 }
 0x1095d51 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d58; continue;
 }
 0x1095d58 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa8u32)); }
  if cpu.cf { pc = 0x1095d6d; continue; }
  pc = 0x1095d63; continue;
 }
 0x1095d63 => {
  pc = 0x1095d95; continue;
 }
 0x1095d65 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095d58; continue;
 }
 0x1095d6d => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1095d87); f_010a621e(cpu); if cpu.returned_to != 0x1095d87 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1095d93; continue; }
  pc = 0x1095d8b; continue;
 }
 0x1095d8b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d9c; continue;
 }
 0x1095d93 => {
  cpu.poll(); pc = 0x1095d65; continue;
 }
 0x1095d95 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095d9c; continue;
 }
 0x1095d9c => {
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
pub fn f_01095da5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095da5); }
 let mut pc: u32 = 0x1095da5;
 loop { match pc {
 0x1095da5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095dc5; continue;
 }
 0x1095dc5 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x1095dd5; continue; }
  pc = 0x1095dcb; continue;
 }
 0x1095dcb => {
  pc = 0x1095e07; continue;
 }
 0x1095dcd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095dc5; continue;
 }
 0x1095dd5 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ecbd6u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1095e05; continue; }
  pc = 0x1095deb; continue;
 }
 0x1095deb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x1095e05; continue; }
  pc = 0x1095df6; continue;
 }
 0x1095df6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095e05; continue;
 }
 0x1095e05 => {
  cpu.poll(); pc = 0x1095dcd; continue;
 }
 0x1095e07 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1095e16; continue; }
  pc = 0x1095e0d; continue;
 }
 0x1095e0d => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095e67; continue;
 }
 0x1095e16 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ecbd6u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1095e34); f_0108ef05(cpu); if cpu.returned_to != 0x1095e34 { return; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1095e4e; continue; }
  pc = 0x1095e3a; continue;
 }
 0x1095e3a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1095e4e); f_0108ef05(cpu); if cpu.returned_to != 0x1095e4e { return; }
  pc = 0x1095e4e; continue;
 }
 0x1095e4e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ecb84u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1095e60); f_01096358(cpu); if cpu.returned_to != 0x1095e60 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095e67; continue;
 }
 0x1095e67 => {
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
pub fn f_01095e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095e70); }
 let mut pc: u32 = 0x1095e70;
 loop { match pc {
 0x1095e70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095e90; continue;
 }
 0x1095e90 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x1095ea0; continue; }
  pc = 0x1095e96; continue;
 }
 0x1095e96 => {
  pc = 0x1095ed2; continue;
 }
 0x1095e98 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095e90; continue;
 }
 0x1095ea0 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ecbd6u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1095ed0; continue; }
  pc = 0x1095eb6; continue;
 }
 0x1095eb6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x1095ed0; continue; }
  pc = 0x1095ec1; continue;
 }
 0x1095ec1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095ed0; continue;
 }
 0x1095ed0 => {
  cpu.poll(); pc = 0x1095e98; continue;
 }
 0x1095ed2 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1095ee1; continue; }
  pc = 0x1095ed8; continue;
 }
 0x1095ed8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095f23; continue;
 }
 0x1095ee1 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ecbd6u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1095eff); f_0108ef05(cpu); if cpu.returned_to != 0x1095eff { return; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x11ecb84u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095f23; continue;
 }
 0x1095f23 => {
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
pub fn f_01095f2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1095f2c); }
 let mut pc: u32 = 0x1095f2c;
 loop { match pc {
 0x1095f2c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edaa8u32)); }
  if cpu.cf { pc = 0x1095f48; continue; }
  pc = 0x1095f42; continue;
 }
 0x1095f42 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x11u32); }
  if !cpu.zf { pc = 0x1095f4a; continue; }
  pc = 0x1095f48; continue;
 }
 0x1095f48 => {
  pc = 0x1095f56; continue;
 }
 0x1095f4a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1096152; continue;
 }
 0x1095f56 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095f5d; continue;
 }
 0x1095f5d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x1095f6d; continue; }
  pc = 0x1095f63; continue;
 }
 0x1095f63 => {
  pc = 0x1095f7a; continue;
 }
 0x1095f65 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095f5d; continue;
 }
 0x1095f6d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11ed305u32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1095f65; continue; }
  pc = 0x1095f7a; continue;
 }
 0x1095f7a => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x1095fb9; continue; }
  pc = 0x1095f80; continue;
 }
 0x1095f80 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1095f87; continue;
 }
 0x1095f87 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x1095f97; continue; }
  pc = 0x1095f8d; continue;
 }
 0x1095f8d => {
  pc = 0x1095fa7; continue;
 }
 0x1095f8f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1095f87; continue;
 }
 0x1095f97 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ed356u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1095f8f; continue; }
  pc = 0x1095fa7; continue;
 }
 0x1095fa7 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x1095fb9; continue; }
  pc = 0x1095fad; continue;
 }
 0x1095fad => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1096152; continue;
 }
 0x1095fb9 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.edx = v; }
  { let v: u32 = 0x11ed304u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edab8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11edab0u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edb0au32, v); }
  { let v: u32 = 0x1eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x1096004); f_0108f030(cpu); if cpu.returned_to != 0x1096004 { return; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  cpu.push(0x1096016); f_0108f030(cpu); if cpu.returned_to != 0x1096016 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.push(0x1096057); f_0108ef05(cpu); if cpu.returned_to != 0x1096057 { return; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x16u32); cpu.eax = v; }
  cpu.push(0x109606c); f_0108ef05(cpu); if cpu.returned_to != 0x109606c { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  cpu.push(0x109608c); f_0108ef05(cpu); if cpu.returned_to != 0x109608c { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda98u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1au32); cpu.eax = v; }
  cpu.push(0x10960a1); f_0108ef05(cpu); if cpu.returned_to != 0x10960a1 { return; }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x46u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x1eu16; cpu.w16(0x4au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x11eca64u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3eu32); cpu.eax = v; }
  cpu.push(0x1096107); f_0108ef05(cpu); if cpu.returned_to != 0x1096107 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11edabcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11edabcu32)); cpu.w32(0x11edabcu32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x52u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1096139); f_010968ea(cpu); if cpu.returned_to != 0x1096139 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x56u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1096152; continue;
 }
 0x1096152 => {
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
pub fn f_0109615b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109615b); }
 let mut pc: u32 = 0x109615b;
 loop { match pc {
 0x109615b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x109616d; continue;
 }
 0x109616d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x109617d; continue; }
  pc = 0x1096173; continue;
 }
 0x1096173 => {
  pc = 0x109618a; continue;
 }
 0x1096175 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x109616d; continue;
 }
 0x109617d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11ed305u32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1096175; continue; }
  pc = 0x109618a; continue;
 }
 0x109618a => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10961c9; continue; }
  pc = 0x1096190; continue;
 }
 0x1096190 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1096197; continue;
 }
 0x1096197 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x10961a7; continue; }
  pc = 0x109619d; continue;
 }
 0x109619d => {
  pc = 0x10961b7; continue;
 }
 0x109619f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1096197; continue;
 }
 0x10961a7 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ed356u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x109619f; continue; }
  pc = 0x10961b7; continue;
 }
 0x10961b7 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x10961c9; continue; }
  pc = 0x10961bd; continue;
 }
 0x10961bd => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109634f; continue;
 }
 0x10961c9 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { let v: u32 = 0x11ed304u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x9000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edab8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11edab0u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edb0au32, v); }
  { let v: u32 = 0x1eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x1096215); f_0108f030(cpu); if cpu.returned_to != 0x1096215 { return; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  cpu.push(0x1096227); f_0108f030(cpu); if cpu.returned_to != 0x1096227 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.push(0x1096268); f_0108ef05(cpu); if cpu.returned_to != 0x1096268 { return; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x16u32); cpu.eax = v; }
  cpu.push(0x109627d); f_0108ef05(cpu); if cpu.returned_to != 0x109627d { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  cpu.push(0x1096293); f_0108ef05(cpu); if cpu.returned_to != 0x1096293 { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda98u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1au32); cpu.eax = v; }
  cpu.push(0x10962a8); f_0108ef05(cpu); if cpu.returned_to != 0x10962a8 { return; }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x46u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x1eu16; cpu.w16(0x4au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3eu32); cpu.eax = v; }
  cpu.push(0x1096304); f_0108ef05(cpu); if cpu.returned_to != 0x1096304 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11edabcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11edabcu32)); cpu.w32(0x11edabcu32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x52u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1096336); f_010968ea(cpu); if cpu.returned_to != 0x1096336 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x56u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109634f; continue;
 }
 0x109634f => {
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
pub fn f_01096358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096358); }
 let mut pc: u32 = 0x1096358;
 loop { match pc {
 0x1096358 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x109636a; continue;
 }
 0x109636a => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.cf { pc = 0x109637a; continue; }
  pc = 0x1096370; continue;
 }
 0x1096370 => {
  pc = 0x1096387; continue;
 }
 0x1096372 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x109636a; continue;
 }
 0x109637a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x11ecb85u32.wrapping_add(cpu.eax)), 0x80u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1096372; continue; }
  pc = 0x1096387; continue;
 }
 0x1096387 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x14u32); }
  if !cpu.zf { pc = 0x1096399; continue; }
  pc = 0x109638d; continue;
 }
 0x109638d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10964fc; continue;
 }
 0x1096399 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x60u32); cpu.edx = v; }
  { let v: u32 = 0x11ecb84u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xa000u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edab4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edaacu32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edb0au32, v); }
  { let v: u32 = 0x1eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x10963e3); f_0108f030(cpu); if cpu.returned_to != 0x10963e3 { return; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  cpu.push(0x10963f5); f_0108f030(cpu); if cpu.returned_to != 0x10963f5 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.push(0x1096436); f_0108ef05(cpu); if cpu.returned_to != 0x1096436 { return; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x16u32); cpu.eax = v; }
  cpu.push(0x109644b); f_0108ef05(cpu); if cpu.returned_to != 0x109644b { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  cpu.push(0x1096460); f_0108f030(cpu); if cpu.returned_to != 0x1096460 { return; }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0x11eda98u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1au32); cpu.eax = v; }
  cpu.push(0x1096475); f_0108ef05(cpu); if cpu.returned_to != 0x1096475 { return; }
  { let v: u16 = cpu.r16(0x11edb08u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x46u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x1eu16; cpu.w16(0x4au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x80u16; cpu.w16(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3eu32); cpu.eax = v; }
  cpu.push(0x10964d0); f_0108f030(cpu); if cpu.returned_to != 0x10964d0 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x52u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10964e3); f_01096730(cpu); if cpu.returned_to != 0x10964e3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x56u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10964fc; continue;
 }
 0x10964fc => {
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
pub fn f_01096505(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096505); }
 let mut pc: u32 = 0x1096505;
 loop { match pc {
 0x1096505 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x1096521); f_0108f030(cpu); if cpu.returned_to != 0x1096521 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1096530); f_0108f030(cpu); if cpu.returned_to != 0x1096530 { return; }
  { let v: u32 = 0x1cu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x109653f); f_0108f030(cpu); if cpu.returned_to != 0x109653f { return; }
  { let v: u32 = 0x7a00u32; cpu.w32(0x11edae4u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2fu16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096560); f_010a619b(cpu); if cpu.returned_to != 0x1096560 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x109657e); f_010a6120(cpu); if cpu.returned_to != 0x109657e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x11edae4u32), 0xffu8); }
  if cpu.zf { pc = 0x1096593; continue; }
  pc = 0x109658a; continue;
 }
 0x109658a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x109659a; continue;
 }
 0x1096593 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x109659a; continue;
 }
 0x109659a => {
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
pub fn f_010965a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10965a3); }
 let mut pc: u32 = 0x10965a3;
 loop { match pc {
 0x10965a3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x10965bf); f_0108f030(cpu); if cpu.returned_to != 0x10965bf { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10965ce); f_0108f030(cpu); if cpu.returned_to != 0x10965ce { return; }
  { let v: u32 = 0xffu32; cpu.w32(0x11edae4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11edad8u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edadcu32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096601); f_010a619b(cpu); if cpu.returned_to != 0x1096601 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x109661f); f_010a6120(cpu); if cpu.returned_to != 0x109661f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x11edae4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
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
pub fn f_01096638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096638); }
 let mut pc: u32 = 0x1096638;
 loop { match pc {
 0x1096638 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x1096654); f_0108f030(cpu); if cpu.returned_to != 0x1096654 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1096663); f_0108f030(cpu); if cpu.returned_to != 0x1096663 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x11edad8u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edadcu32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x109668c); f_010a619b(cpu); if cpu.returned_to != 0x109668c { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x10966aa); f_010a6120(cpu); if cpu.returned_to != 0x10966aa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
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
pub fn f_010966bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10966bd); }
 let mut pc: u32 = 0x10966bd;
 loop { match pc {
 0x10966bd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x10966d9); f_0108f030(cpu); if cpu.returned_to != 0x10966d9 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10966e8); f_0108f030(cpu); if cpu.returned_to != 0x10966e8 { return; }
  { let v: u32 = 0xau32; cpu.w32(0x11edad8u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096709); f_010a619b(cpu); if cpu.returned_to != 0x1096709 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x1096727); f_010a6120(cpu); if cpu.returned_to != 0x1096727 { return; }
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
pub fn f_01096730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096730); }
 let mut pc: u32 = 0x1096730;
 loop { match pc {
 0x1096730 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096766); f_0108ef05(cpu); if cpu.returned_to != 0x1096766 { return; }
  { let v: u32 = 0x1eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096775); f_0108f030(cpu); if cpu.returned_to != 0x1096775 { return; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb00u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x26u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb00u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x10967c4); f_0108f030(cpu); if cpu.returned_to != 0x10967c4 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10967d3); f_0108f030(cpu); if cpu.returned_to != 0x10967d3 { return; }
  { let v: u32 = 0x4u32; cpu.w32(0x11edad8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edaccu32, v); }
  { let v: u16 = cpu.r16(0x11edb00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edaeau32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x109680d); f_010a619b(cpu); if cpu.returned_to != 0x109680d { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x109682b); f_010a6120(cpu); if cpu.returned_to != 0x109682b { return; }
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
pub fn f_01096834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096834); }
 let mut pc: u32 = 0x1096834;
 loop { match pc {
 0x1096834 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10968b3; continue; }
  pc = 0x109685b; continue;
 }
 0x109685b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096879); f_0108ef05(cpu); if cpu.returned_to != 0x1096879 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10968b1; continue; }
  pc = 0x1096883; continue;
 }
 0x1096883 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10968b1); f_0108ef05(cpu); if cpu.returned_to != 0x10968b1 { return; }
  pc = 0x10968b1; continue;
 }
 0x10968b1 => {
  pc = 0x10968e4; continue;
 }
 0x10968b3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10968e4; continue; }
  pc = 0x10968bd; continue;
 }
 0x10968bd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10968e4); f_0108ef05(cpu); if cpu.returned_to != 0x10968e4 { return; }
  pc = 0x10968e4; continue;
 }
 0x10968e4 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010968ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10968ea); }
 let mut pc: u32 = 0x10968ea;
 loop { match pc {
 0x10968ea => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11edab0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11edab0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11edab0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x109692b); f_0108ef05(cpu); if cpu.returned_to != 0x109692b { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x28u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096942); f_0108ef05(cpu); if cpu.returned_to != 0x1096942 { return; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1096986; continue; }
  pc = 0x1096948; continue;
 }
 0x1096948 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096956); f_0108ef05(cpu); if cpu.returned_to != 0x1096956 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x109697a; continue; }
  pc = 0x1096960; continue;
 }
 0x1096960 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.push(0x109697a); f_0108ef05(cpu); if cpu.returned_to != 0x109697a { return; }
  pc = 0x109697a; continue;
 }
 0x109697a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.edx), v); }
  pc = 0x10969a7; continue;
 }
 0x1096986 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10969a7; continue; }
  pc = 0x1096990; continue;
 }
 0x1096990 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10969a7); f_0108ef05(cpu); if cpu.returned_to != 0x10969a7 { return; }
  pc = 0x10969a7; continue;
 }
 0x10969a7 => {
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb04u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x26u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4eu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x11edb04u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x10969f6); f_0108f030(cpu); if cpu.returned_to != 0x10969f6 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1096a05); f_0108f030(cpu); if cpu.returned_to != 0x1096a05 { return; }
  { let v: u32 = 0x3u32; cpu.w32(0x11edad8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11edb0au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edaccu32, v); }
  { let v: u16 = cpu.r16(0x11edb04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edaeau32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096a3f); f_010a619b(cpu); if cpu.returned_to != 0x1096a3f { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x1096a5d); f_010a6120(cpu); if cpu.returned_to != 0x1096a5d { return; }
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
pub fn f_01096a66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096a66); }
 let mut pc: u32 = 0x1096a66;
 loop { match pc {
 0x1096a66 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x1096a82); f_0108f030(cpu); if cpu.returned_to != 0x1096a82 { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1096a91); f_0108f030(cpu); if cpu.returned_to != 0x1096a91 { return; }
  { let v: u16 = cpu.r16(0x11edb00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edaeau32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11edaccu32, v); }
  { let v: u32 = 0x9u32; cpu.w32(0x11edad8u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096ac8); f_010a619b(cpu); if cpu.returned_to != 0x1096ac8 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x1096ae6); f_010a6120(cpu); if cpu.returned_to != 0x1096ae6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096afc); f_0108ef05(cpu); if cpu.returned_to != 0x1096afc { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096b02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096b02); }
 let mut pc: u32 = 0x1096b02;
 loop { match pc {
 0x1096b02 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x34u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11edac8u32; cpu.eax = v; }
  cpu.push(0x1096b1e); f_0108f030(cpu); if cpu.returned_to != 0x1096b1e { return; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1096b2d); f_0108f030(cpu); if cpu.returned_to != 0x1096b2d { return; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.eax = v; }
  cpu.push(0x1096b3f); f_0108ef05(cpu); if cpu.returned_to != 0x1096b3f { return; }
  { let v: u16 = cpu.r16(0x11edb00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11edaeau32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11edaccu32, v); }
  { let v: u32 = 0xau32; cpu.w32(0x11edac8u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x11edad8u32, v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x7au16; cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1096b80); f_010a619b(cpu); if cpu.returned_to != 0x1096b80 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11edac8u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  cpu.push(0x1096b9e); f_010a6120(cpu); if cpu.returned_to != 0x1096b9e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edaacu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1096bb4); f_0108ef05(cpu); if cpu.returned_to != 0x1096bb4 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096bba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096bba); }
 let mut pc: u32 = 0x1096bba;
 loop { match pc {
 0x1096bba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096bbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096bbb); }
 let mut pc: u32 = 0x1096bbb;
 loop { match pc {
 0x1096bbb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x1096bc7); f_01094e58(cpu); if cpu.returned_to != 0x1096bc7 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096bcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096bcc); }
 let mut pc: u32 = 0x1096bcc;
 loop { match pc {
 0x1096bcc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xcfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0x30u32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1096bef; continue; }
  pc = 0x1096be8; continue;
 }
 0x1096be8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1096bef); f_0109afc8(cpu); if cpu.returned_to != 0x1096bef { return; }
  pc = 0x1096bef; continue;
 }
 0x1096bef => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x1096c0a; continue; }
  pc = 0x1096bf9; continue;
 }
 0x1096bf9 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x1096c0a; continue;
 }
 0x1096c0a => {
  { let v: u32 = 0x1096bbbu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1096c16); f_01098c9e(cpu); if cpu.returned_to != 0x1096c16 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1096c31; continue; }
  pc = 0x1096c1c; continue;
 }
 0x1096c1c => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1096c31); f_0109aec3(cpu); if cpu.returned_to != 0x1096c31 { return; }
  pc = 0x1096c31; continue;
 }
 0x1096c31 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x1096c3c; continue; }
  pc = 0x1096c37; continue;
 }
 0x1096c37 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x1096c3c; continue;
 }
 0x1096c3c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096c50); }
 let mut pc: u32 = 0x1096c50;
 loop { match pc {
 0x1096c50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1096ccf; continue; }
  pc = 0x1096c75; continue;
 }
 0x1096c75 => {
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096d2e; continue; }
  pc = 0x1096cae; continue;
 }
 0x1096cae => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x1096cb9); f_010a6310(cpu); if cpu.returned_to != 0x1096cb9 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096d2e; continue; }
  pc = 0x1096ccd; continue;
 }
 0x1096ccd => {
  cpu.poll(); pc = 0x1096cae; continue;
 }
 0x1096ccf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096d2e; continue; }
  pc = 0x1096d0f; continue;
 }
 0x1096d0f => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x1096d1a); f_010a6310(cpu); if cpu.returned_to != 0x1096d1a { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096d0f; continue; }
  pc = 0x1096d2e; continue;
 }
 0x1096d2e => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096d38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096d38); }
 let mut pc: u32 = 0x1096d38;
 loop { match pc {
 0x1096d38 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096dad; continue; }
  pc = 0x1096d81; continue;
 }
 0x1096d81 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096d81; continue; }
  pc = 0x1096dad; continue;
 }
 0x1096dad => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096e38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096e38); }
 let mut pc: u32 = 0x1096e38;
 loop { match pc {
 0x1096e38 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096e88; continue; }
  pc = 0x1096e66; continue;
 }
 0x1096e66 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1096e81; continue; }
  pc = 0x1096e6c; continue;
 }
 0x1096e6c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096e6c; continue; }
  pc = 0x1096e81; continue;
 }
 0x1096e81 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096e66; continue; }
  pc = 0x1096e88; continue;
 }
 0x1096e88 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096e90); }
 let mut pc: u32 = 0x1096e90;
 loop { match pc {
 0x1096e90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1096ed8; continue; }
  pc = 0x1096ebe; continue;
 }
 0x1096ebe => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1096ec2; continue;
 }
 0x1096ec2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1096ed1; continue; }
  pc = 0x1096ec8; continue;
 }
 0x1096ec8 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096ec8; continue; }
  pc = 0x1096ed1; continue;
 }
 0x1096ed1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1096ec2; continue; }
  pc = 0x1096ed8; continue;
 }
 0x1096ed8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096ee0); }
 let mut pc: u32 = 0x1096ee0;
 loop { match pc {
 0x1096ee0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1096ee9); f_010982f8(cpu); if cpu.returned_to != 0x1096ee9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb10u32, v); }
  cpu.push(0x1096ef3); f_01097d24(cpu); if cpu.returned_to != 0x1096ef3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1096f03; continue; }
  pc = 0x1096ef7; continue;
 }
 0x1096ef7 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0xb0000u32; cpu.push(v); }
  pc = 0x1096f0d; continue;
 }
 0x1096f03 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0xb8000u32; cpu.push(v); }
  pc = 0x1096f0d; continue;
 }
 0x1096f0d => {
  cpu.push(0x1096f12); f_0109881c(cpu); if cpu.returned_to != 0x1096f12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb0cu32, v); }
  { let v = 0x4au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1096f28); f_01098210(cpu); if cpu.returned_to != 0x1096f28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb34u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb24u32, v); }
  cpu.push(0x1096f4b); f_01098210(cpu); if cpu.returned_to != 0x1096f4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb3cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb2cu32, v); }
  { let v: u32 = cpu.r32(0x11edb24u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb20u32, v); }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11edb18u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb38u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11edb28u32, v); }
  cpu.push(0x1096f89); f_01097ce8(cpu); if cpu.returned_to != 0x1096f89 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1096f90); f_01097c1c(cpu); if cpu.returned_to != 0x1096f90 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x1096f9a); f_01097b64(cpu); if cpu.returned_to != 0x1096f9a { return; }
  { cpu.sub32(cpu.r32(0x1133eb0u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1096fde; continue; }
  pc = 0x1096fa3; continue;
 }
 0x1096fa3 => {
  { let v = 0x49u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1096fb1); f_01098210(cpu); if cpu.returned_to != 0x1096fb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133eb0u32, v); }
  { cpu.sub32(cpu.r32(0x11edb3cu32), 0x2bu32); }
  if !cpu.zf { pc = 0x1096fce; continue; }
  pc = 0x1096fc7; continue;
 }
 0x1096fc7 => {
  { let v: u8 = cpu.r8(0x1133eb1u32) | 0x20u8; cpu.w8(0x1133eb1u32, v); }
  pc = 0x1096fce; continue;
 }
 0x1096fce => {
  { cpu.sub32(cpu.r32(0x11edb3cu32), 0x32u32); }
  if !cpu.zf { pc = 0x1096fde; continue; }
  pc = 0x1096fd7; continue;
 }
 0x1096fd7 => {
  { let v: u8 = cpu.or8(cpu.r8(0x1133eb1u32), 0x40u8); cpu.w8(0x1133eb1u32, v); }
  pc = 0x1096fde; continue;
 }
 0x1096fde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01096fe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1096fe4); }
 let mut pc: u32 = 0x1096fe4;
 loop { match pc {
 0x1096fe4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1133eb0u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1097053; continue; }
  pc = 0x1096ff2; continue;
 }
 0x1096ff2 => {
  cpu.push(0x1096ff7); f_0109705c(cpu); if cpu.returned_to != 0x1096ff7 { return; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x1133eb0u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x1097010); f_010983b4(cpu); if cpu.returned_to != 0x1097010 { return; }
  { let v: u32 = cpu.r32(0x1133eb0u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xdfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x109702e; continue; }
  pc = 0x1097020; continue;
 }
 0x1097020 => {
  { cpu.and8(cpu.r8(0x1133eb1u32), 0x20u8); }
  if cpu.zf { pc = 0x109702e; continue; }
  pc = 0x1097029; continue;
 }
 0x1097029 => {
  cpu.push(0x109702e); f_010970a8(cpu); if cpu.returned_to != 0x109702e { return; }
  pc = 0x109702e; continue;
 }
 0x109702e => {
  { let v: u32 = cpu.r32(0x1133eb0u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x1097049; continue; }
  pc = 0x109703b; continue;
 }
 0x109703b => {
  { cpu.and8(cpu.r8(0x1133eb1u32), 0x40u8); }
  if cpu.zf { pc = 0x1097049; continue; }
  pc = 0x1097044; continue;
 }
 0x1097044 => {
  cpu.push(0x1097049); f_0109712c(cpu); if cpu.returned_to != 0x1097049 { return; }
  pc = 0x1097049; continue;
 }
 0x1097049 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133eb0u32, v); }
  pc = 0x1097053; continue;
 }
 0x1097053 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109705c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109705c); }
 let mut pc: u32 = 0x109705c;
 loop { match pc {
 0x109705c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1202u32; cpu.edx = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097081); f_010983b4(cpu); if cpu.returned_to != 0x1097081 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097099); f_010983b4(cpu); if cpu.returned_to != 0x1097099 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x10970a1); f_01096ee0(cpu); if cpu.returned_to != 0x10970a1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010970a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10970a8); }
 let mut pc: u32 = 0x10970a8;
 loop { match pc {
 0x10970a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  cpu.push(0x10970b4); f_01097d24(cpu); if cpu.returned_to != 0x10970b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10970bc; continue; }
  pc = 0x10970b8; continue;
 }
 0x10970b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1097123; continue;
 }
 0x10970bc => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1201u32; cpu.edx = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10970db); f_010983b4(cpu); if cpu.returned_to != 0x10970db { return; }
  { let v: u16 = cpu.r16(0x1133eb0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x10970f7); f_010983b4(cpu); if cpu.returned_to != 0x10970f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x1112u32; cpu.ecx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097116); f_010983b4(cpu); if cpu.returned_to != 0x1097116 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x109711e); f_01096ee0(cpu); if cpu.returned_to != 0x109711e { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1097123; continue;
 }
 0x1097123 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109712c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109712c); }
 let mut pc: u32 = 0x109712c;
 loop { match pc {
 0x109712c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  cpu.push(0x1097138); f_01097d24(cpu); if cpu.returned_to != 0x1097138 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1097140; continue; }
  pc = 0x109713c; continue;
 }
 0x109713c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10971a7; continue;
 }
 0x1097140 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1202u32; cpu.edx = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x109715f); f_010983b4(cpu); if cpu.returned_to != 0x109715f { return; }
  { let v: u16 = cpu.r16(0x1133eb0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x109717b); f_010983b4(cpu); if cpu.returned_to != 0x109717b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x1112u32; cpu.ecx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x109719a); f_010983b4(cpu); if cpu.returned_to != 0x109719a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x10971a2); f_01096ee0(cpu); if cpu.returned_to != 0x10971a2 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10971a7; continue;
 }
 0x10971a7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010971b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10971b0); }
 let mut pc: u32 = 0x10971b0;
 loop { match pc {
 0x10971b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x11edb40u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  cpu.push(0x10971cc); f_010a6370(cpu); if cpu.returned_to != 0x10971cc { return; }
  { let v: u32 = 0x11edb40u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10971d8); f_01097298(cpu); if cpu.returned_to != 0x10971d8 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010971e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10971e4); }
 let mut pc: u32 = 0x10971e4;
 loop { match pc {
 0x10971e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11edb18u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x109721a; continue;
 }
 0x109721a => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1097233; continue; }
  pc = 0x1097220; continue;
 }
 0x1097220 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e98u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x109721a; continue;
 }
 0x1097233 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097238); }
 let mut pc: u32 = 0x1097238;
 loop { match pc {
 0x1097238 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb1cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1097252); f_01098210(cpu); if cpu.returned_to != 0x1097252 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb1cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x51u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133ea4u32, v); }
  cpu.push(0x109727d); f_01098210(cpu); if cpu.returned_to != 0x109727d { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097298); }
 let mut pc: u32 = 0x1097298;
 loop { match pc {
 0x1097298 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x10972a5); f_01097238(cpu); if cpu.returned_to != 0x10972a5 { return; }
  { let v: u32 = 0x11edc40u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11edc40u32, v); }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10973e5; continue; }
  pc = 0x10972be; continue;
 }
 0x10972be => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.cf { pc = 0x10972cf; continue; }
  pc = 0x10972c4; continue;
 }
 0x10972c4 => {
  if (cpu.cf || cpu.zf) { pc = 0x10972f1; continue; }
  pc = 0x10972c6; continue;
 }
 0x10972c6 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x10972d8; continue; }
  pc = 0x10972ca; continue;
 }
 0x10972ca => {
  pc = 0x109735f; continue;
 }
 0x10972cf => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1097346; continue; }
  pc = 0x10972d3; continue;
 }
 0x10972d3 => {
  pc = 0x109735f; continue;
 }
 0x10972d8 => {
  cpu.push(0x10972dd); f_0109757c(cpu); if cpu.returned_to != 0x10972dd { return; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11edc40u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11edc40u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10973d9; continue;
 }
 0x10972f1 => {
  { let v: u32 = 0x11edc40u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x11edc40u32; cpu.ecx = v; }
  cpu.push(0x1097302); f_010971e4(cpu); if cpu.returned_to != 0x1097302 { return; }
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.edi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x11edc40u32, v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133ea4u32, v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10973c9; continue; }
  pc = 0x109732d; continue;
 }
 0x109732d => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1097341); f_010977ec(cpu); if cpu.returned_to != 0x1097341 { return; }
  pc = 0x10973c9; continue;
 }
 0x1097346 => {
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1097359; continue; }
  pc = 0x1097352; continue;
 }
 0x1097352 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10973d9; continue;
 }
 0x1097359 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10973d9; continue;
 }
 0x109735f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb24u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10973d9; continue; }
  pc = 0x1097376; continue;
 }
 0x1097376 => {
  { let v: u32 = 0x11edc40u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x11edc40u32; cpu.ecx = v; }
  cpu.push(0x1097387); f_010971e4(cpu); if cpu.returned_to != 0x1097387 { return; }
  { let v: u32 = cpu.r32(0x1133eacu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11edc40u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133ea4u32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10973c9; continue; }
  pc = 0x10973a0; continue;
 }
 0x10973a0 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ea8u32, v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10973c9; continue; }
  pc = 0x10973b5; continue;
 }
 0x10973b5 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10973c9); f_010977ec(cpu); if cpu.returned_to != 0x10973c9 { return; }
  pc = 0x10973c9; continue;
 }
 0x10973c9 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  cpu.push(0x10973d9); f_01097a28(cpu); if cpu.returned_to != 0x10973d9 { return; }
  pc = 0x10973d9; continue;
 }
 0x10973d9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10972be; continue; }
  pc = 0x10973e5; continue;
 }
 0x10973e5 => {
  { let v: u32 = 0x11edc40u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10973f1); f_010971e4(cpu); if cpu.returned_to != 0x10973f1 { return; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133ea4u32, v); }
  cpu.push(0x109740c); f_01097a28(cpu); if cpu.returned_to != 0x109740c { return; }
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
pub fn f_01097414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097414); }
 let mut pc: u32 = 0x1097414;
 loop { match pc {
 0x1097414 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x1097421); f_01097238(cpu); if cpu.returned_to != 0x1097421 { return; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x109743a; continue; }
  pc = 0x1097426; continue;
 }
 0x1097426 => {
  if (cpu.cf || cpu.zf) { pc = 0x1097453; continue; }
  pc = 0x1097428; continue;
 }
 0x1097428 => {
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x10974a5; continue; }
  pc = 0x1097431; continue;
 }
 0x1097431 => {
  if (cpu.cf || cpu.zf) { pc = 0x1097449; continue; }
  pc = 0x1097433; continue;
 }
 0x1097433 => {
  { cpu.sub32(cpu.edx, 0xdu32); }
  if cpu.zf { pc = 0x1097476; continue; }
  pc = 0x1097438; continue;
 }
 0x1097438 => {
  pc = 0x10974a5; continue;
 }
 0x109743a => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1097530; continue; }
  pc = 0x1097442; continue;
 }
 0x1097442 => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if cpu.zf { pc = 0x1097483; continue; }
  pc = 0x1097447; continue;
 }
 0x1097447 => {
  pc = 0x10974a5; continue;
 }
 0x1097449 => {
  cpu.push(0x109744e); f_0109757c(cpu); if cpu.returned_to != 0x109744e { return; }
  pc = 0x1097530; continue;
 }
 0x1097453 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133ea8u32, v); }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x1097530; continue; }
  pc = 0x109746e; continue;
 }
 0x109746e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x109751f; continue;
 }
 0x1097476 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133ea4u32, v); }
  pc = 0x1097530; continue;
 }
 0x1097483 => {
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1133ea4u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x1097530; continue; }
  pc = 0x1097498; continue;
 }
 0x1097498 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1133ea4u32, v); }
  pc = 0x1097530; continue;
 }
 0x10974a5 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11edb18u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb14u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb24u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e98u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x1133ea4u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x1097530; continue; }
  pc = 0x10974f4; continue;
 }
 0x10974f4 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133eacu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1133ea4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1097530; continue; }
  pc = 0x1097505; continue;
 }
 0x1097505 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133ea8u32, v); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1097530; continue; }
  pc = 0x109751c; continue;
 }
 0x109751c => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x109751f; continue;
 }
 0x109751f => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1097530); f_010977ec(cpu); if cpu.returned_to != 0x1097530 { return; }
  pc = 0x1097530; continue;
 }
 0x1097530 => {
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133ea4u32); cpu.eax = v; }
  cpu.push(0x1097540); f_01097a28(cpu); if cpu.returned_to != 0x1097540 { return; }
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
pub fn f_0109757c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109757c); }
 let mut pc: u32 = 0x109757c;
 loop { match pc {
 0x109757c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1097584); f_01097238(cpu); if cpu.returned_to != 0x1097584 { return; }
  { let v: u32 = cpu.r32(0x1133ea0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133e9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb2cu32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb24u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x10975a9); f_01097788(cpu); if cpu.returned_to != 0x10975a9 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133ea4u32, v); }
  cpu.push(0x10975c0); f_01097a28(cpu); if cpu.returned_to != 0x10975c0 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010975c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10975c4); }
 let mut pc: u32 = 0x10975c4;
 loop { match pc {
 0x10975c4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10975cb); f_01097238(cpu); if cpu.returned_to != 0x10975cb { return; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1133ea8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11edb18u32, v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10975eb); f_010977ec(cpu); if cpu.returned_to != 0x10975eb { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11edb18u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097788(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097788); }
 let mut pc: u32 = 0x1097788;
 loop { match pc {
 0x1097788 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  cpu.push(0x10977ae); f_01096e38(cpu); if cpu.returned_to != 0x10977ae { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010977b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10977b4); }
 let mut pc: u32 = 0x10977b4;
 loop { match pc {
 0x10977b4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  cpu.push(0x10977d5); f_01096e90(cpu); if cpu.returned_to != 0x10977d5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010977dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10977dc); } r_010977dc(cpu, 0x10977dc); }
pub fn f_010977ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10977ec); } r_010977dc(cpu, 0x10977ec); }
fn r_010977dc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10977dc => {
  { let v: u32 = cpu.add32(cpu.eax, 0x6e010978u32); cpu.eax = v; }
  if cpu.sf { pc = 0x10977ec; continue; }
  pc = 0x10977e3; continue;
 }
 0x10977e3 => {
  { let v: u32 = cpu.r32(0x79u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edx); cpu.w32(0x79u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.fstatus(); cpu.w16(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x51u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edx); cpu.w32(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10979be; continue; }
  pc = 0x10977fd; continue;
 }
 0x10977ec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10979be; continue; }
  pc = 0x10977fd; continue;
 }
 0x10977fd => {
  cpu.poll(); pc = cpu.r32(0x10977dcu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1097805 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1097811; continue; }
  pc = 0x1097809; continue;
 }
 0x1097809 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11edb2cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x109781b; continue; }
  pc = 0x1097811; continue;
 }
 0x1097811 => {
  cpu.push(0x1097816); f_0109757c(cpu); if cpu.returned_to != 0x1097816 { return; }
  pc = 0x10979be; continue;
 }
 0x109781b => {
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x109783b); f_01096c50(cpu); if cpu.returned_to != 0x109783b { return; }
  { let v: u32 = cpu.r32(0x1133ea0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133e9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  cpu.push(0x1097868); f_01096e38(cpu); if cpu.returned_to != 0x1097868 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109786e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109787a; continue; }
  pc = 0x1097872; continue;
 }
 0x1097872 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11edb2cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1097885; continue; }
  pc = 0x109787a; continue;
 }
 0x109787a => {
  cpu.push(0x109787f); f_0109757c(cpu); if cpu.returned_to != 0x109787f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1097885 => {
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edx = v; }
  cpu.push(0x10978ac); f_01096c50(cpu); if cpu.returned_to != 0x10978ac { return; }
  { let v: u32 = cpu.r32(0x1133ea0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133e9cu32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  cpu.push(0x10978d7); f_01096e38(cpu); if cpu.returned_to != 0x10978d7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10978dd => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10978e9; continue; }
  pc = 0x10978e1; continue;
 }
 0x10978e1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x11edb24u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10978f4; continue; }
  pc = 0x10978e9; continue;
 }
 0x10978e9 => {
  cpu.push(0x10978ee); f_0109757c(cpu); if cpu.returned_to != 0x10978ee { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10978f4 => {
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  cpu.push(0x1097921); f_01096c50(cpu); if cpu.returned_to != 0x1097921 { return; }
  { let v: u32 = cpu.r32(0x1133ea0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1133e9cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  cpu.push(0x109794d); f_01096e38(cpu); if cpu.returned_to != 0x109794d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1097953 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109795f; continue; }
  pc = 0x1097957; continue;
 }
 0x1097957 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11edb24u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109796a; continue; }
  pc = 0x109795f; continue;
 }
 0x109795f => {
  cpu.push(0x1097964); f_0109757c(cpu); if cpu.returned_to != 0x1097964 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109796a => {
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  cpu.push(0x1097990); f_01096c50(cpu); if cpu.returned_to != 0x1097990 { return; }
  { let v: u32 = cpu.r32(0x1133ea0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1133e9cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10979be); f_01096e38(cpu); if cpu.returned_to != 0x10979be { return; }
  pc = 0x10979be; continue;
 }
 0x10979be => {
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
pub fn f_01097a28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097a28); }
 let mut pc: u32 = 0x1097a28;
 loop { match pc {
 0x1097a28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edb24u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1097a47; continue; }
  pc = 0x1097a3b; continue;
 }
 0x1097a3b => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11edb2cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1097a56; continue; }
  pc = 0x1097a47; continue;
 }
 0x1097a47 => {
  { let v: u32 = cpu.r32(0x11edb3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11edb34u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x1097a56; continue;
 }
 0x1097a56 => {
  { let v: u8 = cpu.r8(0x11edb28u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x11edb18u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x11edb1cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097a8a); f_010983b4(cpu); if cpu.returned_to != 0x1097a8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133ea4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ea8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097aa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097aa4); }
 let mut pc: u32 = 0x1097aa4;
 loop { match pc {
 0x1097aa4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x11edb20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x11edb38u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097b64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097b64); }
 let mut pc: u32 = 0x1097b64;
 loop { match pc {
 0x1097b64 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x1097b9f; continue; }
  pc = 0x1097b6a; continue;
 }
 0x1097b6a => {
  if (cpu.cf || cpu.zf) { pc = 0x1097b73; continue; }
  pc = 0x1097b6c; continue;
 }
 0x1097b6c => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1097b8a; continue; }
  pc = 0x1097b71; continue;
 }
 0x1097b71 => {
  pc = 0x1097b9f; continue;
 }
 0x1097b73 => {
  cpu.push(0x1097b78); f_01097d24(cpu); if cpu.returned_to != 0x1097b78 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1097b83; continue; }
  pc = 0x1097b7c; continue;
 }
 0x1097b7c => {
  { let v: u32 = 0xb0cu32; cpu.edx = v; }
  pc = 0x1097b9f; continue;
 }
 0x1097b83 => {
  { let v: u32 = 0x607u32; cpu.edx = v; }
  pc = 0x1097b9f; continue;
 }
 0x1097b8a => {
  cpu.push(0x1097b8f); f_01097d24(cpu); if cpu.returned_to != 0x1097b8f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1097b9a; continue; }
  pc = 0x1097b93; continue;
 }
 0x1097b93 => {
  { let v: u32 = 0xeu32; cpu.edx = v; }
  pc = 0x1097b9f; continue;
 }
 0x1097b9a => {
  { let v: u32 = 0x7u32; cpu.edx = v; }
  pc = 0x1097b9f; continue;
 }
 0x1097b9f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1097ba6); f_01097bd0(cpu); if cpu.returned_to != 0x1097ba6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ba8); }
 let mut pc: u32 = 0x1097ba8;
 loop { match pc {
 0x1097ba8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097bc7); f_010983b4(cpu); if cpu.returned_to != 0x1097bc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097bd0); }
 let mut pc: u32 = 0x1097bd0;
 loop { match pc {
 0x1097bd0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097beb); f_010983b4(cpu); if cpu.returned_to != 0x1097beb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097bf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097bf4); }
 let mut pc: u32 = 0x1097bf4;
 loop { match pc {
 0x1097bf4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x1097c09); f_010983b4(cpu); if cpu.returned_to != 0x1097c09 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097c1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097c1c); }
 let mut pc: u32 = 0x1097c1c;
 loop { match pc {
 0x1097c1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1097c2a); f_01097d24(cpu); if cpu.returned_to != 0x1097c2a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1097cd6; continue; }
  pc = 0x1097c32; continue;
 }
 0x1097c32 => {
  { let v: u8 = 0x5u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11edb1cu32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097c4c); f_010983b4(cpu); if cpu.returned_to != 0x1097c4c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x4eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x1097c5f); f_01098221(cpu); if cpu.returned_to != 0x1097c5f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x11edb30u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097c7e); f_010983b4(cpu); if cpu.returned_to != 0x1097c7e { return; }
  { let v: u32 = cpu.r32(0x11edb0cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11edb14u32, v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1097c9f); f_01098210(cpu); if cpu.returned_to != 0x1097c9f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11edb28u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x51u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133ea4u32, v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x1097cc4); f_01098210(cpu); if cpu.returned_to != 0x1097cc4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11edb18u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133ea8u32, v); }
  pc = 0x1097cd6; continue;
 }
 0x1097cd6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ce0); }
 let mut pc: u32 = 0x1097ce0;
 loop { match pc {
 0x1097ce0 => {
  { let v: u32 = cpu.r32(0x11edb1cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ce8); }
 let mut pc: u32 = 0x1097ce8;
 loop { match pc {
 0x1097ce8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x1097cf4); f_01097d24(cpu); if cpu.returned_to != 0x1097cf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1097d15; continue; }
  pc = 0x1097cf8; continue;
 }
 0x1097cf8 => {
  { let v: u8 = 0x5u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11edb30u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1097d12); f_010983b4(cpu); if cpu.returned_to != 0x1097d12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1097d15; continue;
 }
 0x1097d15 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d1c); }
 let mut pc: u32 = 0x1097d1c;
 loop { match pc {
 0x1097d1c => {
  { let v: u32 = cpu.r32(0x11edb30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d24); }
 let mut pc: u32 = 0x1097d24;
 loop { match pc {
 0x1097d24 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x49u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edb10u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1097d34); f_01098210(cpu); if cpu.returned_to != 0x1097d34 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d44); }
 let mut pc: u32 = 0x1097d44;
 loop { match pc {
 0x1097d44 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d68); }
 let mut pc: u32 = 0x1097d68;
 loop { match pc {
 0x1097d68 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d6c); }
 let mut pc: u32 = 0x1097d6c;
 loop { match pc {
 0x1097d6c => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d70); }
 let mut pc: u32 = 0x1097d70;
 loop { match pc {
 0x1097d70 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d74); }
 let mut pc: u32 = 0x1097d74;
 loop { match pc {
 0x1097d74 => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d78); }
 let mut pc: u32 = 0x1097d78;
 loop { match pc {
 0x1097d78 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d7c); }
 let mut pc: u32 = 0x1097d7c;
 loop { match pc {
 0x1097d7c => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d80); }
 let mut pc: u32 = 0x1097d80;
 loop { match pc {
 0x1097d80 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d84); }
 let mut pc: u32 = 0x1097d84;
 loop { match pc {
 0x1097d84 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d88); }
 let mut pc: u32 = 0x1097d88;
 loop { match pc {
 0x1097d88 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d8c); }
 let mut pc: u32 = 0x1097d8c;
 loop { match pc {
 0x1097d8c => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d90); }
 let mut pc: u32 = 0x1097d90;
 loop { match pc {
 0x1097d90 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d94); }
 let mut pc: u32 = 0x1097d94;
 loop { match pc {
 0x1097d94 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d98); }
 let mut pc: u32 = 0x1097d98;
 loop { match pc {
 0x1097d98 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097d9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097d9c); }
 let mut pc: u32 = 0x1097d9c;
 loop { match pc {
 0x1097d9c => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097da0); }
 let mut pc: u32 = 0x1097da0;
 loop { match pc {
 0x1097da0 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097da4); }
 let mut pc: u32 = 0x1097da4;
 loop { match pc {
 0x1097da4 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097da8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097da8); }
 let mut pc: u32 = 0x1097da8;
 loop { match pc {
 0x1097da8 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dac); }
 let mut pc: u32 = 0x1097dac;
 loop { match pc {
 0x1097dac => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097db0); }
 let mut pc: u32 = 0x1097db0;
 loop { match pc {
 0x1097db0 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097db4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097db4); }
 let mut pc: u32 = 0x1097db4;
 loop { match pc {
 0x1097db4 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097db8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097db8); }
 let mut pc: u32 = 0x1097db8;
 loop { match pc {
 0x1097db8 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dbc); }
 let mut pc: u32 = 0x1097dbc;
 loop { match pc {
 0x1097dbc => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dc0); }
 let mut pc: u32 = 0x1097dc0;
 loop { match pc {
 0x1097dc0 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dc4); }
 let mut pc: u32 = 0x1097dc4;
 loop { match pc {
 0x1097dc4 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dc8); }
 let mut pc: u32 = 0x1097dc8;
 loop { match pc {
 0x1097dc8 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dcc); }
 let mut pc: u32 = 0x1097dcc;
 loop { match pc {
 0x1097dcc => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dd0); }
 let mut pc: u32 = 0x1097dd0;
 loop { match pc {
 0x1097dd0 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dd4); }
 let mut pc: u32 = 0x1097dd4;
 loop { match pc {
 0x1097dd4 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dd8); }
 let mut pc: u32 = 0x1097dd8;
 loop { match pc {
 0x1097dd8 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ddc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ddc); }
 let mut pc: u32 = 0x1097ddc;
 loop { match pc {
 0x1097ddc => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097de0); }
 let mut pc: u32 = 0x1097de0;
 loop { match pc {
 0x1097de0 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097de4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097de4); }
 let mut pc: u32 = 0x1097de4;
 loop { match pc {
 0x1097de4 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097de8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097de8); }
 let mut pc: u32 = 0x1097de8;
 loop { match pc {
 0x1097de8 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dec); }
 let mut pc: u32 = 0x1097dec;
 loop { match pc {
 0x1097dec => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097df0); }
 let mut pc: u32 = 0x1097df0;
 loop { match pc {
 0x1097df0 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097df4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097df4); }
 let mut pc: u32 = 0x1097df4;
 loop { match pc {
 0x1097df4 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097df8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097df8); }
 let mut pc: u32 = 0x1097df8;
 loop { match pc {
 0x1097df8 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097dfc); }
 let mut pc: u32 = 0x1097dfc;
 loop { match pc {
 0x1097dfc => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e00); }
 let mut pc: u32 = 0x1097e00;
 loop { match pc {
 0x1097e00 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e04); }
 let mut pc: u32 = 0x1097e04;
 loop { match pc {
 0x1097e04 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e08); }
 let mut pc: u32 = 0x1097e08;
 loop { match pc {
 0x1097e08 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e0c); }
 let mut pc: u32 = 0x1097e0c;
 loop { match pc {
 0x1097e0c => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e10); }
 let mut pc: u32 = 0x1097e10;
 loop { match pc {
 0x1097e10 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e14); }
 let mut pc: u32 = 0x1097e14;
 loop { match pc {
 0x1097e14 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e18); }
 let mut pc: u32 = 0x1097e18;
 loop { match pc {
 0x1097e18 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e1c); }
 let mut pc: u32 = 0x1097e1c;
 loop { match pc {
 0x1097e1c => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e20); }
 let mut pc: u32 = 0x1097e20;
 loop { match pc {
 0x1097e20 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e24); }
 let mut pc: u32 = 0x1097e24;
 loop { match pc {
 0x1097e24 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e28); }
 let mut pc: u32 = 0x1097e28;
 loop { match pc {
 0x1097e28 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e2c); }
 let mut pc: u32 = 0x1097e2c;
 loop { match pc {
 0x1097e2c => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e30); }
 let mut pc: u32 = 0x1097e30;
 loop { match pc {
 0x1097e30 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e34); }
 let mut pc: u32 = 0x1097e34;
 loop { match pc {
 0x1097e34 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e38); }
 let mut pc: u32 = 0x1097e38;
 loop { match pc {
 0x1097e38 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e3c); }
 let mut pc: u32 = 0x1097e3c;
 loop { match pc {
 0x1097e3c => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e40); }
 let mut pc: u32 = 0x1097e40;
 loop { match pc {
 0x1097e40 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e44); }
 let mut pc: u32 = 0x1097e44;
 loop { match pc {
 0x1097e44 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e48); }
 let mut pc: u32 = 0x1097e48;
 loop { match pc {
 0x1097e48 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e4c); }
 let mut pc: u32 = 0x1097e4c;
 loop { match pc {
 0x1097e4c => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e50); }
 let mut pc: u32 = 0x1097e50;
 loop { match pc {
 0x1097e50 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e54); }
 let mut pc: u32 = 0x1097e54;
 loop { match pc {
 0x1097e54 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e58); }
 let mut pc: u32 = 0x1097e58;
 loop { match pc {
 0x1097e58 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e5c); }
 let mut pc: u32 = 0x1097e5c;
 loop { match pc {
 0x1097e5c => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e60); }
 let mut pc: u32 = 0x1097e60;
 loop { match pc {
 0x1097e60 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e64); }
 let mut pc: u32 = 0x1097e64;
 loop { match pc {
 0x1097e64 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e68); }
 let mut pc: u32 = 0x1097e68;
 loop { match pc {
 0x1097e68 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e6c); }
 let mut pc: u32 = 0x1097e6c;
 loop { match pc {
 0x1097e6c => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e70); }
 let mut pc: u32 = 0x1097e70;
 loop { match pc {
 0x1097e70 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e74); }
 let mut pc: u32 = 0x1097e74;
 loop { match pc {
 0x1097e74 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e78); }
 let mut pc: u32 = 0x1097e78;
 loop { match pc {
 0x1097e78 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e7c); }
 let mut pc: u32 = 0x1097e7c;
 loop { match pc {
 0x1097e7c => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e80); }
 let mut pc: u32 = 0x1097e80;
 loop { match pc {
 0x1097e80 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e84); }
 let mut pc: u32 = 0x1097e84;
 loop { match pc {
 0x1097e84 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e88); }
 let mut pc: u32 = 0x1097e88;
 loop { match pc {
 0x1097e88 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e8c); }
 let mut pc: u32 = 0x1097e8c;
 loop { match pc {
 0x1097e8c => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e90); }
 let mut pc: u32 = 0x1097e90;
 loop { match pc {
 0x1097e90 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e94); }
 let mut pc: u32 = 0x1097e94;
 loop { match pc {
 0x1097e94 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e98); }
 let mut pc: u32 = 0x1097e98;
 loop { match pc {
 0x1097e98 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097e9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097e9c); }
 let mut pc: u32 = 0x1097e9c;
 loop { match pc {
 0x1097e9c => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ea0); }
 let mut pc: u32 = 0x1097ea0;
 loop { match pc {
 0x1097ea0 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ea4); }
 let mut pc: u32 = 0x1097ea4;
 loop { match pc {
 0x1097ea4 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ea8); }
 let mut pc: u32 = 0x1097ea8;
 loop { match pc {
 0x1097ea8 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097eac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097eac); }
 let mut pc: u32 = 0x1097eac;
 loop { match pc {
 0x1097eac => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097eb0); }
 let mut pc: u32 = 0x1097eb0;
 loop { match pc {
 0x1097eb0 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097eb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097eb4); }
 let mut pc: u32 = 0x1097eb4;
 loop { match pc {
 0x1097eb4 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097eb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097eb8); }
 let mut pc: u32 = 0x1097eb8;
 loop { match pc {
 0x1097eb8 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ebc); }
 let mut pc: u32 = 0x1097ebc;
 loop { match pc {
 0x1097ebc => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ec0); }
 let mut pc: u32 = 0x1097ec0;
 loop { match pc {
 0x1097ec0 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ec4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ec4); }
 let mut pc: u32 = 0x1097ec4;
 loop { match pc {
 0x1097ec4 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ec8); }
 let mut pc: u32 = 0x1097ec8;
 loop { match pc {
 0x1097ec8 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ecc); }
 let mut pc: u32 = 0x1097ecc;
 loop { match pc {
 0x1097ecc => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ed0); }
 let mut pc: u32 = 0x1097ed0;
 loop { match pc {
 0x1097ed0 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ed4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ed4); }
 let mut pc: u32 = 0x1097ed4;
 loop { match pc {
 0x1097ed4 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ed8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ed8); }
 let mut pc: u32 = 0x1097ed8;
 loop { match pc {
 0x1097ed8 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097edc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097edc); }
 let mut pc: u32 = 0x1097edc;
 loop { match pc {
 0x1097edc => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ee0); }
 let mut pc: u32 = 0x1097ee0;
 loop { match pc {
 0x1097ee0 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ee4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ee4); }
 let mut pc: u32 = 0x1097ee4;
 loop { match pc {
 0x1097ee4 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ee8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ee8); }
 let mut pc: u32 = 0x1097ee8;
 loop { match pc {
 0x1097ee8 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097eec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097eec); }
 let mut pc: u32 = 0x1097eec;
 loop { match pc {
 0x1097eec => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ef0); }
 let mut pc: u32 = 0x1097ef0;
 loop { match pc {
 0x1097ef0 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ef4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ef4); }
 let mut pc: u32 = 0x1097ef4;
 loop { match pc {
 0x1097ef4 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ef8); }
 let mut pc: u32 = 0x1097ef8;
 loop { match pc {
 0x1097ef8 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097efc); }
 let mut pc: u32 = 0x1097efc;
 loop { match pc {
 0x1097efc => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f00); }
 let mut pc: u32 = 0x1097f00;
 loop { match pc {
 0x1097f00 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f04); }
 let mut pc: u32 = 0x1097f04;
 loop { match pc {
 0x1097f04 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f08); }
 let mut pc: u32 = 0x1097f08;
 loop { match pc {
 0x1097f08 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f0c); }
 let mut pc: u32 = 0x1097f0c;
 loop { match pc {
 0x1097f0c => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f10); }
 let mut pc: u32 = 0x1097f10;
 loop { match pc {
 0x1097f10 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f14); }
 let mut pc: u32 = 0x1097f14;
 loop { match pc {
 0x1097f14 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f18); }
 let mut pc: u32 = 0x1097f18;
 loop { match pc {
 0x1097f18 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f1c); }
 let mut pc: u32 = 0x1097f1c;
 loop { match pc {
 0x1097f1c => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f20); }
 let mut pc: u32 = 0x1097f20;
 loop { match pc {
 0x1097f20 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f24); }
 let mut pc: u32 = 0x1097f24;
 loop { match pc {
 0x1097f24 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f28); }
 let mut pc: u32 = 0x1097f28;
 loop { match pc {
 0x1097f28 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f2c); }
 let mut pc: u32 = 0x1097f2c;
 loop { match pc {
 0x1097f2c => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f30); }
 let mut pc: u32 = 0x1097f30;
 loop { match pc {
 0x1097f30 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f34); }
 let mut pc: u32 = 0x1097f34;
 loop { match pc {
 0x1097f34 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f38); }
 let mut pc: u32 = 0x1097f38;
 loop { match pc {
 0x1097f38 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f3c); }
 let mut pc: u32 = 0x1097f3c;
 loop { match pc {
 0x1097f3c => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
