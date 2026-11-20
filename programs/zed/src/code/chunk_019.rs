#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010acd07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acd07); }
 let mut pc: u32 = 0x10acd07;
 loop { match pc {
 0x10acd07 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acd3f; continue; }
  pc = 0x10acd0f; continue;
 }
 0x10acd0f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acd07; continue; }
  pc = 0x10acd3f; continue;
 }
 0x10acd3f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acd40); }
 let mut pc: u32 = 0x10acd40;
 loop { match pc {
 0x10acd40 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acd66; continue; }
  pc = 0x10acd48; continue;
 }
 0x10acd48 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x1134f8cu32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acd40; continue; }
  pc = 0x10acd66; continue;
 }
 0x10acd66 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acd67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acd67); }
 let mut pc: u32 = 0x10acd67;
 loop { match pc {
 0x10acd67 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acd97; continue; }
  pc = 0x10acd6f; continue;
 }
 0x10acd6f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x1134f8cu32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acd67; continue; }
  pc = 0x10acd97; continue;
 }
 0x10acd97 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acd98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acd98); }
 let mut pc: u32 = 0x10acd98;
 loop { match pc {
 0x10acd98 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acdcf; continue; }
  pc = 0x10acda0; continue;
 }
 0x10acda0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acd98; continue; }
  pc = 0x10acdcf; continue;
 }
 0x10acdcf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acdd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acdd0); }
 let mut pc: u32 = 0x10acdd0;
 loop { match pc {
 0x10acdd0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ace08; continue; }
  pc = 0x10acdd8; continue;
 }
 0x10acdd8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acdd0; continue; }
  pc = 0x10ace08; continue;
 }
 0x10ace08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ace09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ace09); }
 let mut pc: u32 = 0x10ace09;
 loop { match pc {
 0x10ace09 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ace40; continue; }
  pc = 0x10ace11; continue;
 }
 0x10ace11 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ace09; continue; }
  pc = 0x10ace40; continue;
 }
 0x10ace40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ace41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ace41); }
 let mut pc: u32 = 0x10ace41;
 loop { match pc {
 0x10ace41 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ace79; continue; }
  pc = 0x10ace49; continue;
 }
 0x10ace49 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ace41; continue; }
  pc = 0x10ace79; continue;
 }
 0x10ace79 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ace7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ace7a); }
 let mut pc: u32 = 0x10ace7a;
 loop { match pc {
 0x10ace7a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10aceb1; continue; }
  pc = 0x10ace82; continue;
 }
 0x10ace82 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ace7a; continue; }
  pc = 0x10aceb1; continue;
 }
 0x10aceb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aceb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aceb2); }
 let mut pc: u32 = 0x10aceb2;
 loop { match pc {
 0x10aceb2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acef9; continue; }
  pc = 0x10aceba; continue;
 }
 0x10aceba => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10aceb2; continue; }
  pc = 0x10acef9; continue;
 }
 0x10acef9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acefa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acefa); }
 let mut pc: u32 = 0x10acefa;
 loop { match pc {
 0x10acefa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acf4a; continue; }
  pc = 0x10acf02; continue;
 }
 0x10acf02 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acefa; continue; }
  pc = 0x10acf4a; continue;
 }
 0x10acf4a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acf4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acf4b); }
 let mut pc: u32 = 0x10acf4b;
 loop { match pc {
 0x10acf4b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acf9c; continue; }
  pc = 0x10acf53; continue;
 }
 0x10acf53 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acf4b; continue; }
  pc = 0x10acf9c; continue;
 }
 0x10acf9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acf9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acf9d); }
 let mut pc: u32 = 0x10acf9d;
 loop { match pc {
 0x10acf9d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10acfed; continue; }
  pc = 0x10acfa5; continue;
 }
 0x10acfa5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acf9d; continue; }
  pc = 0x10acfed; continue;
 }
 0x10acfed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acfee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acfee); }
 let mut pc: u32 = 0x10acfee;
 loop { match pc {
 0x10acfee => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad03f; continue; }
  pc = 0x10acff6; continue;
 }
 0x10acff6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10acfee; continue; }
  pc = 0x10ad03f; continue;
 }
 0x10ad03f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad040); }
 let mut pc: u32 = 0x10ad040;
 loop { match pc {
 0x10ad040 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad072; continue; }
  pc = 0x10ad048; continue;
 }
 0x10ad048 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad040; continue; }
  pc = 0x10ad072; continue;
 }
 0x10ad072 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad073(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad073); }
 let mut pc: u32 = 0x10ad073;
 loop { match pc {
 0x10ad073 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad0b5; continue; }
  pc = 0x10ad07b; continue;
 }
 0x10ad07b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad073; continue; }
  pc = 0x10ad0b5; continue;
 }
 0x10ad0b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad0b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad0b6); }
 let mut pc: u32 = 0x10ad0b6;
 loop { match pc {
 0x10ad0b6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad0fb; continue; }
  pc = 0x10ad0be; continue;
 }
 0x10ad0be => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad0b6; continue; }
  pc = 0x10ad0fb; continue;
 }
 0x10ad0fb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad0fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad0fc); }
 let mut pc: u32 = 0x10ad0fc;
 loop { match pc {
 0x10ad0fc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad142; continue; }
  pc = 0x10ad104; continue;
 }
 0x10ad104 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad0fc; continue; }
  pc = 0x10ad142; continue;
 }
 0x10ad142 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad143); }
 let mut pc: u32 = 0x10ad143;
 loop { match pc {
 0x10ad143 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad188; continue; }
  pc = 0x10ad14b; continue;
 }
 0x10ad14b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad143; continue; }
  pc = 0x10ad188; continue;
 }
 0x10ad188 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad189(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad189); }
 let mut pc: u32 = 0x10ad189;
 loop { match pc {
 0x10ad189 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1134f7cu32)); }
  if !cpu.cf { pc = 0x10ad1cf; continue; }
  pc = 0x10ad191; continue;
 }
 0x10ad191 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1134f88u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1134f8cu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1134f84u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10ad189; continue; }
  pc = 0x10ad1cf; continue;
 }
 0x10ad1cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
